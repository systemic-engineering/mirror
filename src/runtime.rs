//! Runtime trait + typed Args/Response/Value.
//!
//! Defines the async `Runtime` trait — one method: `compile(Verified) -> Artifact`.
//! Plus the typed `Args`, `Response`, and `Value` enums.
//!
//! `RactorRuntime` is the concrete implementation. Stateless — the caller
//! owns the artifact.

use std::collections::BTreeMap;
use std::fmt;

use ractor::{Actor, ActorProcessingErr, ActorRef};

use crate::check::Verified;
use crate::model::{ActionName, Domain, DomainComplexity};
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
// RuntimeError
// ---------------------------------------------------------------------------

/// A runtime-level error (domain not found, action not found, etc.).
#[derive(Debug)]
pub struct RuntimeError(String);

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ---------------------------------------------------------------------------
// Runtime trait
// ---------------------------------------------------------------------------

/// The compiler backend. Takes a verified domain, produces an artifact
/// wrapped in a Beam — the trace of the compilation.
#[allow(async_fn_in_trait)]
pub trait Runtime: Send + Sync {
    type Artifact;
    type Error: fmt::Display + Send;

    /// Compile a verified domain into a runtime artifact.
    /// Result handles total failure. Beam handles partial success with loss.
    async fn compile(
        &mut self,
        domain: Verified,
    ) -> Result<prism::Beam<Self::Artifact>, Self::Error>;
}

// ---------------------------------------------------------------------------
// DomainActor
// ---------------------------------------------------------------------------

/// Message type for actor-backed domains.
///
/// `Dispatch(action, args, reply)` — tuple variant so `ractor::call!` can
/// construct it as `Dispatch(action, args, tx)`.
pub enum DomainMessage {
    Dispatch(
        ActionName,
        Args,
        ractor::RpcReplyPort<Result<Response, String>>,
    ),
}

pub(crate) struct DomainActorState {
    domain: Domain,
}

pub(crate) struct DomainActor;

impl Actor for DomainActor {
    type Msg = DomainMessage;
    type State = DomainActorState;
    type Arguments = Domain;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(DomainActorState { domain: args })
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            DomainMessage::Dispatch(action, _args, reply) => {
                let action_exists = state.domain.actions.iter().any(|a| a.name == action);
                let result = if action_exists {
                    Ok(Response::Ok(Value::Text(format!(
                        "{}:{}",
                        state.domain.name, action
                    ))))
                } else {
                    Err(format!(
                        "unknown action '{}' in domain @{}",
                        action, state.domain.name
                    ))
                };
                let _ = reply.send(result);
            }
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// RactorRuntime
// ---------------------------------------------------------------------------

/// Ractor runtime. Compiles domains into running actors.
/// Stateless — the caller owns the artifact.
pub struct RactorRuntime;

impl RactorRuntime {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RactorRuntime {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert a ractor spawn error into a RuntimeError.
/// Named function to avoid inline-closure monomorphization coverage issues.
fn spawn_err(e: ractor::SpawnErr) -> RuntimeError {
    RuntimeError(format!("failed to spawn actor: {}", e))
}

impl Runtime for RactorRuntime {
    type Artifact = ActorRef<DomainMessage>;
    type Error = RuntimeError;

