//! Gestalt — the reader's portrait.
//!
//! Not a trace of one encounter. The accumulated model of who
//! the reader is. Every other domain reads from it.
//! Every refract writes to it. Through Reflection only.
//! Zero parameters. Pure state. The gestalt IS the reader.

use std::collections::BTreeMap;

// ---------------------------------------------------------------------------
// Supporting types
// ---------------------------------------------------------------------------

/// How the reader moves through a problem space.
#[derive(Debug, Clone, PartialEq)]
pub enum FocusPattern {
    DepthFirst,
    BreadthFirst,
    Mixed,
}

/// Which direction the reader tends to zoom when exploring.
#[derive(Debug, Clone, PartialEq)]
pub enum ZoomDirection {
    Deeper,
    Simpler,
    Connected,
}

/// A signature of how the reader allocates attention.
#[derive(Debug, Clone)]
pub struct AttentionSignature {
    pub focus_pattern: FocusPattern,
    pub zoom_preference: Vec<ZoomDirection>,
    pub split_frequency: f64,
    pub avg_fork_depth: f64,
}

/// Lifecycle state of a held tension.
#[derive(Debug, Clone, PartialEq)]
pub enum TensionState {
    Held,
    Settling,
    Settled,
}

/// A tension the reader is holding — a question, contradiction, or
/// unresolved frame that hasn't collapsed yet.
#[derive(Debug, Clone)]
pub struct HeldTension {
    pub description: String,
    pub loss: f64,
    pub state: TensionState,
}

// ---------------------------------------------------------------------------
// GestaltProfile
// ---------------------------------------------------------------------------

/// The accumulated portrait of a reader.
///
/// Updated by every encounter. The loss decays toward zero as the model
/// converges. Forkable for independent exploration; mergeable by inverse
/// loss weighting.
#[derive(Debug, Clone)]
pub struct GestaltProfile {
    pub reader: String,
    pub updated: String,
    pub encounters: u64,
    pub loss: f64,
    pub eigenvalues: Vec<f64>,
    pub concept_loss: BTreeMap<String, f64>,
    pub attention: AttentionSignature,
    pub tensions: Vec<HeldTension>,
    pub crystals: Vec<String>,
}

impl GestaltProfile {
    // ---

    /// Create a new profile for `reader`.
    ///
    /// Starts with maximum loss (1.0) and zero encounters — the model knows
    /// nothing yet.
    pub fn new(reader: &str) -> Self {
        Self {
            reader: reader.to_string(),
            updated: String::new(),
            encounters: 0,
            loss: 1.0,
            eigenvalues: Vec::new(),
            concept_loss: BTreeMap::new(),
            attention: AttentionSignature {
                focus_pattern: FocusPattern::Mixed,
                zoom_preference: Vec::new(),
                split_frequency: 0.0,
                avg_fork_depth: 0.0,
            },
            tensions: Vec::new(),
            crystals: Vec::new(),
        }
    }

    // ---

    /// Record an encounter, updating global loss via EMA (alpha=0.3) and
    /// appending the crystal OID.
    pub fn record_encounter(&mut self, crystal_oid: &str, loss: f64) {
        const ALPHA: f64 = 0.3;
        self.loss = ALPHA * loss + (1.0 - ALPHA) * self.loss;
        self.encounters += 1;
        self.crystals.push(crystal_oid.to_string());
    }

    // ---

    /// Update the EMA loss for a named concept (alpha=0.3).
    ///
    /// If the concept hasn't been seen before, its initial loss is 1.0.
    pub fn update_concept_loss(&mut self, concept: &str, loss: f64) {
        const ALPHA: f64 = 0.3;
        let prior = self.concept_loss.get(concept).copied().unwrap_or(1.0);
        let updated = ALPHA * loss + (1.0 - ALPHA) * prior;
        self.concept_loss.insert(concept.to_string(), updated);
    }

    // ---

    /// Add a tension to hold, with the given initial loss.
    ///
    /// State starts as `Held`.
    pub fn hold_tension(&mut self, description: &str, loss: f64) {
        self.tensions.push(HeldTension {
            description: description.to_string(),
            loss,
            state: TensionState::Held,
        });
    }

