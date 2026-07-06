//! N3 TICK 1 RED — `cmd_kintsugi_spec` verdict cache integration.
//!
//! Per Taut scout report + N1 + N2 closure: `cmd_kintsugi_spec` at
//! `bootstrap/src/lib.rs:~1189` currently walks every target on every
//! invocation and spawns `cargo <check>` cold. Zero cross-invocation cache.
//!
//! N1 (`2857fb1`) landed `verdict_is_content_addressed` predicate at
//! `@epistemologic/property/`. N2 (`0a72c42`) landed `@mirror/store/action_cache`
//! species with cache_read + cache_write + cache_exists actions.
//!
//! **This tick (N3) wires the Rust side**:
//! - Before dispatching cargo for a target: compute `(spec_oid, target_oid,
//!   inputs_oid)` from current substrate state; call `cache_read`; if
//!   success(verdict), return the cached verdict
//! - After computing a fresh verdict: call `cache_write(spec_oid, target_oid,
//!   inputs_oid, verdict)` to memoize
//! - `Crystallizations<H>` dispatch table currently disconnected from
//!   `cmd_kintsugi_spec`; wire it
//!
//! **Business-observable outcome**: when N3 lands, the 13-minute pre-commit
//! hook falls for warm-cache commits. Cold-cache (first invocation OR after
//! substrate change touching all targets) still runs cargo. Warm-cache
//! (subsequent invocations with unchanged inputs) returns cached verdicts in
//! O(cache lookup) not O(cargo).
//!
//! **Test shape**: integration tests at Rust altitude, not shard-property
//! tests. Each test exercises real `cmd_kintsugi_spec` behavior against
//! a controlled fixture.

use std::path::PathBuf;

// === Fixture support ===

/// Path to the mirror.spec dogfood instance (used as test fixture).
fn mirror_spec_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent")
        .join("mirror.spec")
}

// === T1: baseline — cold-cache path still works ===
//
// N3 must not break existing cold-cache behavior. First call against a fresh
// substrate state must dispatch cargo and return a valid verdict.

#[test]
fn t01_cold_cache_dispatches_cargo_and_returns_verdict() {
    // Placeholder integration test: N3 exposes a `cmd_kintsugi_spec_with_cache`
    // (or refactors `cmd_kintsugi_spec`) entry point that returns a verdict
    // whether cached or fresh. The test verifies the cold path invokes cargo.
    //
    // Marker assertion — fails until N3 lands the cache-aware entry point.
    assert!(
        mirror::verdict_cache_cold_path_landed(),
        "T1 (RED): N3 must expose a cache-aware kintsugi_spec entry point. Cold-cache path must dispatch cargo and return a valid verdict without regression. See docs/audits/2026-07-06-seam-phase-d-n2-tick-1-mirror-store-action-cache.md for N2 substrate-decl."
    );
}

// === T2: warm cache hits return cached verdict ===
//
// Second call with same inputs must NOT invoke cargo. Verdict comes from
// action_cache.cache_read.

#[test]
fn t02_warm_cache_returns_memoized_verdict_without_cargo() {
    assert!(
        mirror::verdict_cache_warm_path_landed(),
        "T2 (RED): warm-cache path must return the memoized verdict via action_cache.cache_read without invoking cargo. Second invocation with same (spec_oid, target_oid, inputs_oid) key must be O(cache lookup) not O(cargo)."
    );
}

// === T3: input-key discrimination — different inputs → fresh verdict ===
//
// Cache MUST be keyed on (spec_oid, target_oid, inputs_oid). Changing any
// dimension must produce a cache miss and fresh verdict.

#[test]
fn t03_input_change_forces_cache_miss_and_fresh_verdict() {
    assert!(
        mirror::verdict_cache_input_key_discriminates(),
        "T3 (RED): cache key MUST discriminate independently on (spec_oid, target_oid, inputs_oid). Changing any dimension must produce a cache MISS and fresh cargo dispatch."
    );
}

// === T4: cache_write is idempotent ===
//
// Two writes with the same (spec_oid, target_oid, inputs_oid, verdict) must
// be no-ops on the second call. Content-addressed by construction.

#[test]
fn t04_cache_write_is_idempotent_by_content_address() {
    assert!(
        mirror::verdict_cache_write_is_idempotent(),
        "T4 (RED): cache_write MUST be idempotent by content-address per N1 predicate + N2 action_cache discipline. Writing same key+verdict twice → same OID → no-op on second write."
    );
}

// === T5: cache persistence across process boundary ===
//
// Cache lives in @mirror/store (crystals in the store DAG); NOT in-process.
// Two separate cmd_kintsugi_spec invocations against the same substrate
// state must hit the cache on the second run.

#[test]
fn t05_cache_persists_across_process_boundary_via_mirror_store() {
    assert!(
        mirror::verdict_cache_persists_across_processes(),
        "T5 (RED): cache state MUST live in @mirror/store (crystals in the store DAG), NOT in-process. Two separate process invocations against the same substrate state must share the cache."
    );
}

// === T6: Crystallizations<H> dispatch table connected ===
//
// Per Taut scout: `Crystallizations<H>` in `crystallize.rs:520+` is wired
// but disconnected from `cmd_kintsugi_spec`. N3 connects them.

#[test]
fn t06_crystallizations_dispatch_table_connected_to_cmd_kintsugi_spec() {
    assert!(
        mirror::crystallizations_dispatch_wired_into_cmd_kintsugi_spec(),
        "T6 (RED): the `Crystallizations<H>` dispatch table at `bootstrap/src/crystallize.rs:520+` must be wired into `cmd_kintsugi_spec` at `bootstrap/src/lib.rs:~1189`. Taut scout flagged the disconnection as a wired-but-disconnected gap."
    );
}
