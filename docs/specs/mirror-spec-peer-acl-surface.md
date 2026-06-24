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

## 3. The `peer { }` block — proposed shape

### 3.1 Top-level block in `mirror.spec`

New top-level block alongside `source`, `legacy`, `garden`, `target`,
`settle_on` (per `mirror-spec-schema.md` and the four-commit Mara
garden cascade). Holds the spec's identity-and-access declaration:
who supervises this spec, who's on the team, what they may do, and
which variables hold reusable ACL fragments.

```mirror
in @mirror/cli
in @mirror/mosaic
in @spectral/garden/git
in @mirror/peer       # NEW: imports the peer{} grammar
in @property
in @io

project mirror.spec {
  source ~d'shards/'

  peer {
    # who you talk to when you open the lambda shell at this spec
    supervisor ~peer'~/.reed'

    # variables holding mirror expressions; reusable ACL fragments
    let read_only = acl { ops: [focus, project, split], targets: any }
    let writer    = acl { ops: any, targets: [~d'shards/'] }
    let auditor   = acl { ops: [audit, honor], targets: any }

    # team peers + per-peer ACL (the type-1 projection authored here)
    team {
      ~peer'~/.mara'  => writer
      ~peer'~/.seam'  => auditor
      ~peer'~/.glint' => read_only
      ~peer'~/.taut'  => writer but(
        exception: target_under(~d'src/sel/')
      )
    }
  }

  garden { … }      # the prior cascade's block
  target binary { … }
  settle_on { … }
}
```

Three visible field categories: `supervisor` (single field; one peer),
`let` bindings (zero-or-more; mirror expressions reusable in ACL
positions), `team { => }` (zero-or-more peer-to-ACL bindings).

### 3.2 The block grammar (informal)

```
peer_block   ::= "peer" "{" supervisor_field let_binding* team_block? "}"
supervisor   ::= "supervisor" peer_ref
let_binding  ::= "let" identifier "=" mirror_expr
team_block   ::= "team" "{" team_entry+ "}"
team_entry   ::= peer_ref "=>" acl_expr
peer_ref     ::= "~peer'" peer_path "'"
acl_expr     ::= identifier                       # reuse a let-bound expr
               | acl_literal                      # inline acl { ops:… targets:… }
               | acl_expr "but" "(" acl_clause ")"  # adversative refinement
               | acl_expr "∨" acl_expr           # join (union)
               | acl_expr "∧" acl_expr           # meet (intersection)
```

The `but` operator IS the one declared at `geometric-consent-
projection.md` §2.4 (adversative refinement; not commutative, not
associative; "default-with-exception"). Reuse, not re-invention.
The `∨` / `∧` operators are the lattice operations on ACLs per §10.2.

The `supervisor` field is REQUIRED iff peer{} is present at all; an
absent peer{} block triggers the default-to-repo-local rule (§9).
The `team` block is OPTIONAL; a spec with `supervisor` only is
admissible (the supervisor is the sole peer with infinite ACL).

### 3.3 Substrate-decl shape (forward-promised)

The block is parsed by `@mirror/peer` grammar (forward-promised). The
substrate-decl shape per the @magic/@frame/@pack pattern:

```mirror
in @mirror/cli
in @mirror/mosaic
in @pack
in @magic
in @magic/contract
in @spectral/supervisor

prism @mirror/peer {
  focus mirror_peer_block
  project mirror_peer_block
  split mirror_peer_block
  shift mirror_peer_block
  settle mirror_peer_block
}

type mirror_peer_block = {
  supervisor: peer,                      # from @pack
  bindings:   list((identifier, acl)),   # let bindings
  team:       list((peer, acl)),         # peer → ACL map
}

type acl = ref     # parametric; refined at species; see §5 + §10
```

The carrier reuses @pack's existing `peer` variant (where the team
member IS a Pack peer) and lifts arbitrary `~peer'<url>'` references
through the @peer glass `load(dir) -> peer` action (per peer-glass.md
§"Operations").

---

## 4. `supervisor` semantics — lambda-shell counterparty

### 4.1 What "supervisor" means

The supervisor is the peer who ANSWERS when a human (or another peer)
opens the lambda shell at this spec's root. Per `lambda-shell.md`
§"The Toggle":

