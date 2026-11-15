"""Refresh the shared Graphify artifacts inside a disposable Git index snapshot."""

import importlib.util
import json
import os
from pathlib import Path
import shutil
import sys
from urllib.parse import quote


def load_graphify():
    if importlib.util.find_spec("graphify") is not None:
        return
    # pipx and uv install Graphify in a private environment. Use its interpreter.
    launcher = shutil.which("graphify")
    candidates = []
    if os.environ.get("GRAPHIFY_PYTHON"):
        candidates.append(Path(os.environ["GRAPHIFY_PYTHON"]))
    if launcher:
        directory = Path(launcher).resolve().parent
        candidates.extend(directory / name for name in ("python", "python.exe"))
    for python in candidates:
        if python.is_file() and str(python) != sys.executable:
            os.execv(str(python), [str(python), str(Path(__file__).resolve()), *sys.argv[1:]])
    sys.exit("Install graphifyy or set GRAPHIFY_PYTHON to its Python interpreter.")


def refresh(root):
    from graphify.analyze import god_nodes, suggest_questions, surprising_connections
    from graphify.build import build_from_json
    from graphify.cluster import cluster, label_communities_by_hub, score_all
    from graphify.detect import _md5_file, detect, save_manifest
    from graphify.export import to_html, to_json
    from graphify.extract import _file_node_id, _get_extractor, extract
    from graphify.extractors.base import _make_id
    from graphify.report import generate

    os.chdir(root)
    output = Path("graphify-out")
    output.mkdir(exist_ok=True)
    manifest = output / "manifest.json"
    graph_path = output / "graph.json"
    report_path = output / "GRAPH_REPORT.md"
    html_path = output / "graph.html"
    old_hashes = json.loads(manifest.read_text(encoding="utf-8")) if manifest.exists() else {}
    old = json.loads(graph_path.read_text(encoding="utf-8")) if graph_path.exists() else {
        "nodes": [], "links": [], "hyperedges": []
    }

    scan = detect(root)
    if scan.get("walk_errors"):
        sys.exit(f"Graphify could not read the snapshot: {scan['walk_errors']}")
    files = {Path(f).relative_to(root).as_posix(): kind
             for kind, paths in scan["files"].items() for f in paths}
    hashes = {name: _md5_file(root / name) for name in files}
    if not all(hashes.values()):
        sys.exit("Graphify could not hash every source in the Git index snapshot.")
    unchanged = {name for name, digest in hashes.items()
                 if old_hashes.get(name, {}).get("ast_hash") == digest}
    semantic_current = {name for name, digest in hashes.items()
                        if old_hashes.get(name, {}).get("semantic_hash") == digest}
    if unchanged == files.keys() == old_hashes.keys() and all(
        p.is_file() for p in (graph_path, manifest, report_path, html_path)
    ):
        print("Graphify: staged sources unchanged.")
        return

    # Re-resolve structural links across the staged corpus. A new or deleted
    # destination can affect an import in a file whose own bytes did not change.
    paths = [root / name for name in sorted(files)]
    parsable = [p for p in paths if _get_extractor(p) is not None]
    structure = extract(parsable, root=root, cache_root=root, parallel=False)
    if structure.get("failed_sources"):
        sys.exit(f"Graphify extraction failed: {structure['failed_sources']}")
    # Graphify 0.9.48 does not decode %20 in Markdown destinations.
    escaped = {_make_id(str(root / quote(name))): _file_node_id(Path(name))
               for name in files if quote(name) != name}
    for edge in structure["edges"]:
        edge["target"] = escaped.get(edge["target"], edge["target"])

    # The original graph contains only semantic Markdown records. Graphify
    # backfilled their _origin as "ast" from L-number citations. Its native
    # Markdown extractor, in contrast, always emits page/heading node_kind.
    legacy_docs = {n["source_file"] for n in old["nodes"] if "captured_at" in n
                   and files.get(n["source_file"]) == "document"}
    legacy_docs.difference_update(n["source_file"] for n in old["nodes"] if n.get("node_kind"))

    def current_semantics(record):
        source = record.get("source_file")
        return source in semantic_current and (record.get("_origin") == "semantic"
                                               or source in legacy_docs)

    nodes = {n["id"]: n for n in structure["nodes"]}
    for n in old["nodes"]:
        if current_semantics(n):
            nodes[n["id"]] = {**nodes.get(n["id"], {}), **n, "_origin": "semantic"}
    for name, kind in files.items():
        if _get_extractor(root / name) is None:
            nid = _file_node_id(Path(name))
            nodes.setdefault(nid, {"id": nid, "label": name, "source_file": name,
                                   "file_type": kind, "node_kind": "page"})
    edges = [{**e, "_origin": "semantic"} for e in old["links"] if current_semantics(e)]
    edges.extend(structure["edges"])
    data = {
        "nodes": list(nodes.values()),
        "edges": [e for e in edges if e["source"] in nodes and e["target"] in nodes],
        "hyperedges": [h for h in old.get("hyperedges", [])
                       if h.get("source_file") in semantic_current and all(n in nodes for n in h["nodes"])],
    }
    graph = build_from_json(data, root=root, directed=old.get("directed", False))
    groups = cluster(graph)
    labels = label_communities_by_hub(graph, groups)
    # Retain an existing curated name when the community membership is unchanged.
    previous_members = {}
    previous_labels = {}
    for n in old["nodes"]:
        if n.get("community_name"):
            previous_members.setdefault(n["community"], set()).add(n["id"])
            previous_labels[n["community"]] = n["community_name"]
    by_members = {frozenset(ids): previous_labels[cid] for cid, ids in previous_members.items()}
    labels.update({cid: by_members[frozenset(ids)] for cid, ids in groups.items()
                   if frozenset(ids) in by_members})
    if not to_json(graph, groups, str(graph_path), force=True, community_labels=labels):
        sys.exit("Graphify could not write graph.json.")
    if not to_html(graph, groups, str(html_path), community_labels=labels):
        sys.exit("Graphify could not write graph.html.")
    report = generate(graph, groups, score_all(graph, groups), labels, god_nodes(graph),
                      surprising_connections(graph, groups), scan, {}, "desktop",
                      suggested_questions=suggest_questions(graph, groups, labels))
    report += ("\nPembaruan hook menggunakan indeks struktural lokal. Hash semantik kosong "
               "menandai dokumen yang menunggu ekstraksi semantik.\n")
    report_path.write_text(report, encoding="utf-8")
    changed = files.keys() - unchanged
    corpus = {str(root / name) for name in files}
    save_manifest({"changed": [str(root / name) for name in changed - semantic_current]},
                  str(manifest), kind="ast", root=root, scan_corpus=corpus)
    if changed & semantic_current:
        # A semantic pass may already have refreshed this file on a newer revision
        # than its AST. Keep that evidence when advancing the structural stamp.
        save_manifest({"changed": [str(root / name) for name in changed & semantic_current]},
                      str(manifest), kind="both", root=root, scan_corpus=corpus)
    print(f"Graphify: {len(files)} sources, {len(graph)} nodes, {graph.number_of_edges()} edges.")


if __name__ == "__main__":
    load_graphify()
    refresh(Path(sys.argv[1]).resolve())
