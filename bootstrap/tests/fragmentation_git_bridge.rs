//! Cargo-edge bridge test for `fragmentation-git`.
//!
//! Pattern-identical to `fragmentation_bridge.rs` (committed at
//! 6b36808 as P2 of the mirror-init loop). The `as _` idiom is
//! pure symbol-resolution: the test compiles iff the path-dep
//! resolves AND the named items are public in the dependency's
//! API surface.
//!
//! Per Taut's scout (2026-06-27) the three load-bearing entry
//! points for `mirror init` P4 GREEN composition are:
//!
//!   - `NamespacedGitStore::open`
//!   - `NamespacedGitStore::insert_persistent`
//!   - `NamespacedGitStore::set_ref`
//!
//! All three are methods on `NamespacedGitStore`; one symbol-import
//! gates all three. (We do not import the method names; impl-block
//! method surface is verified by the P4 GREEN test pass that
//! actually CALLS them.)

#[test]
fn fragmentation_git_namespaced_store_is_importable() {
    // NamespacedGitStore lives at module path `namespaced` and is NOT
    // re-exported at the crate root — the compile-error on the
    // root-path attempt proved this informatively (same pattern as
    // the fragmentation_bridge test discovery loop).
    use fragmentation_git::namespaced::NamespacedGitStore as _;
}
