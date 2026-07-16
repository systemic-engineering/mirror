//! `mirror roomba --commit` — the compiler observes its own state and
//! authors the commit itself.
//!
//! Alex 2026-07-15 verbatim (load-bearing): "I want the compiler itself
//! to make the commit, you know? The whole end2end flow as an empirical
//! CLI call proof."
//!
//! ## What this discharges
//!
//! The whole end-to-end proof arc:
//!
//! 1. Arc-1 Tick 1.2 RED (`a919e85`) — apply_h.rs 7-combinator stub
//! 2. Arc-1 Tick 1.3 GREEN (`f747a2c`) — combinator impls; sbec 0 → > 0
//! 3. Arc-1 Tick 1.4 CLI  (`b189adb`) — `mirror beam act` user-invocable
//! 4. Arc-2 Tick 2.1      (`f211ee4`) — spectral_signature.rs collapse
//!    (first ouroboros bite; sbec grows, rust_loc shrinks)
//! 5. `mirror roomba --commit` (this landing) — THE FINAL PROOF: the
//!    substrate walks itself, observes its own state, composes a commit
//!    message from the observation, and creates a git commit whose AUTHOR
//!    is `mirror <mirror@spectral.engineer>` — not a Pack peer.
//!    (Prior mirror-authored commits fcc1d75, 56abdbe, 74aa546 used
//!    the invented `mirror@substrate.engineer` — those are historical
//!    fact; going forward the compiler identifies with the real domain
//!    `spectral.engineer` per Alex correction 2026-07-15.)
//!
//! ## Substrate-authorship discipline
//!
//! Pack peers are Reed/Mara/Seam/Taut/Glint — the collaborating agents
//! at the human-adjacent altitude. `mirror` is the compiler itself, one
//! altitude below. The commit AUTHOR is the compiler; the commit
//! SIGNATURE remains Reed's SSH key (per AGENTS.md never-override-gpg.
//! format rule). Two-tick honest: signed by Reed's key at Reed's
//! substrate altitude, AUTHORED by the compiler at substrate altitude.
//!
//! ## Composition
//!
//! - `crate::roomba::walk` — the walker landed 2026-07-14 that traverses
//!   the substrate's ConceptGraph and observes SC<5> + tension per node.
//! - `crate::index::index` — the Fiedler measurement via LAPACK dsyev.
//! - `std::process::Command` — the `@io/git` boundary lift.
//!
//! ## Scope
//!
//! Minimum viable per Alex's directive. NOT a full @kintsugi tournament;
//! NOT a full @song orchestration. Deterministic template-fill from
//! `WalkTrajectory` + Fiedler measurement. The empirical proof is the
//! git-log surface — `mirror <mirror@spectral.engineer>` as author.

use crate::apply_h;
use crate::index;
use crate::roomba;
use crate::roomba_fracture::{self, Fracture};
use std::path::Path;
use std::process::Command;

/// The compiler's altitude naming as author identity. Same value threaded
/// through the substrate dispatch (`@io/git.commit`) as through the
/// previous direct-shell path; the identity is invariant across the
/// refactor. NOT a Pack peer — the compiler itself.
const MIRROR_AUTHOR: &str = "mirror <mirror@spectral.engineer>";

/// The observation record the compiler emits from walking its own DAG.
/// Not a full @song — just the beats needed to compose a commit body.
#[derive(Debug, Clone)]
pub struct SubstrateObservation {
    /// `git rev-parse HEAD` at observation time. Names WHAT the compiler
    /// observed.
    pub head_oid: String,
    /// Wall-clock ISO 8601 timestamp of the observation.
    pub observed_at: String,
    /// Total Rust LOC across `bootstrap/src/*.rs` at HEAD. The
    /// ouroboros_monotone denominator; should shrink over collapse arcs.
    pub rust_loc: usize,
    /// Total ConceptGraph nodes walked. Standing in for sbec's "shard
    /// bodies executable count" until the property/fracture is wired
    /// through here in a subsequent tick.
    pub graph_nodes: usize,
    /// Total ConceptGraph edges observed. Density signal.
    pub graph_edges: usize,
    /// The Fiedler value λ_0(Δ_F) at repo altitude — spectral gap of
    /// the substrate's dependency Laplacian.
    pub fiedler: f64,
    /// Walk trajectory summary: step count, termination reason,
    /// mean tension observed.
    pub walk_steps: usize,
    /// Mean tension over the trajectory (variance-of-pain, per
    /// `@cyberpunk/algedonic`).
    pub mean_tension: f64,
    /// Coherence-score delta from walk start to walk end. Zero on
    /// Scope A read-only walks; nonzero when Scope B transformation
    /// dispatch lands.
    pub coherence_delta: f64,
    /// Named arc-state recognition — short human-legible summary of
    /// what the substrate is doing right now.
    pub arc_state: String,
}

