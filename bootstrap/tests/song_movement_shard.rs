//! Arc 6 TICK 4 RED — `shards/song/movement.mirror` species.
//!
//! Per Seam pre-review at `3d910bc` (docs/audits/2026-07-06-seam-pre-review-song-substrate-decl.md)
//! §6.5: **TICK 4 lands `shards/song/movement.mirror` — StageFreight cascade binding.**
//!
//! Movement is the third `@song` species (after `@song/progression` Arc 6 TICK 2 `54ff1e8`
//! and `@song/voice` Arc 6 TICK 3 `cc5a440`). Per family-root `shards/song.mirror`
//! species-roster:
//!
//! > `@song/movement` — a bounded frame-shift that unfolds over time; binds StageFreight's
//! > five-stage cascade (audition/perform/review/publish/narrate) at the song altitude.
//! > Actions: `song_movement(m: ref) -> ref` (with enter/close at species altitude).
//!
//! **StageFreight binding**: `@io/stagefreight` (family-root at
//! `shards/io/stagefreight.mirror`) declares the wire-protocol boundary for settled-
//! crystal transport. Movement's structural anchor IS the five-stage cascade
//! `audition → perform → review → publish → narrate`. This cascade IS what a movement
//! DOES at the song altitude: audition the material, perform the frame, review the
//! result, publish the closure, narrate the outcome onto the wire.
//!
//! **Psychohistorical binding** (per Seam pre-review §4, STRONG): `movement ⇔ frame-
//! bounded epoch` per `glossary/Frame.md` + `frame.mirror` Bateson-grading. The
//! psychohistorical failure-mode at Narcissus-pole is a frame-that-cannot-close
//! (either the epoch runs indefinitely, or the closure is deceptive).
//!
//! **NO new `song_settles` sub-predicates**: the composed bilateral fully discharged
//! at TICK 3. This species is consolidation — it binds StageFreight to the temporal-
//! bounded-frame altitude and honours movement discipline without adding new gates
//! to the family-root's admissibility contract.
//!
//! **Recognition #S3 LANDED** at Seam Phase D on TICK 3 `10c34cf`. Movement joins
//! the two witnesses (progression + voice) as third species; consolidates the
//! five-op temporal specialisation at species altitude.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_movement_shard() -> String {
    let path = repo_root().join("shards/song/movement.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read shards/song/movement.mirror at {:?}: {}", path, e))
}

fn first_nonempty_line(content: &str) -> Option<&str> {
    content.lines().find(|l| !l.trim().is_empty())
}

fn seam_line_indices(content: &str) -> Vec<usize> {
    content
        .lines()
        .enumerate()
        .filter_map(|(i, l)| if l == "---" { Some(i) } else { None })
        .collect()
}

// === T1-T5: canonical shape + Interpretation B baseline + species pact ===

#[test]
fn t01_song_movement_shard_file_exists_and_declares_species() {
    let content = read_movement_shard();
    assert!(
        content.contains("prism @song/movement")
            || content.contains("prism @song / movement"),
        "T1: shards/song/movement.mirror must declare `prism @song/movement` species per path-namespace pact"
    );
}

#[test]
fn t02_first_nonempty_line_is_narrative_docblock() {
    let content = read_movement_shard();
    let first = first_nonempty_line(&content).expect("T2: must have non-empty content");
    assert!(
        first.trim_start().starts_with('#'),
        "T2: first non-empty line must be `#`-narrative per Interpretation B; got `{}`",
        first
    );
    assert_ne!(first.trim(), "---", "T2: line-1 `---` is drift");
    assert!(
        !first.trim_start().starts_with("in "),
        "T2: `in @...` clauses live BELOW the seam"
    );
}

#[test]
fn t03_exactly_one_seam_at_column_zero_and_in_clauses_below() {
    let content = read_movement_shard();
    let seams = seam_line_indices(&content);
    assert_eq!(
        seams.len(),
        1,
        "T3: exactly one `---` at column 0 required; found {}",
        seams.len()
    );
    let seam_idx = seams[0];
    for (i, line) in content.lines().enumerate() {
        if line.trim_start().starts_with("in @") {
            assert!(
                i > seam_idx,
                "T3: `in @...` at line {} appears ABOVE seam at line {}. Line: `{}`",
                i + 1,
                seam_idx + 1,
                line
            );
        }
    }
}

