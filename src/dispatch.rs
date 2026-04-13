//! Dispatch types — Value, Args, Response for action dispatch.
//!
//! These types are used by both mirror (compile-time) and conversation (runtime).

use std::collections::BTreeMap;

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
    Named(BTreeMap<String, Value>),
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
        map.insert("input".to_owned(), Value::Text("x".to_owned()));
        let args = Args::Named(map.clone());
        assert!(matches!(args, Args::Named(_)));
        assert_eq!(map.len(), 1);
        assert!(matches!(map.get("input"), Some(Value::Text(_))));
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
}