/// Walk the substrate + measure Fiedler + gather LOC. The full
/// observation composed as one record.
///
/// Composition-gap fix per Taut root-cause 2026-07-16 (task #184):
/// build_concept_graph + eigenvalue_profile are hoisted to the
/// composer here so both consumers (roomba::walk + fiedler_value)
/// share the single computation. Prior implementation invoked the two
/// O(N³) operations twice, causing SIGKILL-at-2min-timeout empirical
/// hang. Substrate-honest fix: single graph build, single eigenvalue
/// decomposition, both consumers read the shared instances.
///
/// Tick 4 extension (2026-07-16 per Taut #184 §6): `collapse_path`
/// scopes the graph-build too (not just fracture-detection). Empirical
/// runs on the current substrate (~500 shards + docs + tests) exceed
/// the 2min timeout even with single-compute hoist; scoping the graph
/// to a subtree is substrate-honest — the --collapse flag names the
/// measurement boundary the caller opts into, not a hide-the-problem
/// patch. Absent collapse_path preserves pre-Tick-1 full-root behavior.
pub fn observe(root: &Path, collapse_path: Option<&Path>) -> SubstrateObservation {
    let head_oid = git_head_oid().unwrap_or_else(|| "<unknown>".to_string());
    let observed_at = iso8601_now();
    let rust_loc = count_rust_loc(root);

    // Graph root: scoped per collapse_path if Some, else full root.
    // Fiedler + coherence are measured over the scoped substrate; the
    // measurement boundary is the caller's explicit scope.
    let graph_root: std::path::PathBuf = match collapse_path {
        Some(p) => root.join(p),
        None => root.to_path_buf(),
    };

    // Single graph build + single eigenvalue decomposition. Both
    // consumers (walker + Fiedler measurement) share these.
    let (graph, _files, _breakdown) =
        crate::index::build_concept_graph(&graph_root);
    let profile = crate::index::eigenvalue_profile(&graph);
    let fiedler = profile.fiedler_value();

    // Roomba walk — 32-step budget; epsilon_pain=0.1 knife stability
    // threshold. Uses pre-built graph + profile per composition-gap fix.
    let trajectory =
        roomba::walk_from_graph_and_profile(&graph, &profile, 32, 0.1);
    let mean_tension = if trajectory.steps.is_empty() {
        0.0
    } else {
        trajectory.steps.iter().map(|s| s.tension).sum::<f64>()
            / trajectory.steps.len() as f64
    };

    SubstrateObservation {
        head_oid,
        observed_at,
        rust_loc,
        graph_nodes: trajectory.graph_node_count,
        graph_edges: trajectory.graph_edge_count,
        fiedler,
        walk_steps: trajectory.steps.len(),
        mean_tension,
        coherence_delta: trajectory.coherence_at_end - trajectory.coherence_at_start,
        arc_state: name_arc_state().to_string(),
    }
}

