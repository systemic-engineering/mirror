//! Phase H RED — content-addressed storage contract for `mirror spawn`.
//!
//! The P6 empirical spawn (2026-07-01) landed cmd_spawn's envelope
//! wire (pieces 1–3 real; piece 6 via peer_recall structured
//! observation), but composition_pieces self-diagnoses the remainder
//! as stubs:
//!
//!   4_lead_at_n_plus_1:       stub@N+1
//!   5_supervisor_kick:        stub@spectral/supervisor.start_child
//!   7_lambda_zero_transition: stub@λ₀→runtime
//!
//! And crucially `spec_oid` is the literal string `"uncommitted"` —
//! the envelope acknowledges persisted state that does not exist yet.
//! Phase H makes the state real: spawn writes content-addressed
//! crystals under `<peer-home>/.git/mirror/` using the same
//! NamespacedGitStore + Splinter + set_ref("HEAD", root_oid) pattern
//! established by cmd_init (`31e7d45`, 15/15).
//!
//! These tests fail RED against the current cmd_spawn because:
//!   - No `.git/mirror/` is created inside the peer home.
//!   - No `refs/HEAD` ref is written (envelope.spec_oid stays
//!     "uncommitted").
//!   - The peer_recall structure lives only in the JSON envelope; it
//!     is not persisted as content-addressed crystals.
//!   - The lead reference in the envelope is a raw substrate-text
//!     `~peer'…'` string, not a content-addressed handle.
//!
//! GREEN wires cmd_spawn to:
//!   1. Open `NamespacedGitStore::open(peer_home, "mirror")`.
//!   2. Splinter the peer_recall payloads + spec source into
//!      content-addressed crystals via `insert_persistent`.
//!   3. Compute a root_oid (BLAKE3 over the sorted crystal set) and
//!      `set_ref("HEAD", root_oid)`.
//!   4. Emit the root_oid as `envelope.spec_oid` (replacing the
//!      "uncommitted" marker).
//!   5. Persist a content-addressed handle for the lead (piece 4:
//!      lead-at-N+1 becomes an OID, not a string).
//!
//! Pattern mirrors `bootstrap/tests/init.rs` (Phase G fixture pattern):
//! own the scratch dir, `git init` inside it, drop `mirror.spec` +
//! stage with `git add -A`, invoke via `mirror::kintsugi_main_in`.

use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::Output;

// ────────────────────────────────────────────────────────────────────────────
// Fixture — owned scratch dir, git-init'd, `mirror.spec` staged.
// Auto-cleans on drop. Pattern-identical to `init.rs::FixtureDir`.
// ────────────────────────────────────────────────────────────────────────────

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
    let base = std::env::temp_dir().join(format!("mirror-spawn-{}-{}-{}", label, pid, seq));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).expect("create fixture dir");
    FixtureDir(base)
}

/// Build a peer-home fixture: fresh scratch dir + `git init` + minimal
/// `mirror.spec` + `git add -A`. The spec mirrors the shape of
/// `bootstrap/tests/fixtures/spawn-test-peer/mirror.spec` so
/// extract_spec_project_name / extract_spec_pack_lead resolve the same
/// text: `test-peer` and `~peer'~/.test-lead'` respectively.
fn spawn_fixture_peer_home() -> FixtureDir {
    let dir = fresh_fixture_dir("peer");
    let status = std::process::Command::new("git")
        .arg("init")
        .arg("--quiet")
        .current_dir(dir.path())
        .status()
        .expect("git init");
    assert!(status.success(), "git init failed in fixture");

    // Minimal mirror.spec — same shape as the checked-in fixture so the
    // Phase G text extractors resolve the same peer / lead values.
    let spec = "\
in @mirror/cli
in @mirror/mosaic
in @mirror/spec
in @property
in @io

project test-peer {
  source ~d'shards/'

  pack {
    lead ~peer'~/.test-lead'
  }
}
";
    std::fs::write(dir.path().join("mirror.spec"), spec).expect("write mirror.spec");

    let status = std::process::Command::new("git")
        .arg("add")
        .arg("-A")
        .current_dir(dir.path())
        .status()
        .expect("git add -A");
    assert!(status.success(), "git add -A failed in fixture");
    dir
}

fn run_spawn_in(cwd: &std::path::Path, args: &[&str]) -> Output {
    let mut argv: Vec<String> = vec!["mirror".to_string(), "spawn".to_string()];
    argv.extend(args.iter().map(|s| s.to_string()));
    let out = mirror::kintsugi_main_in(&argv, cwd);
    Output {
        status: std::process::ExitStatus::from_raw(out.exit_code << 8),
        stdout: out.stdout,
        stderr: out.stderr,
    }
}

fn parse_envelope(stdout: &[u8], context: &str) -> serde_json::Value {
    let s = String::from_utf8_lossy(stdout);
    serde_json::from_str(s.trim()).unwrap_or_else(|e| {
        panic!(
            "{} envelope must be valid JSON; got:\n{}\nparse error: {}",
            context, s, e
        )
    })
}

