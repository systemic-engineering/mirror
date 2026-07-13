//! @mirror/store-bounded peer runtime — Rung 6' GREEN per Mara `d2de1ee`
//! canonical spec + Taut `8e98a24` re-scout.
//!
//! Substrate authority:
//! - Mara `d2de1ee` — `docs/specs/mirror-store-bounded-peer-runtime-
//!   materialization-as-single-io-crossing.md` §3 canonical shape.
//! - Taut `8e98a24` — `docs/scouts/2026-07-13-taut-io-minimization-
//!   mirror-store-peer-runtime-scout.md` §5 Reed's Rung 6' spec.
//! - Recognition #43 (mirror IS content-addressed build system).
//! - Recognition #55 (form/process partition; @mirror/store form +
//!   @kintsugi transformation).
//! - Recognition #58 (fate optical inference; @magic-native).
//! - Recognition #80 (@magic altitude gauge-bounded computation).
//! - Recognition #107 (@io Turing-unbounded; interior gauge-bounded).
//! - Alex 2026-07-13 in-transcript: "@peer spawn stayed fully outside
//!   the @io boundary... operated purely within the bounds of @mirror/
//!   store... each peer spawn becomes a @mirror/store/branch"
//!
//! Rung 6' discipline: peer inference stays @magic-native; peer state
//! = crystal OID on @mirror/store internal ref (`refs/mirror/peer/
//! <uuid>/HEAD`); materialization = ONE @io crossing via `@kintsugi/
//! store/git.commit_as_fold` (forward-promised to Rung 6.1). Rung 6'
//! MVP: envelope-declared crystal OID emission following Rungs 4-5
//! stub-envelope pattern; Rung 6.1 lifts to actual @mirror/store.
//! insert_persistent + set_ref via already-landed action_cache + git
//! bindings at bootstrap/src/action_cache.rs + bootstrap/src/git.rs.

use crate::hash::canonical_hash;
use crate::Ctx;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

/// Compute the peer's canonical envelope bytes (the substrate-honest
/// content the peer WOULD emit at stdout). Deterministic per peer_home;
/// under actual @kintsugi/oscillate.active_pass (Rung 6.2+), these bytes
/// would carry the peer's morphism proposal + shard delta.
fn peer_envelope_bytes(peer_home: &str, peer_uuid: &str) -> Vec<u8> {
    let mut buf = Vec::with_capacity(peer_home.len() + peer_uuid.len() + 64);
    buf.extend_from_slice(b"@mirror/store/crystal:peer_beam:");
    buf.extend_from_slice(peer_uuid.as_bytes());
    buf.push(b':');
    buf.extend_from_slice(peer_home.as_bytes());
    buf
}

