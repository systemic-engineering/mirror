//! Arc 6 TICK 3 RED — `shards/song/voice.mirror` species.
//!
//! Per Seam pre-review at `3d910bc` (docs/audits/2026-07-06-seam-pre-review-song-substrate-decl.md)
//! §6.4: **TICK 3 lands `shards/song/voice.mirror` — orchestra binding.**
//!
//! Voice is the second `@song` species (after `@song/progression` Arc 6 TICK 2 `54ff1e8`).
//! It carries the THIRD and FINAL sub-predicate of the family-root's
//! `song_settles(pr, p) -> verdict` composed bilateral (14th #53 instance):
//!
//! - `voice_line_valid(v: song_voice, p: perturbation) -> verdict` — the voices
//!   carry stepwise-or-intentional-leap discipline. FAIL = GLUE WORK (contrapuntal
//!   work invisibly holds the harmony but is not attributed; the voice-leading
//!   discipline is violated by systematic-invisibility-of-stepwise-motion; spec
//!   §8.2 psychohistorical Narcissus-pole naming).
//!
//! **Orchestra binding** at species altitude:
//!
//! - `@mirror/spectral/voice` (orchestra altitude — peer-as-agent with named
//!   authority over sections). `@song/voice` ADDS the *when* to
//!   `@mirror/spectral/voice`'s *what/who*: an agent's time-indexed trajectory
//!   through their authored sections.
//! - `@epistemologic/math/music/voice` (audible altitude — voice-leading
//!   constraints; forward-promised species). Voice-leading discipline lives
//!   at the audible altitude; `@song/voice` composes at temporal altitude.
//!
//! **#S3 five-op temporal specialisation** — THE species-altitude discharge
//! of `focus @ temporal` per family-root prism block. Voice specializes focus
//! as SOLOING / solistic attention: attend to ONE voice / one line at a time.
//! This species contributes the third witness to #S3's promotion criterion
//! (Seam §6.8): after TICK 3 lands, `@song` becomes FIRST substrate species
//! lifting ALL FIVE prism operations at a non-mathematical altitude.
//!
//! Species sibling: `@song/progression` (Arc 6 TICK 2 `54ff1e8`), forthcoming
//! `@song/movement` (TICK 4), `@song/narrative` (TICK 5), `@song/phrase` (TICK 6).

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_voice_shard() -> String {
    let path = repo_root().join("shards/song/voice.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read shards/song/voice.mirror at {:?}: {}", path, e))
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
fn t01_song_voice_shard_file_exists_and_declares_species() {
    let content = read_voice_shard();
    assert!(
        content.contains("prism @song/voice") || content.contains("prism @song / voice"),
        "T1: shards/song/voice.mirror must declare `prism @song/voice` species per path-namespace pact"
    );
}

