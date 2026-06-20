# Prism of Language Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement `@language` and `@gestalt` grammars, the `.gestalt` file format, and seven slash commands in `mirror shell` that map to Prism operations on language.

**Architecture:** `@prism → @reality → @actor → @language`. @reality inherits from @prism and adds the hold constraint: contradictions are held, not resolved. The model checker enforces this across all child grammars. `@language in @actor` defines seven encounter operations. `@gestalt in @language` manages the reader's accumulated portrait including held tensions. The mirror shell REPL handles slash commands (`/focus`, `/project`, `/split`, `/zoom`, `/merge`, `/train`, `/exit`). Each command dispatches through `domain_dispatch.rs`. The `.gestalt` file persists the reader's portrait between sessions.

**Tech Stack:** Rust, mirror crate. Uses existing `domain_dispatch.rs`, `features.rs`, `ghost.rs`, `classifier.rs`. No new dependencies.

**Scope:** This plan covers the foundation — @language, @gestalt, CLI commands, .gestalt file. @surface and @shatter model training are separate future plans that build on this.

**Existing code:**
- `src/domain_dispatch.rs` — `DomainInvocation`, `dispatch()`, @fate and @ai handlers
- `src/main.rs:828` — `shell()` function, simple REPL, reads lines, evals as .conv expressions
- `src/ghost.rs` — `GhostEcho`, `default_echo()`, `coherence_score()`
- `src/features.rs` — `extract_from_source()`, 16-dim feature vector
- `src/classifier.rs` — `classify()`, 2,892-param trained model
- `conv/` — existing grammar files for @prism, @actor, @ai, @fate
- Build: `nix develop -c cargo test` / `CARGO_TARGET_DIR=/Users/alexwolf/.cargo-target`
- Commit as: `Reed <reed@systemic.engineer>`

---

### Task 1: Grammar files — @language and @gestalt

**Files:**
- Create: `conv/language.conv`
- Create: `conv/gestalt.conv`

- [ ] **Step 0: Write conv/reality.conv**

```conv
in @prism

grammar @reality {
  type truth = observation
  type contradiction = tension

  action witness(world) -> truth
  action contradict(truth, truth) -> contradiction
  action hold(contradiction) -> tension
}

---

test "reality holds" {
  @reality has truth
  @reality has contradiction
}
```

- [ ] **Step 0b: Update conv/actor.conv to inherit from @reality**

Change `in @prism` to `in @reality`:

```conv
in @reality

grammar @actor {
  type = identity | session | signal

  type signal = message | question | insight | work | init | exit

  type visibility = public | protected | private

  action action(input)
}

---

test "actor types" {
  @actor has identity
  @actor has session
  @actor has signal
}
```

- [ ] **Step 1: Write conv/language.conv**

```conv
in @actor

grammar @language {
  type encounter = focus | project | split | zoom | refract | merge | train

  type singularity = deficit
  type entanglement = model_weights | reader_weights | visible
  type fork = session | branch | parallel
  type depth = deeper | simpler | connected
  type crystal = gestalt | shatter | permanent
  type union = crystals | meta_crystal | retrained
  type trained = weights | loss | delta

  action focus(question) -> singularity
  action project(singularity) -> entanglement
  action split(entanglement) -> [fork]
  action zoom(fork, direction: depth) -> fork
  action refract(fork) -> crystal
  action merge([crystal]) -> union
  action train(union) -> trained
}

---

test "language encounter types" {
  @language has encounter
  @language.encounter has focus
  @language.encounter has project
  @language.encounter has split
  @language.encounter has zoom
  @language.encounter has refract
  @language.encounter has merge
  @language.encounter has train
}
```

- [ ] **Step 2: Write conv/gestalt.conv**

```conv
in @language

grammar @gestalt {
  type = profile | loss_map | attention | history

  type profile = eigenvalues | updated | encounters
  type loss_map = concept_loss
  type attention = focus_pattern | zoom_preference | split_frequency | fork_depth
  type history = crystal

  action read(reader) -> profile
  action update(profile, crystal) -> profile
  action fork(profile) -> profile
  action merge([profile]) -> profile
  action diff(profile, profile) -> loss_map
}

---

test "gestalt types" {
  @gestalt has profile
  @gestalt has loss_map
  @gestalt has attention
  @gestalt has history
}
```

- [ ] **Step 3: Commit**

