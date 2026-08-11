//! `apply_h::act` at rust/ altitude — the bilateral-dispatch primitive.
//!
//! Reed 2026-08-06 R-PRIM-3 per Taut scout `7af55ee` §7 smallest-
//! primitive-gap identification + Alex 2026-08-06 Q-1 adjudication
//! (expose as `apply_h::act` for naming honesty per bootstrap surface;
//! other 6 combinators land as extensions).
//!
//! ## Naming
//!
//! Mirrors bootstrap's `apply_h.rs` module structure. Bootstrap has the
//! full 7-combinator surface (~405 LOC per task #140); this rust/
//! sibling starts with the MINIMUM subset needed for MCP composition
//! at Phase 1 — the bilateral-sentinel-check `act` primitive. Additional
//! combinators (compose / fold / bind / etc.) land as extensions when
//! composition-shard bodies demand them, per Alex 2026-08-05 substrate-
//! honest reframe (rust/ delivers primitives; substrate delivers
//! composition).
//!
//! ## Composition
//!
//! `@mcp/serve.mirror` composition-shard body (Mara Fire B M-COMP-1)
//! composes `act` via pipeline:
//!
//! ```text
//! phone::read_stdin_frame  |> wire::parse
//!                          |> apply_h::act(root, action_ref, args)
//!                          |> wire::emit
//!                          |> phone::write_stdout_frame
//! ```
//!
//! Each pipe element is a landed rust/ primitive at terminal-geometry
//! altitude; the whole composition sits at substrate altitude in
//! `shards/mcp/serve.mirror`.
//!
//! ## Phase 1 vs Phase 2
//!
//! **Phase 1 (this landing)**: bilateral-sentinel-check act. Given an
//! action_ref like `@subject/visibility/public.consent_scope_universal`,
//! load the bilateral corpus from `<root>/shards`, look up the
//! substrate-decl'd bilateral by action_ref, verify sentinel byte-string
//! containment in the args. Returns [`Verdict::Pass`] / [`Verdict::Fail`].
//!
//! **Phase 2 (M5+ post-Mara-canonical-spec)**: grammar-driven dispatch
//! per `@mcp/tool` annotation walking via `@mirror/spectral.gestalt`.
//! Additional combinators (compose / fold / bind / focus / project /
//! split / shift / settle) land as demand surfaces.
//!
//! ## Composition anchors
//!
//! - `docs/scouts/2026-08-05-taut-primitives-vs-composition-scout.md`
//!   §7 smallest-primitive-gap (Fire A tick 3)
//! - `feedback-rust-delivers-primitives-substrate-delivers-composition`
//!   memory (Alex 2026-08-05 verbatim correction)
//! - `bootstrap/src/apply_h.rs` (source of the 7-combinator surface;
//!   task #140 GREEN ~405 LOC; RETIRING under Fire C when composition-
//!   path fires via rust/ altitude)
//! - Recognition `#R-reality-as-5d-spinning-foam` RATIFIED 2026-08-03
//!   (Layer 0 sub-Turing decidable floor = rust/ interpreter; apply_h.rs
//!   IS a Layer 0 primitive)
//!
//! ## Register
//!
//! Substrate-honest, decidable, sub-Turing. Given a bilateral corpus
//! (finite, byte-loaded from `<root>/shards/**/*.mirror`) + a bounded
//! args vector, this function terminates in O(corpus_size * args_len)
//! at worst — no unbounded recursion; no networking; no fs writes.
//! Pure predicate evaluation over the substrate-decl'd sentinel.

use std::path::{Path, PathBuf};

use roomba::mend::{load_bilateral_corpus, BilateralDecl};

/// The verdict a bilateral-dispatch primitive returns. Mirrors
/// bootstrap's Verdict shape at bilateral-predicate altitude.
///
/// Phase 1 subset: [`Pass`](Verdict::Pass) + [`Fail`](Verdict::Fail).
/// Phase 2+ extension: [`Partial`](Verdict::Partial) with per-clause
/// transparency (opacity map) lands when substrate-decl'd composed
/// bilaterals (multi-clause) enter the composition surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// Sentinel matched; substrate-decl'd predicate discharges.
    Pass,
    /// Sentinel not matched, or action_ref not in corpus.
    /// Contains substrate-honest reason string.
    Fail(String),
}