#[test]
fn t02_first_nonempty_line_is_narrative_docblock() {
    let content = read_voice_shard();
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
    let content = read_voice_shard();
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
fn t04_voice_inherits_prism_meta_glass() {
    let content = read_voice_shard();
    for req in ["in @prism", "in @meta", "in @glass"] {
        assert!(
            content.contains(req),
            "T4: must inherit `{}` (universal + transparency)",
            req
        );
    }
}

#[test]
fn t05_voice_inherits_song_family_root() {
    let content = read_voice_shard();
    assert!(
        content.contains("in @song"),
        "T5: species must declare `in @song` — inherits family-root's `song_voice` carrier + `song_settles` composed bilateral (voice_line_valid discharge point)"
    );
}

// === T6: discharge the third `song_settles` sub-predicate ===

#[test]
fn t06_voice_discharges_voice_line_valid_sub_predicate() {
    let content = read_voice_shard();
    assert!(
        content.contains("voice_line_valid"),
        "T6: species MUST declare `voice_line_valid(v: song_voice, p: perturbation) -> verdict {{ \\ }}` — the THIRD and FINAL sub-predicate of family-root's `song_settles` composed bilateral (per shards/song.mirror §3). After this lands, all three song_settles sub-predicates discharge."
    );
}

// === T7-T8: orchestra binding — @mirror/spectral/voice + @epistemologic/math/music/voice ===

#[test]
fn t07_voice_composes_with_mirror_spectral_voice() {
    let content = read_voice_shard();
    assert!(
        content.contains("@mirror/spectral/voice") || content.contains("@mirror/spectral"),
        "T7: species MUST cite `@mirror/spectral/voice` (peer-as-agent with named authority over sections; the orchestra altitude ancestor). `@song/voice` adds the *when* to `@mirror/spectral/voice`'s *what/who*."
    );
}

#[test]
fn t08_voice_composes_with_epistemologic_math_music_voice() {
    let content = read_voice_shard();
    assert!(
        content.contains("@epistemologic/math/music/voice") || content.contains("music/voice"),
        "T8: species MUST cite `@epistemologic/math/music/voice` (audible-altitude voice-leading constraints; forward-promised per spec §2). `@song/voice` composes at temporal altitude with this audible-altitude ancestor."
    );
}

// === T9: #S3 focus @ temporal specialisation grounding ===

#[test]
fn t09_voice_grounds_s3_focus_at_temporal_specialisation() {
    let content = read_voice_shard();
    // Voice specializes `focus @ temporal` per family-root prism block:
    // "focus @ temporal = attend to ONE voice / one line at a time (soloing;
    // solistic attention)". Species narrative must ground this as its principal
    // contribution toward #S3 promotion.
    let has_focus_grounding = content.contains("soloing")
        || content.contains("solistic")
        || content.contains("solistic attention")
        || content.contains("focus @ temporal")
        || content.contains("attend to one voice")
        || content.contains("attend to ONE voice")
        || content.contains("attend to a single voice");
    assert!(
        has_focus_grounding,
        "T9: species narrative must ground `focus @ temporal` specialisation — cite `soloing`, `solistic attention`, `attend to one voice`, or `focus @ temporal` witness. This is voice's principal contribution toward #S3 promotion after TICK 3 lands (three species discharging all five ops at temporal altitude)."
    );
}

// === T10: psychohistorical Narcissus-pole naming (spec §8.2) ===

#[test]
fn t10_voice_narrative_names_glue_work_narcissus_pole() {
    let content = read_voice_shard();
    assert!(
        content.contains("GLUE WORK") || content.contains("glue work") || content.contains("glue-work"),
        "T10: species narrative MUST cite GLUE WORK Narcissus-pole naming per spec §8.2 psychohistorical failure-mode mapping. `voice_line_valid` FAIL = GLUE WORK: contrapuntal work invisibly holds the harmony but is not attributed; systematic-invisibility-of-stepwise-motion."
    );
}

// === T11-T12: voice-leading vocabulary + counterpoint prior-art ===

#[test]
fn t11_voice_cites_voice_leading_vocabulary() {
    let content = read_voice_shard();
    let has_voice_leading = content.contains("voice-leading")
        || content.contains("voice leading")
        || content.contains("stepwise")
        || content.contains("stepwise motion")
        || content.contains("intentional leap");
    assert!(
        has_voice_leading,
        "T11: species narrative MUST cite voice-leading vocabulary — `voice-leading`, `stepwise motion`, or `intentional leap`. This is the substrate-decl language for the voice_line_valid predicate's discharge structure."
    );
}

#[test]
fn t12_voice_cites_counterpoint_prior_art() {
    let content = read_voice_shard();
    let has_counterpoint_anchor = content.contains("counterpoint")
        || content.contains("Palestrina")
        || content.contains("Fux")
        || content.contains("Gradus ad Parnassum")
        || content.contains("Bach");
    assert!(
        has_counterpoint_anchor,
        "T12: species narrative MUST cite counterpoint prior-art — `counterpoint`, Palestrina (16th c. species counterpoint), Fux (Gradus ad Parnassum 1725), or Bach (invention/fugue voice-leading). Voice-leading discipline traces to species counterpoint tradition."
    );
}

// === T13: multi-voice temporal coordination anchor (Ligeti / Boulez / mirror-spectral) ===

#[test]
fn t13_voice_cites_pack_as_orchestra_ancestor() {
    let content = read_voice_shard();
    // Pack-as-orchestra per docs/specs/mirror-spectral.md; Ligeti micropolyphony
    // per spec §3 ("multi-agent temporal coordination under a shared harmonic
    // frame"). Either witness satisfies: voice as species-altitude Pack-as-
    // orchestra binding.
    let has_orchestra_anchor = content.contains("Pack-as-orchestra")
        || content.contains("Pack as orchestra")
        || content.contains("Pack IS the orchestra")
        || content.contains("orchestra")
        || content.contains("Ligeti")
        || content.contains("micropolyphony")
        || content.contains("polyphony");
    assert!(
        has_orchestra_anchor,
        "T13: species narrative MUST cite Pack-as-orchestra anchor — `Pack-as-orchestra`, `orchestra`, `Ligeti` (micropolyphony), or `polyphony`. This grounds voice as temporal-altitude specialisation of the Pack-as-orchestra recognition (docs/specs/mirror-spectral.md §3.2)."
    );
}

// === T14: species declares advance action at species altitude ===

#[test]
fn t14_voice_declares_advance_or_species_action() {
    let content = read_voice_shard();
    // Per family-root §"Species roster" (line 86-90): @song/voice actions include
    // `advance` and `settle` at species altitude. At minimum the species should
    // declare an action — either `advance` (temporal-motion consistent with
    // progression's advance) or a species-specific voice action.
    let has_species_action = content.contains("advance(v")
        || content.contains("advance(voice")
        || content.contains("settle(v")
        || content.contains("settle(voice")
        || content.contains("enter(v")
        || content.contains("song_voice(v")
        || content.contains("solo(v");
    assert!(
        has_species_action,
        "T14: species MUST declare a species-altitude action operating on `song_voice` — `advance(v: ...)`, `settle(v: ...)`, `enter(v: ...)`, `song_voice(v: ...)`, or `solo(v: ...)`. Per family-root song.mirror species-roster section (voice actions include advance + settle at species altitude)."
    );
}

// === T15: #53 bilateral pattern + composed-bilateral closure signal ===

#[test]
fn t15_voice_carries_bilateral_pattern_and_song_settles_closure() {
    let content = read_voice_shard();
    // Voice's sub-predicate `voice_line_valid` is the THIRD (and closing)
    // sub-predicate of `song_settles`. Species narrative must acknowledge
    // either the #53 bilateral pattern, the composed-bilateral discharge
    // relationship to song_settles, or the Splinter/Narcissus polarity that
    // #57 alignment-as-boundary-mathematics makes visible.
    let has_bilateral_grounding = content.contains("#53")
        || content.contains("song_settles")
        || content.contains("composed bilateral")
        || content.contains("composed-bilateral")
        || content.contains("Splinter-pole")
        || content.contains("Narcissus-pole");
    assert!(
        has_bilateral_grounding,
        "T15: species narrative MUST ground the sub-predicate discharge relationship into `song_settles` — cite `#53`, `song_settles`, `composed bilateral`, `Splinter-pole`, or `Narcissus-pole`. This species discharges the THIRD sub-predicate; after TICK 3 lands all three song_settles sub-predicates are discharged at species altitude."
    );
}
