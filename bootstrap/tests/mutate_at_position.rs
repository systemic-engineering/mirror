//! Bridge α RED — position-aware source-file mutation via `@io/fs.mutate_at`.
//!
//! Autopoietic loop step 7 discharge per Alex 2026-07-15 adjudication
//! (`7181f5c`): source_position placement ratified at @glass extension;
//! @io/fs.mutate_at species mint at shards/io/fs.mirror + resolver arm at
//! bootstrap/src/apply_h.rs. The bridge that projects crystallized
//! inferences back to source files at exactly the byte-range where the
//! `\` fracture originated.
//!
//! RED-phase expectation: no `@io/fs.mutate_at` resolver arm exists;
//! `apply_h::act` returns `Verdict::Partial(Transparency::opaque)` per the
//! unrecognized-action fallback. GREEN-phase expectation: the resolver
//! arm lands + splices replacement at [byte_offset, byte_offset +
//! byte_length) + returns Pass + preserves surrounding bytes + writes
//! POSIX-atomically.
//!
//! [substrate-floor:@io-boundary] Bridge α (second of three). Audit-cite
//! `docs/audits/2026-07-15-seam-autopoietic-loop-phase-d.md` (55dbf20).
//! Signed-off-by: Seam.
//!
//! Reference: docs/specs/autopoietic-inference-loop.md §3.7 (mutate_at)
//! + §4.2 (Bridge α) + §8.2 (Tick 2). LOC ceiling per §4.4: ~50 Rust
//! + ~50 mirror for this bridge.

use mirror::apply_h::{act, Value, Verdict};
use std::path::PathBuf;

fn scratch_root(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "mirror-bridge-alpha-{}-{}",
        tag,
        std::process::id()
    ));
    if dir.exists() {
        let _ = std::fs::remove_dir_all(&dir);
    }
    std::fs::create_dir_all(&dir).expect("scratch root must be creatable");
    dir
}

/// Encode a source_position as the OID string the @io/fs.mutate_at
/// resolver parses. Mirrors the substrate-decl'd source_position record
/// (§3.7): only byte_offset + byte_length are load-bearing for splice
/// precision; file/line/col are informational passthroughs.
fn position(byte_offset: usize, byte_length: usize) -> String {
    format!("byte_offset={},byte_length={}", byte_offset, byte_length)
}

#[test]
fn mutate_at_splices_replacement_at_byte_range() {
    let root = scratch_root("splice");
    let file = root.join("target.txt");
    std::fs::write(&file, b"hello world!").expect("seed write");

    // Replace "world" (offset 6, length 5) with "REED" (length 4).
    // Post-condition: file contains "hello REED!".
    let verdict = act(
        "@io/fs.mutate_at".to_string(),
        vec![
            Value {
                oid: file.to_string_lossy().to_string(),
            },
            Value { oid: position(6, 5) },
            Value {
                oid: "REED".to_string(),
            },
        ],
    );

    assert!(
        matches!(verdict, Verdict::Pass),
        "@io/fs.mutate_at should Pass on well-formed splice; got {:?}",
        verdict
    );

    let after = std::fs::read_to_string(&file).expect("post-mutation read");
    assert_eq!(after, "hello REED!", "splice MUST preserve surrounding bytes");

    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn mutate_at_preserves_bytes_before_and_after_splice_range() {
    let root = scratch_root("preserve");
    let file = root.join("multiline.rs");
    let seed = b"fn foo() {\n    let x = OLDNAME;\n}\n";
    std::fs::write(&file, seed).expect("seed write");

    // Byte layout of the seed (35 bytes total):
    //   [0..11)  "fn foo() {\n"
    //   [11..15) "    "      (4-space indent)
    //   [15..18) "let"
    //   [18..19) " "
    //   [19..20) "x"
    //   [20..21) " "
    //   [21..22) "="
    //   [22..23) " "
    //   [23..30) "OLDNAME"   ← splice target
    //   [30..31) ";"
    //   [31..32) "\n"
    //   [32..33) "}"
    //   [33..34) "\n"
    //
    // Replace "OLDNAME" (offset 23, length 7) with "new_name" (length 8).
    // Length change: +1 byte. Post-condition: surrounding bytes byte-
    // identical; total length = seed.len() + 1.
    let verdict = act(
        "@io/fs.mutate_at".to_string(),
        vec![
            Value {
                oid: file.to_string_lossy().to_string(),
            },
            Value { oid: position(23, 7) },
            Value {
                oid: "new_name".to_string(),
            },
        ],
    );

    assert!(
        matches!(verdict, Verdict::Pass),
        "expected Pass on length-changing splice; got {:?}",
        verdict
    );

    let after = std::fs::read(&file).expect("post-mutation read");
    let expected = b"fn foo() {\n    let x = new_name;\n}\n";
    assert_eq!(
        after,
        expected,
        "surrounding bytes must be byte-identical; only [21,28) replaced"
    );

    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn mutate_at_fails_cleanly_on_out_of_range_position() {
    let root = scratch_root("oor");
    let file = root.join("tiny.txt");
    std::fs::write(&file, b"abc").expect("seed write");

    // Position [10, 15) is well past end of 3-byte file. MUST fail
    // without corrupting the file (POSIX-atomic invariant).
    let verdict = act(
        "@io/fs.mutate_at".to_string(),
        vec![
            Value {
                oid: file.to_string_lossy().to_string(),
            },
            Value { oid: position(10, 5) },
            Value {
                oid: "XYZ".to_string(),
            },
        ],
    );

    assert!(
        matches!(verdict, Verdict::Fail(_)),
        "out-of-range position MUST Fail (not Pass, not Partial); got {:?}",
        verdict
    );

    // File contents preserved (no partial mutation state).
    let after = std::fs::read(&file).expect("post-fail read");
    assert_eq!(after, b"abc", "failed mutation MUST NOT corrupt file");

    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn mutate_at_supports_equal_length_replacement_lossless() {
    let root = scratch_root("lossless");
    let file = root.join("eqlen.txt");
    std::fs::write(&file, b"aaaaXXXXbbbb").expect("seed write");

    // Replace XXXX (offset 4, length 4) with YYYY (length 4).
    // L(ϕ) = 0 per spec §5.5.5 REED-INLINE-6 (equal-length case).
    let verdict = act(
        "@io/fs.mutate_at".to_string(),
        vec![
            Value {
                oid: file.to_string_lossy().to_string(),
            },
            Value { oid: position(4, 4) },
            Value {
                oid: "YYYY".to_string(),
            },
        ],
    );

    assert!(matches!(verdict, Verdict::Pass), "equal-length splice should Pass");
    let after = std::fs::read(&file).expect("post-mutation read");
    assert_eq!(after, b"aaaaYYYYbbbb", "lossless splice: surrounding bytes intact");
    assert_eq!(after.len(), 12, "file length preserved for equal-length splice");

    let _ = std::fs::remove_dir_all(&root);
}