    // ---

    /// Move tensions whose loss is below `threshold` from `Held` to `Settling`.
    ///
    /// Already-`Settling` or `Settled` tensions are not modified.
    pub fn settle_tensions(&mut self, threshold: f64) {
        for tension in &mut self.tensions {
            if tension.state == TensionState::Held && tension.loss < threshold {
                tension.state = TensionState::Settling;
            }
        }
    }

    // ---

    /// Fork this profile for independent exploration.
    ///
    /// Returns a deep clone. Changes to the fork do not affect the original.
    pub fn fork(&self) -> Self {
        self.clone()
    }

    // ---

    /// Merge a slice of profiles into one by weighting each by its inverse loss.
    ///
    /// Global loss is the weighted average. Crystals and tensions are unioned
    /// (with deduplication for crystals). Concept losses are weighted-averaged
    /// per concept. Attention comes from the lowest-loss profile.
    ///
    /// If `profiles` is empty, panics — a merge of nothing is undefined.
    pub fn merge(profiles: &[Self]) -> Self {
        assert!(!profiles.is_empty(), "merge: profiles must be non-empty");

        // Weights: inverse loss. Guard against zero loss with a floor.
        let weights: Vec<f64> = profiles
            .iter()
            .map(|p| 1.0 / p.loss.max(f64::EPSILON))
            .collect();
        let total_weight: f64 = weights.iter().sum();

        // Weighted average loss.
        let merged_loss: f64 = profiles
            .iter()
            .zip(weights.iter())
            .map(|(p, w)| p.loss * w)
            .sum::<f64>()
            / total_weight;

        // Sum encounters.
        let encounters: u64 = profiles.iter().map(|p| p.encounters).sum();

        // Reader name from first profile.
        let reader = profiles[0].reader.clone();

        // Concept loss: weighted average per concept.
        let mut concept_loss: BTreeMap<String, f64> = BTreeMap::new();
        for (profile, &w) in profiles.iter().zip(weights.iter()) {
            for (concept, &loss) in &profile.concept_loss {
                concept_loss
                    .entry(concept.clone())
                    .and_modify(|acc| *acc += loss * w)
                    .or_insert(loss * w);
            }
        }
        // Normalise by total weight that contributed to each concept.
        let mut concept_weight: BTreeMap<String, f64> = BTreeMap::new();
        for (profile, &w) in profiles.iter().zip(weights.iter()) {
            for concept in profile.concept_loss.keys() {
                *concept_weight.entry(concept.clone()).or_insert(0.0) += w;
            }
        }
        for (concept, acc) in &mut concept_loss {
            *acc /= concept_weight[concept.as_str()];
        }

        // Crystals: union with deduplication, preserving first-seen order.
        let mut crystals: Vec<String> = Vec::new();
        let mut seen_crystals: std::collections::HashSet<String> = std::collections::HashSet::new();
        for profile in profiles {
            for crystal in &profile.crystals {
                if seen_crystals.insert(crystal.clone()) {
                    crystals.push(crystal.clone());
                }
            }
        }

        // Tensions: union all (description may repeat across forks — keep all).
        let tensions: Vec<HeldTension> = profiles
            .iter()
            .flat_map(|p| p.tensions.iter().cloned())
            .collect();

        // Attention: inherit from the lowest-loss profile.
        let best = profiles
            .iter()
            .min_by(|a, b| a.loss.partial_cmp(&b.loss).unwrap())
            .unwrap();
        let attention = best.attention.clone();
        let eigenvalues = best.eigenvalues.clone();

        Self {
            reader,
            updated: String::new(),
            encounters,
            loss: merged_loss,
            eigenvalues,
            concept_loss,
            attention,
            tensions,
            crystals,
        }
    }

    // ---

    /// Return the `n` concepts with the highest loss, sorted descending.
    pub fn high_loss_concepts(&self, n: usize) -> Vec<(&str, f64)> {
        let mut pairs: Vec<(&str, f64)> = self
            .concept_loss
            .iter()
            .map(|(k, &v)| (k.as_str(), v))
            .collect();
        pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        pairs.truncate(n);
        pairs
    }

