//! Phase A RED — restart-intensity-shard.
//!
//! Per Seam audit `3746197` §Proposed-next-loop: land
//! `shards/spectral/restart_intensity.mirror` as the substrate-decl form
//! of BEAM's `max_restarts / max_seconds` circuit breaker.
//!
//! Math floor: `docs/math/supervisor/emergent-supervision-from-geometry.md`
//! §5 (bounded reductions ↔ restart intensity). Signature:
//!
//!   type restart_intensity = { budget: ref, period: duration }
//!
//! Composes candidate #147 (formerly #136; renumbered at `ff00ec5`):
//! restart intensity IS `@spawn ≤ @loop` budget at supervision altitude.
//! `budget: ref` per `[[feedback-no-bare-types]]` + `[[feedback-explicit-
//! over-implicit]]`. `period: duration` reuses landed substrate time
//! carrier per emergent-supervision §5.3.
//!
//! **RED phase**: `shards/spectral/restart_intensity.mirror` does not
//! yet exist. Text-check tests fail on file absence and type-declaration
//! absence. Discipline mirrors `loop_parent_family_lift.rs` (Reed
//! `327ff74` 2026-07-02): smallest-tick RED that verifies substrate-decl
//! landed without over-scoping into grammar-compilation altitude.
//!
//! **Composition claim (DEFERRED per feedback-composition-claims-need-
//! empirical-test)**: whether the (budget, period) pair captures BEAM's
//! `{max_restarts, max_seconds}` circuit-breaker semantics under a real
//! restart storm remains empirically unwitnessed. Follows Mara §5.5
//! H4 flag; test doesn't assert semantic equivalence, only carrier
//! presence.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_shard() -> String {
    let path = repo_root().join("shards/spectral/restart_intensity.mirror");
    std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "read shards/spectral/restart_intensity.mirror at {:?}: {}",
            path, e
        )
    })
}

#[test]
fn restart_intensity_shard_declares_type() {
    let content = read_shard();
    assert!(
        content.contains("type restart_intensity"),
        "@spectral/restart_intensity must declare `type restart_intensity` per Mara emergent-supervision §5 + Seam audit 3746197"
    );
}

#[test]
fn restart_intensity_shard_declares_budget_field() {
    let content = read_shard();
    assert!(
        content.contains("budget:"),
        "restart_intensity type must declare `budget:` field (typed ref, not bare u32) per [[feedback-no-bare-types]]"
    );
}

#[test]
fn restart_intensity_shard_declares_period_field() {
    let content = read_shard();
    assert!(
        content.contains("period:"),
        "restart_intensity type must declare `period:` field (duration carrier) per emergent-supervision §5.3"
    );
}

#[test]
fn restart_intensity_shard_uses_ref_carrier_for_budget() {
    let content = read_shard();
    assert!(
        content.contains("budget: ref") || content.contains("budget:  ref"),
        "budget field must carry `ref` typed carrier per [[feedback-no-bare-types]] + [[feedback-explicit-over-implicit]] (Alex 2026-07-02): no bare u32/u64"
    );
}

#[test]
fn restart_intensity_shard_uses_duration_carrier_for_period() {
    let content = read_shard();
    assert!(
        content.contains("period: duration") || content.contains("period:  duration"),
        "period field must carry `duration` typed carrier per emergent-supervision §5.3 substrate time discipline"
    );
}
