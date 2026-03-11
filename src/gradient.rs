use std::convert::Infallible;
use std::marker::PhantomData;

use crate::witness::{ContentAddressed, Oid, Trace};

/// A gradient between two domains.
///
/// `trace` transforms `A → B`, returning a `Trace` that witnesses
/// the transformation. Every output is content-addressed.
///
/// Bidirectional gradients implement the trait twice:
/// `Gradient<A, B>` and `Gradient<B, A>`.
pub trait Gradient<A, B: ContentAddressed> {
    type Error;

    fn trace(&self, source: A) -> Trace<B, Self::Error>;

    fn compose<C: ContentAddressed, G: Gradient<B, C>>(self, other: G) -> Composed<Self, G, B>
    where
        Self: Sized,
    {
        Composed(self, other, PhantomData)
    }
}

// ---------------------------------------------------------------------------
// Identity — the zero gradient. No transformation. Never fails.
// ---------------------------------------------------------------------------

pub struct Identity<A>(PhantomData<A>);

impl<A> Identity<A> {
    pub fn new() -> Self {
        Identity(PhantomData)
    }
}

impl<A> Default for Identity<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: ContentAddressed> Gradient<A, A> for Identity<A> {
    type Error = Infallible;

    fn trace(&self, source: A) -> Trace<A, Infallible> {
        let oid = source.content_oid();
        Trace::leaf(Ok(source), oid)
    }
}

// ---------------------------------------------------------------------------
// Composed — chains two gradients: A → B → C.
// ---------------------------------------------------------------------------

pub struct Composed<F, G, Mid>(pub F, pub G, PhantomData<Mid>);

#[derive(Debug, PartialEq)]
pub enum ComposedError<E1, E2> {
    First(E1),
    Second(E2),
}

