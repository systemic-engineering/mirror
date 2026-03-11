use std::convert::Infallible;

use crate::gradient::Gradient;
use crate::witness::{ContentAddressed, Oid, Trace};

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
/// trace = preview (partial — fails with NotFound).
/// The asymmetry is structural: looking is uncertain, constructing is total.
pub struct PrismGradient<P>(pub P);

impl<S, A: ContentAddressed, P: Prism<S, A>> Gradient<S, A> for PrismGradient<P> {
    type Error = NotFound;

    fn trace(&self, source: S) -> Trace<A, NotFound> {
        match self.0.preview(&source) {
            Some(a) => {
                let oid = a.content_oid();
                Trace::leaf(Ok(a), oid)
            }
            None => Trace::leaf(Err(NotFound), Oid::new("not-found")),
        }
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
/// trace returns the original S alongside extracted values.
/// Infallible — traversal always succeeds.
pub struct TraversalGradient<T>(pub T);

impl<S, A, T: Traversal<S, A>> Gradient<S, (S, Vec<A>)> for TraversalGradient<T>
where
    (S, Vec<A>): ContentAddressed,
{
    type Error = Infallible;

    fn trace(&self, source: S) -> Trace<(S, Vec<A>), Infallible> {
        let items = self.0.traverse(&source);
        let result = (source, items);
        let oid = result.content_oid();
        Trace::leaf(Ok(result), oid)
    }
}

/// Select elements matching a predicate.
///
/// `preview`: if predicate matches, return the element. Else None.
/// `review`: identity — the matched element IS the element.
///
/// Composes with existing adapters:
/// - `PrismGradient(SelectPrism(f))` → Gradient that succeeds/fails on predicate
/// - `PrismAsTraversal(SelectPrism(f))` → Traversal selecting all matches from Vec
pub struct SelectPrism<F>(pub F);

impl<S: Clone, F: Fn(&S) -> bool> Prism<S, S> for SelectPrism<F> {
    fn preview(&self, source: &S) -> Option<S> {
        if (self.0)(source) {
            Some(source.clone())
        } else {
            None
        }
    }

    fn review(&self, focus: S) -> S {
        focus
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
    use sha2::{Digest, Sha256};

    #[derive(Debug, Clone, PartialEq)]
    enum Shape {
        Circle(f64),
        Square(f64),
    }

    impl ContentAddressed for Shape {
        fn content_oid(&self) -> Oid {
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
            Oid::new(hex::encode(hasher.finalize()))
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
    fn prism_gradient_trace_succeeds_on_match() {
        let g = PrismGradient(CircleRadius);
        assert_eq!(g.trace(Shape::Circle(3.0)).into_result(), Ok(3.0));
    }

    #[test]
    fn prism_gradient_trace_fails_on_no_match() {
        let g = PrismGradient(CircleRadius);
        assert_eq!(g.trace(Shape::Square(3.0)).into_result(), Err(NotFound));
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
    fn traversal_gradient_trace_extracts_with_source() {
        let g = TraversalGradient(AllSomes);
        let source = vec![Some(1), None, Some(3)];
        let (s, items) = g.trace(source.clone()).unwrap();
        assert_eq!(s, source);
        assert_eq!(items, vec![1, 3]);
    }

    #[test]
    fn traversal_gradient_roundtrip() {
        let g = TraversalGradient(AllSomes);
        let source = vec![Some(1), None, Some(3)];
        let (s, items) = g.trace(source).unwrap();
        let rebuilt = AllSomes.rebuild(s, items);
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
        let (_, radii) = g.trace(source).unwrap();
        assert_eq!(radii, vec![1.0, 3.0]);
    }

    // -- SelectPrism tests --

    fn is_circle(shape: &Shape) -> bool {
        matches!(shape, Shape::Circle(_))
    }

    #[test]
    fn select_prism_preview_matches() {
        let p = SelectPrism(is_circle);
        assert_eq!(p.preview(&Shape::Circle(3.0)), Some(Shape::Circle(3.0)));
    }

    #[test]
    fn select_prism_preview_rejects() {
        let p = SelectPrism(is_circle);
        assert_eq!(p.preview(&Shape::Square(3.0)), None);
    }

    #[test]
    fn select_prism_review_is_identity() {
        let p = SelectPrism(is_circle);
        assert_eq!(p.review(Shape::Circle(5.0)), Shape::Circle(5.0));
    }

    #[test]
    fn select_prism_law_review_preview_roundtrip() {
        let p = SelectPrism(is_circle);
        let a = Shape::Circle(3.0);
        assert_eq!(p.preview(&p.review(a.clone())), Some(a));
    }

    // -- SelectPrism composition tests --

    #[test]
    fn select_as_traversal_extracts() {
        let t = PrismAsTraversal(SelectPrism(is_circle));
        let source = vec![Shape::Circle(1.0), Shape::Square(2.0), Shape::Circle(3.0)];
        assert_eq!(
            t.traverse(&source),
            vec![Shape::Circle(1.0), Shape::Circle(3.0)]
        );
    }

    #[test]
    fn select_as_traversal_rebuild() {
        let t = PrismAsTraversal(SelectPrism(is_circle));
        let source = vec![Shape::Circle(1.0), Shape::Square(2.0), Shape::Circle(3.0)];
        let rebuilt = t.rebuild(source, vec![Shape::Circle(10.0), Shape::Circle(30.0)]);
        assert_eq!(
            rebuilt,
            vec![Shape::Circle(10.0), Shape::Square(2.0), Shape::Circle(30.0)]
        );
    }

    #[test]
    fn select_as_gradient_hit() {
        let g = PrismGradient(SelectPrism(is_circle));
        assert_eq!(
            g.trace(Shape::Circle(3.0)).into_result(),
            Ok(Shape::Circle(3.0))
        );
    }

    #[test]
    fn select_as_gradient_miss() {
        let g = PrismGradient(SelectPrism(is_circle));
        assert_eq!(g.trace(Shape::Square(3.0)).into_result(), Err(NotFound));
    }
}