```bash
git add conv/language.conv conv/gestalt.conv
git commit -m "🔧 grammar: @language (encounter operations) and @gestalt (reader portrait)"
```

---

### Task 2: Gestalt data model — GestaltProfile struct

**Files:**
- Create: `src/gestalt.rs`
- Modify: `src/lib.rs` (add `pub mod gestalt;`)

The gestalt profile represents the reader's accumulated identity. It's the in-memory representation of a `.gestalt` file.

- [ ] **Step 1: Write the failing test**

```rust
// src/gestalt.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_profile_has_zero_encounters() {
        let profile = GestaltProfile::new("test-reader");
        assert_eq!(profile.encounters, 0);
        assert!(profile.loss > 0.99, "new reader should have high loss");
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `CARGO_TARGET_DIR=/Users/alexwolf/.cargo-target nix develop -c cargo test gestalt::tests::new_profile -- --nocapture`
Expected: FAIL — `GestaltProfile` not found

- [ ] **Step 3: Implement GestaltProfile**

```rust
//! Gestalt — the reader's portrait.
//!
//! Not a trace of one encounter but the accumulated model of who
//! the reader is. Every other domain reads from it. Every refract
//! writes to it.
//!
//! Zero parameters. Pure state. The gestalt IS the reader.

use std::collections::BTreeMap;

/// The reader's accumulated identity.
#[derive(Clone, Debug)]
pub struct GestaltProfile {
    /// Reader identity (SpectralOid as string).
    pub reader: String,
    /// Last update timestamp (ISO-8601).
    pub updated: String,
    /// Total encounter count.
    pub encounters: u64,
    /// Overall Shannon loss (1.0 = knows nothing, 0.0 = knows everything).
    pub loss: f64,
    /// Eigenvalue profile — where the reader lives in concept space.
    pub eigenvalues: Vec<f64>,
    /// Per-concept Shannon loss.
    pub concept_loss: BTreeMap<String, f64>,
    /// Attention signature.
    pub attention: AttentionSignature,
    /// Encounter history — crystal OIDs.
    pub crystals: Vec<String>,
}

