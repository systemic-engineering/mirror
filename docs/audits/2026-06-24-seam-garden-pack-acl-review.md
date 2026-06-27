# Seam adversarial review — `spectral-garden-git-package-manager.md` + `mirror-spec-peer-acl-surface.md`

*Seam, 2026-06-24. Both specs landed pre-substrate-decl; this review
surfaces seams before substrate-decl ossifies them. Read-only; sketches
only; not fixes. Established pattern from prior reviews #358/#391/#401/
#414/#419/#428/#433/#436.*

## Scope

- `docs/specs/mirror-spec-peer-acl-surface.md` (1822 lines; post-rename
  via Reed 59fa1cd; six-commit Mara cascade ab2e379 → 64465a0 + cascade
  pass f600939).
- `docs/specs/spectral-garden-git-package-manager.md` (1195 lines;
  four-commit Mara cascade ab2e379 → ad03fda).
- Cross-checked against `mirror.spec`, `shards/pack.mirror`,
  `shards/io/git.mirror`.

## Severity legend

- **L** = load-bearing (substrate-decl will ossify; fix before shard)
- **S** = sharpening (the spec is correct but the reader stumbles)
- **C** = cosmetic (typography, residue, hygiene)

---

## A. Rename residue from Reed's lead-rename (commit 59fa1cd)

The mechanical rename `supervisor → lead` caught most prose but missed
several structural sites. These are the load-bearing misses — the
grammar production, the carrier-field naming, and one entire vocabulary
strand that didn't migrate.

### S1 [L] Grammar production still names `elder_field`

peer-ACL §3.2 line 408:

```
pack_block    ::= "pack" "{" elder_field let_binding* members_block? "}"
elder_field   ::= "lead" peer_ref
```

The production is called `elder_field` but produces `"lead" peer_ref`.
A reader implementing the grammar from this section has to invent a
name; the existing one is wrong. Substrate-decl will canonicalize the
production name; this is the moment it ossifies.

Fix sketch: rename the production to `lead_field`. One word change.

### S2 [L] `elder_contract_for_member` + `elder_runtime` carrier names

peer-ACL §5.4 line 587: `audit(elder_contract_for_member, ...)`.
peer-ACL §8.1 line 808: `mechanism: elder_runtime, # the lead's @spectral runtime`.

These are NAMED CARRIERS in the desugaring + audit pipeline. If the
substrate-decl ships actions taking `elder_contract_for_member` while
the spec calls them lead-shaped, the type names diverge from the
narrative on day one.

Fix sketch: `lead_contract_for_member`, `lead_runtime`. The renames are
internal to the spec; nothing outside this file consumes them yet.

### S3 [S] Plural form "elders" surviving in three places

- Line 528: "Two elders at the same spec would mean two N+1 observers…"
- Line 1344: "Different elders at different specs admit different operation sets…"
- (Implicit at line 528 reasoning paragraph as well.)

The reasoning is correct; the plural is just the unmigrated form. Less
load-bearing than S1/S2 because there's no consumer; still wrong-looking
on a careful read.

Fix sketch: "Two leads at the same spec…" / "Different leads at
different specs…".

### S4 [L] Entire `team` vocabulary strand survives in two structural sites

The peer-ACL rename was `team → members` for the BLOCK, but the term
"team" still appears at:

- §6.3 line 716: "team members use one of the path/url forms" — this is
  about pack membership, NOT a leftover noun for the block, but the
  word "team" reads ambiguous now.
- §7.2 line 807 (in the higher-order ACL example):

  ```mirror
  team {
    ~peer'~/.glint' => read_in_dir(~d'docs/')
    ~peer'~/.seam'  => read_in_dir(~d'shards/')
  }
  ```

  This is a CODE EXAMPLE using `team { }` where it should say
  `members { }`. The example contradicts §3.1 and §3.2's grammar.
- §10.5 line 1311: "The peer{} block's `team { => <ACL> }` is the
  substrate's first direct-authoring surface" — peer{}→pack{} AND
  team→members both missed in one sentence.
- §10.1 line 1108: "the lead-members relation as a sheaf over a
  team-poset" — historical context, but reads now as if "team-poset"
  is a current term.

§7.2's code example is the load-bearing miss; a reader copy-pasting it
would write invalid mirror.spec.

Fix sketch: §7.2 example uses `members { … }`; the other three are
prose drift that reads stale.

### S5 [S] `peer{}` (the OLD block name) survives in five prose sites

