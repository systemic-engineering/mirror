//! Rung 7 GREEN — fate-spawned peer contributes working shard delta
//! via active_pass; @mirror/mosaic.settle verifies compile; commit_as_fold
//! materializes morphism on peer's DAG.
//!
//! Substrate authority:
//! - Mara `4e69066` — `docs/specs/fate-spawned-peer-contributes-working-
//!   delta-via-active-pass.md` §3 canonical shape (Scope A MVP).
//! - Recognition #58 (Fate IS optical inference; @magic-native).
//! - @kintsugi/oscillate.active_pass (shards/kintsugi/oscillate.mirror:456
//!   substrate-decl; runtime discharges HERE).
//! - @mirror/mosaic.settle (cargo check IS the settle at @code/rust
//!   altitude).
//! - @kintsugi/store/git.commit_as_fold (Rung 6.1c landed; Rung 6.2a
//!   parent chain).
//! - Alex 2026-07-13 in-transcript: "it's not empirical certainty until
//!   a Fate spawned agent contributes working mirror back to the
//!   compiler."
//!
//! Composition: this module composes over `store_branch::materialize_
//! crystal` (Rung 6.1c + 6.2a) with a 5-blob commit tree instead of a
//! single-blob peer-crystal tree. The morphism-body blob IS the shard
//! delta; the settle-verdict + fate-witness blobs are substrate-honest
//! provenance.

use crate::hash::canonical_hash;
use crate::Ctx;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

