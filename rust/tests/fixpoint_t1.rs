//! fixpoint_t1.rs — T1, the fixed point (docs/specs/mirror-fixpoint-parser.md §13).
//!
//! Acceptance test, gated: every test is ignored unless the `red-claims`
//! feature is on (`just test-red` runs it), so the commit gate stays green
//! while T1 is under construction. When T1 is one step from green, a 🔴
//! removes the ignore and the 🟢 lands right after it.
//!
//! Drives the built binary the way `tests/pact_vectors.rs` drives
//! `mirror pact eval`, and reads stdout as mirror-text (spec §1.1;
//! kintsugi-ci-v0.1 §1.4: `<key> <value>` lines, records separated by blank
//! lines; the first record is the summary). Plain string handling, no serde.
//!
//! Each test runs in its own scratch git repository holding copies of the
//! inputs at the same relative paths, so the git objects and
//! `refs/mirror/*` that compile writes (spec §7) never land in the
//! developer's repository.
//!
//! T1.5 re-hashes the git projection with its own BLAKE3 (a dev-dependency):
//! a second observer of the bytes pinned in spec §5.0 and §7, independent of
//! the binary's read path.
//!
//! Gate (spec §13, Seam): T1 may not go green before T2's alternative-swap
//! assertion is green. A normalizer that sorted choice children would pass
//! T1 everywhere.
//!
//! Every failure names what is missing. Nothing is skipped.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The grammar shard, relative to the repository root.
const MIRROR_MIRROR: &str = "mirror.mirror";
/// T1.3's fixture, relative to the repository root.
const REFORMATTED: &str = "tests/fixpoint/mirror.reformatted.mirror";
/// The script that generates `REFORMATTED`, relative to the repository root.
const MAKE_REFORMATTED: &str = "tests/fixpoint/make-reformatted.sh";
/// `@mirror`'s pinned default start rule (spec §1.2′).
const DEFAULT_START_RULE: &str = "shard";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .canonicalize()
        .expect("repository root (CARGO_MANIFEST_DIR/..) resolves")
}

/// The input at `rel` under the repository root, or a failure that names it.
fn require_input(rel: &str) -> PathBuf {
    let path = repo_root().join(rel);
    if !path.is_file() {
        if rel == MIRROR_MIRROR {
            panic!(
                "T1 input missing: {} does not exist.\n\
                 mirror.mirror is the grammar (spec §2). It lands with the seed and \
                 engine work (spec §14) and is not authored by this test.",
                path.display()
            );
        }
        panic!("T1 input missing: {} does not exist.", path.display());
    }
    path
}

// ── mirror-text ───────────────────────────────────────────────────────────

/// One mirror-text record: `<key> <value>` lines, in order.
struct Record {
    fields: Vec<(String, String)>,
}

impl Record {
    fn get(&self, key: &str) -> Option<&str> {
        self.fields
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
}

/// Parses stdout as blank-line-separated records. Every non-blank line must
/// be a `<key> <value>` field line; no key repeats within a record.
fn parse_records(stdout: &str) -> Result<Vec<Record>, String> {
    let mut records = Vec::new();
    let mut cur: Vec<(String, String)> = Vec::new();
    for (n, line) in stdout.lines().enumerate() {
        if line.trim().is_empty() {
            if !cur.is_empty() {
                records.push(Record {
                    fields: std::mem::take(&mut cur),
                });
            }
            continue;
        }
        let mut parts = line.splitn(2, char::is_whitespace);
        let key = parts.next().unwrap_or("");
        let value = parts.next().unwrap_or("").trim();
        let key_ok = !key.is_empty()
            && key
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
        if !key_ok || value.is_empty() {
            return Err(format!(
                "line {} is not a `<key> <value>` field line: {:?}",
                n + 1,
                line
            ));
        }
        if cur.iter().any(|(k, _)| k == key) {
            return Err(format!(
                "line {}: field `{}` appears twice in one record",
                n + 1,
                key
            ));
        }
        cur.push((key.to_string(), value.to_string()));
    }
    if !cur.is_empty() {
        records.push(Record { fields: cur });
    }
    if records.is_empty() {
        return Err("stdout is empty; expected the §1.1 summary record".into());
    }
    Ok(records)
}

/// What `mirror compile` printed: the summary record and the raw text.
struct Compiled {
    summary: Record,
    stdout: String,
    cmd: String,
}

impl Compiled {
    /// A summary field, or a failure that names it and shows the output.
    fn require(&self, key: &str) -> &str {
        self.summary.get(key).unwrap_or_else(|| {
            panic!(
                "`{}`: summary record has no `{}` field (spec §1.1)\nstdout:\n{}",
                self.cmd, key, self.stdout
            )
        })
    }