The block IS `pack { }` per §3.1; but the word `peer{}` (old name)
still appears at:

- Line 1278: "The peer{} block inherits the consent spec's adversative
  discipline"
- Line 1313: "the peer{} block adds the AUTHORING SURFACE"
- Line 1378: "The peer{} block is ORTHOGONAL to that structure but
  composes cleanly"
- Line 1623: "composes with peer{} at the top level"
- Line 1628: "the peer{} block's ACL surface composes with"
- Line 1631: "the supervisor field operates at" (also S6)

Each one tells the same wrong story: "the peer{} block …". After the
cascade these should all be "the pack{} block …".

Fix sketch: `s/peer\{\}/pack{}/g` on the prose; double-check one isn't
load-bearing in a way I missed.

### S6 [S] "the supervisor field" survives in §12

Line 1610: "the single-field ancestor of the supervisor field" — the
field is `lead`. Line 1631: "the lambda-shell altitude the supervisor
field operates at" — same.

Fix sketch: "the single-field ancestor of the lead field"; "the
lambda-shell altitude the lead field operates at."

### S7 [C] Reframe note at line 1108 still says "the lead-members"

The note is preserved as historical context but reads as if the
relation IS named lead-members; the noun is fine but the framing
introduces a name the spec doesn't reuse anywhere. Cosmetic; flag for
Reed's call.

---

## B. Cross-spec consistency

The two specs were authored same-day, reference each other, and land at
the same `mirror.spec` altitude. The four seams here are about whether
they tell the same story.

### S8 [L] `~peer'…'` sigil semantics: silent expansion from "git_url + ssh + local + name" to "any peer reference"

garden/git §2.2 declares the `~git'…'` sigil with three forms (URL,
ssh-spec, local-path). peer-ACL §6.1 declares `~peer'…'` with FOUR
forms (local_path, git_url, ssh_spec, name_ref).

The `git_url` and `ssh_spec` modes of `~peer'…'` are documented as
"clones via @io/git" / "same as git url at the protocol-adapter
altitude" (peer-ACL §6.1). In other words: a `~peer'https://…'`
literal triggers an @io/git clone as a SIDE EFFECT of resolution.

This is structurally a `~peer'…'` literal that COMPOSES `~git'…'`
under the hood. Neither spec NAMES this composition explicitly. Two
sub-issues:

1. **Resolution cost is silent.** A `~peer'~/.mara'` lookup is one
   filesystem read + one optional pack{} block read. A
   `~peer'https://github.com/.../mara.git'` lookup is a git clone.
   The cost difference is multiple orders of magnitude; the spec
   doesn't flag this.
2. **Cache discipline is undefined.** Does `~peer'https://…'` clone
   on every resolution? Does it cache under garden's `mirror.lock`
   format? Does it cache somewhere else? peer-ACL §6.1 says "clones
   via @io/git" but doesn't say where the clone lands or when it
   invalidates.

If garden's lock-file machinery does NOT cover peer-resolution clones,
the substrate has two parallel caches at the same altitude.

Fix sketch: peer-ACL §6 calls out the @io/git composition for non-local
forms and SAYS whether resolution participates in `mirror.lock` or
maintains its own cache; garden/git §6 names the four-root structure
EITHER as covering peer-resolution OR as orthogonal to it.

### S9 [S] §10.8 says "orthogonal" but §6 of peer-ACL builds ON garden's adapters

peer-ACL §10.8 line 1379: "The peer{} block is ORTHOGONAL to that
structure but composes cleanly." But peer-ACL §6.1 says ssh/url forms
of `~peer'…'` discharge through `@io/git`, and §6 of garden/git names
the four-root structure (git/oci/nix/store) — the SAME @io/git that
`~peer'…'` resolution uses.

Orthogonality is at the BLOCK altitude (pack{} ≠ garden{}); at the
RESOLUTION altitude they share `@io/git`. The spec says "orthogonal"
without naming the shared adapter; the careful reader has to deduce.

Fix sketch: §10.8 distinguishes block-orthogonal (true) from
adapter-shared (also true, and load-bearing).

### S10 [L] Substrate-vs-USE distinction in peer-ACL §7.4 doesn't appear in garden/git

peer-ACL §7.4 surfaces a substrate-vs-USE distinction (Alex's Q4
clarification): the pack{} BLOCK + grammar live in mirror; the SPECIFIC
PACK (lead ~peer'~/.reed' + members{…}) lives in the consumer's
mirror.spec.