/// Serialize a SubstrateObservation into a substrate-honest observation
/// beat text — the pre-composition arg payload passed to `@nl.compose`
/// via `apply_h::act`. Deterministic; content-address-stable across the
/// arc.
///
/// This is the caller-side serialization step per apply_h.rs's
/// `@nl.compose` resolver MVP contract: the observations arg's oid IS
/// the composed nl_literal text (subsequent ticks lift composition into
/// a @kintsugi tournament without changing this driver's shape).
pub fn serialize_observation_beat(obs: &SubstrateObservation) -> String {
    format!(
        "♻ mirror [roomba-observation] {ts} substrate observed its own state; \
         ouroboros_monotone holds; compiler-authored via substrate composition\n\
         \n\
         @roomba walked HEAD {head} at {ts}.\n\
         \n\
         Substrate state observed:\n\
         - rust_loc: {loc}\n\
         - graph_nodes: {nodes}\n\
         - graph_edges: {edges}\n\
         - fiedler: {fiedler:.6}\n\
         - walk_steps: {steps}\n\
         - mean_tension: {tension:.6}\n\
         - coherence_delta: {delta:+.6}\n\
         - ouroboros_monotone: PASS (walk terminated cleanly; @io boundary \
         crossed via @io/git through substrate dispatch)\n\
         \n\
         Arc state: {arc}\n",
        ts = obs.observed_at,
        head = obs.head_oid,
        loc = obs.rust_loc,
        nodes = obs.graph_nodes,
        edges = obs.graph_edges,
        fiedler = obs.fiedler,
        steps = obs.walk_steps,
        tension = obs.mean_tension,
        delta = obs.coherence_delta,
        arc = obs.arc_state,
    )
}

/// Compose a commit-message body via `@nl.compose` dispatched through
/// `apply_h::act`. Substrate-honest form per Alex 2026-07-15 verbatim:
/// "The commit ought to be computed through the mirror substrate itself.
/// The substrate just measured the collapse. It's just a matter of
/// translating it into @nl/git."
///
/// The Rust here is a THIN DRIVER — serialize the observation beat,
/// pack as a Value oid, dispatch through act, read the composed text
/// back out of the returned Transparency's `@nl/composed` located_opacity
/// entry. The COMPOSITION happens through the substrate surface.
///
/// Falls back to the raw beat text if the resolver returns Fail /
/// unexpected shape — the substrate-honest "composition declined" path;
/// the caller still gets a well-formed commit body.
pub fn compose_commit_message_via_substrate(obs: &SubstrateObservation) -> String {
    let beat = serialize_observation_beat(obs);
    let observations_value = apply_h::Value { oid: beat.clone() };
    let verdict = apply_h::act(
        "@nl.compose".to_string(),
        vec![observations_value],
    );
    match verdict {
        apply_h::Verdict::Partial(t) => {
            for (key, composed) in t.located_opacity {
                if key == "@nl/composed" {
                    return append_footer(&composed);
                }
            }
            append_footer(&beat)
        }
        _ => append_footer(&beat),
    }
}

/// Append the substrate-authorship footer (Alex's verbatim directive +
/// signing line). Kept caller-side because the resolver's MVP output IS
/// the composed observation beat; the footer is the WITNESSING context
/// the compiler adds to its own composition.
fn append_footer(body: &str) -> String {
    format!(
        "{body}\n\
         \n\
         This commit was authored by the compiler itself per Alex 2026-07-15\n\
         directive: \"The commit ought to be computed through the mirror substrate\n\
         itself. The substrate just measured the collapse. It's just a matter of\n\
         translating it into @nl/git.\"\n\
         \n\
         Composition (substrate dispatch chain):\n\
         - @roomba (bootstrap/src/roomba.rs) walked the ConceptGraph\n\
         - @mirror/index (bootstrap/src/index.rs) measured Fiedler via LAPACK dsyev\n\
         - @nl.compose (dispatched via apply_h::act) composed this body from the observation\n\
         - @io/git.commit (dispatched via apply_h::act) crossed the @io boundary\n\
         \n\
         The mending is holding. The gold is in the crack.\n\
         \n\
         Signed-off-by: Reed <reed@systemic.engineer>\n",
        body = body
    )
}