    fn require_count(&self, key: &str) -> u64 {
        let v = self.require(key);
        v.parse().unwrap_or_else(|_| {
            panic!(
                "`{}`: `{}` is not a non-negative integer: {:?}\nstdout:\n{}",
                self.cmd, key, v, self.stdout
            )
        })
    }
}

fn is_lower_hex(s: &str, len: usize) -> bool {
    s.len() == len
        && s.chars()
            .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// The verdict the exit-code table (spec §1.1) assigns to `code`.
fn verdict_of_exit(code: Option<i32>) -> &'static str {
    match code {
        Some(0) => "success",
        Some(1) => "partial",
        Some(255) => "fail (red)",
        Some(2) => "dark",
        _ => "none (not in the §1.1 exit table)",
    }
}

// ── scratch repository ────────────────────────────────────────────────────

/// A fresh git repository with copies of the given inputs.
struct Sandbox {
    dir: tempfile::TempDir,
}

impl Sandbox {
    fn with(inputs: &[&str]) -> Sandbox {
        let sources: Vec<(String, PathBuf)> = inputs
            .iter()
            .map(|rel| (rel.to_string(), require_input(rel)))
            .collect();
        let dir = tempfile::tempdir().expect("create scratch dir");
        let sb = Sandbox { dir };
        sb.git_ok(&["init", "-q"]);
        for (rel, src) in sources {
            let dst = sb.path().join(&rel);
            if let Some(parent) = dst.parent() {
                std::fs::create_dir_all(parent).expect("create fixture dir");
            }
            std::fs::copy(&src, &dst)
                .unwrap_or_else(|e| panic!("copy {} into scratch repo: {}", src.display(), e));
        }
        sb
    }

    fn path(&self) -> &Path {
        self.dir.path()
    }

    fn git(&self, args: &[&str]) -> Output {
        Command::new("git")
            .args(args)
            .current_dir(self.path())
            .output()
            .unwrap_or_else(|e| panic!("run git {:?}: {}", args, e))
    }

    fn git_bytes(&self, args: &[&str]) -> Vec<u8> {
        let out = self.git(args);
        assert!(
            out.status.success(),
            "git {:?} exited {:?}\nstderr: {}",
            args,
            out.status.code(),
            String::from_utf8_lossy(&out.stderr)
        );
        out.stdout
    }

    fn git_ok(&self, args: &[&str]) -> String {
        String::from_utf8_lossy(&self.git_bytes(args)).into_owned()
    }

    /// Runs `mirror <args>` in the sandbox. Requires exit 0, a summary record
    /// with `verdict success`, and a 64-hex `root`.
    fn compile(&self, args: &[&str]) -> Compiled {
        let cmd = format!("mirror {}", args.join(" "));
        let out = Command::new(env!("CARGO_BIN_EXE_mirror"))
            .args(args)
            .current_dir(self.path())
            .output()
            .expect("run mirror");
        let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            out.status.success(),
            "`{}` exited {:?} (verdict by the §1.1 exit table: {}); T1 requires exit 0 \
             and `verdict success`\nstdout:\n{}\nstderr:\n{}",
            cmd,
            out.status.code(),
            verdict_of_exit(out.status.code()),
            stdout,
            stderr
        );
        let mut records = parse_records(&stdout).unwrap_or_else(|e| {
            panic!(
                "`{}`: stdout is not mirror-text records (spec §1.1): {}\n\
                 stdout:\n{}\nstderr:\n{}",
                cmd, e, stdout, stderr
            )
        });
        let summary = records.remove(0);
        let c = Compiled {
            summary,
            stdout,
            cmd,
        };
        assert_eq!(
            c.require("verdict"),
            "success",
            "`{}`: expected `verdict success`\nstdout:\n{}",
            c.cmd,
            c.stdout
        );
        let root = c.require("root");
        assert!(
            is_lower_hex(root, 64),
            "`{}`: `root` is not 64 lowercase hex (BLAKE3, spec §1.1): {:?}",
            c.cmd,
            root
        );
        c
    }
}

// ── T1.1 ──────────────────────────────────────────────────────────────────

