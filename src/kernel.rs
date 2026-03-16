//! The kernel. Content-addressed vector algebra.
//!
//! Everything conversation needs to transform, compose, and address.

use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;

use sha2::{Digest, Sha256};

// ---------------------------------------------------------------------------
// Oid — content address value type
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Oid(String);

impl Oid {
    pub fn new(hash: impl Into<String>) -> Self {
        Oid(hash.into())
    }
}

impl AsRef<str> for Oid {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Oid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// TraceOid — trace-specific content address
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TraceOid(Oid);

impl TraceOid {
    pub fn new(hash: impl Into<String>) -> Self {
        TraceOid(Oid::new(hash))
    }

    pub fn as_oid(&self) -> &Oid {
        &self.0
    }
}

impl From<Oid> for TraceOid {
    fn from(oid: Oid) -> Self {
        TraceOid(oid)
    }
}

impl AsRef<str> for TraceOid {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl std::fmt::Display for TraceOid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// ContentAddressed — hash identity trait
// ---------------------------------------------------------------------------

pub trait ContentAddressed {
    type Oid: Clone
        + PartialEq
        + Eq
        + std::hash::Hash
        + std::fmt::Debug
        + std::fmt::Display
        + AsRef<str>
        + Into<TraceOid>;
    fn content_oid(&self) -> Self::Oid;
}

// ---------------------------------------------------------------------------
// domain_oid! — OID newtype generator
// ---------------------------------------------------------------------------

#[macro_export]
macro_rules! domain_oid {
    ($(#[$meta:meta])* $vis:vis $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        $vis struct $name($crate::Oid);

        impl $name {
            pub fn new(hash: impl Into<String>) -> Self {
                Self($crate::Oid::new(hash))
            }
        }

        impl From<$crate::Oid> for $name {
            fn from(oid: $crate::Oid) -> Self {
                Self(oid)
            }
        }

        impl From<$name> for $crate::TraceOid {
            fn from(oid: $name) -> Self {
                $crate::TraceOid::from(oid.0)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.0.as_ref()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

// ---------------------------------------------------------------------------
// Trace — origin, destination, payload
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Trace<T, E> {
    result: Result<T, E>,
    oid: TraceOid,
    parent: Option<TraceOid>,
}

impl<T, E> Trace<T, E> {
    pub fn success(value: T, oid: TraceOid, parent: Option<TraceOid>) -> Self {
        Trace {
            result: Ok(value),
            oid,
            parent,
        }
    }

    pub fn failure(error: E, oid: TraceOid, parent: Option<TraceOid>) -> Self {
        Trace {
            result: Err(error),
            oid,
            parent,
        }
    }

    pub fn oid(&self) -> &TraceOid {
        &self.oid
    }

    pub fn parent(&self) -> Option<&TraceOid> {
        self.parent.as_ref()
    }

    pub fn is_ok(&self) -> bool {
        self.result.is_ok()
    }

    pub fn is_err(&self) -> bool {
        self.result.is_err()
    }

    pub fn into_result(self) -> Result<T, E> {
        self.result
    }

    pub fn unwrap(self) -> T
    where
        E: std::fmt::Debug,
    {
        self.result.unwrap()
    }
}

// ---------------------------------------------------------------------------
// ContentAddressed impls for standard types
// ---------------------------------------------------------------------------

impl ContentAddressed for String {
    type Oid = Oid;
    fn content_oid(&self) -> Oid {
        let mut hasher = Sha256::new();
        hasher.update(self.as_bytes());
        Oid::new(hex::encode(hasher.finalize()))
    }
}

impl<A: ContentAddressed> ContentAddressed for Vec<A> {
    type Oid = Oid;
    fn content_oid(&self) -> Oid {
        let mut hasher = Sha256::new();
        for item in self {
            hasher.update(item.content_oid().as_ref().as_bytes());
        }
        Oid::new(hex::encode(hasher.finalize()))
    }
}

impl<A: ContentAddressed> ContentAddressed for Option<A> {
    type Oid = Oid;
    fn content_oid(&self) -> Oid {
        let mut hasher = Sha256::new();
        match self {
            Some(inner) => {
                hasher.update(b"some:");
                hasher.update(inner.content_oid().as_ref().as_bytes());
            }
            None => {
                hasher.update(b"none");
            }
        }
        Oid::new(hex::encode(hasher.finalize()))
    }
}

impl<A: ContentAddressed, B: ContentAddressed> ContentAddressed for (A, B) {
    type Oid = Oid;
    fn content_oid(&self) -> Oid {
        let mut hasher = Sha256::new();
        hasher.update(self.0.content_oid().as_ref().as_bytes());
        hasher.update(self.1.content_oid().as_ref().as_bytes());
        Oid::new(hex::encode(hasher.finalize()))
    }
}

impl ContentAddressed for i32 {
    type Oid = Oid;
    fn content_oid(&self) -> Oid {
        let mut hasher = Sha256::new();
        hasher.update(self.to_le_bytes());
        Oid::new(hex::encode(hasher.finalize()))
    }
}

impl ContentAddressed for f64 {
    type Oid = Oid;
    fn content_oid(&self) -> Oid {
        let mut hasher = Sha256::new();
        hasher.update(self.to_le_bytes());
        Oid::new(hex::encode(hasher.finalize()))
    }
}

impl<E: fragmentation::encoding::Encode> ContentAddressed for fragmentation::fragment::Fractal<E> {
    type Oid = Oid;
    fn content_oid(&self) -> Oid {
        Oid::new(fragmentation::fragment::content_oid(self))
    }
}

impl<E: fragmentation::encoding::Encode> ContentAddressed for crate::tree::Tree<E> {
    type Oid = Oid;
    fn content_oid(&self) -> Oid {
        Oid::new(fragmentation::fragment::content_oid(self))
    }
}

impl ContentAddressed for serde_json::Value {
    type Oid = Oid;
    fn content_oid(&self) -> Oid {
        let mut hasher = Sha256::new();
        hasher.update(self.to_string().as_bytes());
        Oid::new(hex::encode(hasher.finalize()))
    }
}

// ---------------------------------------------------------------------------
// Vector — transformation between content-addressed spaces
// ---------------------------------------------------------------------------

pub trait Vector<A, B: ContentAddressed> {
    type Error;

    fn trace(&self, source: A) -> Trace<B, Self::Error>;

    fn compose<C: ContentAddressed, G: Vector<B, C>>(self, other: G) -> Composed<Self, G, B>
    where
        Self: Sized,
    {
        Composed(self, other, PhantomData)
    }
}

// ---------------------------------------------------------------------------
// Composed — vector pipeline
// ---------------------------------------------------------------------------

pub struct Composed<F, G, Mid>(pub F, pub G, PhantomData<Mid>);

#[derive(Debug, PartialEq)]
pub enum ComposedError<E1, E2> {
    First(E1),
    Second(E2),
}

impl<E1: std::fmt::Display, E2: std::fmt::Display> std::fmt::Display for ComposedError<E1, E2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComposedError::First(e) => write!(f, "{e}"),
            ComposedError::Second(e) => write!(f, "{e}"),
        }
    }
}

impl<E1: std::error::Error, E2: std::error::Error> std::error::Error for ComposedError<E1, E2> {}

impl<A, C, Mid, F, G> Vector<A, C> for Composed<F, G, Mid>
where
    C: ContentAddressed,
    Mid: ContentAddressed,
    F: Vector<A, Mid>,
    G: Vector<Mid, C>,
{
    type Error = ComposedError<F::Error, G::Error>;

    fn trace(&self, source: A) -> Trace<C, Self::Error> {
        let first = self.0.trace(source);
        let first_oid = first.oid().clone();
        match first.into_result() {
            Err(e) => Trace::failure(ComposedError::First(e), first_oid, None),
            Ok(mid) => {
                let second = self.1.trace(mid);
                let second_oid = second.oid().clone();
                match second.into_result() {
                    Ok(value) => Trace::success(value, second_oid, Some(first_oid)),
                    Err(e) => Trace::failure(ComposedError::Second(e), second_oid, Some(first_oid)),
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Latent — deferred + cached vector evaluation
// ---------------------------------------------------------------------------

pub struct Latent<V, B, E> {
    inner: V,
    cache: RefCell<HashMap<String, Trace<B, E>>>,
}

impl<V, B: ContentAddressed, E> Latent<V, B, E> {
    pub fn new(inner: V) -> Self {
        Latent {
            inner,
            cache: RefCell::new(HashMap::new()),
        }
    }
}

impl<A, B, V> Vector<A, B> for Latent<V, B, V::Error>
where
    A: ContentAddressed,
    B: ContentAddressed + Clone,
    V: Vector<A, B>,
    V::Error: Clone,
{
    type Error = V::Error;

    fn trace(&self, source: A) -> Trace<B, Self::Error> {
        let key = source.content_oid().as_ref().to_string();
        if let Some(cached) = self.cache.borrow().get(&key) {
            return cached.clone();
        }
        let result = self.inner.trace(source);
        self.cache.borrow_mut().insert(key, result.clone());
        result
    }
}

// ---------------------------------------------------------------------------
// Setting — domain context
// ---------------------------------------------------------------------------

pub trait Setting: Clone + std::fmt::Debug + PartialEq + Eq {
    type Token: Clone + std::fmt::Debug + PartialEq + Eq + ContentAddressed;

    fn id() -> &'static str;
}

// ---------------------------------------------------------------------------
// Addressable — tree node interface
// ---------------------------------------------------------------------------

pub trait Addressable {
    fn node_name(&self) -> &str;
    fn node_content(&self) -> Option<&str>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use fragmentation::fragment::Fractal;
    use std::cell::Cell;
    use std::rc::Rc;

    // -- Oid --

    #[test]
    fn oid_construction_and_display() {
        let oid = Oid::new("abc123");
        assert_eq!(oid.to_string(), "abc123");
        assert_eq!(oid.as_ref(), "abc123");
    }

    #[test]
    fn oid_equality() {
        let a = Oid::new("same");
        let b = Oid::new("same");
        let c = Oid::new("diff");
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn oid_ordering() {
        let a = Oid::new("aaa");
        let b = Oid::new("bbb");
        assert!(a < b);
    }

    #[test]
    fn oid_clone() {
        let a = Oid::new("x");
        let b = a.clone();
        assert_eq!(a, b);
    }

    // -- TraceOid --

    #[test]
    fn trace_oid_construction_and_display() {
        let oid = TraceOid::new("abc");
        assert_eq!(oid.to_string(), "abc");
        assert_eq!(oid.as_ref(), "abc");
    }

    #[test]
    fn trace_oid_from_oid() {
        let oid = Oid::new("hash");
        let trace_oid = TraceOid::from(oid.clone());
        assert_eq!(trace_oid.as_oid(), &oid);
    }

    #[test]
    fn trace_oid_equality() {
        let a = TraceOid::new("same");
        let b = TraceOid::new("same");
        assert_eq!(a, b);
    }

    #[test]
    fn trace_oid_clone() {
        let a = TraceOid::new("x");
        let b = a.clone();
        assert_eq!(a, b);
    }

    // -- Trace --

    #[test]
    fn trace_success() {
        let t: Trace<String, String> = Trace::success("hello".into(), TraceOid::new("oid"), None);
        assert!(t.is_ok());
        assert!(!t.is_err());
        assert_eq!(t.oid(), &TraceOid::new("oid"));
        assert_eq!(t.parent(), None);
        assert_eq!(t.unwrap(), "hello");
    }

    #[test]
    fn trace_failure() {
        let t: Trace<String, String> = Trace::failure("boom".into(), TraceOid::new("err"), None);
        assert!(t.is_err());
        assert!(!t.is_ok());
        assert_eq!(t.into_result(), Err("boom".into()));
    }

    #[test]
    fn trace_with_parent() {
        let parent = TraceOid::new("parent");
        let t: Trace<i32, String> =
            Trace::success(42, TraceOid::new("child"), Some(parent.clone()));
        assert_eq!(t.parent(), Some(&parent));
    }

    #[test]
    #[should_panic]
    fn trace_unwrap_panics_on_error() {
        let t: Trace<String, String> = Trace::failure("boom".into(), TraceOid::new("err"), None);
        t.unwrap();
    }

    // -- ContentAddressed impls --

    #[test]
    fn content_addressed_string() {
        let a = "hello".to_string();
        let b = "hello".to_string();
        let c = "world".to_string();
        assert_eq!(a.content_oid(), b.content_oid());
        assert_ne!(a.content_oid(), c.content_oid());
    }

    #[test]
    fn content_addressed_vec() {
        let a: Vec<i32> = vec![1, 2, 3];
        let b: Vec<i32> = vec![1, 2, 3];
        let c: Vec<i32> = vec![4, 5, 6];
        assert_eq!(a.content_oid(), b.content_oid());
        assert_ne!(a.content_oid(), c.content_oid());
    }

    #[test]
    fn content_addressed_option() {
        let some_a: Option<String> = Some("x".into());
        let some_b: Option<String> = Some("x".into());
        let some_c: Option<String> = Some("y".into());
        let none: Option<String> = None;
        assert_eq!(some_a.content_oid(), some_b.content_oid());
        assert_ne!(some_a.content_oid(), some_c.content_oid());
        assert_ne!(some_a.content_oid(), none.content_oid());
    }

    #[test]
    fn content_addressed_tuple() {
        let a = ("x".to_string(), "y".to_string());
        let b = ("x".to_string(), "y".to_string());
        let c = ("x".to_string(), "z".to_string());
        assert_eq!(a.content_oid(), b.content_oid());
        assert_ne!(a.content_oid(), c.content_oid());
    }

    #[test]
    fn content_addressed_i32() {
        assert_eq!(42i32.content_oid(), 42i32.content_oid());
        assert_ne!(42i32.content_oid(), 43i32.content_oid());
    }

    #[test]
    fn content_addressed_f64() {
        assert_eq!(3.14f64.content_oid(), 3.14f64.content_oid());
        assert_ne!(3.14f64.content_oid(), 2.71f64.content_oid());
    }

    #[test]
    fn content_addressed_value() {
        let a = serde_json::json!({"key": "val"});
        let b = serde_json::json!({"key": "val"});
        let c = serde_json::json!({"key": "other"});
        assert_eq!(a.content_oid(), b.content_oid());
        assert_ne!(a.content_oid(), c.content_oid());
    }

    #[test]
    fn content_addressed_fractal() {
        use fragmentation::ref_::Ref;
        use fragmentation::sha;
        let ref_a = Ref::new(sha::hash("a"), "a");
        let ref_b = Ref::new(sha::hash("b"), "b");
        let a: Fractal<String> = Fractal::shard_typed(ref_a.clone(), "same".into());
        let b: Fractal<String> = Fractal::shard_typed(ref_b, "same".into());
        let c: Fractal<String> = Fractal::shard_typed(ref_a, "diff".into());
        assert_eq!(a.content_oid(), b.content_oid());
        assert_ne!(a.content_oid(), c.content_oid());
    }

    // -- Vector + Composed --
    // Single monomorphization: Composed<FailIf42, FailAbove100, i32>
    // Both steps can fail, so all 4 branches of Composed::trace() are reachable.

    #[derive(Clone)]
    struct FailIf42;

    impl Vector<i32, i32> for FailIf42 {
        type Error = String;
        fn trace(&self, source: i32) -> Trace<i32, String> {
            if source == 42 {
                Trace::failure("is 42".into(), TraceOid::new("err"), None)
            } else {
                Trace::success(source, TraceOid::new(format!("{}", source)), None)
            }
        }
    }

    #[derive(Clone)]
    struct FailAbove100;

    impl Vector<i32, i32> for FailAbove100 {
        type Error = String;
        fn trace(&self, source: i32) -> Trace<i32, String> {
            if source > 100 {
                Trace::failure("too big".into(), TraceOid::new("err"), None)
            } else {
                Trace::success(source, TraceOid::new(format!("{}", source)), None)
            }
        }
    }

    fn pipeline() -> Composed<FailIf42, FailAbove100, i32> {
        FailIf42.compose(FailAbove100)
    }

    #[test]
    fn vector_compose_chain() {
        assert_eq!(pipeline().trace(5).unwrap(), 5);
    }

    #[test]
    fn vector_compose_parent_link() {
        let t = pipeline().trace(5);
        assert!(t.parent().is_some());
    }

    #[test]
    fn composed_first_error() {
        let t = pipeline().trace(42); // FailIf42 rejects
        assert!(t.is_err());
        assert!(matches!(t.into_result(), Err(ComposedError::First(_))));
    }

    #[test]
    fn composed_second_error() {
        let t = pipeline().trace(200); // FailIf42 passes, FailAbove100 rejects
        assert!(t.is_err());
        assert!(matches!(t.into_result(), Err(ComposedError::Second(_))));
    }

    #[test]
    fn composed_error_display() {
        let first: ComposedError<String, String> = ComposedError::First("parse failed".into());
        let second: ComposedError<String, String> = ComposedError::Second("resolve failed".into());
        assert_eq!(format!("{}", first), "parse failed");
        assert_eq!(format!("{}", second), "resolve failed");
    }

    // -- domain_oid! coverage: exercise From<Oid>, AsRef, Display --

    #[test]
    fn domain_oid_full_coverage() {
        use crate::ast::AstOid;
        let base = Oid::new("abc");
        let ast = AstOid::from(base); // From<Oid>
        assert_eq!(ast.as_ref(), "abc"); // AsRef<str>
        assert_eq!(ast.to_string(), "abc"); // Display
    }

    // -- Sentinel: SHA-256 hex pinning --

    #[test]
    fn sha256_hex_sentinel() {
        // Pin the exact hash for "hello" to detect algorithm drift
        let oid = "hello".to_string().content_oid();
        assert_eq!(
            oid.as_ref(),
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    // -- Latent --
    // Single monomorphization: Latent<TrackedFailIf42, i32, String>
    // TrackedFailIf42 counts invocations and can succeed or fail.

    struct TrackedFailIf42(Rc<Cell<usize>>);

    impl Vector<i32, i32> for TrackedFailIf42 {
        type Error = String;
        fn trace(&self, source: i32) -> Trace<i32, String> {
            self.0.set(self.0.get() + 1);
            if source == 42 {
                Trace::failure("is 42".into(), TraceOid::new("err"), None)
            } else {
                Trace::success(source, TraceOid::new(format!("{}", source)), None)
            }
        }
    }

    fn tracked() -> (Latent<TrackedFailIf42, i32, String>, Rc<Cell<usize>>) {
        let counter = Rc::new(Cell::new(0));
        (Latent::new(TrackedFailIf42(counter.clone())), counter)
    }

    #[test]
    fn latent_cache_miss() {
        let (latent, counter) = tracked();
        let t = latent.trace(5);
        assert_eq!(counter.get(), 1);
        assert_eq!(t.unwrap(), 5);
    }

    #[test]
    fn latent_cache_hit() {
        let (latent, counter) = tracked();
        let _ = latent.trace(5);
        let t = latent.trace(5);
        assert_eq!(counter.get(), 1); // NOT 2 — cached
        assert_eq!(t.unwrap(), 5);
    }

    #[test]
    fn latent_different_inputs() {
        let (latent, counter) = tracked();
        latent.trace(5);
        latent.trace(10);
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn latent_caches_failure() {
        let (latent, counter) = tracked();
        let t1 = latent.trace(42);
        let t2 = latent.trace(42);
        assert_eq!(counter.get(), 1); // NOT 2 — cached
        assert!(t1.is_err());
        assert!(t2.is_err());
    }

    #[test]
    fn latent_compose() {
        let (l1, c1) = tracked();
        let (l2, c2) = tracked();
        let composed = l1.compose(l2);
        let t = composed.trace(5);
        assert_eq!(t.unwrap(), 5);
        assert_eq!(c1.get(), 1);
        assert_eq!(c2.get(), 1);
    }
}
