//! Two-corpus training data loader.
//!
//! Converts raw grammar sources into classifier `Example`s, and merges
//! legitimate and extractive corpora into a single training set.
//!
//! Feature extraction uses the 16-dim spectral pipeline from `features`,
//! placed into the first 16 dims of the 32-dim classifier input. The
//! remaining 16 dims are zero-padded.

use crate::classifier::{Example, INPUT_DIM};
use crate::features::{self, FEATURE_DIM};

// ---------------------------------------------------------------------------
// Settle fixture sources (embedded at compile time)
// ---------------------------------------------------------------------------
//
// Only the three settle fixtures whose filenames appear in training_data.json
// are included in the legitimate corpus. The other nine entries in
// training_data.json reference files that do not exist in fixtures/settle/
// and are skipped per spec.
//
// Note: the settle .conv files use a domain-specific notation (fold/prism/
// traversal/lens/iso keywords) rather than the grammar @name { ... } format
// that the parser expects. extract_from_source returns [0.0; 16] for these
// files. The corpus is still useful as labeled examples; future work can
// extend the feature extractor to handle this format.

/// optic 9 — optic_iso_settle.conv (exists in fixtures/settle/)
const SETTLE_ISO: (&str, usize) = (include_str!("../fixtures/settle/optic_iso_settle.conv"), 9);

/// optic 10 — optic_escalate.conv (exists in fixtures/settle/)
const SETTLE_ESCALATE: (&str, usize) = (include_str!("../fixtures/settle/optic_escalate.conv"), 10);

/// optic 11 — optic_noop.conv (exists in fixtures/settle/)
const SETTLE_NOOP: (&str, usize) = (include_str!("../fixtures/settle/optic_noop.conv"), 11);

/// Settle entries that exist on disk, ordered by optic label.
const SETTLE_CORPUS: &[(&str, usize)] = &[SETTLE_ISO, SETTLE_ESCALATE, SETTLE_NOOP];

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Load a training corpus from (source, optic_label) pairs.
///
/// Features are extracted from each source via `features::extract_from_source`,
/// placed into the first 16 dims of the 32-dim classifier input.
/// The remaining 16 dims are zero-padded.
pub fn examples_from_sources(pairs: &[(&str, usize)]) -> Vec<Example> {
    pairs
        .iter()
        .map(|&(source, label)| {
            let spectral = features::extract_from_source(source);
            let mut input = [0.0f64; INPUT_DIM];
            input[..FEATURE_DIM].copy_from_slice(&spectral);
            Example {
                features: input,
                label,
            }
        })
        .collect()
}

/// Merge legitimate and extractive corpora into a single training set.
///
/// Concatenates both slices preserving their order.
pub fn merge_corpora(legitimate: &[Example], extractive: &[Example]) -> Vec<Example> {
    let mut merged = Vec::with_capacity(legitimate.len() + extractive.len());
    for ex in legitimate {
        merged.push(Example {
            features: ex.features,
            label: ex.label,
        });
    }
    for ex in extractive {
        merged.push(Example {
            features: ex.features,
            label: ex.label,
        });
    }
    merged
}

