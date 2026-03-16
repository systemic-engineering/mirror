//! The kernel. Content-addressed transformations, inlined from story.
//!
//! Everything conversation needs to transform, compose, and address.
//! ~255 lines that replace 5249 lines of story dependency.

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
        let cut: Cut<String, String> =
            Cut::success("hello".into(), CutOid::new("oid"), None);
        assert!(cut.is_ok());
        assert!(!cut.is_err());
        assert_eq!(cut.oid(), &CutOid::new("oid"));
        assert_eq!(cut.parent(), None);
        assert_eq!(cut.unwrap(), "hello");
    }

    #[test]
    fn cut_failure() {
        let cut: Cut<String, String> =
            Cut::failure("boom".into(), CutOid::new("err"), None);
        assert!(cut.is_err());
        assert!(!cut.is_ok());
        assert_eq!(cut.into_result(), Err("boom".into()));
    }

    #[test]
    fn cut_with_parent() {
        let parent = CutOid::new("parent");
        let cut: Cut<i32, String> =
            Cut::success(42, CutOid::new("child"), Some(parent.clone()));
        assert_eq!(cut.parent(), Some(&parent));
    }

    #[test]
    #[should_panic]
    fn cut_unwrap_panics_on_error() {
        let cut: Cut<String, String> =
            Cut::failure("boom".into(), CutOid::new("err"), None);
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
        let a: Vec<u8> = vec![1, 2, 3];
        let b: Vec<u8> = vec![1, 2, 3];
        let c: Vec<u8> = vec![4, 5, 6];
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

    #[derive(Clone)]
    struct Double;

    impl Story<i32, i32> for Double {
        type Error = String;
        fn record(&self, source: i32) -> Cut<i32, String> {
            let result = source * 2;
            Cut::success(result, CutOid::new(format!("{}", result)), None)
        }
    }

    #[derive(Clone)]
    struct AddTen;

    impl Story<i32, i32> for AddTen {
        type Error = String;
        fn record(&self, source: i32) -> Cut<i32, String> {
            let result = source + 10;
            Cut::success(result, CutOid::new(format!("{}", result)), None)
        }
    }

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

    #[test]
    fn story_compose_chain() {
        let pipeline = Double.compose(AddTen);
        let result = pipeline.record(5).unwrap();
        assert_eq!(result, 20); // 5*2=10, 10+10=20
    }

    #[test]
    fn story_compose_parent_link() {
        let pipeline = Double.compose(AddTen);
        let cut = pipeline.record(5);
        assert!(cut.parent().is_some());
    }

    #[test]
    fn composed_first_error() {
        let pipeline = FailIf42.compose(AddTen);
        let cut = pipeline.record(42);
        assert!(cut.is_err());
        let err = cut.into_result().unwrap_err();
        assert!(matches!(err, ComposedError::First(_)));
    }

    #[test]
    fn composed_second_error() {
        let pipeline = Double.compose(FailIf42);
        let cut = pipeline.record(21); // 21*2=42 → FailIf42 fails
        assert!(cut.is_err());
        let err = cut.into_result().unwrap_err();
        assert!(matches!(err, ComposedError::Second(_)));
    }

    #[test]
    fn composed_error_display() {
        let first: ComposedError<String, String> =
            ComposedError::First("parse failed".into());
        let second: ComposedError<String, String> =
            ComposedError::Second("resolve failed".into());
        assert_eq!(format!("{}", first), "parse failed");
        assert_eq!(format!("{}", second), "resolve failed");
    }

    // -- domain_oid! macro --

    domain_oid!(
        /// Test OID type.
        pub TestOid
    );

    #[test]
    fn domain_oid_macro_works() {
        let oid = TestOid::new("hash123");
        assert_eq!(oid.to_string(), "hash123");
        assert_eq!(oid.as_ref(), "hash123");
        let cut_oid: CutOid = oid.into();
        assert_eq!(cut_oid.as_ref(), "hash123");
    }

    // -- Setting + Addressable --

    #[test]
    fn setting_trait_exists() {
        fn requires_setting<C: Setting>() -> &'static str {
            C::id()
        }
        use crate::domain::filesystem::Filesystem;
        assert_eq!(requires_setting::<Filesystem>(), "filesystem");
    }

    #[test]
    fn addressable_trait_exists() {
        fn requires_addressable<T: Addressable>(t: &T) -> &str {
            t.node_name()
        }
        use crate::domain::filesystem::Folder;
        let f = Folder {
            name: "test".into(),
            content: None,
        };
        assert_eq!(requires_addressable(&f), "test");
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

    // -- Parse.compose(Resolve) still works --

    #[test]
    fn parse_compose_resolve_sentinel() {
        use crate::parse::Parse;
        use crate::resolve::Resolve;
        use crate::domain::filesystem::Filesystem;
        use crate::resolve::Conversation;

        let pipeline = Parse.compose::<Conversation<Filesystem>, _>(Resolve::new());
        let source = "in @filesystem\ntemplate $t {\n\tslug\n}\nout r {\n\tx: f { $t }\n}\n";
        let _conv = pipeline.record(source.to_string()).unwrap();
    }
}