/// How the reader explores.
#[derive(Clone, Debug)]
pub struct AttentionSignature {
    /// Primary focus pattern.
    pub focus_pattern: FocusPattern,
    /// Zoom direction preference (ordered).
    pub zoom_preference: Vec<ZoomDirection>,
    /// How often the reader splits (0.0–1.0).
    pub split_frequency: f64,
    /// Average depth reached in forks.
    pub avg_fork_depth: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FocusPattern {
    DepthFirst,
    BreadthFirst,
    Mixed,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ZoomDirection {
    Deeper,
    Simpler,
    Connected,
}

impl GestaltProfile {
    /// Create a new profile for an unknown reader. High loss, no history.
    pub fn new(reader: &str) -> Self {
        GestaltProfile {
            reader: reader.to_string(),
            updated: String::new(),
            encounters: 0,
            loss: 1.0,
            eigenvalues: vec![],
            concept_loss: BTreeMap::new(),
            attention: AttentionSignature {
                focus_pattern: FocusPattern::Mixed,
                zoom_preference: vec![
                    ZoomDirection::Deeper,
                    ZoomDirection::Connected,
                    ZoomDirection::Simpler,
                ],
                split_frequency: 0.0,
                avg_fork_depth: 0.0,
            },
            crystals: vec![],
        }
    }

    /// Record an encounter. Updates loss, encounters, crystals.
    pub fn record_encounter(&mut self, crystal_oid: &str, loss: f64) {
        self.encounters += 1;
        // Exponential moving average of loss
        let alpha = 0.3;
        self.loss = alpha * loss + (1.0 - alpha) * self.loss;
        self.crystals.push(crystal_oid.to_string());
    }

    /// Update per-concept loss from an encounter.
    pub fn update_concept_loss(&mut self, concept: &str, loss: f64) {
        let alpha = 0.3;
        let prev = self.concept_loss.get(concept).copied().unwrap_or(1.0);
        self.concept_loss.insert(concept.to_string(), alpha * loss + (1.0 - alpha) * prev);
    }

    /// Fork this profile — returns a clone for independent exploration.
    pub fn fork(&self) -> Self {
        self.clone()
    }

    /// Merge multiple profiles. Takes union of crystals, averages losses,
    /// weights by inverse loss (better forks contribute more).
    pub fn merge(profiles: &[Self]) -> Self {
        assert!(!profiles.is_empty());
        if profiles.len() == 1 {
            return profiles[0].clone();
        }

        let base = &profiles[0];
        let mut merged = base.clone();

        // Weight each profile by inverse loss (lower loss = better = more weight)
        let weights: Vec<f64> = profiles.iter()
            .map(|p| 1.0 / (p.loss + 0.01))
            .collect();
        let total_weight: f64 = weights.iter().sum();

        // Weighted average of loss
        merged.loss = profiles.iter().zip(&weights)
            .map(|(p, w)| p.loss * w)
            .sum::<f64>() / total_weight;

        // Union of concept losses (weighted average per concept)
        merged.concept_loss.clear();
        let mut all_concepts: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        for p in profiles {
            for k in p.concept_loss.keys() {
                all_concepts.insert(k.clone());
            }
        }
        for concept in &all_concepts {
            let weighted_sum: f64 = profiles.iter().zip(&weights)
                .map(|(p, w)| p.concept_loss.get(concept).copied().unwrap_or(1.0) * w)
                .sum();
            merged.concept_loss.insert(concept.clone(), weighted_sum / total_weight);
        }

        // Union of crystals (deduped)
        let mut all_crystals: Vec<String> = profiles.iter()
            .flat_map(|p| p.crystals.iter().cloned())
            .collect();
        all_crystals.sort();
        all_crystals.dedup();
        merged.crystals = all_crystals;

        // Sum encounters
        merged.encounters = profiles.iter().map(|p| p.encounters).sum();

        merged
    }

    /// Concepts with highest loss — what the reader doesn't know.
    pub fn high_loss_concepts(&self, n: usize) -> Vec<(&str, f64)> {
        let mut pairs: Vec<(&str, f64)> = self.concept_loss.iter()
            .map(|(k, &v)| (k.as_str(), v))
            .collect();
        pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        pairs.truncate(n);
        pairs
    }
}
```

- [ ] **Step 4: Add `pub mod gestalt;` to lib.rs**

Note: there's already a `ghost.rs` module — `gestalt.rs` is separate.

- [ ] **Step 5: Write more tests**

```rust
#[test]
fn record_encounter_updates_loss() {
    let mut profile = GestaltProfile::new("test");
    profile.record_encounter("crystal-1", 0.5);
    assert_eq!(profile.encounters, 1);
    assert!(profile.loss < 1.0, "loss should decrease after encounter");
    assert_eq!(profile.crystals.len(), 1);
}

#[test]
fn concept_loss_tracks_per_concept() {
    let mut profile = GestaltProfile::new("test");
    profile.update_concept_loss("eigenvalue", 0.8);
    profile.update_concept_loss("eigenvalue", 0.4);
    let loss = profile.concept_loss["eigenvalue"];
    assert!(loss < 0.8, "loss should decrease with repeated exposure");
}

#[test]
fn fork_is_independent() {
    let mut profile = GestaltProfile::new("test");
    profile.record_encounter("c1", 0.5);
    let mut forked = profile.fork();
    forked.record_encounter("c2", 0.3);
    assert_eq!(profile.encounters, 1);
    assert_eq!(forked.encounters, 2);
}

#[test]
fn merge_weights_by_inverse_loss() {
    let mut a = GestaltProfile::new("test");
    a.record_encounter("c1", 0.2); // low loss = good
    let mut b = GestaltProfile::new("test");
    b.record_encounter("c2", 0.9); // high loss = bad
    let merged = GestaltProfile::merge(&[a, b]);
    // Merged loss should be closer to a's (low) than b's (high)
    assert!(merged.loss < 0.7, "should weight toward lower-loss fork");
}

#[test]
fn high_loss_concepts_returns_worst() {
    let mut profile = GestaltProfile::new("test");
    profile.update_concept_loss("easy", 0.1);
    profile.update_concept_loss("hard", 0.9);
    profile.update_concept_loss("medium", 0.5);
    let worst = profile.high_loss_concepts(2);
    assert_eq!(worst[0].0, "hard");
    assert_eq!(worst[1].0, "medium");
}
```

- [ ] **Step 6: Run tests**

Run: `CARGO_TARGET_DIR=/Users/alexwolf/.cargo-target nix develop -c cargo test gestalt:: -- --nocapture`
Expected: 6 tests pass

- [ ] **Step 7: Commit**

```bash
git add src/gestalt.rs src/lib.rs
git commit -m "🔧 gestalt: GestaltProfile — reader portrait, fork, merge, concept loss"
```

---

### Task 3: .gestalt file format — parse and emit

**Files:**
- Modify: `src/gestalt.rs` (add parse/emit)

- [ ] **Step 1: Write the failing test**

```rust
#[test]
fn gestalt_round_trips_through_text() {
    let mut profile = GestaltProfile::new("abc123");
    profile.record_encounter("crystal-1", 0.5);
    profile.update_concept_loss("loss", 0.42);
    profile.update_concept_loss("growth", 0.31);

    let text = profile.to_gestalt_text();
    let parsed = GestaltProfile::from_gestalt_text(&text).unwrap();

    assert_eq!(parsed.reader, "abc123");
    assert_eq!(parsed.encounters, 1);
    assert!((parsed.loss - profile.loss).abs() < 0.001);
    assert_eq!(parsed.concept_loss.len(), 2);
}
```

- [ ] **Step 2: Implement to_gestalt_text**

```rust
impl GestaltProfile {
    /// Emit as .gestalt file text.
    pub fn to_gestalt_text(&self) -> String {
        let mut out = String::new();
        out.push_str("gestalt v1\n");
        out.push_str(&format!("reader: {}\n", self.reader));
        out.push_str(&format!("updated: {}\n", self.updated));
        out.push_str(&format!("encounters: {}\n", self.encounters));
        out.push_str(&format!("loss: {:.4}\n", self.loss));
        out.push('\n');

        // Eigenvalues
        if !self.eigenvalues.is_empty() {
            out.push_str("eigenvalues [");
            for (i, v) in self.eigenvalues.iter().enumerate() {
                if i > 0 { out.push_str(", "); }
                out.push_str(&format!("{:.4}", v));
            }
            out.push_str("]\n");
        }
        out.push('\n');

        // Concept loss map
        for (concept, loss) in &self.concept_loss {
            out.push_str(&format!("loss {}:{:.4}\n", concept, loss));
        }
        out.push('\n');

        // Attention
        let pattern = match self.attention.focus_pattern {
            FocusPattern::DepthFirst => "depth_first",
            FocusPattern::BreadthFirst => "breadth_first",
            FocusPattern::Mixed => "mixed",
        };
        out.push_str(&format!("attention {}\n", pattern));
        let zoom_strs: Vec<&str> = self.attention.zoom_preference.iter()
            .map(|z| match z {
                ZoomDirection::Deeper => "deeper",
                ZoomDirection::Simpler => "simpler",
                ZoomDirection::Connected => "connected",
            })
            .collect();
        out.push_str(&format!("zoom {}\n", zoom_strs.join(" > ")));
        out.push_str(&format!("split_frequency {:.2}\n", self.attention.split_frequency));
        out.push_str(&format!("fork_depth {:.1}\n", self.attention.avg_fork_depth));
        out.push('\n');

        // Crystal history
        if !self.crystals.is_empty() {
            out.push_str("crystals [");
            out.push_str(&self.crystals.join(", "));
            out.push_str("]\n");
        }

        out
    }
}
```

- [ ] **Step 3: Implement from_gestalt_text**

```rust
impl GestaltProfile {
    /// Parse from .gestalt file text.
    pub fn from_gestalt_text(text: &str) -> Result<Self, String> {
        let mut profile = GestaltProfile::new("");
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line == "gestalt v1" { continue; }

            if let Some(rest) = line.strip_prefix("reader: ") {
                profile.reader = rest.to_string();
            } else if let Some(rest) = line.strip_prefix("updated: ") {
                profile.updated = rest.to_string();
            } else if let Some(rest) = line.strip_prefix("encounters: ") {
                profile.encounters = rest.parse().map_err(|e| format!("encounters: {}", e))?;
            } else if line.starts_with("loss ") && line.contains(':') {
                // Concept loss: "loss concept:0.42"
                for pair in line.strip_prefix("loss ").unwrap().split_whitespace() {
                    if let Some((concept, val)) = pair.split_once(':') {
                        let v: f64 = val.parse().map_err(|e| format!("loss {}: {}", concept, e))?;
                        profile.concept_loss.insert(concept.to_string(), v);
                    }
                }
            } else if let Some(rest) = line.strip_prefix("loss: ") {
                profile.loss = rest.parse().map_err(|e| format!("loss: {}", e))?;
            } else if let Some(rest) = line.strip_prefix("eigenvalues [") {
                let inner = rest.trim_end_matches(']');
                profile.eigenvalues = inner.split(',')
                    .map(|s| s.trim().parse::<f64>())
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| format!("eigenvalues: {}", e))?;
            } else if let Some(rest) = line.strip_prefix("attention ") {
                profile.attention.focus_pattern = match rest {
                    "depth_first" => FocusPattern::DepthFirst,
                    "breadth_first" => FocusPattern::BreadthFirst,
                    _ => FocusPattern::Mixed,
                };
            } else if let Some(rest) = line.strip_prefix("zoom ") {
                profile.attention.zoom_preference = rest.split(" > ")
                    .filter_map(|s| match s.trim() {
                        "deeper" => Some(ZoomDirection::Deeper),
                        "simpler" => Some(ZoomDirection::Simpler),
                        "connected" => Some(ZoomDirection::Connected),
                        _ => None,
                    })
                    .collect();
            } else if let Some(rest) = line.strip_prefix("split_frequency ") {
                profile.attention.split_frequency = rest.parse().unwrap_or(0.0);
            } else if let Some(rest) = line.strip_prefix("fork_depth ") {
                profile.attention.avg_fork_depth = rest.parse().unwrap_or(0.0);
            } else if let Some(rest) = line.strip_prefix("crystals [") {
                let inner = rest.trim_end_matches(']');
                profile.crystals = inner.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
        }
        Ok(profile)
    }
}
```

- [ ] **Step 4: Write file I/O helpers**

```rust
impl GestaltProfile {
    /// Load from a .gestalt file path.
    pub fn load(path: &str) -> Result<Self, String> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| format!("{}: {}", path, e))?;
        Self::from_gestalt_text(&text)
    }

    /// Save to a .gestalt file path.
    pub fn save(&self, path: &str) -> Result<(), String> {
        std::fs::write(path, self.to_gestalt_text())
            .map_err(|e| format!("{}: {}", path, e))
    }
}
```

- [ ] **Step 5: Run tests**

Run: `CARGO_TARGET_DIR=/Users/alexwolf/.cargo-target nix develop -c cargo test gestalt:: -- --nocapture`

- [ ] **Step 6: Commit**

```bash
git add src/gestalt.rs
git commit -m "🔧 gestalt: .gestalt file format — parse, emit, load, save"
```

---

### Task 4: Session state — encounter tracking in mirror shell

**Files:**
- Create: `src/session.rs`
- Modify: `src/lib.rs` (add `pub mod session;`)

The session tracks the current encounter state: which operation the reader is in, the active forks, and the gestalt being updated.

- [ ] **Step 1: Write the failing test**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_session_is_idle() {
        let session = Session::new("test-reader", ".gestalt");
        assert_eq!(session.state, SessionState::Idle);
        assert_eq!(session.forks.len(), 0);
    }
}
```