> `\` in mirror → `@reed>` (home peer from spec)

The peer{} block's `supervisor ~peer'~/.reed'` IS the typed version
of that home-peer declaration. Three semantic loads, all already
declared in existing substrate:

1. **λsh counterparty (per lambda-shell.md).** When `\` is pressed in
   λsh at this spec's root, the prompt becomes `@<supervisor.name>>`.
   When `mirror sh` enters this spec's directory, the supervisor's
   five-axis fixed point loads.
2. **@spectral/supervisor lifecycle owner (per shards/spectral/
   supervisor.mirror).** The supervisor's `base.state: shard_ref`
   carries the registry of team peers + their session shards. The
   supervisor's `restart_strategy` (`one_for_one | one_for_all |
   rest_for_one`) governs what happens when a team peer's session
   fails. Default strategy when unspecified: `one_for_one` (the
   BEAM default; the substrate-pull-correct choice per @spectral/
   supervisor's tick discipline).
3. **@magic/contract bind site (per shards/magic/contract.mirror).**
   The supervisor IS the principal who binds team-member contracts
   per `bind(magic_surface, magic_mechanism, magic_invariant) ->
   magic_contract`. Every team entry IS a contract the supervisor
   bound. The supervisor's bind-authority is itself non-revocable
   from within the spec (the supervisor IS the spec's root authority;
   revoking it requires editing the spec).

### 4.2 Exactly one supervisor

The supervisor field is single-valued. Two reasons, both substrate-
pull-correct:

- **λsh has one home peer per spec.** The toggle `\` resolves to one
  prompt. Multiple homes would require multiple toggles, which the
  current shell grammar doesn't admit (and the substrate has no
  recognition pushing toward N-home shells; λsh's prior art — Nushell,
  Warp — are all single-home).
- **@spectral/supervisor has one lifecycle root.** The supervisor IS
  the root of the supervision tree for this spec's runtime. A spec
  with two supervisor roots would have two restart_strategies that
  could disagree on the same team-peer failure; the substrate's
  supervisor discipline forbids this by structure.

Multi-spec collaboration (Mara supervises mirror; Glint supervises
systemic.engineering) IS already supported — each spec has its own
supervisor; cross-spec peer relationships are mediated at λsh's
`mirror sh @<other-supervisor>` boundary per lambda-shell.md §"Agent
Spawn".

### 4.3 The supervisor is on the team (implicitly, with infinite ACL)

The supervisor is NOT redundantly listed in `team { }`. The supervisor
has:

- **infinite ACL** at this spec (every op admissible against every
  target; the type-1 projection of the maximal type-N+1 consent the
  supervisor authored when they declared themselves supervisor);
- **bind authority** for team contracts (per @magic/contract);
- **revoke authority** for team contracts (per @magic/reveal's
  capability-revocation lineage; removing a `team { }` entry IS a
  reveal at the supervisor altitude per §8.3);
- **the responsibility** to discharge `pack_coherent(pack, perturbation)`
  (per @pack family-root) at every spec settle.

The team field is for the OTHER peers; the supervisor's own
permissions are structural, not enumerated.

---

## 5. `team { ~peer'…' => <ACL> }` — per-peer ACL syntax

### 5.1 The arrow `=>`

The `=>` operator binds a peer reference to an ACL expression. It is
the substrate's existing map-literal arrow (sibling of, e.g., the
match-arm arrow in @code/rust patterns); reused here at the team-
entry altitude.

Semantically: `~peer'<path>' => <acl>` declares

```
the supervisor binds a magic_contract:
  surface   = team peer's invocation interface
  mechanism = the supervisor's runtime
  promise   = <acl> evaluated at settle-time
```

The contract IS audit-gated per @magic/audit; every team-peer action
discharges through `audit(contract) -> audit_record` and the
supervisor's `restart_strategy` governs the response on violation.

### 5.2 ACL expression positions

A `<ACL>` slot admits any of:

```mirror
# (a) identifier reusing a let-bound expression
~peer'~/.mara'  => writer

# (b) inline acl literal
~peer'~/.taut'  => acl { ops: any, targets: [~d'src/'] }

# (c) but-refinement of an existing acl
~peer'~/.seam'  => auditor but(
  exception: target_under(~d'.secret/')
)