impl<A, C, Mid, F, G> Gradient<A, C> for Composed<F, G, Mid>
where
    C: ContentAddressed,
    Mid: ContentAddressed,
    F: Gradient<A, Mid>,
    G: Gradient<Mid, C>,
{
    type Error = ComposedError<F::Error, G::Error>;

    fn trace(&self, source: A) -> Trace<C, Self::Error> {
        let first = self.0.trace(source);
        let first_oid = first.oid().clone();
        match first.into_result() {
            Err(e) => Trace::leaf(Err(ComposedError::First(e)), first_oid),
            Ok(mid) => {
                let second = self.1.trace(mid);
                let second_oid = second.oid().clone();
                match second.into_result() {
                    Ok(c) => {
                        let oid = c.content_oid();
                        Trace::leaf(Ok(c), oid)
                    }
                    Err(e) => Trace::leaf(Err(ComposedError::Second(e)), second_oid),
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Iso — total gradient. Never fails in either direction.
// ---------------------------------------------------------------------------

pub trait Iso<A, B: ContentAddressed>: Gradient<A, B, Error = Infallible> {}

impl<A, B: ContentAddressed, G: Gradient<A, B, Error = Infallible>> Iso<A, B> for G {}

// ---------------------------------------------------------------------------
// Fallback — try F first; if it fails, try G.
// ---------------------------------------------------------------------------

pub struct Fallback<F, G>(pub F, pub G);

impl<A, B, F, G> Gradient<A, B> for Fallback<F, G>
where
    A: Clone,
    B: ContentAddressed + Clone,
    F: Gradient<A, B>,
    G: Gradient<A, B, Error = F::Error>,
{
    type Error = F::Error;

    fn trace(&self, source: A) -> Trace<B, F::Error> {
        let first = self.0.trace(source.clone());
        if first.is_ok() {
            first
        } else {
            self.1.trace(source)
        }
    }
}

// ---------------------------------------------------------------------------
// When — guarded gradient. Apply only when the predicate holds.
// ---------------------------------------------------------------------------

pub struct When<G, P> {
    pub predicate: P,
    pub gradient: G,
}

impl<A: ContentAddressed, G: Gradient<A, A>, P: Fn(&A) -> bool> Gradient<A, A> for When<G, P> {
    type Error = G::Error;

    fn trace(&self, source: A) -> Trace<A, G::Error> {
        if (self.predicate)(&source) {
            self.gradient.trace(source)
        } else {
            let oid = source.content_oid();
            Trace::leaf(Ok(source), oid)
        }
    }
}

// ---------------------------------------------------------------------------
// Vec<G> — ordered pipeline. Trace left-to-right.
// ---------------------------------------------------------------------------

impl<A: ContentAddressed, G: Gradient<A, A>> Gradient<A, A> for Vec<G> {
    type Error = G::Error;

    fn trace(&self, source: A) -> Trace<A, G::Error> {
        let mut current = source;
        for g in self.iter() {
            let t = g.trace(current);
            match t.into_result() {
                Ok(next) => current = next,
                Err(e) => {
                    return Trace::leaf(Err(e), Oid::new("error"));
                }
            }
        }
        let oid = current.content_oid();
        Trace::leaf(Ok(current), oid)
    }
}

// ---------------------------------------------------------------------------
// Option<G> — optional gradient. None behaves as Identity.
// ---------------------------------------------------------------------------

impl<A: ContentAddressed, G: Gradient<A, A>> Gradient<A, A> for Option<G> {
    type Error = G::Error;

    fn trace(&self, source: A) -> Trace<A, G::Error> {
        match self {
            Some(g) => g.trace(source),
            None => {
                let oid = source.content_oid();
                Trace::leaf(Ok(source), oid)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // -- Test gradients --

    struct Double;
    impl Gradient<i32, i32> for Double {
        type Error = ();
        fn trace(&self, source: i32) -> Trace<i32, ()> {
            let result = source * 2;
            Trace::leaf(Ok(result), result.content_oid())
        }
    }

    // Succeeds on positive, fails on non-positive. Single type so
    // combinators that branch on Ok/Err share one monomorphization.
    struct DoublePositive;
    impl Gradient<i32, i32> for DoublePositive {
        type Error = ();
        fn trace(&self, source: i32) -> Trace<i32, ()> {
            if source > 0 {
                let result = source * 2;
                Trace::leaf(Ok(result), result.content_oid())
            } else {
                Trace::leaf(Err(()), Oid::new("error"))
            }
        }
    }

    // Stringifies positive, fails on non-positive. Paired with DoublePositive
    // for Composed tests: input 3 → Ok→Ok, input -1 → Err, input 100 → Ok→Err.
    struct StringifyPositive;
    impl Gradient<i32, String> for StringifyPositive {
        type Error = ();
        fn trace(&self, source: i32) -> Trace<String, ()> {
            if source < 10 {
                let result = source.to_string();
                let oid = result.content_oid();
                Trace::leaf(Ok(result), oid)
            } else {
                Trace::leaf(Err(()), Oid::new("error"))
            }
        }
    }

    struct FlipSign;
    impl Gradient<i32, i32> for FlipSign {
        type Error = Infallible;
        fn trace(&self, source: i32) -> Trace<i32, Infallible> {
            let result = -source;
            Trace::leaf(Ok(result), result.content_oid())
        }
    }

    fn requires_iso<A, B: ContentAddressed>(_: impl Iso<A, B>) {}

    // -- Identity --

    #[test]
    fn identity_trace_returns_input() {
        let g: Identity<i32> = Identity::new();
        let t = g.trace(42);
        assert_eq!(t.into_result(), Ok(42));
    }

    #[test]
    fn identity_default() {
        let g: Identity<i32> = Identity::default();
        assert_eq!(g.trace(42).into_result(), Ok(42));
    }

    // -- Composed --
    // All compose tests share Composed<DoublePositive, StringifyPositive, i32>
    // → single monomorphization, all branches covered.

    #[test]
    fn compose_chains_trace() {
        let g = DoublePositive.compose(StringifyPositive);
        // 3 → DoublePositive: Ok(6) → StringifyPositive: Ok("6")
        assert_eq!(g.trace(3).into_result(), Ok("6".to_string()));
    }

    #[test]
    fn compose_first_error() {
        let g = DoublePositive.compose(StringifyPositive);
        // -1 → DoublePositive: Err
        let t = g.trace(-1);
        assert!(t.is_err());
        assert_eq!(t.into_result(), Err(ComposedError::First(())));
    }

    #[test]
    fn compose_second_error() {
        let g = DoublePositive.compose(StringifyPositive);
        // 100 → DoublePositive: Ok(200) → StringifyPositive: Err (200 >= 10)
        let t = g.trace(100);
        assert!(t.is_err());
        assert_eq!(t.into_result(), Err(ComposedError::Second(())));
    }

    // -- Vec pipeline --

    #[test]
    fn vec_trace_applies_in_order() {
        let gs: Vec<DoublePositive> = vec![DoublePositive, DoublePositive];
        assert_eq!(gs.trace(3).into_result(), Ok(12));
    }

    #[test]
    fn vec_error_stops_pipeline() {
        let gs: Vec<DoublePositive> = vec![DoublePositive];
        let t = gs.trace(-1);
        assert!(t.is_err());
        assert_eq!(t.into_result(), Err(()));
    }

    #[test]
    fn vec_empty_is_identity() {
        let gs: Vec<DoublePositive> = vec![];
        assert_eq!(gs.trace(5).into_result(), Ok(5));
    }

    // -- When --

    #[test]
    fn when_applies_when_true() {
        let g = When {
            predicate: |x: &i32| *x > 0,
            gradient: Double,
        };
        assert_eq!(g.trace(3).into_result(), Ok(6));
        assert_eq!(g.trace(-1).into_result(), Ok(-1));
    }

    // -- Fallback --
    // Both tests share Fallback<DoublePositive, Double> → single monomorphization.

    #[test]
    fn fallback_uses_first_when_ok() {
        let g = Fallback(DoublePositive, Double);
        // 3 → DoublePositive succeeds (6) → returns first
        assert_eq!(g.trace(3).into_result(), Ok(6));
    }

    #[test]
    fn fallback_uses_second_when_first_fails() {
        let g = Fallback(DoublePositive, Double);
        // -1 → DoublePositive fails → Double(-1) = -2
        assert_eq!(g.trace(-1).into_result(), Ok(-2));
    }

    // -- Iso --

    #[test]
    fn infallible_gradient_is_iso() {
        requires_iso(FlipSign);
        assert_eq!(FlipSign.trace(5).into_result(), Ok(-5));
    }

    // -- Option --

    #[test]
    fn some_applies_gradient() {
        assert_eq!(Some(Double).trace(3).into_result(), Ok(6));
    }

    #[test]
    fn none_is_identity() {
        assert_eq!(None::<Double>.trace(3).into_result(), Ok(3));
    }

    // -- Trace structure --

    #[test]
    fn trace_carries_oid() {
        let t = Double.trace(3);
        assert!(t.is_ok());
        assert_eq!(t.oid(), &6i32.content_oid());
    }

    #[test]
    fn error_trace_carries_oid() {
        let t = DoublePositive.trace(-1);
        assert!(t.is_err());
        assert_eq!(t.oid(), &Oid::new("error"));
    }
}