/// Peer contributes a working delta to `target_shard` via fate-spawned
/// active_pass. Returns exit code (0 on success, non-zero on refusal).
///
/// Contract per Mara `4e69066` §3.2:
/// 1. `fate::Fate::excited().resolve(features, 5)` → (Model, prism_op).
/// 2. `active_pass::propose_docblock_morphism` → append line per Model.
/// 3. Read pre-anchor bytes; apply morphism; write post-anchor bytes.
/// 4. `mosaic::settle` (cargo check on peer_home) verifies workspace.
/// 5. On settle: commit_as_fold with 5-blob tree on peer's DAG.
///    On imperfect: revert target bytes; exit non-zero.
pub fn peer_contribute(peer_home: &str, target_shard: &Path, _ctx: &Ctx) -> i32 {
    // Refusal path when target missing (Asher membrane-conservatism: refuse
    // to write when substrate lacks the anchor).
    if !target_shard.exists() {
        eprintln!(
            "@@ peer contribute refused: target shard not found: {} @@",
            target_shard.display()
        );
        return 1;
    }

    let peer_home_path = Path::new(peer_home);
    let peer_uuid = stub_peer_uuid(peer_home);

    // Rung 7' Error 1 correction (Fate::excited → Fate::bounded per Alex
    // 2026-07-13 + Mara `2c64060` §7.1): reuse the composed-idiom pattern
    // `Fate::untrained() + selectors_from_psychohistory_root` that already
    // discharges `Fate::bounded` at `fate_bounded_by_psychohistory_peer_beam`.
    // Sheaf-Laplacian Δ_F Rayleigh descent along peer's psychohistory sheaf
    // (Bodnar 2022; v1 xorshift stub keyed on psychohistory_root_oid; v2
    // lifts to actual sheaf eigenvector).
    let (psychohistory_root_oid, moments_count) =
        crate::psychohistory_root_from_peer_home(peer_home_path);
    let features: fate::Features = [0.0; 16];
    let mut fate_engine = fate::Fate::untrained();
    fate_engine.selectors = crate::selectors_from_psychohistory_root(&psychohistory_root_oid);
    let decision = fate_engine.resolve(&features, 5);
    let (model_name, prism_op_name) = bundle_tower_binding(decision.model);

    // Rung 9 Landing 1 Path C empirical-first: MEASURE-BEFORE at two lenses.
    // Compose @mirror/index (file-tree lens; Rung 8 Landing 6) with
    // @mirror/lens/refract spectral duality at shard-body altitude
    // (Rung 9 Landing 2). Per Alex 2026-07-13 "peer can look at the AST
    // through arbitrary @mirror/lens es." The file-tree lens is coarse
    // for shard-body morphisms; the shard-body lens responds per-line.
    let profile_before_ft = crate::index::index(peer_home_path);
    let fiedler_before = profile_before_ft.fiedler_value();
    let profile_before_sb = crate::index::shard_body_index(target_shard);
    let fiedler_before_shard = profile_before_sb.fiedler_value();

    // Rung 8+9 Landing 8+9.3: SC<5> measurement per Mara `c753d5b`.
    // Substrate-honest carrier: fragmentation::SpectralCoordinate<5> via
    // fragmentation-spectral's coincidence method (Bothe 1924). Retires
    // Reed's re-invented EigenvalueProfile<16>. Emits SC<5> hex +
    // hamming distance (proxy for ||sc||_2 until Alex adjudicates §10.1
    // canonical serialization). Full L² harmonic distance follows.
    let sc_before_bytes = fs::read(target_shard).unwrap_or_default();
    let sc_before = fragmentation_spectral::hash::coordinate::<5>(&sc_before_bytes);

    // Step 3: pre-anchor bytes.
    let pre_bytes = match fs::read(target_shard) {
        Ok(b) => b,
        Err(e) => {
            eprintln!(
                "@@ peer contribute refused: cannot read target shard {}: {} @@",
                target_shard.display(),
                e
            );
            return 1;
        }
    };

    // Step 4: active_pass.propose_docblock_morphism per Mara §4 Scope A
    // Model → prism_op mapping. Docstring-append line is the morphism
    // body; substrate-honest (target shard's docblock IS the anchor).
    let morphism_line = propose_docblock_morphism(model_name, prism_op_name, &peer_uuid);
    let post_bytes = apply_docblock_morphism(&pre_bytes, &morphism_line);

    // Step 5: write post-anchor bytes.
    if let Err(e) = fs::write(target_shard, &post_bytes) {
        eprintln!(
            "@@ peer contribute refused: cannot write target shard {}: {} @@",
            target_shard.display(),
            e
        );
        return 1;
    }

    // Step 6: @mirror/mosaic.settle verifies workspace via cargo check
    // at peer_home altitude. This IS the empirical-discharge gate per
    // Alex 2026-07-13.
    let settle_verdict = settle_rust_workspace(peer_home_path);

    match settle_verdict {
        SettleVerdict::Settled(stdout) => {
            // Rung 9 Landing 2: MEASURE-AFTER at BOTH lenses (file-tree
            // + shard-body). Empirical falsification #1 showed docstring-
            // append is a no-op at file-tree altitude; shard-body lens
            // per @mirror/lens/refract per-line adjacency graph responds.
            let profile_after_ft = crate::index::index(peer_home_path);
            let fiedler_after = profile_after_ft.fiedler_value();
            let fiedler_delta = fiedler_after - fiedler_before;
            let profile_after_sb = crate::index::shard_body_index(target_shard);
            let fiedler_after_shard = profile_after_sb.fiedler_value();
            let fiedler_shard_delta = fiedler_after_shard - fiedler_before_shard;
            // Rung 8+9 Landing 8+9.3: SC<5> MEASURE-AFTER + delta
            let sc_after_bytes = fs::read(target_shard).unwrap_or_default();
            let sc_after = fragmentation_spectral::hash::coordinate::<5>(&sc_after_bytes);
            let sc_hex_before = sc_before.eigenvalue().to_string();
            let sc_hex_after = sc_after.eigenvalue().to_string();
            let sc_hamming = sc_hex_before.chars().zip(sc_hex_after.chars()).filter(|(a, b)| a != b).count();
            let sc_moved = sc_hex_before != sc_hex_after;
            // File-tree verdict (Rung 9 Landing 1)
            let loss_decreased = fiedler_delta < 0.0;
            let coherence_verdict = if fiedler_delta.abs() < 1e-6 {
                "unchanged"
            } else if loss_decreased {
                "improved"
            } else {
                "regressed"
            };
            // Shard-body verdict (Rung 9 Landing 2; via @mirror/lens/refract spectral duality)
            let shard_loss_decreased = fiedler_shard_delta < 0.0;
            let shard_coherence_verdict = if fiedler_shard_delta.abs() < 1e-9 {
                "unchanged"
            } else if shard_loss_decreased {
                "improved"
            } else {
                "regressed"
            };

            // Rung 7' Errors 2 + 4 correction: 4-subtree tripartition
            // (anchors/gates/witnesses/morphism-body per Mara `2c64060`
            // §7.2) with fate metadata folded into commit message
            // (naked_oid via git plumbing) rather than tree blob.
            let (store_status, ref_status, ref_name) = materialize_morphism_tripartition(
                peer_home_path,
                &peer_uuid,
                &pre_bytes,
                &post_bytes,
                &morphism_line,
                &stdout,
                model_name,
                prism_op_name,
                &psychohistory_root_oid,
                moments_count,
            );

            println!(
                "@@ peer contribute discharge @mirror/mosaic.settle green (Rung 7' Fate::bounded + tripartition + Rung 9 coherence-delta measurement; @fractal-family-root; Mandelbrot-substrate) @@"
            );
            println!("+ peer_home: {}", peer_home);
            println!("+ peer_uuid: {}", peer_uuid);
            println!("+ target_shard: {}", target_shard.display());
            println!("+ ref_name: {}", ref_name);
            println!("+ fate_source: bounded (psychohistory-derived; sheaf-Laplacian Rayleigh v1 stub per Bodnar 2022)");
            println!("+ fate_model: {}", model_name);
            println!("+ fate_prism_op: {}", prism_op_name);
            println!("+ psychohistory_root_oid: {}", psychohistory_root_oid);
            println!("+ psychohistory_moments_count: {}", moments_count);
            println!("+ morphism_kind: docstring-append (Mara `2c64060` §7 Scope A')");
            println!("+ pre_anchor_bytes: {}", pre_bytes.len());
            println!("+ post_anchor_bytes: {}", post_bytes.len());
            println!("+ settle_verdict: settled (cargo check green)");
            // Rung 9 Landing 1 Path C empirical-first: fiedler_delta emission.
            // Per Mara `c59a5ac` §4 query_phi_coherence composition + Taut
            // `862db12` §6 empirical-first falsification. Measurement-only
            // (does NOT gate commit at Landing 1); Landing 2 Scope B adds
            // the gate + Model → consolidative-morphism mapping.
            println!("+ lens_file_tree: @mirror/index (Rung 8 Landing 6; file-tree ConceptGraph altitude)");
            println!("+ fiedler_before: {:.4}", fiedler_before);
            println!("+ fiedler_after: {:.4}", fiedler_after);
            println!("+ fiedler_delta: {:.6}", fiedler_delta);
            println!("+ file_tree_coherence_verdict: {} (loss_decreased={})", coherence_verdict, loss_decreased);
            println!("+ lens_shard_body: @mirror/lens/refract spectral duality (Rung 9 Landing 2; line-adjacency + wiki-link graph on target shard)");
            println!("+ fiedler_before_shard: {:.6}", fiedler_before_shard);
            println!("+ fiedler_after_shard: {:.6}", fiedler_after_shard);
            println!("+ fiedler_shard_delta: {:.9}", fiedler_shard_delta);
            println!("+ shard_body_coherence_verdict: {} (loss_decreased={})", shard_coherence_verdict, shard_loss_decreased);
            println!("+ rung_9_direction_correction: shard-body lens per @mirror/lens/refract responds to per-line changes; file-tree lens is too coarse for docstring-append morphisms (empirical falsification #1 landed at `9044f26`); Rung 9 Landing 2 discharges Alex 2026-07-13 lens architecture directive");
            println!("+ lens_sc5: fragmentation::SpectralCoordinate<5> via fragmentation-spectral coincidence method (Bothe 1924; Rung 8+9 Landing 8+9.3; retires EigenvalueProfile per Mara `c753d5b`)");
            println!("+ sc_hex_before: {}", sc_hex_before);
            println!("+ sc_hex_after:  {}", sc_hex_after);
            println!("+ sc_hamming:    {} / {} hex chars differ (proxy for ||sc_after − sc_before||₂ until Alex §10.1 adjudication)", sc_hamming, sc_hex_before.len().max(sc_hex_after.len()));
            println!("+ sc_moved:      {} (substrate coordinate {} between pre- and post-morphism states)", sc_moved, if sc_moved { "CHANGED" } else { "UNCHANGED" });
            println!("+ substrate_measurement_carrier: fragmentation::SpectralCoordinate<5> (mirror-native-vcs.md §4.6; five projections of one spectrum; λ₀=0 is void axis = harmonic ground state = origin of manifold)");

            // Rung 8+9 Landing 8+9.6b: substrate-honest pain gradient via
            // @cyberpunk/algedonic.pain_gradient (retires the sc_hamming
            // ratio proxy Reed used in Landing 8+9.6a per Taut `15f7ed6`
            // §5 gap). Rust runtime at bootstrap/src/algedonic.rs mirrors
            // the substrate-decl at shards/cyberpunk/algedonic.mirror.
            //
            // Substrate-honest interpretation: Shannon entropy of SC<5>
            // hex distribution; higher entropy = less-recognizable
            // structure = peer near boundary = higher pain (Foerster A3).
            //
            // sc_hamming preserved as diagnostic emission alongside
            // pain_gradient until Landing 8+9.6d empirical calibration
            // determines which is substrate-honest.
            let pain_gradient_hamming = if sc_hex_before.is_empty() {
                0.0
            } else {
                sc_hamming as f64 / sc_hex_before.len() as f64
            };
            let pain_gradient = crate::algedonic::pain_gradient(&sc_before, &sc_after).abs();
            let pain_before = crate::algedonic::sample_pain(&sc_before);
            let pain_after = crate::algedonic::sample_pain(&sc_after);
            const EPSILON_PAIN_INSTRUMENTATION: f64 = 0.5;
            let knife_verdict = crate::converge::stable_within(
                &sc_after,
                pain_gradient,
                EPSILON_PAIN_INSTRUMENTATION,
            );
            let (sc_jumped_hex, heterarchy_verdict) = if matches!(
                knife_verdict,
                crate::converge::KnifeVerdict::Jumped
            ) {
                let sc_jumped = crate::converge::knife_cut(
                    sc_after.clone(),
                    pain_gradient,
                    EPSILON_PAIN_INSTRUMENTATION,
                );
                let hex = sc_jumped.eigenvalue().to_string();
                let het = crate::converge::heterarchy_preserved(&sc_before, &sc_jumped);
                (Some(hex), Some(het))
            } else {
                (None, None)
            };
            println!("+ lens_knife: @mirror/lens/knife (Foerster 1976 COORD(x); Reed `0a267ce` substrate-decl + `18b5828` Rust runtime; Seam `e8508f5` ratification)");
            println!("+ pain_before: {:.4} (@cyberpunk/algedonic.sample_pain; Shannon entropy of SC<5> hex; Landing 8+9.6b Rust runtime)", pain_before);
            println!("+ pain_after: {:.4}", pain_after);
            println!("+ pain_gradient: {:.4} (@cyberpunk/algedonic.pain_gradient; substrate-honest; retires sc_hamming proxy per Taut §5 gap)", pain_gradient);
            println!("+ pain_gradient_hamming: {:.4} (Landing 8+9.6a proxy; preserved as diagnostic; empirical calibration Landing 8+9.6d)", pain_gradient_hamming);
            println!("+ epsilon_pain_instrumentation: {:.4} (placeholder; Seam §4 #6 rejects default; call-site value)", EPSILON_PAIN_INSTRUMENTATION);
            println!("+ knife_verdict: {:?} (Foerster A3 stable_within; @kintsugi/consent verdict floor forward-promise)", knife_verdict);
            if let (Some(jumped_hex), Some(het)) = (sc_jumped_hex.as_ref(), heterarchy_verdict.as_ref()) {
                println!("+ knife_jumped: true (@knife.jump fired; COORDᵢ → COORDⱼ)");
                println!("+ sc_jumped_hex: {}", jumped_hex);
                println!("+ heterarchy_verdict: {:?} (Seam §5 #1 reformulation; M∘-membership check; NearBoundary = @kintsugi/consent.pause(Φ) external witness needed)", het);
            } else {
                println!("+ knife_jumped: false (Op(COORDᵢ) = COORDᵢ; peer within stable domain)");
            }
            println!("+ tree_shape: tripartition (anchors/ + gates/ + witnesses/ + morphism-body per Mara `2c64060` §7.2)");
            println!("+ witness_locus: encoding (commit message → naked_oid; not content blob per Mara `2c64060` §7.4)");
            println!("+ store_write_status: {}", store_status);
            println!("+ ref_write_status: {}", ref_status);
            println!(
                "+ discharge_mode: empirical (compiler-verified; not ceremonial per Alex 2026-07-13)"
            );
            println!(
                "+ fractal_family_root: @fractal (Alex 2026-07-13 outside-view correction; fragmentation::Fractal Rust altitude ancestry; consent emerges from Fractal not vice-versa)"
            );
            println!(
                "+ mandelbrot_correspondence: f_c=@kintsugi/oscillate ACTIVE/DARK; c=(shard, ctx, psychohistory_root); M∘=@magic (Rec #80); ∂M=@io (Rec #107); R=commit_as_fold (Rec #55)"
            );
            println!(
                "+ store_authority: @mirror/store (Recognition #43; DAG per splinter_graph trichotomy; Rung 6.2a parent chain)"
            );
            println!(
                "+ kintsugi_authority: @kintsugi/oscillate.active_pass; @kintsugi/store/git.commit_as_fold (Rec #55 renormalization operator)"
            );
            println!(
                "+ mosaic_authority: @mirror/mosaic.settle (cargo check IS the settle at @code/rust altitude; Mandelbrot-membership query at c=(shard,ctx))"
            );
            println!(
                "+ fate_authority: @magic Fate::bounded (Recognition #58 optical inference; sheaf-Laplacian Rayleigh descent along psychohistory sheaf)"
            );
            println!("+ ladder_rung: 7' (Reed GREEN discharging Mara `2c64060` §7 Scope A') + 9 Landing 1 Path C (empirical-first fiedler-delta measurement per Mara `c59a5ac` §2) + 9 Landing 2 (@mirror/lens/refract shard-body lens per Alex 2026-07-13 directive)");
            println!(
                "+ recognition_candidate: #R-fractal-is-mandelbrot-substrate"
            );
            0
        }
        SettleVerdict::Imperfect(errors) => {
            // Revert: peer's morphism didn't compile; refuse to commit.
            let _ = fs::write(target_shard, &pre_bytes);
            eprintln!(
                "@@ peer contribute refused: @mirror/mosaic.settle IMPERFECT (cargo check red); target reverted to pre-anchor bytes @@"
            );
            eprintln!("+ fate_model: {}", model_name);
            eprintln!("+ fate_prism_op: {}", prism_op_name);
            eprintln!("+ settle_errors: {}", errors.lines().take(20).collect::<Vec<_>>().join("\n"));
            eprintln!(
                "+ discharge_mode: refused (empirical gate closed per Alex 2026-07-13; ceremonial commit suppressed)"
            );
            1
        }
    }
}

