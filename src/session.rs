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
    pub fn focus(&mut self, _question: &str) -> Result<String, String> {
        unimplemented!("focus: red phase")
    }

    // ---

    /// /project -- make the entanglement visible.
    pub fn project(&mut self) -> Result<String, String> {
        unimplemented!("project: red phase")
    }

    // ---

    /// /split -- fork the session.
    pub fn split(&mut self) -> Result<String, String> {
        unimplemented!("split: red phase")
    }

    // ---

    /// /zoom -- go deeper in the active fork.
    pub fn zoom(&mut self, _direction: &str) -> Result<String, String> {
        unimplemented!("zoom: red phase")
    }

    // ---

    /// Switch the active fork by name.
    pub fn switch_fork(&mut self, _name: &str) -> Result<String, String> {
        unimplemented!("switch_fork: red phase")
    }

    // ---

    /// /merge -- reunite forks.
    pub fn merge(&mut self) -> Result<String, String> {
        unimplemented!("merge: red phase")
    }

    // ---

    /// /train -- update weights inline.
    pub fn train(&mut self) -> Result<String, String> {
        unimplemented!("train: red phase")
    }

    // ---

    /// /exit -- refract. Crystallize and save.
    pub fn refract(&mut self) -> Result<String, String> {
        unimplemented!("refract: red phase")
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
        // After refract, train was called, so encounters should be 1
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
