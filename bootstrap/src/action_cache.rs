//! N3 TICK 1 — Rust wiring for `@mirror/store/action_cache`.
//!
//! Per N1 (`2857fb1`, `@epistemologic/property/verdict_is_content_addressed`)
//! + N2 (`0a72c42`, `shards/mirror/store/action_cache.mirror`):
//!
//! The verdict is a total function of `(spec_oid, target_oid, inputs_oid)`.
//! Memoization is authorized by construction. This module operationalises
//! `cache_read` / `cache_write` / `cache_exists` at Rust altitude so
//! `cmd_kintsugi_spec` consults the cache before dispatching cargo.
//!
//! **Business-observable outcome**: the 13-minute pre-commit hook falls on
//! warm-cache commits — cold-cache dispatches cargo, warm-cache returns the
//! memoized verdict in O(stat + read).
//!
//! ## Persistence
//!
//! Cache state lives in `@mirror/store` (crystals in the store DAG), NOT
//! in-process. Two separate process invocations against the same substrate
//! state share the cache. Per mosaic-store-cache-invariants spec §6.2.
//!
//! Layout under `<cache_root>/action_cache/`:
//!
//! ```text
//! <spec_oid>/<target_oid>/<inputs_oid>/verdict.json
//! ```
//!
//! Each `<...>_oid` is a hex-encoded BLAKE3 hash (64 chars). The
//! JSON file stores the memoized verdict envelope.
//!
//! **Cold-cache path (miss)**: `cache_read` returns `None`; caller
//! dispatches cargo; `cache_write` writes the verdict.
//!
//! **Warm-cache path (hit)**: `cache_read` returns `Some(verdict)`; caller
//! skips cargo entirely.
//!
//! **Idempotence**: `cache_write` with the same key + verdict is a no-op on
//! second call (write-if-absent semantics; content-addressed by construction
//! per N1 predicate).

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Key types — no-bare-types discipline (feedback-no-bare-types).
// ---------------------------------------------------------------------------

/// Hex-encoded BLAKE3 hash of the mirror.spec file bytes.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpecOid(String);