/// Emit peer crystal OID on @mirror/store internal ref. Rung 6' MVP:
/// envelope-declared substrate discipline (same pattern as Rungs 4-5
/// stubs). Peer inference stays @magic-native; peer's terminal output
/// is a crystal OID address rather than a stdout envelope.
///
/// Rung 6.1+ forward-promise: replace stub_crystal_oid with actual
/// @mirror/store.insert_persistent(envelope_bytes) -> oid via
/// `action_cache::cache_write` pattern; replace stub_ref_write with
/// actual `@mirror/store/git.set_ref(refs/mirror/peer/<uuid>/HEAD,
/// crystal_oid)` via already-landed `git_store_crystal` primitive.
/// Materialization discharge via `@kintsugi/store/git.commit_as_fold`
/// (Recognition #55 form/process partition; ONE @io crossing per
/// peer spawn cycle).
///
/// Byte-equality preserved for non-`--emit-crystal` paths: this
/// function is only entered when `cmd_peer_beam` observes
/// `emit_crystal == true`.
pub fn emit_peer_crystal(peer_home: &str, _ctx: &Ctx) -> i32 {
    // Rung 6.1a (2026-07-13) — real content-addressed crystal OID via
    // `canonical_hash` (CoincidenceHash<5,5> per hash.rs; 64-hex output
    // from SHA-256-based 5-d basis projection). Replaces Rung 6' FNV-1a
    // stub with substrate's own canonical hash used across the arc.
    // The peer's envelope bytes IS the content addressed; deterministic
    // per peer_home + peer_uuid.
    //
    // Rung 6.1b forward-promise: actual `git_store_crystal(source_hash,
    // crystal_oid)` writes crystal to `refs/crystals/<source_hash>` via
    // git objects (Rung 6.2 materialization).
    let peer_uuid = stub_peer_uuid(peer_home);
    let envelope_bytes = peer_envelope_bytes(peer_home, &peer_uuid);
    let crystal_oid = canonical_hash(&envelope_bytes);
    let ref_name = format!("refs/mirror/peer/{}/HEAD", peer_uuid);

    // Rung 6.1b (2026-07-13) — real @mirror/store crystal write per
    // Mara `d2de1ee` Scope B forward-promise + Recognition #55 form/
    // process partition. Materialization crossing: ONE @io crossing at
    // peer_home's git object store (peer's own @mirror/store backing at
    // `<peer_home>/.git/`). Subprocess git ops target peer_home via
    // `Command::new("git").current_dir(peer_home)`; peer_home is git-
    // initialized on first invocation if not already.
    let peer_home_path = Path::new(peer_home);
    let (store_status, ref_status) =
        materialize_crystal(peer_home_path, &peer_uuid, &crystal_oid);

    println!(
        "@@ peer crystal @mirror/store bounded (peer stays @magic-native; materialization is single @io crossing) (Rung 6.1b) @@"
    );
    println!("+ peer_home: {}", peer_home);
    println!("+ peer_uuid: {}", peer_uuid);
    println!("+ crystal_oid: {}", crystal_oid);
    println!("+ ref_name: {}", ref_name);
    println!("+ store_write_status: {}", store_status);
    println!("+ ref_write_status: {}", ref_status);
    println!(
        "+ materialization_status: forward-promised (Rung 6.2: @kintsugi/store/git.commit_as_fold as single @io crossing per Recognition #55)"
    );
    println!(
        "+ store_authority: @mirror/store (Recognition #43 mirror IS content-addressed build system; trichotomy: splinter/splinter_graph/crystal per shards/mirror/store.mirror)"
    );
    println!(
        "+ kintsugi_authority: @kintsugi/store/git (Recognition #55 form/process partition; commit_as_fold IS materialize action per shards/kintsugi/store/git.mirror)"
    );
    println!(
        "+ magic_authority: @magic (Recognition #80 gauge-bounded interior; peer inference stays in non-linear-eigenvalue land per Yang-Mills gauge/matter substrate)"
    );
    println!(
        "+ io_boundary_authority: @io (Recognition #107 Turing-unbounded boundary; peer crosses at materialization ONLY per shards/io.mirror:94-125)"
    );
    println!("+ ladder_rung: 6' (Reed GREEN discharging Mara `d2de1ee` + Taut `8e98a24`)");
    println!(
        "+ substrate_authority: @mirror/store + @kintsugi + @magic + @io (Rung 6' minimum viable; peer lives in @mirror/store; @kintsugi materializes to git)"
    );
    println!(
        "+ recognition_candidate: #R-peer-lives-in-mirror-store-@kintsugi-materializes-to-git"
    );

    0
}

