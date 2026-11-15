//! Command line for `desktop-codegen`.
//!
//! Run it as `cargo run -p desktop-codegen --locked -- <source> <check|generate>`.
//! Exit code `0` means ok, `1` means the source is not valid or an output
//! is out of date, and `2` means the command was used wrongly.

use std::process::ExitCode;

use desktop_codegen::{Problem, SOURCES, Source, check, generate, repo_root, source};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [name, action] = args.as_slice() else {
        return usage();
    };
    let Some(source) = source(name) else {
        return usage();
    };
    let root = repo_root();
    match action.as_str() {
        "check" => match check(&root, source) {
            Ok(line) => {
                println!("{line}");
                ExitCode::SUCCESS
            }
            Err(problems) => failed(source, &problems),
        },
        "generate" => match generate(&root, source) {
            Ok((written, line)) => {
                for path in written {
                    println!("wrote {path}");
                }
                println!("{line}");
                ExitCode::SUCCESS
            }
            Err(problems) => failed(source, &problems),
        },
        _ => usage(),
    }
}

/// Prints every problem and the failure line, then exits with code 1.
fn failed(source: &Source, problems: &[Problem]) -> ExitCode {
    for problem in problems {
        eprintln!("{problem}");
    }
    eprintln!("{}: failed, {} problems", source.name, problems.len());
    ExitCode::from(1)
}

/// Prints the two usage lines and exits with code 2.
fn usage() -> ExitCode {
    let names: Vec<&str> = SOURCES.iter().map(|source| source.name).collect();
    eprintln!("usage: desktop-codegen <source> <check|generate>");
    eprintln!("sources: {}", names.join(", "));
    ExitCode::from(2)
}
