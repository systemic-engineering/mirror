//! P3 RED — `mirror init <path>` contract per Mara spec §4.7.
//!
//! Per Mara's mirror-init spec (`docs/specs/mirror-init.md`, commits
//! `fe215bd` → `14dd043`, ~1208 lines) + Seam's audit (commit `8392ab5`;
//! 0C/3S/8L/12✓) + Reed's Cargo-edge GREEN (commit `6b36808`; bridge
//! verified; R2 = 0 bytes Δ empirically): mirror init is the bridge
//! command that makes the declared substrate operational.
//!
//! These tests assert the envelope shape contract per spec §4.7. They
//! fail against the cmd_init stub in `lib.rs` until the GREEN tick wires
//! the real composition (NamespacedGitStore + project::project +
//! per-file Splinter + set_ref("HEAD", root)).
//!
//! Pattern mirrors `bootstrap/tests/spawn.rs` + `bootstrap/tests/recall.rs`:
//! in-process dispatch via `mirror::kintsugi_main_in`.

use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::Output;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn run_init(args: &[&str]) -> Output {
    let mut argv: Vec<String> = vec!["mirror".to_string(), "init".to_string()];
    argv.extend(args.iter().map(|s| s.to_string()));
    let out = mirror::kintsugi_main_in(&argv, &repo_root());
    Output {
        status: std::process::ExitStatus::from_raw(out.exit_code << 8),
        stdout: out.stdout,
        stderr: out.stderr,
    }
}

fn parse_envelope(stdout: &[u8]) -> serde_json::Value {
    let s = String::from_utf8_lossy(stdout);
    serde_json::from_str(s.trim()).unwrap_or_else(|e| {
        panic!(
            "mirror init stdout must be valid JSON; got:\n{}\nerr: {}",
            s, e
        )
    })
}

