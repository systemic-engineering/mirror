//! Tension classifier — the only learned component.
//!
//! 2,892 parameters. Two matmuls. A sigmoid. A softmax.
//! Spectral features in, optic category out.
//!
//! The classifier doesn't know domains. It sees eigenvalue shapes.
//! It outputs which of the 12 optic categories to apply.
//!
//! At f16: 5,784 bytes. Fits in an HTTP header.

// ---------------------------------------------------------------------------
// Optic categories — the 12 output classes
// ---------------------------------------------------------------------------

/// The 12 optic categories the classifier selects from.
/// Each maps to a specific Prism operation + configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Optic {
    /// fold: decompose, observe structure
    FoldDecompose = 0,
    /// fold: accumulate into namespace
    FoldAccumulate = 1,
    /// prism: project at current precision
    PrismProject = 2,
    /// prism: narrow precision, discard
    PrismNarrow = 3,
    /// traversal: walk breadth-first
    TraversalBreadth = 4,
    /// traversal: walk depth-first
    TraversalDepth = 5,
    /// lens: focus and transform
    LensTransform = 6,
    /// lens: focus and merge
    LensMerge = 7,
    /// lens: focus and split
    LensSplit = 8,
    /// iso: settle (convergence detected)
    IsoSettle = 9,
    /// escalate: tension unresolvable, witness needed
    Escalate = 10,
    /// noop: no tension detected, pass through
    Noop = 11,
}

impl Optic {
    pub fn from_index(i: usize) -> Self {
        match i {
            0 => Optic::FoldDecompose,
            1 => Optic::FoldAccumulate,
            2 => Optic::PrismProject,
            3 => Optic::PrismNarrow,
            4 => Optic::TraversalBreadth,
            5 => Optic::TraversalDepth,
            6 => Optic::LensTransform,
            7 => Optic::LensMerge,
            8 => Optic::LensSplit,
            9 => Optic::IsoSettle,
            10 => Optic::Escalate,
            11 => Optic::Noop,
            _ => Optic::Noop,
        }
    }

    pub fn count() -> usize {
        12
    }
}

// ---------------------------------------------------------------------------
// Weights — the 2,892 parameters
// ---------------------------------------------------------------------------

/// The classifier weights. Two layers.
///
/// W1: 32 × 64 = 2,048 parameters
/// b1: 64 parameters
/// W2: 64 × 12 = 768 parameters
/// b2: 12 parameters
/// Total: 2,892 parameters
pub struct Weights {
    pub w1: Vec<f64>, // 32 × 64 = 2048, row-major
    pub b1: Vec<f64>, // 64
    pub w2: Vec<f64>, // 64 × 12 = 768, row-major
    pub b2: Vec<f64>, // 12
}

/// Input dimension: spectral features.
pub const INPUT_DIM: usize = 32;
/// Hidden dimension.
pub const HIDDEN_DIM: usize = 64;
/// Output dimension: optic categories.
pub const OUTPUT_DIM: usize = 12;
/// Total parameter count.
pub const PARAM_COUNT: usize = INPUT_DIM * HIDDEN_DIM + HIDDEN_DIM + HIDDEN_DIM * OUTPUT_DIM + OUTPUT_DIM;

impl Weights {
    /// Initialize with zeros. The untrained classifier.
    pub fn zeros() -> Self {
        Weights {
            w1: vec![0.0; INPUT_DIM * HIDDEN_DIM],
            b1: vec![0.0; HIDDEN_DIM],
            w2: vec![0.0; HIDDEN_DIM * OUTPUT_DIM],
            b2: vec![0.0; OUTPUT_DIM],
        }
    }

    /// Initialize with small random values (Xavier-ish).
    pub fn random(seed: u64) -> Self {
        let mut rng = SimpleRng(seed);
        let scale1 = (2.0 / (INPUT_DIM + HIDDEN_DIM) as f64).sqrt();
        let scale2 = (2.0 / (HIDDEN_DIM + OUTPUT_DIM) as f64).sqrt();

        Weights {
            w1: (0..INPUT_DIM * HIDDEN_DIM).map(|_| rng.next_normal() * scale1).collect(),
            b1: vec![0.0; HIDDEN_DIM],
            w2: (0..HIDDEN_DIM * OUTPUT_DIM).map(|_| rng.next_normal() * scale2).collect(),
            b2: vec![0.0; OUTPUT_DIM],
        }
    }

