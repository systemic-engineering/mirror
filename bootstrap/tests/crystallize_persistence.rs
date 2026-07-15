//! Bridge γ RED — crystallization persistence at `.mirror/objects/<OID>`.
//!
//! Autopoietic loop step 6 discharge per Alex 2026-07-15 adjudication of
//! post-Seam-Phase-D residues (b82945b + 7181f5c). The existing
//! `apply_h::crystallize` combinator content-addresses the before/after
//! `ouroboros_state` into a `BenchCrystal { before_oid, after_oid,
//! crystal_oid }` but does NOT persist to disk. Bridge γ extends the
//! surface with `crystallize_and_persist(before, after, root)` which
//! writes the crystal payload bytes to `<root>/.mirror/objects/<crystal_oid>`.
//!
//! RED-phase expectation: `apply_h::crystallize_and_persist` does not
//! exist yet — this test does not compile. GREEN-phase expectation: the
//! function lands + persists the payload + the round-trip law holds
//! (`hash_tagged("bench_crystal", read(path(oid))) == oid`).
//!
//! [substrate-floor:@io-boundary] Bridge γ (first of three). Audit-cite
//! `docs/audits/2026-07-15-seam-autopoietic-loop-phase-d.md` (55dbf20).
//! Signed-off-by: Seam.
//!
//! Reference: docs/specs/autopoietic-inference-loop.md §4.1 (Bridge γ)
//! + §8.1 (Tick 1). LOC ceiling per §4.4: ~50 Rust for this bridge.

use mirror::apply_h::{crystallize_and_persist, OuroborosState};
use mirror::hash::hash_tagged;
use std::path::PathBuf;

/// Per-test scratch root. Isolated per-name to avoid cross-test races when
/// `cargo test` runs in parallel. Removed at test entry so the FS state is
/// deterministic across re-runs; the test also does best-effort cleanup at exit.
fn scratch_root(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "mirror-bridge-gamma-{}-{}",
        tag,
        std::process::id()
    ));
    if dir.exists() {
        let _ = std::fs::remove_dir_all(&dir);
    }
    std::fs::create_dir_all(&dir).expect("scratch root must be creatable");
    dir
}

#[test]
fn crystallize_persists_content_addressed_payload_at_mirror_objects() {
    let root = scratch_root("payload");
    let before = OuroborosState {
        oid: "sha256-before-payload".to_string(),
    };
    let after = OuroborosState {
        oid: "sha256-after-payload".to_string(),
    };

    let crystal = crystallize_and_persist(before.clone(), after.clone(), &root)
        .expect("bridge γ persistence should succeed on writable scratch root");

    // Assertion 1 — the file exists at the substrate's declared location.
    // Per spec §4.1 + §5.5.3 crystal-ref-transport: `.mirror/objects/<OID>` IS
    // the substrate's content-addressed-storage path convention.
    let expected_path = root.join(".mirror").join("objects").join(&crystal.crystal_oid);
    assert!(
        expected_path.exists(),
        "crystal payload MUST be persisted at {} (bridge γ landing)",
        expected_path.display()
    );

    // Assertion 2 — the payload IS the pre-hash byte stream.
    // Per apply_h::crystallize (§1.7): pre-hash bytes = `before:<oid>|after:<oid>`.
    // Persisting these bytes preserves the content-addressed invariant.
    let payload_bytes = std::fs::read(&expected_path).expect("payload must be readable");
    let expected_payload = format!("before:{}|after:{}", before.oid, after.oid);
    assert_eq!(
        payload_bytes,
        expected_payload.as_bytes(),
        "payload bytes MUST be pre-hash content `before:<oid>|after:<oid>`"
    );

    // Assertion 3 — the content-address round-trip law holds.
    // Classical CAS invariant: hash(read(cas.path(oid))) == oid. Without this,
    // the crystal is not content-addressed and bridge γ is a lie.
    let re_hash = hash_tagged("bench_crystal", &payload_bytes);
    assert_eq!(
        re_hash, crystal.crystal_oid,
        "round-trip: hash(read(path(oid))) MUST equal oid — CAS invariant"
    );

    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn crystallize_persistence_is_idempotent_for_identical_state() {
    let root = scratch_root("idempotent");
    let before = OuroborosState {
        oid: "sha256-a".to_string(),
    };
    let after = OuroborosState {
        oid: "sha256-b".to_string(),
    };

    let c1 = crystallize_and_persist(before.clone(), after.clone(), &root)
        .expect("first persistence should succeed");
    let c2 = crystallize_and_persist(before, after, &root)
        .expect("second persistence should succeed — same OID, same path, safe re-write");

    // Same inputs → same OID → same file path → both calls succeed.
    // Idempotency: the autopoietic loop MUST NOT fail on re-persisting a
    // crystal that was previously produced (cache-hit / re-entry from a
    // second tournament pass per spec §8.5 empirical-proof scenario).
    assert_eq!(
        c1.crystal_oid, c2.crystal_oid,
        "identical inputs MUST produce identical crystal_oid"
    );

    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn crystallize_persistence_creates_objects_directory_on_demand() {
    let root = scratch_root("mkdir");
    // The scratch root exists but `.mirror/objects/` does not — bridge γ MUST
    // create the directory on demand (parallel to git's `mkdir -p .git/objects`
    // behavior; parallel to @mirror/store.write_crystal's on-demand-mkdir per
    // shards/mirror/store.mirror docblock).
    let objects_dir = root.join(".mirror").join("objects");
    assert!(!objects_dir.exists(), "pre-condition: objects dir absent");

    let before = OuroborosState {
        oid: "sha256-mkdir-before".to_string(),
    };
    let after = OuroborosState {
        oid: "sha256-mkdir-after".to_string(),
    };

    let crystal = crystallize_and_persist(before, after, &root)
        .expect("bridge γ MUST create objects dir on demand");

    assert!(
        objects_dir.exists(),
        "post-condition: objects dir created at {}",
        objects_dir.display()
    );
    assert!(
        objects_dir.join(&crystal.crystal_oid).exists(),
        "post-condition: crystal payload written"
    );

    let _ = std::fs::remove_dir_all(&root);
}