#[test]
fn t04_movement_inherits_prism_meta_glass() {
    let content = read_movement_shard();
    for req in ["in @prism", "in @meta", "in @glass"] {
        assert!(
            content.contains(req),
            "T4: must inherit `{}` (universal + transparency)",
            req
        );
    }
}

#[test]
fn t05_movement_inherits_song_family_root() {
    let content = read_movement_shard();
    assert!(
        content.contains("in @song"),
        "T5: species must declare `in @song` — inherits family-root's carriers and #S3 (LANDED) five-op temporal specialisation"
    );
}

// === T6-T7: StageFreight binding (family-root + five-stage cascade) ===

#[test]
fn t06_movement_composes_with_io_stagefreight() {
    let content = read_movement_shard();
    assert!(
        content.contains("@io/stagefreight"),
        "T6: species MUST cite `@io/stagefreight` (wire-protocol boundary family-root; docs/specs/stagefreight-wire-v0.1.md forward-promised) — movement binds StageFreight's cascade at the song altitude per family-root song.mirror species-roster"
    );
}

#[test]
fn t07_movement_cites_five_stage_stagefreight_cascade() {
    let content = read_movement_shard();
    // Five-stage cascade: audition → perform → review → publish → narrate.
    // At minimum, three of five stages must appear as substrate-decl vocabulary.
    let stages: Vec<&str> = ["audition", "perform", "review", "publish", "narrate"]
        .iter()
        .filter(|s| content.contains(*s))
        .copied()
        .collect();
    assert!(
        stages.len() >= 3,
        "T7: species narrative MUST cite at least 3 of the 5 StageFreight cascade stages (audition / perform / review / publish / narrate). Found: {:?}. Per family-root song.mirror species-roster: `binds StageFreight's five-stage cascade (audition/perform/review/publish/narrate)`.",
        stages
    );
}

// === T8-T9: enter/close species actions ===

#[test]
fn t08_movement_declares_enter_action() {
    let content = read_movement_shard();
    // Per family-root `# @song/movement ... Actions: song_movement(m: ref) -> ref`
    // and species-roster line "movement (enter/close)".
    let has_enter = content.contains("enter(m") || content.contains("enter(movement");
    assert!(
        has_enter,
        "T8: species MUST declare an `enter(m: song_movement, ...)` or `enter(movement: ..., ...)` action. Per family-root song.mirror species-roster: movement actions are `enter` and `close` at species altitude."
    );
}

#[test]
fn t09_movement_declares_close_action() {
    let content = read_movement_shard();
    let has_close = content.contains("close(m") || content.contains("close(movement");
    assert!(
        has_close,
        "T9: species MUST declare a `close(m: song_movement, ...)` or `close(movement: ..., ...)` action. Per family-root song.mirror species-roster: movement actions are `enter` and `close`. Close IS the frame-boundary closure event."
    );
}

// === T10: bounded frame-shift narrative anchor ===

#[test]
fn t10_movement_grounds_bounded_frame_shift_narrative() {
    let content = read_movement_shard();
    let has_frame_shift = content.contains("frame-shift")
        || content.contains("frame shift")
        || content.contains("bounded frame")
        || content.contains("frame-bounded")
        || content.contains("bounded-frame")
        || content.contains("bounded epoch");
    assert!(
        has_frame_shift,
        "T10: species narrative MUST ground the bounded-frame-shift discipline — cite `frame-shift`, `bounded frame`, `frame-bounded`, or `bounded epoch`. Per family-root song.mirror species-roster: `a bounded frame-shift that unfolds over time`."
    );
}

// === T11-T12: prior-art anchors (sonata form / symphonic movement + Bateson frame) ===

