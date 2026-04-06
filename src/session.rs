//! Session — encounter state for mirror shell.
//!
//! /focus → /project → /split → /zoom → /merge → /tock → /exit
//! The Prism of Language as a state machine.
//! Every tick is a reader action. Every tock launches Reflection.

use crate::gestalt::GestaltProfile;

// ---------------------------------------------------------------------------
// State machine
// ---------------------------------------------------------------------------

/// The lifecycle state of an encounter session.
#[derive(Debug, Clone, PartialEq)]
pub enum SessionState {
    Idle,
    Focused { question: String },
    Projected,
    Forked { active_fork: usize },
    Merged,
    Tocked,
}

// ---------------------------------------------------------------------------
// Fork
// ---------------------------------------------------------------------------

/// An independent exploration branch, forked from the session gestalt.
pub struct Fork {
    pub name: String,
    pub gestalt: GestaltProfile,
    pub zoom_depth: usize,
}

// ---------------------------------------------------------------------------
// Session
// ---------------------------------------------------------------------------

/// An encounter session — the stateful shell around a reader's gestalt.
pub struct Session {
    pub state: SessionState,
    pub gestalt: GestaltProfile,
    pub forks: Vec<Fork>,
    pub gestalt_path: String,
    pub tick_count: u64,
    pub tock_count: u64,
}

impl Session {
    // ---

    /// Create a new session for `reader`, bound to `gestalt_path`.
    ///
    /// Loads an existing gestalt from disk if the file exists,
    /// otherwise starts fresh with `GestaltProfile::new(reader)`.
    pub fn new(reader: &str, gestalt_path: &str) -> Self {
        let gestalt =
            GestaltProfile::load(gestalt_path).unwrap_or_else(|_| GestaltProfile::new(reader));
        Self {
            state: SessionState::Idle,
            gestalt,
            forks: Vec::new(),
            gestalt_path: gestalt_path.to_string(),
            tick_count: 0,
            tock_count: 0,
        }
    }

    // ---

    /// Focus the session on a question.
    ///
    /// Increments `tick_count`, transitions state to `Focused`.
    pub fn focus(&mut self, question: &str) -> Result<String, String> {
        self.tick_count += 1;
        self.state = SessionState::Focused {
            question: question.to_string(),
        };
        Ok(format!("focused: {}", question))
    }

    // ---

    /// Project from the current focus — begin resolving a question into structure.
    ///
    /// Requires state to be `Focused` or `Tocked`. Transitions to `Projected`.
    pub fn project(&mut self) -> Result<String, String> {
        match &self.state {
            SessionState::Focused { question } => {
                let q = question.clone();
                self.state = SessionState::Projected;
                Ok(format!("projected from: {}", q))
            }
            SessionState::Tocked => {
                self.state = SessionState::Projected;
                Ok("projected from tocked state".to_string())
            }
            other => Err(format!(
                "project requires Focused or Tocked state, got {:?}",
                other
            )),
        }
    }

    // ---

    /// Split the current gestalt into 2 named forks for independent exploration.
    ///
    /// Each fork starts as a deep clone of the session gestalt.
    pub fn split(&mut self) -> Result<String, String> {
        let fork_a = Fork {
            name: "a".to_string(),
            gestalt: self.gestalt.fork(),
            zoom_depth: 0,
        };
        let fork_b = Fork {
            name: "b".to_string(),
            gestalt: self.gestalt.fork(),
            zoom_depth: 0,
        };
        self.forks = vec![fork_a, fork_b];
        self.state = SessionState::Forked { active_fork: 0 };
        Ok("split into forks: a, b".to_string())
    }

    // ---