    /// Load from raw bytes (f64 little-endian, packed).
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != PARAM_COUNT * 8 {
            return None;
        }
        let mut offset = 0;
        let read = |buf: &[u8], off: &mut usize, count: usize| -> Vec<f64> {
            let vals: Vec<f64> = (0..count)
                .map(|i| {
                    let start = *off + i * 8;
                    f64::from_le_bytes(buf[start..start + 8].try_into().unwrap())
                })
                .collect();
            *off += count * 8;
            vals
        };
        let w1 = read(bytes, &mut offset, INPUT_DIM * HIDDEN_DIM);
        let b1 = read(bytes, &mut offset, HIDDEN_DIM);
        let w2 = read(bytes, &mut offset, HIDDEN_DIM * OUTPUT_DIM);
        let b2 = read(bytes, &mut offset, OUTPUT_DIM);
        Some(Weights { w1, b1, w2, b2 })
    }

    /// Serialize to raw bytes (f64 little-endian, packed).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(PARAM_COUNT * 8);
        for &v in self.w1.iter().chain(self.b1.iter()).chain(self.w2.iter()).chain(self.b2.iter()) {
            bytes.extend_from_slice(&v.to_le_bytes());
        }
        bytes
    }

    /// Total number of parameters.
    pub fn param_count(&self) -> usize {
        self.w1.len() + self.b1.len() + self.w2.len() + self.b2.len()
    }
}

// ---------------------------------------------------------------------------
// Forward pass
// ---------------------------------------------------------------------------

/// The forward pass. spectral_features → optic category.
///
/// hidden = sigmoid(W1 · input + b1)
/// output = softmax(W2 · hidden + b2)
///
/// Returns (predicted_optic, confidence, all_probabilities).
pub fn classify(weights: &Weights, spectral_features: &[f64; INPUT_DIM]) -> (Optic, f64, [f64; OUTPUT_DIM]) {
    // Layer 1: hidden = sigmoid(W1 · input + b1)
    let mut hidden = [0.0f64; HIDDEN_DIM];
    for i in 0..HIDDEN_DIM {
        let mut sum = weights.b1[i];
        for j in 0..INPUT_DIM {
            sum += weights.w1[i * INPUT_DIM + j] * spectral_features[j];
        }
        hidden[i] = sigmoid(sum);
    }

    // Layer 2: logits = W2 · hidden + b2
    let mut logits = [0.0f64; OUTPUT_DIM];
    for i in 0..OUTPUT_DIM {
        let mut sum = weights.b2[i];
        for j in 0..HIDDEN_DIM {
            sum += weights.w2[i * HIDDEN_DIM + j] * hidden[j];
        }
        logits[i] = sum;
    }

    // Softmax
    let probs = softmax(&logits);

    // Argmax
    let mut best_idx = 0;
    let mut best_prob = probs[0];
    for i in 1..OUTPUT_DIM {
        if probs[i] > best_prob {
            best_prob = probs[i];
            best_idx = i;
        }
    }

    (Optic::from_index(best_idx), best_prob, probs)
}

// ---------------------------------------------------------------------------
// Activation functions
// ---------------------------------------------------------------------------

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn softmax(logits: &[f64; OUTPUT_DIM]) -> [f64; OUTPUT_DIM] {
    let max = logits.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let mut exps = [0.0f64; OUTPUT_DIM];
    let mut sum = 0.0;
    for i in 0..OUTPUT_DIM {
        exps[i] = (logits[i] - max).exp();
        sum += exps[i];
    }
    for i in 0..OUTPUT_DIM {
        exps[i] /= sum;
    }
    exps
}

// ---------------------------------------------------------------------------
// Simple RNG (for weight initialization, no external dep)
// ---------------------------------------------------------------------------

struct SimpleRng(u64);