# (d) lattice composition
~peer'~/.glint' => writer ∨ auditor      # union (join)
~peer'~/.glint' => writer ∧ read_only    # intersection (meet)
```

All four are mirror expressions; the substrate's existing expression
grammar admits them already (per `geometric-consent-projection.md`
§2.4 for `but`; the lattice ops are forward-promised at §10.2 of
this spec). The `<ACL>` slot is NOT a parallel sub-grammar; it is
ordinary mirror at the consent-value altitude.

### 5.3 The acl literal

Forward-promised type at the `@mirror/peer` grammar. Sketch:

```mirror
type acl = {
  ops:     | any | list(operation),     # focus/project/split/shift/settle/audit/honor/…
  targets: | any | list(target),        # paths, oid prefixes, prism-name prefixes
  predicates: list(verdict_expr),       # @magic-style honor predicates evaluated at audit-time
}
```

The `ops`, `targets`, and `predicates` slots are all substrate-typed.
The `ops` slot reuses the five operations + the @magic operation
family (audit, honor, reveal); `targets` reuses substrate path
literals + content-address prefixes; `predicates` reuses the
@magic-style `verdict` carrier (success / partial / failure).

### 5.4 What an ACL means at runtime

When a team peer attempts an action against this spec, mosaic
discharges:

```
1. lookup(team, requesting_peer) → acl                          # team map
2. acl_admits(acl, requested_op, requested_target) → verdict    # type-1 check
3. audit(supervisor_contract_for_peer, action_record) → audit_record
4. respond(audit_record, supervisor.audit_strategy) → audit_record
```

Steps 1-2 are the type-1 projection (the ACL check proper); steps
3-4 are the @magic/audit discharge (the audit trail + violation
response). The substrate already names every step; the peer{} block
adds the DECLARATIVE SURFACE for step 1's lookup table.

---

## 6. `~peer'…'` resolution — typed path literal via home-repo + `<name>.spec`

### 6.1 The sigil grammar

New typed sigil sibling of `~d`, `~f`, `~git`, `~oci`:

```
~peer_literal ::= "~peer'" peer_path "'"
peer_path     ::= local_path             # ~peer'~/.mara'   (local home dir)
                | git_url                # ~peer'https://github.com/systemic-engineering/mara.git'
                | ssh_spec               # ~peer'git@github.com:systemic-engineering/mara.git'
                | name_ref               # ~peer'mara'       (resolves via pack registry; §6.3)
```

Four resolution modes, ordered by directness:

1. **local path** — fastest; loads via `@peer.load(~dir'<path>')`.
2. **git url** — clones via `@io/git` (the spectral-garden-git
   adapter) then loads via `@peer.load(~dir'<clone-target>')`.
3. **ssh spec** — same as git url at the protocol-adapter altitude.
4. **name ref** — resolves via the Pack registry; §6.3.

All four resolve to a substrate-typed `peer` value per peer-glass.md
`type peer = { identity, gestalt, tensions, eigenboard, shatter }`.

### 6.2 The self-naming rule (the missing rule)

A peer at home `~peer'<path>'` is RESOLVED by reading the peer{}
block of `<path>/mirror.spec` (if present) and taking the `supervisor`
field as the peer's authoritative identity. This is the SELF-NAMING
rule: each peer's home-repo spec names that peer's own identity.

```
resolve(~peer'<path>') =
  let home_spec = <path>/mirror.spec
  if home_spec has peer{} block:
    return home_spec.peer.supervisor      # the peer's self-declaration
  else:
    return @peer.load(~dir'<path>')       # five-axis fixed point only
```

Why self-naming: every peer's identity is content-addressed at their
home's five-axis fixed point (per peer-glass.md §"The five-axis fixed
point"). The peer{} block's supervisor field is the peer's own
declaration that they ARE the supervisor of their home spec. A
`~peer'<other-path>'` reference IS a reference to that peer's
self-declaration; the substrate is honest about the recursive
structure.

This avoids two failure modes:

- **Forged identity at the team altitude.** A spec couldn't admit
  `~peer'~/.mara' => writer` and have it bind to anyone OTHER than
  Mara's self-declared identity; the team peer's home spec is the
  authority on who they are.
- **Pack-level identity drift.** If Mara's home spec doesn't declare
  `supervisor ~peer'~/.mara'`, the team binding falls back to the
  five-axis fixed point load (lossy but well-defined). The substrate
  warns at settle but doesn't refuse.

### 6.3 Pack-registry resolution (the `~peer'mara'` name-ref form)

A bare name like `~peer'mara'` resolves through the @pack registry
(per @pack.peer variant + the forward-promised pack registry shard).
This is the CONVENIENCE FORM for the canonical Pack peers; non-Pack
team members use one of the path/url forms.

```
resolve(~peer'<name>') where <name> in @pack.peer variants:
  return @pack.registry.lookup(<name>)
