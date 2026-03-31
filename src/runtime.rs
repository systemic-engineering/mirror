//! Runtime trait + typed Args/Response/Value.
//!
//! Defines the async `Runtime` trait for domain lifecycle management and
//! dispatch, plus the typed `Args`, `Response`, and `Value` enums.
//!
//! `RactorRuntime` is the concrete implementation, with actor support for
//! domains that have `in @actor` via ractor.

use std::collections::{BTreeMap, HashMap};
use std::fmt;

use ractor::{Actor, ActorProcessingErr, ActorRef};

use crate::check::{self, Evidence, PropertyKind, PropertyViolation, Verified, Violations};
use crate::model::{ActionName, Domain, DomainName, PropertyName, TypeName};
use crate::Oid;

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

impl RuntimeError {
    fn new(msg: impl Into<String>) -> Self {
        Self(msg.into())
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ---------------------------------------------------------------------------
// Runtime trait
// ---------------------------------------------------------------------------

/// Async lifecycle + dispatch interface for domain runtimes.
#[allow(async_fn_in_trait)]
pub trait Runtime: Send + Sync {
    type Error: fmt::Display + Send;

    /// Register a verified domain with the runtime.
    async fn register(&mut self, domain: &Verified) -> Result<(), Self::Error>;

    /// Dispatch an action call into a registered domain.
    async fn dispatch(
        &self,
        domain: &DomainName,
        action: &ActionName,
        args: Args,
    ) -> Result<Response, Self::Error>;

    /// Re-run `ensures` checks against a running domain.
    async fn check_ensures(&self, domain: &DomainName) -> Result<Verified, Violations>;

    /// Shut down and deregister a domain.
    async fn shutdown(&mut self, domain: &DomainName) -> Result<(), Self::Error>;
}

// ---------------------------------------------------------------------------
// DomainActor
// ---------------------------------------------------------------------------

/// Message type for actor-backed domains.
///
/// `Dispatch(action, args, reply)` — tuple variant so `ractor::call!` can
/// construct it as `Dispatch(action, args, tx)`.
pub(crate) enum DomainMessage {
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

/// Runtime implementation with actor support for `in @actor` domains.
pub struct RactorRuntime {
    pub(crate) domains: HashMap<String, Domain>,
    pub(crate) actors: HashMap<String, ActorRef<DomainMessage>>,
}

impl RactorRuntime {
    /// Create an empty runtime with no registered domains.
    pub fn new() -> Self {
        Self {
            domains: HashMap::new(),
            actors: HashMap::new(),
        }
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

/// Convert a ractor call error into a RuntimeError.
/// Named function to avoid inline-closure monomorphization coverage issues.
fn call_err(e: ractor::RactorErr<DomainMessage>) -> RuntimeError {
    RuntimeError(format!("actor call failed: {}", e))
}

impl Runtime for RactorRuntime {
    type Error = RuntimeError;

    async fn register(&mut self, domain: &Verified) -> Result<(), RuntimeError> {
        let d = domain.domain();
        if self.domains.contains_key(d.name.as_str()) {
            return Err(RuntimeError(format!(
                "domain '{}' already registered",
                d.name
            )));
        }
        self.domains.insert(d.name.as_str().to_owned(), d.clone());

        if d.is_actor() {
            let (actor_ref, _handle) = Actor::spawn(None, DomainActor, d.clone())
                .await
                .map_err(spawn_err)?;
            self.actors.insert(d.name.as_str().to_string(), actor_ref);
        }

        Ok(())
    }

    async fn dispatch(
        &self,
        domain: &DomainName,
        action: &ActionName,
        args: Args,
    ) -> Result<Response, RuntimeError> {
        let d = self
            .domains
            .get(domain.as_str())
            .ok_or_else(|| RuntimeError::new(format!("domain not registered: @{}", domain)))?;

        if let Some(actor_ref) = self.actors.get(domain.as_str()) {
            let result = ractor::call!(actor_ref, DomainMessage::Dispatch, action.clone(), args)
                .map_err(call_err)?;
            return result.map_err(RuntimeError);
        }

        let action_exists = d.actions.iter().any(|a| &a.name == action);
        if !action_exists {
            return Err(RuntimeError::new(format!(
                "unknown action '{}' in domain @{}",
                action, domain
            )));
        }

        Ok(Response::Ok(Value::Text(format!("{}:{}", domain, action))))
    }

    async fn check_ensures(&self, domain: &DomainName) -> Result<Verified, Violations> {
        let d = self
            .domains
            .get(domain.as_str())
            .ok_or_else(|| Violations {
                domain: domain.clone(),
                violations: vec![PropertyViolation {
                    domain: domain.clone(),
                    property: PropertyName::new("ensures"),
                    kind: PropertyKind::Ensures,
                    reason: format!("domain not registered: @{}", domain),
                    evidence: Evidence::Unresolvable {
                        name: TypeName::new(domain.as_str()),
                        candidates: vec![],
                    },
                }],
            })?;

        check::verify(d.clone()).map_err(|mut v| {
            // Re-tag violations as Ensures kind.
            for violation in &mut v.violations {
                violation.kind = PropertyKind::Ensures;
            }
            v
        })
    }

    async fn shutdown(&mut self, domain: &DomainName) -> Result<(), RuntimeError> {
        if let Some(actor_ref) = self.actors.remove(domain.as_str()) {
            actor_ref.stop(None);
        }
        self.domains.remove(domain.as_str());
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Action, Domain, DomainName, Properties, TypeDef, Variant, VariantName};
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
            properties: Properties::empty(),
            registry: None,
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
        let rt = RactorRuntime::default();
        assert!(rt.domains.is_empty());
        assert!(rt.actors.is_empty());
    }

    #[tokio::test]
    async fn ractor_runtime_register_non_actor() {
        let mut rt = RactorRuntime::new();
        let verified = simple_verified();
        rt.register(&verified).await.unwrap();
        assert!(rt.domains.contains_key("color"));
    }

    #[tokio::test]
    async fn ractor_runtime_register_duplicate_errors() {
        let mut rt = RactorRuntime::new();
        let verified = simple_verified();
        rt.register(&verified).await.unwrap();
        let result = rt.register(&verified).await;
        assert!(result.is_err());
        let err = format!("{}", result.unwrap_err());
        assert!(
            err.contains("already registered"),
            "error should say already registered: {}",
            err
        );
    }

    #[tokio::test]
    async fn ractor_runtime_dispatch_non_actor() {
        let mut rt = RactorRuntime::new();
        let verified = simple_verified();
        rt.register(&verified).await.unwrap();

        let domain = DomainName::new("color");
        let action = ActionName::new("paint");
        let resp = rt.dispatch(&domain, &action, Args::Empty).await.unwrap();

        assert!(matches!(resp, Response::Ok(Value::Text(_))));
    }

    #[tokio::test]
    async fn ractor_runtime_dispatch_unknown_domain() {
        let rt = RactorRuntime::new();
        let domain = DomainName::new("nonexistent");
        let action = ActionName::new("paint");
        let err = rt
            .dispatch(&domain, &action, Args::Empty)
            .await
            .unwrap_err();
        assert!(err.to_string().contains("nonexistent"));
    }

    #[tokio::test]
    async fn ractor_runtime_dispatch_unknown_action() {
        let mut rt = RactorRuntime::new();
        let verified = simple_verified();
        rt.register(&verified).await.unwrap();

        let domain = DomainName::new("color");
        let action = ActionName::new("fly");
        let err = rt
            .dispatch(&domain, &action, Args::Empty)
            .await
            .unwrap_err();
        assert!(err.to_string().contains("fly"));
        assert!(err.to_string().contains("color"));
    }

    #[tokio::test]
    async fn ractor_runtime_shutdown() {
        let mut rt = RactorRuntime::new();
        let verified = simple_verified();
        rt.register(&verified).await.unwrap();
        assert!(rt.domains.contains_key("color"));

        rt.shutdown(&DomainName::new("color")).await.unwrap();
        assert!(!rt.domains.contains_key("color"));
    }

    #[tokio::test]
    async fn ractor_runtime_check_ensures_unknown_domain() {
        let rt = RactorRuntime::new();
        let domain = DomainName::new("ghost");
        let err = rt.check_ensures(&domain).await.unwrap_err();
        assert_eq!(err.violations.len(), 1);
        assert!(err.violations[0].reason.contains("ghost"));
    }

    #[tokio::test]
    async fn ractor_runtime_check_ensures_registered_domain() {
        let mut rt = RactorRuntime::new();
        let verified = simple_verified();
        rt.register(&verified).await.unwrap();

        let domain = DomainName::new("color");
        // simple_verified() has no properties → verify() passes → Ok(Verified)
        let result = rt.check_ensures(&domain).await;
        assert!(result.is_ok());
    }

    // -----------------------------------------------------------------------
    // Error conversion helpers — coverage for spawn_err / call_err
    // -----------------------------------------------------------------------

    #[test]
    fn spawn_err_formats_message() {
        let e = spawn_err(ractor::SpawnErr::ActorAlreadyStarted);
        assert!(e.to_string().contains("failed to spawn actor"));
    }

    #[test]
    fn call_err_formats_message() {
        let e = call_err(ractor::RactorErr::Timeout);
        assert!(e.to_string().contains("actor call failed"));
    }

    // -----------------------------------------------------------------------
    // Actor domain helpers + tests
    // -----------------------------------------------------------------------

    /// Build a verified actor domain:
    ///   in @actor
    ///   type target = rust | beam
    ///   action compile
    fn actor_verified() -> check::Verified {
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
            properties: Properties::empty(),
            registry: None,
        };
        check::verify(domain).unwrap()
    }

    #[tokio::test]
    async fn ractor_runtime_register_actor_domain() {
        let mut rt = RactorRuntime::new();
        let verified = actor_verified();
        rt.register(&verified).await.unwrap();
        assert!(rt.domains.contains_key("compiler"));
        assert!(rt.actors.contains_key("compiler"));
    }

    #[tokio::test]
    async fn ractor_runtime_dispatch_actor_domain() {
        let mut rt = RactorRuntime::new();
        let verified = actor_verified();
        rt.register(&verified).await.unwrap();
        let resp = rt
            .dispatch(
                &DomainName::new("compiler"),
                &ActionName::new("compile"),
                Args::Single(Value::Text("test.conv".into())),
            )
            .await
            .unwrap();
        assert!(matches!(resp, Response::Ok(_)));
    }

    #[tokio::test]
    async fn ractor_runtime_dispatch_actor_unknown_action() {
        let mut rt = RactorRuntime::new();
        let verified = actor_verified();
        rt.register(&verified).await.unwrap();
        let err = rt
            .dispatch(
                &DomainName::new("compiler"),
                &ActionName::new("nonexistent"),
                Args::Empty,
            )
            .await
            .unwrap_err();
        assert!(err.to_string().contains("nonexistent"));
        assert!(err.to_string().contains("compiler"));
    }

    #[tokio::test]
    async fn ractor_runtime_shutdown_actor() {
        let mut rt = RactorRuntime::new();
        let verified = actor_verified();
        rt.register(&verified).await.unwrap();
        assert!(rt.actors.contains_key("compiler"));
        rt.shutdown(&DomainName::new("compiler")).await.unwrap();
        assert!(!rt.actors.contains_key("compiler"));
        assert!(!rt.domains.contains_key("compiler"));
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
        match &schedule {
            InferenceSchedule::Diffusion(ev) => {
                assert!(ev.fiedler_value().unwrap() > 0.0);
            }
            InferenceSchedule::Immediate => panic!("expected Diffusion"),
        }
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
            t_full >= t_half,
            "full {} should >= half {}",
            t_full,
            t_half
        );
        assert!(
            t_half >= t_zero,
            "half {} should >= zero {}",
            t_half,
            t_zero
        );
    }

    #[test]
    fn schedule_immediate_always_zero_temperature() {
        let schedule = InferenceSchedule::Immediate;
        assert_eq!(schedule.temperature(0.0), 0.0);
        assert_eq!(schedule.temperature(0.5), 0.0);
        assert_eq!(schedule.temperature(1.0), 0.0);
    }

    #[tokio::test]
    async fn ractor_runtime_check_ensures_retags_violations_as_ensures() {
        use crate::model::{Properties, PropertyName, TypeDef, TypeName, Variant, VariantName};

        // Build a domain that will fail check::verify: two disconnected types,
        // with "requires connected". We insert it directly into the runtime
        // bypassing the Verified wrapper to simulate a stale registration.
        let color = TypeDef {
            name: TypeName::new("color"),
            variants: vec![Variant {
                name: VariantName::new("red"),
                params: vec![],
            }],
        };
        let shape = TypeDef {
            name: TypeName::new("shape"),
            variants: vec![Variant {
                name: VariantName::new("circle"),
                params: vec![],
            }],
        };
        let bad_domain = Domain {
            name: DomainName::new("broken"),
            types: vec![color, shape],
            actions: vec![],
            lenses: vec![],
            properties: Properties {
                requires: vec![PropertyName::new("connected")],
                invariants: vec![],
                ensures: vec![],
            },
            registry: None,
        };

        let mut rt = RactorRuntime::new();
        rt.domains.insert("broken".to_owned(), bad_domain);

        let domain = DomainName::new("broken");
        let err = rt.check_ensures(&domain).await.unwrap_err();
        assert!(!err.violations.is_empty());
        // All violations should be re-tagged as Ensures.
        for v in &err.violations {
            assert!(matches!(v.kind, PropertyKind::Ensures));
        }
    }
}
