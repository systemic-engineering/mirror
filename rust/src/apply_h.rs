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