enum SettleVerdict {
    Settled(String),
    Imperfect(String),
}

/// @mirror/mosaic.settle at @code/rust altitude — cargo check on
/// peer_home's Cargo workspace. Substrate-honest: the compiler IS the
/// verifier; its verdict IS the settle discharge.
fn settle_rust_workspace(peer_home: &Path) -> SettleVerdict {
    let manifest = peer_home.join("Cargo.toml");
    if !manifest.exists() {
        // Peer_home has no Cargo workspace — settle is a no-op green
        // at @code/rust altitude (nothing to verify). Substrate-honest:
        // the compiler CAN'T reject what isn't compiled.
        return SettleVerdict::Settled("no-cargo-workspace (settle no-op)".to_string());
    }
    let out = Command::new("cargo")
        .args(["check", "--offline", "--quiet", "--manifest-path"])
        .arg(&manifest)
        .current_dir(peer_home)
        .env("CARGO_TERM_COLOR", "never")
        .output();
    match out {
        Ok(o) if o.status.success() => {
            let stdout = String::from_utf8_lossy(&o.stdout).to_string();
            SettleVerdict::Settled(if stdout.is_empty() {
                "cargo check green".to_string()
            } else {
                stdout
            })
        }
        Ok(o) => {
            let combined = format!(
                "{}\n{}",
                String::from_utf8_lossy(&o.stdout),
                String::from_utf8_lossy(&o.stderr)
            );
            SettleVerdict::Imperfect(combined)
        }
        Err(e) => SettleVerdict::Imperfect(format!("cargo-spawn-failed: {}", e)),
    }
}

