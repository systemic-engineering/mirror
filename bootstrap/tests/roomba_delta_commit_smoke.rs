//! End2end empirical smoke: the ouroboros theorem discharged in one
//! test run. Alex 2026-07-15 verbatim (load-bearing): "I want the
//! empirical proof to really be a roomba walk across the graph. Bumping
//! into things. kintsugi resolving the fracture. And then the commit
//! being the DELTA of that resolution translated into @nl language and
//! of course as the blobs in the commit tree, actually committed to
//! disk."
//!
//! This test discharges each stage of the theorem against a temp
//! fixture directory:
//!   1. Walker walks a fixture file with stale-name-rot
//!   2. Detects the fracture (Fracture carrier emitted)
//!   3. Compose mended bytes
//!   4. Dispatch @epistemologic/reality/time.compare via apply_h::act
//!   5. Dispatch @io/fs.write via apply_h::act (mutation on disk)
//!   6. Verify the on-disk bytes are mended
//!
//! The full CLI-invoked round-trip (adding @io/git.commit real-blob
//! commit) is exercised manually via `mirror roomba --commit` per the
//! task brief's EMPIRICAL DEMONSTRATION section; this smoke test
//! discharges the substrate-composition steps that the CLI invocation
//! composes over.

use mirror::apply_h;
use mirror::roomba_fracture::{self, Fracture};
use std::path::PathBuf;

fn temp_fixture_dir(name: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!("mirror_roomba_delta_smoke_{}", name));
    let _ = std::fs::remove_dir_all(&p);
    std::fs::create_dir_all(p.join("bootstrap").join("src")).unwrap();
    p
}