/// The bilateral-dispatch primitive. Load bilateral corpus rooted at
/// `root`, look up the substrate-decl'd bilateral by `action_ref`,
/// verify sentinel byte-string containment against `args`.
///
/// Reed 2026-08-06 R-PRIM-3 per Alex 2026-08-06 Q-1 adjudication
/// (expose as `apply_h::act` naming honesty).
///
/// # Arguments
///
/// * `root` — The substrate repo root (contains `shards/` subdirectory).
/// * `action_ref` — Full action reference like
///   `@subject/visibility/public.consent_scope_universal`.
///   Concatenation of shard-ref + `.` + bilateral-name per
///   `BilateralDecl::full_action_ref` semantics.
/// * `args` — Positional args. Sentinel-containment check runs against
///   the concatenation of `args` joined by ASCII space (matching
///   bootstrap's argv byte-substring semantics).
///
/// # Returns
///
/// [`Verdict::Pass`] if sentinel matches; [`Verdict::Fail`] with
/// substrate-honest reason otherwise.
///
/// # Substrate-honest semantics
///
/// This is the MINIMUM `act` primitive at rust/ altitude. It handles
/// the base-case bilateral-sentinel-check dispatch. Composition-shards
/// (Mara Fire B) compose over it for MCP tool dispatch that maps to
/// bilateral predicates. Non-bilateral dispatch (e.g. "execute a rust/
/// cmd_ verb") is composition-shard's responsibility, not this
/// primitive's.
pub fn act(root: &Path, action_ref: &str, args: &[String]) -> Verdict {
    // Fire E M-E2 extension: shard-body-projector dispatch (Reed
    // 2026-08-09 per Alex 2026-08-09 kintsugi-sugar first self-applied
    // loop + Mara canonical spec 733cbf1 §1.1 P1 identity-carrier prism
    // sentinel + Seam Phase D 0a4c5a0 SEAM-RATIFY-WITH-SHARPENING).
    //
    // When action_ref matches `@kintsugi/fracture/<species>.detect`,
    // dispatch to the shard-body-projector primitive for that species
    // BEFORE the bilateral-sentinel-check corpus lookup. The projector
    // reads the shard file at args[0], detects the sugar-fracturable
    // pattern per species-specific Rice-safe byte-check, returns
    // Verdict::Pass on detection.
    //
    // SHARED PRIMITIVE with Fire D M5 wire-executor (per CURRENT.md
    // 2026-08-05→2026-08-09 addendum): Fire E validates at low-risk
    // pure-canonicalization altitude FIRST; Fire D wire-executor shares
    // the validated primitive when it lands.
    //
    // Substrate-honest per Alex 2026-08-05 memory feedback-rust-
    // delivers-primitives-substrate-delivers-composition: rust/ delivers
    // the byte-check detector primitives; substrate delivers the
    // composition (@kintsugi/mend/sugar shard-body wires these into the
    // walker cascade via apply_h::act dispatch).
    if let Some(species) = strip_fracture_detect_ref(action_ref) {
        return dispatch_fracture_detect(root, species, args);
    }

    // Fire E M-E3.5-REVISED extension: beta-reduction-rule reducible
    // dispatch (Reed 2026-08-11 per Alex 2026-08-11 concur-with-Mara-
    // lean on 6 [ALEX-Q1-Q6] + trust-transfer + Mara canonical spec
    // 5ad8528 + M-E3-REVISED shards landed at d983854).
    //
    // When action_ref matches `@epistemologic/normalization/rules/
    // <rule>.reducible`, dispatch to the rule's `reducible` bilateral
    // predicate. Under Mara §6.2 discipline, the fracture-detector and
    // the beta-reduction rule are the same operator at two altitudes;
    // .reducible at AST altitude routes to the M-E2 landed source-
    // altitude detector.
    if let Some(species) = strip_normalization_rule_reducible_ref(action_ref) {
        return dispatch_reduction_rule_reducible(root, species, args);
    }

    let corpus = load_bilateral_corpus(root);
    let decl: &BilateralDecl = match corpus.get(action_ref) {
        Some(d) => d,
        None => {
            return Verdict::Fail(format!(
                "apply_h::act: action_ref `{}` not found in bilateral corpus at `{}`",
                action_ref,
                root.display()
            ))
        }
    };

    // Sentinel-containment check per bootstrap's byte-substring
    // semantics. Concat args by ASCII space and substring-match the
    // sentinel. Trivially decidable in O(args_concat_len * sentinel_len).
    let args_joined = args.join(" ");
    if args_joined.contains(&decl.sentinel) {
        Verdict::Pass
    } else {
        Verdict::Fail(format!(
            "apply_h::act: sentinel `{}` not found in args `{}` for `{}`",
            decl.sentinel, args_joined, action_ref
        ))
    }
}

// =====================================================================
// Fire E M-E2 — shard-body-projector primitives
// =====================================================================
//
// Rice-safe byte-check detectors for the four ratified sugar-fracturable
// patterns per Mara canonical spec 733cbf1 §1 + Alex 2026-08-09 concur-
// with-Mara-lean on 6 [ALEX-Q] + concur-with-Seam-lean on SEAM-1+SEAM-2.
// M-E2 lands the P1 identity-carrier prism detector (Alex's ancestor;
// 235 shards; ~1,175–1,645 LOC removal potential per Taut scout
// 2b01da0 §1.1). P3/P4/P5 detectors land at subsequent Fire E ticks as
// demand pulls them (per Fire A discipline: minimum primitive first;
// extensions land as composition-shard bodies pull).
//
// Fixed-point exemption per Q3 Mara-lean: `shards/prism.mirror` and
// `shards/glass.mirror` self-referential fixed-points are EXEMPT from
// sugar-detection (they ARE the substrate's declaration of what the
// sugar rule refers to). Detector returns Verdict::Fail for those paths.

/// Strip `@epistemologic/normalization/rules/<species>.reducible`
/// action_ref → species name. Returns None if action_ref doesn't match
/// the reduction-rule reducible pattern.
fn strip_normalization_rule_reducible_ref(action_ref: &str) -> Option<&str> {
    let prefix = "@epistemologic/normalization/rules/";
    let suffix = ".reducible";
    if action_ref.starts_with(prefix) && action_ref.ends_with(suffix) {
        Some(&action_ref[prefix.len()..action_ref.len() - suffix.len()])
    } else {
        None
    }
}