    async fn compile(
        &mut self,
        domain: Verified,
    ) -> Result<prism::Beam<ActorRef<DomainMessage>>, RuntimeError> {
        let d = domain.into_domain();
        let (actor_ref, _handle) = Actor::spawn(None, DomainActor, d)
            .await
            .map_err(spawn_err)?;
        Ok(prism::Beam::new(actor_ref))
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check;
    use crate::model::{
        Action, Domain, DomainName, Properties, TypeDef, TypeName, Variant, VariantName,
    };
    use crate::resolve::Visibility;

    // -----------------------------------------------------------------------
    // Test helpers
    // -----------------------------------------------------------------------

    /// Build a simple verified domain:
    ///   type color = red | blue
    ///   action paint
    fn simple_verified() -> Verified {
        let color = TypeDef {
            name: TypeName::new("color"),
            variants: vec![
                Variant {
                    name: VariantName::new("red"),
                    params: vec![],
                },
                Variant {
                    name: VariantName::new("blue"),
                    params: vec![],
                },
            ],
        };
        let paint = Action {
            name: ActionName::new("paint"),
            fields: vec![],
            visibility: Visibility::Public,
            calls: vec![],
        };
        let domain = Domain {
            name: DomainName::new("color"),
            types: vec![color],
            actions: vec![paint],
            lenses: vec![],
            extends: vec![],
            calls: vec![],
            properties: Properties::empty(),
        };
        check::verify(domain).unwrap()
    }

    /// Build a verified actor domain:
    ///   in @actor
    ///   type target = rust | beam
    ///   action compile
    fn actor_verified() -> Verified {
        use crate::model::Lens;
        let domain = Domain {
            name: DomainName::new("compiler"),
            types: vec![TypeDef {
                name: TypeName::new("target"),
                variants: vec![
                    Variant {
                        name: VariantName::new("rust"),
                        params: vec![],
                    },
                    Variant {
                        name: VariantName::new("beam"),
                        params: vec![],
                    },
                ],
            }],
            actions: vec![Action {
                name: ActionName::new("compile"),
                fields: vec![],
                visibility: Visibility::Protected,
                calls: vec![],
            }],
            lenses: vec![Lens {
                target: DomainName::new("actor"),
            }],
            extends: vec![],
            calls: vec![],
            properties: Properties::empty(),
        };
        check::verify(domain).unwrap()
    }

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
    // RactorRuntime tests
    // -----------------------------------------------------------------------

    #[test]
    fn ractor_runtime_default() {
        let _rt = RactorRuntime::default();
    }

    #[tokio::test]
    async fn compile_produces_artifact() {
        let mut rt = RactorRuntime::new();
        let verified = simple_verified();
        let beam = rt.compile(verified).await.unwrap();
        let artifact = beam.result;
        let resp = ractor::call!(
            artifact, DomainMessage::Dispatch,
            ActionName::new("paint"), Args::Empty
        ).unwrap();
        assert!(matches!(resp, Ok(Response::Ok(_))));
        artifact.stop(None);
    }

    #[tokio::test]
    async fn compile_unknown_action_errors() {
        let mut rt = RactorRuntime::new();
        let verified = simple_verified();
        let beam = rt.compile(verified).await.unwrap();
        let artifact = beam.result;
        let resp = ractor::call!(
            artifact, DomainMessage::Dispatch,
            ActionName::new("fly"), Args::Empty
        ).unwrap();
        assert!(matches!(resp, Err(_)));
        artifact.stop(None);
    }

    #[tokio::test]
    async fn compile_actor_domain() {
        let mut rt = RactorRuntime::new();
        let verified = actor_verified();
        let beam = rt.compile(verified).await.unwrap();
        let artifact = beam.result;
        let resp = ractor::call!(
            artifact, DomainMessage::Dispatch,
            ActionName::new("compile"), Args::Empty
        ).unwrap();
        assert!(matches!(resp, Ok(Response::Ok(_))));
        artifact.stop(None);
    }

    #[tokio::test]
    async fn compile_and_stop() {
        let mut rt = RactorRuntime::new();
        let verified = simple_verified();
        let beam = rt.compile(verified).await.unwrap();
        let artifact = beam.result;
        artifact.stop(None);
        // Actor should be stopped — let it settle
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }

    // -----------------------------------------------------------------------
    // Error conversion helpers — coverage for spawn_err
    // -----------------------------------------------------------------------

    #[test]
    fn spawn_err_formats_message() {
        let e = spawn_err(ractor::SpawnErr::ActorAlreadyStarted);
        assert!(e.to_string().contains("failed to spawn actor"));
    }

    // -----------------------------------------------------------------------
    // InferenceSchedule tests
    // -----------------------------------------------------------------------

    #[test]
    fn inference_schedule_immediate_for_trivial() {
        use crate::model::Domain;
        use crate::parse::Parse;
        use crate::Vector;
        let source = "grammar @simple {\n  type = a | b\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let domain = Domain::from_grammar(grammar).unwrap();
        let verified = check::verify(domain).unwrap();
        let schedule = InferenceSchedule::from_verified(&verified);
        assert!(matches!(schedule, InferenceSchedule::Immediate));
    }

    #[test]
    fn inference_schedule_diffusion_for_spectrum() {
        use crate::model::Domain;
        use crate::parse::Parse;
        use crate::Vector;
        let source =
            "grammar @linked {\n  type color = red | blue\n  type pair = combo(color)\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let domain = Domain::from_grammar(grammar).unwrap();
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
        use crate::model::Domain;
        use crate::parse::Parse;
        use crate::Vector;
        let source =
            "grammar @linked {\n  type color = red | blue\n  type pair = combo(color)\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .unwrap();
        let domain = Domain::from_grammar(grammar).unwrap();
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
