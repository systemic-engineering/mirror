//! R1 at the bin's API (docs/audits/2026-10-29-seam-pact-eval-and-ruler-envelope.md
//! §2.3): the bin never narrows with a bare budget. No file under
//! `rust/src/` calls `engine::run_bounded` or `engine::Budget::new`; a
//! bounded run goes through `ruler::run_bounded` and a `Ruler<Count>`.
//!
//! Stricter than the two strings: `run_bounded(` and `Budget::new(` may not
//! appear at all, so a `use engine::run_bounded;` can't slip past.

use std::path::{Path, PathBuf};

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

#[test]
fn the_bin_never_calls_the_bare_bounded_core() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    rust_files(&src, &mut files);
    files.sort();
    assert!(!files.is_empty());
    for f in &files {
        let text = std::fs::read_to_string(f).unwrap();
        for (i, line) in text.lines().enumerate() {
            let code = line.split("//").next().unwrap();
            for banned in ["engine::run_bounded", "engine::Budget::new", "run_bounded(", "Budget::new("] {
                if banned.starts_with("engine::") || !code.contains("ruler::run_bounded(") {
                    assert!(!code.contains(banned), "{}:{}: {banned}", f.display(), i + 1);
                }
            }
        }
    }
}
