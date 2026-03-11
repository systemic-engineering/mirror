use std::convert::Infallible;

use crate::gradient::Gradient;

/// Focus optionally into a structure.
///
/// `preview` extracts `A` from `S` if it's there.
/// `review` always reconstructs `S` from `A`.
///
/// A Fractal IS a Prism: preview asks "is there structure here?"
/// Shard returns None. Fractal returns the children.
/// The self-similarity means the same Prism composes with itself.
pub trait Prism<S, A> {
    fn preview(&self, source: &S) -> Option<A>;
    fn review(&self, focus: A) -> S;
}

/// The error when a prism's preview finds nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotFound;

/// Wraps a `Prism` as a `Gradient<S, A>`.
///
/// emit = preview (partial — fails with NotFound).
/// absorb = review (total — always succeeds).
/// The asymmetry is structural: looking is uncertain, constructing is total.
pub struct PrismGradient<P>(pub P);

impl<S, A, P: Prism<S, A>> Gradient<S, A> for PrismGradient<P> {
    type Error = NotFound;

    fn emit(&self, source: S) -> Result<A, NotFound> {
        self.0.preview(&source).ok_or(NotFound)
    }

    fn absorb(&self, source: A) -> Result<S, NotFound> {
        Ok(self.0.review(source))
    }
}

/// Focus on multiple elements within a structure.
///
/// `traverse` extracts all `A` values from `S`.
/// `rebuild` reconstructs `S` from a modified set of `A` values.
pub trait Traversal<S, A> {
    fn traverse(&self, source: &S) -> Vec<A>;
    fn rebuild(&self, source: S, items: Vec<A>) -> S;
}

/// Wraps a `Traversal` as a `Gradient<S, (S, Vec<A>)>`.
///
/// emit returns the original S alongside extracted values —
/// absorb needs both to know where to put the new values.
/// Both directions infallible.
pub struct TraversalGradient<T>(pub T);

impl<S, A, T: Traversal<S, A>> Gradient<S, (S, Vec<A>)> for TraversalGradient<T> {
    type Error = Infallible;

    fn emit(&self, source: S) -> Result<(S, Vec<A>), Infallible> {
        let items = self.0.traverse(&source);
        Ok((source, items))
    }

    fn absorb(&self, (source, items): (S, Vec<A>)) -> Result<S, Infallible> {
        Ok(self.0.rebuild(source, items))
    }
}

/// Lifts a `Prism<S, A>` to a `Traversal<Vec<S>, A>`.
///
/// `traverse` extracts every element where the prism matches.
/// `rebuild` replaces matching elements in-order, leaving non-matching unchanged.
///
/// A prism selects zero or one element from `S`; applied across `Vec<S>`
/// it selects all matching elements. Tree children are `Vec<Tree<E>>` —
/// no separate Collection type needed.
pub struct PrismAsTraversal<P>(pub P);

impl<S, A, P: Prism<S, A>> Traversal<Vec<S>, A> for PrismAsTraversal<P> {
    fn traverse(&self, source: &Vec<S>) -> Vec<A> {
        source.iter().filter_map(|s| self.0.preview(s)).collect()
    }