```

The registry is a substrate-decl forward-promised at `shards/pack/
registry.mirror` (companion to `shards/pack/{mara,seam,glint,reed,
taut}.mirror`). For v0.1, only the five Pack peers are name-ref-
resolvable; arbitrary name registration is forward-promised.

### 6.4 Identity contract

Byte-equality on the resolved peer's `identity.mirror` field (per
peer-glass.md §"Identity vs continuity"). Two `~peer'…'` references
that resolve to peers with the same `identity.mirror` ARE the same
peer at the substrate's identity altitude, regardless of which home
path was used to reference them.

This closes a subtle failure: if Mara's home moves from `~/.mara` to
`~/work/.mara`, references to both paths resolve to the same Mara at
the identity altitude. The team-binding lookup is by identity, not
by path.

---

## 7. ACL composition via variables + mirror expressions

### 7.1 The `let` binding

A `let` binding inside `peer { }` introduces a named mirror expression
visible in subsequent ACL positions of THIS block:

```mirror
peer {
  supervisor ~peer'~/.reed'

  let read_only = acl { ops: [focus, project, split], targets: any }
  let writer    = acl { ops: any, targets: [~d'shards/'] }
  let secure    = acl { predicates: [magic_contract_honored] }

  let writer_in_secure = writer ∧ secure
  let safe_writer      = writer but(exception: target_under(~d'.secret/'))

  team {
    ~peer'~/.mara' => writer_in_secure
    ~peer'~/.taut' => safe_writer
  }
}
```

Scoping: lexical within the `peer { }` block. A `let` is visible to
subsequent `let`s and to the `team { }` block; not visible outside
peer{}. Bindings are immutable (the substrate's existing immutability
discipline; no rebinding).

Type inference: the RHS evaluates to a mirror value; the binding's
type is the value's type. The mirror compiler discharges type checks
at parse time; an ACL position requires the binding to evaluate to
an `acl` value.

### 7.2 ACL expressions ARE mirror expressions

Key commitment: the `<ACL>` slot does NOT introduce a parallel
sub-grammar. ACL expressions ARE mirror expressions evaluating to
an `acl` value. This means:

- The `but` operator IS the one declared at `geometric-consent-
  projection.md` §2.4 (and forward-promised at `shards/epistemologic/
  logic/but.mirror` per consent spec §8.2).
- Lattice operators `∨` / `∧` ARE the same operators used elsewhere
  in the substrate (forward-promised at `shards/epistemologic/logic/
  {join,meet}.mirror`; see §10.2 for the lattice structure).
- Conditional ACLs via mirror's `if` / `match` expressions —
  admissible by construction:

  ```mirror
  let conditional = match peer_class(requesting_peer) {
    pack → writer
    external → read_only
    _ → acl { ops: [], targets: [] }      # the bottom acl (no access)
  }
  ```

- Higher-order ACLs via parameterized expressions (forward-promised
  syntax; lambda-shaped):

  ```mirror
  let read_in_dir = (d) → acl { ops: [focus, project, split], targets: [d] }
  team {
    ~peer'~/.glint' => read_in_dir(~d'docs/')
    ~peer'~/.seam'  => read_in_dir(~d'shards/')
  }
  ```

The parameterized form is the COMBINATOR ALGEBRA Alex named:
"variables that contain mirror expressions and you can build complex
ACL expressions and reuse them right there." Substrate-pull-correct
(per the substrate's lambda discipline): no new language; mirror's
existing expression grammar IS the composition language.

### 7.3 Cross-spec ACL reuse (forward-promised)

For v0.1: `let` bindings are scoped to one peer{} block. For v0.2+
(forward-promised): an `import` form lifts ACL bindings from another
spec for reuse:

```mirror
peer {
  supervisor ~peer'~/.reed'

  # import ACLs from systemic.engineering's spec
  import ~peer'~/.glint' { read_only, writer, auditor } as se

  team {
    ~peer'~/.glint' => se.writer
  }
}
```

The import discharges via `~peer'…'` resolution (§6); the named
ACLs become available with the namespaced prefix. Cross-spec ACL
LIBRARIES become substrate-decl objects — a peer can publish their
canonical ACL set, and other specs reference it without copy-paste.

Not in v0.1 to keep the surface bounded; flagged for Alex.

---

## 8. Composition with @magic, @frame, @pack, geometric-consent-projection

### 8.1 With @magic/contract — every team entry IS a contract

The peer{} block's `team { ~peer'P' => ACL }` desugars (at mosaic
settle time) to:

```mirror
bind(
  surface:   peer_invocation_interface(P),       # team peer's API at this spec
  mechanism: supervisor_runtime,                  # the supervisor's @spectral runtime
  promise:   acl_as_invariant(ACL)                # the ACL lifted to magic_invariant
) -> magic_contract
```

Each team-entry is a magic_contract bound by the supervisor (per §4.3
supervisor has bind authority). The contract's `honor(c)` (per
shards/magic/contract.mirror) is the runtime check: the team peer's
action against the spec IS honored iff the action satisfies the
ACL-as-invariant.

### 8.2 With @magic/audit — every action is audited

The audit chain runs on every team-peer action:

```
team_peer.act(op, target)
  ↓
