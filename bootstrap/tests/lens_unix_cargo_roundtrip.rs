//! T8 — `@mirror/lens/unix` cargo round-trip.
//!
//! The substrate floor for `@mirror/lens/unix` landed at commit
//! `5aa5777` declaring `mount / path / read / write / stat` as the
//! abstract impedance surface; all bodies are `\` obligations. This
//! test verifies the OPERATIONAL discharge of those bodies through
//! `bootstrap::lens_unix::UnixLens` — specifically that cargo can read
//! a substrate-generated source crate through the lens AS IF the files
//! lived on disk.
//!
//! ## What this proves
//!
//! 1. A `Splinter` carrying a tiny Cargo crate (`Cargo.toml` +
//!    `src/lib.rs`) materializes into a Unix-shaped directory tree.
//! 2. The materialized tree has the path structure cargo expects.
//! 3. `cargo metadata --manifest-path <mount>/Cargo.toml` succeeds,
//!    parses the manifest, and reports the crate's package metadata.
//! 4. Cargo "doesn't know" it's reading through the lens — from its
//!    perspective it's just a directory on disk.
//!
//! If this test passes, the ouroboros pipeline closes its filesystem
//! face: `shards/spectral/*.mirror` -> `@code/metalogue/materialize` ->
//! `@mirror/store` shard -> **`@mirror/lens/unix`** -> `@io/cargo` ->
//! binary -> `@spectral` runtime. The lens is the load-bearing
//! impedance match per `docs/specs/spectral-runtime.md` §5.
//!
//! ## Why a real cargo invocation
//!
//! The substrate's spec specifically names cargo as the consumer
//! (`docs/specs/spectral-runtime.md` §5 step 5). A mock or in-memory
//! check would not exercise the kernel VFS path that cargo actually
//! uses to read `Cargo.toml`. The round-trip through `cargo metadata`
//! is the smallest cargo invocation that requires reading a manifest —
//! it doesn't compile anything, so it stays fast (~100ms in the warm
//! case) but does fully parse the manifest from disk.
//!
//! ## STOP-conditions checked
//!
//! - **Platform compatibility (STOP-condition 1, 4)**: the v0 floor
//!   materializes; no FUSE / kernel extensions involved. Works
//!   identically on macOS and Linux.
//! - **Substrate iteration gap (STOP-condition 2)**: the lens iterates
//!   the `Splinter`'s `Record` tree directly — the `Content::Record(map)`
//!   variant IS the substrate's iterability primitive at this altitude.
//!   No missing substrate action.
//! - **`mirror.spec` / Justfile (STOP-condition 3)**: this test does
//!   NOT touch Taut's territory. It invokes cargo directly via
//!   `std::process::Command` from inside the test; no spec / build-graph
//!   coordination needed.

use std::collections::BTreeMap;
use std::process::Command;

use mirror::crystallize::{Blake3, Content, FieldName, Splinter, Text};
use mirror::lens_unix::UnixLens;

/// Build a minimal cargo crate as a Splinter:
///
/// ```
/// {
///   "Cargo.toml":  "[package]\nname = \"hello\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[lib]\npath = \"src/lib.rs\"\n",
///   "src": {
///     "lib.rs":   "pub fn hello() -> &'static str { \"hi\" }\n"
///   }
/// }
/// ```
///
/// The shape mirrors what `@code/metalogue/materialize` would emit
/// for a tiny @spectral runtime crate. Two leaves, one nested Record.
fn minimal_cargo_crate() -> Splinter<Blake3> {
    let cargo_toml = "[package]\nname = \"hello\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[lib]\npath = \"src/lib.rs\"\n";
    let lib_rs = "pub fn hello() -> &'static str { \"hi\" }\n";

    let mut src: BTreeMap<FieldName, Splinter<Blake3>> = BTreeMap::new();
    src.insert(
        FieldName::new("lib.rs").expect("valid field name"),
        Splinter::new(Content::Text(Text::new(lib_rs))),
    );

    let mut root: BTreeMap<FieldName, Splinter<Blake3>> = BTreeMap::new();
    root.insert(
        FieldName::new("Cargo.toml").expect("valid field name"),
        Splinter::new(Content::Text(Text::new(cargo_toml))),
    );
    root.insert(
        FieldName::new("src").expect("valid field name"),
        Splinter::new(Content::Record(src)),
    );

    Splinter::new(Content::Record(root))
}

