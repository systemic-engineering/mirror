# `pack { elder … members { … } }` — the lambda-shell-counterparty + ACL surface in `mirror.spec`

*Mara, canonical spec for the substrate's per-spec pack-identity and
access-control surface, configured through a top-level `pack { }` block
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

*Vocabulary note: Alex's probe used `peer { supervisor … team { … } }`.
The cascade through Pack discussion (Alex 2026-06-24 afternoon)
renamed the block to `pack { }`, the role-noun to `elder`, and the
sub-block to `members { }` — because `peer` is the TYPE of an entry
(`~peer'…'` resolves to a `peer`-typed value per `peer-glass.md`),
while `pack` is the GROUP containing them (per `shards/pack.mirror`'s
@pack family-root). The block names the pack scoped to one spec; the
elder is the pack's N+1 observer; members are the antichain below.
The original probe vocabulary is preserved as Alex's verbatim cue
throughout; the substrate vocabulary settled where the substrate's
existing nouns already lived. The spec filename keeps the historical
name `mirror-spec-peer-acl-surface.md` for commit-history continuity.*

*Pack-discipline: candidate-altitude. Surface spec only — no impl, no
Rust, no substrate-decl shard. Composes against `docs/specs/geometric-
consent-projection.md` (Mara, 2026-06-17; ACL-as-projection at logical
type 1), `docs/specs/peer-cognition.md` (Mara, 2026-06-17; the @peer
root), `docs/specs/lambda-shell.md` (Reed+Alex 2026-05-07; `λsh` +
`peer = @<name>` per-spec declaration), `shards/spectral/supervisor.
mirror` (lifecycle-owner glass; lifecycle-altitude composition only —
the elder of this spec is NOT delegation-chain-shaped, see §10),
`shards/pack.mirror` (#84 multi-repo agent runtime — the family-root
that `pack { }` extends), `shards/magic/contract.mirror` +
`shards/magic/audit.mirror` + `shards/magic/reveal.mirror`
(binding/verification/capability-revocation lineage). Section caps
load-bearing per the Mara stall-recovery discipline.*

---

## Status

- **Status:** RED (spec only; the `pack { }` block surface does not
  exist in `mirror-spec-schema.md`; no shard parses `~peer'…'`; no
  resolver lifts `<ACL>` expressions to substrate verdicts).
- **Altitude:** surface spec for `mirror.spec`; substrate-decl forward-
  promised; substrate-pull DISCOVERY: massive existing inheritance —
  see §2.
- **Recognition territory:** not promotion-bearing. The pack{} block
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
3. The `pack { }` block — proposed shape
4. `elder` semantics — lambda-shell counterparty + spawn-and-probe responsibility
5. `members { ~peer'…' => <ACL> }` — per-member ACL syntax
6. `~peer'…'` resolution — typed path literal via home-repo + `<name>.spec`
7. ACL composition via variables + mirror expressions
8. Composition with @magic, @frame, @pack, geometric-consent-projection
9. Default behavior — when `pack{ }` is absent (repo-local)
10. Mathematical shape — antichain of members, ACL lattice, elder as spawn-and-probe N+1 observer
11. Open surface questions
12. Cross-references — recognitions, prior specs, the @spectral/garden/git sibling
13. Honest hedges
14. Pack-discipline trail

---

## 1. Position statement

`mirror.spec` already names what it BUILDS (targets, sources, settle
predicates) and now, via the prior cascade, what it CONSUMES (garden
sources). It does NOT yet name WHO it belongs to and WHO ELSE may
operate it. The pack{} block IS that missing declaration: a top-level
substrate-decl naming the spec's elder (the counterparty in λsh per
`lambda-shell.md`'s "home peer from spec", AND the spawn-and-probe
responsible at N+1 per the spectral-Tomm machinery), the pack members
that may operate the spec under per-member ACLs, and the variables
holding reusable mirror expressions ACL composition uses.

Three structural commitments load-bear the surface choice:

