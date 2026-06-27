//! Cargo-edge RED: `fragmentation` crate is importable from mirror's bootstrap.
//!
//! Per Mara's mirror-init spec (`mirror/docs/specs/mirror-init.md`,
//! commits `fe215bd` → `14dd043`, ~1208 lines) + Taut's scout
//! (`mirror/docs/scouts/2026-06-27-taut-fragmentation-git-store-for-mirror-init.md`,
//! commit `5580a7e`):
//!
//! Mirror has declared fragmentation as the Rust substrate
//! ([[architecture-fragmentation-is-the-rust-substrate]] memory entry +
//! `mirror/docs/specs/store-vs-db-and-the-cascade.md` Mara 2026-06-04)
//! and never added the Cargo edge. The entire git surface is 60 lines
//! of `Command::new("git")` shell-out in `bootstrap/src/git.rs`. The
//! substrate "already had the word" AND the wiring — the consumer
//! just never plugged in.
//!
//! This test is the smallest possible bridge proof: import the
//! canonical content-addressed persistence primitive (`FrgmntStore`
//! per Taut's scout §3). Compile-time symbol resolution suffices at
//! this altitude; runtime exercise lands in subsequent ticks once
//! the Cargo edge holds.
//!
//! ## What this RED test asserts
//!
//! - `fragmentation` is a resolvable crate dependency
//! - `fragmentation::frgmnt_store::FrgmntStore` is the public symbol
//!   the substrate names (per Taut's scout's load-bearing-primitive
//!   inventory)
//!
//! ## What it does NOT assert
//!
//! - Runtime behavior (write/read/persist) — follow-up ticks
//! - `fragmentation-git` workspace member integration — separate Cargo
//!   edge in v0.1+
//! - Binary size impact (R2 per Seam's audit) — measured at GREEN
//!
//! ## Falsifiability
//!
//! Until `fragmentation = { path = "../../fragmentation" }` is added
//! to `bootstrap/Cargo.toml`, this file fails to compile (unresolved
//! crate). The GREEN tick adds the edge; this file then compiles +
//! the test passes (compile-success IS the verdict at this altitude).

#[test]
fn fragmentation_frgmnt_store_is_importable() {
    // The `as _` idiom: pure symbol-resolution check, no instantiation.
    // `FrgmntStore<N: Fragmentable + Clone>` is generic; instantiating
    // would require a concrete `Fragmentable` carrier which the bridge
    // test deliberately doesn't have at this altitude (runtime exercise
    // lands in subsequent `mirror init` ticks).
    //
    // The `use ... as _` statement verifies the module path resolves
    // AND that `FrgmntStore` is a public symbol — the two things the
    // Cargo edge being wired actually guarantees.
    use fragmentation::frgmnt_store::FrgmntStore as _;
}