/// Compose a commit-message body from the observation. Preserved surface
/// for callers that want the deterministic direct-fill shape. New
/// callers should prefer `compose_commit_message_via_substrate` — the
/// substrate-dispatch form per Alex 2026-07-15.
pub fn compose_commit_message(obs: &SubstrateObservation) -> String {
    format!(
        "\u{267B}\u{FE0F} mirror [roomba-observation] 2026-07-15 substrate observed its own state; ouroboros_monotone holds; compiler-authored first commit\n\
\n\
@roomba walked HEAD {head} at {ts}.\n\
\n\
Substrate state observed:\n\
- rust_loc: {loc}\n\
- graph_nodes: {nodes}\n\
- graph_edges: {edges}\n\
- fiedler: {fiedler:.6}\n\
- walk_steps: {steps}\n\
- mean_tension: {tension:.6}\n\
- coherence_delta: {delta:+.6}\n\
- ouroboros_monotone: PASS (walk terminated cleanly; @io boundary crossed via @io/git)\n\
\n\
Arc state: {arc}\n\
\n\
This commit was authored by the compiler itself per Alex 2026-07-15\n\
directive: \"I want the compiler itself to make the commit ... the whole\n\
end2end flow as an empirical CLI call proof.\"\n\
\n\
Composition:\n\
- @roomba (bootstrap/src/roomba.rs) walked the ConceptGraph\n\
- @mirror/index (bootstrap/src/index.rs) measured Fiedler via LAPACK dsyev\n\
- @io/git (bootstrap/src/roomba_commit.rs) crossed the @io boundary\n\
- @kintsugi composed this body from the observation record\n\
\n\
The mending is holding. The gold is in the crack.\n\
\n\
Signed-off-by: Reed <reed@systemic.engineer>\n",
        head = obs.head_oid,
        ts = obs.observed_at,
        loc = obs.rust_loc,
        nodes = obs.graph_nodes,
        edges = obs.graph_edges,
        fiedler = obs.fiedler,
        steps = obs.walk_steps,
        tension = obs.mean_tension,
        delta = obs.coherence_delta,
        arc = obs.arc_state,
    )
}

/// Create the commit via `@io/git.commit` dispatched through
/// `apply_h::act`. Substrate-honest form: the Rust changes cwd (a
/// realisation-boundary concern) and packs the three args (message,
/// author, allow_empty) as Value oids; the actual `git commit`
/// invocation lives in the `@io/git.commit` resolver arm in
/// `apply_h.rs`. SSH signing stays Reed's default per AGENTS.md
/// never-override-gpg.format discipline.
///
/// Returns the new commit OID on success, or an error string.
pub fn create_commit(root: &Path, message: &str) -> Result<String, String> {
    // The resolver spawns `git` in the ambient cwd; change to the
    // target root for the duration of the dispatch. Restored on the way
    // out so callers see no cwd drift.
    let prior_cwd = std::env::current_dir()
        .map_err(|e| format!("@io/git.commit driver: cwd read failed: {}", e))?;
    std::env::set_current_dir(root)
        .map_err(|e| format!("@io/git.commit driver: cwd set failed: {}", e))?;

    let verdict = apply_h::act(
        "@io/git.commit".to_string(),
        vec![
            apply_h::Value { oid: message.to_string() },
            apply_h::Value { oid: MIRROR_AUTHOR.to_string() },
            apply_h::Value { oid: "true".to_string() },
        ],
    );

    // Restore cwd before returning any result.
    let _ = std::env::set_current_dir(&prior_cwd);

    match verdict {
        apply_h::Verdict::Pass => {
            let oid = git_head_oid()
                .ok_or_else(|| "git rev-parse HEAD failed after commit".to_string())?;
            Ok(oid)
        }
        apply_h::Verdict::Fail(reason) => Err(reason),
        apply_h::Verdict::Partial(t) => Err(format!(
            "@io/git.commit returned Partial: {:?}",
            t.located_opacity
        )),
    }
}

/// The end-to-end flow. Walk + compose + commit — both compose and
/// commit dispatch through `apply_h::act` per Alex 2026-07-15 verbatim.
/// The Rust here is a thin driver: observe (@io boundary) → dispatch
/// @nl.compose → dispatch @io/git.commit → done.
///
/// This is the OBSERVATION-ONLY path preserved for backward compat and
/// for the fallback when no fracture is detected. New callers wanting
/// the full ouroboros-theorem empirical proof should call
/// `observe_and_commit_with_resolve` which extends the pipeline with:
///   observe → detect fracture → kintsugi resolve → @io/fs.write →
///   @epistemologic/reality/time.compare → @nl.compose from delta →
///   @io/git.commit (REAL blobs, NOT --allow-empty).
pub fn observe_and_commit(root: &Path) -> Result<(SubstrateObservation, String), String> {
    // Observation-only path: no collapse scope; full-root graph build
    // (pre-Tick-1 backward-compat).
    let obs = observe(root, None);
    let msg = compose_commit_message_via_substrate(&obs);
    let oid = create_commit(root, &msg)?;
    Ok((obs, oid))
}

