//! `spectral.rs` — build-time shard manifest as macro-consumer floor
//! for `prismqueer`'s Connes (A, H, D) triple.
//!
//! Per Taut substrate-scout 2026-07-23 (macro-territory partial
//! verdict): `prism/prismqueer` already encodes the Connes spectral
//! triple as a trait tower (`bundle::{Fiber, Connection, Transport,
//! Bundle}` at `bundle.rs:71-193`) and exports `apply_h` as the
//! operator-on-Hilbert-space action (`lib.rs:216-238`). The gap Alex
//! named 2026-07-22 ("what if we had rust/src/spectral.rs") is a
//! compile-time pipeline that binds every `shards/**/*.mirror` to
//! that trait tower at rust/ altitude.
//!
//! ## Tick 1 scope (this landing)
//!
//! The thinnest possible pipeline proof: expose the shard manifest
//! that `rust/build.rs` emits at cargo build time. Zero parsing, zero
//! trait binding — Tick 1 verifies the enumeration surface only.
//!
//! ## What this is NOT (forward-promised to Ticks 2+)
//!
//! - NOT a claim that each shard is bound to `prismqueer::bundle::
//!   Bundle` at rust/ altitude yet — Tick 2 lands the first per-shard
//!   `impl Bundle` end-to-end.
//! - NOT a retirement of any hardcoded-arm class — Ticks 4-N retire
//!   `main.rs::VERBS` (class 1+2), `main.rs::at_operator` `@io/*` arms
//!   (class 3), `liquid.rs::dispatch_spec_property` strip_prefix arms
//!   (class 5), and `liquid.rs::pillar::dispatch` predicate arms
//!   (class 6) — one class per tick, each is a substrate delta.
//! - NOT a runtime dispatch surface — that already exists at
//!   `liquid.rs:960-970::pillar::dispatch` and is what Ticks 4-N
//!   structurally replace via compile-time trait dispatch.
//!
//! ## Composition anchors (LANDED)
//!
//! - `prism/prismqueer/src/bundle.rs:71-193` — Connes (A, H, D) tower.
//! - `prism/prismqueer/src/lib.rs:216-238` — `apply_h` action.
//! - `prism/projections/src/lib.rs:96` — `#[proc_macro] declaration`.
//! - `mirror/docs/specs/spectral-triple-grammar.md` (22.1KB canonical).
//! - `mirror/docs/specs/prism-core-as-spectral-triple.md` (22.3KB).
//! - `mirror/shards/epistemologic/spectral_triple.mirror` (substrate).
//!
//! Substrate-already-had-the-word: `spectral` is spoken at
//! `shards/spectral/*` (12 species), `shards/mirror/spectral.mirror`
//! (species), `shards/epistemologic/spectral_triple.mirror` (property),
//! and `prismqueer::{spectral_dimension, spectral_oid, spectral_uuid}`
//! (numerics + addressing). This module is the rust/ altitude echo of
//! that landed vocabulary — not a new mint.

include!(concat!(env!("OUT_DIR"), "/shard_manifest.rs"));

/// Accessor for the compile-time manifest of every
/// `shards/**/*.mirror` file present in the workspace at cargo build
/// time. Paths are `shards/<relative>` strings, sorted lexicographically
/// for byte-stable output.
///
/// See `rust/build.rs` for the emitter. See `#[cfg(test)] mod tests`
/// below for pipeline verification.
pub fn shard_paths() -> &'static [&'static str] {
    SHARD_PATHS
}

#[cfg(test)]
mod tests {
    use super::shard_paths;

    #[test]
    fn manifest_non_empty() {
        let paths = shard_paths();
        assert!(
            !paths.is_empty(),
            "SHARD_PATHS is empty — build.rs did not walk shards/. \
             Expected 300+ entries (per Taut 2026-07-23 shard census); \
             got 0. Either the shards/ tree is missing at ../shards/ \
             relative to rust/, or build.rs did not re-run.",
        );
    }

    #[test]
    fn manifest_contains_known_shards() {
        let paths = shard_paths();
        // Spot-check three shards from three different family-roots to
        // catch phantom-successful-walk (empty subtree yielding empty
        // vec) that manifest_non_empty alone would not detect.
        // Each verified present at 2026-07-23 09:58:28:
        //   shards/reality/subject.mirror       (26.1KB, 2026-07-22)
        //   shards/mirror/spec/system.mirror    (21.2KB, 2026-07-22)
        //   shards/magic/trick.mirror           ( 9.9KB, 2026-07-22)
        for expected in [
            "shards/reality/subject.mirror",
            "shards/mirror/spec/system.mirror",
            "shards/magic/trick.mirror",
        ] {
            assert!(
                paths.iter().any(|p| *p == expected),
                "expected shard `{}` in manifest; got {} entries. First 5: {:?}",
                expected,
                paths.len(),
                &paths.iter().take(5).collect::<Vec<_>>(),
            );
        }
    }

    #[test]
    fn manifest_is_sorted() {
        let paths = shard_paths();
        let mut sorted: Vec<&str> = paths.iter().copied().collect();
        sorted.sort();
        let original: Vec<&str> = paths.iter().copied().collect();
        assert_eq!(
            original, sorted,
            "SHARD_PATHS is not sorted; deterministic manifest broken. \
             build.rs must sort paths after enumeration.",
        );
    }
}
