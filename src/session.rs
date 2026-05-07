//! Session — encounter state for mirror shell.
//!
//! Tracks the Prism of Language operations as they happen:
//! focus -> project -> split -> zoom -> merge -> train -> refract.

use crate::gestalt::GestaltProfile;

// ---------------------------------------------------------------------------
// State machine
// ---------------------------------------------------------------------------

/// The lifecycle state of an encounter session.
#[derive(Clone, Debug, PartialEq)]
pub enum SessionState {
    Idle,
    Focused { question: String },
    Projected,
    Forked { active_fork: usize },
    Merged,
    Trained,
}

// ---------------------------------------------------------------------------
// Fork
// ---------------------------------------------------------------------------

/// An independent exploration branch, forked from the session gestalt.
#[derive(Clone, Debug)]
pub struct Fork {
    pub name: String,
    pub gestalt: GestaltProfile,
    pub zoom_depth: usize,
}

// ---------------------------------------------------------------------------
// Session
// ---------------------------------------------------------------------------

/// A mirror shell session. Tracks encounter state and gestalt.
#[derive(Clone, Debug)]
pub struct Session {
    pub state: SessionState,
    pub gestalt: GestaltProfile,
    pub forks: Vec<Fork>,
    pub gestalt_path: String,
}

impl Session {
    // ---

    /// Start a new session. Loads gestalt from disk or creates new.
    pub fn new(reader: &str, gestalt_path: &str) -> Self {
        let gestalt =
            GestaltProfile::load(gestalt_path).unwrap_or_else(|_| GestaltProfile::new(reader));
        Session {
            state: SessionState::Idle,
            gestalt,
            forks: vec![],
            gestalt_path: gestalt_path.to_string(),
        }
    }

    // ---

    /// /focus -- ask a question, create singularity.
    pub fn focus(&mut self, question: &str) -> Result<String, String> {
        self.state = SessionState::Focused {
            question: question.to_string(),
        };
        Ok(format!("singularity: deficit identified ({})", question))
    }

    // ---

    /// /project -- make the entanglement visible.
    pub fn project(&mut self) -> Result<String, String> {
        match &self.state {
            SessionState::Focused { .. } => {
                self.state = SessionState::Projected;
                Ok(format!(
                    "entanglement visible:\n  reader loss: {:.2}\n  encounters: {}",
                    self.gestalt.loss, self.gestalt.encounters
                ))
            }
            _ => Err("nothing to project -- /focus first".to_string()),
        }
    }

    // ---

    /// /split -- fork the session.
    pub fn split(&mut self) -> Result<String, String> {
        if !matches!(
            self.state,
            SessionState::Projected | SessionState::Focused { .. }
        ) {
            return Err("/split requires a focused or projected state".to_string());
        }
        self.forks = vec![
            Fork {
                name: "fork-a".into(),
                gestalt: self.gestalt.fork(),
                zoom_depth: 0,
            },
            Fork {
                name: "fork-b".into(),
                gestalt: self.gestalt.fork(),
                zoom_depth: 0,
            },
        ];
        self.state = SessionState::Forked { active_fork: 0 };
        let names: Vec<&str> = self.forks.iter().map(|f| f.name.as_str()).collect();
        Ok(format!("split: {}", names.join(", ")))
    }

    // ---

