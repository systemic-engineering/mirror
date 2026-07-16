//! Tick 1 RED — `mirror roomba --commit --collapse=<path>` fracture-
//! detection scope discipline.
//!
//! Alex 2026-07-16 directive: `--collapse=<path>` extends `mirror roomba
//! --commit` with a Rust scope-restrictor. Substrate-honest reading:
//! the fracture-detector `roomba_fracture::scan(root, sub_path)` walks
//! only within `root.join(sub_path)` when Some; falls back to
//! `bootstrap/src` when None (pre-Tick-1 backward-compat).
//!
//! Same word/semantic as pipeforward §5.5.4 --collapse (both = explicit
//! force-a-collapse; scope-restrictor for the Rust→mirror ouroboros
//! discharge path per Reed's forward-path Tick 1 per Seam Phase D
//! adjudication task #180).
//!
//! RED-phase expectation: `roomba_fracture::scan(root, Some(path))` does
//! not exist yet — tests do not compile. GREEN-phase expectation: scan
//! function lands with optional sub_path parameter; None-path preserves
//! pre-Tick-1 behavior; Some-path scopes to that subpath.
//!
//! Marker: [substrate-floor:@io-boundary] — the @io/fs read boundary at
//! which fracture-detection scan reads .rs files. Audit-cite: Seam Phase
//! D adjudication task #180 forward-path Tick 1 UNBLOCKED verdict.
//! Signed-off-by: Seam.

use mirror::roomba_fracture;
use std::path::PathBuf;

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "roomba-collapse-scope-{}-{}",
        tag,
        std::process::id()
    ));
    if dir.exists() {
        let _ = std::fs::remove_dir_all(&dir);
    }
    std::fs::create_dir_all(&dir).expect("scratch");
    dir
}

#[test]
fn collapse_path_scopes_fracture_detection_to_specified_subdir() {
    // Substrate-honest semantic: --collapse=<path> restricts the fracture-
    // detection scope to files WITHIN root.join(path). Files outside the
    // scope are NOT scanned even if they contain stale-names.
    let root = scratch("scope");
    std::fs::create_dir_all(root.join("in_scope")).expect("in_scope dir");
    std::fs::create_dir_all(root.join("out_of_scope")).expect("out_of_scope dir");

    // Docblock with stale-name ("mirror execute" per current RENAME_TABLE).
    let stale_content = "//! This docblock uses mirror execute which is stale.\n";
    std::fs::write(root.join("in_scope").join("foo.rs"), stale_content)
        .expect("seed in_scope");
    std::fs::write(root.join("out_of_scope").join("bar.rs"), stale_content)
        .expect("seed out_of_scope");

    // Scan with collapse_path=Some("in_scope"): finds ONE fracture, in
    // in_scope/foo.rs. The out_of_scope/bar.rs is NOT scanned.
    let fractures =
        roomba_fracture::scan(&root, Some(&PathBuf::from("in_scope")));

    assert_eq!(
        fractures.len(),
        1,
        "expected 1 fracture in in_scope/, got {:?}",
        fractures
            .iter()
            .map(|f| f.file_path.display().to_string())
            .collect::<Vec<_>>()
    );
    assert!(
        fractures[0]
            .file_path
            .to_string_lossy()
            .contains("in_scope"),
        "fracture MUST be in in_scope/ subdir; got {}",
        fractures[0].file_path.display()
    );

    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn collapse_path_none_preserves_pre_tick_1_backward_compat() {
    // Substrate-honest backward-compat: None-path defaults to
    // root/bootstrap/src (pre-Tick-1 scan_bootstrap_src behavior).
    // Existing callers that pass None get the same walk they had before.
    let root = scratch("backcompat");

    // No bootstrap/src in scratch dir; scan returns 0 fractures.
    let fractures = roomba_fracture::scan(&root, None);

    assert_eq!(
        fractures.len(),
        0,
        "None-path scan on scratch without bootstrap/src should find zero fractures; got {:?}",
        fractures
    );

    // Now create bootstrap/src with a fracture.
    std::fs::create_dir_all(root.join("bootstrap").join("src"))
        .expect("bootstrap/src");
    std::fs::write(
        root.join("bootstrap").join("src").join("lib.rs"),
        "//! mirror execute is deprecated.\n",
    )
    .expect("seed bootstrap/src fixture");

    // None-path scan finds it (default to bootstrap/src).
    let default_fractures = roomba_fracture::scan(&root, None);
    assert_eq!(
        default_fractures.len(),
        1,
        "None-path scan MUST find the bootstrap/src fixture (backward-compat)"
    );

    // Some-path="bootstrap/src" is equivalent to None: same 1 fracture.
    let explicit_fractures =
        roomba_fracture::scan(&root, Some(&PathBuf::from("bootstrap/src")));
    assert_eq!(
        default_fractures.len(),
        explicit_fractures.len(),
        "Some(bootstrap/src) MUST be equivalent to None (backward-compat contract)"
    );

    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn collapse_path_scan_recurses_into_nested_subdirs() {
    // The scan traverses recursively into subdirectories under the
    // collapse-path root; nested fractures are found.
    let root = scratch("recursive");
    let nested = root.join("target/deep/nested");
    std::fs::create_dir_all(&nested).expect("nested dirs");

    let stale_content = "//! mirror execute is the deprecated verb.\n";
    std::fs::write(nested.join("deep.rs"), stale_content).expect("seed nested");

    let fractures = roomba_fracture::scan(&root, Some(&PathBuf::from("target")));

    assert_eq!(
        fractures.len(),
        1,
        "recursive scan MUST find deep fracture; got {:?}",
        fractures
            .iter()
            .map(|f| f.file_path.display().to_string())
            .collect::<Vec<_>>()
    );

    let _ = std::fs::remove_dir_all(&root);
}