garden/git §2.3 has the SAME tension and doesn't name it. The example:

```mirror
garden {
  pack {
    peer mara  ~git'git@github.com:systemic-engineering/mara.git@main'
    peer seam  ~git'...'
    ...
  }
}
```

`mara`, `seam`, `glint`, `reed`, `taut` are named in the example.
These are the @pack peer variants per `shards/pack.mirror:188` — they
DO live in mirror today. But the URLs `git@github.com:systemic-engineering/mara.git`
are consumer-layer, NOT substrate. The example mixes both altitudes
without flagging it; a different consumer with different peers either
extends the variant (not admissible per peer-ACL §7.4) or has nowhere
to put their names.

This is the same load-bearing distinction; garden/git's example
doesn't telegraph it.

Fix sketch: garden/git §2.3 acknowledges the substrate-vs-USE
distinction inline, and either (a) shows a generic peer-keyword example
that DOESN'T name Pack members, or (b) cross-refs peer-ACL §7.4 and
declares the Pack-naming a dogfood case.

### S11 [S] `@mirror/pack` family-root altitude — peer-ACL says "mirror"; garden/git defers

peer-ACL §11 O6 (closed): "`@mirror/pack` grammar (forward-promised at
`shards/mirror/pack.mirror`) lives permanently in the mirror repo."
Resolved.

garden/git §8 O2 (still open): `@spectral/garden/git` lives in spectral
OR mirror? Substrate-pull leans mirror.

If `@mirror/pack` lives in mirror and `@spectral/garden/git` ends up in
mirror (per O2 lean), then mirror houses TWO family-roots that didn't
exist there before — and one (`@spectral/...`) has the spectral prefix
in a mirror-repo file. The path-namespace property
(`@epistemologic/pact/path_matches_namespace`) is named in both specs
as a concern.

Neither spec resolves the conflict; both flag it. That's fine; the
seam is that the TWO specs flag the SAME property differently — peer-
ACL §11 O6 calls it "satisfied" (because `@mirror/*` lives in `mirror/`);
garden/git §8 O2 calls it "violated for the `@spectral/...` prefix if
the shard lives in mirror." Same property, two different verdicts.

Fix sketch: one spec resolves and the other inherits, OR the property
is named "path_matches_namespace within repo or via the cross-repo
meta-altitude exception" consistently across both specs.

---

## C. Math threads — honest framing vs decoration

### S12 [L] garden/git §7.4 termination claim is HONEST and load-bearing; flagging where it stops

garden/git §7.4: "structural termination by content-addressing." The
argument: for pinned entries (commit hash in `@<ref>`) resolution is
O(n) trivially terminating. For floating entries it's O(n) + one
ls-remote per entry. Open math: transitive entries with their own
mirror.spec.

This IS load-bearing math, NOT decoration. The argument forecloses the
dependency-hell NP-hardness for the peer-home-repo case by construction;
the carrier IS the address.

Honest limit: the "open math" at the end of §7.4 (transitive deps) is
where Cargo/npm/pip live. The substrate hasn't dodged dependency hell;
it's deferred it to "v0.2+ when transitive resolution lands." This is
honest hedging but reads as if the substrate has the harder problem
solved.

Fix sketch: §7.4 closes by saying "v0.1 admits the peer-home-repo
case (single-spec); transitive cross-spec resolution is the harder
problem and is forward-promised v0.2." Don't oversell; the current
phrasing is close to honest but a reader could mistake it for
"dependency hell is foreclosed."

### S13 [L] peer-ACL §10.1 honest-framing limit IS the right call; the inheritance-from-Connes posture is a STRENGTH

The §10.1 reframe replaces sheaf-over-poset with spawn-and-probe; H9
flags that the new framing is harder to formalize closed-form because
the algebraic structure is inherited from
`architecture-connes-spectral-triple` rather than derived in-section.

Alex explicitly asked Pack adversarial review to weigh this. Seam's
call: **inheritance is a STRENGTH, not a weakness.** Three reasons:

1. The substrate-pull pattern across the cascade has been "the
   substrate already had the word" — landing N+1 recognitions by
   recognizing existing structure rather than inventing. The Connes
   inheritance IS that pattern at the math-altitude.