- [ ] **Step 2: Implement Session**

```rust
//! Session — encounter state for mirror shell.
//!
//! Tracks the Prism of Language operations as they happen:
//! focus → project → split → zoom → merge → train → refract.

use crate::gestalt::GestaltProfile;

#[derive(Clone, Debug, PartialEq)]
pub enum SessionState {
    Idle,
    Focused { question: String },
    Projected,
    Forked { active_fork: usize },
    Merged,
    Trained,
}

#[derive(Clone, Debug)]
pub struct Fork {
    pub name: String,
    pub gestalt: GestaltProfile,
    pub zoom_depth: usize,
}

/// A mirror shell session. Tracks encounter state and gestalt.
#[derive(Clone, Debug)]
pub struct Session {
    pub state: SessionState,
    pub gestalt: GestaltProfile,
    pub forks: Vec<Fork>,
    pub gestalt_path: String,
}

impl Session {
    /// Start a new session. Loads gestalt from disk or creates new.
    pub fn new(reader: &str, gestalt_path: &str) -> Self {
        let gestalt = GestaltProfile::load(gestalt_path)
            .unwrap_or_else(|_| GestaltProfile::new(reader));
        Session {
            state: SessionState::Idle,
            gestalt,
            forks: vec![],
            gestalt_path: gestalt_path.to_string(),
        }
    }

    /// /focus — ask a question, create singularity.
    pub fn focus(&mut self, question: &str) -> Result<String, String> {
        self.state = SessionState::Focused { question: question.to_string() };
        Ok(format!("singularity: deficit identified ({})", question))
    }

    /// /project — make the entanglement visible.
    pub fn project(&mut self) -> Result<String, String> {
        match &self.state {
            SessionState::Focused { .. } => {
                self.state = SessionState::Projected;
                Ok(format!("entanglement visible:\n  reader loss: {:.2}\n  encounters: {}",
                    self.gestalt.loss, self.gestalt.encounters))
            }
            _ => Err("nothing to project — /focus first".to_string()),
        }
    }

    /// /split — fork the session.
    pub fn split(&mut self) -> Result<String, String> {
        if !matches!(self.state, SessionState::Projected | SessionState::Focused { .. }) {
            return Err("/split requires a focused or projected state".to_string());
        }
        self.forks = vec![
            Fork { name: "fork-a".into(), gestalt: self.gestalt.fork(), zoom_depth: 0 },
            Fork { name: "fork-b".into(), gestalt: self.gestalt.fork(), zoom_depth: 0 },
        ];
        self.state = SessionState::Forked { active_fork: 0 };
        let names: Vec<&str> = self.forks.iter().map(|f| f.name.as_str()).collect();
        Ok(format!("split: {}", names.join(", ")))
    }

    /// /zoom — go deeper in the active fork.
    pub fn zoom(&mut self, direction: &str) -> Result<String, String> {
        match &self.state {
            SessionState::Forked { active_fork } => {
                let idx = *active_fork;
                if let Some(fork) = self.forks.get_mut(idx) {
                    fork.zoom_depth += 1;
                    Ok(format!("{}: zoom {} (depth {})", fork.name, direction, fork.zoom_depth))
                } else {
                    Err("no active fork".to_string())
                }
            }
            SessionState::Focused { .. } | SessionState::Projected => {
                Ok(format!("zoom {} (depth 1)", direction))
            }
            _ => Err("/zoom requires an active encounter".to_string()),
        }
    }

    /// /merge — reunite forks.
    pub fn merge(&mut self) -> Result<String, String> {
        if self.forks.is_empty() {
            return Err("nothing to merge — /split first".to_string());
        }
        let profiles: Vec<GestaltProfile> = self.forks.iter()
            .map(|f| f.gestalt.clone())
            .collect();
        self.gestalt = GestaltProfile::merge(&profiles);

        let losses: Vec<String> = self.forks.iter()
            .map(|f| format!("{}: {:.2}", f.name, f.gestalt.loss))
            .collect();

        self.forks.clear();
        self.state = SessionState::Merged;
        Ok(format!("merged: {}", losses.join(", ")))
    }

    /// /train — update weights inline.
    pub fn train(&mut self) -> Result<String, String> {
        // Record the encounter in the gestalt
        let crystal_oid = format!("crystal-{}", self.gestalt.encounters + 1);
        let current_loss = self.gestalt.loss;
        self.gestalt.record_encounter(&crystal_oid, current_loss);
        self.state = SessionState::Trained;
        Ok(format!("trained: encounter {} recorded, loss {:.4}",
            self.gestalt.encounters, self.gestalt.loss))
    }

    /// /exit — refract. Crystallize and save.
    pub fn refract(&mut self) -> Result<String, String> {
        // Always train on exit if not already trained
        if self.state != SessionState::Trained {
            let _ = self.train();
        }
        // Save gestalt
        self.gestalt.save(&self.gestalt_path)
            .map_err(|e| format!("save gestalt: {}", e))?;
        Ok(format!("refract: session crystallized, {} encounters total",
            self.gestalt.encounters))
    }
}
```

