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

/// Load a training corpus from (source, optic_label) pairs.
///
/// Features are extracted from each source via `features::extract_from_source`,
/// placed into the first 16 dims of the 32-dim classifier input.
/// The remaining 16 dims are zero-padded.
pub fn examples_from_sources(_pairs: &[(&str, usize)]) -> Vec<Example> {
    todo!("not yet implemented")
}

/// Merge legitimate and extractive corpora into a single training set.
///
/// Concatenates both slices preserving their order.
pub fn merge_corpora(_legitimate: &[Example], _extractive: &[Example]) -> Vec<Example> {
    todo!("not yet implemented")
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
}
