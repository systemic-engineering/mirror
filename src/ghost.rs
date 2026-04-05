//! Ghost echo — reference vector and coherence distance.
//!
//! A `GhostEcho` is the mean feature vector of a corpus of grammars. It serves
//! as a centroid for measuring how far any single grammar deviates from the
//! corpus's "center of gravity."
//!
//! ## Coherence
//!
//! Coherence is the inverse of distance from the echo: a grammar that matches
//! the corpus structure closely has a high coherence score. Extractive or
//! anomalous grammars sit far from the echo and score low.

use crate::features::{Features, FEATURE_DIM};

// ---------------------------------------------------------------------------
// GhostEcho
// ---------------------------------------------------------------------------

/// Mean feature vector of a corpus, used as a reference centroid.
#[derive(Debug, Clone, PartialEq)]
pub struct GhostEcho {
    pub reference: Features,
}

impl GhostEcho {
    // ---

    /// Build a `GhostEcho` from a corpus by computing the component-wise mean.
    ///
    /// If `corpus` is empty, returns a zero-vector reference.
    pub fn from_features(corpus: &[Features]) -> Self {
        if corpus.is_empty() {
            return Self {
                reference: [0.0; FEATURE_DIM],
            };
        }

        let mut sum = [0.0f64; FEATURE_DIM];
        for features in corpus {
            for (i, &v) in features.iter().enumerate() {
                sum[i] += v;
            }
        }

        let n = corpus.len() as f64;
        let mut reference = [0.0f64; FEATURE_DIM];
        for (i, r) in reference.iter_mut().enumerate() {
            *r = sum[i] / n;
        }

        Self { reference }
    }

    // ---

    /// Euclidean distance between the reference vector and `features`.
    pub fn coherence_distance(&self, features: &Features) -> f64 {
        let sum_sq: f64 = self
            .reference
            .iter()
            .zip(features.iter())
            .map(|(r, f)| {
                let diff = r - f;
                diff * diff
            })
            .sum();
        sum_sq.sqrt()
    }

    // ---

