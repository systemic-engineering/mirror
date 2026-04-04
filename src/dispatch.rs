//! Dispatch types — Value, Args, Response for action dispatch.
//!
//! These types are used by both mirror (compile-time) and conversation (runtime).

use std::collections::BTreeMap;

use crate::check::Verified;
use crate::model::{ActionName, DomainComplexity};
use crate::Oid;

// ---------------------------------------------------------------------------
// InferenceSchedule
// ---------------------------------------------------------------------------

/// Inference schedule derived from domain eigenvalues.
/// Compile-time ceiling. Runtime narrows via context_complexity (0.0-1.0).
pub enum InferenceSchedule {
    /// Trivial domain. No exploration needed. Collapse immediately.
    Immediate,
    /// Heat kernel curve from domain's eigenvalues.
    Diffusion(coincidence::eigenvalues::Eigenvalues),
}

impl InferenceSchedule {
    pub fn from_verified(verified: &Verified) -> Self {
        match verified.complexity() {
            DomainComplexity::Trivial => InferenceSchedule::Immediate,
            DomainComplexity::Spectrum(spectrum) => {
                InferenceSchedule::Diffusion(spectrum.eigenvalues().clone())
            }
        }
    }

    pub fn temperature(&self, context_complexity: f64) -> f64 {
        match self {
            InferenceSchedule::Immediate => 0.0,
            InferenceSchedule::Diffusion(eigenvalues) => {
                let t = eigenvalues.diffusion_time(context_complexity);
                eigenvalues.temperature_at(t)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Value
// ---------------------------------------------------------------------------

/// A typed runtime value.
#[derive(Clone, Debug)]
pub enum Value {
    Text(String),
    Bytes(Vec<u8>),
    Oid(Oid),
    List(Vec<Value>),
    Map(BTreeMap<String, Value>),
}

// ---------------------------------------------------------------------------
// Args
// ---------------------------------------------------------------------------

/// Arguments passed to a domain action.
#[derive(Clone, Debug)]
pub enum Args {
    Empty,
    Single(Value),
    Named(BTreeMap<ActionName, Value>),
}

// ---------------------------------------------------------------------------
// Response
// ---------------------------------------------------------------------------

/// The result of a successful or failed action dispatch.
#[derive(Clone, Debug)]
pub enum Response {
    Ok(Value),
    Error(String),
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check;
    use crate::model::Mirror;
    use crate::parse::Parse;
    use crate::Vector;

    // -----------------------------------------------------------------------
    // Args tests
    // -----------------------------------------------------------------------

    #[test]
    fn args_empty() {
        let args = Args::Empty;
        assert!(matches!(args, Args::Empty));
    }

    #[test]
    fn args_single() {
        let text = Value::Text("hello".to_owned());
        let args = Args::Single(text);
        assert!(matches!(args, Args::Single(Value::Text(_))));
    }

    #[test]
    fn args_named() {
        let mut map = BTreeMap::new();
        map.insert(ActionName::new("input"), Value::Text("x".to_owned()));
        let args = Args::Named(map.clone());
        assert!(matches!(args, Args::Named(_)));
        // Verify the map contents via the original map (no branch needed).
        assert_eq!(map.len(), 1);
        assert!(matches!(
            map.get(&ActionName::new("input")),
            Some(Value::Text(_))
        ));
    }

    // -----------------------------------------------------------------------
    // Value tests
    // -----------------------------------------------------------------------

    #[test]
    fn value_variants() {
        let text = Value::Text("hi".to_owned());
        assert!(matches!(text, Value::Text(_)));

        let bytes = Value::Bytes(vec![1, 2, 3]);
        assert!(matches!(bytes, Value::Bytes(_)));

        let list = Value::List(vec![Value::Text("a".to_owned())]);
        assert!(matches!(list, Value::List(_)));

        let mut map = BTreeMap::new();
        map.insert("key".to_owned(), Value::Text("val".to_owned()));
        let map_val = Value::Map(map);
        assert!(matches!(map_val, Value::Map(_)));
    }

    #[test]
    fn value_oid() {
        use crate::Oid;
        let oid = Oid::new("test:val");
        let v = Value::Oid(oid);
        assert!(matches!(v, Value::Oid(_)));
    }

    // -----------------------------------------------------------------------
    // Response tests
    // -----------------------------------------------------------------------

    #[test]
    fn response_ok() {
        let r = Response::Ok(Value::Text("done".to_owned()));
        assert!(matches!(r, Response::Ok(_)));
    }

    #[test]
    fn response_error() {
        let r = Response::Error("something went wrong".to_owned());
        assert!(matches!(r, Response::Error(_)));
    }

    // -----------------------------------------------------------------------
    // InferenceSchedule tests
    // -----------------------------------------------------------------------

    #[test]
    fn inference_schedule_immediate_for_trivial() {
        let source = "grammar @simple {\n  type = a | b\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let domain = Mirror::from_grammar(grammar).unwrap();
        let verified = check::verify(domain).unwrap();
        let schedule = InferenceSchedule::from_verified(&verified);
        assert!(matches!(schedule, InferenceSchedule::Immediate));
    }

    #[test]
    fn inference_schedule_diffusion_for_spectrum() {
        let source =
            "grammar @linked {\n  type color = red | blue\n  type pair = combo(color)\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let domain = Mirror::from_grammar(grammar).unwrap();
        let verified = check::verify(domain).unwrap();
        let schedule = InferenceSchedule::from_verified(&verified);
        assert!(
            matches!(&schedule, InferenceSchedule::Diffusion(_)),
            "expected Diffusion"
        );
        let temp = schedule.temperature(1.0);
        assert!(
            temp > 0.0,
            "diffusion schedule should have nonzero temperature"
        );
    }

    #[test]
    fn schedule_temperature_decreases_with_complexity() {
        let source =
            "grammar @linked {\n  type color = red | blue\n  type pair = combo(color)\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let domain = Mirror::from_grammar(grammar).unwrap();
        let verified = check::verify(domain).unwrap();
        let schedule = InferenceSchedule::from_verified(&verified);
        let t_full = schedule.temperature(1.0);
        let t_half = schedule.temperature(0.5);
        let t_zero = schedule.temperature(0.0);
        assert!(
            t_zero >= t_half,
            "zero {} should >= half {}",
            t_zero,
            t_half
        );
        assert!(
            t_half >= t_full,
            "half {} should >= full {}",
            t_half,
            t_full
        );
    }

    #[test]
    fn schedule_immediate_always_zero_temperature() {
        let schedule = InferenceSchedule::Immediate;
        assert_eq!(schedule.temperature(0.0), 0.0);
        assert_eq!(schedule.temperature(0.5), 0.0);
        assert_eq!(schedule.temperature(1.0), 0.0);
    }
}