/// Dispatch reduction-rule `.reducible` action_ref to the rule-species
/// predicate. args[0] = shard_path relative to `root`. Under Mara §6.2
/// discipline, reducible at AST altitude routes to the M-E2 landed
/// source-altitude detector for the corresponding pattern.
fn dispatch_reduction_rule_reducible(root: &Path, species: &str, args: &[String]) -> Verdict {
    let rel_path = match args.first() {
        Some(p) => p,
        None => {
            return Verdict::Fail(format!(
                "apply_h::act[@epistemologic/normalization/rules/{}.reducible]: \
                 missing shard_path in args",
                species
            ))
        }
    };
    let shard_path: PathBuf = root.join(rel_path);

    match species {
        "identity_projection_elision" => detect_prism_boilerplate_at(&shard_path),
        // Fire E subsequent-tick reducibles land as demand pulls per
        // Fire A discipline (minimum first; extensions when consumers surface):
        //   "glass_identity_elision"      P3 depth-≥1 identity-carrier glass
        //   "out_derivable_elision"       P4 root `out @X` line derivable
        //   "docblock_stub_elision"       P5 docblock template derivable
        other => Verdict::Fail(format!(
            "apply_h::act[@epistemologic/normalization/rules/{}.reducible]: no reducible \
             primitive at rust/ altitude (Fire E M-E3.5-REVISED lands identity_projection_\
             elision first; P3/P4/P5 rule reducibles are subsequent-tick forward-promises \
             per Mara 5ad8528 §7.2)",
            other
        )),
    }
}

/// Strip `@kintsugi/fracture/<species>.detect` action_ref → species
/// name. Returns None if action_ref doesn't match the fracture-detect
/// pattern.
fn strip_fracture_detect_ref(action_ref: &str) -> Option<&str> {
    let prefix = "@kintsugi/fracture/";
    let suffix = ".detect";
    if action_ref.starts_with(prefix) && action_ref.ends_with(suffix) {
        Some(&action_ref[prefix.len()..action_ref.len() - suffix.len()])
    } else {
        None
    }
}

/// Dispatch fracture-detect action_ref to the species-specific detector.
/// args[0] = shard_path relative to `root` (composition-shard body
/// convention per Mara §4.1 pipeline).
fn dispatch_fracture_detect(root: &Path, species: &str, args: &[String]) -> Verdict {
    let rel_path = match args.first() {
        Some(p) => p,
        None => {
            return Verdict::Fail(format!(
                "apply_h::act[@kintsugi/fracture/{}.detect]: missing shard_path in args",
                species
            ))
        }
    };
    let shard_path: PathBuf = root.join(rel_path);

    match species {
        "prism_boilerplate" => detect_prism_boilerplate_at(&shard_path),
        // Fire E subsequent-tick detectors land as demand pulls:
        //   "glass_boilerplate"    P3 depth-≥1 identity-carrier glass
        //   "out_derivable"        P4 root `out @X` line derivable
        //   "path_namespace_stub"  P5 docblock template derivable
        other => Verdict::Fail(format!(
            "apply_h::act[@kintsugi/fracture/{}.detect]: no detector primitive at rust/ altitude \
             (Fire E M-E2 lands prism_boilerplate first; P3/P4/P5 detectors are subsequent-tick \
             forward-promises per Mara spec §8.2)",
            other
        )),
    }
}

/// P1 identity-carrier prism detector (Alex's ancestor pattern per Mara
/// spec §1.1). Reads the shard source at `shard_path` and returns:
///
/// - `Verdict::Pass` if the source contains an identity-carrier prism
///   block at the depth-0 family altitude (all five 5-op arms bind the
///   same carrier == last-segment of the path-derived family literal).
///
/// - `Verdict::Fail(reason)` on fixed-point exemption (`shards/prism.
///   mirror`), path-derivation failure, read-file failure, or absent
///   pattern.
///
/// # Substrate discipline
///
/// Rice-safe byte-check per Mara §1.1 sentinel. Reads only byte-visible
/// state: (1) shard-path decomposition; (2) prism-decl line matching;
/// (3) five-arm carrier-literal byte-equality check. No program
/// semantics inspection. Terminates in O(source_len) at worst.
///
/// Tolerates whitespace variation between op-keyword and carrier-literal
/// on arm lines (single-space `focus fs` and double-space `focus  fs`
/// both match). This is per-landed-shard byte-shape reality: some P1
/// shards use single-space, others (@io/fs, @subject/visibility/public,
/// etc.) use double-space. SEAM-2 preserve-indent-as-parameter still
/// holds because detection tolerates variation; write-back at Fire E
/// M-E4 walker cascade preserves original indent bytes.
///
/// # Fixed-point exemption (Q3 Mara-lean; Alex 2026-08-09 concur)
///
/// `shards/prism.mirror` self-referential fixed-point IS the substrate's
/// declaration of what the P1 sugar rule refers to. Detector returns
/// `Fail("fixed-point exemption")` for that path. Sibling exemption for
/// `shards/glass.mirror` lives at the glass_boilerplate detector
/// (subsequent tick).
pub fn detect_prism_boilerplate_at(shard_path: &Path) -> Verdict {
    // Q3 Mara-lean: fixed-point exemption for shards/prism.mirror.
    if let Some(name) = shard_path.file_name() {
        if name == "prism.mirror" {
            return Verdict::Fail(format!(
                "detect_prism_boilerplate: Q3 fixed-point exemption \
                 shards/prism.mirror (self-referential; substrate's own \
                 declaration of what the sugar rule refers to); path=`{}`",
                shard_path.display()
            ));
        }
    }

    let family = match derive_family_from_shard_path(shard_path) {
        Some(f) => f,
        None => {
            return Verdict::Fail(format!(
                "detect_prism_boilerplate: cannot derive family literal from path `{}` \
                 (expected shards/<family>.mirror shape)",
                shard_path.display()
            ))
        }
    };

    // Last segment of family for arm-carrier literals per landed
    // convention (grep-verified: shards/spectral.mirror → `spectral`;
    // shards/uuid.mirror → `uuid`; shards/io/fs.mirror → `fs`).
    let last_seg: &str = family.rsplit('/').next().unwrap_or(&family);

    let source = match std::fs::read_to_string(shard_path) {
        Ok(s) => s,
        Err(e) => {
            return Verdict::Fail(format!(
                "detect_prism_boilerplate: read_file(`{}`) failed: {}",
                shard_path.display(),
                e
            ))
        }
    };

    if source_carries_p1_identity_prism(&source, &family, last_seg) {
        Verdict::Pass
    } else {
        Verdict::Fail(format!(
            "detect_prism_boilerplate: no P1 identity-carrier prism block \
             detected at `{}` (family=`{}`, last_seg=`{}`)",
            shard_path.display(),
            family,
            last_seg
        ))
    }
}