/// `mirror compile mirror.mirror` exits 0 and prints a summary with
/// `verdict success`, `crystals > 0` and a `root`, read from `@mirror`'s
/// default start `shard`. `success` means no holes, no undefined atoms and
/// no Dark lines (spec §1.1), so those counts are 0.
#[test]
#[cfg_attr(
    not(feature = "red-claims"),
    ignore = "acceptance: green at T1 (spec §14)"
)]
fn t1_1_compile_mirror_mirror_is_success_with_crystals_and_root() {
    let sb = Sandbox::with(&[MIRROR_MIRROR]);
    let c = sb.compile(&["compile", MIRROR_MIRROR]);
    assert!(
        c.require_count("crystals") > 0,
        "`crystals 0`: the crystal DAG is empty\nstdout:\n{}",
        c.stdout
    );
    let start = c.require("start");
    assert_eq!(
        start.rsplit('.').next(),
        Some(DEFAULT_START_RULE),
        "`start` is {:?}; without --start, @mirror's default start is the exported \
         rule `{}` (spec §1.2′)",
        start,
        DEFAULT_START_RULE
    );
    for key in ["holes", "undefined", "undeclared"] {
        assert_eq!(
            c.require_count(key),
            0,
            "`verdict success` with `{}` non-zero contradicts spec §1.1\nstdout:\n{}",
            key,
            c.stdout
        );
    }
}

// ── T1.2 ──────────────────────────────────────────────────────────────────

/// A second run prints the same `root`.
#[test]
#[cfg_attr(
    not(feature = "red-claims"),
    ignore = "acceptance: green at T1 (spec §14)"
)]
fn t1_2_second_compile_prints_the_same_root() {
    let sb = Sandbox::with(&[MIRROR_MIRROR]);
    let first = sb.compile(&["compile", MIRROR_MIRROR]);
    let second = sb.compile(&["compile", MIRROR_MIRROR]);
    assert_eq!(
        first.require("root"),
        second.require("root"),
        "two compiles of mirror.mirror disagree on `root`"
    );
}

// ── T1.3 ──────────────────────────────────────────────────────────────────

/// The reformatted copy (layout changed, declarations reordered) prints the
/// same `root`. The fixture must be current (the script's output equals the
/// checked-in bytes) and must differ from mirror.mirror.
#[test]
#[cfg_attr(
    not(feature = "red-claims"),
    ignore = "acceptance: green at T1 (spec §14)"
)]
fn t1_3_reformatted_copy_prints_the_same_root() {
    let root_dir = repo_root();
    let original = require_input(MIRROR_MIRROR);
    let fixture = root_dir.join(REFORMATTED);
    if !fixture.is_file() {
        panic!(
            "T1 input missing: {} does not exist. Generate it with `{}` \
             (it reads mirror.mirror) and check it in.",
            fixture.display(),
            MAKE_REFORMATTED
        );
    }
    let script = root_dir.join(MAKE_REFORMATTED);
    let regen = Command::new("bash")
        .arg(&script)
        .arg(&original)
        .arg("-")
        .output()
        .unwrap_or_else(|e| panic!("run {}: {}", script.display(), e));
    assert!(
        regen.status.success(),
        "{} exited {:?}\nstderr: {}",
        MAKE_REFORMATTED,
        regen.status.code(),
        String::from_utf8_lossy(&regen.stderr)
    );
    let checked_in = std::fs::read(&fixture).expect("read reformatted fixture");
    assert!(
        regen.stdout == checked_in,
        "{} is stale: `{}` now produces different bytes. Regenerate and check it in.",
        REFORMATTED,
        MAKE_REFORMATTED
    );
    assert!(
        checked_in != std::fs::read(&original).expect("read mirror.mirror"),
        "{} equals mirror.mirror byte for byte; the test would be vacuous",
        REFORMATTED
    );

    let sb = Sandbox::with(&[MIRROR_MIRROR, REFORMATTED]);
    let a = sb.compile(&["compile", MIRROR_MIRROR]);
    let b = sb.compile(&["compile", REFORMATTED]);
    assert_eq!(
        a.require("root"),
        b.require("root"),
        "the reformatted copy has a different `root`: layout, declaration order or \
         the path leaked into the normal form (spec §1, §2 items 5, 6, 10; decision 3)"
    );
}

// ── T1.4 ──────────────────────────────────────────────────────────────────