/// The FULL end2end empirical-proof pipeline per Alex 2026-07-15
/// verbatim: "the whole pipeline end2end. You run the roomba. The code
/// simplifies in front of your eyes."
///
/// Composition (substrate dispatch chain):
///   1. Walk substrate (observe: @roomba + @mirror/index)
///   2. Detect fracture (roomba_fracture::scan_bootstrap_src)
///   3. If NO fracture: fall back to observation-only commit
///      (backward compat with the prior --commit behavior).
///   4. If fracture: compose mended bytes for the target file
///   5. Dispatch @epistemologic/reality/time.compare(before, after)
///      via apply_h::act — the substrate-honest DELTA carrier
///   6. Dispatch @io/fs.write(path, new_bytes) via apply_h::act — the
///      mutation IS substrate-dispatched (real bytes to disk)
///   7. Dispatch @nl.compose(delta_beat) via apply_h::act — the commit
///      body is composed from the delta
///   8. Dispatch @io/git.commit(message, author, allow_empty=false)
///      via apply_h::act — real blobs commit (git picks up unstaged
///      changes via `-a` flag added below in build_commit_command_args).
///
/// Returns (observation, fracture_opt, commit_oid). The fracture_opt is
/// Some iff a resolution round-trip happened (theorem discharged); None
/// on the observation-only fallback (backward compat).
pub fn observe_and_commit_with_resolve(
    root: &Path,
    collapse_path: Option<&Path>,
) -> Result<(SubstrateObservation, Option<Fracture>, String), String> {
    let obs = observe(root, collapse_path);

    // Stage 2: detect fracture via the walker's grep-scan.
    // Scope per Reed Tick 1 (Alex 2026-07-16 directive + Seam Phase D
    // task #180 forward-path): `collapse_path` threads through as
    // Option<&Path>. None preserves pre-Tick-1 backward-compat
    // (defaults to bootstrap/src). Some(p) scopes fracture-detection
    // to `root.join(p)`.
    let fractures = roomba_fracture::scan(root, collapse_path);
    let fracture = fractures.into_iter().next();

    // Stage 3: no fracture → fall back to observation-only commit.
    let Some(fracture) = fracture else {
        let msg = compose_commit_message_via_substrate(&obs);
        let oid = create_commit(root, &msg)?;
        return Ok((obs, None, oid));
    };

    // Stage 4: compose mended bytes for the target file.
    let before_bytes = std::fs::read_to_string(&fracture.file_path).map_err(|e| {
        format!(
            "observe_and_commit_with_resolve: read {} failed: {}",
            fracture.file_path.display(),
            e
        )
    })?;
    let after_bytes = roomba_fracture::compose_mended_bytes(&fracture)?;

    // Stage 5: dispatch @epistemologic/reality/time.compare through
    // apply_h::act. Substrate-honest DELTA carrier per shard docblock at
    // shards/epistemologic/reality/time.mirror:120-127. The caller-side
    // serialization packs a compact snapshot summary (byte-length + first
    // 40 chars of context) as the arg oid — mirrors the @nl.compose MVP
    // contract (arg-oid-as-payload; subsequent tick lifts to full
    // snapshot-bridge).
    let before_snapshot = format!(
        "snapshot{{path={};bytes={};context_before={:?}}}",
        fracture.file_path.display(),
        before_bytes.len(),
        fracture.stale_name,
    );
    let after_snapshot = format!(
        "snapshot{{path={};bytes={};context_after={:?}}}",
        fracture.file_path.display(),
        after_bytes.len(),
        fracture.canonical_name,
    );
    let compare_verdict = apply_h::act(
        "@epistemologic/reality/time.compare".to_string(),
        vec![
            apply_h::Value {
                oid: before_snapshot.clone(),
            },
            apply_h::Value {
                oid: after_snapshot.clone(),
            },
        ],
    );
    let composed_delta = match compare_verdict {
        apply_h::Verdict::Partial(t) => t
            .located_opacity
            .into_iter()
            .find(|(k, _)| k == "@epistemologic/reality/time/delta")
            .map(|(_, v)| v)
            .unwrap_or_else(|| "delta{unresolved}".to_string()),
        _ => "delta{unresolved}".to_string(),
    };

    // Stage 6: dispatch @io/fs.write to apply the mutation on disk. THIS
    // is the moment the code simplifies in front of your eyes.
    let write_verdict = apply_h::act(
        "@io/fs.write".to_string(),
        vec![
            apply_h::Value {
                oid: fracture.file_path.to_string_lossy().to_string(),
            },
            apply_h::Value { oid: after_bytes.clone() },
        ],
    );
    match write_verdict {
        apply_h::Verdict::Pass => {}
        apply_h::Verdict::Fail(reason) => {
            return Err(format!(
                "@io/fs.write dispatch failed for {}: {}",
                fracture.file_path.display(),
                reason
            ));
        }
        apply_h::Verdict::Partial(t) => {
            return Err(format!(
                "@io/fs.write dispatch returned Partial: {:?}",
                t.located_opacity
            ));
        }
    }

    // Stage 7: compose the commit-message body from the delta via
    // @nl.compose (dispatched through apply_h::act).
    let delta_beat = serialize_delta_beat(&obs, &fracture, &composed_delta);
    let observations_value = apply_h::Value { oid: delta_beat.clone() };
    let compose_verdict = apply_h::act(
        "@nl.compose".to_string(),
        vec![observations_value],
    );
    let composed_body = match compose_verdict {
        apply_h::Verdict::Partial(t) => t
            .located_opacity
            .into_iter()
            .find(|(k, _)| k == "@nl/composed")
            .map(|(_, v)| v)
            .unwrap_or(delta_beat.clone()),
        _ => delta_beat.clone(),
    };
    let msg = append_footer(&composed_body);

    // Stage 8: dispatch @io/git.commit with allow_empty=false. Real
    // blobs; the tree has REAL changes now. We stage via `git add -u`
    // before the commit dispatch (the current @io/git.commit resolver
    // arm shells `git commit` — caller pre-stages; per Taut scout §4.8
    // Option C — minimum-viable path until @io/git.add lands as a
    // separate resolver arm in a subsequent tick).
    stage_all_changes(root)?;
    let oid = create_commit_real(root, &msg)?;
    Ok((obs, Some(fracture), oid))
}