/// Derive family literal from `shards/<family>.mirror` path. Returns
/// None if path doesn't match the shards/*.mirror shape.
///
/// Examples:
///   shards/spectral.mirror                    → Some("spectral")
///   shards/uuid.mirror                        → Some("uuid")
///   shards/io/fs.mirror                       → Some("io/fs")
///   shards/kintsugi/fracture/keyword.mirror   → Some("kintsugi/fracture/keyword")
///   /tmp/foo.txt                              → None
fn derive_family_from_shard_path(shard_path: &Path) -> Option<String> {
    let s = shard_path.to_str()?;
    // Find the last occurrence of `shards/` (handles both relative
    // `shards/spectral.mirror` and absolute `/tmp/repo/shards/foo.mirror`
    // per tempdir test-fixture pattern).
    let idx = s.rfind("shards/")?;
    let after_prefix = &s[idx + "shards/".len()..];
    let family = after_prefix.strip_suffix(".mirror")?;
    Some(family.to_string())
}

/// Check whether `source` carries a P1 identity-carrier prism block at
/// the depth-0 family altitude.
///
/// Rice-safe byte-check: scans source line-by-line looking for
///   1. Line matching `prism @<family> {` (with optional trailing whitespace)
///   2. Followed by 5 lines each of shape `<op> <last_seg>` where
///      <op> ∈ {focus, project, split, shift, settle} in exact order and
///      <last_seg> is byte-identical across all 5 arms.
///   3. Followed by a line matching `}` (optionally indented).
///
/// Tolerates whitespace variation between op-keyword and carrier-literal
/// (single-space + double-space both match). Ignores leading whitespace
/// on each line (some shards use `  focus` two-space indent).
fn source_carries_p1_identity_prism(source: &str, family: &str, last_seg: &str) -> bool {
    let decl_line = format!("prism @{} {{", family);
    let expected_ops = ["focus", "project", "split", "shift", "settle"];

    let lines: Vec<&str> = source.lines().collect();

    // Find the prism-decl line.
    for (i, line) in lines.iter().enumerate() {
        if line.trim_start() != decl_line {
            continue;
        }
        // Need at least i + 1 (decl) + 5 (arms) + 1 (close-brace) = i + 7 lines.
        if i + 6 >= lines.len() {
            return false;
        }
        // Check 5 arm lines.
        let mut all_match = true;
        for (offset, op) in expected_ops.iter().enumerate() {
            let arm_line = lines[i + 1 + offset].trim();
            let mut tokens = arm_line.split_whitespace();
            let arm_op = tokens.next();
            let arm_carrier = tokens.next();
            let extra = tokens.next();
            if arm_op != Some(*op) || arm_carrier != Some(last_seg) || extra.is_some() {
                all_match = false;
                break;
            }
        }
        if !all_match {
            continue;
        }
        // Check closing brace line.
        let close_line = lines[i + 6].trim();
        if close_line == "}" {
            return true;
        }
    }
    false
}

// =====================================================================
// Fire E M-E3.5-REVISED — beta-reduction primitives (source altitude)
// =====================================================================
//
// Per Mara canonical spec 5ad8528 §6.2: the fracture-detector at
// source altitude (M-E2) and the beta-reduction rule at AST altitude
// are the same operator at two altitudes. At rust/ altitude today, we
// operate on source bytes (no shard AST parser at rust/ yet; Fire D
// M5 territory). The source-level byte transformation IS equivalent
// to the AST-level reduction from the crystal-OID stability standpoint
// after re-parse: `crystal_oid(parse(reduce_source(s))) =
// crystal_oid(beta_normal(parse(s)))` by Church-Rosser (Mara math
// 5ad8528 §2). Lifting to full AST parse+reduce at rust/ altitude is
// forward-promised at Fire D M5 co-tick.
//
// M-E3.5-REVISED lands identity_projection_elision (P1) first per Fire
// A discipline (minimum primitive first; extensions land as demand
// pulls). P3/P4/P5 rule reducers land at subsequent M-E4 walker ticks
// as needed.

/// Result of applying a beta-reduction rule to a shard source at
/// rust/ altitude.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReduceResult {
    /// Rule matched and applied; carries the reduced source bytes.
    Reduced { new_source: String },
    /// Rule did not match (pattern absent, or fixed-point exemption).
    /// Source is unchanged.
    NoReduction,
    /// Reduction attempted but failed (path derivation error, read
    /// error, etc.). Substrate-honest reason string.
    Error(String),
}