/// Stage 2 at k = 1: the stage-1 parser `E(N₀)` reads mirror.mirror to the
/// same `root` as the seed `P₀` (spec §1.2′, §3).
#[test]
#[cfg_attr(
    not(feature = "red-claims"),
    ignore = "acceptance: green at T1 (spec §14)"
)]
fn t1_4_stage_one_parser_prints_the_seeds_root() {
    let sb = Sandbox::with(&[MIRROR_MIRROR]);
    let stage0 = sb.compile(&["compile", "--stage", "0", MIRROR_MIRROR]);
    let stage1 = sb.compile(&["compile", "--stage", "1", MIRROR_MIRROR]);
    assert_eq!(
        stage0.require("root"),
        stage1.require("root"),
        "stage 2 fails: E(N₀) (--stage 1) reads mirror.mirror to a different `root` \
         than the seed P₀ (--stage 0) (spec §3; k = 1 is required)"
    );
}

// ── T1.5 ──────────────────────────────────────────────────────────────────

/// Reads `τ ‖ enc(x)` (spec §5.0) from a stored payload, or says why not.
/// Returns (altitude name, x).
fn split_payload(bytes: &[u8]) -> Result<(String, &[u8]), String> {
    if bytes.first() != Some(&b'@') {
        return Err("does not start with `@` (the first byte of τ)".into());
    }
    let nul = bytes
        .iter()
        .position(|&b| b == 0)
        .ok_or("τ has no NUL after the altitude name")?;
    let name = String::from_utf8(bytes[..nul].to_vec())
        .map_err(|_| "τ's altitude name is not UTF-8".to_string())?;
    let rest = &bytes[nul + 1..];
    let rest = match rest.first() {
        Some(0x00) => &rest[1..],
        Some(0x01) if rest.len() >= 33 => &rest[33..],
        Some(0x01) => return Err("τ's 0x01 is not followed by 32 address bytes".into()),
        _ => return Err("τ's grammar byte is neither 0x00 nor 0x01".into()),
    };
    if rest.len() < 8 {
        return Err("no u64 length for enc(…)".into());
    }
    let mut len_bytes = [0u8; 8];
    len_bytes.copy_from_slice(&rest[..8]);
    let len = u64::from_be_bytes(len_bytes) as usize;
    let body = &rest[8..];
    if body.len() != len {
        return Err(format!(
            "enc(…) declares {} bytes but {} follow (u64 big-endian, spec §5.0)",
            len,
            body.len()
        ));
    }
    Ok((name, body))
}

/// An independent re-hash of `g(root)` by the bytes of spec §5.0 and §7.
struct Rehash<'a> {
    sb: &'a Sandbox,
    /// git id → addr, so a shared object is hashed once.
    memo: BTreeMap<String, [u8; 32]>,
    /// distinct splinter addresses reached (trees and atom blobs).
    splinters: BTreeSet<[u8; 32]>,
}

impl<'a> Rehash<'a> {
    fn blob(&mut self, oid: &str, path: &str) -> [u8; 32] {
        if let Some(a) = self.memo.get(oid) {
            return *a;
        }
        let payload = self.sb.git_bytes(&["cat-file", "blob", oid]);
        if let Err(e) = split_payload(&payload) {
            panic!(
                "{}: atom blob payload is not `τ ‖ enc(content)` (spec §7): {}",
                path, e
            );
        }
        let mut h = blake3::Hasher::new();
        h.update(b"splinter\0");
        h.update(&payload);
        let addr = *h.finalize().as_bytes();
        self.memo.insert(oid.to_string(), addr);
        self.splinters.insert(addr);
        addr
    }