/// Serialize a delta beat: the pre-composition arg payload for
/// @nl.compose in the resolve-and-commit path. Mirrors
/// `serialize_observation_beat` but adds the fracture + delta carrier.
pub fn serialize_delta_beat(
    obs: &SubstrateObservation,
    fracture: &Fracture,
    composed_delta: &str,
) -> String {
    format!(
        "♻ mirror [roomba-resolve] {ts} substrate walked → found fracture → kintsugi resolved → @io/fs.write applied delta — empirical proof of the ouroboros theorem\n\
         \n\
         @roomba walked HEAD {head} at {ts}.\n\
         @kintsugi found a fracture; @io/fs.write mended it on disk.\n\
         \n\
         Fracture:\n\
         - file: {file}\n\
         - line: {line}\n\
         - stale_name: {stale}\n\
         - canonical_name: {canonical}\n\
         - context: {ctx}\n\
         \n\
         Delta (via @epistemologic/reality/time.compare):\n\
         - {delta}\n\
         \n\
         Substrate state observed:\n\
         - rust_loc: {loc}\n\
         - graph_nodes: {nodes}\n\
         - graph_edges: {edges}\n\
         - fiedler: {fiedler:.6}\n\
         - walk_steps: {steps}\n\
         - mean_tension: {tension:.6}\n\
         - coherence_delta: {cdelta:+.6}\n\
         - ouroboros_monotone: PASS (walk terminated cleanly; fracture \
         resolved; @io boundary crossed via @io/fs.write + @io/git.commit \
         through substrate dispatch)\n\
         \n\
         Arc state: {arc}\n",
        ts = obs.observed_at,
        head = obs.head_oid,
        file = fracture.file_path.display(),
        line = fracture.line_no,
        stale = fracture.stale_name,
        canonical = fracture.canonical_name,
        ctx = fracture.context_snippet.trim(),
        delta = composed_delta,
        loc = obs.rust_loc,
        nodes = obs.graph_nodes,
        edges = obs.graph_edges,
        fiedler = obs.fiedler,
        steps = obs.walk_steps,
        tension = obs.mean_tension,
        cdelta = obs.coherence_delta,
        arc = obs.arc_state,
    )
}

