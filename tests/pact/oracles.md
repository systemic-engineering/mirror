# Pact evaluation oracles (mirror-006)

*Reed (mirror session), 2026-10-28. Shared with spectral; vectors flow both ways.*

Convention, as in `tests/properties-on-glass/oracles.md`: if the oracle and an implementation disagree, the oracle is right, or the oracle gets a dated correction here.

## What is being specified

A `pact` already declares properties in mirror: a head with `requires` clauses (Recognition #37: `requires` is a Paskian agreement), e.g. `shards/epistemologic/pact/algebra.mirror`. Every pact body is a `\` hole today; nothing evaluates them. These vectors fix the evaluation:

- **Datalog, not Prolog.** Function-free Horn clauses. Bottom-up, semi-naive, to the least fixed point. Rule order and fact order never change the result.
- **Negation under the well-founded semantics** (Van Gelder, Ross, Schlipf 1991). Every ground atom is true, false or undefined. Undefined is a comma: the outcome is Opaque and names the undefined atoms. It is never read as false.
- **Closed world for facts only.** An extensional atom that is neither a fact nor derivable by any rule is `absent`, and therefore false.

## Vector format (v 0)

One JSON file per vector. `pact` is normative; `text` is a readable rendering only (the surface syntax is still open, see mirror-006).

- `pact`: rules. `id` names the rule (provenance refers to rules by id, so it survives permutation). `head` is an atom. `body` is a list of literals, `{"atom": …}` or `{"not": …}`.
- An atom is a list: predicate, then arguments. Arguments starting with `?` are variables, everything else is a constant string.
- `facts`: ground atoms.
- `query`: ground atoms whose status must be reported even when they are false. `true` and `undefined` always list every derived (intensional) atom; `false` lists only queried atoms, because the false set is unbounded.
- `permute: true`: the result must be byte-identical under every permutation of `pact` and of `facts` (or a fixed sample of permutations, stated by the runner).

## Result

Canonical JSON: keys sorted, no insignificant whitespace, every list of atoms sorted lexicographically element by element.

```
{"false":[…],"outcome":"clear"|"opaque","true":[…],"undefined":[…]}
```

- `outcome` is `clear` iff `undefined` is empty; otherwise `opaque`. (`dark` is reserved for a pact that can't be evaluated at all, e.g. a function symbol or an unsafe variable; vector 05, not yet written.)
- A false atom carries `because`: one entry per rule whose head unifies with it, in rule-id order. Each entry lists the body literals that are not true under that grounding, in body order, each with its `status` (`false`, `undefined` or `absent`) and, for intensional atoms, its own `because`. A negative literal `not x` fails when `x` is true; its status is reported as `false` with `x`'s derivation left out of v 0.
- v 0 limit: why-not trees are specified only for rules whose body variables are all bound by the head. Existential body variables need a choice of grounding and are left for a later version.

Why-not is not semiring provenance. Semiring provenance (Green, Karvounarakis, Tannen 2007) annotates true facts with how they were derived; vector 06 will cover it. A false atom has no derivation, so its "why" is the failed body literals. That shape is already the pact verdict surface: `failure(opacity_map)`, one opacity per failed site.

## Vector 01: determinism

Transitive closure over a graph with a cycle (`b → c → b`). Nine `path` facts, the least fixed point. Exercises recursion, which `@mq` patterns cannot express (pattern recursion is bounded by pattern size; Mara 2026-08-23, Lemma M5.1). Must terminate and must not depend on order.

## Vector 03: false with provenance

`send(a1)` is false because `ready(a1)` is false because `answered(a1, alex, yes)` is absent. Outcome is `clear`: a definite no, with its reason. Never undefined, never a crash.

## Vector 04: negation cycle

`p ← not q`, `q ← not p`: no stratification exists. Well-founded model: `p`, `q` undefined; `t ← p` inherits undefined; `r ← not s` is true because `s` has no rules. Outcome `opaque`. Negation-as-failure (Prolog) would loop or pick one; stratified Datalog would reject the program; stable models would give two answers. The well-founded model gives one.