#[test]
fn t11_movement_cites_sonata_or_symphonic_prior_art() {
    let content = read_movement_shard();
    let has_movement_anchor = content.contains("sonata")
        || content.contains("Sonata")
        || content.contains("symphony")
        || content.contains("symphonic")
        || content.contains("Symphony")
        || content.contains("Beethoven")
        || content.contains("Haydn")
        || content.contains("Mozart");
    assert!(
        has_movement_anchor,
        "T11: species narrative MUST cite classical-music prior art for movement structure — `sonata` (sonata form as bounded frame-shift), `symphony`/`symphonic` (multi-movement work structure), or a canonical composer (Haydn / Mozart / Beethoven). Movement discipline traces to the symphonic tradition."
    );
}

#[test]
fn t12_movement_cites_bateson_frame_or_frame_shard() {
    let content = read_movement_shard();
    // Per Seam pre-review §4 STRONG binding: movement ⇔ frame-bounded epoch
    // per glossary/Frame.md + frame.mirror Bateson-grading.
    let has_frame_anchor = content.contains("Bateson")
        || content.contains("frame.mirror")
        || content.contains("@frame")
        || content.contains("Frame.md")
        || content.contains("logical type")
        || content.contains("Learning II")
        || content.contains("Learning III");
    assert!(
        has_frame_anchor,
        "T12: species narrative MUST cite the frame-shift ancestor — `Bateson` (logical types / Learning II or III), `frame.mirror`, `@frame`, or `Frame.md`. Per Seam pre-review §4: movement ⇔ frame-bounded epoch STRONG binding via Bateson-grading."
    );
}

// === T13: psychohistorical binding (frame-bounded epoch per spec §8) ===

#[test]
fn t13_movement_cites_psychohistorical_epoch_binding() {
    let content = read_movement_shard();
    // Per spec §8 + Seam §4: movement ⇔ frame-bounded epoch STRONG binding.
    // The species narrative should acknowledge this psychohistorical altitude.
    let has_epoch_binding = content.contains("epoch")
        || content.contains("Epoch")
        || content.contains("psychohistor")
        || content.contains("OBC")
        || content.contains("regulation stock")
        || content.contains("ADO");
    assert!(
        has_epoch_binding,
        "T13: species narrative MUST cite the psychohistorical binding — `epoch` (frame-bounded epoch per Seam §4), `psychohistory`, `OBC`, `regulation stock`, or `ADO`. Per spec §8: movement corresponds to a frame-bounded epoch in Alex's systemic.engineering corpus."
    );
}

// === T14: composed-bilateral acknowledgement (no new sub-predicate; song_settles closed at TICK 3) ===

#[test]
fn t14_movement_acknowledges_song_settles_closure_at_tick_3() {
    let content = read_movement_shard();
    // Movement does NOT add a new song_settles sub-predicate; the composed bilateral
    // fully discharged at TICK 3 (@song/voice `cc5a440`). Species narrative must
    // acknowledge this or cite one of the closure vocabulary items.
    let has_closure_ack = content.contains("song_settles")
        || content.contains("composed bilateral")
        || content.contains("composed-bilateral")
        || content.contains("TICK 3")
        || content.contains("@song/voice")
        || content.contains("@song/progression")
        || content.contains("#53");
    assert!(
        has_closure_ack,
        "T14: species narrative MUST acknowledge the composed-bilateral closure landscape — cite `song_settles`, `composed bilateral`, `TICK 3`, `@song/voice`, `@song/progression`, or `#53`. Movement is consolidation, not a new sub-predicate; the family-root bilateral already discharges via progression + voice."
    );
}

// === T15: #S3 LANDED acknowledgement + species-roster sibling awareness ===

#[test]
fn t15_movement_acknowledges_s3_landed_and_species_siblings() {
    let content = read_movement_shard();
    // #S3 promoted CANDIDATE → LANDED at Seam Phase D `10c34cf` after TICK 3.
    // Species narrative should mark #S3 as LANDED (or at minimum cite #S3 +
    // acknowledge the five-op temporal specialisation Movement inherits).
    let has_s3_landed = content.contains("#S3")
        || content.contains("five-op temporal")
        || content.contains("five-operation temporal")
        || content.contains("five operations");
    assert!(
        has_s3_landed,
        "T15: species narrative MUST cite `#S3` recognition or the five-op temporal specialisation. #S3 PROMOTED to LANDED at Seam Phase D `10c34cf` after TICK 3 close; movement inherits and consolidates."
    );
}