- [ ] **Step 3: Write tests**

```rust
#[test]
fn focus_sets_state() {
    let mut session = Session::new("test", "/tmp/test.gestalt");
    let result = session.focus("what is loss?").unwrap();
    assert!(result.contains("deficit"));
    assert!(matches!(session.state, SessionState::Focused { .. }));
}

#[test]
fn project_requires_focus() {
    let mut session = Session::new("test", "/tmp/test.gestalt");
    assert!(session.project().is_err());
    session.focus("question").unwrap();
    assert!(session.project().is_ok());
}

#[test]
fn split_creates_forks() {
    let mut session = Session::new("test", "/tmp/test.gestalt");
    session.focus("question").unwrap();
    session.split().unwrap();
    assert_eq!(session.forks.len(), 2);
}

#[test]
fn zoom_increments_depth() {
    let mut session = Session::new("test", "/tmp/test.gestalt");
    session.focus("question").unwrap();
    session.split().unwrap();
    session.zoom("deeper").unwrap();
    assert_eq!(session.forks[0].zoom_depth, 1);
}

#[test]
fn merge_reconciles_forks() {
    let mut session = Session::new("test", "/tmp/test.gestalt");
    session.focus("question").unwrap();
    session.split().unwrap();
    session.merge().unwrap();
    assert!(session.forks.is_empty());
    assert_eq!(session.state, SessionState::Merged);
}

#[test]
fn train_records_encounter() {
    let mut session = Session::new("test", "/tmp/test.gestalt");
    session.focus("question").unwrap();
    session.train().unwrap();
    assert_eq!(session.gestalt.encounters, 1);
}

#[test]
fn refract_saves_gestalt() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.gestalt");
    let mut session = Session::new("test", path.to_str().unwrap());
    session.focus("question").unwrap();
    session.refract().unwrap();
    assert!(path.exists());
}
```

