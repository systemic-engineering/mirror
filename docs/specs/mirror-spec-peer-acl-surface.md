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

## 2. Discovery — existing ACL surface across the substrate

Alex's cue "we already have some of the shape there" was substrate-
pull-correct. The discovery sweep surfaced SIX load-bearing existing
shapes; the peer{} block is their unification surface, not a new
machinery. Each row names a SHAPE the substrate already carries; the
§3 surface composes them.

### 2.1 `peer = @<name>` in `mirror.spec` — already declared

`docs/specs/lambda-shell.md` §"The Toggle" (lines 36-52) ALREADY
shows the per-spec peer declaration:

```mirror
spec @mirror {
  peer = @reed
}

spec @systemic.engineering {
  peer = @glint
}
```

Use: `\` in λsh resolves to `@reed>` in mirror; `@glint>` in
systemic.engineering. This is the SUPERVISOR field of the peer{}
block at single-field altitude. The lambda-shell spec uses keyword
`spec` rather than `project` (per the `mirror.spec` grammar's actual
top-level — `project mirror.spec { … }` per `mirror.spec:18`); the
shape, however, IS the supervisor declaration.

The peer{} block GENERALIZES: from one field (`peer = @reed`) to a
typed block with a supervisor, a team, and a per-team-member ACL.

### 2.2 `@peer` glass — five-axis fixed point at typed directory

`docs/specs/peer-glass.md` (Reed+Alex 2026-05-25) declares the typed
surface a peer carries:

```mirror
type peer = {
  identity:    mirror,        # focus(self)
  gestalt:     mirror,        # project(self)
  tensions:    mirror,        # split(self)
  eigenboard:  spec,          # shift(self)
  shatter:     mirror,        # settle(self)
}