    fn tree(&mut self, oid: &str, path: &str) -> [u8; 32] {
        if let Some(a) = self.memo.get(oid) {
            return *a;
        }
        let listing = self.sb.git_ok(&["ls-tree", "--full-tree", oid]);
        let mut label: Option<String> = None;
        let mut entries: Vec<(String, String, String, String)> = Vec::new();
        for line in listing.lines() {
            let (meta, name) = line
                .split_once('\t')
                .unwrap_or_else(|| panic!("{}: unexpected ls-tree line {:?}", path, line));
            let mut m = meta.split(' ');
            let mode = m.next().unwrap_or("").to_string();
            let kind = m.next().unwrap_or("").to_string();
            let child = m.next().unwrap_or("").to_string();
            if name == "@" {
                assert!(
                    mode == "100644" && kind == "blob",
                    "{}: `@` entry is {} {}, expected a 100644 blob (spec §7)",
                    path,
                    mode,
                    kind
                );
                label = Some(child);
            } else {
                entries.push((name.to_string(), mode, kind, child));
            }
        }
        let label = label.unwrap_or_else(|| {
            panic!(
                "{}: tree {} has no `@` entry (spec §7)\n{}",
                path, oid, listing
            )
        });
        let at_payload = self.sb.git_bytes(&["cat-file", "blob", &label]);
        if let Err(e) = split_payload(&at_payload) {
            panic!(
                "{}: `@` payload is not `τ ‖ enc(ℓ)` (spec §5.0, §7): {}",
                path, e
            );
        }

        // Fragment (plain numbered entries) or lens (`^`-prefixed), never mixed.
        let lens = entries
            .first()
            .map(|e| e.0.starts_with('^'))
            .unwrap_or(false);
        let k = entries.len();
        let width = std::cmp::max(3, if k == 0 { 1 } else { (k - 1).to_string().len() });
        let mut h = blake3::Hasher::new();
        h.update(if lens {
            b"lens\0" as &[u8]
        } else {
            b"fragment\0"
        });
        h.update(&at_payload);
        h.update(&(k as u64).to_be_bytes());
        for (i, (name, mode, kind, child)) in entries.iter().enumerate() {
            let expected = format!(
                "{}{:0width$}",
                if lens { "^" } else { "" },
                i,
                width = width
            );
            assert_eq!(
                name, &expected,
                "{}: entry {} is named {:?}, expected {:?} (width max(3, digits(k−1)) = {}, \
                 spec §7)",
                path, i, name, expected, width
            );
            let child_path = format!("{}/{}", path, name);
            let addr = match (mode.as_str(), kind.as_str()) {
                ("100644", "blob") => self.blob(child, &child_path),
                ("040000", "tree") => self.tree(child, &child_path),
                _ => panic!(
                    "{}: entry is {} {}, expected 100644 blob or 040000 tree (spec §7)",
                    child_path, mode, kind
                ),
            };
            h.update(&addr);
        }
        let addr = *h.finalize().as_bytes();
        self.memo.insert(oid.to_string(), addr);
        self.splinters.insert(addr);
        addr
    }
}

/// `git cat-file -t <git>` prints `tree`; walking it reproduces the
/// fragments; the walk's re-encoding, hashed by this test's own BLAKE3,
/// equals `root`, and reaches exactly `crystals` distinct splinters.
#[test]
#[cfg_attr(
    not(feature = "red-claims"),
    ignore = "acceptance: green at T1 (spec §14)"
)]
fn t1_5_git_projection_is_a_tree_whose_walk_rehashes_to_root() {
    let sb = Sandbox::with(&[MIRROR_MIRROR]);
    let c = sb.compile(&["compile", MIRROR_MIRROR]);
    let git_id = c.require("git").to_string();
    let root = c.require("root").to_string();

    let format = sb.git_ok(&["rev-parse", "--show-object-format"]);
    let hex_len = if format.trim() == "sha256" { 64 } else { 40 };
    assert!(
        is_lower_hex(&git_id, hex_len),
        "`git` is not a {}-hex {} object id (spec §1.1): {:?}",
        hex_len,
        format.trim(),
        git_id
    );
    assert_ne!(
        git_id, root,
        "`git` equals `root`: the identifiers are never one column (spec §1.2)"
    );

    let kind = sb.git(&["cat-file", "-t", &git_id]);
    assert!(
        kind.status.success(),
        "`git cat-file -t {}` failed: the object is not in the repository's object \
         database (spec §7: compile writes native git objects)\nstderr: {}",
        git_id,
        String::from_utf8_lossy(&kind.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&kind.stdout).trim(),
        "tree",
        "`git cat-file -t {}`",
        git_id
    );

    let mut r = Rehash {
        sb: &sb,
        memo: BTreeMap::new(),
        splinters: BTreeSet::new(),
    };
    let addr = r.tree(&git_id, "<root>");
    assert_eq!(
        hex(&addr),
        root,
        "re-hashing the walk of g(root) (spec §5.0, §7) gives a different address than \
         the record's `root`"
    );
    assert_eq!(
        r.splinters.len() as u64,
        c.require_count("crystals"),
        "the walk of g(root) reaches {} distinct splinters, but the record says \
         `crystals {}` (g is a bijection on splinters, math Prop. 7.4)",
        r.splinters.len(),
        c.require("crystals")
    );
}