    /// /zoom -- go deeper in the active fork.
    pub fn zoom(&mut self, direction: &str) -> Result<String, String> {
        match &self.state {
            SessionState::Forked { active_fork } => {
                let idx = *active_fork;
                if let Some(fork) = self.forks.get_mut(idx) {
                    fork.zoom_depth += 1;
                    Ok(format!(
                        "{}: zoom {} (depth {})",
                        fork.name, direction, fork.zoom_depth
                    ))
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

    // ---

    /// Switch the active fork by name.
    pub fn switch_fork(&mut self, name: &str) -> Result<String, String> {
        let idx = self
            .forks
            .iter()
            .position(|f| f.name == name)
            .ok_or_else(|| format!("no fork named '{}'", name))?;
        self.state = SessionState::Forked { active_fork: idx };
        Ok(format!("switched to fork '{}'", name))
    }

    // ---

    /// /merge -- reunite forks.
    pub fn merge(&mut self) -> Result<String, String> {
        if self.forks.is_empty() {
            return Err("nothing to merge -- /split first".to_string());
        }
        let profiles: Vec<GestaltProfile> = self.forks.iter().map(|f| f.gestalt.clone()).collect();
        self.gestalt = GestaltProfile::merge(&profiles);

        let losses: Vec<String> = self
            .forks
            .iter()
            .map(|f| format!("{}: {:.2}", f.name, f.gestalt.loss))
            .collect();

        self.forks.clear();
        self.state = SessionState::Merged;
        Ok(format!("merged: {}", losses.join(", ")))
    }

    // ---

    /// /train -- update weights inline.
    pub fn train(&mut self) -> Result<String, String> {
        let crystal_oid = format!("crystal-{}", self.gestalt.encounters + 1);
        let current_loss = self.gestalt.loss;
        self.gestalt.record_encounter(&crystal_oid, current_loss);
        self.state = SessionState::Trained;
        Ok(format!(
            "trained: encounter {} recorded, loss {:.4}",
            self.gestalt.encounters, self.gestalt.loss
        ))
    }

    // ---

    /// /exit -- refract. Crystallize and save.
    pub fn refract(&mut self) -> Result<String, String> {
        // Always train on exit if not already trained
        if self.state != SessionState::Trained {
            let _ = self.train();
        }
        // Save gestalt
        self.gestalt
            .save(&self.gestalt_path)
            .map_err(|e| format!("save gestalt: {}", e))?;
        Ok(format!(
            "refract: session crystallized, {} encounters total",
            self.gestalt.encounters
        ))
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // 1: new session starts idle with empty forks
    #[test]
    fn new_session_is_idle() {
        let s = Session::new("test-reader", ".gestalt");
        assert_eq!(s.state, SessionState::Idle);
        assert_eq!(s.forks.len(), 0);
    }

    // 2: focus sets state and returns deficit message
    #[test]
    fn focus_sets_state() {
        let mut s = Session::new("test", "/tmp/test.gestalt");
        let result = s.focus("what is loss?").unwrap();
        assert!(result.contains("deficit"));
        assert!(matches!(s.state, SessionState::Focused { .. }));
    }

    // 3: project requires focus
    #[test]
    fn project_requires_focus() {
        let mut s = Session::new("test", "/tmp/test.gestalt");
        assert!(s.project().is_err());
        s.focus("question").unwrap();
        assert!(s.project().is_ok());
    }

    // 4: split creates 2 forks
    #[test]
    fn split_creates_forks() {
        let mut s = Session::new("test", "/tmp/test.gestalt");
        s.focus("question").unwrap();
        s.split().unwrap();
        assert_eq!(s.forks.len(), 2);
    }

    // 5: split requires focused or projected state
    #[test]
    fn split_requires_focus_or_projected() {
        let mut s = Session::new("test", "/tmp/test.gestalt");
        assert!(s.split().is_err(), "split should fail when Idle");
    }

    // 6: zoom increments fork depth
    #[test]
    fn zoom_increments_depth() {
        let mut s = Session::new("test", "/tmp/test.gestalt");
        s.focus("question").unwrap();
        s.split().unwrap();
        s.zoom("deeper").unwrap();
        assert_eq!(s.forks[0].zoom_depth, 1);
    }

    // 7: zoom works on focused state too
    #[test]
    fn zoom_on_focused() {
        let mut s = Session::new("test", "/tmp/test.gestalt");
        s.focus("question").unwrap();
        let r = s.zoom("deeper");
        assert!(r.is_ok(), "zoom should work when Focused");
    }

    // 8: merge reconciles forks
    #[test]
    fn merge_reconciles_forks() {
        let mut s = Session::new("test", "/tmp/test.gestalt");
        s.focus("question").unwrap();
        s.split().unwrap();
        s.merge().unwrap();
        assert!(s.forks.is_empty());
        assert_eq!(s.state, SessionState::Merged);
    }

    // 9: merge errors when no forks
    #[test]
    fn merge_errors_without_forks() {
        let mut s = Session::new("test", "/tmp/test.gestalt");
        assert!(s.merge().is_err(), "merge should fail with no forks");
    }

    // 10: train records encounter
    #[test]
    fn train_records_encounter() {
        let mut s = Session::new("test", "/tmp/test.gestalt");
        s.focus("question").unwrap();
        s.train().unwrap();
        assert_eq!(s.gestalt.encounters, 1);
        assert_eq!(s.state, SessionState::Trained);
    }

    // 11: refract saves gestalt to disk
    #[test]
    fn refract_saves_gestalt() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.gestalt");
        let mut s = Session::new("test", path.to_str().unwrap());
        s.focus("question").unwrap();
        s.refract().unwrap();
        assert!(path.exists());
    }

    // 12: refract auto-trains if not already trained
    #[test]
    fn refract_auto_trains() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.gestalt");
        let mut s = Session::new("test", path.to_str().unwrap());
        s.focus("question").unwrap();
        assert_ne!(s.state, SessionState::Trained);
        s.refract().unwrap();
        assert_eq!(s.gestalt.encounters, 1);
    }

    // 13: switch_fork changes active fork
    #[test]
    fn switch_fork_changes_active() {
        let mut s = Session::new("test", "/tmp/test.gestalt");
        s.focus("question").unwrap();
        s.split().unwrap();
        assert!(matches!(s.state, SessionState::Forked { active_fork: 0 }));
        s.switch_fork("fork-b").unwrap();
        assert!(matches!(s.state, SessionState::Forked { active_fork: 1 }));
    }

    // 14: full lifecycle: focus -> project -> split -> zoom -> merge -> train -> refract
    #[test]
    fn full_lifecycle() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("lifecycle.gestalt");
        let mut s = Session::new("test", path.to_str().unwrap());

        s.focus("what is structure?").unwrap();
        assert!(matches!(s.state, SessionState::Focused { .. }));

        s.project().unwrap();
        assert_eq!(s.state, SessionState::Projected);

        s.split().unwrap();
        assert!(matches!(s.state, SessionState::Forked { .. }));
        assert_eq!(s.forks.len(), 2);

        s.zoom("deeper").unwrap();
        assert_eq!(s.forks[0].zoom_depth, 1);

        s.merge().unwrap();
        assert_eq!(s.state, SessionState::Merged);
        assert!(s.forks.is_empty());

        s.train().unwrap();
        assert_eq!(s.state, SessionState::Trained);
        assert_eq!(s.gestalt.encounters, 1);

        s.refract().unwrap();
        assert!(path.exists());
    }
}