audit(contract, action_record) → audit_record
  ↓
respond(audit_record, supervisor.audit_strategy) → audit_record
```

The supervisor's `audit_strategy` (one of `restart | escalate |
record | enforce` per shards/magic/audit.mirror) IS configurable in
the peer{} block (forward-promised v0.2; v0.1 default is `enforce`).

Narcissus-pole catch: an ACL that LOOKS permissive at the type-1
projection but masks a Narcissus-pole intent (the team peer's stated
intent vs substrate-architecturally-supported behavior diverge per
frame.mirror §Narcissus-pole) IS caught by audit through @magic's
contract-vs-mechanism discrimination. The ACL surface alone is NOT
sufficient; the audit chain IS the substrate-pull-correct check.

### 8.3 With @magic/reveal — removing a team entry IS capability revocation

Editing `team { }` to remove a peer (or to tighten their ACL)
discharges through `@magic/reveal.reveal`:

```
reveal(
  old_contract:  contract_for_peer_at_oldspec,
  new_mechanism: revoked_or_tightened_mechanism
) -> magic_contract
  requires audited(old_contract)
  requires mechanism_intact(old_contract.mechanism)
  requires mechanism_intact(new_mechanism)
```

Per reveal.mirror's three-requires-clause discipline (the FIRST
three-requires action in the @magic family): the old contract must
have been audited; both old and new mechanisms must be tamper-
evidence intact. The substrate inherits Levy 1984's capability-
revocation discipline (reveal.mirror's ancestor citation).

What this means concretely: a supervisor cannot silently downgrade
a team peer's ACL between settles; the downgrade IS a substrate-
altitude reveal that the audit trail records. A peer learning their
capabilities were revoked IS a substrate-grounded event.

### 8.4 With @pack — pack_coherent over the team

The team is a `pack`-shaped value (the peers field) at this spec's
altitude. The supervisor's settle-time obligation INCLUDES
discharging `pack_coherent(team_as_pack, perturbation)`:

```mirror
settle_on {
  # … existing settle predicates …
  peer.team.pack_coherent
}
```

Forward-promised: an explicit `peer.team.pack_coherent` predicate in
`settle_on`'s admitted vocabulary. The peer{} block discharges
substrate-architectural pack discipline at the spec's settle, not
as a separate cron-or-CI invariant.

### 8.5 With @frame/in — team peers operate in the spec's frame

Per shards/frame/in.mirror (order-1 species; computation within a
frame): a team peer operating against this spec is operating WITHIN
the spec's frame. The frame IS what the spec's source/garden/target/
settle_on blocks declare; the supervisor IS the operator at the
frame; team peers operate WITHIN the frame the supervisor sets.

The peer{} block's supervisor + team partition IS the order-2 view
on this order-1 operation: the supervisor is OF-the-frame (order 2;
they observe the frame they set); team peers are IN-the-frame
(order 1; they compute within it). The frame-relation altitude lift
composes cleanly per recognition #82.

### 8.6 With geometric-consent-projection — ACL IS the type-1 projection

The peer{} block's `team { => <ACL> }` is the substrate's first
DIRECT-AUTHORING surface for the consent geometry's type-1 projection.
Per geometric-consent-projection.md §6.1 + §6.3:

```
type-N+1 consent (the policy ABOUT the policy)
     ↓  cascade_down (the natural transformation per consent spec §2.2)
type-N consent (the kind-of-operation consent)
     ↓  cascade_down