/// Reduce the identity-projection-elision (P1) redex from a shard
/// source. Given a shard file that carries the P1 identity-carrier
/// prism block per Mara canonical spec 5ad8528 §1.1, remove the block
/// bytes and collapse resulting excess blank lines.
///
/// The reduced source, when re-parsed, produces the SAME crystal-OID
/// as the original source's beta-normal AST (Church-Rosser confluence
/// per Mara math 5ad8528 §2). This is the empirical operationalization
/// of Recognition candidate #82.
///
/// # Fixed-point exemption (Q3 Mara-lean; Alex 2026-08-09 concur)
///
/// `shards/prism.mirror` returns `NoReduction` — the self-referential
/// fixed-point IS the substrate's declaration of what the sugar rule
/// refers to; reducing it would collapse the substrate's own vocabulary.
///
/// # Substrate discipline
///
/// Rice-safe byte-check + byte-substring removal per Mara §1.1.
/// Terminates in O(source_len) at worst. No unbounded recursion; no
/// networking; no fs writes. Pure source-byte transformation.
pub fn reduce_identity_prism_at(shard_path: &Path) -> ReduceResult {
    // Q3 Mara-lean: fixed-point exemption for shards/prism.mirror.
    if let Some(name) = shard_path.file_name() {
        if name == "prism.mirror" {
            return ReduceResult::NoReduction;
        }
    }

    let family = match derive_family_from_shard_path(shard_path) {
        Some(f) => f,
        None => {
            return ReduceResult::Error(format!(
                "reduce_identity_prism_at: cannot derive family literal from path `{}`",
                shard_path.display()
            ))
        }
    };

    let last_seg: &str = family.rsplit('/').next().unwrap_or(&family);

    let source = match std::fs::read_to_string(shard_path) {
        Ok(s) => s,
        Err(e) => {
            return ReduceResult::Error(format!(
                "reduce_identity_prism_at: read_file(`{}`) failed: {}",
                shard_path.display(),
                e
            ))
        }
    };

    match find_p1_block_byte_range(&source, &family, last_seg) {
        Some((start, end)) => {
            let mut new_source = String::with_capacity(source.len() - (end - start));
            new_source.push_str(&source[..start]);
            new_source.push_str(&source[end..]);
            let new_source = collapse_excess_blank_lines(&new_source);
            ReduceResult::Reduced { new_source }
        }
        None => ReduceResult::NoReduction,
    }
}

/// Locate the byte range of a P1 identity-carrier prism block in
/// `source`. Returns `Some((start, end))` where the range covers the
/// full block including the opening `prism` keyword, all five arm
/// lines, the closing `}`, and one trailing newline if present.
///
/// Tolerates whitespace variation on arm lines per landed shard reality
/// (per source_carries_p1_identity_prism discipline).
fn find_p1_block_byte_range(source: &str, family: &str, last_seg: &str) -> Option<(usize, usize)> {
    let decl_line_prefix = format!("prism @{} {{", family);
    let expected_ops = ["focus", "project", "split", "shift", "settle"];

    for (pos, _) in source.match_indices(&decl_line_prefix) {
        // Verify pos is at start of a line (or start of source).
        if pos > 0 && !source[..pos].ends_with('\n') {
            continue;
        }

        // Skip past the decl-line's newline.
        let mut cursor = match source[pos..].find('\n') {
            Some(n) => pos + n + 1,
            None => continue,
        };

        // Verify 5 arm lines in canonical order with expected carrier.
        let mut arms_ok = true;
        for op in expected_ops.iter() {
            let line_end = match source[cursor..].find('\n') {
                Some(n) => cursor + n,
                None => {
                    arms_ok = false;
                    break;
                }
            };
            let arm_line = source[cursor..line_end].trim();
            let mut tokens = arm_line.split_whitespace();
            let arm_op = tokens.next();
            let arm_carrier = tokens.next();
            let extra = tokens.next();
            if arm_op != Some(*op) || arm_carrier != Some(last_seg) || extra.is_some() {
                arms_ok = false;
                break;
            }
            cursor = line_end + 1;
        }
        if !arms_ok {
            continue;
        }

        // Verify closing brace line.
        let close_line_end = source[cursor..]
            .find('\n')
            .map(|n| cursor + n)
            .unwrap_or(source.len());
        let close_line = source[cursor..close_line_end].trim();
        if close_line != "}" {
            continue;
        }

        // Include closing brace + trailing newline if present.
        let block_end = if close_line_end < source.len() {
            close_line_end + 1
        } else {
            close_line_end
        };
        return Some((pos, block_end));
    }
    None
}