#[test]
fn ouroboros_theorem_empirical_smoke() {
    // Stage 0: seed a fixture .rs file with a stale-name-rot in a
    // docblock. This is the "code that simplifies in front of your
    // eyes" input.
    let root = temp_fixture_dir("ouroboros_theorem");
    let fixture_path = root.join("bootstrap").join("src").join("stale.rs");
    let stale_content = "//! stale: mirror execute this action\n//! non-stale line\npub fn f() {}\n";
    std::fs::write(&fixture_path, stale_content).unwrap();

    // Stage 1 + 2: walker walks + detects fracture.
    let fractures = roomba_fracture::scan_bootstrap_src(&root);
    assert!(
        !fractures.is_empty(),
        "walker should detect at least one fracture in the seeded fixture"
    );
    let fracture: &Fracture = &fractures[0];
    assert_eq!(fracture.line_no, 1);
    assert_eq!(fracture.stale_name, "mirror execute");
    assert_eq!(fracture.canonical_name, "mirror beam act");
    assert!(fracture.context_snippet.contains("mirror execute"));

    // Stage 3: compose mended bytes.
    let mended = roomba_fracture::compose_mended_bytes(fracture).unwrap();
    assert!(
        mended.contains("mirror beam act this action"),
        "stage-3 mended bytes should contain the canonical form"
    );
    assert!(
        !mended.lines().next().unwrap().contains("mirror execute"),
        "stage-3 mended bytes should have replaced the stale form on target line"
    );

    // Stage 4: dispatch @epistemologic/reality/time.compare through
    // apply_h::act. Substrate-honest DELTA carrier per shard docblock.
    let before_snap = format!(
        "snapshot{{path={};bytes={};context_before=mirror execute}}",
        fixture_path.display(),
        stale_content.len(),
    );
    let after_snap = format!(
        "snapshot{{path={};bytes={};context_after=mirror beam act}}",
        fixture_path.display(),
        mended.len(),
    );
    let compare_verdict = apply_h::act(
        "@epistemologic/reality/time.compare".to_string(),
        vec![
            apply_h::Value { oid: before_snap.clone() },
            apply_h::Value { oid: after_snap.clone() },
        ],
    );
    let composed_delta = match compare_verdict {
        apply_h::Verdict::Partial(t) => {
            let found = t.located_opacity.iter().find(|(k, _)| {
                k == "@epistemologic/reality/time/delta"
            });
            assert!(
                found.is_some(),
                "stage-4 compare should surface delta via Transparency"
            );
            found.map(|(_, v)| v.clone()).unwrap()
        }
        other => panic!("stage-4 compare unexpected verdict: {:?}", other),
    };
    assert!(composed_delta.contains("delta{"));

    // Stage 5: dispatch @io/fs.write through apply_h::act. THIS is the
    // moment the code simplifies in front of your eyes.
    let write_verdict = apply_h::act(
        "@io/fs.write".to_string(),
        vec![
            apply_h::Value {
                oid: fixture_path.to_string_lossy().to_string(),
            },
            apply_h::Value { oid: mended.clone() },
        ],
    );
    match write_verdict {
        apply_h::Verdict::Pass => {}
        other => panic!("stage-5 @io/fs.write unexpected verdict: {:?}", other),
    }

    // Stage 6: verify the on-disk bytes are mended.
    let on_disk = std::fs::read_to_string(&fixture_path).unwrap();
    assert_eq!(
        on_disk, mended,
        "stage-6 on-disk bytes must equal the mended composition"
    );
    assert!(on_disk.contains("mirror beam act this action"));
    assert!(!on_disk.lines().next().unwrap().contains("mirror execute"));

    // Post-conditions: run the scanner again; expect zero fractures on
    // the fixture (fixed-point of the ouroboros — the mending held).
    let fractures_post = roomba_fracture::scan_bootstrap_src(&root);
    assert!(
        fractures_post.is_empty(),
        "post-mending: scanner should find zero fractures (fixed-point of the ouroboros); found: {:?}",
        fractures_post
    );

    // Cleanup.
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn no_fracture_path_returns_none() {
    // When the fixture has no stale-name-rot, the scanner returns empty.
    // This is the backward-compat path: roomba_commit falls back to
    // observation-only commit.
    let root = temp_fixture_dir("no_fracture");
    let fixture_path = root.join("bootstrap").join("src").join("clean.rs");
    let clean_content = "//! all clean here\n//! nothing to mend\npub fn f() {}\n";
    std::fs::write(&fixture_path, clean_content).unwrap();

    let fractures = roomba_fracture::scan_bootstrap_src(&root);
    assert!(
        fractures.is_empty(),
        "clean fixture should yield zero fractures"
    );

    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn io_fs_write_dispatches_through_act() {
    // Direct assertion on the resolver arm: @io/fs.write writes bytes
    // to the target path via std::fs::write at the @io boundary.
    let mut tmp = std::env::temp_dir();
    tmp.push("roomba_delta_smoke_iofswrite.txt");
    let _ = std::fs::remove_file(&tmp);

    let content = "the code simplifies in front of your eyes";
    let verdict = apply_h::act(
        "@io/fs.write".to_string(),
        vec![
            apply_h::Value {
                oid: tmp.to_string_lossy().to_string(),
            },
            apply_h::Value {
                oid: content.to_string(),
            },
        ],
    );
    match verdict {
        apply_h::Verdict::Pass => {}
        other => panic!("@io/fs.write dispatch unexpected verdict: {:?}", other),
    }
    let on_disk = std::fs::read_to_string(&tmp).unwrap();
    assert_eq!(on_disk, content);
    let _ = std::fs::remove_file(&tmp);
}

#[test]
fn compare_dispatch_returns_partial_with_delta_carrier() {
    // Direct assertion: @epistemologic/reality/time.compare returns a
    // Partial carrying the composed delta at key
    // `@epistemologic/reality/time/delta` per the MVP contract.
    let before = apply_h::Value { oid: "snap-before".to_string() };
    let after = apply_h::Value { oid: "snap-after".to_string() };
    let v = apply_h::act(
        "@epistemologic/reality/time.compare".to_string(),
        vec![before, after],
    );
    match v {
        apply_h::Verdict::Partial(t) => {
            let found = t.located_opacity.iter().any(|(k, v)| {
                k == "@epistemologic/reality/time/delta"
                    && v.contains("snap-before")
                    && v.contains("snap-after")
            });
            assert!(
                found,
                "expected delta carrier in Transparency; got {:?}",
                t.located_opacity
            );
        }
        other => panic!(
            "@epistemologic/reality/time.compare unexpected verdict: {:?}",
            other
        ),
    }
}