// ────────────────────────────────────────────────────────────────────────────
// Phase H RED — content-addressed storage assertions.
// ────────────────────────────────────────────────────────────────────────────

/// Phase H RED: `<peer-home>/.git/mirror/` exists after spawn.
/// GREEN opens `NamespacedGitStore::open(peer_home, "mirror")` inside
/// cmd_spawn, which creates the namespaced storage directory tree.
#[test]
fn spawn_creates_namespaced_store() {
    let peer = spawn_fixture_peer_home();
    let peer_path_arg = peer.path().to_str().expect("utf8 path").to_string();
    let out = run_spawn_in(peer.path(), &[&peer_path_arg, "--hello-world"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "spawn --hello-world must exit 0; stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let store_dir = peer.path().join(".git").join("mirror");
    assert!(
        store_dir.exists(),
        "<peer-home>/.git/mirror/ must exist after spawn (NamespacedGitStore::open); \
         not found at {:?}",
        store_dir
    );
    assert!(
        store_dir.join("objects").exists(),
        "<peer-home>/.git/mirror/objects/ must exist after spawn"
    );
    assert!(
        store_dir.join("refs").exists(),
        "<peer-home>/.git/mirror/refs/ must exist after spawn"
    );
}

/// Phase H RED: `<peer-home>/.git/mirror/refs/HEAD` resolves to a
/// non-empty content-addressed OID (not the "uncommitted" stub marker).
/// GREEN wires `set_ref("HEAD", root_oid)` at the end of cmd_spawn.
#[test]
fn spawn_sets_head_ref_to_spec_oid() {
    let peer = spawn_fixture_peer_home();
    let peer_path_arg = peer.path().to_str().expect("utf8 path").to_string();
    let out = run_spawn_in(peer.path(), &[&peer_path_arg, "--hello-world"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "spawn --hello-world must exit 0; stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let head_ref = peer.path().join(".git/mirror/refs/HEAD");
    assert!(
        head_ref.exists(),
        "<peer-home>/.git/mirror/refs/HEAD must exist after spawn; not found at {:?}",
        head_ref
    );
    let head_content = std::fs::read_to_string(&head_ref).expect("read HEAD");
    let head_trim = head_content.trim();
    assert!(
        !head_trim.is_empty(),
        "HEAD ref must carry a non-empty root_oid (not empty, not 'uncommitted')"
    );
    assert_ne!(
        head_trim, "uncommitted",
        "HEAD ref must NOT be the stub marker 'uncommitted'; got: {:?}",
        head_trim
    );
    // BLAKE3 hex is 64 lowercase hex chars — matches the init.rs contract.
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

/// Phase H RED: the envelope's `spec_oid` field matches the on-disk
/// HEAD ref. Today it is the literal `"uncommitted"` string;
/// GREEN replaces that with the root_oid returned from the
/// storage composition.
#[test]
fn spawn_envelope_spec_oid_matches_head() {
    let peer = spawn_fixture_peer_home();
    let peer_path_arg = peer.path().to_str().expect("utf8 path").to_string();
    let out = run_spawn_in(peer.path(), &[&peer_path_arg, "--hello-world"]);
    let envelope = parse_envelope(&out.stdout, "spawn --hello-world");
    let spec_oid = envelope["spec_oid"]
        .as_str()
        .expect("envelope.spec_oid is a string")
        .to_string();
    assert_ne!(
        spec_oid, "uncommitted",
        "envelope.spec_oid must NOT be the stub marker 'uncommitted' \
         (Phase H stores the peer_recall crystals + writes HEAD)"
    );
    let head_ref = peer.path().join(".git/mirror/refs/HEAD");
    let head_oid = std::fs::read_to_string(&head_ref)
        .expect("read HEAD")
        .trim()
        .to_string();
    assert_eq!(
        spec_oid, head_oid,
        "envelope.spec_oid must equal .git/mirror/refs/HEAD (composition \
         is consistent across the envelope surface and storage surface)"
    );
    // Belt-and-braces shape check on the envelope value itself.
    assert_eq!(
        spec_oid.len(),
        64,
        "envelope.spec_oid must be a BLAKE3 hex (64 chars); got {} chars",
        spec_oid.len()
    );
    assert!(
        spec_oid.chars().all(|c| c.is_ascii_hexdigit()),
        "envelope.spec_oid must be lowercase hex; got: {:?}",
        spec_oid
    );
}

/// Phase H RED: content-addressed crystals for the peer_recall
/// structure exist under `<peer-home>/.git/mirror/objects/`. Today
/// the peer_recall JSON is embedded in the envelope only; GREEN
/// persists each payload (cascade / pack_trail / pull_frontier /
/// dogfood) as a Splinter crystal so the lead observes
/// content-addressable state, not throwaway JSON.
#[test]
fn spawn_persists_peer_recall_structure() {
    let peer = spawn_fixture_peer_home();
    let peer_path_arg = peer.path().to_str().expect("utf8 path").to_string();
    let out = run_spawn_in(peer.path(), &[&peer_path_arg, "--hello-world"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "spawn --hello-world must exit 0; stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let objects_dir = peer.path().join(".git/mirror/objects");
    assert!(
        objects_dir.exists(),
        "<peer-home>/.git/mirror/objects/ must exist after spawn"
    );
    // Walk objects/ and count files — the substrate's on-disk crystal
    // presence check. Zero crystals means no persistence happened.
    let mut crystal_count = 0usize;
    for entry in walkdir(&objects_dir) {
        if entry.is_file() {
            crystal_count += 1;
        }
    }
    // Four peer_recall payloads + at minimum the spec source itself =
    // ≥ 4 crystals. GREEN may write more; the floor is what we assert.
    assert!(
        crystal_count >= 4,
        "spawn must persist ≥ 4 content-addressed crystals under \
         <peer-home>/.git/mirror/objects/ (cascade / pack_trail / \
         pull_frontier / dogfood); found only {}",
        crystal_count
    );
}

/// Phase H RED: the lead reference in the envelope is
/// content-addressed (either a hex OID or an object with an `oid`
/// field), not just the raw substrate-text `~peer'…'` string.
///
/// Piece 4 (`lead_at_n_plus_1`) is the substrate's N+1 observer
/// contract; GREEN promotes it from stub-string to a persisted
/// handle so the lead becomes reachable through
/// `.git/mirror/refs/` or the Splinter crystal set — not through
/// an unresolvable text field.
#[test]
fn spawn_hello_world_persists_lead_reference() {
    let peer = spawn_fixture_peer_home();
    let peer_path_arg = peer.path().to_str().expect("utf8 path").to_string();
    let out = run_spawn_in(peer.path(), &[&peer_path_arg, "--hello-world"]);
    let envelope = parse_envelope(&out.stdout, "spawn --hello-world");
    let lead_field = &envelope["lead"];
    // Two accepted GREEN shapes: (a) hex OID string, (b) object with
    // an `oid` field carrying the hex OID. Either proves the lead is
    // content-addressed.
    let lead_oid: String = if let Some(s) = lead_field.as_str() {
        assert_ne!(
            s, "<no-lead>",
            "envelope.lead must not be the fallback stub"
        );
        assert!(
            !s.starts_with("~peer"),
            "envelope.lead must NOT be the raw substrate-text sigil \
             (Phase H content-addresses the lead reference); got: {:?}",
            s
        );
        s.to_string()
    } else if let Some(obj) = lead_field.as_object() {
        obj.get("oid")
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| {
                panic!(
                    "envelope.lead object must carry an 'oid' field; got: {}",
                    lead_field
                )
            })
            .to_string()
    } else {
        panic!(
            "envelope.lead must be a hex OID string OR an object with \
             an 'oid' field; got: {}",
            lead_field
        );
    };
    // Content-address discipline: BLAKE3 hex is 64 lowercase hex chars.
    assert_eq!(
        lead_oid.len(),
        64,
        "envelope.lead OID must be BLAKE3 hex (64 chars); got {} chars: {:?}",
        lead_oid.len(),
        lead_oid
    );
    assert!(
        lead_oid.chars().all(|c| c.is_ascii_hexdigit()),
        "envelope.lead OID must be lowercase hex; got: {:?}",
        lead_oid
    );
    // Storage-side reachability: the lead crystal must land under
    // objects/ (persistence, not just envelope emission).
    let objects_dir = peer.path().join(".git/mirror/objects");
    let mut found_matching_object = false;
    for entry in walkdir(&objects_dir) {
        if !entry.is_file() {
            continue;
        }
        // NamespacedGitStore stores crystals under
        // objects/<xx>/<remaining-62> where <xx>/<remaining-62> is
        // the hex OID. Verify the lead_oid resolves to a real file.
        let s = entry.to_string_lossy();
        if s.contains(&lead_oid[..2]) && s.contains(&lead_oid[2..]) {
            found_matching_object = true;
            break;
        }
    }
    assert!(
        found_matching_object,
        "envelope.lead OID {:?} must resolve to a crystal on disk under \
         <peer-home>/.git/mirror/objects/ (content-addressed \
         persistence, not just envelope emission)",
        lead_oid
    );
}

// ────────────────────────────────────────────────────────────────────────────
// Tiny fixture-local dir walker (no walkdir dep per brief: do not touch
// Cargo.toml). Recursive, returns every entry as an owned PathBuf.
// ────────────────────────────────────────────────────────────────────────────

fn walkdir(root: &std::path::Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack: Vec<PathBuf> = vec![root.to_path_buf()];
    while let Some(p) = stack.pop() {
        let rd = match std::fs::read_dir(&p) {
            Ok(r) => r,
            Err(_) => continue,
        };
        for entry in rd.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path.clone());
            }
            out.push(path);
        }
    }
    out
}