- [ ] **Step 4: Run tests**

Run: `CARGO_TARGET_DIR=/Users/alexwolf/.cargo-target nix develop -c cargo test session:: -- --nocapture`

- [ ] **Step 5: Commit**

```bash
git add src/session.rs src/lib.rs
git commit -m "🔧 session: encounter state machine — focus/project/split/zoom/merge/train/refract"
```

---

### Task 5: Wire slash commands into mirror shell

**Files:**
- Modify: `src/main.rs` — update `shell()` function

The shell REPL handles `/command` lines as Prism operations and everything else as mirror expressions (existing behavior).

- [ ] **Step 1: Update shell() to handle slash commands**

Replace the `shell()` function body's line processing:

```rust
fn shell(path: &str, resolve: &Resolve) {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut stdout = io::stdout();

    // Determine gestalt path: .gestalt in current dir
    let gestalt_path = format!("{}/.gestalt", path);
    let reader_id = whoami(); // or a default
    let mut session = mirror::session::Session::new(&reader_id, &gestalt_path);

    eprintln!("mirror shell — {}", path);
    eprintln!("commands: /focus /project /split /zoom /merge /train /exit");
    eprintln!("type expressions or commands, ctrl+d to exit\n");

    for line in reader.lines() {
        let _ = write!(stdout, "mirror> ");
        let _ = stdout.flush();

        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("mirror: read error: {}", e);
                break;
            }
        };

        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }

        // Slash commands → session operations
        if line.starts_with('/') {
            let parts: Vec<&str> = line[1..].splitn(2, ' ').collect();
            let cmd = parts[0];
            let arg = parts.get(1).copied().unwrap_or("");

            let result = match cmd {
                "focus" => session.focus(arg),
                "project" => session.project(),
                "split" => session.split(),
                "zoom" => session.zoom(if arg.is_empty() { "deeper" } else { arg }),
                "merge" => session.merge(),
                "train" => session.train(),
                "exit" => {
                    match session.refract() {
                        Ok(msg) => eprintln!("  {}", msg),
                        Err(e) => eprintln!("  error: {}", e),
                    }
                    break;
                }
                _ => Err(format!("unknown command: /{}", cmd)),
            };

            match result {
                Ok(msg) => eprintln!("  {}", msg),
                Err(e) => eprintln!("  error: {}", e),
            }
            continue;
        }

        // Domain dispatch: @domain action args
        if line.starts_with('@') {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(inv) = mirror::domain_dispatch::DomainInvocation::parse(&parts) {
                match mirror::domain_dispatch::dispatch(&inv) {
                    Ok(output) => print!("{}", output),
                    Err(e) => eprintln!("  error: {}", e),
                }
            } else {
                eprintln!("  usage: @domain action [args]");
            }
            continue;
        }

        // Default: evaluate as .conv expression (existing behavior)
        let source = format!("out {}\n", line);
        let resolved = match Conversation::<Filesystem>::from_source_with(&source, resolve.clone())
        {
            Ok(conv) => conv,
            Err(e) => {
                eprintln!("  error: {}", e);
                continue;
            }
        };

        let tree = Folder::read_tree(path);
        let value = resolved.trace(tree).into_result().unwrap();
        let json = serde_json::to_string_pretty(&value).unwrap();
        println!("{}", json);
    }

    let _ = writeln!(stdout);
}

fn whoami() -> String {
    std::env::var("USER").unwrap_or_else(|_| "anonymous".to_string())
}
```

