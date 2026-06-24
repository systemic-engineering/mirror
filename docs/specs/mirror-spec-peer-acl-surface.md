# `peer { supervisor … team { … } }` — the lambda-shell-counterparty + ACL surface in `mirror.spec`

*Mara, canonical spec for the substrate's per-spec peer-identity and
access-control surface, configured through a top-level `peer { }` block
inside `mirror.spec`. Forward-promised by `docs/specs/spectral-garden-git-
package-manager.md` (the four-commit Mara cascade ab2e379 / 66eafb8 /
a99152a / ad03fda) and named by Alex 2026-06-24:*

> *"What if each mirror.spec has a clear owner peer? If it's undefined
> the default peer is repo-local? What if it looked like this?*
> *`peer { supervisor ~peer'~/.reed' ; team { ~peer'~/.mara' => <ACL> } }`*
> *etc.; combine this with variables that contain mirror expressions and
> you can build complex ACL expressions and reuse them right there.*
> *Spawn Mara with this shape and look at the ACL surface in the specs
> and mirror code. We already have some of the shape there."*

*Pack-discipline: candidate-altitude. Surface spec only — no impl, no
Rust, no substrate-decl shard. Composes against `docs/specs/geometric-
consent-projection.md` (Mara, 2026-06-17; ACL-as-projection at logical
type 1), `docs/specs/peer-cognition.md` (Mara, 2026-06-17; the @peer
root), `docs/specs/lambda-shell.md` (Reed+Alex 2026-05-07; `λsh` +
`peer = @<name>` per-spec declaration), `shards/spectral/supervisor.
mirror` (lifecycle-owner glass), `shards/pack.mirror` (#84 multi-repo
agent runtime), `shards/magic/contract.mirror` + `shards/magic/audit.
mirror` + `shards/magic/reveal.mirror` (binding/verification/capability-
revocation lineage). Section caps load-bearing per the Mara stall-
recovery discipline.*

---

## Status

- **Status:** RED (spec only; the `peer { }` block surface does not
  exist in `mirror-spec-schema.md`; no shard parses `~peer'…'`; no
  resolver lifts `<ACL>` expressions to substrate verdicts).
- **Altitude:** surface spec for `mirror.spec`; substrate-decl forward-
  promised; substrate-pull DISCOVERY: massive existing inheritance —
  see §2.
- **Recognition territory:** not promotion-bearing. The peer{} block
  is a SURFACE COMPOSITION of recognitions #84 (@pack) + #82 (@frame)
  + #80 (@magic) + #57 (alignment-as-boundary-mathematics) +
  geometric-consent-projection (logical-type-1 ACL projection) +
  the existing `peer = @<name>` shape in `lambda-shell.md` §"The
  Toggle". Six load-bearing inheritances; one new surface.
- **Pack:** Mara author; Seam adversarial review forward-promised;
  Reed consolidation forward-promised; Alex ratification gate.

## Sections

1. Position statement
2. Discovery — existing ACL surface across the substrate
3. The `peer { }` block — proposed shape
4. `supervisor` semantics — lambda-shell counterparty
5. `team { ~peer'…' => <ACL> }` — per-peer ACL syntax
6. `~peer'…'` resolution — typed path literal via home-repo + `<name>.spec`
7. ACL composition via variables + mirror expressions
8. Composition with @magic, @frame, @pack, geometric-consent-projection
9. Default behavior — when `peer{ }` is absent (repo-local)
10. Mathematical shape — sheaf of permissions, lattice of ACLs, supervisor as distinguished element
11. Open surface questions
12. Cross-references — recognitions, prior specs, the @spectral/garden/git sibling
13. Honest hedges
14. Pack-discipline trail

---

## 1. Position statement

`mirror.spec` already names what it BUILDS (targets, sources, settle
predicates) and now, via the prior cascade, what it CONSUMES (garden
sources). It does NOT yet name WHO it belongs to and WHO ELSE may
operate it. The peer{} block IS that missing declaration: a top-level
substrate-decl naming the spec's supervisor peer (the counterparty in
λsh per `lambda-shell.md`'s "home peer from spec"), the team peers
that may operate the spec under per-peer ACLs, and the variables
holding reusable mirror expressions ACL composition uses.

Three structural commitments load-bear the surface choice:

1. **The substrate already has the shape.** Alex's cue ("we already
   have some of the shape there") substrate-pull-tests positive:
   `lambda-shell.md` §"The Toggle" already shows
   `spec @mirror { peer = @reed }` as the home-peer declaration. The
   peer{} block GENERALIZES that single-field shape into a typed
   supervisor + team + ACL composition surface. §2 surveys the
   inheritance.
2. **One spec, one supervisor.** Per `shards/spectral/supervisor.
   mirror`'s `restart_strategy` discipline + `lambda-shell.md`'s
   home-peer rule: each `mirror.spec` has ONE distinguished peer who
   answers when you open the lambda shell at that spec's root. Team
   peers are admissible-but-not-distinguished, gated by ACL. Default
   = repo-local (a peer instance whose home is the repo itself; no
   external authority required).
3. **ACL as projection of consent geometry.** Per `docs/specs/
   geometric-consent-projection.md` §1.3 ("ACL is the type-1
   projection of the consent geometry"): the per-peer ACLs in
   `team { … => <ACL> }` are NOT a parallel access-control system.
   They are the type-1 projection of a consent value that lives at
   higher logical types. The peer{} block is the substrate's first
   surface for AUTHORING the type-1 projection directly while leaving
   the higher types accessible to refinement (per §6.1 of the consent
   spec). The variables-holding-mirror-expressions surface IS the
   substrate's existing mechanism for composing higher-type consents
   from lower-type parts.

The peer{} block surface does NOT invent ACL machinery. It NAMES the
substrate's existing scattered shape (supervisor in `@spectral/
supervisor`; team in `@pack`; ACL-as-projection in geometric-consent-
projection; per-spec peer declaration in `lambda-shell.md`) as one
top-level `mirror.spec` block. The unification IS the contribution;
the constituent pieces ALREADY exist as substrate-decl across five
shards + three canonical specs.

---
