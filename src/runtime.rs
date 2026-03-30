//! Runtime trait + typed Args/Response/Value.
//!
//! Defines the async `Runtime` trait for domain lifecycle management and
//! dispatch, plus the typed `Args`, `Response`, and `Value` enums.
//!
//! `RactorRuntime` is the concrete implementation. This module provides the
//! non-actor path only — actor support is added in the next task.

use std::collections::{BTreeMap, HashMap};
use std::fmt;

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
// RactorRuntime
// ---------------------------------------------------------------------------

/// Non-actor runtime implementation (actor support added in Task 5).
pub struct RactorRuntime {
    pub(crate) domains: HashMap<String, Domain>,
}

impl RactorRuntime {
    /// Create an empty runtime with no registered domains.
    pub fn new() -> Self {
        Self {
            domains: HashMap::new(),
        }
    }
}

impl Default for RactorRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime for RactorRuntime {
    type Error = RuntimeError;

    async fn register(&mut self, _domain: &Verified) -> Result<(), RuntimeError> {
        todo!("register not implemented")
    }

    async fn dispatch(
        &self,
        _domain: &DomainName,
        _action: &ActionName,
        _args: Args,
    ) -> Result<Response, RuntimeError> {
        todo!("dispatch not implemented")
    }

    async fn check_ensures(&self, domain: &DomainName) -> Result<Verified, Violations> {
        Err(Violations {
            domain: domain.clone(),
            violations: vec![],
        })
    }

    async fn shutdown(&mut self, _domain: &DomainName) -> Result<(), RuntimeError> {
        todo!("shutdown not implemented")
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
        let args = Args::Single(Value::Text("hello".to_owned()));
        match args {
            Args::Single(Value::Text(s)) => assert_eq!(s, "hello"),
            _ => panic!("expected Args::Single(Value::Text)"),
        }
    }

    #[test]
    fn args_named() {
        let mut map = BTreeMap::new();
        map.insert(ActionName::new("input"), Value::Text("x".to_owned()));
        let args = Args::Named(map);
        match args {
            Args::Named(m) => {
                assert_eq!(m.len(), 1);
                assert!(matches!(
                    m.get(&ActionName::new("input")),
                    Some(Value::Text(_))
                ));
            }
            _ => panic!("expected Args::Named"),
        }
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
        match r {
            Response::Error(msg) => assert!(msg.contains("wrong")),
            _ => panic!("expected Response::Error"),
        }
    }

    // -----------------------------------------------------------------------
    // RactorRuntime tests (will fail until implemented)
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn ractor_runtime_register_non_actor() {
        let mut rt = RactorRuntime::new();
        let verified = simple_verified();
        rt.register(&verified).await.unwrap();
        assert!(rt.domains.contains_key("color"));
    }

    #[tokio::test]
    async fn ractor_runtime_dispatch_non_actor() {
        let mut rt = RactorRuntime::new();
        let verified = simple_verified();
        rt.register(&verified).await.unwrap();

        let domain = DomainName::new("color");
        let action = ActionName::new("paint");
        let resp = rt.dispatch(&domain, &action, Args::Empty).await.unwrap();

        match resp {
            Response::Ok(Value::Text(s)) => {
                assert_eq!(s, "color:paint");
            }
            other => panic!("expected Ok(Text), got {:?}", other),
        }
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
}