    fn rebuild(&self, source: Vec<S>, items: Vec<A>) -> Vec<S> {
        let mut items = items.into_iter();
        source
            .into_iter()
            .map(|s| {
                if self.0.preview(&s).is_some() {
                    match items.next() {
                        Some(a) => self.0.review(a),
                        None => s,
                    }
                } else {
                    s
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gradient::Gradient;
    use crate::witness::{LegacyOid, Session, Witnessed};
    use sha2::{Digest, Sha256};

    // -- Test types --

    #[derive(Debug, Clone, PartialEq)]
    enum Shape {
        Circle(f64),
        Square(f64),
    }

    impl LegacyOid for Shape {
        fn oid(&self) -> String {
            let mut hasher = Sha256::new();
            match self {
                Shape::Circle(r) => {
                    hasher.update(b"circle:");
                    hasher.update(r.to_le_bytes());
                }
                Shape::Square(s) => {
                    hasher.update(b"square:");
                    hasher.update(s.to_le_bytes());
                }
            }
            hex::encode(hasher.finalize())
        }
    }

    impl LegacyOid for f64 {
        fn oid(&self) -> String {
            let mut hasher = Sha256::new();
            hasher.update(self.to_le_bytes());
            hex::encode(hasher.finalize())
        }
    }

    struct CircleRadius;

    impl Prism<Shape, f64> for CircleRadius {
        fn preview(&self, source: &Shape) -> Option<f64> {
            match source {
                Shape::Circle(r) => Some(*r),
                _ => None,
            }
        }

        fn review(&self, focus: f64) -> Shape {
            Shape::Circle(focus)
        }
    }

    // -- Prism tests --

    #[test]
    fn preview_returns_some_on_match() {
        assert_eq!(CircleRadius.preview(&Shape::Circle(3.0)), Some(3.0));
    }

    #[test]
    fn preview_returns_none_on_no_match() {
        assert_eq!(CircleRadius.preview(&Shape::Square(3.0)), None);
    }

    #[test]
    fn review_reconstructs() {
        assert_eq!(CircleRadius.review(5.0), Shape::Circle(5.0));
    }

    #[test]
    fn prism_law_review_then_preview_roundtrips() {
        let p = CircleRadius;
        assert_eq!(p.preview(&p.review(3.0)), Some(3.0));
    }

    // -- PrismGradient tests --

    #[test]
    fn prism_gradient_emit_succeeds_on_match() {
        let g = PrismGradient(CircleRadius);
        assert_eq!(g.emit(Shape::Circle(3.0)), Ok(3.0));
    }

    #[test]
    fn prism_gradient_emit_fails_on_no_match() {
        let g = PrismGradient(CircleRadius);
        assert_eq!(g.emit(Shape::Square(3.0)), Err(NotFound));
    }

    #[test]
    fn prism_gradient_absorb_always_succeeds() {
        let g = PrismGradient(CircleRadius);
        assert_eq!(g.absorb(4.0), Ok(Shape::Circle(4.0)));
    }

    // -- Traversal tests --

    struct AllSomes;

    impl Traversal<Vec<Option<i32>>, i32> for AllSomes {
        fn traverse(&self, source: &Vec<Option<i32>>) -> Vec<i32> {
            source.iter().filter_map(|x| *x).collect()
        }

        fn rebuild(&self, source: Vec<Option<i32>>, items: Vec<i32>) -> Vec<Option<i32>> {
            let mut items = items.into_iter();
            source
                .into_iter()
                .map(|x| {
                    if x.is_some() {
                        items.next().map(Some).unwrap_or(None)
                    } else {
                        None
                    }
                })
                .collect()
        }
    }

    #[test]
    fn traverse_extracts_matching() {
        let t = AllSomes;
        assert_eq!(t.traverse(&vec![Some(1), None, Some(3)]), vec![1, 3]);
    }

    #[test]
    fn rebuild_reconstructs() {
        let t = AllSomes;
        let source = vec![Some(1), None, Some(3)];
        let rebuilt = t.rebuild(source, vec![10, 30]);
        assert_eq!(rebuilt, vec![Some(10), None, Some(30)]);
    }

    #[test]
    fn traversal_law_rebuild_traverse_is_identity() {
        let t = AllSomes;
        let source = vec![Some(1), None, Some(3)];
        let rebuilt = t.rebuild(source.clone(), t.traverse(&source));
        assert_eq!(rebuilt, source);
    }

    // -- TraversalGradient tests --

    #[test]
    fn traversal_gradient_emit_extracts_with_source() {
        let g = TraversalGradient(AllSomes);
        let source = vec![Some(1), None, Some(3)];
        let (s, items) = g.emit(source.clone()).unwrap();
        assert_eq!(s, source);
        assert_eq!(items, vec![1, 3]);
    }

    #[test]
    fn traversal_gradient_absorb_rebuilds() {
        let g = TraversalGradient(AllSomes);
        let source = vec![Some(1), None, Some(3)];
        let rebuilt = g.absorb((source, vec![10, 30])).unwrap();
        assert_eq!(rebuilt, vec![Some(10), None, Some(30)]);
    }

    #[test]
    fn traversal_gradient_roundtrip() {
        let g = TraversalGradient(AllSomes);
        let source = vec![Some(1), None, Some(3)];
        let (s, items) = g.emit(source).unwrap();
        let rebuilt = g.absorb((s, items)).unwrap();
        assert_eq!(rebuilt, vec![Some(1), None, Some(3)]);
    }

    // -- PrismAsTraversal tests --

    #[test]
    fn prism_as_traversal_extracts_matches() {
        let t = PrismAsTraversal(CircleRadius);
        let source = vec![Shape::Circle(1.0), Shape::Square(2.0), Shape::Circle(3.0)];
        assert_eq!(t.traverse(&source), vec![1.0, 3.0]);
    }

    #[test]
    fn prism_as_traversal_empty_on_no_match() {
        let t = PrismAsTraversal(CircleRadius);
        let source = vec![Shape::Square(1.0), Shape::Square(2.0)];
        assert_eq!(t.traverse(&source), Vec::<f64>::new());
    }

    #[test]
    fn prism_as_traversal_rebuild_replaces_matches() {
        let t = PrismAsTraversal(CircleRadius);
        let source = vec![Shape::Circle(1.0), Shape::Square(2.0), Shape::Circle(3.0)];
        let rebuilt = t.rebuild(source, vec![10.0, 30.0]);
        assert_eq!(
            rebuilt,
            vec![Shape::Circle(10.0), Shape::Square(2.0), Shape::Circle(30.0),]
        );
    }

    #[test]
    fn prism_as_traversal_roundtrip_is_identity() {
        let t = PrismAsTraversal(CircleRadius);
        let source = vec![Shape::Circle(1.0), Shape::Square(2.0)];
        let items = t.traverse(&source);
        let rebuilt = t.rebuild(source.clone(), items);
        assert_eq!(rebuilt, source);
    }

    #[test]
    fn prism_as_traversal_passes_through_when_items_exhausted() {
        let t = PrismAsTraversal(CircleRadius);
        let source = vec![Shape::Circle(1.0), Shape::Circle(2.0)];
        let rebuilt = t.rebuild(source, vec![10.0]);
        assert_eq!(rebuilt, vec![Shape::Circle(10.0), Shape::Circle(2.0)]);
    }

    #[test]
    fn prism_as_traversal_as_gradient() {
        let g = TraversalGradient(PrismAsTraversal(CircleRadius));
        let source = vec![Shape::Circle(1.0), Shape::Square(2.0), Shape::Circle(3.0)];
        let (s, radii) = g.emit(source).unwrap();
        assert_eq!(radii, vec![1.0, 3.0]);
        let rebuilt = g.absorb((s, vec![10.0, 30.0])).unwrap();
        assert_eq!(
            rebuilt,
            vec![Shape::Circle(10.0), Shape::Square(2.0), Shape::Circle(30.0),]
        );
    }

    // -- Witnessed integration --

    #[test]
    fn prism_gradient_witnessed_records_observation() {
        let session = Session::new("test", "reed");
        let g = Witnessed::new(PrismGradient(CircleRadius), &session, "circle-focus");

        let result = g.emit(Shape::Circle(3.0)).unwrap();
        assert_eq!(result, 3.0);
        assert_eq!(session.len(), 1);

        let events = session.events();
        assert_eq!(events[0].message, "circle-focus");
    }

    #[test]
    fn prism_gradient_witnessed_no_event_on_not_found() {
        let session = Session::new("test", "reed");
        let g = Witnessed::new(PrismGradient(CircleRadius), &session, "circle-focus");

        assert!(g.emit(Shape::Square(3.0)).is_err());
        assert!(session.is_empty());
    }
}