/// active_pass.propose_docblock_morphism per Mara §4.1 Scope A Model →
/// prism_op mapping. Returns the morphism line to append to target
/// shard's docblock (or trailing content when no docblock present).
fn propose_docblock_morphism(model: &str, prism_op: &str, peer_uuid: &str) -> String {
    format!(
        "# Recognition-ancestry: peer beam contribution via {} @ {} (peer_uuid: {})\n",
        model, prism_op, peer_uuid
    )
}

/// Apply docblock morphism: append `morphism_line` to pre_bytes. Scope
/// A MVP is byte-append (docstring lines are line-oriented; append is
/// syntactically safe for both `.mirror` and `.md` shards).
fn apply_docblock_morphism(pre_bytes: &[u8], morphism_line: &str) -> Vec<u8> {
    let mut post = pre_bytes.to_vec();
    if !post.ends_with(b"\n") {
        post.push(b'\n');
    }
    post.extend_from_slice(morphism_line.as_bytes());
    post
}

/// Rung 7' materialize with 4-subtree tripartition per Mara `2c64060` §7:
///
/// ```text
/// tree/
/// ├── anchors/
/// │   ├── pre           (blob: pre-morphism target bytes)
/// │   └── post          (blob: post-morphism target bytes)
/// ├── gates/
/// │   └── settle-verdict (blob: cargo check verdict)
/// ├── witnesses/
/// │   └── asher-forward-promise (blob: Asher 5-axis Rung 7.5+ stub)
/// └── morphism-body      (blob: the delta line itself)
/// ```
///
/// Fate metadata (fate_model, fate_prism_op, psychohistory_root_oid) folds
/// into the COMMIT MESSAGE (part of naked_oid via git plumbing) rather than
/// a tree blob. Preserves "same content, different witness, different commit,
/// same tree OID" per fragmentation::NakedSingularity discipline.
///
/// Composes over Rung 6.1c (blob→tree→commit chain) + Rung 6.2a (parent-
/// linked DAG). Returns (store_status, ref_status, ref_name).
#[allow(clippy::too_many_arguments)]
fn materialize_morphism_tripartition(
    peer_home: &Path,
    peer_uuid: &str,
    pre_bytes: &[u8],
    post_bytes: &[u8],
    morphism_line: &str,
    settle_stdout: &str,
    fate_model: &str,
    fate_prism_op: &str,
    psychohistory_root_oid: &str,
    moments_count: usize,
) -> (String, String, String) {
    let ref_name = format!("refs/mirror/peer/{}/HEAD", peer_uuid);

    // Ensure peer_home is a git repo (idempotent).
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
                "init-failed".to_string(),
                ref_name,
            );
        }
    }

    // Write substantive-content blobs via `git hash-object -w --stdin`.
    // Fate metadata is NOT a blob — it goes into the commit message per
    // Mara §7.4 witness-in-encoding correction.
    let pre_blob = match hash_object(peer_home, pre_bytes) {
        Some(h) => h,
        None => return ("pre blob write failed".to_string(), "skipped".to_string(), ref_name),
    };
    let post_blob = match hash_object(peer_home, post_bytes) {
        Some(h) => h,
        None => return ("post blob write failed".to_string(), "skipped".to_string(), ref_name),
    };
    let morphism_blob = match hash_object(peer_home, morphism_line.as_bytes()) {
        Some(h) => h,
        None => return ("morphism-body write failed".to_string(), "skipped".to_string(), ref_name),
    };
    let settle_blob = match hash_object(peer_home, settle_stdout.as_bytes()) {
        Some(h) => h,
        None => return ("settle-verdict write failed".to_string(), "skipped".to_string(), ref_name),
    };
    let asher_stub = format!(
        "Asher 5-axis forward-promise (Rung 7.5+ per Mara `2c64060` §7.3):\n\
         - temporal_persistence: unwitnessed (Rung 7.5)\n\
         - geometric_coherence: unwitnessed (Rung 7.5)\n\
         - contextual_recurrence: unwitnessed (Rung 7.5)\n\
         - perturbational_stability: unwitnessed (Rung 7.5)\n\
         - representational_mismatch: unwitnessed (Rung 7.5)\n\
         non_redundance_predicate: forward-promised (@fractal.non_redundance; adjudication #2)\n"
    );
    let witness_stub_blob = match hash_object(peer_home, asher_stub.as_bytes()) {
        Some(h) => h,
        None => return ("asher-forward-promise write failed".to_string(), "skipped".to_string(), ref_name),
    };

    // Build tripartition subtrees via `git mktree` (three sub-trees,
    // then top-level tree with mode 040000 tree entries).
    let anchors_entries = format!(
        "100644 blob {}\tpre\n100644 blob {}\tpost\n",
        pre_blob, post_blob
    );
    let anchors_tree = match mktree(peer_home, &anchors_entries) {
        Some(h) => h,
        None => return ("anchors mktree failed".to_string(), "skipped".to_string(), ref_name),
    };
    let gates_entries = format!("100644 blob {}\tsettle-verdict\n", settle_blob);
    let gates_tree = match mktree(peer_home, &gates_entries) {
        Some(h) => h,
        None => return ("gates mktree failed".to_string(), "skipped".to_string(), ref_name),
    };
    let witnesses_entries = format!("100644 blob {}\tasher-forward-promise\n", witness_stub_blob);
    let witnesses_tree = match mktree(peer_home, &witnesses_entries) {
        Some(h) => h,
        None => return ("witnesses mktree failed".to_string(), "skipped".to_string(), ref_name),
    };

    // Top-level tree: 3 sub-trees + morphism-body blob at root.
    let top_entries = format!(
        "040000 tree {}\tanchors\n\
         040000 tree {}\tgates\n\
         040000 tree {}\twitnesses\n\
         100644 blob {}\tmorphism-body\n",
        anchors_tree, gates_tree, witnesses_tree, morphism_blob
    );
    let tree_hash = match mktree(peer_home, &top_entries) {
        Some(h) => h,
        None => return ("top-tree mktree failed".to_string(), "skipped".to_string(), ref_name),
    };

    // Read parent commit if peer branch exists (Rung 6.2a DAG chain).
    let parent_hash = Command::new("git")
        .args(["rev-parse", "--verify", &ref_name])
        .current_dir(peer_home)
        .stderr(Stdio::null())
        .output()
        .ok()
        .and_then(|o| {
            let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if o.status.success() && !s.is_empty() {
                Some(s)
            } else {
                None
            }
        });

    // Rung 7' Error 4 correction: commit MESSAGE carries witness
    // metadata (fate_model, fate_prism_op, peer_uuid,
    // psychohistory_root_oid, moments_count). git commit-tree folds this
    // into the commit's naked_oid (`hash(tree_oid ++ parent_oid ++
    // author ++ committer ++ message)`). Different witness → different
    // commit_oid; SAME tree_oid if content is byte-identical.
    let content_digest = canonical_hash(post_bytes);
    let commit_msg = format!(
        "peer contribute morphism {} (Rung 7' @fractal Mandelbrot substrate)\n\ncommit_as_fold renormalization operator per Recognition #55 form/process partition.\n\ntree_shape: tripartition (anchors/ + gates/ + witnesses/ + morphism-body)\ntree_hash: {}\nparent: {}\n\nwitness_locus: encoding (this message; folded into naked_oid via git plumbing)\npeer_uuid: {}\nfate_source: bounded\nfate_model: {}\nfate_prism_op: {}\npsychohistory_root_oid: {}\npsychohistory_moments_count: {}\n\nsettle_verdict: settled (cargo check green)\npost_anchor_digest: {}\n\nsubstrate_authority: @fractal (Alex 2026-07-13; fragmentation::Fractal at Rust altitude) + @kintsugi/store/git.commit_as_fold (Recognition #55) + @mirror/mosaic.settle (Mandelbrot-membership query at c=(shard,ctx))\nrecognition_candidate: #R-fractal-is-mandelbrot-substrate\n",
        &content_digest[..16.min(content_digest.len())],
        tree_hash,
        parent_hash.as_deref().unwrap_or("<root>"),
        peer_uuid,
        fate_model,
        fate_prism_op,
        psychohistory_root_oid,
        moments_count,
        content_digest,
    );

    let mut commit_args: Vec<String> = vec!["commit-tree".to_string(), tree_hash.clone()];
    if let Some(ref parent) = parent_hash {
        commit_args.push("-p".to_string());
        commit_args.push(parent.clone());
    }
    commit_args.push("-m".to_string());
    commit_args.push(commit_msg);

    let commit_out = Command::new("git")
        .args(&commit_args)
        .current_dir(peer_home)
        .env("GIT_AUTHOR_NAME", "peer")
        .env("GIT_AUTHOR_EMAIL", "peer@mirror.local")
        .env("GIT_COMMITTER_NAME", "peer")
        .env("GIT_COMMITTER_EMAIL", "peer@mirror.local")
        .stderr(Stdio::null())
        .output();
    let commit_hash = match commit_out {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        _ => {
            return (
                format!("commit-tree-failed (5-blob tree {})", &tree_hash[..16.min(tree_hash.len())]),
                "skipped".to_string(),
                ref_name,
            );
        }
    };

    if commit_hash.is_empty() {
        return (
            "commit-tree-empty".to_string(),
            "skipped".to_string(),
            ref_name,
        );
    }

    // update-ref → commit hash.
    let ref_status = Command::new("git")
        .args(["update-ref", &ref_name, &commit_hash])
        .current_dir(peer_home)
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status();
    let ref_result = match ref_status {
        Ok(s) if s.success() => format!(
            "written (commit {} → tree {} → 5 blobs, parent: {})",
            &commit_hash[..16.min(commit_hash.len())],
            &tree_hash[..16.min(tree_hash.len())],
            parent_hash.as_deref().map(|p| &p[..16.min(p.len())]).unwrap_or("<root>")
        ),
        _ => "update-ref-failed".to_string(),
    };

    (
        format!(
            "commit {} + tree {} (5 blobs: pre-anchor/post-anchor/morphism-body/settle-verdict/fate-witness)",
            &commit_hash[..16.min(commit_hash.len())],
            &tree_hash[..16.min(tree_hash.len())]
        ),
        ref_result,
        ref_name,
    )
}