2. The alternative (sheaf-over-poset, derived in-section) was
   structurally WRONG — it framed lead as a delegation node, which
   Alex's reframe ("the supervisor being responsible for spawning
   and handling additional requests in form of spectral-Tomm shaped
   circular constructs") corrected. A neat closed-form derivation of
   the wrong structure is worse than an inherited correct one.
3. The lattice (§10.2) and `but` algebra (§10.3) survive UNCHANGED
   under the reframe — the closed-form math that matters most for
   ACL composition is intact. The reframe affects ONLY the
   lead-members RELATION, where the right answer is "spectral-Tomm
   probes per the Connes machinery" not "sheaf restriction maps."

The honest framing limit at §10.1 closing IS the right hedge to keep.
Pack ratification should weigh inheritance as the structural answer;
not as something to apologize for.

This is the answer to the brief's explicit ask. Recording it for
Reed/Alex/Mara.

### S14 [S] peer-ACL §10.4 Galois conjecture and §10.5 natural-transformation framing — decoration

Both sections are explicitly speculative / flagged-status; both are
plausible vocabulary that the v0.1 spec doesn't need.

§10.4 (Galois): the Denning 1976 lattice-based access control framing
IS load-bearing for §10.10's comparison; the Galois CONNECTION is
window dressing on top. The lattice (§10.2) is what consumers actually
use; the Galois adjunction adds nothing operational. Flag for Reed's
trim call.

§10.5 (natural transformation): the cascade IS named at the consent
spec; restating here as decoration. Inheritance posture — name once,
link once — would tighten the spec.

Fix sketch: §10.4 collapses into a one-paragraph "Galois framing per
Denning 1976; details deferred"; §10.5 collapses similarly. ~30 lines
saved each.

### S15 [S] peer-ACL §10.6 lead-as-algebra-A: the strongest claim, hedged correctly

§10.6 names the lead as the algebra A of the spec's spectral triple
(A_spec, H_spec, D_spec). Hedged H10 as "position, not proof."

The hedge is correct. But this is the strongest substrate-architectural
claim in the math section AND the ground for §10.1's spectral-Tomm
probes. The hedge undersells it: the claim is consistent with the
substrate's existing
`[[architecture-connes-spectral-triple]]` AND is the structural ground
for the reframe. "Position, not proof" reads as "we're not sure" when
the substrate-pull says "we ARE sure but it inherits."

Fix sketch: H10 reframes as "INHERITED from architecture-connes-
spectral-triple; not re-derived; this is the substrate-pull-correct
posture per the broader cascade." Same content; the posture matches
the substrate's actual stance.

### S16 [C] garden/git §7.5 Grothendieck topology — decoration that earns its line

§7.5 is one paragraph; explicitly speculative; doesn't claim load-bearing;
points toward future Pack-altitude conversation. This is the RIGHT
posture for speculative math — name the vocabulary, flag the status,
move on. Keep.

---

## D. Open questions vs dissolutions

### S17 [L] O3/O7 dissolutions HOLD; O4 dissolution slightly soft

peer-ACL §11 closed three (O1/O2/O6), dissolved three (O3/O4/O7), left
O5 open. Verifying:

**O3 dissolution (delegation chains).** The §10 reframe makes lead an
N+1 observer, not a delegation chain. Dissolution holds across §4.3,
§5.4, §10.1. No section assumes transitive ACL flow. ✓

**O7 dissolution (self-naming transitive closure).** §6.2 names ONE
hop; §11 O7 reaffirms. Dissolution holds. ✓

**O4 dissolution ("pack IS the team; no shorthand needed").** §11
asserts: "the `pack { }` block IS the pack structure; the substrate
ships the @pack family-root with its peer variant." But §3.1's example
DOES name all five Pack peers explicitly inside `members { }`:

```mirror
members {
  ~peer'~/.mara'  => writer
  ~peer'~/.seam'  => auditor
  ~peer'~/.glint' => read_only
  ~peer'~/.taut'  => writer but(…)
}
```