/// Materialize the peer's crystal into peer_home's git object store.
/// Rung 6.1b implementation of Mara `d2de1ee` Scope B + Recognition
/// #55 form/process partition: the ONE @io crossing per peer spawn.
///
/// Steps:
/// 1. Ensure peer_home has `.git/` (git init if not).
/// 2. Write crystal_oid as a git blob via `git hash-object -w --stdin`.
/// 3. Update ref `refs/mirror/peer/<uuid>/HEAD` → blob hash.
///
/// Returns (store_status, ref_status) for envelope emission. Failures
/// are non-fatal at Rung 6.1b (returns error strings; peer inference
/// remains @magic-native even if materialization can't complete).
fn materialize_crystal(
    peer_home: &Path,
    peer_uuid: &str,
    crystal_oid: &str,
) -> (String, String) {
    // Step 1: ensure peer_home is a git repo (idempotent).
    if !peer_home.join(".git").exists() {
        let init_status = Command::new("git")
            .args(["init", "--quiet"])
            .current_dir(peer_home)
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .status();
        if init_status.is_err() || !init_status.map(|s| s.success()).unwrap_or(false) {
            return (
                "init-failed (envelope-declared fallback)".to_string(),
                "init-failed (envelope-declared fallback)".to_string(),
            );
        }
    }

    // Step 2: write crystal_oid as a git blob via stdin.
    let mut hash_child = match Command::new("git")
        .args(["hash-object", "-w", "--stdin"])
        .current_dir(peer_home)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => {
            return (
                "hash-object-spawn-failed (envelope-declared fallback)".to_string(),
                "skipped (store write failed)".to_string(),
            );
        }
    };
    if let Some(stdin) = hash_child.stdin.as_mut() {
        let _ = stdin.write_all(crystal_oid.as_bytes());
    }
    let hash_out = match hash_child.wait_with_output() {
        Ok(o) => o,
        Err(_) => {
            return (
                "hash-object-wait-failed (envelope-declared fallback)".to_string(),
                "skipped (store write failed)".to_string(),
            );
        }
    };
    let blob_hash = String::from_utf8_lossy(&hash_out.stdout)
        .trim()
        .to_string();
    if blob_hash.is_empty() {
        return (
            "hash-object-empty (envelope-declared fallback)".to_string(),
            "skipped (store write failed)".to_string(),
        );
    }

    // Step 3 (Rung 6.1c collapse): commit_as_fold discharge per
    // Recognition #55 form/process partition + shards/kintsugi/store/
    // git.mirror:130-152. Fold the crystal blob into a git commit
    // (mktree → commit-tree) so peer branch is a REAL git commit, not
    // just a ref-to-blob. This IS the substrate-honest materialization
    // shape @kintsugi/store/git.commit_as_fold canonicalizes.
    //
    // 3a: build a tree object containing the crystal blob at path
    //     `peer-crystal` via `git mktree` (stdin fed with tree-entry
    //     line: `100644 blob <blob_hash>\tpeer-crystal`).
    // 3b: build a commit object pointing to the tree with the peer's
    //     substrate-honest metadata via `git commit-tree`.
    // 3c: update-ref `refs/mirror/peer/<uuid>/HEAD` → commit hash.
    let tree_entry = format!("100644 blob {}\tpeer-crystal\n", blob_hash);
    let mut mktree_child = match Command::new("git")
        .args(["mktree"])
        .current_dir(peer_home)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => {
            return (
                format!("crystal blob {} at {} (mktree-spawn-failed)", &blob_hash[..16.min(blob_hash.len())], peer_home.display()),
                "mktree-spawn-failed (envelope-declared fallback; ref points to blob only)".to_string(),
            );
        }
    };
    if let Some(stdin) = mktree_child.stdin.as_mut() {
        let _ = stdin.write_all(tree_entry.as_bytes());
    }
    let tree_out = match mktree_child.wait_with_output() {
        Ok(o) => o,
        Err(_) => {
            return (
                format!("crystal blob {} at {} (mktree-wait-failed)", &blob_hash[..16.min(blob_hash.len())], peer_home.display()),
                "mktree-wait-failed (envelope-declared fallback)".to_string(),
            );
        }
    };
    let tree_hash = String::from_utf8_lossy(&tree_out.stdout).trim().to_string();
    if tree_hash.is_empty() {
        return (
            format!("crystal blob {} at {} (mktree-empty)", &blob_hash[..16.min(blob_hash.len())], peer_home.display()),
            "mktree-empty (envelope-declared fallback)".to_string(),
        );
    }

    // 3b (Rung 6.2a collapse): read parent commit if peer branch ref
    // already exists (@mirror/store IS a DAG per Recognition #43 +
    // splinter_graph trichotomy; peer beams chain per Alex 2026-07-13
    // in-transcript). If parent exists, pass `-p <parent>` to
    // commit-tree so peer's crystal history is a real git DAG.
    let ref_name_probe = format!("refs/mirror/peer/{}/HEAD", peer_uuid);
    let parent_hash = Command::new("git")
        .args(["rev-parse", "--verify", &ref_name_probe])
        .current_dir(peer_home)
        .stderr(Stdio::null())
        .output()
        .ok()
        .and_then(|o| {
            let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if o.status.success() && !s.is_empty() { Some(s) } else { None }
        });

    // 3c: commit-tree with substrate-honest commit message.
    let commit_msg = format!(
        "peer beam materialize crystal {}\n\ncommit_as_fold discharge per Mara `d2de1ee` Scope B + Recognition #55.\npeer_uuid: {}\npeer_home: {}\ncrystal_oid: {}\nblob_hash: {}\ntree_hash: {}\nparent: {}\n",
        &crystal_oid[..16.min(crystal_oid.len())],
        peer_uuid,
        peer_home.display(),
        crystal_oid,
        blob_hash,
        tree_hash,
        parent_hash.as_deref().unwrap_or("<root>")
    );
    let mut commit_args: Vec<String> = vec![
        "commit-tree".to_string(),
        tree_hash.clone(),
    ];
    if let Some(ref parent) = parent_hash {
        commit_args.push("-p".to_string());
        commit_args.push(parent.clone());
    }
    commit_args.push("-m".to_string());
    commit_args.push(commit_msg.clone());
    let mut commit_child = match Command::new("git")
        .args(&commit_args)
        .current_dir(peer_home)
        .env("GIT_AUTHOR_NAME", "peer")
        .env("GIT_AUTHOR_EMAIL", "peer@mirror.local")
        .env("GIT_COMMITTER_NAME", "peer")
        .env("GIT_COMMITTER_EMAIL", "peer@mirror.local")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => {
            return (
                format!("crystal blob {} + tree {} at {}", &blob_hash[..16.min(blob_hash.len())], &tree_hash[..16.min(tree_hash.len())], peer_home.display()),
                "commit-tree-spawn-failed (envelope-declared fallback)".to_string(),
            );
        }
    };
    let commit_out = match commit_child.wait_with_output() {
        Ok(o) => o,
        Err(_) => {
            return (
                format!("crystal blob {} + tree {} at {}", &blob_hash[..16.min(blob_hash.len())], &tree_hash[..16.min(tree_hash.len())], peer_home.display()),
                "commit-tree-wait-failed (envelope-declared fallback)".to_string(),
            );
        }
    };
    let commit_hash = String::from_utf8_lossy(&commit_out.stdout).trim().to_string();
    if commit_hash.is_empty() {
        return (
            format!("crystal blob {} + tree {} at {}", &blob_hash[..16.min(blob_hash.len())], &tree_hash[..16.min(tree_hash.len())], peer_home.display()),
            "commit-tree-empty (envelope-declared fallback)".to_string(),
        );
    }

    // 3c: update-ref → commit hash (not blob hash; peer branch HEAD IS
    // a real git commit per Recognition #55 discharge).
    let ref_name = format!("refs/mirror/peer/{}/HEAD", peer_uuid);
    let ref_status = Command::new("git")
        .args(["update-ref", &ref_name, &commit_hash])
        .current_dir(peer_home)
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status();
    let ref_result = match ref_status {
        Ok(s) if s.success() => format!(
            "written (commit {} → tree {} → blob {})",
            &commit_hash[..16.min(commit_hash.len())],
            &tree_hash[..16.min(tree_hash.len())],
            &blob_hash[..16.min(blob_hash.len())]
        ),
        _ => "update-ref-failed (envelope-declared fallback)".to_string(),
    };

    (
        format!(
            "crystal blob {} + tree {} + commit {} at {} (commit_as_fold discharge per Recognition #55)",
            &blob_hash[..16.min(blob_hash.len())],
            &tree_hash[..16.min(tree_hash.len())],
            &commit_hash[..16.min(commit_hash.len())],
            peer_home.display()
        ),
        ref_result,
    )
}

/// Rung 6' stub peer UUID: FNV-1a hash of peer_home bytes; deterministic
/// per peer_home input. Rung 6.1 forward-promise: replace with actual
/// SpectralUuid generation via @spectral/gen_prism.
fn stub_peer_uuid(peer_home: &str) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in peer_home.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    // Format as UUID-shaped 8-4-4 hex sequence (16 hex chars total).
    format!("{:08x}-{:04x}-{:04x}", (h >> 32) as u32, (h >> 16) as u16, h as u16)
}