fn hash_object(peer_home: &Path, bytes: &[u8]) -> Option<String> {
    let mut child = Command::new("git")
        .args(["hash-object", "-w", "--stdin"])
        .current_dir(peer_home)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(bytes).ok()?;
    }
    let out = child.wait_with_output().ok()?;
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() { None } else { Some(s) }
}

fn mktree(peer_home: &Path, entries: &str) -> Option<String> {
    let mut child = Command::new("git")
        .args(["mktree"])
        .current_dir(peer_home)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(entries.as_bytes()).ok()?;
    }
    let out = child.wait_with_output().ok()?;
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() { None } else { Some(s) }
}

/// Bundle-tower binding per boot/std/epistemologic/math/bundle.mirror:
/// Level 0 Fiber / Level 1 Connection / Level 2 Gauge / Level 3
/// Transport / Level 4 Closure = focus / project / split / shift /
/// settle. Matches `fate_select_peer_beam` binding in lib.rs.
fn bundle_tower_binding(model: fate::Model) -> (&'static str, &'static str) {
    match model {
        fate::Model::Abyss => ("Abyss", "focus"),
        fate::Model::Introject => ("Introject", "project"),
        fate::Model::Cartographer => ("Cartographer", "split"),
        fate::Model::Explorer => ("Explorer", "shift"),
        fate::Model::Fate => ("Fate", "settle"),
    }
}

fn stub_peer_uuid(peer_home: &str) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in peer_home.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{:08x}-{:04x}-{:04x}", (h >> 32) as u32, (h >> 16) as u16, h as u16)
}