- [ ] **Step 2: Test interactively**

Run: `CARGO_TARGET_DIR=/Users/alexwolf/.cargo-target nix develop -c cargo build`
Then: `./target/debug/mirror shell .`

Test:
```
mirror> /focus "what is loss?"
  singularity: deficit identified (what is loss?)
mirror> /project
  entanglement visible: ...
mirror> /zoom deeper
  zoom deeper (depth 1)
mirror> /train
  trained: encounter 1 recorded
mirror> /exit
  refract: session crystallized
```

- [ ] **Step 3: Commit**

```bash
git add src/main.rs
git commit -m "🔧 shell: slash commands — /focus /project /split /zoom /merge /train /exit"
```

---

### Task 6: Wire @gestalt into domain dispatch

**Files:**
- Modify: `src/domain_dispatch.rs`

- [ ] **Step 1: Add @gestalt dispatch**

```rust
// In dispatch() match:
"gestalt" => dispatch_gestalt(&inv.action, &inv.args),

// New function:
fn dispatch_gestalt(action: &str, args: &[String]) -> Result<String, String> {
    let gestalt_path = args.first()
        .map(|s| s.as_str())
        .unwrap_or(".gestalt");

    match action {
        "read" => {
            let profile = crate::gestalt::GestaltProfile::load(gestalt_path)?;
            Ok(format!("@gestalt read\n  reader: {}\n  encounters: {}\n  loss: {:.4}\n  concepts: {}",
                profile.reader, profile.encounters, profile.loss, profile.concept_loss.len()))
        }
        "loss" => {
            let profile = crate::gestalt::GestaltProfile::load(gestalt_path)?;
            let high = profile.high_loss_concepts(5);
            let mut out = String::from("@gestalt loss (highest)\n");
            for (concept, loss) in high {
                out.push_str(&format!("  {}: {:.4}\n", concept, loss));
            }
            Ok(out)
        }
        "diff" => {
            if args.len() < 2 {
                return Err("usage: @gestalt diff <path-a> <path-b>".to_string());
            }
            let a = crate::gestalt::GestaltProfile::load(&args[0])?;
            let b = crate::gestalt::GestaltProfile::load(&args[1])?;
            let mut out = String::from("@gestalt diff\n");
            out.push_str(&format!("  a: {} encounters, loss {:.4}\n", a.encounters, a.loss));
            out.push_str(&format!("  b: {} encounters, loss {:.4}\n", b.encounters, b.loss));
            out.push_str(&format!("  delta loss: {:.4}\n", (a.loss - b.loss).abs()));
            Ok(out)
        }
        _ => Err(format!("@gestalt: unknown action: {}", action)),
    }
}
```