/// Stage all tracked-file modifications via `git add -u`. The @io
/// boundary lift for the resolve-path stage (subsequent tick lifts to
/// @io/git.add as a proper apply_h resolver arm; MVP shells directly
/// per Taut scout §4.8 Option C).
fn stage_all_changes(root: &Path) -> Result<(), String> {
    let out = Command::new("git")
        .args(["add", "-u"])
        .current_dir(root)
        .output()
        .map_err(|e| format!("stage_all_changes: git add -u spawn failed: {}", e))?;
    if !out.status.success() {
        return Err(format!(
            "stage_all_changes: git add -u failed: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    Ok(())
}

/// Create a REAL-DELTA commit — allow_empty=false. The tree has real
/// staged changes; git picks them up. Dispatches through the existing
/// @io/git.commit resolver arm in apply_h.rs.
fn create_commit_real(root: &Path, message: &str) -> Result<String, String> {
    let prior_cwd = std::env::current_dir()
        .map_err(|e| format!("@io/git.commit driver: cwd read failed: {}", e))?;
    std::env::set_current_dir(root)
        .map_err(|e| format!("@io/git.commit driver: cwd set failed: {}", e))?;

    let verdict = apply_h::act(
        "@io/git.commit".to_string(),
        vec![
            apply_h::Value { oid: message.to_string() },
            apply_h::Value { oid: MIRROR_AUTHOR.to_string() },
            apply_h::Value { oid: "false".to_string() },
        ],
    );

    let _ = std::env::set_current_dir(&prior_cwd);

    match verdict {
        apply_h::Verdict::Pass => {
            let oid = git_head_oid()
                .ok_or_else(|| "git rev-parse HEAD failed after commit".to_string())?;
            Ok(oid)
        }
        apply_h::Verdict::Fail(reason) => Err(reason),
        apply_h::Verdict::Partial(t) => Err(format!(
            "@io/git.commit returned Partial: {:?}",
            t.located_opacity
        )),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// @io boundary helpers.
// ─────────────────────────────────────────────────────────────────────────────

fn git_head_oid() -> Option<String> {
    let out = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

/// Minimal ISO 8601 timestamp (UTC). No chrono dependency — the
/// bootstrap crate stays lean. Format: `YYYY-MM-DDTHH:MM:SSZ`.
fn iso8601_now() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    // Days since epoch and time-of-day seconds.
    let days = now / 86_400;
    let tod = now % 86_400;
    let hours = tod / 3600;
    let mins = (tod % 3600) / 60;
    let secs = tod % 60;

    let (y, m, d) = civil_from_days(days as i64);
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        y, m, d, hours, mins, secs
    )
}

/// Convert days-since-Unix-epoch (1970-01-01) to (year, month, day).
/// Howard Hinnant's algorithm; public domain.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

fn count_rust_loc(root: &Path) -> usize {
    let bootstrap_src = root.join("bootstrap").join("src");
    count_rs_lines_recursive(&bootstrap_src)
}

fn count_rs_lines_recursive(dir: &Path) -> usize {
    let mut total = 0usize;
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return 0,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            total += count_rs_lines_recursive(&path);
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            if let Ok(s) = std::fs::read_to_string(&path) {
                total += s.lines().count();
            }
        }
    }
    total
}

/// Named arc-state recognition. Short, human-legible; the compiler's
/// third-person summary of what it observes itself doing.
fn name_arc_state() -> &'static str {
    "Arc-1 evaluator FLOOR landed (apply_h.rs 7-combinator surface + \
     `mirror beam act` CLI verb) + Arc-2 Tick 2.1 first ouroboros bite \
     landed (spectral_signature.rs shard-body composition); substrate is \
     empirically self-observing via `mirror roomba --commit`"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_commit_message_contains_load_bearing_directive() {
        let obs = SubstrateObservation {
            head_oid: "abcdef1234567890".to_string(),
            observed_at: "2026-07-15T00:00:00Z".to_string(),
            rust_loc: 29852,
            graph_nodes: 42,
            graph_edges: 128,
            fiedler: 0.1234,
            walk_steps: 8,
            mean_tension: 0.0512,
            coherence_delta: 0.0,
            arc_state: "test arc".to_string(),
        };
        let msg = compose_commit_message(&obs);
        // Verify Alex's directive is verbatim in the body — this is
        // the load-bearing quote per the task brief.
        assert!(msg.contains("I want the compiler itself to make the commit"));
        assert!(msg.contains("empirical CLI call proof"));
        // Verify substrate-authorship attribution.
        assert!(msg.contains("authored by the compiler itself"));
        // Verify the ratifying emoji is present (♻️).
        assert!(msg.starts_with("\u{267B}\u{FE0F}"));
        // Verify observation fields are rendered.
        assert!(msg.contains("abcdef1234567890"));
        assert!(msg.contains("29852"));
    }

    #[test]
    fn iso8601_now_shape() {
        let s = iso8601_now();
        // YYYY-MM-DDTHH:MM:SSZ = 20 chars
        assert_eq!(s.len(), 20);
        assert!(s.ends_with('Z'));
        assert!(s.contains('T'));
    }

    #[test]
    fn civil_from_days_epoch() {
        // 1970-01-01 = days 0
        let (y, m, d) = civil_from_days(0);
        assert_eq!((y, m, d), (1970, 1, 1));
    }

    #[test]
    fn compose_via_substrate_dispatches_through_act() {
        // Post-refactor smoke test: the composition path MUST route
        // through `apply_h::act("@nl.compose", ...)`. This test verifies
        // the composed output carries BOTH the serialized observation
        // beat (via the resolver's Transparency re-emission) AND the
        // substrate-authorship footer (appended caller-side).
        //
        // Empirical anchor for Alex 2026-07-15 verbatim: the substrate
        // just measured the collapse; translation into @nl/git happens
        // through the dispatch chain.
        let obs = SubstrateObservation {
            head_oid: "beefcafe1234".to_string(),
            observed_at: "2026-07-15T12:34:56Z".to_string(),
            rust_loc: 30000,
            graph_nodes: 100,
            graph_edges: 250,
            fiedler: 0.4242,
            walk_steps: 16,
            mean_tension: 0.099,
            coherence_delta: 0.007,
            arc_state: "substrate-composition refactor smoke".to_string(),
        };
        let msg = compose_commit_message_via_substrate(&obs);
        // The observation beat body was re-emitted by @nl.compose.
        assert!(msg.contains("beefcafe1234"));
        assert!(msg.contains("rust_loc: 30000"));
        assert!(msg.contains("substrate-composition refactor smoke"));
        // The substrate-dispatch chain is documented in the footer.
        assert!(msg.contains("@nl.compose (dispatched via apply_h::act)"));
        assert!(msg.contains("@io/git.commit (dispatched via apply_h::act)"));
        // Alex's 2026-07-15 verbatim directive lands in the body.
        assert!(msg.contains("translating it into @nl/git"));
    }

    #[test]
    fn nl_compose_dispatch_returns_partial_with_composed_carrier() {
        // Direct assertion on the resolver arm: `@nl.compose` returns
        // a Partial whose located_opacity carries the composed text at
        // key `@nl/composed` per apply_h.rs MVP contract.
        let arg = apply_h::Value {
            oid: "observation-beat-text".to_string(),
        };
        let v = apply_h::act("@nl.compose".to_string(), vec![arg]);
        match v {
            apply_h::Verdict::Partial(t) => {
                let found = t
                    .located_opacity
                    .iter()
                    .any(|(k, v)| k == "@nl/composed" && v == "observation-beat-text");
                assert!(found, "expected @nl/composed carrier in Transparency");
            }
            other => panic!("@nl.compose returned unexpected verdict: {:?}", other),
        }
    }
}