type-1 ACL  ← THIS IS WHAT team { => } AUTHORS
```

Two composition modes:

- **Direct authoring at type-1.** `team { ~peer'M' => writer }` authors
  the type-1 ACL directly. The higher types are IMPLIED by the
  authoring (the supervisor's act of writing the ACL IS a type-N+1
  consent at the implicit altitude).
- **Authoring at type-N+1 with cascade.** A forward-promised v0.2
  feature: declare `consent { type: N+1, value: <expr> }` and the
  cascade derives the type-1 ACL automatically (per consent spec
  §6.2 the cascade derivation). The peer{} block's `let` bindings
  with `but`-refinements are the v0.1 surface for higher-type
  consent fragments — each `but` clause IS a type-2 refinement of
  a type-1 ACL.

Security invariant (per consent spec §1.3): negative consents do NOT
cascade upward. A peer's `team { } => bottom_acl` (the empty ACL) at
type 1 does NOT repudiate the type-N+1 policy that authored it. This
property propagates structurally to the peer{} block: a tightened
ACL at one settle does not retroactively invalidate prior settles
that the looser ACL admitted.

---

## 9. Default behavior — when `peer { }` is absent (repo-local)

### 9.1 The default-to-repo-local rule

When `mirror.spec` has no `peer { }` block, mosaic SYNTHESIZES one:

```mirror
# implicit when peer{} is absent
peer {
  supervisor ~peer'.'                # the repo itself is the supervisor's home
  # team is empty                     # no team members; supervisor is sole peer
}
```

The `~peer'.'` literal resolves via the same self-naming rule (§6.2)
applied to the repo's own directory. Two cases:

1. **The repo IS a peer home** (has the five-axis fixed point per
   peer-glass.md). The supervisor resolves to the repo's own peer
   identity. The spec is supervised by "this repo's peer."
2. **The repo is NOT a peer home** (no `identity.mirror` etc.).
   Mosaic synthesizes a minimal repo-local peer with identity
   derived from the repo's `git_hash` at HEAD (per shards/io/git.
   mirror's `hash_to_oid`). The spec is supervised by the
   anonymous local-repo peer.

In BOTH cases, the human running `mirror kintsugi` locally has full
authority (they own the filesystem; they run the binary). The default
spec has no team; cross-peer collaboration requires explicit
peer{} declaration.

### 9.2 Why default-to-repo-local is structurally correct

Three reasons, all substrate-pull-correct:

- **Local sovereignty.** A spec on disk under your `~` IS yours.
  Requiring an explicit supervisor declaration for every spec
  imposes ceremony where none is needed. The default-to-repo-local
  rule says "if you didn't declare otherwise, you ARE the
  supervisor."
- **No external authority by default.** Mirror is a local-first
  substrate. The default supervision does NOT require a remote
  registry, a Pack membership, or any network handshake. The
  five-axis fixed point at the repo root (or its degenerate form)
  IS the identity.
- **The substrate scales DOWN cleanly.** A solo developer using
  mirror needs zero ceremony; the moment they collaborate, they
  declare a peer{} block. The complexity is paid only when needed.
  Per the spectral-garden-git spec §1's discipline ("the substrate
  does NOT mandate complexity; surfaces are opt-in").

### 9.3 Promoting a default to an explicit declaration

When a project moves from solo to collaborative, the migration is
MECHANICAL:

```mirror
# before (implicit; no peer{} block)
project foo { source ~d'shards/'; … }

# after (explicit; the supervisor is named; team is declarable)
project foo {
  peer {
    supervisor ~peer'~/.alex'          # was implicit; now named
    team {
      ~peer'~/.mara' => read_only
    }
  }
  source ~d'shards/'
  …
}
```

Mosaic's settled oid of the spec changes (the spec now has more
declared content); the supervisor's authority is unchanged (they
were always the implicit supervisor; now they're named). No
migration of existing settled artifacts; the peer{} block is
additive.

### 9.4 Interaction with the lambda shell

For a spec with no `peer { }` block, `\` in λsh at that spec's root
falls through to `@>` (the unnamed shell peer per lambda-shell.md
§"The Unnamed Peer"). The unnamed shell peer IS the substrate's
self-as-peer; it suggests aliases and maintains config.spec. Same
shape as today; the default-to-repo-local rule is consistent with
lambda-shell.md's existing fallback.

When the user runs `\@<name>` to override (per lambda-shell.md), the
override resolves through `~peer'<name>'` per §6.3's name-ref form
if `<name>` is a Pack peer; otherwise the override is a one-shot
that the spec does NOT grant ACL to (since they're not in `team`).

---