/// Load the legitimate corpus from settle fixtures.
///
/// Reads .conv source files and their optic labels from training_data.json.
/// Returns `Example` structs with 16-dim features in 32-dim input.
///
/// Entries in training_data.json that reference non-existent files are skipped.
/// Currently 3 of 12 entries resolve to files present in `fixtures/settle/`:
/// optic_iso_settle.conv (9), optic_escalate.conv (10), optic_noop.conv (11).
pub fn legitimate_corpus() -> Vec<Example> {
    examples_from_sources(SETTLE_CORPUS)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_GRAMMAR: &str = "grammar @test {\n  type color = red | blue | green\n}\n";

    #[test]
    fn merged_corpus_has_both_types() {
        let legit = examples_from_sources(&[(SIMPLE_GRAMMAR, 0)]);
        let extractive = examples_from_sources(&[(SIMPLE_GRAMMAR, 1)]);
        let merged = merge_corpora(&legit, &extractive);
        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].label, 0);
        assert_eq!(merged[1].label, 1);
    }

    #[test]
    fn examples_from_sources_extracts_features() {
        let examples = examples_from_sources(&[(SIMPLE_GRAMMAR, 0)]);
        assert_eq!(examples.len(), 1);
        let features = &examples[0].features;
        // First 16 dims (spectral features) must contain at least one non-zero value.
        let nonzero = features[..FEATURE_DIM].iter().any(|&v| v != 0.0);
        assert!(
            nonzero,
            "features[0..16] are all zero — spectral extraction failed"
        );
    }

    #[test]
    fn examples_from_sources_zero_pads() {
        let examples = examples_from_sources(&[(SIMPLE_GRAMMAR, 0)]);
        assert_eq!(examples.len(), 1);
        let features = &examples[0].features;
        // Dims 16..32 must all be zero.
        for (i, &v) in features[FEATURE_DIM..].iter().enumerate() {
            assert_eq!(
                v,
                0.0,
                "features[{}] = {} — expected zero padding",
                FEATURE_DIM + i,
                v
            );
        }
    }

    /// Load all .conv files from fixtures/settle/ via include_str!, extract
    /// features, and print values. Verifies that feature extraction runs
    /// without panic and produces a valid 16-dim vector for each file.
    ///
    /// Note: settle fixtures use a domain DSL notation (fold/prism/traversal/
    /// lens/iso), not the grammar @name { ... } format. The spectral extractor
    /// returns [0.0; 16] for these files — expected until the extractor is
    /// extended to handle this format.
    #[test]
    fn settle_fixtures_produce_16_features() {
        let fixtures: &[(&str, &str)] = &[
            (
                "composition",
                include_str!("../fixtures/settle/composition.conv"),
            ),
            ("dedup", include_str!("../fixtures/settle/dedup.conv")),
            ("emotion", include_str!("../fixtures/settle/emotion.conv")),
            ("identity", include_str!("../fixtures/settle/identity.conv")),
            ("layered", include_str!("../fixtures/settle/layered.conv")),
            (
                "messy_types",
                include_str!("../fixtures/settle/messy_types.conv"),
            ),
            (
                "optic_escalate",
                include_str!("../fixtures/settle/optic_escalate.conv"),
            ),
            (
                "optic_iso_settle",
                include_str!("../fixtures/settle/optic_iso_settle.conv"),
            ),
            (
                "optic_noop",
                include_str!("../fixtures/settle/optic_noop.conv"),
            ),
            (
                "redundant_layers",
                include_str!("../fixtures/settle/redundant_layers.conv"),
            ),
            ("sort", include_str!("../fixtures/settle/sort.conv")),
            ("tension", include_str!("../fixtures/settle/tension.conv")),
        ];

        for (name, source) in fixtures {
            let features = features::extract_from_source(source);

            // Feature vector must be exactly FEATURE_DIM (16) dims.
            assert_eq!(
                features.len(),
                FEATURE_DIM,
                "fixture '{}' produced wrong feature dim",
                name
            );

            // All values must be in [0, 1].
            for (i, &v) in features.iter().enumerate() {
                assert!(
                    (0.0..=1.0).contains(&v),
                    "fixture '{}': feature[{}] = {} out of [0, 1]",
                    name,
                    i,
                    v
                );
            }

            println!(
                "{}: {:?}",
                name,
                features
                    .iter()
                    .map(|v| format!("{:.3}", v))
                    .collect::<Vec<_>>()
            );
        }
    }

    /// legitimate_corpus() returns a non-empty Vec<Example>.
    #[test]
    fn legitimate_corpus_is_non_empty() {
        let corpus = legitimate_corpus();
        assert!(
            !corpus.is_empty(),
            "legitimate_corpus() returned empty — check SETTLE_CORPUS mapping"
        );
    }

    /// legitimate_corpus() returns exactly 3 examples (the 3 settle fixtures
    /// whose filenames exist in both fixtures/settle/ and training_data.json).
    #[test]
    fn legitimate_corpus_has_correct_count() {
        let corpus = legitimate_corpus();
        assert_eq!(
            corpus.len(),
            3,
            "expected 3 settle fixtures with JSON entries, got {}",
            corpus.len()
        );
    }

    /// legitimate_corpus() examples carry the correct optic labels (9, 10, 11).
    #[test]
    fn legitimate_corpus_labels_match_training_data_json() {
        let corpus = legitimate_corpus();
        let labels: Vec<usize> = corpus.iter().map(|e| e.label).collect();
        assert_eq!(labels, vec![9, 10, 11]);
    }

    /// legitimate_corpus() examples have 32-dim input with zero padding in dims 16..32.
    #[test]
    fn legitimate_corpus_zero_pads_upper_dims() {
        let corpus = legitimate_corpus();
        for ex in &corpus {
            for (i, &v) in ex.features[FEATURE_DIM..].iter().enumerate() {
                assert_eq!(
                    v,
                    0.0,
                    "example label={}: features[{}] = {} — expected zero padding",
                    ex.label,
                    FEATURE_DIM + i,
                    v
                );
            }
        }
    }
}
