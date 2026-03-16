//! The kernel. Content-addressed transformations, inlined from story.
//!
//! Everything conversation needs to transform, compose, and address.
//! ~255 lines that replace 5249 lines of story dependency.

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
// CutOid — cut-specific content address
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CutOid(Oid);

impl CutOid {
    pub fn new(hash: impl Into<String>) -> Self {
        CutOid(Oid::new(hash))
    }

    pub fn as_oid(&self) -> &Oid {
        &self.0
    }
}

impl From<Oid> for CutOid {
    fn from(oid: Oid) -> Self {
        CutOid(oid)
    }
}

impl AsRef<str> for CutOid {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl std::fmt::Display for CutOid {
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
        + Into<CutOid>;
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

        impl From<$name> for $crate::CutOid {
            fn from(oid: $name) -> Self {
                $crate::CutOid::from(oid.0)
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
// Cut — transformation result with content address
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Cut<T, E> {
    result: Result<T, E>,
    oid: CutOid,
    parent: Option<CutOid>,
}

impl<T, E> Cut<T, E> {
    pub fn success(value: T, oid: CutOid, parent: Option<CutOid>) -> Self {
        Cut {
            result: Ok(value),
            oid,
            parent,
        }
    }

    pub fn failure(error: E, oid: CutOid, parent: Option<CutOid>) -> Self {
        Cut {
            result: Err(error),
            oid,
            parent,
        }
    }

    pub fn oid(&self) -> &CutOid {
        &self.oid
    }

    pub fn parent(&self) -> Option<&CutOid> {
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

impl ContentAddressed for serde_json::Value {
    type Oid = Oid;
    fn content_oid(&self) -> Oid {
        let mut hasher = Sha256::new();
        hasher.update(self.to_string().as_bytes());
        Oid::new(hex::encode(hasher.finalize()))
    }
}

// ---------------------------------------------------------------------------
// Story — transformation contract
// ---------------------------------------------------------------------------

pub trait Story<A, B: ContentAddressed> {
    type Error;

    fn record(&self, source: A) -> Cut<B, Self::Error>;

    fn compose<C: ContentAddressed, G: Story<B, C>>(self, other: G) -> Composed<Self, G, B>
    where
        Self: Sized,
    {
        Composed(self, other, PhantomData)
    }
}

// ---------------------------------------------------------------------------
// Composed — story pipeline
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

impl<A, C, Mid, F, G> Story<A, C> for Composed<F, G, Mid>
where
    C: ContentAddressed,
    Mid: ContentAddressed,
    F: Story<A, Mid>,
    G: Story<Mid, C>,
{
    type Error = ComposedError<F::Error, G::Error>;

    fn record(&self, source: A) -> Cut<C, Self::Error> {
        let first = self.0.record(source);
        let first_oid = first.oid().clone();
        match first.into_result() {
            Err(e) => Cut::failure(ComposedError::First(e), first_oid, None),
            Ok(mid) => {
                let second = self.1.record(mid);
                let second_oid = second.oid().clone();
                match second.into_result() {
                    Ok(value) => Cut::success(value, second_oid, Some(first_oid)),
                    Err(e) => {
                        Cut::failure(ComposedError::Second(e), second_oid, Some(first_oid))
                    }
                }
            }
        }
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

    // -- CutOid --

    #[test]
    fn cut_oid_construction_and_display() {
        let oid = CutOid::new("abc");
        assert_eq!(oid.to_string(), "abc");
        assert_eq!(oid.as_ref(), "abc");
    }

    #[test]
    fn cut_oid_from_oid() {
        let oid = Oid::new("hash");
        let cut_oid = CutOid::from(oid.clone());
        assert_eq!(cut_oid.as_oid(), &oid);
    }

    #[test]
    fn cut_oid_equality() {
        let a = CutOid::new("same");
        let b = CutOid::new("same");
        assert_eq!(a, b);
    }

    #[test]
    fn cut_oid_clone() {
        let a = CutOid::new("x");
        let b = a.clone();
        assert_eq!(a, b);
    }

    // -- Cut --

    #[test]
    fn cut_success() {
        let cut: Cut<String, String> = Cut::success("hello".into(), CutOid::new("oid"), None);
        assert!(cut.is_ok());
        assert!(!cut.is_err());
        assert_eq!(cut.oid(), &CutOid::new("oid"));
        assert_eq!(cut.parent(), None);
        assert_eq!(cut.unwrap(), "hello");
    }

    #[test]
    fn cut_failure() {
        let cut: Cut<String, String> = Cut::failure("boom".into(), CutOid::new("err"), None);
        assert!(cut.is_err());
        assert!(!cut.is_ok());
        assert_eq!(cut.into_result(), Err("boom".into()));
    }

    #[test]
    fn cut_with_parent() {
        let parent = CutOid::new("parent");
        let cut: Cut<i32, String> = Cut::success(42, CutOid::new("child"), Some(parent.clone()));
        assert_eq!(cut.parent(), Some(&parent));
    }

    #[test]
    #[should_panic]
    fn cut_unwrap_panics_on_error() {
        let cut: Cut<String, String> = Cut::failure("boom".into(), CutOid::new("err"), None);
        cut.unwrap();
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

    // -- Story + Composed --
    // Single monomorphization: Composed<FailIf42, FailAbove100, i32>
    // Both steps can fail, so all 4 branches of Composed::record() are reachable.

    #[derive(Clone)]
    struct FailIf42;

    impl Story<i32, i32> for FailIf42 {
        type Error = String;
        fn record(&self, source: i32) -> Cut<i32, String> {
            if source == 42 {
                Cut::failure("is 42".into(), CutOid::new("err"), None)
            } else {
                Cut::success(source, CutOid::new(format!("{}", source)), None)
            }
        }
    }

    #[derive(Clone)]
    struct FailAbove100;

    impl Story<i32, i32> for FailAbove100 {
        type Error = String;
        fn record(&self, source: i32) -> Cut<i32, String> {
            if source > 100 {
                Cut::failure("too big".into(), CutOid::new("err"), None)
            } else {
                Cut::success(source, CutOid::new(format!("{}", source)), None)
            }
        }
    }

    fn pipeline() -> Composed<FailIf42, FailAbove100, i32> {
        FailIf42.compose(FailAbove100)
    }

    #[test]
    fn story_compose_chain() {
        assert_eq!(pipeline().record(5).unwrap(), 5);
    }

    #[test]
    fn story_compose_parent_link() {
        let cut = pipeline().record(5);
        assert!(cut.parent().is_some());
    }

    #[test]
    fn composed_first_error() {
        let cut = pipeline().record(42); // FailIf42 rejects
        assert!(cut.is_err());
        assert!(matches!(cut.into_result(), Err(ComposedError::First(_))));
    }

    #[test]
    fn composed_second_error() {
        let cut = pipeline().record(200); // FailIf42 passes, FailAbove100 rejects
        assert!(cut.is_err());
        assert!(matches!(cut.into_result(), Err(ComposedError::Second(_))));
    }

    #[test]
    fn composed_error_display() {
        let first: ComposedError<String, String> = ComposedError::First("parse failed".into());
        let second: ComposedError<String, String> = ComposedError::Second("resolve failed".into());
        assert_eq!(format!("{}", first), "parse failed");
        assert_eq!(format!("{}", second), "resolve failed");
    }

    // -- domain_oid! tested via AstOid/ConversationOid/FolderOid --
    // -- Full macro test returns post-refactor when $crate::* = kernel::* --

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

}