    // ---

    /// Return references to all tensions currently in the `Held` state.
    pub fn held_tensions(&self) -> Vec<&HeldTension> {
        self.tensions
            .iter()
            .filter(|t| t.state == TensionState::Held)
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // 1: new profile starts at maximum uncertainty.
    #[test]
    fn new_profile_has_high_loss() {
        let p = GestaltProfile::new("alex");
        assert_eq!(p.loss, 1.0);
        assert_eq!(p.encounters, 0);
    }

    // 2: recording an encounter drives loss below its initial value.
    #[test]
    fn record_encounter_decreases_loss() {
        let mut p = GestaltProfile::new("alex");
        p.record_encounter("oid:abc123", 0.2);
        assert!(
            p.loss < 1.0,
            "loss should have decreased, got {}",
            p.loss
        );
        assert_eq!(p.encounters, 1);
        assert_eq!(p.crystals, vec!["oid:abc123"]);
    }

    // 3: a held tension is tracked and surfaced by held_tensions().
    #[test]
    fn hold_tension_tracked() {
        let mut p = GestaltProfile::new("alex");
        p.hold_tension("is structure emergent or imposed?", 0.8);
        let held = p.held_tensions();
        assert_eq!(held.len(), 1);
        assert_eq!(held[0].description, "is structure emergent or imposed?");
        assert_eq!(held[0].state, TensionState::Held);
    }

    // 4: tensions below threshold transition to Settling; high-loss ones stay Held.
    #[test]
    fn settle_tensions_below_threshold() {
        let mut p = GestaltProfile::new("alex");
        p.hold_tension("low tension", 0.1);
        p.hold_tension("high tension", 0.9);
        p.settle_tensions(0.5);

        let states: Vec<(&str, &TensionState)> = p
            .tensions
            .iter()
            .map(|t| (t.description.as_str(), &t.state))
            .collect();

        assert_eq!(
            states[0],
            ("low tension", &TensionState::Settling),
            "low-loss tension should be Settling"
        );
        assert_eq!(
            states[1],
            ("high tension", &TensionState::Held),
            "high-loss tension should remain Held"
        );
    }

    // 5: changes to a fork do not affect the original.
    #[test]
    fn fork_is_independent() {
        let mut original = GestaltProfile::new("alex");
        original.record_encounter("oid:root", 0.5);

        let mut forked = original.fork();
        forked.record_encounter("oid:fork-only", 0.1);
        forked.hold_tension("fork tension", 0.3);

        // Original should be unmodified.
        assert_eq!(original.encounters, 1);
        assert_eq!(original.crystals.len(), 1);
        assert!(original.tensions.is_empty());

        // Fork should carry both encounters and its own tension.
        assert_eq!(forked.encounters, 2);
        assert_eq!(forked.crystals.len(), 2);
        assert_eq!(forked.tensions.len(), 1);
    }

    // 6: merge weights by inverse loss — merged result closer to the better fork.
    #[test]
    fn merge_weights_by_inverse_loss() {
        let mut good = GestaltProfile::new("alex");
        // Drive good fork's loss low.
        for _ in 0..10 {
            good.record_encounter("oid:g", 0.05);
        }

        let mut poor = GestaltProfile::new("alex");
        // Drive poor fork's loss high but below initial.
        for _ in 0..3 {
            poor.record_encounter("oid:p", 0.9);
        }

        let good_loss = good.loss;
        let poor_loss = poor.loss;

        let merged = GestaltProfile::merge(&[good, poor]);

        // Merged loss should be closer to good_loss than to poor_loss.
        let dist_to_good = (merged.loss - good_loss).abs();
        let dist_to_poor = (merged.loss - poor_loss).abs();
        assert!(
            dist_to_good < dist_to_poor,
            "merged loss ({:.4}) should be closer to good ({:.4}) than poor ({:.4})",
            merged.loss,
            good_loss,
            poor_loss
        );
    }
}
