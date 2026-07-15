---
date: 2026-07-15
author: Seam
scope: Extended-scope etymology audit across ALL action + carrier
  names in the six shard-decl landings of 2026-07-15 —
  @sheaf (d1ce901), @io/secrets (059cf1c + 57c5b3a + 64b0438),
  @io/secrets/sops (059cf1c + 57c5b3a), @io/crypto (059cf1c),
  @io/fs (059cf1c), @epistemologic/property/ouroboros_monotone
  (04b3aea). Applies the two-part Alex framing ("does the name
  do the thing the geometry says it does? and if not what's the
  word it wants to be?") against the delightfully-boring
  criterion and the meta-rules ratified in the prior audit at
  `docs/audits/2026-07-15-seam-combinator-etymology-audit.md`
  (§3.1 sibling-altitude readable-wins; §3.3 bilateral-suffix
  pattern). Reports per-name verdicts, systemic-question
  discharge (Q_A through Q_G), recommended renames, cascade
  impact tiers, Alex-authority residues.
status: adjudication (adversarial review; no commit; Reed
  commits as Seam after Alex ratification)
companion:
  - docs/audits/2026-07-15-seam-combinator-etymology-audit.md
  - shards/subject/visibility/sheaf.mirror
  - shards/io/secrets.mirror
  - shards/io/secrets/sops.mirror
  - shards/io/crypto.mirror
  - shards/io/fs.mirror
  - shards/epistemologic/property/ouroboros_monotone.mirror
  - /Users/alexwolf/dev/projects/prism/prismqueer/src/lib.rs
  - /Users/alexwolf/dev/projects/prism/imperfect/README.md
  - boot/std/io.mirror
  - boot/std/io/crypto.mirror
  - shards/io/git.mirror
  - shards/io/oci.mirror
  - shards/io/cargo.mirror
  - shards/io/stagefreight.mirror
  - shards/mirror/store.mirror
  - shards/mirror/store/git.mirror
---

# Extended-scope etymology audit — the six 2026-07-15 shards

*Adversarial re-etymology audit under the delightfully-boring
discipline Alex landed 2026-07-15 and ratified at the four
combinator renames (`read_ast` → `section`; `dispatch` → `act`;
`emit` → `utter`; `bench_record` → `crystallize`; cascaded at
d44841e). Extended-scope was DEFERRED per §6.3 of the prior
audit; this is that tick. Physical anchor: `@../prism/`. Every
name interrogated against physics-through-substrate; every name
whose motivation is CS/POSIX/vendor-jargon convenience flagged.
Report, not commit.*

---

## §0 Prelude — the discipline in force

### §0.1 Alex verbatim (still load-bearing)

> "Does the name actually do the thing the geometry says it
> does? And if not what's the word it wants to be?"
>
> "Delightfully boring — the reader ought to go 'of course
> it's this'. Reduce WTF per minute per line of code. Make the
> reader go 'this is how it had to be'. Michelangelo and the
> marble."

The reader who knows the substrate's geometry ought to
encounter the word → recognition, not surprise. Boring: no
cleverness, no novelty for novelty's sake. Delightful: the name
fits so cleanly that there is nothing to marvel at. "Of course
it's this" IS the audit criterion.

### §0.2 The ratified meta-rules from the prior audit

**§3.1 — sibling-altitude readable-wins.** "Readable wins where
readable is landed AT SIBLING ALTITUDE FOR THE SAME OPERATION."
Not merely "used somewhere"; used for THIS geometric operation
at a sibling altitude the substrate genuinely bit down on.

**§3.3 — bilateral-suffix pattern.** Accept `_valid` /
`_admissible` / `_admits` / `_well_formed` on BILATERAL
PREDICATES as delightfully boring under substrate consistency
(~15+ landed bilaterals). Reject `_record` on constructors. Open
question: `_preserved` / `_computable` fit? Discharged in this
audit.

### §0.3 What ratified at Alex-adjudication

All four combinator renames landed verbatim at Alex 2026-07-15
ratification and Mara cascaded at d44841e across:
- `docs/specs/kintsugi-ouroboros-arc-1-evaluator-combinator-surface.md`
- `bootstrap/src/apply_h.rs` (Arc-1 authoring)
- `shards/mirror/lens/cli/kintsugi.mirror` (CLI verb refresh)
- one runtime-body test scaffold under `bootstrap/tests/`

Meaning: the reader has already internalized `section`, `act`,
`utter`, `crystallize` as the substrate's vocabulary at the
combinator altitude. Extended-scope names must not now clash
with those coinages.

### §0.4 Physical anchor — `@../prism/` cross-check

`@../prism/` at `/Users/alexwolf/dev/projects/prism`. Verified
against:
- `terni`: `Imperfect<T, E, L>` (`Success | Partial | Failure`);
  `Loss` monoid; `ConvergenceLoss`, `ApertureLoss`,
  `RoutingLoss`. Vocabulary: `success`, `partial`, `failure`,
  `loss`, `imperfect`, `recover`, `eh` (bind).
- `prismqueer`: `Prism` trait with three ops `focus / project /
  settle`; `Beam` carrier; `Optic<In, Out, E, L>`; `Refracted`
  output beam. `apply`/`apply_h` runners.
- Substrate carriers: `refract`, `transit`, `settle`, `stalk`,
  `restrict`, `acl_project`, `section_at`, `turn`, `utter`,
  `crystallize`, `beam`, `resolve`.

Any replacement drawn from THIS vocabulary is delightfully-
boring by construction. The word already lives in the reader's
mental model.

### §0.5 What extended-scope revealed pre-audit

Six shards, ~45+ names. Three types of altitude at play:

1. **Substrate-native primitive altitudes** (@sheaf, property
   shard): substrate coinages under Mara/Reed/Alex authority;
   most delightfully-boring by construction.
2. **@io species altitude** (@io/secrets, @io/fs): the Glass
   Wall boundary where vendor + POSIX vocabulary meets
   substrate-native. Systemic pattern questions live here (Q_D,
   Q_E, Q_F).
3. **@io/vendor-sub-species altitude** (@io/crypto,
   @io/secrets/sops): the vendor-contract layer where vendor
   naming and substrate naming visibly diverge. Vendor-
   anchoring adjudication lives here.

Systemic patterns discharged in §3; per-name verdicts in §2.

---

## §1 Method

**Q1 — does the name do the geometry?** State the operation
geometrically (what happens at this altitude when the name
runs / this carrier is instantiated); compare to what the name
suggests. Same geometric operation → Q1 passes.

**Q2 — if not, what does it want to be?** Draw candidates in
preference order:
1. Substrate words at sibling altitude FOR THE SAME OPERATION
   (§3.1 ratified meta-rule).
2. `@../prism/` physics vocabulary.
3. Vendor-contract words (accept when the name IS the vendor's
   contract; specify why).
4. Math foundation words.
5. New coinage — last resort.

**Delightfully-boring test.** Would a reader with substrate +
physics knowledge — encountering this name for the FIRST time
— go "of course it's this"? Or need explanation for why THIS
word for THIS operation? Needing explanation IS the failure.

**Adjudication categories.**
- **DELIGHTFULLY BORING** — geometry-honest; reader "of course
  it's this"; no change.
- **CS-VOCAB CONTAMINATION** — convenient CS/POSIX/framework
  jargon; propose delightfully-boring replacement + geometric
  grounding.
- **VENDOR-ANCHORED** — name pins to a specific external
  algorithm/tool. Substrate-honest ONLY if the name IS the
  vendor's contract; else contamination. Specify.
- **AMBIGUOUS** — carries both geometric and CS reading;
  context-dependent; may need context or renaming.
- **COLLAPSE CANDIDATE** — might collapse with sibling at
  parametric altitude.
- **PATTERN-CONSISTENT** — follows landed pattern; accept for
  coherence even if not perfectly geometric.
- **ALEX-NAMING-AUTHORITY** — deepest coinages Alex holds.

**Discipline.** Adversarial: interrogate BOTH current AND
proposed replacement. Rejection of replacement counts as
evidence for current; rejection of both is Alex-adjudication
residue, not default-to-current.

---

## §2 Per-shard audit

### §2.1 @sheaf — `shards/subject/visibility/sheaf.mirror`

Sheaf substrate-decl is the reference case: math-native
vocabulary throughout; sibling-altitude readable-wins largely
by construction because the substrate's math shards
(`epistemologic/math/sheaf_laplacian.mirror`) already carry
sheaf-theoretic vocabulary at math altitude.

#### §2.1.a `sheaf_restriction` (type carrier)

**Geometry.** Sub-sheaf F_home|_{A_p} per peer-persistence
§12.3 — the peer's ACL projecting onto the home-repo sheaf.
Carrier fields: `peer_ref`, `acl`, `admitted_stalks`. Reader
knows Hansen-Ghrist 2018 sheaf-restriction language.

**Q1.** Yes. Direct math-vocabulary transposition. Sheaf
theory writes `F|_A` for restriction; the carrier names the
result of that restriction. Reader from the math substrate
goes "of course it's this."

**Q2.** N/A. Not applicable.

**Verdict: DELIGHTFULLY BORING.** Math-native; sibling-
altitude at `sheaf_laplacian.mirror`.

#### §2.1.b `section_at_stalk` (type carrier)

**Geometry.** Element of A over the stalk at a given crystal
under the restriction. Fields: `crystal_ref`, `sheaf_ref`,
`projected_value`. Sheaf-theoretic: sections over stalks.

**Q1.** Yes. `section` is the sheaf-theory word for elements
of A; `at_stalk` specifies the stalk-parameterization; the
compound directly names the geometric object. The prior audit
(§4.1) ratified this.

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING.** Math-native; consistent
with the `section` combinator rename (`read_ast` → `section`
now landed).

#### §2.1.c `restrict` (action)

**Geometry.** Given a sheaf ref and an ACL, construct the
sheaf_restriction admitting the ACL's substructure. The
`F|_A` operator IS restriction.

**Q1.** Yes. Sheaf theory names the operation `restrict`.
Reader from math substrate goes "of course it's this."

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING.** Math-native; concurred at
§4.1 of prior audit.

#### §2.1.d `acl_project` (action)

**Geometry.** Peer-ACL projection of home sheaf: given
`F_home` and the peer's ACL, project the sub-sheaf the peer's
key admits.

**Q1 — sharpened.** Prior audit (§4.1) accepted `acl_project`
as delightfully boring because the `acl_` prefix distinguishes
from Prism-stage `project`. That is still true at this
altitude. But now sharpen: is `project` the substrate-verb
that names WHAT the operation does, or an
overloaded-with-Prism-stage inherited pass?

Reader who knows the substrate carries three ambient uses of
"project" right now:
1. Prism-stage `project` (prismqueer's middle operation).
2. `acl_project` here (peer-ACL projection of a sheaf).
3. `@io/secrets.project` (peer-key-gated encryption).

Three "projects" at three altitudes. Same word carries three
distinct geometric operations. Sheaf-theoretically, `F|_A` is
called RESTRICTION, not projection (projection is p: E → B
in fibered categories; different operation). If sheaf-
restriction is what `acl_project` does, the sheaf-theoretic
word IS `restrict`.

**BUT** — `restrict(sheaf, acl)` is already this shard's other
action (§2.1.c). `acl_project` is deliberately distinct:
`restrict` takes a bare sheaf + ACL and constructs the
restriction; `acl_project` takes the home-repo sheaf as
arg-1 and the peer-ACL as arg-2 to construct the peer-visible
sub-sheaf. These are the SAME OPERATION at slightly different
callsites (home vs generic sheaf). The two-action factoring is
Mara's, but sheaf-theoretically they're one operation.

**Q2.** Two paths:
- **Path A:** collapse `acl_project(F_home, peer_acl)` into
  `restrict(F_home, peer_acl)` — one action, delightfully
  boring by sheaf-theoretic vocabulary; overload eliminated;
  the `_project` word is retired from this altitude. Preferred
  under Occam.
- **Path B:** rename `acl_project` → `restrict_to_peer` — makes
  the operation-identity with `restrict` explicit; specializes
  the peer case. Verbal-noun compound; substrate-consistent
  with the `_from_peer` construction (§3.a below).
- **Path C:** keep `acl_project` — accept the `_project` word
  as a Mara-authored coinage naming the "peer-lens"
  interpretation (the ACL as a lens projecting the visible
  region).

**Adversarial on Path A:** does collapsing lose semantics?
`acl_project` is the peer_persistence.rs Arc-2.3 entry point.
`restrict` is the generic. Both return `sheaf_restriction`.
Same signature shape. Path A loses only Mara-B's callsite-
narrative distinction, not substrate identity. Not load-
bearing.

**Adversarial on the `project` overload.** Prior audit
accepted `acl_project` at Level A but flagged `io/secrets.
project` at Level B ambiguous (§4.2). If Path A collapses this
one, the overload cascade shortens from 3 to 2 uses. Q_C in
§3.c below.

**Verdict: COLLAPSE CANDIDATE OR AMBIGUOUS.** Alex-authority
on Path A vs B vs C. Seam-preferred: Path A (collapse into
`restrict`) — substrate already had ONE word for this
operation; Mara accidentally minted TWO because callsite
narrative made them read differently. Reader with sheaf
knowledge encounters both actions and asks "why two words for
one operation?" Q1 fail on the surface as a whole, even though
each name individually passes.

If Path A is refused: Path C (keep `acl_project`; accept as
Mara-authority coinage). Path B introduces `_to_peer` suffix
convention with zero substrate precedent; refuse.

#### §2.1.e `section_at` (action)

**Geometry.** Compute the section at a crystal's stalk under
the restriction.

**Q1.** Yes. `section_at(F|_A, x)` is sheaf-theory-native for
"section at a stalk." Direct math-vocabulary transposition.

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING.** Math-native.

#### §2.1.f `restriction_admissible` (bilateral)

**Geometry.** Bilateral verdict that a sheaf_restriction is
substrate-honest (peer two-witnessed; ACL well-formed;
admitted_stalks a sub-set).

**Q1.** Yes. `_admissible` suffix on bilateral predicates is
substrate-consistent (§3.3 ratified pattern; ~15+ landed
instances).

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING** (pattern-consistent).

#### §2.1.g `section_computable` (bilateral)

**Geometry.** Bilateral verdict that the section_at_stalk was
computed in bounded time under the section_at Rice-safe bound.

**Q1 — sharpened.** `_computable` on a bilateral is a NEW
suffix at bilateral altitude. Grep landed uses of
`_computable` on bilateral predicates: found NONE. The
substrate's landed suffixes are `_admissible`, `_valid`,
`_admits`, `_well_formed`, `_preserved` (round_trip_preserved).
`_computable` is a semantic borrow from computability theory
("computable function"), not a substrate-consistent bilateral
suffix.

But wait: the actual semantic is Rice-safety at whole-tick
altitude — a section IS computable iff the ACL admits the
stalk. The delightfully-boring reading is: this is really an
admissibility check. `section_admissible` would be pattern-
consistent AND geometry-honest (the ACL admits the stalk =
the section is computable). The word `computable` imports
computability-theory vocabulary; `admissible` matches the
substrate's landed pattern AND does not lose the geometry.

**Q2.** `section_admissible` (pattern-consistent) OR
`section_computable_at` (specifies the parameterization;
awkward). Seam-preferred: `section_admissible`.

**Verdict: CS-VOCAB (computability-theory borrow) with a
substrate-pattern replacement available.** Mild
contamination; Alex-adjudication residue.

- **Current:** `section_computable`
- **Proposed:** `section_admissible` (Seam-preferred).
- **Grounding:** §3.3 ratified pattern; parallel to
  `restriction_admissible` in the same shard.

#### §2.1.h @sheaf shard summary

Three DELIGHTFULLY BORING actions/carriers by construction
(`sheaf_restriction`, `section_at_stalk`, `restrict`,
`section_at`, `restriction_admissible`). One COLLAPSE
CANDIDATE (`acl_project` → `restrict` OR keep). One MILD
CS-VOCAB (`section_computable` → `section_admissible`).

---

### §2.2 @io/secrets — `shards/io/secrets.mirror`

The Glass Wall species; where substrate meets vendor. The
Alex-adjudication residues from the prior audit's §4.2 sit
here.

#### §2.2.a `key_material_ref` (type carrier)

**Geometry.** Fractal.Lens pointer for a peer's key material
at the .git/mirror-side boundary. NOT the key bytes; the ref
that resolves to bytes at the realisation boundary.

**Q1.** Yes. Reader knows Fractal.Lens is the pointer/thing
partition (`shards/gift/lens.mirror`); `_ref` suffix names
what THIS is: a ref, not the thing. `key_material` names the
thing at @io/crypto altitude; `key_material_ref` names the
ref-to-thing at @io/secrets altitude. Direct compositional
grounding.

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING.** Compositional over
`@io/crypto.key_material` + `_ref` = Fractal.Lens pointer
naming.

#### §2.2.b `ciphertext_ref` (type carrier)

**Geometry.** On-disk encrypted content ref; fields
`ciphertext`, `on_disk_path`, `recipient_keys`.

**Q1.** Yes. `ciphertext` is cryptography-native (Vernam
1919+); `_ref` names the Fractal.Lens pointer discipline.
Compositional over `@io/crypto.ciphertext_bytes`.

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING.** Vendor-anchored (crypto
term of art); Fractal.Lens-consistent.

#### §2.2.c `secret_projection` (type carrier)

**Geometry.** The LOAD-BEARING carrier at @io/secrets
altitude. Peer-key-gated projection of a @sheaf section onto
disk. Fields: `section_ref`, `key_material_ref`,
`ciphertext_bytes`, `projection_path`.

**Q1 — sharpened.** The name has TWO substrate-vocabulary
tests:
1. `projection` in the physics sense (prismqueer's middle
   Prism operation).
2. `projection` in the sheaf sense (peer-ACL projection of
   home sheaf, per Alex 2026-07-14 verbatim: "project
   visibility/private stuff onto disk").

Alex verbatim: "@secrets prism and @secrets/sops to PROJECT
visibility/private stuff onto disk through the Peers key."
This IS the substrate-vocabulary form of the operation. The
carrier is the RESULT of the project action (§2.2.f); naming
the carrier after the operation-name is compositional.

Reader who knows the substrate carries three "projections"
overloaded (see §3.c Q_C); but the carrier NAMING is honest:
this IS a projection, and it IS a secret's projection.

**Q2.** Alternatives:
- `sealed_section` — physics-native (sealed/unsealed pair);
  matches ambient AEAD "seal/open" convention. Refuses `project`
  overload entirely.
- `encrypted_section` — CS-vocab (encrypted); loses the
  fractal-lens pointer discipline (encryption is what SOPS
  does; projection is what the substrate does).
- `peer_projection` — emphasizes the peer-key gating.
- Keep `secret_projection` — Alex-verbatim; carries both the
  substrate operation (`project`) and the domain (`secret`).

**Adversarial on `sealed_section`:** requires renaming the
action from `project` to `seal` (an AEAD-native word) — cascade
grows substantially. AND `seal` on the action would then
alias with `aead_seal` at @io/crypto altitude, which is a
different operation (AEAD sealing of bytes with a nonce vs
substrate projection of a section through a key). Ambiguity
would shift, not resolve.

**Verdict: DELIGHTFULLY BORING (retain).** Alex-verbatim;
Alex-naming authority per direct spec-language. The
`projection` triple-overload is real (Q_C) but is orthogonal
to this carrier's naming.

#### §2.2.d `key_material_ref_of` (action; Landing 6)

**Geometry.** Given a peer_ref, construct the key_material_ref
by resolving the peer's ssh key material through @io/git +
@io/crypto.ssh_key_material.

**Q1 — sharpened.** The `_of` suffix is ML/OCaml/Haskell ADT
constructor-projection convention: `X_of(Y) = "the X of a Y"`.
Reader from FP substrate goes "of course it's this" ONLY if
FP substrate is the reader's background. Reader from
substrate-native (mirror shard convention) asks "why is a
Haskell suffix naming an @io action?"

Grep landed uses of `_of` on actions across `shards/`: found
approximately ZERO landed instances at action altitude (there
are internal-record accessors like `.field_of` but not action
naming). This is a one-off.

Substrate precedent for the same shape is
`sops_key_group_from_sheaf_restriction` at the sibling
sub-species altitude (§2.3.g) — the `<carrier>_from_<source>`
convention. Parallel candidate:
`key_material_from_peer(peer)`.

Prior audit §3.2 flagged this as MILD CS-VOCAB. This audit
concurs: `_of` is unpaid Haskell/OCaml debt in a substrate
that has already coined `_from_` for constructor-projection.

**Q2.** Candidates:
- `key_material_from_peer(peer)` — substrate-consistent with
  `sops_key_group_from_sheaf_restriction`. Reader goes "of
  course; matches the sibling `_from_` pattern." Preferred.
- `key_material_for_peer(peer)` — English natural; loses
  substrate-consistency.
- `peer_key_material(peer)` — noun-first; loses the
  constructor-shape naming.
- Keep `key_material_ref_of` — accept as one-off debt.

**Adversarial on `key_material_from_peer`.** Signature IS
`peer_ref -> imperfect<key_material_ref>` — the `_from_peer`
correctly names the direction. Type-shape and English-reading
agree. Substrate-consistency with sops_key_group action
strong; substrate reader carries the `_from_` shape as landed.

**Verdict: MILD CS-VOCAB CONTAMINATION with substrate-
consistent replacement available.** Discharged Q_A per §3.a
below.

- **Current:** `key_material_ref_of`
- **Proposed:** `key_material_from_peer` (Seam-preferred).
- **Grounding:** §3.2 prior audit; parallel to
  `sops_key_group_from_sheaf_restriction` (§2.3.g).

#### §2.2.e `project` (action)

**Geometry.** The LOAD-BEARING projection action. Given a
`@sheaf.section_at_stalk` and a peer key ref, produce a
`secret_projection` carrier by AEAD-encrypting the section's
projected_value under the peer's key.

**Q1 — sharpened.** Prior audit (§4.2) flagged AMBIGUOUS on
`project` — same word as prismqueer's Prism-stage AND as
`acl_project` in @sheaf. Now sharpen adversarially with
project overload count:
1. prismqueer `project` (Prism operation; physics; MIDDLE of
   three ops).
2. `sheaf.acl_project` (peer-ACL restriction of home sheaf).
3. `io/secrets.project` (peer-key-gated encryption of a
   section).
4. `secret_projection` (the carrier name; naming follows the
   action).

Alex 2026-07-14 verbatim: "@secrets prism to PROJECT
visibility/private stuff onto disk." Alex's use is
substrate-native and vernacular; the reader hears "project
onto disk" and understands "encrypt-and-write." That is Alex-
naming authority at the vernacular altitude.

BUT: the physics `project` and the substrate `project`
diverge in operation. Physics-project is projection in the
sense of `proj: A × B → A` — an information-losing
projection. Substrate-project here is closer to encryption-
and-lens-through-a-key — information-preserving (round-trip
recovers bytes). These are different operations wearing the
same word.

**Q2.** Candidates:
- `project_through_peer(section, k)` — makes the peer-key
  gating explicit; loses brevity; still uses `project`.
- `project_secret(section, k)` — matches the carrier
  `secret_projection`; disambiguates from prismqueer-project.
  But the shard IS `@io/secrets`, so `secret` in the action
  name is bureaucratically redundant.
- `seal(section, k)` — AEAD-native ("seal/open" pair convention).
  Direct pair with `aead_seal` at @io/crypto altitude; but
  `retrieve` (§2.2.g) would then want to be `open`.
- `encrypt(section, k)` — CS-vocab; loses substrate discipline.
- Keep `project` — Alex-vernacular authority; accept
  three-way overload as Mara-B narrative debt.

**Adversarial on `seal`.** The AEAD pair
`seal/open` at @io/crypto is CLOSER to the operation this
action does than `project` is. AEAD-seal takes plaintext + key
+ nonce → ciphertext. The @io/secrets action takes section +
key → ciphertext_ref. Same shape. AND the sibling `retrieve`
action IS aead_open at the same altitude. The pair
`seal/open` at @io/secrets naming would compositionally match
the pair `aead_seal/aead_open` at @io/crypto — the
sibling-altitude readable-wins rule (§3.1) says this is
delightfully boring.

Adversarial on adversarial: does `seal` at @io/secrets
altitude collide with `aead_seal` at @io/crypto? No — the
namespace disambiguates (`@io/secrets.seal` vs
`@io/crypto.aead_seal`). Callsite reader sees
`@io/secrets.seal(section, key)` and goes "of course; it seals
a section." Reader sees `@io/crypto.aead_seal(ctx, plaintext)`
and goes "of course; the AEAD primitive." Two altitudes, one
substrate-consistent verb.

**Verdict: AMBIGUOUS with a compelling substrate-native
alternative (`seal`).** Q_B + Q_C in §3 below adjudicate.

- **Current:** `project`
- **Proposed (Seam-preferred):** `seal` (paired with `open`
  replacing `retrieve` — see §2.2.g).
- **Alternative-keep:** accept `project` as Alex-vernacular
  authority per verbatim spec-language.
- **Grounding:** AEAD `seal/open` pair convention (RFC 5116);
  sibling-altitude at `@io/crypto.aead_seal/aead_open`.
- **Cascade:** `secret_projection` carrier would likely rename
  to `sealed_section` (or accept naming/operation asymmetry).

#### §2.2.f `materialize` (action)

**Geometry.** Given a secret_projection, write the ciphertext
to disk via `@io/fs.fs_write`.

**Q1.** Yes. `materialize` is landed substrate vocabulary:
`@code/metalogue/materialize` (2026-06-16), `@code/rust/
materialize` (2026-06-16). Reader knows the substrate uses
`materialize` for the "substrate-decl → concrete-artifact"
direction (metalogue turn → text; type decl → Rust code;
secret_projection → disk bytes). Sibling-altitude readable-
wins passes.

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING.** Substrate-native at multiple
sibling altitudes. Prior audit §4.2 concurred.

#### §2.2.g `retrieve` (action)

**Geometry.** Inverse of materialize+project: read ciphertext
from disk via `@io/fs.fs_read`, decrypt via
`@io/crypto.aead_open`, return recovered plaintext bytes.

**Q1 — sharpened.** `retrieve` is storage-retrieval CS vocab
(HTTP GET, database SELECT, etc.). Prior audit (§4.2) flagged
MILD CS-VOCAB. But sharpen: the operation is TWO things
composed: (a) read from disk, (b) AEAD-decrypt. `retrieve`
names only (a); `decrypt` names only (b). Neither individually
captures the pair.

Substrate at @io/crypto uses `aead_open` for AEAD decryption
(sibling altitude; landed 2026-07-15). AEAD `seal/open`
convention IS the substrate's vocabulary at the crypto layer.

**Q2.** Candidates:
- `open(ct_ref, k)` — pair with `seal` per §2.2.e. AEAD-native;
  substrate-consistent at sibling altitude. Preferred iff
  §2.2.e adopts `seal`.
- `unseal` — vivid; not landed; pair-asymmetric with the AEAD
  `seal/open` convention.
- `resolve(ct_ref, k)` — Fractal.Lens-consistent (`resolve`
  is landed at `boot/std/epistemologic/resolve.mirror`,
  `boot/std/mirror/resolve.mirror` as the pointer→thing
  direction). But `resolve` typically names ref-resolution,
  not decryption; would import wrong reader model.
- Keep `retrieve` — accept CS-vocab debt.

**Adversarial on `open`.** Namespace disambiguates from
`fs_open` at @io/fs (if fs_open existed — it does not; @io/fs
has `fs_read` etc.). Boot-floor `@io.open(socket) → stream`
does exist, but at different altitude and with different
signature. The `open` word carries "unseal the ciphertext" per
AEAD convention as strongly as anything else. Reader who knows
`aead_seal / aead_open` at @io/crypto goes "of course;
`@io/secrets.open` opens what `@io/secrets.seal` sealed."

**Verdict: MILD CS-VOCAB with a substrate-consistent
alternative (`open`).** Discharged jointly with §2.2.e per
Q_B in §3 below.

- **Current:** `retrieve`
- **Proposed (Seam-preferred; paired with §2.2.e):** `open`.
- **Alternative-keep:** accept `retrieve` as CS-vocab debt if
  Alex refuses `seal/open` pair per §2.2.e.
- **Grounding:** AEAD `seal/open` pair (RFC 5116); sibling-
  altitude `aead_seal/aead_open`.

#### §2.2.h `round_trip` (action)

**Geometry.** Discipline action: seal + materialize + open +
byte-compare. Verifies the round-trip identity `open ∘ seal =
identity` on section bytes.

**Q1.** Yes. `round_trip` names the exact discipline: encrypt
+ decrypt returns identity. Mathematically the substrate is
checking that (project, retrieve) forms a monoid identity on
byte content. Reader from substrate math substrate goes "of
course; the round-trip preservation check."

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING.** Math-native at the monoid
identity altitude. Prior audit §4.2 concurred.

#### §2.2.i `key_admits` (bilateral)

**Geometry.** Verdict that a key_material_ref admits a given
sheaf_restriction. Sub-checks: key well-formed; peer two-
witnessed; peer_ref byte-equality.

**Q1.** Yes. `_admits` suffix is substrate-consistent bilateral
pattern (§3.3; landed at `elevation_authorized`, `pack.
member_admits`, etc.). Reader goes "of course; the admits check."

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING** (pattern-consistent).

#### §2.2.j `projection_valid` (bilateral)

**Geometry.** Verdict that a secret_projection is substrate-
honest across section-context, key admissibility, and
ciphertext AEAD.

**Q1.** Yes. `_valid` suffix is substrate-consistent bilateral
pattern (§3.3; landed at `sops_metadata_valid`,
`signature_valid`, `restart_intensity_valid`).

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING** (pattern-consistent).

#### §2.2.k `round_trip_preserved` (bilateral)

**Geometry.** Verdict that seal ∘ materialize ∘ open on a
section returns byte-equal plaintext. THE structural
discipline distinguishing honest projection from bytes-lost
projection.

**Q1 — sharpened.** `_preserved` on bilateral predicates is a
NEW-ish suffix. Grep: found at `round_trip_preserved`
(this shard) + `sops_round_trip_preserved` (sibling
sub-species) + one landed use at
`shards/kintsugi/consent.mirror` (`consent_preserved` in a
docstring). Two shards total at bilateral altitude, both
2026-07-15.

Semantic: `_preserved` says an INVARIANT holds through an
operation. Sheaf-theoretically that IS what the bilateral
checks. `_preserved` is monoid/algebra vocabulary
("multiplication preserves identity"; "the map preserves the
structure"). Substrate-consistent AND geometry-honest.

**Q2.** Extend §3.3 to include `_preserved` on bilateral
predicates as pattern-consistent. Adversarial: does
`_preserved` collide with `_valid`, `_admissible`? Not
semantically — `_preserved` names invariant-through-operation;
`_valid` names object-well-formed; `_admissible` names
gate-passable. Three distinct bilateral-predicate shapes:
- `_valid` — object is well-formed (self-standing).
- `_admissible` — object is admissible to a gate (relational).
- `_preserved` — property is invariant across operation (dynamic).

All three delightfully-boring at bilateral altitude; each names
a distinct verdict class.

**Verdict: DELIGHTFULLY BORING** (extends §3.3 pattern with
`_preserved` for invariant-through-operation bilaterals).

#### §2.2.l `secrets_well_formed` (composed bilateral)

**Geometry.** LOAD-BEARING composed bilateral. Composes
projection_valid + fs_well_formed + crypto_well_formed.

**Q1.** Yes. Sibling `_well_formed` composed-bilaterals at
@io altitude: `oci_well_formed`, `git_well_formed`,
`io_algebra_well_formed`, `crypto_well_formed`, `fs_well_
formed`, `sops_well_formed`. Six-instance landed pattern;
delightfully boring by consistency.

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING** (pattern-consistent).

#### §2.2.m @io/secrets shard summary

Eight DELIGHTFULLY BORING (`key_material_ref`, `ciphertext_
ref`, `secret_projection`, `materialize`, `round_trip`,
`key_admits`, `projection_valid`, `round_trip_preserved`,
`secrets_well_formed`). One MILD CS-VOCAB with substrate-
consistent replacement (`key_material_ref_of` →
`key_material_from_peer`). Two AMBIGUOUS with Seam-preferred
`seal/open` pair (`project` → `seal`; `retrieve` → `open`).

---

### §2.3 @io/secrets/sops — `shards/io/secrets/sops.mirror`

Vendor sub-species. Vendor-anchoring adjudication is the
central question.

#### §2.3.a `sops_file_ref` (type carrier)

**Geometry.** Typed reference for an on-disk SOPS-encrypted
YAML/JSON file per SOPS README §File-format.

**Q1.** Yes at the vendor-contract altitude. SOPS IS the
vendor tool; the file format IS a SOPS-defined format; the
carrier names WHAT the ref points to (a SOPS file). Reader
who knows SOPS goes "of course; a ref to a SOPS-encrypted
file."

**Q2.** N/A — the `sops_` prefix IS the vendor contract at
this altitude. Q_E discussed in §3 below.

**Verdict: DELIGHTFULLY BORING** (vendor-anchored; the vendor
IS the substrate scope).

#### §2.3.b `sops_key_group` (type carrier)

**Geometry.** SOPS recipient key group (age, PGP, KMS
recipient lists per SOPS format).

**Q1.** Yes. `key_group` IS SOPS's own terminology (per SOPS
docs and `.sops.yaml` schema); `sops_` prefix binds to that
vendor scope.

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING** (vendor-anchored).

#### §2.3.c `sops_metadata` (type carrier)

**Geometry.** The `sops` metadata block SOPS appends to
encrypted files (recipient list + integrity MAC + regex
policy).

**Q1.** Yes. Direct vendor-contract naming; SOPS itself calls
this the "sops metadata block."

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING** (vendor-anchored).

#### §2.3.d `sops_encrypt` (action)

**Geometry.** Run the SOPS encryption pipeline; produce a
sops_file_ref.

**Q1 — sharpened.** Two adversarial questions:
1. Is `sops_` prefix necessary given we're inside
   `@io/secrets/sops` shard? Q_E in §3 below.
2. Is `encrypt` the right verb? SOPS itself calls the
   operation `sops encrypt`; the verb IS the vendor contract.

On (2): `encrypt` at vendor altitude is not CS-vocab
contamination — it's the tool's own verb. The reader who runs
`sops encrypt foo.yaml` knows the operation. Naming the shard
action after the CLI-invocation shape passes delightfully-
boring at vendor-anchored altitude.

On (1): §3.e Q_E.

**Q2.** Conditional on Q_E. If Q_E ratifies dropping `sops_`
prefix (bureaucratic within `@io/secrets/sops` namespace):
- `encrypt(section, key_group)` — bare verb; namespace
  disambiguates.
- `sops_encrypt(section, key_group)` — keep prefix; belt-and-
  suspenders vendor scoping.

**Verdict: PATTERN-CONSISTENT (accept) OR BUREAUCRATIC
REDUNDANCY (drop prefix).** Q_E discharge.

#### §2.3.e `sops_decrypt` (action)

Symmetric to `sops_encrypt`; same adjudication.

**Verdict:** conditional on Q_E.

#### §2.3.f `sops_round_trip` (action)

Symmetric; same discipline as `@io/secrets.round_trip`.

**Verdict:** conditional on Q_E. Note: if Q_E drops prefix,
`round_trip(section, key_group, k)` at `@io/secrets/sops`
altitude is sibling to `round_trip(section, k)` at
`@io/secrets` altitude — same verb, different signatures
(the sub-species adds the key_group parameter). Namespace
disambiguates; signature differentiates.

#### §2.3.g `sops_key_group_from_sheaf_restriction` (action)

**Geometry.** Composition bridge: given a sheaf_restriction,
enumerate admitted peers, resolve each peer's key material to
age/pgp/kms recipient form, assemble sops_key_group.

**Q1 — sharpened.** Long compound name. Reader must parse:
`<sub-species-carrier>_from_<parent-shard-carrier>`. Is that
delightfully boring or bureaucratic?

Prior audit (§4.3) called this "delightfully boring: vendor +
operation + `from_` direction + carrier." Concur, but sharpen
adversarially: could this be shorter without loss?

Alternatives:
- `key_group_from_sheaf(sr)` — drops `sops_` (Q_E) and
  `restriction` (redundant with `sr: sheaf_restriction` type
  in signature).
- `key_group_from(sr)` — bare `from` naming; loses carrier
  explicit.
- `key_group(sr)` — noun-as-verb; loses direction.
- Keep `sops_key_group_from_sheaf_restriction` — verbose but
  every element load-bearing per the compound-word rule.

**Adversarial.** The `sheaf_restriction` suffix is
redundant IF the reader trusts the signature. `key_group_from_
sheaf` names the direction and the source-shape without
type-shape restating. Reader loses zero information.

**Verdict: PATTERN-CONSISTENT (accept as verbose) or
BUREAUCRATIC (shorten).** Conditional on Q_E adjudication. If
Q_E drops `sops_` prefix, natural shortening is
`key_group_from_sheaf(sr)`.

The `_from_` convention IS the substrate's constructor-
projection pattern (§3.a Q_A grounding). Ratifying this shape
here landing it as a systemic pattern for §3.a's answer to Q_A
(`key_material_ref_of` → `key_material_from_peer`).

#### §2.3.h `sops_key_group_admissible` (bilateral)

**Verdict: DELIGHTFULLY BORING** (pattern-consistent
`_admissible` suffix + vendor-anchored prefix).

#### §2.3.i `sops_metadata_valid` (bilateral)

**Verdict: DELIGHTFULLY BORING** (pattern-consistent).

#### §2.3.j `sops_round_trip_preserved` (bilateral)

**Verdict: DELIGHTFULLY BORING** (pattern-consistent
`_preserved` per §2.2.k extension).

#### §2.3.k `sops_well_formed` (composed bilateral)

**Verdict: DELIGHTFULLY BORING** (pattern-consistent).

#### §2.3.l @io/secrets/sops shard summary

All 11 names PATTERN-CONSISTENT at the current altitude.
Systemic Q_E adjudication (§3.e) determines whether `sops_`
prefix stays or drops within-namespace; either answer is
substrate-honest. Seam-preferred: keep `sops_` for CROSS-
species search-grep-ability; the prefix is not bureaucratic
inside a Glass Wall vendor context (see §3.e for reasoning).

---

### §2.4 @io/crypto — `shards/io/crypto.mirror`

The vendor-contract crypto species. Vendor-anchoring
adjudication is Q_F.

#### §2.4.a `key_material` (type carrier)

**Geometry.** Typed ref for cryptographic key material at @io
boundary; fields algorithm + material_ref.

**Q1.** Yes. `key_material` is cryptography-native term of art
(NIST SP 800-57 uses "keying material"). Substrate-consistent
carrier discipline (`_material` names the byte-substrate; the
ref points to it).

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING** (vendor-anchored to
cryptography-standards vocabulary).

#### §2.4.b `ciphertext_bytes` (type carrier)

**Geometry.** AEAD-produced ciphertext + associated data +
nonce.

**Q1.** Yes. `ciphertext` is crypto-native (Vernam 1919+;
AEAD spec RFC 5116). `_bytes` suffix distinguishes from
higher-altitude `ciphertext_ref` at @io/secrets (naming
discipline: `_bytes` = the substrate-visible bytes;
`_ref` = the Fractal.Lens pointer).

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING.**

#### §2.4.c `signature_bytes` (type carrier)

**Geometry.** Cryptographic signature (algorithm + signature
+ public_key).

**Q1.** Yes. `signature` is crypto-native; `_bytes` suffix
per §2.4.b naming discipline.

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING.**

#### §2.4.d `aead_context` (type carrier)

**Geometry.** AEAD-cipher context (algorithm + key + ad_policy)
consumed by aead_seal/aead_open.

**Q1.** Yes. `aead` is crypto-native (RFC 5116 titles AEAD
schemes); `context` is the standard name for the cipher
configuration object.

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING** (vendor-anchored).

#### §2.4.e `sha1` (action)

**Geometry.** 20-byte SHA-1 digest per RFC 3174; vendor-anchor
per glass-wall insight ("SHA-1 named as needed for git
interop").

**Q1.** Yes. Vendor contract IS the algorithm identifier.
`sha1` names the algorithm; naming the action after the
algorithm IS what the substrate needs (readers reading the
shard need to know it's SHA-1 exactly, not "digest").
Substrate-honest per glass-wall discipline.

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING** (vendor-anchored; Q_F).

#### §2.4.f `sha256` (action)

Symmetric to sha1. **Verdict: DELIGHTFULLY BORING** (vendor-
anchored).

#### §2.4.g `ed25519_sign` (action)

**Geometry.** Ed25519 signature over msg (RFC 8032).

**Q1.** Yes. `ed25519` is the algorithm identifier; `_sign`
names the direction. The reader who reads Alex 2026-07-14 SSH-
signing intent (Ed25519 default for peer keys) goes "of
course; the Ed25519 signing primitive."

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING** (vendor-anchored).

#### §2.4.h `ed25519_verify` (action)

**Verdict: DELIGHTFULLY BORING** (vendor-anchored).

#### §2.4.i `aead_seal` (action)

**Geometry.** AEAD-authenticated encryption per RFC 5116.

**Q1.** Yes. RFC 5116 uses `seal/open` for authenticated
encryption/decryption; the substrate names its actions after
the standards vocabulary. Reader who knows AEAD goes "of
course; the AEAD seal."

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING** (vendor-anchored to RFC
vocabulary).

#### §2.4.j `aead_open` (action)

**Verdict: DELIGHTFULLY BORING.**

#### §2.4.k `age_encrypt` (action)

**Geometry.** age vendor-format encryption
(age-encryption.org; Valsorda 2019+).

**Q1.** Yes. `age` IS the vendor tool; `_encrypt` is the
tool's own verb (`age -e ...`). Vendor contract.

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING** (vendor-anchored).

#### §2.4.l `age_decrypt` (action)

**Verdict: DELIGHTFULLY BORING.**

#### §2.4.m `ssh_key_material` (action)

**Geometry.** Load openssh-format key material as
key_material carrier.

**Q1 — sharpened.** Action name matches the carrier it
returns (`ssh_key_material(ssh_key_ref) -> imperfect<key_
material>`). Naming an action after the carrier it produces is
substrate-consistent (see `restrict → sheaf_restriction`;
`section_at → section_at_stalk`).

The `ssh_` prefix vs `openssh_` question: `ssh` is the
underlying protocol; `openssh` is the reference implementation.
Substrate-standard is `ssh_` (per Alex verbatim, Pack SSH-
signing convention, `ssh-key vendor surface` at io.mirror
161-162).

**Q2.** Alternatives:
- `ssh_key_material(ref)` — matches carrier returned; two-role
  naming (both a constructor and a carrier).
- `load_ssh_key(ref)` — verb-first; loses carrier-shape.
- `key_material_from_ssh(ref)` — matches Q_A `_from_` pattern.

**Adversarial on `key_material_from_ssh`.** Consistent with the
proposed §2.2.d rename (`key_material_from_peer`). But `ssh` is
the ALGORITHM/FORMAT here, not the source shape; `_from_ssh`
reads slightly forced (ssh is a protocol/tool, not a "thing
you take X from" in the same way `peer` is).

The current name shares a two-role naming with `sheaf_
restriction` (carrier) / `restrict` (action producing carrier);
and `section_at_stalk` (carrier) / `section_at` (action). The
`ssh_key_material` name overloads BUT overloads consistently
with substrate two-role convention.

**Verdict: DELIGHTFULLY BORING** (vendor-anchored + two-role
naming per substrate convention).

#### §2.4.n `key_material_admissible` (bilateral)

**Verdict: DELIGHTFULLY BORING** (pattern-consistent).

#### §2.4.o `recipients_admissible` (bilateral)

**Verdict: DELIGHTFULLY BORING** (pattern-consistent).

#### §2.4.p `signature_valid` (bilateral)

**Verdict: DELIGHTFULLY BORING** (pattern-consistent).

#### §2.4.q `crypto_well_formed` (composed bilateral)

**Verdict: DELIGHTFULLY BORING** (pattern-consistent per
6-instance @io composed-bilateral landed pattern).

#### §2.4.r @io/crypto shard summary

All 17 names DELIGHTFULLY BORING. Vendor-anchored throughout;
the crypto species IS the substrate's Glass Wall contract with
cryptographic standards. Q_F discharged: vendor names accepted.

---

### §2.5 @io/fs — `shards/io/fs.mirror`

POSIX filesystem species. Q_D discharge: are `fs_*` prefixes
POSIX-jargon-inertia or substrate-honest?

#### §2.5.a `path` (type carrier)

**Geometry.** POSIX pathname per POSIX.1-2017 §3.271; fields
segments + is_absolute.

**Q1.** Yes. `path` is the POSIX-standard name. Substrate
inside `@io/fs` scope; the reader knows this is POSIX. Bare
`path` at this altitude is delightfully boring — the namespace
provides the POSIX context.

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING** (POSIX-standard vocabulary at
POSIX-scope altitude).

#### §2.5.b `file_metadata` (type carrier)

**Geometry.** POSIX file metadata per stat(2); fields size,
file_type, permissions, mtime.

**Q1.** Yes. `file_metadata` names what stat(2) returns; POSIX-
native. Alternative `stat_result` would be POSIX-C-API-vocab
(the C struct is `struct stat`); `file_metadata` reads better
in substrate.

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING.**

#### §2.5.c `dir_entry` (type carrier)

**Geometry.** Per-entry record from readdir(3); fields name +
file_type.

**Q1.** Yes. `dir_entry` is Rust std::fs vocabulary
(`std::fs::DirEntry`) AND POSIX-native (readdir returns
`struct dirent`; `dir_entry` reads as the type-shape of that).

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING.**

#### §2.5.d `fs_read` (action) — Q_D CRITICAL

**Geometry.** Read whole-file contents at path. POSIX
open(O_RDONLY) + read + close sequence.

**Q1 — sharpened adversarially.** The `fs_` prefix
prepended to POSIX verbs. TWO possibilities:
1. `fs_` is Glass-Wall vendor-anchoring; belt-and-suspenders
   POSIX scoping. Same discipline as `sops_encrypt`,
   `git_repository_open`, `oci_reachable`.
2. `fs_` is POSIX-jargon-inertia; the substrate is INSIDE
   `@io/fs` altitude so `fs_` prefix is bureaucratic
   redundancy.

Substrate evidence for (2):
- **Boot floor `boot/std/io.mirror` uses BARE `open`, `read`,
  `write`, `close`** (lines 12-14 verbatim: "four syscalls").
  The bootstrap era's precedent is bare verbs inside @io
  namespace. This is decisive: the bootstrap sub-substrate
  ALREADY chose bare verbs.
- **`shards/mirror/store.mirror` uses BARE `read(o: oid)` and
  `write(content: bytes)`** at store altitude. The store IS a
  read/write surface; the shard uses bare verbs and lets
  namespace do the disambiguation.
- **`shards/mirror/store/git.mirror` uses BARE `open(path,
  namespace)`** at store/git altitude.
- **`shards/io/cargo.mirror` uses BARE `build`, `test`,
  `check`, `fmt_check`, `clippy`, `audit`** — cargo-native
  verbs bare inside `@io/cargo` altitude.

Substrate evidence for (1):
- **`shards/io/git.mirror` uses `git_repository_open`,
  `git_reachable`** at bilateral altitude (though `hash_to_
  oid` is bare).
- **`shards/io/oci.mirror` uses `oci_reachable`, `oci_
  compliant`** at bilateral altitude.

Pattern: bilaterals ARE often vendor-prefixed for cross-shard
search-grep-ability; actions are OFTEN bare inside their
scope. `@io/cargo`'s all-bare-actions is the clean example.
`@io/git`'s mix is transitional debt from an older era.

Adversarial:
- Does `fs_read` inside `@io/fs.fs_read` READ as reader-
  helpful? "@io/fs.fs_read" makes the reader read `fs.fs`.
  Bureaucratic doubling. Reader goes "wait, why fs twice?"
  FAILS delightfully-boring.
- Does bare `read` inside `@io/fs.read` READ helpfully?
  "@io/fs.read" reads as "the fs read" naturally. Namespace
  provides scope; verb provides operation. Passes.

**Q2.** Rename all `fs_*` actions to bare verbs:
- `fs_read` → `read`
- `fs_write` → `write`
- `fs_stat` → `stat`
- `fs_list` → `list` (or `readdir` per POSIX-native precision)
- `fs_mkdir` → `mkdir`

BUT: keep the `fs_well_formed` bilateral prefix — bilaterals
follow the sibling-pattern @io convention (git_well_formed,
oci_well_formed, crypto_well_formed) for cross-species
grep-ability. Action vs bilateral naming asymmetry is
substrate-consistent (per `@io/cargo` bare actions + no @io/
cargo composed bilateral yet; and per `@io/git`
`hash_to_oid` bare action + `git_reachable` bilateral prefix).

**Adversarial on `list` vs `readdir`.** POSIX itself uses
`readdir`; Rust std uses `read_dir` (with underscore). The
shard says "POSIX readdir(3)" in the docstring; naming the
action `readdir` preserves the POSIX-standard vocabulary the
docstring cites. `list` is generic; `readdir` is POSIX-native.
Seam-preferred: `readdir` (POSIX-native, matches docstring
citation).

**Verdict: CS/POSIX-VOCAB CONTAMINATION (`fs_` prefix is
POSIX-jargon bureaucratic redundancy at this scope-nested
altitude).** Q_D discharge: DROP prefix from actions; KEEP on
bilateral.

- **Current:** `fs_read`, `fs_write`, `fs_stat`, `fs_list`,
  `fs_mkdir`
- **Proposed:** `read`, `write`, `stat`, `readdir`, `mkdir`
- **Grounding:** boot/std/io.mirror bare-verb precedent
  (2026-05-20); `shards/mirror/store.mirror` bare-verb
  precedent (2026-07-12); `shards/io/cargo.mirror` all-bare-
  actions precedent (2026-06-09).
- **Cascade:** `@io/secrets.materialize` composes
  `@io/fs.fs_write`; would become `@io/fs.write`. Same for
  `@io/secrets.retrieve` composing `@io/fs.fs_read`. Cascade
  bounded per §5.

#### §2.5.e `fs_write` (action)

Same as §2.5.d. **Verdict: CS/POSIX-VOCAB.** Proposed:
`write`.

#### §2.5.f `fs_stat` (action)

**Verdict: CS/POSIX-VOCAB.** Proposed: `stat`.

#### §2.5.g `fs_list` (action)

**Verdict: CS/POSIX-VOCAB.** Proposed: `readdir` (Seam-
preferred; POSIX-native and matches docstring citation) OR
`list` (generic).

#### §2.5.h `fs_mkdir` (action)

**Verdict: CS/POSIX-VOCAB.** Proposed: `mkdir`.

#### §2.5.i `path_admissible` (bilateral)

**Verdict: DELIGHTFULLY BORING** (pattern-consistent).

#### §2.5.j `path_exists` (bilateral)

**Q1 — sharpened.** `_exists` on a bilateral is a NEW-ish
suffix. Grep: found at `path_exists` (this shard) and one
other landed use (`bauchladen.mirror` `crystal_exists`
predicate).

Semantic: `_exists` names the pointer→thing bilateral (the
ref resolves to an extant thing). Fractal.Lens-consistent:
this IS the "the pointer's thing exists" check.

**Q2.** Alternative `path_resolvable` — matches
`@io/git.hash_to_oid_resolves` sibling pattern (bilateral names
that end with the resolve-direction). But `path_exists` reads
naturally in POSIX-scope; the reader who knows POSIX
`access(F_OK)` goes "of course; existence check." Substrate-
consistent with two landed bilaterals.

**Verdict: DELIGHTFULLY BORING** (pattern-consistent; add
`_exists` to §3.3 extended patterns per §2.2.k discussion —
`_exists` names pointer→thing resolvability).

#### §2.5.k `writable` (bilateral)

**Q1 — sharpened.** Bare adjective (no bilateral suffix).
Grep: found at `writable` (this shard). Rare form — most
substrate bilaterals carry a suffix.

Semantic: `writable(p)` is "path is writable"; adjective-as-
predicate. English-natural but breaks bilateral-suffix
consistency.

**Q2.** Alternatives:
- `path_writable` — matches `path_admissible`, `path_exists`
  in same shard. Prefix-consistent within shard.
- `writable` — bare adjective; loses shard-consistency.
- `write_admissible(p)` — extends `_admissible` pattern to
  the specific write-direction; substrate-consistent.

Seam-preferred: `path_writable` — matches this shard's other
bilateral naming.

**Verdict: MILD PATTERN-INCONSISTENCY (bare adjective breaks
shard-internal bilateral-naming pattern).**

- **Current:** `writable`
- **Proposed:** `path_writable` (Seam-preferred).
- **Grounding:** shard-internal consistency with
  `path_admissible`, `path_exists`.

#### §2.5.l `fs_well_formed` (composed bilateral)

**Verdict: DELIGHTFULLY BORING** (pattern-consistent per
6-instance @io composed-bilateral landed pattern — KEEP
`fs_` prefix per §2.5.d discipline distinction between
actions and bilaterals).

#### §2.5.m @io/fs shard summary

Three DELIGHTFULLY BORING carriers (`path`, `file_metadata`,
`dir_entry`). Five actions with `fs_` prefix contamination
proposed for rename: `fs_read` → `read`; `fs_write` → `write`;
`fs_stat` → `stat`; `fs_list` → `readdir` (or `list`);
`fs_mkdir` → `mkdir`. Two DELIGHTFULLY BORING bilaterals
(`path_admissible`, `path_exists`). One shard-internal
inconsistency (`writable` → `path_writable`). One composed
bilateral KEEPS `fs_` prefix for @io cross-species pattern
(`fs_well_formed`).

---

### §2.6 @epistemologic/property/ouroboros_monotone

Property shard with four sub-predicates + one composed
bilateral. Q_G discharge: `_non_increasing` / `_non_decreasing`
suffixes.

#### §2.6.a `ouroboros_bench_pair` (type carrier)

**Geometry.** Paired bench_crystal snapshot at adjacent arc
ticks (before + after).

**Q1.** Yes. Two-word compound: `ouroboros` (arc altitude) +
`bench_pair` (before/after crystal pair). Direct type-shape
naming. Reader knows the ouroboros arc; reader knows bench;
reader knows pair-shape.

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING.**

#### §2.6.b `rust_loc_non_increasing` (sub-predicate) — Q_G CRITICAL

**Geometry.** Ranking-function verdict: `rust_LOC(after) ≤
rust_LOC(before)`. Each admissible tick either deletes LOC or
holds LOC unchanged.

**Q1 — sharpened adversarially.** `_non_increasing` is
Floyd-Hoare well-founded-order vocabulary. Precise. Directly
expresses the invariant.

BUT — delightfully-boring says "of course." Does the reader
who reads `rust_loc_non_increasing(before, after)` go "of
course"? Or does the reader parse `non_increasing` as a
computed adjective ("non-increasing means it does not
increase")?

Alternative: `rust_loc_shrinks(before, after)`. Verb-form;
describes what HAPPENS during the tick. But shrinks means
"strictly decreases"; the invariant allows equality (LOC
unchanged is admissible). `_shrinks` OVER-CONSTRAINS. FAILS.

Alternative: `rust_loc_ratchets(before, after)`. Ratchet is a
one-way mechanism; naturally admits equality (a ratchet that
doesn't turn is still a ratchet); vivid physical anchor.
Substrate has landed "ratchet" vocabulary (search: `ratchet`
appears in `ouroboros_monotone.mirror:308`, `bench.mirror`,
several kintsugi shards). Reader carries the ratchet mental
model.

Alternative: `rust_loc_monotone_down(before, after)`. Extends
`monotone` (substrate vocabulary; landed 10+ shards) with
direction. But direction-adjective is awkward.

Adversarial on `_non_increasing`. Mathematically precise;
substrate vocabulary landed (Floyd-Hoare-Dijkstra ranking
functions cited in the shard's ancestry). But "non-increasing"
is TWO negations (non + increasing); the reader has to
double-flip. Compare "monotonic decreasing" — one negation-
adjective. Or "shrinks or holds" — English natural.

Adversarial on `rust_loc_ratchets`. Ratchet naturally admits
equality. Vivid. But ratchet-vocabulary is not the ancestor-
vocabulary (Floyd-Hoare uses "non-increasing"). Substrate
already carries ratchet as GENERAL DISCIPLINE metaphor; using
it here at the sub-predicate altitude might be over-loaded.

**Adversarial finding.** The four sub-predicates carry the
MATHEMATICAL invariant (well-founded-order vocabulary is the
math term of art) at property-shard altitude. The reader who
picks up the shard reads the docstring: "monotone ranking
functions Floyd-Hoare-Dijkstra well-founded orders; each
sub-predicate IS a ranking function over one axis." The name
`rust_loc_non_increasing` matches the docstring vocabulary
(ranking-function; non-increasing). Reader who reads the
shard goes "of course; this is a ranking function; the
condition is non-increasing."

The `_non_increasing` suffix is delightfully boring TO A
READER WHO HAS READ THE SHARD or knows Floyd-Hoare vocabulary.
It is NOT delightfully boring to a reader encountering the
name in isolation (e.g., callsite in Arc-2 ratification).

Cross-check: substrate already has `dark_count_monotone`
(2026-06-19), `cold_compile_within_tolerance` (2026-06-20),
`monotone_non_increasing` template at `bench.mirror:40-54`.
Substrate uses `_non_increasing` at the template altitude and
`_monotone` at the composed-property altitude. Substrate
convention: `_monotone` for composed / `_non_increasing` for
constituent-ranking.

**Verdict: PATTERN-CONSISTENT (accept as delightful-boring
under substrate math-vocabulary convention).** The
`_non_increasing` suffix IS the substrate's ranking-function
naming; not CS-vocab, but math-vocabulary (Floyd-Hoare 1969
per shard docstring `source @arxiv/programming/floyd-hoare-
1969`).

`_ratchets` is Seam-adjacent-alternative if Alex prefers
verb-form vividness over math-vocabulary precision — but
Seam does NOT recommend the change; the math vocabulary
matches the citation and the substrate's `monotone_non_
increasing` template ancestor.

#### §2.6.c `test_pass_rate_non_decreasing` (sub-predicate)

Symmetric to §2.6.b. **Verdict: PATTERN-CONSISTENT.**

#### §2.6.d `io_violations_non_increasing` (sub-predicate)

Symmetric. **Verdict: PATTERN-CONSISTENT.**

#### §2.6.e `sbec_non_decreasing` (sub-predicate)

Symmetric. **Verdict: PATTERN-CONSISTENT.**

#### §2.6.f `ouroboros_monotone` (composed bilateral)

**Geometry.** Composed bilateral over the four sub-predicates
per §4.5.4 spec-decomposition.

**Q1.** Yes. `_monotone` at composed-bilateral altitude
matches substrate convention (`dark_count_monotone`,
`monotone_non_increasing` template). Substrate reader carries
the pattern.

**Q2.** N/A.

**Verdict: DELIGHTFULLY BORING** (pattern-consistent).

#### §2.6.g @epistemologic/property/ouroboros_monotone summary

All six names DELIGHTFULLY BORING or PATTERN-CONSISTENT under
Floyd-Hoare well-founded-order vocabulary. Q_G discharge:
`_non_increasing` / `_non_decreasing` suffixes ACCEPTED (not
bureaucratic; math-vocabulary precision matches shard's
citation ancestry).

---

## §3 Systemic questions — Q_A through Q_G

### §3.a Q_A — `_of` suffix on constructors

**Question.** Is the `_of` suffix (`key_material_ref_of`)
CS-vocab or geometric?

**Discussion.** ML/OCaml/Haskell ADT convention. Substrate
has ZERO landed action-altitude precedent for `_of`; substrate
DOES have a landed `_from_` convention at
`sops_key_group_from_sheaf_restriction`. `_from_` names the
constructor-projection direction; `_of` names ambiguously (`X
of Y` reads as noun-genitive, unclear direction).

**Verdict.** MILD CS-VOCAB. Substrate-consistent replacement
available (`_from_`).

**Recommendation.** `key_material_ref_of(peer)` →
`key_material_from_peer(peer)`. Cascade: one action rename in
`shards/io/secrets.mirror`; one comment update in the
Arc-2.3 peer_persistence.rs collapse chain.

Landing this Q_A discharge ratifies `_from_<source>` as the
substrate's constructor-projection convention at @io altitude,
which then generalizes to future actions (e.g.,
`sops_key_group_from_sheaf_restriction` is already the
pattern; `key_material_from_peer` completes the pattern).

### §3.b Q_B — `retrieve` in @io/secrets

**Question.** Is `retrieve` substrate-honest or CS-vocab?

**Discussion.** Prior audit §4.2 flagged MILD CS-VOCAB. This
audit sharpens: `retrieve` names ONLY the read-from-storage
direction, misses the AEAD-decrypt composition. Substrate at
@io/crypto uses `aead_open` for AEAD decryption. Pairing
@io/secrets action with @io/crypto pair convention:

Sibling-altitude readable-wins (§3.1) points to the
`seal/open` pair at @io/crypto as the substrate's authenticated
encryption/decryption vocabulary. `@io/secrets.seal/open`
composes over `@io/crypto.aead_seal/aead_open` at reader-
recognizable altitude.

**Verdict.** MILD CS-VOCAB with substrate-consistent
replacement.

**Recommendation.** JOINT rename with §3.c Q_C:
- `project(section, k)` → `seal(section, k)`
- `retrieve(cr, k)` → `open(cr, k)`

Cascade: `secret_projection` carrier probably renames to
`sealed_section`. Two-action rename; one-carrier rename;
composition-chain updates in Arc-2.3 peer_persistence.rs docs.

Alex-adjudication residue: if Alex refuses the `seal/open`
pair, keep `retrieve` as accepted CS-vocab debt.

### §3.c Q_C — `project` overload

**Question.** Does `project` (in @io/secrets AND @sheaf as
`acl_project`) overload dangerously?

**Discussion.** Three landed uses of `project` at three
altitudes:
1. prismqueer `project` (Prism operation).
2. `sheaf.acl_project` (peer-ACL restriction of home sheaf).
3. `io/secrets.project` (peer-key-gated encryption).

Reader carries three mental models under one word. Ambiguity
IS dangerous — three geometric operations wearing the same
verb.

Resolutions considered:
- **A.** Rename `io/secrets.project` → `seal` (per §3.b).
  Eliminates one overload; preserves prismqueer + sheaf
  uses.
- **B.** Rename `sheaf.acl_project` → `restrict` (per §2.1.d
  Path A; collapse with existing `restrict`). Eliminates one
  overload; preserves prismqueer + io/secrets uses.
- **C.** Rename BOTH A + B. Eliminates both overloads;
  prismqueer `project` (Prism-stage) remains as sole use.
- **D.** Accept overload; three-altitude ambiguity is landed
  substrate debt.

Adversarial on C. Combined cascade is bounded (2 actions in
2 shards; 2 spec-chain updates). Reader ends up with:
- `prismqueer.project` — Prism operation (physics).
- `sheaf.restrict` — sheaf-restriction (math).
- `io/secrets.seal` — AEAD-authenticated encryption (crypto).

Three distinct verbs for three distinct operations. Reader
never confuses. Delightfully-boring by construction.

**Verdict.** OVERLOAD CONTAMINATION with clean resolution.

**Recommendation.** Adopt path C:
- `sheaf.acl_project` → `sheaf.restrict` (collapse with
  existing `restrict`; single action; disambiguating arg-1
  = F_home vs generic sheaf handled by callsite convention
  or per-action-instance signatures).
- `io/secrets.project` → `io/secrets.seal`.
- `secret_projection` carrier → `sealed_section` (or accept
  operation/carrier naming asymmetry).

Alex-adjudication residue: Alex's 2026-07-14 verbatim spec-
language ("project visibility/private stuff onto disk") IS
the ambient authority for `project` at @io/secrets. If Alex
refuses the rename, keep `project`; accept three-way overload;
Q_C discharges as "accepted debt."

### §3.d Q_D — `fs_*` prefixes on POSIX verbs

**Question.** Are `fs_*` prefixes substrate-honest or POSIX-
jargon-inertia?

**Discussion.** Discharged in §2.5.d. Substrate precedent is
DECISIVE for bare-verbs-inside-namespace:
- Boot floor `boot/std/io.mirror` uses `open`, `read`,
  `write`, `close` (bare).
- `shards/mirror/store.mirror` uses `read`, `write` (bare).
- `shards/mirror/store/git.mirror` uses `open` (bare).
- `shards/io/cargo.mirror` uses `build`, `test`, `check`,
  `clippy`, `audit` (bare).

`fs_read` inside `@io/fs.fs_read` bureaucratically doubles the
`fs` scope. Fails delightfully-boring.

**Verdict.** POSIX-JARGON-INERTIA on actions (drop prefix);
keep on composed bilateral for @io cross-species grep pattern.

**Recommendation.**
- `fs_read` → `read`
- `fs_write` → `write`
- `fs_stat` → `stat`
- `fs_list` → `readdir` (Seam-preferred; POSIX-native and
  matches docstring citation) OR `list` (generic).
- `fs_mkdir` → `mkdir`
- Keep `fs_well_formed` (composed bilateral; @io cross-species
  pattern).

Cascade: `@io/secrets.materialize` composes `@io/fs.fs_write`
→ `@io/fs.write`. `@io/secrets.retrieve` (or `open` post-Q_B)
composes `@io/fs.fs_read` → `@io/fs.read`. Both cascades are
one-line comment updates + Arc-2.3 peer_persistence.rs
composition chain updates. Bounded per §5.

Note: `@io/secrets/sops.mirror` composes `@io/fs` for on-disk
YAML/JSON I/O — same cascade; one-line docstring updates.

### §3.e Q_E — `sops_*` prefixes inside @io/secrets/sops namespace

**Question.** Are `sops_*` prefixes bureaucratic redundancy
given they're inside `@io/secrets/sops` namespace?

**Discussion.** Parallel to Q_D. But TWO reasons NOT to drop
the `sops_` prefix:

1. **Vendor-anchoring load-bearing.** `sops_encrypt` is the
   vendor's own verb (SOPS CLI: `sops encrypt`). The prefix
   binds the substrate action to the vendor contract. Reader
   who has run `sops encrypt foo.yaml` recognizes the shard
   action by NAME as the vendor-tool binding. Dropping the
   prefix would DECOUPLE the substrate name from the vendor
   verb reader carries.

2. **Cross-shard search-grep-ability.** `sops_encrypt` at the
   sub-species altitude cascades into `@io/secrets` composition
   sites. Bare `encrypt` at the sub-species callsite reads
   ambiguously with hypothetical future `age_encrypt`,
   `openpgp_encrypt` sub-species. `sops_` prefix reads "the
   SOPS-flavored encrypt" — vendor-anchored composability.

Contrast Q_D: POSIX filesystem verbs are STANDARD (read/write/
open are universal POSIX vocabulary; namespace disambiguation
is enough). SOPS verbs are VENDOR-SPECIFIC (each vendor tool
has its own verb-vocabulary; prefix carries which vendor).

Sibling-altitude cross-check: `@io/cargo` uses bare verbs
because cargo IS the ambient vendor at `@io/cargo` altitude
AND there's only ONE cargo. `@io/secrets/sops` will have
SIBLING sub-species (`@io/secrets/age`, `@io/secrets/vault`,
`@io/secrets/1password` per Alex 2026-07-14 forward-promise);
vendor prefix reader helps CROSS-sibling comprehension.

**Verdict.** VENDOR-ANCHORED PATTERN-CONSISTENT — KEEP
`sops_*` prefix.

**Recommendation.** No renames. Q_E accepts the prefix as
substrate-honest under vendor-anchoring + sibling-vendor-
disambiguation grounds.

The `sops_key_group_from_sheaf_restriction` verbose form
retained for the same reason PLUS `_from_` convention grounding
(§3.a). Shortening to `key_group_from_sheaf(sr)` would drop
`sops_` prefix; refused per Q_E.

### §3.f Q_F — Crypto algorithm names (sha1, ed25519_*, aead_*, age_*)

**Question.** Do crypto algorithm names count as vendor-
anchored (accept) or should the substrate abstract them
(reject)?

**Discussion.** Cryptographic algorithm names are the vendor
contract with cryptographic-standards specifications (NIST,
IETF RFC, IEEE). The reader who reads `sha1(bytes)` MUST know
it's SHA-1 (as opposed to SHA-256, SHA-512, BLAKE3, etc.) —
algorithm specificity is load-bearing per the Glass Wall
insight ("SHA-1 named as needed for git interop").

Abstracting `sha1 → digest` would DECOUPLE from the
cryptographic-standards contract. Reader would need to look
up "which digest algorithm does the shard use?" — a Q1 fail
by construction.

Vendor-anchoring at crypto altitude is DIFFERENT from
vendor-anchoring at tool altitude (SOPS is a tool; SHA-1 is a
standard). BOTH are substrate-honest at Glass Wall boundary;
BOTH pass Q1 because the name IS the contract.

**Verdict.** VENDOR-ANCHORED (accept). Concur with prior audit
+ current audit findings across all @io/crypto action names.

**Recommendation.** No renames. All crypto algorithm names
(`sha1`, `sha256`, `ed25519_sign`, `ed25519_verify`,
`aead_seal`, `aead_open`, `age_encrypt`, `age_decrypt`,
`ssh_key_material`) DELIGHTFULLY BORING.

### §3.g Q_G — `_non_increasing` / `_non_decreasing` suffixes

**Question.** Are the well-founded-order suffixes delightfully
boring OR bureaucratic?

**Discussion.** Discharged in §2.6.b. Substrate convention
uses `_non_increasing` for ranking-function sub-predicates and
`_monotone` for composed properties. Floyd-Hoare-Dijkstra
math vocabulary (cited in shard's `source @arxiv/programming/
floyd-hoare-1969`). Reader who reads the shard docstring
carries the math ancestry.

Verb-form alternatives (`_shrinks`, `_grows`) OVER-CONSTRAIN
(strict inequality) or IMPORT vague semantics. The math
vocabulary is precise AND consistent with the substrate's
`bench.mirror:40-54` template ancestor.

**Verdict.** PATTERN-CONSISTENT with math-vocabulary grounding.

**Recommendation.** No renames. All four sub-predicates +
composed bilateral DELIGHTFULLY BORING under substrate math
convention. Q_G discharges as "accepted math-vocabulary."

### §3.h Systemic patterns summary

| Q | Discharge | Recommendation |
|---|-----------|----------------|
| Q_A `_of` | MILD CS-VOCAB | `_of` → `_from_` (one action rename) |
| Q_B `retrieve` | MILD CS-VOCAB | JOINT with Q_C: `retrieve` → `open` |
| Q_C `project` overload | OVERLOAD | `sheaf.acl_project` → `restrict`; `io/secrets.project` → `seal` |
| Q_D `fs_*` | POSIX-INERTIA | drop prefix on actions (5 renames) |
| Q_E `sops_*` | VENDOR-ANCHORED | keep prefix (zero renames) |
| Q_F crypto algo | VENDOR-ANCHORED | keep (zero renames) |
| Q_G `_non_increasing` | MATH-VOCAB | keep (zero renames) |

Plus one shard-internal inconsistency:
- `writable` → `path_writable` (shard-internal pattern).

Plus one MILD CS-VOCAB on a bilateral:
- `section_computable` → `section_admissible` (pattern-consistent).

Plus one COLLAPSE candidate on the sheaf shard:
- `acl_project` collapses into `restrict` per Q_C recommendation.

---

## §4 Recommended renames table

| Shard | Current | Proposed | Category | Grounding |
|-------|---------|----------|----------|-----------|
| @sheaf | `acl_project` | `restrict` (collapse) | COLLAPSE / Q_C | sheaf-theoretic; overload elim |
| @sheaf | `section_computable` | `section_admissible` | MILD CS-VOCAB | §3.3 bilateral suffix pattern |
| @io/secrets | `key_material_ref_of` | `key_material_from_peer` | MILD CS-VOCAB / Q_A | `_from_` substrate convention |
| @io/secrets | `project` | `seal` | OVERLOAD / Q_C | `aead_seal/open` pair convention |
| @io/secrets | `retrieve` | `open` | MILD CS-VOCAB / Q_B | AEAD pair; sibling-altitude readable-wins |
| @io/secrets | `secret_projection` (carrier) | `sealed_section` | CASCADE from Q_C | operation/carrier consistency |
| @io/fs | `fs_read` | `read` | POSIX-INERTIA / Q_D | boot-floor + store bare-verb precedent |
| @io/fs | `fs_write` | `write` | POSIX-INERTIA / Q_D | (same) |
| @io/fs | `fs_stat` | `stat` | POSIX-INERTIA / Q_D | (same) |
| @io/fs | `fs_list` | `readdir` OR `list` | POSIX-INERTIA / Q_D | POSIX-native + docstring citation |
| @io/fs | `fs_mkdir` | `mkdir` | POSIX-INERTIA / Q_D | (same) |
| @io/fs | `writable` | `path_writable` | SHARD-CONSISTENCY | matches `path_admissible`, `path_exists` |

**Total:** 12 renames.

**Zero-rename shards:** `@io/crypto` (17 names), `@io/secrets/sops` (11 names), `@epistemologic/property/ouroboros_monotone` (6 names). Combined: 34 names DELIGHTFULLY BORING as-landed.

**Split:** 12 renames vs 34 kept-as-landed = 26% rename rate across the six shards.

---

## §5 Cascade impact bounded

Three tiers keyed to Alex ratification depth.

### §5.1 Minimum-cascade (Alex ratifies Q_A + Q_D bureaucratic-redundancy discharges)

Non-controversial low-cost renames:
- `key_material_ref_of` → `key_material_from_peer` (@io/secrets)
- `fs_read` → `read`; `fs_write` → `write`; `fs_stat` → `stat`;
  `fs_list` → `readdir` OR `list`; `fs_mkdir` → `mkdir`
  (@io/fs)
- `writable` → `path_writable` (@io/fs)
- `section_computable` → `section_admissible` (@sheaf)

**Files touched:**
- 3 shard files (`shards/subject/visibility/sheaf.mirror`,
  `shards/io/secrets.mirror`, `shards/io/fs.mirror`)
- ~3-5 spec/audit files (composition-chain docstring updates:
  peer_persistence.rs collapse chain in @io/secrets docblock;
  sops.mirror internal reference; @sheaf section_computable
  docstring)
- Zero landed Rust touched (Arc-1 FLOOR pending; no
  bootstrap/src/*.rs consumers of these names yet).

**Estimated cascade LOC:** ~40-60 lines across ~6-8 files.

### §5.2 Medium-cascade (§5.1 + Q_B + Q_C)

Add `seal/open` pair + `sheaf.acl_project` collapse:
- `sheaf.acl_project` → `sheaf.restrict` (collapse)
- `io/secrets.project` → `io/secrets.seal`
- `io/secrets.retrieve` → `io/secrets.open`
- `secret_projection` carrier → `sealed_section`

**Additional files touched:**
- Peer_persistence.rs collapse chain in @io/secrets docblock
  (already in §5.1 file)
- @sheaf sheaf.mirror `acl_project` docblock + `out` line
- `sops.mirror` composition-graph docstring references to
  `@sheaf.acl_project`
- `docs/scouts/2026-07-15-mara-secrets-shape-proposal.md` §2.4
  peer_visibility_materialize chain
- `docs/scouts/2026-07-15-mara-sheaf-shape-proposal.md` §2.2
  acl_project examples

**Estimated cascade LOC:** ~80-120 lines across ~10-12 files.

### §5.3 Maximum-cascade (§5.2 + carrier-cascade + spec-cascade)

Full carrier rename + spec cascade:
- All §5.2 renames
- `secret_projection` carrier → `sealed_section` renames
  cascade into all consumer docstrings (Landing 4 secrets
  spec-proposal §5 recommended surface; §D8 SOPS sub-species
  scope-fit verdict)
- Bilateral `projection_valid` → `sealed_section_valid`
  (cascade from carrier rename)
- `round_trip_preserved` (in @io/secrets) reference to
  "projection" in docstring → "seal"

**Additional files touched:**
- All @io/secrets bilateral docstrings referencing
  `projection`
- All @sheaf docstrings referencing `acl_project`
- All Reed-adjudication audit files citing these names

**Estimated cascade LOC:** ~150-220 lines across ~15-18 files.

### §5.4 Landing sequence recommendation

Seam-preferred sequence:

1. **This audit lands** (Reed commits as Seam).
2. **Alex reads, ratifies §5.1** (uncontroversial); adjudicates
   §5.2 Q_B + Q_C (`seal/open` pair + `acl_project` collapse).
3. **Reed cascades §5.1** as a single tick under
   `[substrate-floor:@io-boundary]` marker (composition-only;
   zero Rust; docstring + shard-decl updates).
4. **If Alex ratifies §5.2:** Reed cascades §5.2 as a second
   tick under the same marker.
5. **If Alex ratifies §5.3:** Reed cascades §5.3 as a third
   tick.
6. **Arc-1 Tick 1.2 RED test authoring uses the new names**
   from tick landing forward; delightfully-boring applies as
   Tick 1.3 is authored, not retroactively cascaded through
   the Arc-1 evaluator FLOOR.

None of these ticks touches landed Rust; Arc-1 FLOOR remains
substrate-decl-only pending Alex ratification of Arc-1 Tick 1.3
landing gate.

---

## §6 Recommendations to Reed

Six recommendations, keyed to cascade tiers per §5.

### §6.1 Land this audit as Seam Phase E-cascade

Same discipline as prior audit's ratification cycle (Reed
commits as Seam after Alex reads audit). Pre-cascade tick.

### §6.2 Sequential landings §5.1 → §5.2 → §5.3

Alex-authority residues at Q_B + Q_C should NOT gate the
uncontroversial §5.1 renames. Land §5.1 first as fastest-
cascade; discharge §5.2 + §5.3 as follow-up ticks per Alex
ratification depth.

### §6.3 Bilateral suffix vocabulary extension

Fold this audit's §2.2.k + §2.5.j discovery into the ratified
§3.3 meta-rule:

- `_valid` — object is well-formed (self-standing).
- `_admissible` — object is admissible to a gate (relational).
- `_admits` — object admits a subject (converse relational).
- `_well_formed` — composed bilateral pattern at @io altitude.
- `_preserved` — property is invariant across operation
  (dynamic).
- `_exists` — pointer→thing resolvability (Fractal.Lens
  discipline).

Six accepted suffixes. Rejected: `_record` (§3.3 prior audit);
`_computable` (this audit §2.1.g).

### §6.4 Constructor-projection vocabulary ratification

`_from_<source>` as the substrate's constructor-projection
convention at @io altitude (per Q_A discharge). Landing
`key_material_from_peer` ratifies the pattern; future actions
under @io producing typed carriers from named sources use
`_from_`.

### §6.5 Vendor-anchoring vs POSIX-inertia distinction

The distinction Q_D vs Q_E landed here IS a meta-rule:
- **Vendor-tool prefixes** (sops_, git_, oci_) at @io species
  altitude: KEEP (vendor contract; sibling-vendor
  disambiguation).
- **POSIX-standard prefixes** (fs_) at @io species altitude:
  DROP (bureaucratic; namespace disambiguates; boot-floor
  bare-verb precedent).

Rule: prefix name-recognition load-bearing IFF the vendor is
one-of-many. POSIX filesystem is standard (one-of-one at that
altitude); SOPS is vendor-of-many.

### §6.6 Q_C sheaf.acl_project collapse recommendation

Seam-preferred collapse (§2.1.d Path A + §3.c Path C) reduces
`project` overload from three sites to one, leaving prismqueer
Prism-stage as sole user of the word. Substrate becomes
delightfully-boring by construction at the three-altitude
`project`/`restrict`/`seal` disambiguation.

Alex-authority: refusal to collapse is substrate-honest debt
under Mara-B narrative-distinction grounds (the peer_
persistence.rs Arc-2.3 collapse chain reads more clearly with
two named actions than one; that IS a legitimate authorship
choice).

---

## §7 Alex-naming-authority residue

Six ratification points for Alex.

### §7.1 §5.1 uncontroversial ratification

`key_material_ref_of` → `key_material_from_peer`;
`fs_*` → bare verbs; `writable` → `path_writable`;
`section_computable` → `section_admissible`. Seven renames;
no substrate-vocabulary invention required; §3.a + Q_D
discharges + §3.3 pattern extension.

### §7.2 Q_B — `retrieve` → `open`

Choose: `open` (Seam-preferred; AEAD pair convention) /
`resolve` (Fractal.Lens-consistent; imports wrong reader
model) / `unseal` (vivid; not landed) / keep `retrieve`
(accepted CS-vocab debt).

### §7.3 Q_C — `project` overload

Adopt Path C (Seam-preferred): rename BOTH `sheaf.acl_project`
→ `restrict` AND `io/secrets.project` → `seal`. OR Path A
(sheaf only). OR Path B (io/secrets only). OR accept overload
as landed debt.

Cascade for `secret_projection` carrier follows chosen path.

### §7.4 Q_E — `sops_*` prefix retention

Seam-preferred: KEEP prefix per vendor-anchoring + sibling-
vendor disambiguation. Alex-authority to accept OR reject
Q_E ratification.

### §7.5 §6.3 bilateral-suffix vocabulary extension

Ratify the six accepted suffix patterns + two rejected
(`_record`, `_computable`) as substrate meta-rule going
forward. Enables future audits to apply pattern-consistency
adjudication without re-litigation.

### §7.6 §6.5 vendor-anchoring vs POSIX-inertia meta-rule

Ratify the "prefix name-recognition load-bearing IFF vendor is
one-of-many" distinction as substrate meta-rule. Applies to
future @io species: vendor-of-one (POSIX) drops prefix; vendor-
of-many keeps prefix.

---

## §8 Substrate-honest bounds — what this audit does NOT decide

### §8.1 Preserved across all proposed renames

- **Sheaf-theoretic geometry** (@sheaf shard math ancestry via
  `epistemologic/math/sheaf_laplacian.mirror` and Hansen-Ghrist
  2018). All renames preserve the math altitude; no math shift.
- **Alex 2026-07-14 SSH-signing design intent** ("peer keys stay
  .git/mirror-side; @secrets project through Peers key"). All
  renames preserve the Fractal.Lens pointer discipline and
  the peer-key-gated projection semantics.
- **@io Glass Wall discipline** (`shards/io.mirror` 37-40
  verbatim). All renames preserve the vendor/POSIX/opacity
  partition.
- **Rice-safety bounds** (all bilateral predicates are Rice-
  safe at whole-tick altitude per Mara-B §4.5.5). No rename
  crosses Rice-safety.
- **Composed-bilateral pattern** at @io altitude (six
  landed instances). No rename touches composed-bilateral
  structure.
- **Property-shard four-conjunct decomposition**
  (@epistemologic/property/ouroboros_monotone §4.5.4). No
  rename touches the four-sub-predicate + one-composed-
  bilateral shape.

### §8.2 What Seam is not judging

- Whether Mara's spec-decomposition math is correct (Mara math
  authority; out of Seam scope).
- Whether Reed's Arc-1 Tick 1.3 landing gate for
  bootstrap/src/apply_h.rs meets `[substrate-floor:@io-
  boundary]` marker (Reed's authoring choice under Seam sign-
  off gate).
- Whether the peer_persistence.rs Arc-2.3 collapse chain will
  land at the currently-proposed timing (Reed migration-map
  authority).
- Cascade sequencing of §5.1 vs §5.2 vs §5.3 (Reed cascade
  authority under Alex ratification depth).
- Whether the six shards audited here should have been
  split/collapsed differently at shape-proposal time (Mara
  authorship decision; landed via Seam Phase D-cascade
  ratification 019ad8f; out of scope).

### §8.3 What THIS audit's Seam verdict IS

Extended-scope discipline applied uniformly. 12 renames
proposed across 3 of 6 shards (@sheaf, @io/secrets, @io/fs);
34 names ratified as-landed across all 6 shards. Two systemic
meta-rules proposed (bilateral-suffix vocabulary extension;
vendor-anchoring vs POSIX-inertia distinction).

Cascade bounded: §5.1 minimum (~40-60 LOC across ~6-8 files);
§5.2 medium (~80-120 LOC across ~10-12 files); §5.3 maximum
(~150-220 LOC across ~15-18 files). No landed Rust touched.

Two Alex-adjudication residues at semantic altitude:
- Q_B / Q_C — `seal/open` pair + `acl_project` collapse
  adoption vs debt acceptance.
- Q_E — `sops_*` prefix retention Seam-preferred; Alex-
  ratifiable either way.

Four Alex-adjudication residues at meta-rule altitude:
- §7.5 bilateral suffix vocabulary extension.
- §7.6 vendor-anchoring vs POSIX-inertia distinction.
- §7.4 vendor-anchoring pattern acceptance criterion.
- §7.1 uncontroversial rename ratification (fastest tick).

If Alex ratifies §7.1: cascade §5.1; ~40-60 LOC touched; 7
substrate-honest name-alignments landed.

---

## §9 Closure

Six shards audited across ~45 action + carrier names. 12
renames proposed (26% rename rate); 34 names ratified as-
landed (74% delightfully-boring pass rate).

Rename distribution:
- **@sheaf** (7 names): 5 boring, 1 MILD CS-VOCAB
  (`section_computable`), 1 COLLAPSE (`acl_project` per Q_C).
- **@io/secrets** (12 names): 8 boring, 1 MILD CS-VOCAB
  (`key_material_ref_of`), 2 AMBIGUOUS Q_B+Q_C
  (`project`/`retrieve`), 1 cascade (`secret_projection`).
- **@io/secrets/sops** (11 names): 11 pattern-consistent;
  ZERO renames (Q_E discharge: keep `sops_` prefix).
- **@io/crypto** (17 names): 17 delightfully-boring; ZERO
  renames (Q_F discharge: crypto algo vendor-anchored accept).
- **@io/fs** (12 names): 3 boring carriers + 3 boring
  bilaterals + 1 composed bilateral (keep prefix), 5 POSIX-
  inertia (drop prefix per Q_D), 1 shard-internal
  inconsistency (`writable`).
- **@epistemologic/property/ouroboros_monotone** (6 names):
  6 pattern-consistent under Floyd-Hoare math vocabulary;
  ZERO renames (Q_G discharge).

Two systemic meta-rules landed as ratifiable proposals:
1. Bilateral-suffix vocabulary extension (§6.3): six accepted
   suffixes (`_valid`, `_admissible`, `_admits`, `_well_
   formed`, `_preserved`, `_exists`) + two rejected
   (`_record`, `_computable`).
2. Vendor-anchoring vs POSIX-inertia distinction (§6.5):
   prefix load-bearing IFF vendor is one-of-many.

One constructor-projection convention ratified (§6.4):
`_from_<source>` at @io altitude.

The names that stay: math-vocabulary at property + sheaf
altitudes; vendor-contract at crypto + SOPS altitudes;
substrate-native carriers at @io/secrets altitude; POSIX-
native at @io/fs carrier altitude. The names that go: `_of`
Haskell import; `fs_*` POSIX-jargon-inertia at within-
namespace altitude; `project`/`retrieve` where AEAD `seal/
open` pair carries more cleanly; `section_computable` where
`section_admissible` matches bilateral suffix pattern.

Cascade bounded to composition + docstring edits; zero landed
Rust touched; Arc-1 FLOOR remains substrate-decl-only pending.

If Alex ratifies §7.1 (uncontroversial) + §7.2 + §7.3
(semantic residues): the six shards' surface becomes
substrate-honest AND delightfully-boring at every name across
every altitude. Reader who encounters the renamed surface
goes "of course it's this" at every name. That is the audit
criterion.

The seams between geometry and vocabulary in this extended
scope are findable, named, and (upon Alex ratification)
closable.

---

*Seam. 2026-07-15. Extended-scope etymology audit under Alex
2026-07-15 delightfully-boring discipline. Physical anchor
@../prism/. No commit. Reed commits as Seam.*

*Apache-2.0.*