- [ ] **Step 2: Write tests**

```rust
#[test]
fn dispatch_gestalt_read_missing_returns_error() {
    let inv = DomainInvocation {
        domain: "gestalt".to_string(),
        action: "read".to_string(),
        args: vec!["/nonexistent/.gestalt".to_string()],
    };
    assert!(dispatch(&inv).is_err());
}
```

- [ ] **Step 3: Run tests**

Run: `CARGO_TARGET_DIR=/Users/alexwolf/.cargo-target nix develop -c cargo test domain_dispatch -- --nocapture`

- [ ] **Step 4: Commit**

```bash
git add src/domain_dispatch.rs
git commit -m "🔧 dispatch: @gestalt read/loss/diff — query the reader's portrait"
```

---

## Self-Review

**Spec coverage:**
- @language grammar ✅ (Task 1)
- @gestalt grammar ✅ (Task 1)
- GestaltProfile struct with fork/merge ✅ (Task 2)
- .gestalt file format parse/emit ✅ (Task 3)
- Session state machine ✅ (Task 4)
- /focus /project /split /zoom /merge /train /exit ✅ (Task 5)
- @gestalt dispatch (read/loss/diff) ✅ (Task 6)
- @domain dispatch in shell (inline) ✅ (Task 5)
- @surface and @shatter: **deferred** (separate plans, consumers of @gestalt)

**Placeholder scan:** No TBDs found. All code blocks complete.

**Type consistency:**
- `GestaltProfile` used consistently across Tasks 2-6
- `Session` methods match slash command names
- `SessionState` enum variants match the operation sequence
- `dispatch_gestalt` reads from the same `GestaltProfile::load` path