impl SimpleRng {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0
    }

    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Box-Muller transform for normal distribution.
    fn next_normal(&mut self) -> f64 {
        let u1 = self.next_f64().max(1e-10);
        let u2 = self.next_f64();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn param_count_correct() {
        assert_eq!(PARAM_COUNT, 2892);
        let w = Weights::zeros();
        assert_eq!(w.param_count(), 2892);
    }

    #[test]
    fn zeros_classify_uniform() {
        let w = Weights::zeros();
        let input = [0.0; INPUT_DIM];
        let (_, _, probs) = classify(&w, &input);
        // All zeros → sigmoid(0) = 0.5 for all hidden → uniform softmax
        for &p in &probs {
            assert!((p - 1.0 / OUTPUT_DIM as f64).abs() < 1e-6);
        }
    }

    #[test]
    fn random_weights_classify() {
        let w = Weights::random(42);
        let input = [1.0; INPUT_DIM];
        let (optic, confidence, probs) = classify(&w, &input);
        // Should produce a valid optic
        assert!(confidence > 0.0);
        assert!(confidence <= 1.0);
        // Probabilities sum to 1
        let sum: f64 = probs.iter().sum();
        assert!((sum - 1.0).abs() < 1e-6);
    }

    #[test]
    fn weights_roundtrip_bytes() {
        let w = Weights::random(123);
        let bytes = w.to_bytes();
        assert_eq!(bytes.len(), PARAM_COUNT * 8);
        let w2 = Weights::from_bytes(&bytes).unwrap();
        assert_eq!(w.w1, w2.w1);
        assert_eq!(w.b1, w2.b1);
        assert_eq!(w.w2, w2.w2);
        assert_eq!(w.b2, w2.b2);
    }

    #[test]
    fn weights_from_bytes_wrong_size() {
        assert!(Weights::from_bytes(&[0u8; 100]).is_none());
    }

    #[test]
    fn sigmoid_values() {
        assert!((sigmoid(0.0) - 0.5).abs() < 1e-10);
        assert!(sigmoid(10.0) > 0.999);
        assert!(sigmoid(-10.0) < 0.001);
    }

    #[test]
    fn softmax_sums_to_one() {
        let logits = [1.0, 2.0, 3.0, 0.5, -1.0, 0.0, 1.5, -0.5, 2.5, 0.1, -2.0, 1.0];
        let probs = softmax(&logits);
        let sum: f64 = probs.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10);
    }

    #[test]
    fn softmax_max_has_highest_prob() {
        let logits = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0];
        let probs = softmax(&logits);
        assert!(probs[9] > 0.99);
    }

    #[test]
    fn optic_from_index_all_valid() {
        for i in 0..12 {
            let optic = Optic::from_index(i);
            assert_eq!(optic as u8, i as u8);
        }
    }

    #[test]
    fn optic_out_of_range_is_noop() {
        assert_eq!(Optic::from_index(99), Optic::Noop);
    }

    #[test]
    fn deterministic_classification() {
        let w = Weights::random(42);
        let input = [0.5; INPUT_DIM];
        let (optic1, conf1, _) = classify(&w, &input);
        let (optic2, conf2, _) = classify(&w, &input);
        assert_eq!(optic1, optic2);
        assert_eq!(conf1, conf2);
    }

    #[test]
    fn weight_size_at_f64() {
        let w = Weights::random(1);
        let bytes = w.to_bytes();
        assert_eq!(bytes.len(), 23136); // 2892 * 8
    }

    #[test]
    fn trained_weights_load_and_classify() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("abyss.weights");
        if !path.exists() {
            return; // skip if weights not trained yet
        }
        let bytes = std::fs::read(&path).unwrap();
        let w = Weights::from_bytes(&bytes).unwrap();
        assert_eq!(w.param_count(), PARAM_COUNT);

        // Classify a "lens" commit: lots of insertions, .rs files, 🟢 marker
        let mut input = [0.0; INPUT_DIM];
        input[0] = 0.15; // files_changed / 20
        input[1] = 0.3;  // insertions / 1000
        input[2] = 0.05; // deletions / 1000
        input[3] = 0.9;  // confidence
        input[4] = 0.85; // ins ratio
        input[6] = 1.0;  // .rs file
        input[16] = 1.0; // 🟢
        input[29] = 1.0; // conversation repo

        let (optic, confidence, _) = classify(&w, &input);
        // Should classify as some optic with reasonable confidence
        assert!(confidence > 0.1, "confidence should be non-trivial: {}", confidence);
        // The trained model should not return Noop for a clearly active commit
        assert_ne!(optic, Optic::Noop, "active commit should not be Noop");
    }
}
