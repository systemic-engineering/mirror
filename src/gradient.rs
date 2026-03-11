use std::convert::Infallible;
use std::marker::PhantomData;

/// A gradient between two domains.
///
/// `emit` transforms `A → B`. `absorb` transforms `B → A`.
/// Both are fallible — transformations can fail.
///
/// The implementing type carries the strategy: configuration,
/// context, or state the transformation needs.
pub trait Gradient<A, B> {
    type Error;

    fn emit(&self, source: A) -> Result<B, Self::Error>;
    fn absorb(&self, source: B) -> Result<A, Self::Error>;

    fn compose<C, G: Gradient<B, C>>(self, other: G) -> Composed<Self, G, B>
    where
        Self: Sized,
    {
        Composed(self, other, PhantomData)
    }

    fn roundtrip(&self, source: A) -> Result<A, Self::Error> {
        self.emit(source).and_then(|b| self.absorb(b))
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

impl<A> Gradient<A, A> for Identity<A> {
    type Error = Infallible;

    fn emit(&self, source: A) -> Result<A, Infallible> {
        Ok(source)
    }

    fn absorb(&self, source: A) -> Result<A, Infallible> {
        Ok(source)
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
    F: Gradient<A, Mid>,
    G: Gradient<Mid, C>,
{
    type Error = ComposedError<F::Error, G::Error>;

    fn emit(&self, source: A) -> Result<C, Self::Error> {
        self.0
            .emit(source)
            .map_err(ComposedError::First)
            .and_then(|mid| self.1.emit(mid).map_err(ComposedError::Second))
    }

    fn absorb(&self, source: C) -> Result<A, Self::Error> {
        self.1
            .absorb(source)
            .map_err(ComposedError::Second)
            .and_then(|mid| self.0.absorb(mid).map_err(ComposedError::First))
    }
}

// ---------------------------------------------------------------------------
// Iso — total gradient. Never fails in either direction.
// ---------------------------------------------------------------------------

pub trait Iso<A, B>: Gradient<A, B, Error = Infallible> {}

impl<A, B, G: Gradient<A, B, Error = Infallible>> Iso<A, B> for G {}

// ---------------------------------------------------------------------------
// Fallback — try F first; if it fails, try G.
// ---------------------------------------------------------------------------

pub struct Fallback<F, G>(pub F, pub G);

impl<A, B, F, G> Gradient<A, B> for Fallback<F, G>
where
    A: Clone,
    B: Clone,
    F: Gradient<A, B>,
    G: Gradient<A, B, Error = F::Error>,
{
    type Error = F::Error;

    fn emit(&self, source: A) -> Result<B, F::Error> {
        self.0.emit(source.clone()).or_else(|_| self.1.emit(source))
    }

    fn absorb(&self, source: B) -> Result<A, F::Error> {
        self.0
            .absorb(source.clone())
            .or_else(|_| self.1.absorb(source))
    }
}

// ---------------------------------------------------------------------------
// When — guarded gradient. Apply only when the predicate holds.
// ---------------------------------------------------------------------------

pub struct When<G, P> {
    pub predicate: P,
    pub gradient: G,
}

impl<A, G: Gradient<A, A>, P: Fn(&A) -> bool> Gradient<A, A> for When<G, P> {
    type Error = G::Error;

    fn emit(&self, source: A) -> Result<A, G::Error> {
        if (self.predicate)(&source) {
            self.gradient.emit(source)
        } else {
            Ok(source)
        }
    }

    fn absorb(&self, source: A) -> Result<A, G::Error> {
        if (self.predicate)(&source) {
            self.gradient.absorb(source)
        } else {
            Ok(source)
        }
    }
}

// ---------------------------------------------------------------------------
// Vec<G> — ordered pipeline. Emit left-to-right. Absorb right-to-left.
// ---------------------------------------------------------------------------

impl<A, G: Gradient<A, A>> Gradient<A, A> for Vec<G> {
    type Error = G::Error;

    fn emit(&self, source: A) -> Result<A, G::Error> {
        self.iter().try_fold(source, |acc, g| g.emit(acc))
    }

    fn absorb(&self, source: A) -> Result<A, G::Error> {
        self.iter().rev().try_fold(source, |acc, g| g.absorb(acc))
    }
}

// ---------------------------------------------------------------------------
// Option<G> — optional gradient. None behaves as Identity.
// ---------------------------------------------------------------------------

impl<A, G: Gradient<A, A>> Gradient<A, A> for Option<G> {
    type Error = G::Error;

    fn emit(&self, source: A) -> Result<A, G::Error> {
        match self {
            Some(g) => g.emit(source),
            None => Ok(source),
        }
    }

    fn absorb(&self, source: A) -> Result<A, G::Error> {
        match self {
            Some(g) => g.absorb(source),
            None => Ok(source),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Double;
    impl Gradient<i32, i32> for Double {
        type Error = ();
        fn emit(&self, source: i32) -> Result<i32, ()> {
            Ok(source * 2)
        }
        fn absorb(&self, source: i32) -> Result<i32, ()> {
            Ok(source / 2)
        }
    }

    struct Stringify;
    impl Gradient<i32, String> for Stringify {
        type Error = ();
        fn emit(&self, source: i32) -> Result<String, ()> {
            Ok(source.to_string())
        }
        fn absorb(&self, source: String) -> Result<i32, ()> {
            source.parse().map_err(|_| ())
        }
    }

    // Shared test stub: always fails. One type = one monomorphization.
    #[derive(Clone)]
    struct Fails;
    impl Gradient<i32, i32> for Fails {
        type Error = ();
        fn emit(&self, _: i32) -> Result<i32, ()> {
            Err(())
        }
        fn absorb(&self, _: i32) -> Result<i32, ()> {
            Err(())
        }
    }

    struct FlipSign;
    impl Gradient<i32, i32> for FlipSign {
        type Error = Infallible;
        fn emit(&self, source: i32) -> Result<i32, Infallible> {
            Ok(-source)
        }
        fn absorb(&self, source: i32) -> Result<i32, Infallible> {
            Ok(-source)
        }
    }

    fn requires_iso<A, B>(_: impl Iso<A, B>) {}

    // -- Identity --

    #[test]
    fn identity_emit_returns_input() {
        let g: Identity<i32> = Identity::new();
        assert_eq!(g.emit(42), Ok(42));
    }

    #[test]
    fn identity_absorb_returns_input() {
        let g: Identity<i32> = Identity::new();
        assert_eq!(g.absorb(42), Ok(42));
    }

    #[test]
    fn identity_roundtrip() {
        let g: Identity<i32> = Identity::new();
        assert_eq!(g.roundtrip(7), Ok(7));
    }

    #[test]
    fn identity_default() {
        let g: Identity<i32> = Identity::default();
        assert_eq!(g.emit(42), Ok(42));
    }

    // -- Composed --

    #[test]
    fn compose_chains_emit() {
        let g = Double.compose(Stringify);
        assert_eq!(g.emit(3), Ok("6".to_string()));
    }

    #[test]
    fn compose_chains_absorb() {
        let g = Double.compose(Stringify);
        assert_eq!(g.absorb("6".to_string()), Ok(3));
    }

    // -- Vec pipeline --

    #[test]
    fn vec_emit_applies_in_order() {
        let gs: Vec<Double> = vec![Double, Double];
        assert_eq!(gs.emit(3), Ok(12));
    }

    #[test]
    fn vec_absorb_applies_in_reverse() {
        let gs: Vec<Double> = vec![Double, Double];
        assert_eq!(gs.absorb(12), Ok(3));
    }

    #[test]
    fn vec_empty_is_identity() {
        let gs: Vec<Double> = vec![];
        assert_eq!(gs.emit(5), Ok(5));
        assert_eq!(gs.absorb(5), Ok(5));
    }

    // -- When (same instance tests both branches, both directions) --

    #[test]
    fn when_applies_when_true() {
        let g = When {
            predicate: |x: &i32| *x > 0,
            gradient: Double,
        };
        assert_eq!(g.emit(3), Ok(6));
        assert_eq!(g.emit(-1), Ok(-1));
        assert_eq!(g.absorb(6), Ok(3));
        assert_eq!(g.absorb(-1), Ok(-1));
    }

    // -- Fallback (shared Fails type) --

    #[test]
    fn fallback_uses_first_when_ok() {
        let g = Fallback(Double, Fails);
        assert_eq!(g.emit(3), Ok(6));
        assert_eq!(g.absorb(6), Ok(3));
    }

    #[test]
    fn fallback_uses_second_when_first_fails() {
        let g = Fallback(Fails, Double);
        assert_eq!(g.emit(3), Ok(6));
        assert_eq!(g.absorb(6), Ok(3));
    }

    // -- Iso --

    #[test]
    fn infallible_gradient_is_iso() {
        requires_iso(FlipSign);
        assert_eq!(FlipSign.emit(5), Ok(-5));
        assert_eq!(FlipSign.absorb(-5), Ok(5));
    }

    // -- Roundtrip --

    #[test]
    fn roundtrip_returns_original() {
        assert_eq!(Double.roundtrip(4), Ok(4));
    }

    // -- Option --

    #[test]
    fn some_applies_gradient() {
        assert_eq!(Some(Double).emit(3), Ok(6));
        assert_eq!(Some(Double).absorb(6), Ok(3));
    }

    #[test]
    fn none_is_identity() {
        assert_eq!(None::<Double>.emit(3), Ok(3));
        assert_eq!(None::<Double>.absorb(3), Ok(3));
    }
}