#[test]
fn cargo_reads_substrate_generated_crate_through_unix_lens() {
    // Step 1: build the substrate-side shard (a Splinter carrying the
    // crate's source).
    let shard = minimal_cargo_crate();

    // Step 2: mount it. The lens materializes the Record tree into a
    // tempdir. This is the OS-level realization of the substrate's
    // `mount(shard) -> imperfect` action.
    let lens = UnixLens::mount(&shard).expect("mount succeeds");

    // Step 3: compute the Cargo.toml path via the lens's `path` action.
    let manifest_path = lens.path(&["Cargo.toml"]);
    assert!(
        manifest_path.exists(),
        "Cargo.toml must be materialized at {}",
        manifest_path.display()
    );

    // Step 4: round-trip. Invoke `cargo metadata` against the mounted
    // manifest. Cargo does NOT know it's reading through a lens — from
    // its perspective the tempdir IS a normal directory.
    //
    // `--no-deps`: don't fetch the registry; the crate is standalone.
    // `--format-version 1`: stable JSON output (parseable if we want
    // to assert on it, though for the floor we just check the exit
    // status — success means cargo successfully read the manifest).
    let output = Command::new("cargo")
        .args([
            "metadata",
            "--no-deps",
            "--format-version",
            "1",
            "--manifest-path",
        ])
        .arg(&manifest_path)
        .output()
        .expect("cargo invocation succeeds (cargo must be on PATH)");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "cargo metadata must succeed.\n--- stdout ---\n{}\n--- stderr ---\n{}",
        stdout,
        stderr
    );

    // Step 5: verify cargo actually read OUR crate. The metadata
    // output is JSON; assert the package name appears in it. This
    // closes the round-trip: bytes from the Splinter -> bytes on disk
    // -> bytes parsed by cargo -> bytes in cargo's metadata output.
    assert!(
        stdout.contains("\"name\":\"hello\"") || stdout.contains("\"name\": \"hello\""),
        "cargo metadata output must mention package name 'hello'.\nstdout:\n{}",
        stdout
    );
}

/// A second smoke test demonstrating that the lens's `path` /  `read` /
/// `stat` triple satisfy the substrate's expectations independently of
/// cargo — useful when debugging without paying cargo's startup cost.
#[test]
fn lens_surface_round_trips_without_cargo() {
    let shard = minimal_cargo_crate();
    let lens = UnixLens::mount(&shard).expect("mount succeeds");

    // path + read of a top-level file.
    let manifest_path = lens.path(&["Cargo.toml"]);
    let manifest_bytes = lens.read(&manifest_path).expect("read manifest");
    let manifest_str = std::str::from_utf8(&manifest_bytes).expect("utf-8");
    assert!(manifest_str.contains("name = \"hello\""));

    // path + read of a nested file.
    let lib_path = lens.path(&["src", "lib.rs"]);
    let lib_bytes = lens.read(&lib_path).expect("read lib.rs");
    let lib_str = std::str::from_utf8(&lib_bytes).expect("utf-8");
    assert!(lib_str.contains("pub fn hello"));

    // stat verdicts match the substrate's `verdict` shape.
    use mirror::lens_unix::Verdict;
    assert_eq!(lens.stat(&manifest_path), Verdict::Pass);
    assert_eq!(lens.stat(&lib_path), Verdict::Pass);
    let missing = lens.path(&["does", "not", "exist"]);
    assert!(matches!(lens.stat(&missing), Verdict::Failure(_)));
}