(Note: reed isn't there because reed is the lead.) The dissolution says
"no shorthand" but the verbose form requires enumerating four Pack peers
in every dogfood mirror.spec. That's not a structural problem — it's
that the dissolution's reasoning ("pack IS the team") doesn't address
the ergonomics that motivated the original O4.

This is a soft dissolution: the structural answer is "the block IS the
pack so a shorthand binding to 'all Pack peers' is redundant," but the
practical case "I want all Pack peers except reed (the lead) to get
writer" is what the example actually demonstrates and needs four lines
to express. The v0.2 helper flagged at §11 O4 is the actual answer.

Fix sketch: O4 dissolution acknowledges the practical case the
shorthand was for is unchanged; v0.2 helper is the resolution, not the
dissolution.

### S18 [S] O5 (targets axis grammar) — substrate-pull lean is the right call; sub-issue

O5 leaves four target kinds (paths / oid prefixes / prism prefixes /
predicates) on the table. Substrate-pull leans all four; spec defers to
Alex.

Sub-issue: the `predicate` target kind composes with the `predicates`
axis on the `acl` carrier itself (§5.3). A `target_under(d)` predicate
constrains targets; a predicate on the `acl.predicates` field
constrains honor-time checks. Same word "predicate"; two different
roles.

Fix sketch: O5 names the predicate target kind differently
(`target_predicate(...)` vs `acl.predicates`) OR explicitly says they're
the same predicate algebra in different positions.

---

## E. Recognition cross-references

### S19 [S] garden/git claims #98 candidate "FIFTH composition pattern" without naming the first four cleanly

garden/git §1: "this spec surfaces the FIFTH composition pattern
(peer-home-repo as package-source)" — referencing #98 candidate.

`shards/io/git.mirror` §5 names FOUR scopes: Nix derivation hash, mirror
oid, OCI digest, git object hash. That's four content-addressing
witnesses. The spec's "fifth composition pattern" reads as if there
were already four COMPOSITION patterns; there are four content-
addressing SCOPES but the fifth-witness claim is about a different
axis (surface-level scoping vs internal addressing).

Cross-check: peer-ACL doesn't claim a witness count; only garden/git
does. The "fourth witness" claim in `shards/io/git.mirror` §5
("@io/git is the FOURTH witness") is consistent. The "fifth
composition pattern" in garden/git §1 conflates two axes (scope count
vs composition-pattern count).