#[test]
fn init_exits_zero_on_valid_repo() {
    // P4 GREEN: the original P3 test used `.` against the mirror repo
    // itself; once the wire-up landed, that would index 800+ files into
    // the real `.git/mirror/` and slow the test surface. Switch to the
    // fixture pattern (same as the P4 RED tests) to keep the envelope-
    // shape assertion fast + sandboxed.
    let fixture = init_fixture_repo();
    let path_arg = fixture.path().to_str().expect("utf8 path").to_string();
    let out = run_init_in_fixture(fixture.path(), &[&path_arg]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "init must exit 0 on valid repo; got {:?}\nstderr: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Envelope must carry the 9 contract keys per Mara spec §4.7.
#[test]
fn init_envelope_carries_contract_keys() {
    let fixture = init_fixture_repo();
    let path_arg = fixture.path().to_str().expect("utf8 path").to_string();
    let out = run_init_in_fixture(fixture.path(), &[&path_arg]);
    let envelope = parse_envelope(&out.stdout);
    for key in [
        "spec_version",
        "operation",
        "repo",
        "store",
        "indexed",
        "bytes_total",
        "root_oid",
        "hooks_installed",
        "verdict",
    ] {
        assert!(
            envelope.get(key).is_some(),
            "init envelope must carry '{}' per spec §4.7; got: {}",
            key,
            envelope
        );
    }
}

/// envelope.operation must equal "init".
#[test]
fn init_operation_equals_init() {
    let fixture = init_fixture_repo();
    let path_arg = fixture.path().to_str().expect("utf8 path").to_string();
    let out = run_init_in_fixture(fixture.path(), &[&path_arg]);
    let envelope = parse_envelope(&out.stdout);
    assert_eq!(
        envelope["operation"].as_str().unwrap_or(""),
        "init",
        "envelope.operation must == 'init'; got: {}",
        envelope["operation"]
    );
}

/// envelope.spec_version must == "v0.1.0" (matches spawn/recall envelope vocabulary).
#[test]
fn init_spec_version_v0_1_0() {
    let fixture = init_fixture_repo();
    let path_arg = fixture.path().to_str().expect("utf8 path").to_string();
    let out = run_init_in_fixture(fixture.path(), &[&path_arg]);
    let envelope = parse_envelope(&out.stdout);
    assert_eq!(
        envelope["spec_version"].as_str().unwrap_or(""),
        "v0.1.0",
        "envelope.spec_version must match round-trip vocabulary; got: {}",
        envelope["spec_version"]
    );
}

/// hooks_installed defaults to false (no --install-hooks flag).
#[test]
fn init_hooks_default_false() {
    let fixture = init_fixture_repo();
    let path_arg = fixture.path().to_str().expect("utf8 path").to_string();
    let out = run_init_in_fixture(fixture.path(), &[&path_arg]);
    let envelope = parse_envelope(&out.stdout);
    assert_eq!(
        envelope["hooks_installed"].as_bool(),
        Some(false),
        "envelope.hooks_installed must default false without --install-hooks; got: {}",
        envelope["hooks_installed"]
    );
}

#[test]
fn init_exits_non_zero_on_missing_repo() {
    let out = run_init(&["/nonexistent/mirror-init-test"]);
    // RED: stub does not validate the repo path. GREEN must reject
    // non-existent paths with a typed error.
    assert_ne!(
        out.status.code(),
        Some(0),
        "init must exit non-zero when repo path is missing"
    );
}

#[test]
fn init_exits_non_zero_on_missing_arg() {
    let argv = vec!["mirror".to_string(), "init".to_string()];
    let out = mirror::kintsugi_main_in(&argv, &repo_root());
    assert_ne!(
        out.exit_code, 0,
        "init without args must exit non-zero (usage message)"
    );
}

// ============================================================================
// P4 RED — empirical composition tests per Mara discharge map (`1de09a9`).
//
// These tests fixture a real git repo with known files, then invoke
// `mirror init` and assert against the discharge map's three load-bearing
// outputs:
//
//   1. `.git/mirror/` (the namespaced store directory) exists with
//      objects/ + refs/ subdirs (NamespacedGitStore::open produced them).
//   2. `.git/mirror/refs/HEAD` resolves to a non-stub root_oid (the
//      composition wrote set_ref("HEAD", root_oid)).
//   3. The envelope's indexed/bytes_total/root_oid/store fields reflect
//      the real composition (not stub markers).
//
// These RED tests FAIL against the P3 stub envelope because the stub
// writes nothing to disk and emits placeholder values.
// ============================================================================

/// Owned fixture directory — auto-cleans on drop. tempfile is not a
/// dev-dep of this crate (per brief: do not touch Cargo.toml); the
/// fixture rolls its own scratch dir under `std::env::temp_dir()`.
struct FixtureDir(PathBuf);

impl FixtureDir {
    fn path(&self) -> &std::path::Path {
        &self.0
    }
}

impl Drop for FixtureDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn fresh_fixture_dir(label: &str) -> FixtureDir {
    use std::sync::atomic::{AtomicU64, Ordering};
    static N: AtomicU64 = AtomicU64::new(0);
    let pid = std::process::id();
    let seq = N.fetch_add(1, Ordering::SeqCst);
    let base = std::env::temp_dir().join(format!("mirror-init-{}-{}-{}", label, pid, seq));
    // Idempotent fresh start.
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).expect("create fixture dir");
    FixtureDir(base)
}

fn init_fixture_repo() -> FixtureDir {
    let dir = fresh_fixture_dir("repo");
    // Initialize a real git repo; NamespacedGitStore needs .git/ to anchor.
    // Shell out to `git init` (no git2 dev-dep per brief: do not touch Cargo.toml).
    let status = std::process::Command::new("git")
        .arg("init")
        .arg("--quiet")
        .current_dir(dir.path())
        .status()
        .expect("git init");
    assert!(status.success(), "git init failed in fixture");
    // Two files of known content for deterministic indexing.
    std::fs::write(dir.path().join("hello.txt"), b"hello mirror init\n").expect("write hello.txt");
    std::fs::create_dir_all(dir.path().join("sub")).expect("mkdir sub");
    std::fs::write(dir.path().join("sub/deep.md"), b"# deep\n").expect("write deep.md");
    // `git add -A` populates the index — the v0 walk reads tracked
    // working-tree paths via `git ls-files`.
    let status = std::process::Command::new("git")
        .arg("add")
        .arg("-A")
        .current_dir(dir.path())
        .status()
        .expect("git add -A");
    assert!(status.success(), "git add -A failed in fixture");
    dir
}

fn run_init_in_fixture(fixture: &std::path::Path, args: &[&str]) -> Output {
    let mut argv: Vec<String> = vec!["mirror".to_string(), "init".to_string()];
    argv.extend(args.iter().map(|s| s.to_string()));
    let out = mirror::kintsugi_main_in(&argv, fixture);
    Output {
        status: std::process::ExitStatus::from_raw(out.exit_code << 8),
        stdout: out.stdout,
        stderr: out.stderr,
    }
}

/// P4 RED: `.git/mirror/` directory exists after init (NamespacedGitStore::open).
#[test]
fn init_creates_namespaced_store_directory() {
    let fixture = init_fixture_repo();
    let path_arg = fixture.path().to_str().expect("utf8 path").to_string();
    let out = run_init_in_fixture(fixture.path(), &[&path_arg]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "init must exit 0; stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let store_dir = fixture.path().join(".git").join("mirror");
    assert!(
        store_dir.exists(),
        ".git/mirror/ must exist after init; not found at {:?}",
        store_dir
    );
    assert!(
        store_dir.join("objects").exists(),
        ".git/mirror/objects/ must exist after init"
    );
    assert!(
        store_dir.join("refs").exists(),
        ".git/mirror/refs/ must exist after init"
    );
}

/// P4 RED: `.git/mirror/refs/HEAD` resolves to a non-stub root_oid
/// (set_ref("HEAD", root_oid) was called per discharge map).
#[test]
fn init_sets_head_ref_to_root_oid() {
    let fixture = init_fixture_repo();
    let path_arg = fixture.path().to_str().expect("utf8 path").to_string();
    let out = run_init_in_fixture(fixture.path(), &[&path_arg]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "init must exit 0; stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let head_ref = fixture.path().join(".git/mirror/refs/HEAD");
    assert!(
        head_ref.exists(),
        ".git/mirror/refs/HEAD must exist after init; not found at {:?}",
        head_ref
    );
    let head_content = std::fs::read_to_string(&head_ref).expect("read HEAD");
    let head_trim = head_content.trim();
    assert!(
        !head_trim.is_empty(),
        "HEAD ref must carry a non-empty root_oid"
    );
    // BLAKE3 hex is 64 chars; the stub marker has @ which is not hex.
    assert!(
        head_trim.chars().all(|c| c.is_ascii_hexdigit()),
        "HEAD must be a hex root_oid (not stub marker); got: {:?}",
        head_trim
    );
    assert_eq!(
        head_trim.len(),
        64,
        "HEAD must be a BLAKE3 hex (64 chars); got {} chars",
        head_trim.len()
    );
}

/// P4 RED: envelope.root_oid matches the on-disk HEAD ref (composition
/// is consistent across the envelope surface and the storage surface).
#[test]
fn init_envelope_root_oid_matches_head_ref() {
    let fixture = init_fixture_repo();
    let path_arg = fixture.path().to_str().expect("utf8 path").to_string();
    let out = run_init_in_fixture(fixture.path(), &[&path_arg]);
    let envelope = parse_envelope(&out.stdout);
    let envelope_oid = envelope["root_oid"]
        .as_str()
        .expect("envelope.root_oid is a string")
        .to_string();
    let head_ref = fixture.path().join(".git/mirror/refs/HEAD");
    let head_oid = std::fs::read_to_string(&head_ref)
        .expect("read HEAD")
        .trim()
        .to_string();
    assert_eq!(
        envelope_oid, head_oid,
        "envelope.root_oid must equal .git/mirror/refs/HEAD"
    );
}

/// P4 RED: envelope.indexed equals the number of files projected
/// (the fixture has 2 files).
#[test]
fn init_envelope_indexed_reflects_file_count() {
    let fixture = init_fixture_repo();
    let path_arg = fixture.path().to_str().expect("utf8 path").to_string();
    let out = run_init_in_fixture(fixture.path(), &[&path_arg]);
    let envelope = parse_envelope(&out.stdout);
    assert_eq!(
        envelope["indexed"].as_u64(),
        Some(2),
        "envelope.indexed must equal projected file count (2); got: {}",
        envelope["indexed"]
    );
}

/// P4 RED: envelope.bytes_total equals the sum of projected file bytes
/// (`hello mirror init\n` = 18 bytes; `# deep\n` = 7 bytes; total 25).
#[test]
fn init_envelope_bytes_total_reflects_projected_bytes() {
    let fixture = init_fixture_repo();
    let path_arg = fixture.path().to_str().expect("utf8 path").to_string();
    let out = run_init_in_fixture(fixture.path(), &[&path_arg]);
    let envelope = parse_envelope(&out.stdout);
    // 18 + 7 = 25
    assert_eq!(
        envelope["bytes_total"].as_u64(),
        Some(25),
        "envelope.bytes_total must equal sum of projected bytes (25); got: {}",
        envelope["bytes_total"]
    );
}

/// P4 RED: envelope.store path points at `.git/mirror/` under the repo.
#[test]
fn init_envelope_store_path_points_at_namespaced_dir() {
    let fixture = init_fixture_repo();
    let path_arg = fixture.path().to_str().expect("utf8 path").to_string();
    let out = run_init_in_fixture(fixture.path(), &[&path_arg]);
    let envelope = parse_envelope(&out.stdout);
    let store_field = envelope["store"]
        .as_str()
        .expect("envelope.store is a string")
        .to_string();
    // Tempdir paths on macOS resolve through /var/folders/.../.private — we
    // accept either the canonicalized or non-canonicalized form, but the
    // suffix `.git/mirror` must be present.
    assert!(
        store_field.contains(".git/mirror"),
        "envelope.store must reference .git/mirror/; got: {}",
        store_field
    );
}

/// P4 RED: re-running init against the same repo is idempotent at the
/// root_oid level (same git state -> same BLAKE3 root).
#[test]
fn init_is_idempotent_per_git_state() {
    let fixture = init_fixture_repo();
    let path_arg = fixture.path().to_str().expect("utf8 path").to_string();
    let out1 = run_init_in_fixture(fixture.path(), &[&path_arg]);
    let envelope1 = parse_envelope(&out1.stdout);
    let oid1 = envelope1["root_oid"].as_str().unwrap_or("").to_string();

    let out2 = run_init_in_fixture(fixture.path(), &[&path_arg]);
    let envelope2 = parse_envelope(&out2.stdout);
    let oid2 = envelope2["root_oid"].as_str().unwrap_or("").to_string();

    assert_eq!(
        oid1, oid2,
        "re-running init against the same git state must yield the same root_oid"
    );
}

/// P4 RED: init in a non-git directory exits non-zero (NamespacedGitStore
/// fails with NotAGitRepo). The current stub merely checks `path.is_dir`,
/// so a non-git directory passes — GREEN must surface the wire failure.
#[test]
fn init_exits_non_zero_in_non_git_directory() {
    let dir = fresh_fixture_dir("non-git");
    let path_arg = dir.path().to_str().expect("utf8 path").to_string();
    let out = run_init_in_fixture(dir.path(), &[&path_arg]);
    assert_ne!(
        out.status.code(),
        Some(0),
        "init must exit non-zero when target is not a git repo"
    );
}