1. **The substrate already has the shape — and the noun.** Alex's cue
   ("we already have some of the shape there") substrate-pull-tests
   positive at two altitudes: `lambda-shell.md` §"The Toggle" already
   shows `spec @mirror { peer = @reed }` as the home-peer declaration,
   AND `shards/pack.mirror` (#84) already carries the `pack` family-
   root with `peer` as variant type and `pack` as the group record.
   The pack{} block GENERALIZES the single-field home-peer shape into
   a typed elder + members + ACL composition surface, USING the
   substrate's already-declared `pack`/`peer` partition. §2 surveys
   the inheritance.
2. **One spec, one elder.** Per `lambda-shell.md`'s home-peer rule
   plus the spectral-Tomm probe machinery
   (`architecture-error-as-tomm-probe`,
   `architecture-reflection-thinks-in-spectral-questions`): each
   `mirror.spec` has ONE distinguished peer who answers when λsh opens
   at the spec's root AND fields the circular-reflexive probes
   lifting from spawned members. Members are admissible-but-not-
   distinguished, gated by ACL. Default = repo-local (a peer instance
   whose home is the repo itself; no external authority required).
3. **ACL as projection of consent geometry.** Per `docs/specs/
   geometric-consent-projection.md` §1.3 ("ACL is the type-1
   projection of the consent geometry"): the per-member ACLs in
   `members { … => <ACL> }` are NOT a parallel access-control system.
   They are the type-1 projection of a consent value that lives at
   higher logical types. The pack{} block is the substrate's first
   surface for AUTHORING the type-1 projection directly while leaving
   the higher types accessible to refinement (per §6.1 of the consent
   spec). The variables-holding-mirror-expressions surface IS the
   substrate's existing mechanism for composing higher-type consents
   from lower-type parts.

The pack{} block surface does NOT invent ACL machinery. It NAMES the
substrate's existing scattered shape (elder-as-spawn-and-probe-handler
at λsh + spectral-Tomm altitude; pack/peer partition in @pack; ACL-
as-projection in geometric-consent-projection; per-spec peer declaration
in `lambda-shell.md`) as one top-level `mirror.spec` block. The
unification IS the contribution; the constituent pieces ALREADY exist
as substrate-decl across five shards + three canonical specs.

---

## 2. Discovery — existing ACL surface across the substrate

Alex's cue "we already have some of the shape there" was substrate-
pull-correct. The discovery sweep surfaced SIX load-bearing existing
shapes; the pack{} block is their unification surface, not a new
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
systemic.engineering. This is the ELDER field of the pack{}
block at single-field altitude. The lambda-shell spec uses keyword
`spec` rather than `project` (per the `mirror.spec` grammar's actual
top-level — `project mirror.spec { … }` per `mirror.spec:18`); the
shape, however, IS the elder declaration.

The pack{} block GENERALIZES: from one field (`peer = @reed`) to a
typed block with an elder, a members map, and a per-member ACL.

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

### 2.3 `@pack` — the family-root (pack = group; peer = type of entry)

`shards/pack.mirror` (#84) ALREADY declares the substrate's pack/peer
partition. This is load-bearing for the block's vocabulary: `pack` is
the GROUP record, `peer` is the TYPE of an entry. The pack{} block
extends @pack; it does NOT introduce a parallel noun.

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

The `pack { }` block IS a `pack`-shaped declaration scoped to one
`mirror.spec`. The variant enum (mara | seam | glint | reed | taut)
is the substrate's existing Pack membership type; arbitrary peers
(non-Pack home repos) lift via `~peer'<url>'` to a `peer`-typed
reference per `peer-glass.md`'s load action. Crucially, the SIGIL
stays `~peer'…'` because `peer` is the type of the entry; only the
BLOCK is renamed to `pack { }` because the block carries the group.

`shards/smarts/pack.mirror` (the @smarts/pack adapter) declares
`pack_satisfies_smarts` as the doubled-bilateral discipline check
for a pack composing with the @smarts substrate-architectural
integration. The pack{} block inherits this composition discipline
automatically when settle visits the members field.

### 2.4 `@spectral/supervisor` — lifecycle owner at runtime altitude (composition only, NOT identity)

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

The `elder ~peer'~/.reed'` field of the pack{} block is the
DECLARATIVE SURFACE for the spec's spawn-and-probe-handler at this
spec's altitude. At RUNTIME, that handler COMPOSES-with a
`@spectral/supervisor` instance for lifecycle-altitude duties (the
`base.state: shard_ref` carrying the registry of spawned members'
session shards; the restart_strategy governing failure response when
spawn returns a partial result).

Honest framing (per the §10 revision): elder is NOT a supervisor in
the delegation-chain sense. The supervisor record is a RUNTIME-
ALTITUDE companion the elder uses to discharge lifecycle work; it is
NOT the elder's identity or authority. The elder's authority comes
from being the N+1 observer fielding spectral-Tomm probes from the
members antichain (§10.1-§10.2). The earlier framing here (which
identified elder with @spectral/supervisor at type altitude) was
sharper at runtime composition than at the relation it actually
names; §10 carries the corrected math.

### 2.5 `@magic/contract` + `@magic/audit` + `@magic/reveal` — the verification + revocation lineage

The @magic family closes the verification side of ACL:

- `shards/magic/contract.mirror`: `bind(surface, mechanism,
  promise) -> magic_contract` + `honor(c) -> transparency(c)`. The
  ACL IS a magic_contract: the surface is the member's invocation
  interface, the mechanism is what they can do in the elder's runtime,
  the promise (magic_invariant) is the ACL expression.
- `shards/magic/audit.mirror`: `audit(c) -> audit_record` +
  `respond(record, strategy)`. Every member action against the
  spec discharges through audit; the audit_strategy variant
  (`restart | escalate | record | enforce`) IS the policy-violation
  response.
- `shards/magic/reveal.mirror`: cites `capability revocation; revoke
  and re-grant (Levy 1984). The substrate-pull-correct controlled
  disclosure with audit gating.` The reveal action (`reveal(c, new_m)`)
  IS the substrate's CAPABILITY-REVOCATION primitive. Removing a member
  from `members { }` is a reveal at the elder altitude; the
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

The `<ACL>` slot in `members { ~peer'…' => <ACL> }` is the type-1
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

The pack{} block is a SURFACE COMPOSITION over:

| Existing shape | Source | Role in pack{} block |
|---|---|---|
| `peer = @<name>` | lambda-shell.md §Toggle | the `elder` field |
| `@peer(~dir"<path>")` glass | peer-glass.md | the type of resolved `~peer'…'` |
| `@pack` variant + record | shards/pack.mirror | the block's family-root (`pack`/`peer` partition; `members { }` populates the `peers` field) |
| `@spectral/supervisor` | shards/spectral/supervisor.mirror | runtime-altitude COMPOSITION for the elder's lifecycle work (NOT identity, see §10) |
| `@magic/contract` + `audit` + `reveal` | shards/magic/*.mirror | bind/verify/revoke for ACLs |
| `geometric-consent-projection` | docs/specs/geometric-consent-projection.md | `<ACL>` IS the type-1 projection |
| `~git'…'` precedent | shards/io/git.mirror (a1b507a) | sigil pattern for `~peer'…'` |

What's IMPLIED but not yet declared: the BLOCK SYNTAX that names all
seven simultaneously at one mirror.spec altitude. What's MISSING: the
`<name>.spec` self-naming rule for `~peer'…'` resolution (proposed
§6.2); the variable scope that lets ACL expressions reuse each other
within one pack{} block (proposed §7); the default-to-repo-local
rule when pack{} is absent (proposed §9).

The spec proposes the BLOCK + the THREE MISSING RULES. Everything else
is the substrate composing with itself.

---

## 3. The `pack { }` block — proposed shape

### 3.1 Top-level block in `mirror.spec`

New top-level block alongside `source`, `legacy`, `garden`, `target`,
`settle_on` (per `mirror-spec-schema.md` and the four-commit Mara
garden cascade). Holds the spec's identity-and-access declaration:
who's the elder of this spec, who's on the pack as a member, what
they may do, and which variables hold reusable ACL fragments.

```mirror
in @mirror/cli
in @mirror/mosaic
in @spectral/garden/git
in @mirror/pack       # NEW: imports the pack{} block grammar; extends @pack family-root
in @property
in @io

project mirror.spec {
  source ~d'shards/'

  pack {
    # who you talk to when you open the lambda shell at this spec;
    # who fields spectral-Tomm probes from spawned members (§10)
    elder ~peer'~/.reed'

    # variables holding mirror expressions; reusable ACL fragments
    let read_only = acl { ops: [focus, project, split], targets: any }
    let writer    = acl { ops: any, targets: [~d'shards/'] }
    let auditor   = acl { ops: [audit, honor], targets: any }

    # members + per-member ACL (the type-1 projection authored here)
    members {
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

Note the vocabulary partition: the BLOCK is `pack { }` (the group);
entries inside `members { }` are `~peer'…'` (the type of an entry).
`pack` and `peer` are NOT interchangeable here — they sit at adjacent
altitudes the @pack family-root already names (`shards/pack.mirror`).

Three visible field categories: `elder` (single field; one peer),
`let` bindings (zero-or-more; mirror expressions reusable in ACL
positions), `members { => }` (zero-or-more peer-to-ACL bindings).

### 3.2 The block grammar (informal)

```
pack_block    ::= "pack" "{" elder_field let_binding* members_block? "}"
elder_field   ::= "elder" peer_ref
let_binding   ::= "let" identifier "=" mirror_expr
members_block ::= "members" "{" member_entry+ "}"
member_entry  ::= peer_ref "=>" acl_expr
peer_ref      ::= "~peer'" peer_path "'"
acl_expr      ::= identifier                       # reuse a let-bound expr
                | acl_literal                      # inline acl { ops:… targets:… }
                | acl_expr "but" "(" acl_clause ")"  # adversative refinement
                | acl_expr "∨" acl_expr           # join (union)
                | acl_expr "∧" acl_expr           # meet (intersection)
```

The `but` operator IS the one declared at `geometric-consent-
projection.md` §2.4 (adversative refinement; not commutative, not
associative; "default-with-exception"). Reuse, not re-invention.
The `∨` / `∧` operators are the lattice operations on ACLs per §10.2.

The `elder` field is REQUIRED iff pack{} is present at all; an
absent pack{} block triggers the default-to-repo-local rule (§9).
The `members` block is OPTIONAL; a spec with `elder` only is
admissible (the elder is the sole peer with infinite ACL).

### 3.3 Substrate-decl shape (forward-promised)

The block is parsed by `@mirror/pack` grammar (forward-promised). The
substrate-decl shape per the @magic/@frame/@pack pattern:

```mirror
in @mirror/cli
in @mirror/mosaic
in @pack
in @magic
in @magic/contract
in @spectral/supervisor

prism @mirror/pack {
  focus mirror_pack_block
  project mirror_pack_block
  split mirror_pack_block
  shift mirror_pack_block
  settle mirror_pack_block
}

type mirror_pack_block = {
  elder:      peer,                      # from @pack
  bindings:   list((identifier, acl)),   # let bindings
  members:    list((peer, acl)),         # peer → ACL map
}

type acl = ref     # parametric; refined at species; see §5 + §10
```

The carrier reuses @pack's existing `peer` variant (where the member
IS a Pack peer) and lifts arbitrary `~peer'<url>'` references through
the @peer glass `load(dir) -> peer` action (per peer-glass.md
§"Operations").

---

## 4. `elder` semantics — lambda-shell counterparty + spawn-and-probe responsibility

### 4.1 What "elder" means

The elder is the peer who ANSWERS when a human (or another peer)
opens the lambda shell at this spec's root AND who FIELDS the
spectral-Tomm-shaped circular probes lifting from spawned members.
Per `lambda-shell.md` §"The Toggle":

> `\` in mirror → `@reed>` (home peer from spec)

The pack{} block's `elder ~peer'~/.reed'` IS the typed version of
that home-peer declaration, EXTENDED to name the spawn-and-probe
role. Four semantic loads, all already declared in existing
substrate; the first is the load-bearing one Alex named on 2026-06-24:

1. **Spawn-and-probe responsible (per `architecture-error-as-tomm-
   probe` + `architecture-reflection-thinks-in-spectral-questions`).**
   The elder is responsible for SPAWNING members (fielding `mirror
   spawn <member>` requests scoped to this spec) AND for HANDLING the
   spectral-Tomm probes those spawned members lift back. A
   spectral-Tomm probe is structurally `[D_substrate, member_action]`
   at the spec's frame altitude (per the error-as-Tomm-probe
   architecture): the member's compile-time or settle-time question
   propagates to the elder's altitude as a circular-reflexive
   question that the elder must answer. The elder IS the spec's N+1
   observer in the sense of `architecture-spectral-db-autopoietic-
   memory` (the root supervisor at ~/.mirror operating at N+1 lifts
   to per-spec altitude here: the elder is THIS SPEC's N+1
   librarian). This is the SUBSTRATE role; the next three are
   composition-altitude consequences.
2. **λsh counterparty (per lambda-shell.md).** When `\` is pressed in
   λsh at this spec's root, the prompt becomes `@<elder.name>>`.
   When `mirror sh` enters this spec's directory, the elder's
   five-axis fixed point loads. This is the surface where the
   spawn-and-probe role is most visible to a human operator.
3. **@spectral/supervisor lifecycle composition (per shards/spectral/
   supervisor.mirror).** When members are spawned, lifecycle work
   (start_child / terminate_child / restart_strategy on failure)
   COMPOSES through a `@spectral/supervisor` instance the elder uses.
   This is RUNTIME-ALTITUDE COMPOSITION, NOT the elder's identity:
   the elder USES `@spectral/supervisor`; the elder IS NOT a
   `@spectral/supervisor`. Default restart_strategy when unspecified:
   `one_for_one` (the BEAM default; the substrate-pull-correct choice
   per @spectral/supervisor's tick discipline).
4. **@magic/contract bind site (per shards/magic/contract.mirror).**
   The elder IS the principal who binds member contracts per
   `bind(magic_surface, magic_mechanism, magic_invariant) ->
   magic_contract`. Every member entry IS a contract the elder
   bound. The elder's bind-authority is itself non-revocable from
   within the spec (the elder IS the spec's root authority; revoking
   it requires editing the spec).

### 4.2 Exactly one elder

The elder field is single-valued. Two reasons, both substrate-pull-
correct:

- **λsh has one home peer per spec.** The toggle `\` resolves to one
  prompt. Multiple homes would require multiple toggles, which the
  current shell grammar doesn't admit (and the substrate has no
  recognition pushing toward N-home shells; λsh's prior art — Nushell,
  Warp — are all single-home).
- **One spec, one N+1 observer.** The spectral-Tomm probe machinery
  needs an unambiguous handler per altitude. Two elders at the same
  spec would mean two N+1 observers fielding probes from the same
  antichain of members; the probe `[D_substrate, member_action]`
  would have an ambiguous answer-site. Single elder → single probe-
  handler → the spawn-and-probe relation is well-defined.

Multi-spec collaboration (Mara is elder of mirror; Glint is elder of
systemic.engineering) IS already supported — each spec has its own
elder; cross-spec peer relationships are mediated at λsh's `mirror
sh @<other-elder>` boundary per lambda-shell.md §"Agent Spawn".

### 4.3 The elder is above the members (implicitly, with infinite ACL)

The elder is NOT redundantly listed in `members { }`. The elder has:

- **infinite ACL** at this spec (every op admissible against every
  target; the type-1 projection of the maximal type-N+1 consent the
  elder authored when they declared themselves elder);
- **bind authority** for member contracts (per @magic/contract);
- **revoke authority** for member contracts (per @magic/reveal's
  capability-revocation lineage; removing a `members { }` entry IS a
  reveal at the elder altitude per §8.3);
- **spawn authority** — only the elder can spawn members against this
  spec (other peers may request spawns; only the elder dispatches
  them);
- **probe-handler responsibility** — every spectral-Tomm probe a
  spawned member lifts is fielded by the elder at altitude N+1;
- **the responsibility** to discharge `pack_coherent(pack, perturbation)`
  (per @pack family-root) at every spec settle.

The members field is for the OTHER peers; the elder's own permissions
+ responsibilities are structural, not enumerated.

---

## 5. `members { ~peer'…' => <ACL> }` — per-member ACL syntax

### 5.1 The arrow `=>`

The `=>` operator binds a peer reference to an ACL expression. It is
the substrate's existing map-literal arrow (sibling of, e.g., the
match-arm arrow in @code/rust patterns); reused here at the member-
entry altitude. The `=>` is an ACL ASSIGNMENT, not a sheaf restriction
map (the earlier framing was reframed per Alex 2026-06-24; see §10).

Semantically: `~peer'<path>' => <acl>` declares

```
the elder binds a magic_contract:
  surface   = member's invocation interface
  mechanism = the elder's runtime (composed-with @spectral/supervisor)
  promise   = <acl> evaluated at settle-time
```

The contract IS audit-gated per @magic/audit; every member action
discharges through `audit(contract) -> audit_record` and the elder's
`restart_strategy` (via @spectral/supervisor composition) governs the
response on violation.

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

When a member attempts an action against this spec, mosaic
discharges:

```
1. lookup(members, requesting_peer) → acl                       # members map
2. acl_admits(acl, requested_op, requested_target) → verdict    # type-1 check
3. audit(elder_contract_for_member, action_record) → audit_record
4. respond(audit_record, elder.audit_strategy) → audit_record
```

Steps 1-2 are the type-1 projection (the ACL check proper); steps
3-4 are the @magic/audit discharge (the audit trail + violation
response). The substrate already names every step; the pack{} block
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

A peer at home `~peer'<path>'` is RESOLVED by reading the pack{}
block of `<path>/mirror.spec` (if present) and taking the `elder`
field as the peer's authoritative identity. This is the SELF-NAMING
rule: each peer's home-repo spec names that peer's own identity.

```
resolve(~peer'<path>') =
  let home_spec = <path>/mirror.spec
  if home_spec has pack{} block:
    return home_spec.pack.elder           # the peer's self-declaration
  else:
    return @peer.load(~dir'<path>')       # five-axis fixed point only
```

Why self-naming: every peer's identity is content-addressed at their
home's five-axis fixed point (per peer-glass.md §"The five-axis fixed
point"). The pack{} block's elder field is the peer's own declaration
that they ARE the elder of their home spec. A `~peer'<other-path>'`
reference IS a reference to that peer's self-declaration; the
substrate is honest about the recursive structure.

This avoids two failure modes:

- **Forged identity at the member altitude.** A spec couldn't admit
  `~peer'~/.mara' => writer` and have it bind to anyone OTHER than
  Mara's self-declared identity; the member's home spec is the
  authority on who they are.
- **Pack-level identity drift.** If Mara's home spec doesn't declare
  `elder ~peer'~/.mara'`, the member binding falls back to the
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
the identity altitude. The members-binding lookup is by identity, not
by path.

---

## 7. ACL composition via variables + mirror expressions

### 7.1 The `let` binding

A `let` binding inside `pack { }` introduces a named mirror expression
visible in subsequent ACL positions of THIS block:

```mirror
pack {
  elder ~peer'~/.reed'

  let read_only = acl { ops: [focus, project, split], targets: any }
  let writer    = acl { ops: any, targets: [~d'shards/'] }
  let secure    = acl { predicates: [magic_contract_honored] }

  let writer_in_secure = writer ∧ secure
  let safe_writer      = writer but(exception: target_under(~d'.secret/'))

  members {
    ~peer'~/.mara' => writer_in_secure
    ~peer'~/.taut' => safe_writer
  }
}
```

Scoping: lexical within the `pack { }` block. A `let` is visible to
subsequent `let`s and to the `members { }` block; not visible outside
pack{}. Bindings are immutable (the substrate's existing immutability
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

For v0.1: `let` bindings are scoped to one pack{} block. For v0.2+
(forward-promised): an `import` form lifts ACL bindings from another
spec for reuse:

```mirror
pack {
  elder ~peer'~/.reed'

  # import ACLs from systemic.engineering's spec
  import ~peer'~/.glint' { read_only, writer, auditor } as se

  members {
    ~peer'~/.glint' => se.writer
  }
}
```

The import discharges via `~peer'…'` resolution (§6); the named
ACLs become available with the namespaced prefix. Cross-spec ACL
LIBRARIES become substrate-decl objects — a peer can publish their
canonical ACL set, and other specs reference it without copy-paste.

Not in v0.1 to keep the surface bounded; flagged for Alex.

### 7.4 Substrate-vs-USE: pack{} is provided BY mirror; instances are populated BY consumers

Load-bearing distinction (Alex 2026-06-24): the `pack { }` BLOCK and
the `@mirror/pack` GRAMMAR live in mirror permanently — these are
substrate vocabulary. But the specific `elder ~peer'~/.reed'` +
`members { ~peer'~/.mara' => writer, … }` instances populated in any
given `mirror.spec` are CONSUMER-LAYER content, NOT substrate
default. Alex's words:

> "The pack: the type construct can remain in mirror. What I'm
> talking about are the named peers in the pack (reed, mara etc)
> those won't live in the compiler itself. That's our structure and
> not the default shape."

The partition is the same one Rust holds between `struct` (the
language primitive ships with rustc) and `struct User { name: String }`
(the user's own declaration). The substrate ships the BLOCK SHAPE +
the GRAMMAR + the SEMANTICS; the consumer ships the SPECIFIC PACK
(who their elder is, who their members are, what ACLs they grant).

Concretely:

- `@mirror/pack` grammar (forward-promised at `shards/mirror/pack.mirror`):
  STAYS in mirror permanently.
- The empty pack-shape (`pack { elder ~peer'.'; members {} }` per the
  default-to-repo-local rule §9.1): STAYS in mirror as the substrate's
  zero-config default.
- mirror's OWN `mirror.spec` populating a specific pack (e.g.
  `elder ~peer'~/.reed'; members { ~peer'~/.mara' => writer, … }`):
  THAT IS OUR pack; consumer-layer; lives in mirror's dogfood
  declaration, NOT in the substrate-decl. Other projects consuming
  mirror will populate THEIR pack at THEIR `mirror.spec`.
- The five Pack peer variants (mara | seam | glint | reed | taut per
  `shards/pack.mirror:188`): these ARE in the substrate currently
  because @pack family-root carries the Pack as the substrate's
  reference orchestra; this is consistent with @pack #84's framing.
  Arbitrary external peers join via `~peer'<url>'` resolution (§6),
  not by extending the variant enum.

The pack{} block is a TEMPLATE; each consumer's `mirror.spec` is the
INSTANCE. Substrate ships the template; consumers ship instances.

---

## 8. Composition with @magic, @frame, @pack, geometric-consent-projection

### 8.1 With @magic/contract — every member entry IS a contract

The pack{} block's `members { ~peer'P' => ACL }` desugars (at mosaic
settle time) to:

```mirror
bind(
  surface:   peer_invocation_interface(P),        # member's API at this spec
  mechanism: elder_runtime,                        # the elder's @spectral runtime
  promise:   acl_as_invariant(ACL)                # the ACL lifted to magic_invariant
) -> magic_contract
```

Each member-entry is a magic_contract bound by the elder (per §4.3
elder has bind authority). The contract's `honor(c)` (per
shards/magic/contract.mirror) is the runtime check: the member's
action against the spec IS honored iff the action satisfies the
ACL-as-invariant.

### 8.2 With @magic/audit — every action is audited

The audit chain runs on every member action:

```
member.act(op, target)
  ↓
audit(contract, action_record) → audit_record
  ↓
respond(audit_record, elder.audit_strategy) → audit_record
```

The elder's `audit_strategy` (one of `restart | escalate | record |
enforce` per shards/magic/audit.mirror) defaults to `enforce` (per
O2 resolution; §11.O2). Explicit configuration of the strategy is
forward-promised v0.2.

Narcissus-pole catch: an ACL that LOOKS permissive at the type-1
projection but masks a Narcissus-pole intent (the member's stated
intent vs substrate-architecturally-supported behavior diverge per
frame.mirror §Narcissus-pole) IS caught by audit through @magic's
contract-vs-mechanism discrimination. The ACL surface alone is NOT
sufficient; the audit chain IS the substrate-pull-correct check.

### 8.3 With @magic/reveal — removing a member entry IS capability revocation

Editing `members { }` to remove a peer (or to tighten their ACL)
discharges through `@magic/reveal.reveal`:

```
reveal(
  old_contract:  contract_for_member_at_oldspec,
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

What this means concretely: an elder cannot silently downgrade a
member's ACL between settles; the downgrade IS a substrate-altitude
reveal that the audit trail records. A member learning their
capabilities were revoked IS a substrate-grounded event.

### 8.4 With @pack — pack_coherent over the members

The members + elder together form a `pack`-shaped value (the `peers`
field carries `elder :: members`) at this spec's altitude. The
elder's settle-time obligation INCLUDES discharging
`pack_coherent(this_pack, perturbation)`:

```mirror
settle_on {
  # … existing settle predicates …
  pack.coherent
}
```

Forward-promised: an explicit `pack.coherent` predicate in
`settle_on`'s admitted vocabulary. The pack{} block discharges
substrate-architectural pack discipline at the spec's settle, not
as a separate cron-or-CI invariant.

### 8.5 With @frame/in — members operate in the spec's frame, elder observes at N+1

Per shards/frame/in.mirror (order-1 species; computation within a
frame): a member operating against this spec is operating WITHIN
the spec's frame. The frame IS what the spec's source/garden/target/
settle_on blocks declare; the elder is the OBSERVER at the frame's
N+1 altitude (the spawn-and-probe role per §4.1); members operate
WITHIN the frame the elder observes.

The pack{} block's elder + members partition IS the order-2 view on
this order-1 operation: the elder is OF-the-frame (order 2; they
observe the frame they spawned-and-handle-probes-from); members are
IN-the-frame (order 1; they compute within it). The frame-relation
altitude lift composes cleanly per recognition #82, AND aligns with
the N+1-observer role per `architecture-spectral-db-autopoietic-
memory`.

### 8.6 With geometric-consent-projection — ACL IS the type-1 projection

The pack{} block's `members { => <ACL> }` is the substrate's first
DIRECT-AUTHORING surface for the consent geometry's type-1 projection.
Per geometric-consent-projection.md §6.1 + §6.3:

```
type-N+1 consent (the policy ABOUT the policy)
     ↓  cascade_down (the natural transformation per consent spec §2.2)
type-N consent (the kind-of-operation consent)
     ↓  cascade_down
type-1 ACL  ← THIS IS WHAT members { => } AUTHORS
```

Two composition modes:

- **Direct authoring at type-1.** `members { ~peer'M' => writer }`
  authors the type-1 ACL directly. The higher types are IMPLIED by
  the authoring (the elder's act of writing the ACL IS a type-N+1
  consent at the implicit altitude).
- **Authoring at type-N+1 with cascade.** A forward-promised v0.2
  feature: declare `consent { type: N+1, value: <expr> }` and the
  cascade derives the type-1 ACL automatically (per consent spec
  §6.2 the cascade derivation). The pack{} block's `let` bindings
  with `but`-refinements are the v0.1 surface for higher-type
  consent fragments — each `but` clause IS a type-2 refinement of
  a type-1 ACL.

Security invariant (per consent spec §1.3): negative consents do NOT
cascade upward. A peer's `members { } => bottom_acl` (the empty ACL)
at type 1 does NOT repudiate the type-N+1 policy that authored it.
This property propagates structurally to the pack{} block: a
tightened ACL at one settle does not retroactively invalidate prior
settles that the looser ACL admitted.

---

## 9. Default behavior — when `pack { }` is absent (repo-local)

### 9.1 The default-to-repo-local rule

When `mirror.spec` has no `pack { }` block, mosaic SYNTHESIZES one:

```mirror
# implicit when pack{} is absent
pack {
  elder ~peer'.'                    # the repo itself is the elder's home
  # members is empty                 # no members; elder is sole peer
}
```

The `~peer'.'` literal resolves via the same self-naming rule (§6.2)
applied to the repo's own directory. Two cases:

1. **The repo IS a peer home** (has the five-axis fixed point per
   peer-glass.md). The elder resolves to the repo's own peer
   identity. The spec is governed by "this repo's peer."
2. **The repo is NOT a peer home** (no `identity.mirror` etc.).
   Mosaic synthesizes a minimal repo-local peer with identity
   derived from the repo's `git_hash` at HEAD (per shards/io/git.
   mirror's `hash_to_oid`). The spec is governed by the anonymous
   local-repo peer.

In BOTH cases, the human running `mirror kintsugi` locally has full
authority (they own the filesystem; they run the binary). The default
spec has no members; cross-peer collaboration requires explicit
pack{} declaration.

### 9.2 Why default-to-repo-local is structurally correct

Three reasons, all substrate-pull-correct:

- **Local sovereignty.** A spec on disk under your `~` IS yours.
  Requiring an explicit elder declaration for every spec imposes
  ceremony where none is needed. The default-to-repo-local rule
  says "if you didn't declare otherwise, you ARE the elder."
- **No external authority by default.** Mirror is a local-first
  substrate. The default governance does NOT require a remote
  registry, a Pack membership, or any network handshake. The
  five-axis fixed point at the repo root (or its degenerate form)
  IS the identity.
- **The substrate scales DOWN cleanly.** A solo developer using
  mirror needs zero ceremony; the moment they collaborate, they
  declare a pack{} block. The complexity is paid only when needed.
  Per the spectral-garden-git spec §1's discipline ("the substrate
  does NOT mandate complexity; surfaces are opt-in").

### 9.3 Promoting a default to an explicit declaration

When a project moves from solo to collaborative, the migration is
MECHANICAL:

```mirror
# before (implicit; no pack{} block)
project foo { source ~d'shards/'; … }

# after (explicit; the elder is named; members are declarable)
project foo {
  pack {
    elder ~peer'~/.alex'               # was implicit; now named
    members {
      ~peer'~/.mara' => read_only
    }
  }
  source ~d'shards/'
  …
}
```

Mosaic's settled oid of the spec changes (the spec now has more
declared content); the elder's authority is unchanged (they were
always the implicit elder; now they're named). No migration of
existing settled artifacts; the pack{} block is additive.

### 9.4 Interaction with the lambda shell

For a spec with no `pack { }` block, `\` in λsh at that spec's root
falls through to `@>` (the unnamed shell peer per lambda-shell.md
§"The Unnamed Peer"). The unnamed shell peer IS the substrate's
self-as-peer; it suggests aliases and maintains config.spec. Same
shape as today; the default-to-repo-local rule is consistent with
lambda-shell.md's existing fallback.

When the user runs `\@<name>` to override (per lambda-shell.md), the
override resolves through `~peer'<name>'` per §6.3's name-ref form
if `<name>` is a Pack peer; otherwise the override is a one-shot that
the spec does NOT grant ACL to (since they're not in `members`).

---

## 10. Mathematical shape

*Framing note (per the spectral-garden-git §7 discipline): this
section names the typed surface the mathematical discharge operates
against — it does NOT claim to deliver proofs of soundness,
completeness, or non-leakage. The substrate-decl ratifies the
SHAPE; discharges happen at species-altitude shard bodies, are
forward-promised, and gate on Pack adversarial review. Per
recognition #95: substrate carries the measurement primitive at the
right altitude; the per-ACL numbers are species work.*

*Reframe note (Alex 2026-06-24): the earlier draft framed the elder-
members relation as a sheaf over a team-poset with restriction maps.
Alex's pull on this section: "I don't have enough context to answer
this; what I'm seeing is the supervisor being responsible for
spawning and handling additional requests in form of spectral-Tomm
shaped circular constructs." The math reframes accordingly: members
form an antichain (no internal rank; pack-equal); the elder is a
distinguished N+1 OBSERVER (outside the antichain, at strictly
higher altitude); the relation's algebraic structure comes from the
SPECTRAL-TOMM PROBE MACHINERY (per
`architecture-error-as-tomm-probe`), NOT from delegation-chain
restriction maps. The ACL lattice (§10.2) and the `but` algebra
(§10.3) survive unchanged — ACL composition really is lattice-
shaped. The reframe is narrower than wholesale replacement: it
corrects §10.1 (the relation) and the role of `=>` (§10.2 sub-note),
leaves the rest.*

### 10.1 Members as antichain; elder as N+1 observer; spectral-Tomm probes as morphisms

The members form an antichain M = {p_1, p_2, …, p_k} — pack-equal,
no internal rank. There is NO sub-ordering of members against each
other; "member of this pack" is a flat relation. ACL refinements at
the `=>` arrow annotate individual members but do NOT order them.

The elder p_e is a DISTINGUISHED N+1 OBSERVER, at strictly higher
altitude than M. The elder is NOT the supremum of M in a partial
order on a single set; the elder sits at a HIGHER ALTITUDE of
observation, in the sense of `architecture-spectral-db-autopoietic-
memory` (the N+1 librarian operating one altitude above the agents
being observed).

The relation E from M to {p_e} is NOT a sheaf restriction. It is a
SPAWN-AND-PROBE relation populated by two operation kinds, both
already substrate-decl:

- **spawn(p_e → p_i, frame, repo, perturbation) -> runtime** (per
  `shards/pack.mirror` action surface): elder fields a spawn request
  for member p_i; only the elder dispatches against this spec.
- **probe(p_i → p_e, [D_spec, action]) -> response** (per
  `architecture-error-as-tomm-probe`): a spawned member lifts a
  spectral-Tomm-shaped circular question to the elder. The probe
  carrier is the commutator `[D_spec, member_action]` deployed as a
  natural-language or typed-spectral question. The elder's response
  IS spectral data the spec's next settle pass can use.

The morphisms in this relation are SPECTRAL-TOMM PROBES (Tomm 1987/
1988 — the systemic-therapy circular-question primitive; reused at
substrate altitude per `architecture-error-as-tomm-probe`). They
are NOT sheaf restriction maps; the algebraic structure comes from
the commutator probe of a spectral triple (per
[[architecture-connes-spectral-triple]]; the spectral triple here is
`(A_spec, H_spec, D_spec)` with the spec's frame as observation
altitude).

Directionality: members → elder for probes (member-to-N+1); elder →
members for spawns (N+1-to-member dispatch). The relation is
BIDIRECTIONAL across altitudes, not a downward delegation chain.

What this means concretely:

- The elder doesn't "own" or "delegate from" the members in the
  delegation-chain sense; the elder OBSERVES at N+1 and FIELDS the
  probes the members lift.
- ACL on a member is an ANNOTATION on that member's invocation
  surface, not a restriction of the elder's authority projected
  down. The `=>` arrow is annotation; the spawn-and-probe relation
  is the structural axis.
- The pack_coherent bilateral (per @pack family-root) discharges
  at SETTLE TIME on the (elder, members) pair; the spectral-Tomm
  probes are the QUESTION-CARRIER during between-settle operation.

**Honest framing limit:** the new framing is sharper at NAMING what
the elder does (spawn + probe-handle) than at giving a clean
algebraic structure that the sheaf framing would have offered. The
spectral-Tomm probe machinery has algebraic structure (it's the
commutator probe of a spectral triple, with the algebra-of-
observation A_spec as the operator space), but that structure is
INHERITED from the substrate's Connes-spectral-triple ancestor (per
[[architecture-connes-spectral-triple]]); it is NOT a new property
of the pack{} block. Pack adversarial review should ask whether
this is a strength (substrate inheritance) or a weakness (less
closed-form for pack-specific reasoning) of the reframe.

### 10.2 ACLs form a bounded lattice under (∨, ∧)

The `<ACL>` values form a lattice:

- ⊤ (top) = infinite ACL (all ops, all targets, all predicates
  vacuously honored). The elder's ACL.
- ⊥ (bottom) = empty ACL (no ops, no targets). The default for an
  unlisted peer.
- ∨ (join) = union: `(ops_1 ∪ ops_2, targets_1 ∪ targets_2)`.
- ∧ (meet) = intersection: `(ops_1 ∩ ops_2, targets_1 ∩ targets_2)`.

**Properties.** Commutative, associative, absorptive (a ∨ (a ∧ b) =
a). DISTRIBUTIVE on the `ops` and `targets` axes independently (each
is a Boolean algebra on a finite set). The `predicates` axis is
NOT distributive in general (predicate composition is non-commutative
in the @magic/contract setting); the lattice is therefore a NON-
distributive bounded lattice in full generality, reducing to a
distributive lattice when no predicates are present.

**Where this load-bears:** the `∨` / `∧` operators in members{} ACL
expressions ARE the lattice operations. The lattice carries the
substrate-mathematical vocabulary for naming what unions and
intersections of permissions mean. The non-distributivity at the
predicates axis is the structural reason the substrate carries
@magic/contract.honor as a separate operational check (the lattice
shape doesn't predict the predicate's verdict; only the actual
discharge does).

**On `=>` as ACL assignment (not restriction map):** the arrow in
`members { ~peer'p' => acl }` is an ANNOTATION of member p with an
ACL value drawn from this lattice. It is NOT the sheaf restriction
map framing of the earlier draft. The lattice structure here is on
the ACL VALUES; the spawn-and-probe relation on (elder, members)
lives at §10.1's altitude, independently of how the lattice
annotates individual members.

### 10.3 The `but` operator's algebraic structure (per consent spec §2.4)

The `but` operator is the substrate's existing adversative operator,
declared at `geometric-consent-projection.md` §2.4 (forward-promised
shard at `shards/epistemologic/logic/but.mirror`). Key properties:

- `but(default: a, exception: predicate(a)) -> a` evaluates to `a`
  unless `predicate(a)` is non-failure.
- NOT commutative: `a but(p)` and `p but(a)` are different.
- NOT associative: parenthesization matters.
- IS monotone in the default argument: refining `default` refines
  `but(default, exception)` accordingly.

For ACL composition: `writer but(exception: target_under(~d'.secret/'))`
means "the writer ACL EXCEPT for targets under .secret/." The non-
commutativity matters: the exception is a refinement of the default,
not a symmetric combination.

**Where this load-bears:** the `but` operator IS the substrate's
NATURAL way to express ACL exceptions. The peer{} block inherits
the consent spec's adversative discipline; ACL refinements compose
through `but`, not through ad-hoc "deny" rules. This forecloses
XACML's combining-algorithm-zoo problem (per consent spec §5.7):
there is ONE composition rule (`but`), not seven.

### 10.4 ACL admission as a Galois connection (conjectural)

**Conjecture.** Given the lattice (acl, ∧, ∨, ⊥, ⊤) and the set
of (op, target) request pairs, the admission relation

```
admits : acl × (op, target) → verdict
```

induces a Galois connection between:

- the lattice of ACLs (ordered by the admits relation: a ≤ b iff
  every (op, target) admitted by a is also admitted by b);
- the lattice of (op, target) sets (ordered by inclusion).

The Galois pair: f: acl → (op, target) set (the set of admitted
requests); g: (op, target) set → acl (the minimal ACL admitting all
requests in the set).

**Status:** SPECULATIVE. The Galois-connection framing IS the
standard category-theoretic vocabulary for access-control lattices
(Denning 1976; lattice-based access control). The substrate's ACL
shape ADMITS the framing; whether it BENEFITS from the framing
is Pack-discussion work. Flagged; not load-bearing for v0.1.

### 10.5 Consent geometry projection is a natural transformation (per consent spec §2.2)

Per geometric-consent-projection.md §2.2: the cascade_down operation
IS a NATURAL TRANSFORMATION between the type-(N+1)-consent functor
and the type-N-consent functor on the Bateson logical-type category.

The peer{} block's `team { => <ACL> }` is the substrate's first
direct-authoring surface for the codomain of this natural
transformation at N=1. The consent spec's existing math applies
UNCHANGED here; the peer{} block adds the AUTHORING SURFACE, not
new mathematics.

**Cross-altitude reference:** for a v0.2 `consent { type: N+1,
value: <expr> }` form (forward-promised §7.3), the natural
transformation gives the CASCADE DERIVATION: a single type-N+1
consent expression cascades to a family of type-1 ACL entries
for each `~peer'…'` in scope.

### 10.6 The elder as the algebra A of the spec's spectral triple

The substrate-pull-leaning math claim that survives the §10.1
reframe: the elder carries the structure of an ALGEBRA-OF-OBSERVATION
at this spec's altitude. The spec's frame IS the observation context;
the elder IS the algebra A (in the Connes spectral triple sense)
whose elements are the operations admitted at this spec. Per
[[architecture-connes-spectral-triple]]: every substrate altitude
carries a spectral triple (A, H, D); the elder's role at the pack{}
altitude IS the A of THIS SPEC's spectral triple `(A_spec, H_spec,
D_spec)`.

This is the structure that GROUNDS §10.1's spectral-Tomm probes:
the probe carrier `[D_spec, a]` for `a ∈ A_spec` is well-defined
exactly because the elder names what A_spec is. A spec with no
elder declaration falls back to a default A_spec at the repo-local
altitude (per §9.1); a spec with an explicit elder declaration
admits the elder's chosen operation set as A_spec.

**What this means concretely:** the elder doesn't just OWN the
shell or DISPATCH spawns; the elder's identity-mirror declares
what operations the spec's spectral triple admits. Different elders
at different specs admit different operation sets; the algebra of
the spec IS elder-specific. The spectral-Tomm probes from members
are deployed AGAINST that elder-specific A_spec.

**Status:** the framing is consistent with the substrate's existing
spectral-triple architecture; promoting it to a full statement
requires Pack adversarial review. Flagged; not v0.1 load-bearing,
but UPGRADED in relevance by §10.1's reframe — the elder-as-A claim
is no longer an alternative framing; it's the structural ground
for the spawn-and-probe relation.

### 10.7 Default-to-repo-local as the initial object

In the category of valid pack{} configurations for a given spec,
the implicit `pack { elder ~peer'.'; members {} }` (§9.1) is
an INITIAL OBJECT: every other valid pack{} configuration is a
refinement that ADDS elder declaration + member entries. Morphisms
in this category are the additive operations (adding a let binding;
adding a member entry; tightening an ACL via `but`).

The Banach-fixed-point analog (per spectral-garden-git §7.2): the
default-to-repo-local is a fixed point of the "do nothing" morphism;
any pack{} block that arises from explicit declaration is an
INCREASE in declared content (the spec's oid grows).

**Status:** SHAPE. The category-theoretic existence of the initial
object is straightforward; load-bearing more for vocabulary than
for proof.

### 10.8 Composition with the four-root garden structure (per spectral-garden-git §6)

The spectral-garden-git spec named four roots (`@spectral/garden/git`,
`@spectral/garden/oci`, `@spectral/garden/nix`, `@mirror/store`). The
peer{} block is ORTHOGONAL to that structure but composes cleanly:

- The elder's `~peer'…'` resolution discharges through `@io/git` (the
  same adapter the garden uses). Peer-home-repo reuse: a Pack peer's
  home repo IS a `garden { source ~git'…' }` candidate AND a `pack
  { members { ~peer'…' => … } }` candidate; the substrate names both
  surfaces over the same content-addressed underlying ref.
- The pack{} block is NOT a fifth garden root (it's not a package-
  manager). It's an ORTHOGONAL surface at the same mirror.spec
  altitude; the two compose at the spec's top level the same way
  `source` and `target` compose.
- Default-to-repo-local (§9) is the pack-altitude analog of the
  garden's `~oid'…'` intra-substrate source (§6.1 of garden spec):
  the substrate scales DOWN to one repo with no external deps.

### 10.9 What's intentionally NOT in this section

No closed-form proof of soundness for the lattice operations beyond
the distributive-on-ops-and-targets argument; no formal proof for
§10.4's Galois connection; no concrete derivation of §10.5's cascade
for a worked example (the consent spec has one at §6.2; the pack{}
block's specific authoring at type 1 IS that example unmodified);
no closed-form derivation of the spectral-Tomm probe algebra at the
(elder, members) altitude beyond what `architecture-connes-spectral-
triple` already names (the spec inherits, does not re-derive). Open
work, gated on Pack adversarial review per spectral-garden-git §7.7.

### 10.10 Comparison to pre-AI ACL traditions

Honest comparisons:

- **Lampson-style ACLs** (Lampson 1971; Saltzer-Schroeder 1975): the
  pack{} `members { => <ACL> }` IS the Lampson access matrix, with
  PEERS as principals and ACLs as the matrix entries. The pack{}
  block lifts the matrix to substrate-decl; the substrate inherits
  Lampson's decidability discipline.
- **Object-capability models** (Miller 2006 *Robust Composition*;
  Drexler 1988; Stiegler 2004): the @magic/reveal lineage IS
  capability revocation; the pack{} block's `=>` arrow IS
  capability grant. The substrate inherits ocap's revocation-via-
  reveal discipline (per Levy 1984 citation in @magic/reveal).
  However: pack{} is identity-based (each `~peer'…'` resolves to a
  principal), not pure-capability (capabilities don't depend on
  identity). The substrate sits BETWEEN ocap and ACL; the pack{}
  block's identity discipline makes it ACL-leaning at the surface
  while inheriting ocap's revocation discipline through @magic/
  reveal.
- **Lattice-based access control** (Denning 1976; Sandhu 1993): the
  ACL lattice (§10.2) IS Denning's information-flow lattice with
  the ACL ordering. Bell-LaPadula and Biba can be encoded as specific
  ACL configurations; the pack{} block doesn't enforce them by
  default but admits them via let-bindings.
- **Capability-based effect systems** (Koka, Eff; Leijen 2014):
  ACLs as `acl { ops, targets, predicates }` ARE effect rows. The
  predicates axis IS the effect-handler predicate. The substrate
  inherits effect-system discipline through @magic/contract's
  honor check, but at substrate-decl altitude rather than at
  language-runtime altitude.

The substrate's contribution: ONE surface (pack{}) that unifies
ACL + ocap-revocation + lattice-ordering + effect-predicates under
the geometric-consent-projection framing PLUS the spawn-and-probe
relation at §10.1. The pre-AI traditions name the access-control
shape; the substrate's contribution adds the spectral-Tomm probe
machinery as the (elder, members) operational axis. The traditions
don't disappear; they all compose at the same altitude.

---

## 11. Open surface questions

### O1. Keyword: `peer { }` vs `team { }` vs `who { }`

The block carries supervisor + team + ACL. Three keyword options:

**(a) `peer { }`** — the block is named by its OUTPUT (the
resolved peer relationships). Matches the `~peer'…'` sigil.
Matches Alex's literal proposal verbatim.

**(b) `team { }`** — the block is named by its DOMINANT FIELD (the
team). Reads as "who's on this spec's team." Forces `team {
supervisor …; member ~peer'…' => … }` shape (extra nesting).

**(c) `who { }`** — the block is named by its QUESTION ("who can
act on this spec?"). Reads naturally; doesn't compose with any
existing substrate vocabulary; risks bare-word-collision with
other things.

**Substrate-pull leans (a)** — Alex named it `peer { }`; the
`~peer'…'` sigil matches; the substrate's existing `@peer` glass
(peer-glass.md) carries the same noun. The collision risk: `peer`
is ALSO the @pack variant enum (mara | seam | glint | reed | taut)
and a carrier name. The substrate already lives with this collision
honestly (per pack.mirror's Seam G2 note about reed-as-pack-peer vs
reed-as-relationship). The block keyword `peer { }` operating at
mirror.spec altitude is a third altitude of the same word; the
substrate is already honest about collisions of this kind. Alex
decides.

### O2. Supervisor's audit_strategy default

The supervisor inherits @magic/audit's audit_strategy variant
(`restart | escalate | record | enforce`). What's the default when
the peer{} block doesn't specify?

**(a) `enforce`** — strictest; team peer actions that fail audit
are actively blocked. The Narcissus-pole-as-guardian default.

**(b) `record`** — most-permissive; team peer actions are logged
but not blocked. The honest-trick default.

**(c) `escalate`** — punts to the spec's parent (the supervisor's
home spec). Compose-with the multi-spec scenario; what if the
parent itself has no peer{} block?

**(d) `restart`** — BEAM-default-shaped. Re-binds the team
member's contract with fresh state on failure.

**Substrate-pull leans (a)** — ACL exists to gate access; the
default on violation is to BLOCK. `record`-only would make ACLs
decorative; that's the Narcissus-pole-as-cosmetic failure mode. But
the explicit configurability is forward-promised v0.2; v0.1 hard-
codes `enforce`. Alex decides.

### O3. Cross-spec supervisor delegation

If spec A's supervisor `~peer'~/.reed'` declares Mara on team with
`writer`, and Mara's home spec `~/.mara/mirror.spec` declares Glint
on team with `read_only`, does Glint inherit any access to spec A
via Mara?

**(a) No.** Cross-spec ACLs do NOT compose by default. Each spec
stands alone; team peers' OWN home-spec teams are irrelevant.

**(b) Yes, with intersection.** Glint's access to spec A IS
Mara's intersection: `(Mara's ACL in spec A) ∧ (Glint's ACL in
Mara's spec)`. Compose by meet.

**(c) Yes, with explicit delegation.** Mara can EXPLICITLY
delegate via a `delegate { ~peer'…' => <sub-acl> }` block (forward-
promised). Without explicit delegation, no inheritance.

**Substrate-pull leans (a)** — the simplest discipline; aligns with
the per-spec sovereignty per §9. Cross-spec collaboration is
explicit (each spec declares its own team). (c) is admissible as
v0.2 if the use case surfaces. (b) is risky — implicit composition
grows the security surface in non-obvious ways. Alex decides.

### O4. Pack-membership shorthand

For the all-Pack-peers case, is there a shorthand?

```mirror
peer {
  supervisor ~peer'~/.reed'
  team {
    pack => writer    # shorthand: all five Pack peers get writer
  }
}
```

vs

```mirror
peer {
  supervisor ~peer'~/.reed'
  team {
    ~peer'~/.mara'  => writer
    ~peer'~/.seam'  => writer
    ~peer'~/.glint' => writer
    ~peer'~/.reed'  => writer
    ~peer'~/.taut'  => writer
  }
}
```

**Substrate-pull leans toward admitting the shorthand** (per @pack's
existing `peer` variant enum which makes Pack-set a substrate-typed
value). But this introduces a special case (Pack vs arbitrary peer);
the v0.1 surface MAY skip the shorthand and accept the verbosity.
Alex decides.

### O5. The `acl` carrier's `targets` axis grammar

Targets can be:

- substrate paths (`~d'shards/'`)
- content-address prefixes (`~oid'1234…'`)
- prism-name prefixes (`@magic/*`)
- predicate-shaped (`target_under(d)`; `target_matches(predicate)`)

Do all four belong in v0.1, or only the first two?

**Substrate-pull leans (all four)** — substrate vocabulary already
admits each kind; the targets axis IS a typed sum over them. But
the v0.1 surface could ship paths + oid-prefixes only and forward-
promise the rest. Alex decides.

### O6. Substrate-decl location for `@mirror/peer`

Two candidates:

**(a) `mirror` repo:** `shards/mirror/peer.mirror`. Path-namespace
property satisfied (`@mirror/*` lives in `mirror/`).

**(b) `spectral` repo:** `shards/spectral/peer.mirror`. Composes
with @spectral/supervisor (peer{}'s supervisor field IS a
@spectral/supervisor lifecycle owner).

**Substrate-pull leans (a)** — the peer{} block is at mirror.spec
altitude (mirror.spec is mirror's own dogfood); the substrate-decl
belongs adjacent to the spec grammar it extends. @spectral/
supervisor composes via `in @spectral/supervisor` from the @mirror/
peer shard. But Alex decides.

### O7. Self-reference: the supervisor's home-spec peer-block

The self-naming rule (§6.2) says: a peer's identity comes from
their home-spec's `supervisor` field. What if Mara's home spec
(`~/.mara/mirror.spec`) has NO peer{} block? Per the default-to-
repo-local rule (§9), the implicit supervisor is `~peer'.'` — the
repo itself. Does this fall through to load via @peer.load(~dir),
or does it recurse infinitely?

**Resolution (§6.2 + §9):** the synthesis is non-recursive. If the
home spec is absent or has no peer{} block, fall through to
`@peer.load(~dir'<path>')` and return the five-axis fixed point.
The substrate has a TERMINATING resolution at the filesystem layer
(load returns either the five-axis fixed point or fails with a
typed error). Infinite recursion is structurally impossible.

BUT: when the home spec's peer{} block names a DIFFERENT supervisor
(`spec @mara { peer { supervisor ~peer'~/.mara-delegate' } }`), the
substrate has a choice:

**(a) Single hop only.** Resolution follows the supervisor field
ONE level; further redirection is ignored.

**(b) Transitive closure.** Resolution follows the chain to a
fixed point. Risk: cycles. The substrate's parent-acyclic property
should foreclose cycles structurally, but the discipline isn't
automatic.

**Substrate-pull leans (a)** — single-hop is the simplest discipline.
Delegation chains are admissible via explicit `~peer'…'` (the
supervisor of M's home spec MAY name a different `~peer'…'` and that
is ONE redirection). Alex decides.

---

## 12. Cross-references

### Recognitions

- **#84** (`@pack` multi-repo agent runtime): the team{} field IS a
  per-spec scoped `pack`-shaped value. The Pack peer variant is
  the substrate's existing typed surface for team membership.
- **#82** (`@frame` cognitive-order substrate-decl): supervisor
  operates AT-frame; team operates IN-frame; the partition IS the
  frame-relation altitude per recognition #82's Q3 (the reflection
  loop alters the frame; the supervisor IS the reflection altitude
  for the team).
- **#80** (`@magic` form/process substrate-decl): every team entry
  desugars to a magic_contract bind; @magic/contract.honor IS the
  runtime ACL check; @magic/audit IS the violation discharge;
  @magic/reveal IS capability revocation.
- **#57** (alignment-as-boundary-mathematics at @io): the ACL
  surface IS the boundary harness; alignment fires at the team-
  peer-action boundary, not as internal-state shaping.
- **#51** (mirror as expanding Hilbert space): each peer{} block
  adds a substrate dimension scoped to one spec (the supervisor's
  shell's algebra A); cross-spec collaboration is the direct sum
  of supervisor-algebras under explicit team binding.
- **#42** (Bateson logical-type primitive): the consent-geometry
  projection cascades down the Bateson tower; the peer{} block
  authors at type 1 with higher types implicit.
- **#37** (Pask agreement): bilateral predicates throughout
  (pack_coherent, invariant_preserved, audited, mechanism_intact);
  the peer{} block's settle-time obligations are bilateral.

### Prior specs

- **`docs/specs/lambda-shell.md`** (Reed+Alex 2026-05-07): the
  `peer = @<name>` per-spec declaration is the single-field
  ancestor of the supervisor field.
- **`docs/specs/peer-glass.md`** (Reed+Alex 2026-05-25): the @peer
  glass + `@peer(~dir"<path>")` instantiation syntax. The
  five-axis fixed point IS the identity contract.
- **`docs/specs/peer-cognition.md`** (Mara 2026-06-17): the @peer
  root cognition spec; the standalone-use heuristic; sheaf-
  coherence collapse measurement.
- **`docs/specs/geometric-consent-projection.md`** (Mara 2026-06-17):
  ACL as type-1 projection; cascade as natural transformation; the
  `but` operator's algebraic structure; lattice-based access
  control inheritance via Denning.
- **`docs/specs/spectral-runtime.md`** (Mara 2026-06-10): the
  ouroboros runtime; supervision tree; @spectral/supervisor as
  lifecycle owner.
- **`docs/specs/spectral-garden-git-package-manager.md`** (Mara
  2026-06-24; the four-commit cascade ab2e379+): the sibling
  spec at the same mirror.spec altitude; orthogonal surface that
  composes with peer{} at the top level.
- **`docs/specs/threat-model-v0.md`** (Reed 2026-06-12): the
  threat model the peer{} block's ACL surface composes with.
- **`docs/specs/lambda-shell.md`** + **`docs/specs/cli-as-prism.md`**
  + **`docs/specs/the-convergence.md`**: the lambda-shell altitude
  the supervisor field operates at.

### Related shards

- `shards/pack.mirror` (the @pack family-root; peer variant + pack
  record + pack_coherent bilateral).
- `shards/pack/{mara,seam,glint,reed,taut}.mirror` (the five Pack
  peer species; each declares the peer's substrate-decl identity).
- `shards/spectral/supervisor.mirror` (the runtime-altitude
  lifecycle owner).
- `shards/magic/contract.mirror` + `shards/magic/audit.mirror` +
  `shards/magic/reveal.mirror` (the verification + revocation
  lineage).
- `shards/io/git.mirror` (the @io/git adapter; `~git'…'` precedent
  for `~peer'…'`).
- `shards/smarts/pack.mirror` (the @smarts/pack adapter;
  pack_satisfies_smarts the peer{} block discharges at settle).

---

## 13. Honest hedges

### H1. Surface-only; no impl

This spec is RED. No `@mirror/peer` substrate-decl exists yet. No
parser handles `peer { }` blocks in mirror.spec. No resolver lifts
`<ACL>` expressions to substrate verdicts. The spec ratifies the
SHAPE; the discharge is forward-promised across multiple ticks +
Pack rounds.

### H2. Mara writing the spec ahead of substrate-decl

Same inversion as the prior cascade (per spectral-garden-git H2):
Alex's directive landed; this spec is the immediate response. The
shape MAY shift when Reed writes the actual `shards/mirror/peer.
mirror` substrate-decl. This spec then re-ratifies against the
substrate-decl, not vice versa.

### H3. The substrate's existing inheritance is OVERDETERMINED

§2's discovery surfaced SEVEN existing shapes (§2.1–2.7). The peer{}
block's contribution is small (the block syntax + three missing
rules). This is the kintsugi-substrate-already-had-the-word pattern
running at MAXIMUM density; the candidate-recognition flag here
is NOT a new substrate primitive, just a surface unification. The
hedge: if Alex's pull is toward something I MISSED in the discovery,
this spec needs revision. Substrate-pull-honest: I surveyed deeply
but not exhaustively. Specific known gaps in the discovery: no
read of `docs/specs/threat-model-v0.md` body (only headers); no
read of `peer-cognition.md` body (only headers); no read of `cli-
as-prism.md` for the sh-stage's authority surface; no read of
`src/sel/mcp/*.rs` (file not present at the path the brief named).

### H4. Math section names shape, not delivery

Per §10's framing note: the sheaf + lattice + Galois + natural-
transformation + spectral-triple-A framings are mathematical
VOCABULARY for naming what the substrate does. They are NOT formal
proofs. The strongest formal claims are §10.1's sheaf-with-
composition-associativity (the no-permission-leak property) and
§10.2's lattice properties (distributivity on ops + targets axes,
non-distributivity at predicates). Everything else is named shape
for future discharge.

### H5. Open questions intentionally unresolved

§11 enumerates seven surface questions left for Alex. The spec does
NOT resolve them unilaterally; substrate-pull on each leans a
direction (noted inline), but the call is Alex's. Pack discipline:
name the questions, lean transparently, await ratification.

### H6. Spec-vs-shard altitude

This is a `.md` spec, not a `.mirror` shard. The shard substrate-
decl (forward-promised §12 "Related shards") will be SHORTER and
TIGHTER; this spec is the canonical narrative explaining WHY the
shard takes the shape it does. The shard is source-of-truth; this
spec is documentation.

### H7. v0.1 is bounded; v0.2+ deferred

This spec is the peer{} block v0.1. Many shapes deferred:

- explicit `consent { type: N+1, value: <expr> }` cascade authoring (§7.3)
- cross-spec ACL `import` (§7.3)
- pack-membership shorthand `team { pack => … }` (§11 O4)
- explicit `audit_strategy` configuration (§11 O2)
- explicit `delegate { }` for cross-spec composition (§11 O3)
- richer `targets` grammar with predicates (§11 O5)
- transitive supervisor resolution (§11 O7)

v0.1 ratifies the SUPERVISOR + TEAM-with-ACL + LET-bindings + DEFAULT-
TO-REPO-LOCAL surface. The rest lands when substrate-pull sends us
there.

### H8. Alex's framing question is the ratification gate

Alex's verbatim probe: "What if each mirror.spec has a clear owner
peer? If it's undefined the default peer is repo-local?" Both
"what ifs" answered affirmatively in this spec. The supervisor field
IS the clear owner peer; the default-to-repo-local rule (§9) covers
the undefined case. The ratification is Alex's; if either framing
turns out to be wrong, the spec rescinds in that direction.

### H9. Recognition #98-candidate territory not pushed

Per the brief's explicit boundary: do not promote recognitions in
this work. The peer{} block surfaces the SEVEN-INHERITANCE pattern
(§2.8) that COULD be a new substrate-pattern-recognition (the
"surface unification of seven existing shapes" pattern; akin to #43
mirror-as-content-addressed-build-system). Flagged here; not
promoted.

### H10. The supervisor-as-spectral-triple-A claim is hedged

§10.6 names the supervisor as the algebra A of the lambda-shell's
spectral triple. The claim is consistent with the substrate's
existing [[architecture-connes-spectral-triple]] but is NOT
empirically demonstrated against a worked example. Honest framing:
position, not proof.

---

## 14. Pack-discipline trail

- **2026-06-24 (Alex morning)**: directive on `@spectral/garden/git`
  as the substrate's garden-package-manager surface. Mara writes
  the canonical spec in four commits (ab2e379, 66eafb8, a99152a,
  ad03fda).
- **2026-06-24 (Alex · same day)**: probe to Mara — "What if each
  mirror.spec has a clear owner peer? If it's undefined the default
  peer is repo-local? … We already have some of the shape there."
  Mara spawned with discovery + spec brief.
- **2026-06-24 (Mara discovery)**: substrate-pull discovery surfaces
  SEVEN existing shapes (§2.8 table). The peer{} block IS a
  surface unification of inherited substrate; new mechanics are
  three missing rules (self-naming, lexical-scope, default-to-
  repo-local).
- **2026-06-24 (Mara, this spec)**: canonical spec lands across
  six commits:
  1. e89fce6 — skeleton + §1 position statement
  2. 49d493d — §2 discovery (seven inheritance shapes)
  3. d2a13f5 — §3-§6 surface design (block + supervisor + team
     + sigil)
  4. dcb2d69 — §7-§9 composition + defaults
  5. 2630c92 — §10 math (sheaf + lattice + Galois + ocap)
  6. (this commit) — §11-§14 open questions + cross-refs +
     hedges + Pack trail
- **Forward-promised**: Seam adversarial review → Reed consolidation
  → substrate-decl shard `shards/mirror/peer.mirror` → Alex
  ratification → dogfood (mirror's own mirror.spec gains a peer{}
  block).

The Pack-discipline composition this tick: Alex frames; Mara
canonicalizes (TWO specs back-to-back same day — spectral-garden-
git four-commit cascade + peer-acl-surface six-commit cascade);
Seam reviews next; Reed consolidates; substrate-decl shards follow.
Spec-before-shard inversion noted in H2; same pattern as the prior
cascade.

Section caps held; banking discipline held; the substrate-already-
had-the-word pattern (§2.8) drove this spec end-to-end.
