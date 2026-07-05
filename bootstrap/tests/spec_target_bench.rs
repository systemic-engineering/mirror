//! Phase A RED — target bench grammar keyword per /loop 2026-07-05 Arc 3a
//! + Seam Phase D `91e79c8` §7 canonical execution.
//!
//! Opens perf measurement floor: `cargo bench` becomes a valid check
//! value for mirror.spec target blocks, enabling Sub-arc 3b (wiring
//! `@mirror/bench` into `mirror.spec` via one `target bench { ... }` block).
//!
//! **RED phase**: `bootstrap/src/lib.rs` `cargo_args_for_check` at line
//! ~1093 does NOT include the `"bench" => &["bench"]` arm; unknown
//! values fall back to `&["check"]` with a stderr warning.
//!
//! **GREEN phase** (Mara): add `"bench" => &["bench"]` arm; extend
//! docblock in `shards/mirror/spec.mirror` to mention bench as valid
//! `check` value (docblock addition; no grammar change per Seam §7).

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (repo root)")
        .to_path_buf()
}

fn read_source(rel: &str) -> String {
    let path = repo_root().join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {:?}: {}", path, e))
}

#[test]
fn cargo_args_for_check_has_bench_arm() {
    let content = read_source("bootstrap/src/lib.rs");
    assert!(
        content.contains(r#""bench" => &["bench"]"#),
        "bootstrap/src/lib.rs `cargo_args_for_check` must contain \
         `\"bench\" => &[\"bench\"]` arm per Seam Phase D `91e79c8` §7 \
         Sub-arc 3a. Opens perf measurement floor for `target bench` \
         dispatch via `cargo bench`."
    );
}

#[test]
fn shards_mirror_spec_mentions_bench_check() {
    let content = read_source("shards/mirror/spec.mirror");
    assert!(
        content.contains("bench"),
        "shards/mirror/spec.mirror must mention `bench` per Seam \
         `91e79c8` §7 Sub-arc 3a docblock convention. Names the \
         substrate-decl acceptance of `bench` as a valid `check` \
         action value; consumer of `@mirror/bench` (LANDED 2026-07-01)."
    );
}