/// Collapse runs of 3+ consecutive newlines to exactly 2 (single
/// blank line). Preserves single blank lines and no-blank-line
/// separations. Applied after block removal to keep source density
/// consistent with landed shard conventions (max one blank line
/// between sections per grep-verified corpus).
fn collapse_excess_blank_lines(source: &str) -> String {
    let mut result = String::with_capacity(source.len());
    let mut consecutive_newlines = 0;
    for ch in source.chars() {
        if ch == '\n' {
            consecutive_newlines += 1;
            if consecutive_newlines <= 2 {
                result.push(ch);
            }
        } else {
            consecutive_newlines = 0;
            result.push(ch);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Fixture: minimal substrate root with one bilateral-decl'd shard.
    /// Returns TempDir handle (dropped on test end — cleans up).
    fn fixture_root() -> TempDir {
        let dir = TempDir::new().expect("tempdir");
        let shards = dir.path().join("shards").join("test");
        fs::create_dir_all(&shards).expect("mkdir");
        let shard = shards.join("visibility.mirror");
        // Substrate-decl per shards/**/*.mirror grammar. `bilateral
        // <name> { sentinel "<bytes>" arity <n> }` shape extracted by
        // `roomba::mend::extract_bilaterals` line-scan (documented at
        // rust/roomba/src/mend.rs `fn extract_bilaterals`).
        let source = r#"# @test/visibility — fixture for apply_h::act tests.

bilateral consent_scope_universal {
  sentinel "scope=universal"
  arity 1
}
"#;
        fs::write(&shard, source).expect("write shard");
        dir
    }

    #[test]
    fn act_pass_on_sentinel_match() {
        let root = fixture_root();
        let verdict = act(
            root.path(),
            "@test/visibility.consent_scope_universal",
            &["scope=universal".to_string()],
        );
        assert_eq!(verdict, Verdict::Pass);
    }

    #[test]
    fn act_fail_on_sentinel_miss() {
        let root = fixture_root();
        let verdict = act(
            root.path(),
            "@test/visibility.consent_scope_universal",
            &["scope=private".to_string()],
        );
        match verdict {
            Verdict::Fail(reason) => {
                assert!(reason.contains("sentinel"));
                assert!(reason.contains("scope=universal"));
            }
            _ => panic!("expected Fail on sentinel miss, got {:?}", verdict),
        }
    }

    #[test]
    fn act_fail_on_unknown_action_ref() {
        let root = fixture_root();
        let verdict = act(
            root.path(),
            "@nonexistent/shard.nonexistent_predicate",
            &["anything".to_string()],
        );
        match verdict {
            Verdict::Fail(reason) => {
                assert!(reason.contains("not found in bilateral corpus"));
            }
            _ => panic!("expected Fail on unknown action_ref, got {:?}", verdict),
        }
    }

    // =================================================================
    // Fire E M-E2 tests — shard-body-projector primitive
    // =================================================================

    /// Fixture: minimal substrate root with a shard carrying the exact
    /// P1 identity-carrier prism block (matches shards/spectral.mirror
    /// grep-verified byte-shape).
    fn fixture_p1_shard(carrier: &str, family_relpath: &str) -> TempDir {
        let dir = TempDir::new().expect("tempdir");
        let shard_path = dir.path().join("shards").join(family_relpath);
        fs::create_dir_all(shard_path.parent().unwrap()).expect("mkdir");
        let source = format!(
            "# Fixture shard for detect_prism_boilerplate P1 tests.\n\
             \n\
             prism @{family} {{\n\
               focus {c}\n\
               project {c}\n\
               split {c}\n\
               shift {c}\n\
               settle {c}\n\
             }}\n\
             \n\
             out @{family}\n",
            family = carrier,
            c = carrier,
        );
        fs::write(&shard_path, source).expect("write shard");
        dir
    }

    #[test]
    fn strip_fracture_detect_ref_recognizes_prism_boilerplate() {
        assert_eq!(
            strip_fracture_detect_ref("@kintsugi/fracture/prism_boilerplate.detect"),
            Some("prism_boilerplate")
        );
    }

    #[test]
    fn strip_fracture_detect_ref_rejects_non_fracture_action() {
        assert_eq!(
            strip_fracture_detect_ref("@subject/visibility/public.consent_scope_universal"),
            None
        );
    }

    #[test]
    fn strip_fracture_detect_ref_rejects_non_detect_suffix() {
        assert_eq!(
            strip_fracture_detect_ref("@kintsugi/fracture/prism_boilerplate.resolve"),
            None
        );
    }

    #[test]
    fn derive_family_depth_0_shard() {
        assert_eq!(
            derive_family_from_shard_path(Path::new("shards/spectral.mirror")).as_deref(),
            Some("spectral")
        );
    }

    #[test]
    fn derive_family_depth_1_shard() {
        assert_eq!(
            derive_family_from_shard_path(Path::new("shards/io/fs.mirror")).as_deref(),
            Some("io/fs")
        );
    }

    #[test]
    fn derive_family_depth_3_shard() {
        assert_eq!(
            derive_family_from_shard_path(Path::new(
                "shards/kintsugi/fracture/keyword.mirror"
            ))
            .as_deref(),
            Some("kintsugi/fracture/keyword")
        );
    }

    #[test]
    fn derive_family_absolute_tempdir_path() {
        assert_eq!(
            derive_family_from_shard_path(Path::new(
                "/tmp/reed-fixture-abc/shards/spectral.mirror"
            ))
            .as_deref(),
            Some("spectral")
        );
    }

    #[test]
    fn derive_family_returns_none_for_non_shard_path() {
        assert_eq!(
            derive_family_from_shard_path(Path::new("/tmp/foo.txt")),
            None
        );
    }

    #[test]
    fn source_carries_p1_identity_prism_pass_on_canonical_shape() {
        // Extra blank line between arms should NOT match — arms must be
        // contiguous per Mara §1.1 sentinel byte-shape.
        let noncontiguous = "prism @spectral {\n  focus spectral\n  project spectral\n\
                             \n  split spectral\n  shift spectral\n  settle spectral\n}\n";
        assert!(!source_carries_p1_identity_prism(
            noncontiguous,
            "spectral",
            "spectral"
        ));

        // Canonical shape (contiguous 5 arms) passes.
        let canonical = format!(
            "prism @spectral {{\n  focus spectral\n  project spectral\n  \
             split spectral\n  shift spectral\n  settle spectral\n}}\n"
        );
        assert!(source_carries_p1_identity_prism(
            &canonical,
            "spectral",
            "spectral"
        ));
    }

    #[test]
    fn source_carries_p1_tolerates_double_space_arm_indent() {
        // shards/io/fs.mirror grep-verified: uses `focus  fs` (double space).
        let source = format!(
            "prism @io/fs {{\n  focus  fs\n  project fs\n  split  fs\n  \
             shift  fs\n  settle fs\n}}\n"
        );
        assert!(source_carries_p1_identity_prism(&source, "io/fs", "fs"));
    }

    #[test]
    fn source_carries_p1_fails_for_named_carrier_variant() {
        // P2 named-carrier shape (Case 2): family=@dance, arms=ensemble.
        // Should NOT match P1 detector.
        let source = format!(
            "prism @dance {{\n  focus ensemble\n  project ensemble\n  \
             split ensemble\n  shift ensemble\n  settle ensemble\n}}\n"
        );
        assert!(!source_carries_p1_identity_prism(&source, "dance", "dance"));
    }

    #[test]
    fn source_carries_p1_fails_for_missing_arm() {
        // 4 arms instead of 5 — should NOT match.
        let source = format!(
            "prism @spectral {{\n  focus spectral\n  project spectral\n  \
             split spectral\n  shift spectral\n}}\n"
        );
        assert!(!source_carries_p1_identity_prism(&source, "spectral", "spectral"));
    }

    #[test]
    fn source_carries_p1_fails_for_wrong_op_order() {
        // Op order matters (focus/project/split/shift/settle is canonical).
        let source = format!(
            "prism @spectral {{\n  focus spectral\n  split spectral\n  \
             project spectral\n  shift spectral\n  settle spectral\n}}\n"
        );
        assert!(!source_carries_p1_identity_prism(&source, "spectral", "spectral"));
    }

    #[test]
    fn detect_prism_boilerplate_pass_on_fixture_p1_shard() {
        let dir = fixture_p1_shard("spectral", "spectral.mirror");
        let shard_path = dir.path().join("shards").join("spectral.mirror");
        assert_eq!(detect_prism_boilerplate_at(&shard_path), Verdict::Pass);
    }

    #[test]
    fn detect_prism_boilerplate_fixed_point_exemption() {
        let dir = fixture_p1_shard("prism", "prism.mirror");
        let shard_path = dir.path().join("shards").join("prism.mirror");
        // Even though the fixture matches P1 byte-shape, the Q3 fixed-
        // point exemption applies to shards/prism.mirror.
        match detect_prism_boilerplate_at(&shard_path) {
            Verdict::Fail(reason) => {
                assert!(reason.contains("fixed-point exemption"));
                assert!(reason.contains("prism.mirror"));
            }
            other => panic!("expected Fail on fixed-point exemption, got {:?}", other),
        }
    }

    #[test]
    fn detect_prism_boilerplate_fail_on_missing_file() {
        let dir = TempDir::new().expect("tempdir");
        let shard_path = dir.path().join("shards").join("nonexistent.mirror");
        match detect_prism_boilerplate_at(&shard_path) {
            Verdict::Fail(reason) => assert!(reason.contains("read_file")),
            other => panic!("expected Fail on missing file, got {:?}", other),
        }
    }

    #[test]
    fn dispatch_via_act_routes_to_fracture_detector_pass() {
        let dir = fixture_p1_shard("spectral", "spectral.mirror");
        let verdict = act(
            dir.path(),
            "@kintsugi/fracture/prism_boilerplate.detect",
            &["shards/spectral.mirror".to_string()],
        );
        assert_eq!(verdict, Verdict::Pass);
    }

    #[test]
    fn dispatch_via_act_returns_fail_for_unknown_fracture_species() {
        let dir = TempDir::new().expect("tempdir");
        let verdict = act(
            dir.path(),
            "@kintsugi/fracture/glass_boilerplate.detect",
            &["shards/some_species.mirror".to_string()],
        );
        match verdict {
            Verdict::Fail(reason) => {
                assert!(reason.contains("no detector primitive"));
                assert!(reason.contains("glass_boilerplate"));
            }
            other => panic!("expected Fail on unknown fracture species, got {:?}", other),
        }
    }

    #[test]
    fn dispatch_via_act_returns_fail_for_missing_shard_path_arg() {
        let dir = TempDir::new().expect("tempdir");
        let verdict = act(
            dir.path(),
            "@kintsugi/fracture/prism_boilerplate.detect",
            &[],
        );
        match verdict {
            Verdict::Fail(reason) => assert!(reason.contains("missing shard_path")),
            other => panic!("expected Fail on missing shard_path, got {:?}", other),
        }
    }

    // =================================================================
    // Fire E M-E3.5-REVISED tests — beta-reduction primitives
    // =================================================================

    #[test]
    fn strip_normalization_rule_reducible_ref_recognizes_identity_projection() {
        assert_eq!(
            strip_normalization_rule_reducible_ref(
                "@epistemologic/normalization/rules/identity_projection_elision.reducible"
            ),
            Some("identity_projection_elision")
        );
    }

    #[test]
    fn strip_normalization_rule_reducible_ref_rejects_non_normalization_action() {
        assert_eq!(
            strip_normalization_rule_reducible_ref(
                "@kintsugi/fracture/prism_boilerplate.detect"
            ),
            None
        );
    }

    #[test]
    fn strip_normalization_rule_reducible_ref_rejects_non_reducible_suffix() {
        assert_eq!(
            strip_normalization_rule_reducible_ref(
                "@epistemologic/normalization/rules/identity_projection_elision.reduce"
            ),
            None
        );
    }

    #[test]
    fn reduce_identity_prism_at_removes_block_from_p1_fixture() {
        let dir = fixture_p1_shard("spectral", "spectral.mirror");
        let shard_path = dir.path().join("shards").join("spectral.mirror");
        match reduce_identity_prism_at(&shard_path) {
            ReduceResult::Reduced { new_source } => {
                // Reduced source must NOT contain the prism block.
                assert!(!new_source.contains("prism @spectral {"));
                assert!(!new_source.contains("  focus spectral"));
                assert!(!new_source.contains("  settle spectral"));
                // But MUST preserve the surrounding docblock + out line.
                assert!(new_source.contains("# Fixture shard for detect_prism_boilerplate P1 tests."));
                assert!(new_source.contains("out @spectral"));
            }
            other => panic!("expected Reduced on P1 fixture, got {:?}", other),
        }
    }

    #[test]
    fn reduce_identity_prism_at_is_idempotent() {
        // Round-trip: after reduce, detect must return Fail (no P1 block).
        let dir = fixture_p1_shard("spectral", "spectral.mirror");
        let shard_path = dir.path().join("shards").join("spectral.mirror");

        let reduced = match reduce_identity_prism_at(&shard_path) {
            ReduceResult::Reduced { new_source } => new_source,
            other => panic!("expected Reduced, got {:?}", other),
        };

        // Write reduced source back, verify detector no longer fires.
        fs::write(&shard_path, &reduced).expect("rewrite");
        let post_verdict = detect_prism_boilerplate_at(&shard_path);
        match post_verdict {
            Verdict::Fail(_) => {} // idempotent: reduced source doesn't detect P1
            other => panic!("expected Fail after reduce (idempotent), got {:?}", other),
        }

        // And re-reducing the reduced source is NoReduction.
        match reduce_identity_prism_at(&shard_path) {
            ReduceResult::NoReduction => {}
            other => panic!("expected NoReduction on re-reduce, got {:?}", other),
        }
    }

    #[test]
    fn reduce_identity_prism_at_no_reduction_for_fixed_point_exemption() {
        let dir = fixture_p1_shard("prism", "prism.mirror");
        let shard_path = dir.path().join("shards").join("prism.mirror");
        match reduce_identity_prism_at(&shard_path) {
            ReduceResult::NoReduction => {} // Q3 fixed-point exemption
            other => panic!("expected NoReduction on shards/prism.mirror, got {:?}", other),
        }
    }

    #[test]
    fn reduce_identity_prism_at_no_reduction_for_non_p1_shard() {
        // Write a shard that does NOT carry P1 pattern (named-carrier P2).
        let dir = TempDir::new().expect("tempdir");
        let shards = dir.path().join("shards");
        fs::create_dir_all(&shards).expect("mkdir");
        let shard_path = shards.join("dance.mirror");
        let source = "# Non-P1 fixture.\n\n\
                      prism @dance {\n  \
                      focus ensemble\n  \
                      project ensemble\n  \
                      split ensemble\n  \
                      shift ensemble\n  \
                      settle ensemble\n\
                      }\n";
        fs::write(&shard_path, source).expect("write");
        match reduce_identity_prism_at(&shard_path) {
            ReduceResult::NoReduction => {} // named-carrier P2, not P1
            other => panic!("expected NoReduction on non-P1 shard, got {:?}", other),
        }
    }

    #[test]
    fn reduce_identity_prism_at_collapses_excess_blank_lines() {
        // After removing the block, verify blank-line collapse policy
        // (max 2 consecutive newlines = 1 blank line between sections).
        let dir = fixture_p1_shard("spectral", "spectral.mirror");
        let shard_path = dir.path().join("shards").join("spectral.mirror");
        let reduced = match reduce_identity_prism_at(&shard_path) {
            ReduceResult::Reduced { new_source } => new_source,
            other => panic!("expected Reduced, got {:?}", other),
        };
        // No triple+ newlines in reduced source.
        assert!(!reduced.contains("\n\n\n"));
    }

    #[test]
    fn dispatch_via_act_routes_to_reducible_pass() {
        // act() dispatch of @epistemologic/normalization/rules/
        // identity_projection_elision.reducible routes to the M-E2
        // detector; same Pass verdict on P1 fixture.
        let dir = fixture_p1_shard("spectral", "spectral.mirror");
        let verdict = act(
            dir.path(),
            "@epistemologic/normalization/rules/identity_projection_elision.reducible",
            &["shards/spectral.mirror".to_string()],
        );
        assert_eq!(verdict, Verdict::Pass);
    }

    #[test]
    fn dispatch_via_act_reducible_returns_fail_for_unknown_rule() {
        let dir = TempDir::new().expect("tempdir");
        let verdict = act(
            dir.path(),
            "@epistemologic/normalization/rules/glass_identity_elision.reducible",
            &["shards/some_species.mirror".to_string()],
        );
        match verdict {
            Verdict::Fail(reason) => {
                assert!(reason.contains("no reducible primitive"));
                assert!(reason.contains("glass_identity_elision"));
            }
            other => panic!("expected Fail on unknown rule, got {:?}", other),
        }
    }

    #[test]
    fn collapse_excess_blank_lines_preserves_single_blank() {
        assert_eq!(
            collapse_excess_blank_lines("a\n\nb"),
            "a\n\nb"
        );
    }

    #[test]
    fn collapse_excess_blank_lines_collapses_triple_to_double() {
        assert_eq!(
            collapse_excess_blank_lines("a\n\n\nb"),
            "a\n\nb"
        );
    }

    #[test]
    fn collapse_excess_blank_lines_collapses_quintuple_to_double() {
        assert_eq!(
            collapse_excess_blank_lines("a\n\n\n\n\nb"),
            "a\n\nb"
        );
    }

    #[test]
    fn act_multi_arg_sentinel_matches_concatenation() {
        let root = fixture_root();
        let verdict = act(
            root.path(),
            "@test/visibility.consent_scope_universal",
            &[
                "peer=alice".to_string(),
                "scope=universal".to_string(),
                "trust=1".to_string(),
            ],
        );
        assert_eq!(verdict, Verdict::Pass);
    }
}