load(dir: ~dir) -> peer { \ }
```

And: `@peer(~dir"<path>")` is the existing INSTANTIATION SYNTAX. The
peer's home directory IS the manifest material form; the five-axis
fixed point IS the identity contract. `peer-cognition.md` (1449 lines,
Mara 2026-06-17) extends this to a full @peer root cognition spec.

The `~peer'…'` literal proposed in §6 IS a typed sibling of
`~dir"…"` plus the existing `~git'…'` (the spectral-garden-git
sibling spec, ad03fda) — a typed path literal whose resolution loads
the five-axis peer per `peer-glass.md`'s `load(dir)` action.

### 2.3 `@pack` — team membership + multi-repo span

`shards/pack.mirror` (#84) declares the substrate's existing team
shape:

```mirror
type peer = | mara | seam | glint | reed | taut
type pack = {
  peers:       ref,             # list of peer
  repos:       ref,             # list of repository
  repo_span:   repository_span,
  runtime:     runtime,
}
pack_coherent(pk: pack, p: perturbation) -> verdict { \ }
```

The `team { … }` field of the peer{} block IS a `pack`-shaped
declaration scoped to one `mirror.spec`. The variant enum (mara |
seam | glint | reed | taut) is the substrate's existing Pack
membership type; arbitrary peers (non-Pack home repos) lift via
`~peer'<url>'` to a `peer`-typed reference per `peer-glass.md`'s
load action.

`shards/smarts/pack.mirror` (the @smarts/pack adapter) declares
`pack_satisfies_smarts` as the doubled-bilateral discipline check
for a pack composing with the @smarts substrate-architectural
integration. The peer{} block inherits this composition discipline
automatically when settle visits the team field.

### 2.4 `@spectral/supervisor` — lifecycle owner at runtime altitude

`shards/spectral/supervisor.mirror` (474 lines; the second sub-shard
of the @spectral runtime cascade) declares:

```mirror
type supervisor = {
  base:             gen_prism,
  child_specs:      [child_spec],
  restart_strategy: restart_strategy,
}
start_child(s: supervisor, spec: child_spec) -> gen_prism { \ }
terminate_child(s: supervisor, id: uuid_spectral) -> au { \ }
```

The `supervisor ~peer'~/.reed'` field of the peer{} block is the
DECLARATIVE SURFACE for a `@spectral/supervisor` instance scoped to
one `mirror.spec`. The supervisor's `base.state` (a `shard_ref`) IS
the registry of peers operating under this spec. The lifecycle
actions (start_child / terminate_child) discharge automatically per
@spectral's restart_strategy when team peers join + leave.

### 2.5 `@magic/contract` + `@magic/audit` + `@magic/reveal` — the verification + revocation lineage

The @magic family closes the verification side of ACL:

- `shards/magic/contract.mirror`: `bind(surface, mechanism,
  promise) -> magic_contract` + `honor(c) -> transparency(c)`. The
  ACL IS a magic_contract: the surface is the team-member's
  invocation interface, the mechanism is what they can do in the
  supervisor's runtime, the promise (magic_invariant) is the ACL
  expression.
- `shards/magic/audit.mirror`: `audit(c) -> audit_record` +
  `respond(record, strategy)`. Every team-member action against the
  spec discharges through audit; the audit_strategy variant
  (`restart | escalate | record | enforce`) IS the policy-violation
  response.
- `shards/magic/reveal.mirror`: cites `capability revocation; revoke
  and re-grant (Levy 1984). The substrate-pull-correct controlled
  disclosure with audit gating.` The reveal action (`reveal(c, new_m)`)
  IS the substrate's CAPABILITY-REVOCATION primitive. Removing a peer
  from `team { }` is a reveal at the supervisor altitude; the
  audit_strategy=enforce closes the previously-honored contract.

### 2.6 Geometric consent projection — ACL as type-1 projection

`docs/specs/geometric-consent-projection.md` (1447 lines; Mara
2026-06-17) establishes that ACL is NOT a foundational primitive
but a PROJECTION at logical type 1 of a richer consent geometry
over the Bateson logical-type tower (§2.3 of the consent spec; §6.2
for the cascade derivation):

```
project_at(scope, type=1) → acl    (filesystem-style permission bits)
project_at(scope, type=2) → kind-of-operation consent
project_at(scope, type=3) → frame consent (reasoning about ops)
project_at(scope, type=N+1) → policy about the policy
```

Cascade direction: positive consents at type N+1 imply matching
type-N consents (DOWNWARD); negative consents at type N do NOT
repudiate the type-N+1 author (NO UPWARD CASCADE; the security
invariant per the consent spec §1.3).

The `<ACL>` slot in `team { ~peer'…' => <ACL> }` is the type-1
projection. Variables-holding-mirror-expressions (§7) are the
substrate's existing mechanism for AUTHORING the higher-type
consent that cascades down to the projection.

### 2.7 The `~git'…'` precedent — typed path literals for substrate-refs

`shards/io/git.mirror` (a1b507a) + the spectral-garden-git spec
(ad03fda) established the `~git'…'` typed path literal: a sigil
that parses to a substrate-typed `git_repository + git_ref` pair.

The `~peer'…'` literal proposed in §6 is a sibling at the peer
identity altitude: parses to a substrate-typed `peer + home_repo`
pair, resolved via the home repo's `<name>.spec` peer{} block per
the self-naming rule (§6.2). Companion sigils: `~d`/`~dir`,
`~f`/`~file`, `~git`, now `~peer`.

### 2.8 Discovery summary

The peer{} block is a SURFACE COMPOSITION over:

| Existing shape | Source | Role in peer{} block |
|---|---|---|
| `peer = @<name>` | lambda-shell.md §Toggle | the `supervisor` field |
| `@peer(~dir"<path>")` glass | peer-glass.md | the type of resolved `~peer'…'` |
| `@pack` variant + record | shards/pack.mirror | the `team { }` field |
| `@spectral/supervisor` | shards/spectral/supervisor.mirror | runtime-altitude semantics for `supervisor` |
| `@magic/contract` + `audit` + `reveal` | shards/magic/*.mirror | bind/verify/revoke for ACLs |
| `geometric-consent-projection` | docs/specs/geometric-consent-projection.md | `<ACL>` IS the type-1 projection |
| `~git'…'` precedent | shards/io/git.mirror (a1b507a) | sigil pattern for `~peer'…'` |

What's IMPLIED but not yet declared: the BLOCK SYNTAX that names all
seven simultaneously at one mirror.spec altitude. What's MISSING: the
`<name>.spec` self-naming rule for `~peer'…'` resolution (proposed
§6.2); the variable scope that lets ACL expressions reuse each other
within one peer{} block (proposed §7); the default-to-repo-local
rule when peer{} is absent (proposed §9).

The spec proposes the BLOCK + the THREE MISSING RULES. Everything else
is the substrate composing with itself.

---
