//! Arc 6 TICK 1 RED — `@song` top-level family-root.
//!
//! Per Seam pre-review at `3d910bc` (docs/audits/2026-07-06-seam-pre-review-song-substrate-decl.md):
//! RATIFY-WITH-CORRECTIONS on Shape β from Mara spec `c0101f8`.
//!
//! **Substrate-pull-notable outcome**: #S3 five-op temporal specialisation verified
//! STRUCTURAL against `shards/prism.mirror` L38-45 canonical linalg + `cadence.mirror`
//! L1-80 Lawvere framing. If ratified after this family-root + 3 species land, `@song`
//! becomes the FIRST substrate species to lift ALL FIVE operations at a non-mathematical
//! altitude.
//!
//! Hedges resolved in review:
//! - H1: family-root (not marker; song carries typed species with actions)
//! - H2: @song (not @progression; legibility-over-foundation per @loop-vs-@moi precedent)
//!
//! Sibling to `@mirror` / `@kintsugi` / `@loop` / `@third`.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_song_shard() -> String {
    let path = repo_root().join("shards/song.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read shards/song.mirror at {:?}: {}", path, e))
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

// === T1-T4: canonical shape + Interpretation B baseline ===

#[test]
fn t01_song_shard_file_exists_and_declares_family_root() {
    let content = read_song_shard();
    assert!(
        content.contains("prism @song"),
        "T1: shards/song.mirror must declare `prism @song` family-root per path-namespace pact"
    );
}

#[test]
fn t02_first_nonempty_line_is_narrative_docblock() {
    let content = read_song_shard();
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
    let content = read_song_shard();
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
fn t04_song_inherits_prism_meta_glass() {
    let content = read_song_shard();
    for req in ["in @prism", "in @meta", "in @glass"] {
        assert!(
            content.contains(req),
            "T4: must inherit `{}` (universal + transparency)",
            req
        );
    }
}

#[test]
fn t05_song_declares_five_op_prism_body_discharges_s3() {
    let content = read_song_shard();
    for op in ["focus", "project", "split", "shift", "settle"] {
        assert!(
            content.contains(op),
            "T5: prism @song must declare five-op body operation `{}` — discharges recognition #S3 (five-op temporal specialisation)",
            op
        );
    }
}

// === T6-T10: species forward-promises (voice/movement/progression/phrase/narrative) ===

#[test]
fn t06_song_names_voice_species_forward_promise() {
    let content = read_song_shard();
    assert!(
        content.contains("song/voice") || content.contains("@song/voice") || content.contains("voice"),
        "T6: must name `voice` as forward-promised species (orchestra binding; lifts @mirror/spectral/voice)"
    );
}

#[test]
fn t07_song_names_movement_species_forward_promise() {
    let content = read_song_shard();
    assert!(
        content.contains("movement"),
        "T7: must name `movement` as forward-promised species (StageFreight cascade binding: audition/perform/review/publish/narrate)"
    );
}

#[test]
fn t08_song_names_progression_species_forward_promise() {
    let content = read_song_shard();
    assert!(
        content.contains("progression"),
        "T8: must name `progression` as forward-promised species (music binding; consumes @epistemologic/math/music/cadence)"
    );
}

#[test]
fn t09_song_names_phrase_species_forward_promise() {
    let content = read_song_shard();
    assert!(
        content.contains("phrase"),
        "T9: must name `phrase` as forward-promised species (atomic unit; OBC binding closure)"
    );
}

#[test]
fn t10_song_names_narrative_species_forward_promise() {
    let content = read_song_shard();
    assert!(
        content.contains("narrative"),
        "T10: must name `narrative` as forward-promised species (psychohistory + wire binding; sibling to @io/stagefreight/narrative)"
    );
}

// === T11-T14: composition points (four altitudes) ===

#[test]
fn t11_song_composes_with_epistemologic_math_music() {
    let content = read_song_shard();
    assert!(
        content.contains("@epistemologic/math/music"),
        "T11: must cite `@epistemologic/math/music` (audible altitude ancestor; music-math family-root already lands harmonic/interval/dissonance/cadence)"
    );
}

#[test]
fn t12_song_composes_with_kintsugi_oscillate_and_shift() {
    let content = read_song_shard();
    assert!(
        content.contains("@kintsugi/oscillate") && content.contains("@kintsugi/shift"),
        "T12: must cite `@kintsugi/oscillate` (rhythmic pulse) + `@kintsugi/shift` (cross-altitude morphism, TICK 2 `49f0486`) — process altitude ancestors"
    );
}

#[test]
fn t13_song_composes_with_mirror_spectral_score() {
    let content = read_song_shard();
    assert!(
        content.contains("@mirror/spectral/score") || content.contains("@mirror/spectral"),
        "T13: must cite `@mirror/spectral/score` (orchestra altitude ancestor; mirror-spectral.md 'the Pack IS the orchestra; the metalogue IS the score')"
    );
}

#[test]
fn t14_song_composes_with_io_stagefreight_narrative() {
    let content = read_song_shard();
    assert!(
        content.contains("@io/stagefreight/narrative") || content.contains("@io/stagefreight"),
        "T14: must cite `@io/stagefreight/narrative` (wire altitude ancestor; StageFreight stage_play cascade Story→Play→Narrative)"
    );
}

// === T15: prior-art anchors + hedge/candidate markers ===

#[test]
fn t15_narrative_grounds_s3_and_cadence_lawvere_and_schenker() {
    let content = read_song_shard();
    // Seam's per-#S3 correction: cadence.mirror Lawvere framing must be cited as `settle @ temporal` ancestor
    let has_cadence_lawvere = content.contains("cadence.mirror")
        || content.contains("Lawvere")
        || content.contains("authentic cadence");
    assert!(
        has_cadence_lawvere,
        "T15a: narrative must cite `cadence.mirror` Lawvere framing as `settle @ temporal` ancestor per Seam Correction (audit `3d910bc`)"
    );
    // Schenker for project @ temporal
    assert!(
        content.contains("Schenker"),
        "T15b: narrative must cite Schenker (Schenkerian reduction) as `project @ temporal` ancestor per Seam §5 prior-art anchors"
    );
    // #S3 candidate marker
    assert!(
        content.contains("#S3") || content.contains("five-op temporal") || content.contains("five operations"),
        "T15c: narrative must carry #S3 recognition candidate marker (five-op temporal specialisation — substrate-pull-notable outcome pending Pack promotion after 3 species land)"
    );
}