impl SpecOid {
    pub fn new(hex: impl Into<String>) -> Self {
        Self(hex.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Hex-encoded BLAKE3 hash of a specific target block within the spec.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TargetOid(String);

impl TargetOid {
    pub fn new(hex: impl Into<String>) -> Self {
        Self(hex.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Hex-encoded BLAKE3 hash of the target's transitive input closure.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InputsOid(String);

impl InputsOid {
    pub fn new(hex: impl Into<String>) -> Self {
        Self(hex.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The memoized verdict envelope. Stored as JSON on-disk; content-addressed
/// by the (spec_oid, target_oid, inputs_oid) triple.
///
/// Kept intentionally minimal: N1 predicate authorizes memoization on the
/// verdict as a total function of input OIDs. Fields mirror `PerFileVerdict`'s
/// substrate-observable columns.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CachedVerdict {
    /// `"success"` | `"partial"` | `"failure"`.
    pub verdict: String,
    /// Total objective (loss).
    pub objective: f64,
    /// Dark count.
    pub dark_count: u64,
    /// Label / manifest path with cargo pretty (opaque to the cache).
    pub label: String,
}

// ---------------------------------------------------------------------------
// OID computation.
// ---------------------------------------------------------------------------

fn hex_encode(bytes: &[u8; 32]) -> String {
    let mut s = String::with_capacity(64);
    for b in bytes.iter() {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

/// Blake3 hash of the mirror.spec bytes → `SpecOid`.
pub fn compute_spec_oid(spec_bytes: &[u8]) -> SpecOid {
    let digest = blake3::hash(spec_bytes);
    SpecOid::new(hex_encode(digest.as_bytes()))
}

/// Blake3 hash of a target's identifying fields → `TargetOid`.
///
/// The target block is not currently reachable from the AST as a byte-slice
/// (`spec_targets_from_ast` produces a synthetic `SpecTarget`). We hash the
/// substrate-observable dimensions instead: `block_name || 0x00 || emit ||
/// 0x00 || check`. This is content-addressed w.r.t. the target's meaningful
/// identity; changing any dimension produces a distinct OID.
pub fn compute_target_oid(block_name: &str, emit: &str, check: &str) -> TargetOid {
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(block_name.as_bytes());
    buf.push(0);
    buf.extend_from_slice(emit.as_bytes());
    buf.push(0);
    buf.extend_from_slice(check.as_bytes());
    let digest = blake3::hash(&buf);
    TargetOid::new(hex_encode(digest.as_bytes()))
}

/// Blake3 hash of the target's transitive input closure → `InputsOid`.
///
/// **Reed-lean conservative first cut** (per N3 brief): hash of all
/// `.rs`, `.toml`, `.lock` files under the manifest-path directory tree,
/// plus the manifest path bytes themselves. Deterministic under a sorted
/// filename walk. Skips `target/` (build artifacts).
///
/// Future tick lifts this to the substrate's `splinter_graph` closure walk
/// from `@mirror/store`. For now: conservative + correct is the win. If we
/// over-invalidate (hash more inputs than a target actually depends on),
/// we get cache MISSES on unchanged targets — safe. Under-invalidation
/// would be the unsafe direction.
pub fn compute_inputs_oid(manifest: &Path) -> InputsOid {
    let manifest_dir = manifest.parent().unwrap_or(Path::new("."));
    let mut file_hashes: Vec<(String, [u8; 32])> = Vec::new();
    collect_file_hashes(manifest_dir, &mut file_hashes);
    // Sort by relative path so the OID is stable across filesystem walk
    // orderings.
    file_hashes.sort_by(|a, b| a.0.cmp(&b.0));

    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(manifest.to_string_lossy().as_bytes());
    buf.push(0);
    for (rel, hash) in &file_hashes {
        buf.extend_from_slice(rel.as_bytes());
        buf.push(0);
        buf.extend_from_slice(hash);
    }
    let digest = blake3::hash(&buf);
    InputsOid::new(hex_encode(digest.as_bytes()))
}

/// Recursively walk `dir` collecting `(relative_path, blake3_hash)` pairs
/// for every `.rs`, `.toml`, `.lock` file. Skips `target/` and any
/// dotted directory (`.git/`, `.mirror/`, etc.) so cache directories
/// don't self-reference.
fn collect_file_hashes(dir: &Path, out: &mut Vec<(String, [u8; 32])>) {
    let base = dir;
    fn walk(base: &Path, cur: &Path, out: &mut Vec<(String, [u8; 32])>) -> std::io::Result<()> {
        let entries = fs::read_dir(cur)?;
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name();
            let name_s = name.to_string_lossy();
            if name_s.starts_with('.') || name_s == "target" || name_s == "node_modules" {
                continue;
            }
            let ft = entry.file_type()?;
            if ft.is_dir() {
                let _ = walk(base, &path, out);
            } else if ft.is_file() {
                let is_input = name_s.ends_with(".rs")
                    || name_s.ends_with(".toml")
                    || name_s.ends_with(".lock");
                if !is_input {
                    continue;
                }
                if let Ok(bytes) = fs::read(&path) {
                    let digest = blake3::hash(&bytes);
                    let rel = path
                        .strip_prefix(base)
                        .unwrap_or(&path)
                        .to_string_lossy()
                        .to_string();
                    out.push((rel, *digest.as_bytes()));
                }
            }
        }
        Ok(())
    }
    let _ = walk(base, base, out);
}

// ---------------------------------------------------------------------------
// Cache root — persistence in @mirror/store.
// ---------------------------------------------------------------------------

/// Resolve the cache root: `<cwd>/.mirror/action_cache`. Created on first
/// write. Per the mosaic-store-cache-invariants spec §6.2 (P4 hook unblock),
/// the cache lives in `@mirror/store` so two process invocations share it.
pub fn cache_root(cwd: &Path) -> PathBuf {
    cwd.join(".mirror").join("action_cache")
}

fn cache_entry_path(
    cwd: &Path,
    spec_oid: &SpecOid,
    target_oid: &TargetOid,
    inputs_oid: &InputsOid,
) -> PathBuf {
    cache_root(cwd)
        .join(spec_oid.as_str())
        .join(target_oid.as_str())
        .join(inputs_oid.as_str())
        .join("verdict.json")
}

// ---------------------------------------------------------------------------
// The three action_cache actions — cache_read / cache_write / cache_exists.
// ---------------------------------------------------------------------------

/// `cache_read(spec_oid, target_oid, inputs_oid) -> Option<CachedVerdict>`.
///
/// Returns `Some(v)` on a warm-cache HIT (verdict deserialized cleanly),
/// `None` on a MISS (no file, unreadable, or malformed JSON — treated as
/// miss to keep the read path robust against partial writes).
pub fn cache_read(
    cwd: &Path,
    spec_oid: &SpecOid,
    target_oid: &TargetOid,
    inputs_oid: &InputsOid,
) -> Option<CachedVerdict> {
    let path = cache_entry_path(cwd, spec_oid, target_oid, inputs_oid);
    let bytes = fs::read(&path).ok()?;
    serde_json::from_slice(&bytes).ok()
}

/// `cache_write(spec_oid, target_oid, inputs_oid, verdict) -> ()`.
///
/// Idempotent by content-address per N1 predicate: writing the same
/// key+verdict twice is a no-op on the second call. Implemented as
/// write-if-absent: if the path already exists AND the payload matches,
/// skip; otherwise write.
pub fn cache_write(
    cwd: &Path,
    spec_oid: &SpecOid,
    target_oid: &TargetOid,
    inputs_oid: &InputsOid,
    verdict: &CachedVerdict,
) -> std::io::Result<()> {
    let path = cache_entry_path(cwd, spec_oid, target_oid, inputs_oid);
    // Fast-path: if the same JSON is already there, no-op.
    if let Ok(existing) = fs::read(&path) {
        if let Ok(existing_verdict) = serde_json::from_slice::<CachedVerdict>(&existing) {
            if existing_verdict == *verdict {
                return Ok(());
            }
        }
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_vec(verdict)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    fs::write(&path, json)
}

/// `cache_exists(spec_oid, target_oid, inputs_oid) -> bool`.
///
/// Stat-only fast-path — does not deserialize the verdict. Used by
/// callers that only care whether a memoization exists (e.g. dry-run
/// benchmarks or the "would this be warm?" check).
pub fn cache_exists(
    cwd: &Path,
    spec_oid: &SpecOid,
    target_oid: &TargetOid,
    inputs_oid: &InputsOid,
) -> bool {
    cache_entry_path(cwd, spec_oid, target_oid, inputs_oid).is_file()
}

// ---------------------------------------------------------------------------
// Tests.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn tempdir() -> PathBuf {
        let mut base = std::env::temp_dir();
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let pid = std::process::id();
        base.push(format!("mirror-action-cache-test-{}-{}", pid, stamp));
        fs::create_dir_all(&base).unwrap();
        base
    }

    #[test]
    fn cache_read_misses_when_empty() {
        let cwd = tempdir();
        let spec = SpecOid::new("aa".repeat(32));
        let tgt = TargetOid::new("bb".repeat(32));
        let inp = InputsOid::new("cc".repeat(32));
        assert!(cache_read(&cwd, &spec, &tgt, &inp).is_none());
        assert!(!cache_exists(&cwd, &spec, &tgt, &inp));
    }

    #[test]
    fn cache_write_then_read_hits() {
        let cwd = tempdir();
        let spec = SpecOid::new("aa".repeat(32));
        let tgt = TargetOid::new("bb".repeat(32));
        let inp = InputsOid::new("cc".repeat(32));
        let v = CachedVerdict {
            verdict: "success".into(),
            objective: 0.0,
            dark_count: 0,
            label: "bootstrap/Cargo.toml (check)".into(),
        };
        cache_write(&cwd, &spec, &tgt, &inp, &v).unwrap();
        assert!(cache_exists(&cwd, &spec, &tgt, &inp));
        let got = cache_read(&cwd, &spec, &tgt, &inp).unwrap();
        assert_eq!(got, v);
    }

    #[test]
    fn cache_write_is_idempotent() {
        let cwd = tempdir();
        let spec = SpecOid::new("aa".repeat(32));
        let tgt = TargetOid::new("bb".repeat(32));
        let inp = InputsOid::new("cc".repeat(32));
        let v = CachedVerdict {
            verdict: "success".into(),
            objective: 0.0,
            dark_count: 0,
            label: "bootstrap/Cargo.toml (check)".into(),
        };
        cache_write(&cwd, &spec, &tgt, &inp, &v).unwrap();
        // Second write with same content — must be a no-op (fast-path
        // succeeds; no re-write, no error).
        cache_write(&cwd, &spec, &tgt, &inp, &v).unwrap();
        let got = cache_read(&cwd, &spec, &tgt, &inp).unwrap();
        assert_eq!(got, v);
    }

    #[test]
    fn input_key_discriminates_across_dimensions() {
        let cwd = tempdir();
        let spec_a = SpecOid::new("aa".repeat(32));
        let spec_b = SpecOid::new("dd".repeat(32));
        let tgt = TargetOid::new("bb".repeat(32));
        let inp = InputsOid::new("cc".repeat(32));
        let v = CachedVerdict {
            verdict: "success".into(),
            objective: 0.0,
            dark_count: 0,
            label: "l".into(),
        };
        cache_write(&cwd, &spec_a, &tgt, &inp, &v).unwrap();
        // Different spec_oid → miss.
        assert!(cache_read(&cwd, &spec_b, &tgt, &inp).is_none());
        // Different target_oid → miss.
        let tgt2 = TargetOid::new("ee".repeat(32));
        assert!(cache_read(&cwd, &spec_a, &tgt2, &inp).is_none());
        // Different inputs_oid → miss.
        let inp2 = InputsOid::new("ff".repeat(32));
        assert!(cache_read(&cwd, &spec_a, &tgt, &inp2).is_none());
    }

    #[test]
    fn spec_oid_is_content_addressed() {
        let a = compute_spec_oid(b"foo");
        let b = compute_spec_oid(b"foo");
        let c = compute_spec_oid(b"bar");
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_eq!(a.as_str().len(), 64);
    }

    #[test]
    fn target_oid_discriminates_by_check_field() {
        let a = compute_target_oid("tests", "cargo", "test");
        let b = compute_target_oid("tests", "cargo", "check");
        let c = compute_target_oid("lint", "cargo", "clippy");
        assert_ne!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);
    }

    #[test]
    fn inputs_oid_reflects_file_contents() {
        let dir = tempdir();
        fs::write(dir.join("a.rs"), b"fn a() {}").unwrap();
        fs::write(dir.join("Cargo.toml"), b"[package]\nname=\"x\"").unwrap();
        let manifest = dir.join("Cargo.toml");
        let a = compute_inputs_oid(&manifest);
        // Same state → same OID.
        let b = compute_inputs_oid(&manifest);
        assert_eq!(a, b);
        // Mutate a source file → different OID.
        fs::write(dir.join("a.rs"), b"fn a() { let _ = 1; }").unwrap();
        let c = compute_inputs_oid(&manifest);
        assert_ne!(a, c);
    }
}
