//! Contract tests for the `architecture` source of spec `_root/0007`.
//!
//! These tests use only the public paths of `desktop_codegen` and the
//! standard library. `every_source_passes_check_on_the_repo` runs `check`
//! for every entry of `SOURCES`, so a registered source cannot slip past
//! the gate (AC-1, AC-3). The rules and cases are in
//! `docs/specs/_root/0007-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md`.

use desktop_codegen::{SOURCES, check, repo_root};

#[test]
fn every_source_passes_check_on_the_repo() {
    let root = repo_root();
    let mut failures = Vec::new();
    for source in SOURCES {
        match check(&root, source) {
            Ok(line) => {
                if !line.starts_with(&format!("{}: ok, ", source.name)) {
                    failures.push(format!("{}: unexpected line: {line}", source.name));
                }
                if source.name == "architecture" && !line.ends_with("0 outputs up to date") {
                    failures.push(format!("architecture: unexpected line: {line}"));
                }
            }
            Err(problems) => {
                for problem in problems {
                    failures.push(format!("{}: {problem}", source.name));
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