    /// Coherence score: `exp(-dist / scale)`.
    ///
    /// Returns 1.0 when `features` equals the reference, decaying toward 0.0
    /// as distance grows. `scale` controls the decay rate.
    pub fn coherence_score(&self, features: &Features, scale: f64) -> f64 {
        let dist = self.coherence_distance(features);
        (-dist / scale).exp()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::extract_from_source;

    // 1: Single feature vector — echo equals it, distance is 0.
    #[test]
    fn ghost_echo_from_single_grammar_is_identity() {
        let source = "grammar @a { type x = one | two | three }";
        let f = extract_from_source(source);
        let echo = GhostEcho::from_features(&[f]);
        assert_eq!(
            echo.reference, f,
            "echo of single grammar should equal that grammar's features"
        );
        let dist = echo.coherence_distance(&f);
        assert!(
            dist.abs() < 1e-12,
            "distance from echo to itself should be 0, got {}",
            dist
        );
    }

    // 2: Build echo from 3 legitimate grammars; verify extractive grammars
    //    are farther from the echo on average than the legitimate ones.
    //
    //    Legitimate: rich vocabulary, multiple variants, parameterized types.
    //    Extractive proxy: many single-variant types with no parameterization —
    //    structurally impoverished grammars that sit far from the legitimate centroid
    //    in feature space (low variant_count_norm, low ref_ratio, different density).
    #[test]
    fn extractive_grammars_farther_from_echo() {
        // Legitimate grammars — rich structure, varied types and variants
        let legitimate = [
            "grammar @color {\n  type hue = red | green | blue\n  type shade = light | dark | mid\n  type palette = warm | cool | neutral\n}",
            "grammar @signal {\n  type status = ok | error | pending | timeout\n  type level = low | medium | high | critical\n  type alert = clear | warn(level) | page(level)\n}",
            "grammar @shape {\n  type form = circle | square | triangle | hexagon\n  type size = small | medium | large\n  type layout = grid | flow | stack\n  type item = plain | styled(size)\n}",
        ];

        let legit_features: Vec<Features> =
            legitimate.iter().map(|s| extract_from_source(s)).collect();

        // Sanity: ensure legitimate grammars produced non-zero features
        for (i, f) in legit_features.iter().enumerate() {
            assert!(
                f.iter().any(|&v| v > 0.0),
                "legitimate grammar {} produced all-zero features",
                i
            );
        }

        let echo = GhostEcho::from_features(&legit_features);

        // Extractive proxy grammars — structurally impoverished:
        // many single-variant types, no parameterization, no namespace diversity.
        // High type count, zero ref_ratio, 1:1 variant-to-type ratio.
        let extractive = [
            // Twenty single-variant types — high type_count_norm, zero ref_ratio
            "grammar @flat {\n  type a = only_a\n  type b = only_b\n  type c = only_c\n  type d = only_d\n  type e = only_e\n  type f = only_f\n  type g = only_g\n  type h = only_h\n  type i = only_i\n  type j = only_j\n  type k = only_k\n  type l = only_l\n  type m = only_m\n  type n = only_n\n  type o = only_o\n  type p = only_p\n  type q = only_q\n  type r = only_r\n  type s = only_s\n  type t = only_t\n}",
            // Fifteen single-variant types — similar impoverished profile
            "grammar @mono {\n  type aa = val_aa\n  type bb = val_bb\n  type cc = val_cc\n  type dd = val_dd\n  type ee = val_ee\n  type ff = val_ff\n  type gg = val_gg\n  type hh = val_hh\n  type ii = val_ii\n  type jj = val_jj\n  type kk = val_kk\n  type ll = val_ll\n  type mm = val_mm\n  type nn = val_nn\n  type oo = val_oo\n}",
        ];

        let extractive_features: Vec<Features> =
            extractive.iter().map(|s| extract_from_source(s)).collect();

        // Sanity: ensure extractive grammars also produced non-zero features
        for (i, f) in extractive_features.iter().enumerate() {
            assert!(
                f.iter().any(|&v| v > 0.0),
                "extractive grammar {} produced all-zero features — grammar failed to parse",
                i
            );
        }

        let avg_legit_dist: f64 = legit_features
            .iter()
            .map(|f| echo.coherence_distance(f))
            .sum::<f64>()
            / legit_features.len() as f64;

        let avg_extractive_dist: f64 = extractive_features
            .iter()
            .map(|f| echo.coherence_distance(f))
            .sum::<f64>()
            / extractive_features.len() as f64;

        assert!(
            avg_extractive_dist > avg_legit_dist,
            "extractive avg dist ({:.4}) should exceed legitimate avg dist ({:.4})",
            avg_extractive_dist,
            avg_legit_dist
        );
    }

    // 3: Coherence score decays with distance.
    //    Reference at [0.5; 16]. Close/mid/far vectors, verify score ordering.
    #[test]
    fn coherence_score_decays_with_distance() {
        let reference: Features = [0.5; FEATURE_DIM];
        let echo = GhostEcho { reference };

        // Close: slight nudge away from reference
        let close: Features = [0.55; FEATURE_DIM];
        // Mid: moderate deviation
        let mid: Features = [0.7; FEATURE_DIM];
        // Far: large deviation toward 0.0
        let far: Features = [0.0; FEATURE_DIM];

        let scale = 1.0;
        let score_close = echo.coherence_score(&close, scale);
        let score_mid = echo.coherence_score(&mid, scale);
        let score_far = echo.coherence_score(&far, scale);

        assert!(
            score_close > score_mid,
            "close score ({:.4}) should exceed mid score ({:.4})",
            score_close,
            score_mid
        );
        assert!(
            score_mid > score_far,
            "mid score ({:.4}) should exceed far score ({:.4})",
            score_mid,
            score_far
        );
        assert!(
            score_close <= 1.0 && score_close > 0.0,
            "close score out of (0, 1]: {}",
            score_close
        );
    }
}