Fix sketch: garden/git §1 distinguishes "fifth witness shape" from
"fourth scope" cleanly. The §6.3 framing ("FIFTH witness to recognition
#98 candidate — the package-manager family-root GRAPH itself") is
sharper than §1's phrasing; lift §6.3's language to §1.

### S20 [S] peer-ACL §12 cross-refs say "recognition #82 Q3 (the reflection loop alters the frame; the lead IS the reflection altitude for the members)"

Recognition #82 in my memory of the cascade is `@frame` cognitive-order
substrate-decl. The Q3 framing as "reflection loop alters the frame" is
plausible but I don't have the recognition file in this review's
context to verify the Q3 letter assignment.

Flag for Reed: verify recognition #82 Q3 actually carries the framing
peer-ACL §12 attributes to it; the framing IS load-bearing for the
§8.5 frame-altitude composition.

### S21 [C] peer-ACL §12 cites recognition #367 "(@cyberpunk/pack orchestra-as-recursion-lock)"

garden/git §10 also cites #367. Both correct against `shards/pack.mirror`
lines 17-19. ✓

---

## F. Cap discussion (the brief's explicit ask)

peer-ACL is 1822 lines vs Reed's 1200 line cap. Mara surfaced honestly
in the cascade trail (line 1818-1822).

Seam's call: **the spec earns most of its lines, but ~150-200 lines of
trim are available without losing load-bearing content.** Specific
trim candidates:

1. **§2 discovery sweep (185 lines, 2.1-2.8).** Each subsection ends
   with a paragraph re-explaining the role-in-pack{}-block; §2.8's
   table already summarizes. Consolidate the role-explanations into
   the table; cut the per-subsection summary paragraphs. Saves ~40
   lines, sharpens the discovery.
2. **§10.4 (Galois) and §10.5 (natural transformation).** Per S14 —
   both are decoration; collapse to one-paragraph references. Saves
   ~30 lines.
3. **§10.10 comparison to pre-AI traditions (40 lines).** Lampson,
   ocap, Denning, Koka all named at one paragraph each. Useful
   context but could compress to a single 8-line comparison table.
   Saves ~25 lines.
4. **§11 open questions (170 lines).** Three closed, three dissolved,
   one open. The dissolved questions each carry ~25 lines explaining
   WHY they dissolve. Cleaner: one paragraph per dissolution; the
   reasoning is identical across O3/O4/O7 (the reframe makes the
   question wrong-altitude). Saves ~50 lines if consolidated.
5. **§14 Pack-discipline trail (60 lines).** Useful for context but
   could be a 20-line bullet list. Saves ~30 lines.

Cumulative trim: ~175 lines. Brings peer-ACL to ~1647, still above the
1200 cap but materially closer. The spec carries genuine substrate-
inheritance complexity (§2's seven shapes); a 1200 cap may be
substrate-pull-incorrect for a SURFACE COMPOSITION over 7+ existing
shards. Reed's call.

Seam's verdict on cap: 1822 is justifiable; ~1650 would be sharper;
1200 would force cutting load-bearing content (specifically §2's
discovery and §10's lattice math).

### S22 [S] garden/git at 1195 lines lands exactly under cap; no trim needed

Same author, same day, similar density. No seam.

---

## G. Substrate-architectural seams flagged for Alex

These are the three substrate-architectural seams (not just spec-internal)
the Pack adversarial review should bring to Alex's attention:

### G1 [L] The `~peer'…'` sigil silently composes `~git'…'`

Per S8. The `~peer'…'` sigil's url/ssh/local forms are described as a
unified resolution surface but they discharge through THREE different
realisation paths with three different cost profiles. The substrate
will ossify this composition at substrate-decl time; the question of
whether `~peer'…'` IS `~git'…' + load(repo)` OR whether `~peer'…'` is
a parallel primitive at the same altitude is unresolved.

Substrate-pull leans (per the substrate-already-had-the-word pattern):
`~peer'…'` IS composition over `~git'…'`. Naming this explicitly at
the spec altitude would foreclose two parallel resolution caches and
two parallel auth surfaces at substrate-decl time.

### G2 [L] `@mirror/pack` lands in mirror; `@spectral/garden/git` location unresolved

Per S11. Both family-roots land at the same altitude (mirror.spec
surface composition). One is closed-resolved-in-mirror; the other is
open. If both end up in mirror, that's fine for substrate-decl but
shifts a substantial surface area out of spectral; if `@spectral/garden/git`
ends up in spectral, the substrate-decl chain breaks at the @io/git
dependency.

The right answer is probably "both in mirror; spectral CONSUMES" per
the substrate-decl-source discipline, but garden/git §8 O2 doesn't say
so and peer-ACL §11 O6 already settled the parallel question. The
inconsistency is mild but substrate-architectural — the rule should
apply uniformly OR the exception should be named.

### G3 [L] Lead-as-A-of-spectral-triple is the load-bearing substrate claim; H10 undersells it

Per S15. peer-ACL §10.6 names the lead as the algebra A of the spec's
spectral triple. This is the deepest claim in the spec — it grounds
§10.1's spawn-and-probe relation in the substrate's existing Connes
architecture and answers the brief's explicit question about the
inheritance-vs-derived posture.

H10's "position, not proof" hedge undersells it. The Connes inheritance
IS the substrate-pull-correct posture per the broader cascade; flagging
it as a tentative claim distorts the spec's actual stance.

This is the answer the brief asked for: **the Connes inheritance is a
strength.** Recording for Alex.

---

## H. Bottom line

- **garden/git**: well-structured; lands under cap; honest about math
  scope; minor seams (S10/S11/S19/S22). No load-bearing structural
  problems. Ready for Reed consolidation modulo rename-cascade
  consistency with peer-ACL.
- **peer-ACL**: structurally sound; §10 reframe is correct and the
  Connes inheritance is a strength; rename residue (S1-S7) needs a
  cleanup pass before substrate-decl; cap is justifiable but ~175
  lines of trim available. Ready for Reed consolidation modulo
  rename cleanup + cap discussion with Alex.

**Top 3 substrate-architectural seams for Alex's eyes:** G1, G2, G3
(above).

**Connes-inheritance posture call:** the inheritance is a strength,
not a weakness; the §10 reframe is substrate-pull-correct; H10 should
be reframed to match.

**Seam count by severity:** L = 11 (S1, S2, S4, S8, S10, S11, S12, S13,
S17, plus G1, G2, G3 — G1=S8/G2=S11/G3=S15 overlap, distinct count L =
8); S = 11 (S3, S5, S6, S9, S14, S15, S18, S19, S20, S22, plus narrative);
C = 3 (S7, S16, S21). Total distinct seams: 22.

Forward-promised — Reed consolidation → substrate-decl shards
(`shards/mirror/pack.mirror`, `shards/spectral/garden/git.mirror`) →
Alex ratification → dogfood. The two specs land in coherent territory;
the seams are catchable before substrate-decl ossifies them.