    /// Increment the zoom depth on the currently active fork.
    ///
    /// `direction` is recorded as a string label (e.g. "deeper", "simpler", "connected").
    pub fn zoom(&mut self, direction: &str) -> Result<String, String> {
        let active = match &self.state {
            SessionState::Forked { active_fork } => *active_fork,
            other => return Err(format!("zoom requires Forked state, got {:?}", other)),
        };
        if self.forks.is_empty() {
            return Err("zoom: no forks available".to_string());
        }
        let idx = active % self.forks.len();
        self.forks[idx].zoom_depth += 1;
        Ok(format!(
            "zoomed {} on fork '{}' (depth {})",
            direction, self.forks[idx].name, self.forks[idx].zoom_depth
        ))
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

    /// Merge all forks back into the session gestalt.
    ///
    /// Uses `GestaltProfile::merge` weighted by inverse loss.
    /// Clears the fork list. Transitions to `Merged`.
    pub fn merge(&mut self) -> Result<String, String> {
        if self.forks.is_empty() {
            self.state = SessionState::Merged;
            return Ok("merged (no forks — base gestalt kept)".to_string());
        }
        let profiles: Vec<GestaltProfile> = self.forks.iter().map(|f| f.gestalt.clone()).collect();
        self.gestalt = GestaltProfile::merge(&profiles);
        self.forks.clear();
        self.state = SessionState::Merged;
        Ok(format!("merged {} forks", profiles.len()))
    }

    // ---

    /// Tock — end the current reflection cycle.
    ///
    /// Records an encounter on the gestalt, settles tensions below the
    /// threshold (0.3), increments `tock_count`, transitions to `Tocked`.
    pub fn tock(&mut self) -> Result<String, String> {
        let crystal_oid = format!("tock:{}", self.tock_count);
        let loss = self.gestalt.loss;
        self.gestalt.record_encounter(&crystal_oid, loss);
        self.gestalt.settle_tensions(0.3);
        self.tock_count += 1;
        self.state = SessionState::Tocked;
        Ok(format!(
            "tock {} — encounters: {}",
            self.tock_count, self.gestalt.encounters
        ))
    }

    // ---

    /// Refract — save the gestalt to disk.
    ///
    /// If the session hasn't been tocked yet, calls `tock()` first.
    /// Writes the gestalt to `self.gestalt_path`.
    pub fn refract(&mut self) -> Result<String, String> {
        if self.state != SessionState::Tocked {
            self.tock()?;
        }
        self.gestalt
            .save(&self.gestalt_path)
            .map_err(|e| format!("refract save failed: {}", e))?;
        Ok(format!("refracted → {}", self.gestalt_path))
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // 1: new_session_is_idle
    #[test]
    fn new_session_is_idle() {
        let s = Session::new("alex", "/nonexistent/path/gestalt.gestalt");
        assert_eq!(s.state, SessionState::Idle);
        assert_eq!(s.tick_count, 0);
        assert_eq!(s.tock_count, 0);
        assert!(s.forks.is_empty());
    }

    // 2: focus_project_sequence — focus then project works
    #[test]
    fn focus_project_sequence() {
        let mut s = Session::new("alex", "/nonexistent/path/gestalt.gestalt");
        let r = s.focus("is the structure emergent?");
        assert!(r.is_ok(), "focus failed: {:?}", r);
        assert_eq!(s.tick_count, 1);
        assert!(matches!(s.state, SessionState::Focused { .. }));

        let r = s.project();
        assert!(r.is_ok(), "project failed: {:?}", r);
        assert_eq!(s.state, SessionState::Projected);
    }

    // 3: project_requires_focus — project without focus fails
    #[test]
    fn project_requires_focus() {
        let mut s = Session::new("alex", "/nonexistent/path/gestalt.gestalt");
        let r = s.project();
        assert!(r.is_err(), "project should fail when not Focused");
    }

    // 4: split_creates_forks — 2 forks
    #[test]
    fn split_creates_forks() {
        let mut s = Session::new("alex", "/nonexistent/path/gestalt.gestalt");
        let r = s.split();
        assert!(r.is_ok(), "split failed: {:?}", r);
        assert_eq!(s.forks.len(), 2);
        assert_eq!(s.forks[0].name, "a");
        assert_eq!(s.forks[1].name, "b");
        assert!(matches!(s.state, SessionState::Forked { active_fork: 0 }));
    }

    // 5: zoom_increments_fork_depth
    #[test]
    fn zoom_increments_fork_depth() {
        let mut s = Session::new("alex", "/nonexistent/path/gestalt.gestalt");
        s.split().unwrap();
        let depth_before = s.forks[0].zoom_depth;
        let r = s.zoom("deeper");
        assert!(r.is_ok(), "zoom failed: {:?}", r);
        assert_eq!(s.forks[0].zoom_depth, depth_before + 1);
    }

    // 6: merge_reconciles — forks cleared, state=Merged
    #[test]
    fn merge_reconciles() {
        let mut s = Session::new("alex", "/nonexistent/path/gestalt.gestalt");
        s.split().unwrap();
        assert_eq!(s.forks.len(), 2);
        let r = s.merge();
        assert!(r.is_ok(), "merge failed: {:?}", r);
        assert!(s.forks.is_empty(), "forks should be cleared after merge");
        assert_eq!(s.state, SessionState::Merged);
    }

    // 7: tock_records_encounter — encounters++, tock_count++
    #[test]
    fn tock_records_encounter() {
        let mut s = Session::new("alex", "/nonexistent/path/gestalt.gestalt");
        let encounters_before = s.gestalt.encounters;
        let tock_before = s.tock_count;
        let r = s.tock();
        assert!(r.is_ok(), "tock failed: {:?}", r);
        assert_eq!(s.gestalt.encounters, encounters_before + 1);
        assert_eq!(s.tock_count, tock_before + 1);
        assert_eq!(s.state, SessionState::Tocked);
    }

    // 8: refract_saves_gestalt — file exists after refract (use tempfile)
    #[test]
    fn refract_saves_gestalt() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("test.gestalt");
        let path_str = path.to_str().unwrap();

        let mut s = Session::new("alex", path_str);
        s.focus("does structure emerge?").unwrap();
        let r = s.refract();
        assert!(r.is_ok(), "refract failed: {:?}", r);
        assert!(path.exists(), "gestalt file should exist after refract");
    }

    // 9: tick_tock_counting — tick_count and tock_count track separately
    #[test]
    fn tick_tock_counting() {
        let mut s = Session::new("alex", "/nonexistent/path/gestalt.gestalt");

        s.focus("q1").unwrap();
        s.focus("q2").unwrap();
        s.focus("q3").unwrap();
        assert_eq!(s.tick_count, 3);
        assert_eq!(s.tock_count, 0);

        s.tock().unwrap();
        assert_eq!(s.tick_count, 3);
        assert_eq!(s.tock_count, 1);

        s.tock().unwrap();
        assert_eq!(s.tock_count, 2);
    }
}
