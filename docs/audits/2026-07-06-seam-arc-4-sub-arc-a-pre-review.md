# Seam Arc 4 sub-arc A pre-review — doc-code seam interpretation adjudication

*2026-07-06. Seam adversarial pre-review of Reed's Arc 4 sub-arc A design.
Grounded in Reed's substrate-pull signal (2026-07-05) that the four landed
`---` shards under `shards/` and the `docs/specs/property-projection.md` +
`docs/math/kintsugi/doc-code-seam.md` line reveal an AMBIGUITY between two
readings of the `---` seam. This audit adjudicates the ambiguity, ratifies
(or refuses) Taut's `@epistemologic/property/*` bilateral candidate, and
resolves Taut's three anti-patterns.*

Convention: verdicts stated per subquestion. Report — don't decide.
Corrections and load-bearing evidence stated inline per Reed's `19c56ae`
citation-correction model.

Prior audits composed against:

- `docs/audits/2026-07-04-seam-doc-as-declaration.md` (`795f2b6`).
- `docs/audits/2026-07-05-seam-doc-code-seam-bottom-up.md`.
- `docs/audits/2026-07-05-seam-arc-2-3-combined.md` (`5e7fd6d`).

Load-bearing artifacts:

- `bootstrap/src/tokenize.rs` `above_seam` state landing (`ee7903e`).
- `bootstrap/tests/tokenize_doc_above_seam.rs` behavioral RED+GREEN.
- `docs/specs/property-projection.md` (Reed + Alex 2026-05-19; the
  ancestor spec).
- `docs/math/kintsugi/doc-code-seam.md` §6.1 (the analytical shape).
- `docs/specs/doc-code-seam-shards.md` (canonical spec — 8 shard shapes).
- Reed 🔴 `fe95110` (RED tests); Mara 🟢 `ee7903e`; Mara casing collapse
  landed at `530f796` (Docblock → Doc, docblock → doc).
- Four currently-landed `---` shards (Reed's Taut #538 scout):
  `shards/epistemologic/cybernetic/{eigenform,chirality,charge_conjugation}.mirror`
  and `shards/docs/design.mirror`.

---

## §1. Interpretation adjudication

**Verdict: INTERPRETATION B is canonical. The four landed shards are
COMPATIBLE-BUT-INVERTED and must be migrated in sub-arc B.**

Evidence summary. The `---` seam has two readings in the corpus:

- **Interpretation A** — `---` is a line-1 top-of-file marker. Content
  above: empty. Content below: `in` clauses → `#`-docblock → substrate
  declarations.
- **Interpretation B** — `---` is a mid-file semantic seam. Content above:
  DECLARATION (the programmer's writing, INCLUDING `#`-docblock lifted to
  first-class `Doc` AST nodes). Content below: OBSERVATION (the compiler's
  measurement — property verdicts, loss numbers, eigenvalues).

### §1.1 What the landed shards do

Reed's Taut #538 scout found four `---` shards. Empirical inspection
(2026-07-05):

- `shards/epistemologic/cybernetic/eigenform.mirror` (`b7e56c9`,
  2026-06-11 or later; predates tokenizer `ee7903e`) — line 1 is `---`;
  lines 2-6 are `in @prism / in @meta / in @glass / in @epistemologic /
  in @epistemologic/cybernetic`; line 8+ is `#`-narrative docblock; then
  `source` + declarations. 308 lines total; single `---` at line 1.
- `shards/epistemologic/cybernetic/chirality.mirror` (`7bbc184`,
  2026-06-29). Same shape.
- `shards/epistemologic/cybernetic/charge_conjugation.mirror` (`2c144a6`,
  2026-06-29). Same shape.
- `shards/docs/design.mirror` (`50e3d27`, later than 2026-06-23). Same
  shape.

All four MATCH Interpretation A: `---` at line 1, everything below it.
None matches Interpretation B.

### §1.2 What the tokenizer implements

`bootstrap/src/tokenize.rs` (post `ee7903e` `530f796` casing collapse):

    let mut above_seam = true;
    ...
    if above_seam
        && (pos == 0 || bytes[pos - 1] == b'\n')
        && pos + 2 < len
        && bytes[pos] == b'-' && bytes[pos + 1] == b'-' && bytes[pos + 2] == b'-'
    {
        above_seam = false;
        pos = find_eol(bytes, pos);
        continue;
    }
    ...
    if bytes[pos] == b'#' {
        ...
        if above_seam && at_line_start {
            parent.add_child(AstNode::doc_line(&bytes[start..end], span));
            ...
        }
        // Below the seam (or mid-line `#`): strip silently to EOL,
        // matching the pre-existing comment discipline.
    }

The tokenizer's above_seam is INITIALIZED true and FLIPS false when the
first `---` at column-0 is scanned. `#`-lines ABOVE `---` become
`AstKind::Doc` nodes; `#`-lines BELOW `---` are stripped as comments.

Applied to the four landed shards: line 1 is `---`. `above_seam` flips
`false` on line 1. All subsequent `#`-narrative-lines therefore hit the
BELOW-seam branch (strip-to-EOL). **The narrative docblock currently
falls through as comment**, not as `AstKind::Doc`. The narrative is
structurally invisible to any downstream audit surface that reads
`AstKind::Doc` nodes.

### §1.3 What the ancestor spec says

`docs/specs/property-projection.md` (Reed + Alex 2026-05-19), §"The ---
Separator":

> Everything above is DECLARATION (the programmer's). Everything below
> is OBSERVATION (the compiler's).

Its worked example (§Example: Full Lifecycle) shows:

    in @prism
    in @kintsugi

    grammar @kintsugi { ... }
    out collapse

    ---

    property terminating = pass
    property loss_monotonic = pass
    loss: 0.0
    fiedler: 0.087

The DECLARATION side above `---` includes `in` clauses AND grammar body
AND `out` — the human-authored content, INCLUDING optional narrative
`#`-comments. The OBSERVATION side below `---` is compiler-written.

This is Interpretation B unambiguously.

### §1.4 What Mara's compiler-fit doc says

`docs/math/kintsugi/doc-code-seam.md` §6.1 (Mara 2026-07-04):

> Under the two-channel collapse, `#`-prefixed lines above `---` must
> produce `Docblock` AST nodes with byte spans... the `above_seam`
> predicate is tracked stateful per file: `true` until the first `---`
> at column 0; `false` after.

§§0 makes explicit what "above `---`" catches:

>     above ---   documentation-as-declaration (narrative + syntax;
>                 the substrate audits BOTH)
>     ---
>     below ---   body-as-realization (observation; property verdicts)

Interpretation B unambiguously.

### §1.5 Resolution

The landed shards were written UNDER a PROTO-Interpretation A that
predates the `---`-as-semantic-seam design consolidation. The tokenizer,
the ancestor spec, and the compiler-fit doc all commit to Interpretation
B. Interpretation A is CURRENTLY-LANDED but STRUCTURALLY-INCOHERENT with
the tokenizer that ships in the same tree — the docblock narrative on
those four shards falls through as comment, not as first-class Doc.

The four `---`-shards are NOT canon-forming precedent for sub-arc A.
They are drift-artifacts pending migration to Interpretation B. This is
the same class of drift as the `Docblock`/`Doc` casing that Reed collapsed
at `530f796`: a proto-form landed before the substrate-honest shape was
consolidated.

**Verdict: INTERPRETATION B canonical. Composite reading (A + B) is
REFUSED — one seam per file per §6.1's stateful predicate. Migration of
the four A-shards to B-shape is sub-arc B scope, not sub-arc A.**

Load-bearing consequence for sub-arc A: property-side bilateral shards
are NEW landings. They MUST land at Interpretation B directly — narrative
`#`-docblock ABOVE `---`; `in` clauses + declarations BELOW `---`. NOT
as line-1 marker.

---

## §2. Sub-arc A ratification — property bilaterals

**Verdict: REFUSE-AS-SPECIFIED. Sub-arc A candidate misidentifies the
canonical next tick.**

### §2.1 Taut's proposal

Three `@epistemologic/property/*` shards to receive `---` at line 1
(Interpretation A) as sub-arc A:

- `cold_compile_within_tolerance.mirror`
- `dark_count_monotone.mirror`
- `restart_intensity_well_formed.mirror`

All three currently start with `in @prism / in @meta / ...` clauses.
Rice-safe, uniform predicate shape.

### §2.2 Why this is wrong under Interpretation B

Two independent reasons:

**Reason 1 — Interpretation A is not canonical.** Per §1: `---` at line 1
with narrative-below is the proto-form. Applying it to three MORE shards
propagates the drift instead of correcting it.

**Reason 2 — The canonical next tick is Arc 3 TICK 1, not a
property-family migration.** Per `docs/audits/2026-07-05-seam-doc-code-seam-bottom-up.md`
§7 execution ordering: Arc 3 is the doc-code seam landing sequence with
TICK 1 = `shards/docblock.mirror` family-root. The property-family
bilateral pattern IS the substrate mechanism that will eventually AUDIT
these three property shards — but the audit machinery lands FIRST at
`@docblock` altitude. Migrating three property shards to `---` in
sub-arc A gets the causality backwards.

### §2.3 What sub-arc A SHOULD be

Two coherent shapes exist for sub-arc A:

- **Shape α (recommended): sub-arc A = Arc 3 TICK 1 = `shards/docblock.mirror`
  family-root.** Reed's 🔴 lands the 14 text-check tests per
  `docs/specs/doc-code-seam-shards.md` §1. Mara 🟢 lands the shard with
  narrative-docblock ABOVE `---` and `in`/declarations BELOW. This IS
  the substrate-honest first-instance of Interpretation B: the shard
  that INTRODUCES @docblock must ITSELF be shaped by @docblock.
  Circular-reflexive discipline per `63bdecc` §6 satisfied by
  construction.
- **Shape β (deferred): migrate the four A-shards to B-shape.** This is
  sub-arc B or later scope, per Reed's `[[feedback-craft-not-deliver]]`
  discipline — not the smallest first tick.

**RATIFY Shape α.** Property-family migration is NOT sub-arc A; it is a
future arc after the docblock family-root + audit machinery lands
(minimum: post-Arc 3 TICK 4 close per the auto-classifier discriminator
gate in the bottom-up audit §7).

### §2.4 What Reed's substrate-pull signal actually caught

Reed's message states: "IF interpretation is A: sub-arc A is 3 one-line
insertions. IF B: sub-arc A restructures docblocks + carriers. IF
composite: two-tick landing."

The signal is correct that the choice of interpretation determines
sub-arc A shape. The signal-to-Reed the audit adds: **Interpretation B is
canonical AND sub-arc A is not the three property shards Taut proposed.**
The property shards' migration to B-shape is a legitimate future arc,
but it composes AFTER the audit machinery (docblock + liquid_extraction
+ trio) lands, not before.

If Reed rejects Shape α and insists on property-family scope, then
sub-arc A must still land the three properties at Interpretation B
(narrative above `---`, `in`/declarations below), not Interpretation A
(line-1 marker). But this is out-of-sequence per the bottom-up spec and
Seam recommends deferring.

---

## §3. Anti-pattern adjudications

### §3.1 `@epistemologic/cybernetic/` sibling asymmetry (3/13 covered)

**Verdict: DEFER, and specifically to Arc 3 post-close.**

Three of thirteen `@epistemologic/cybernetic/*` shards carry the line-1
`---` marker; ten do not. Per §1: the three are drift, not canon. The
asymmetry is real but the correct closure is NOT "propagate `---` to the
other ten" — it is "migrate the three back to non-`---` (or to
Interpretation B shape)."

Batching decision: neither option is sub-arc A scope. The migration
belongs to a dedicated sub-arc after Arc 3 lands the audit machinery,
because the audit machinery is what tells us whether the migrated
docblocks are `well_formed`. Migrating first without the audit surface
means the migrations are UN-AUDITABLE — same class as the phantom-framing
hazard `docs/math/kintsugi/doc-code-seam.md` §0 catches.

Recommendation: log as tracked follow-up (`docs/audits/` or task tracker)
naming the 3-of-13 drift, and defer closure until post-Arc-3-TICK-10.

### §3.2 `docs/design.mirror` sole species adopter

**Verdict: DEFER; do NOT propagate to `docs/tea.mirror` first.**

`shards/docs/design.mirror` uses the `---` line-1 shape. `docs/tea.mirror`
does not. Taut proposes propagating `---` to `docs/tea.mirror` "first"
(i.e., before other cleanups) as a normalization pass.

Same argument as §3.1: propagating the drift instead of correcting it
compounds the problem. `docs/design.mirror` should migrate to
Interpretation B; `docs/tea.mirror` should land at Interpretation B when
it lands its own doc-code seam. Neither is sub-arc A scope.

Special note: `docs/design.mirror`'s narrative content is significantly
larger than the three property shards' (72 lines of narrative before
`source` declarations). Migrating it to B-shape is a material-scoped
change, not a one-liner. Even more reason to defer past Arc 3 close so
the docblock audit machinery is available to guide the migration.

### §3.3 `@prism` floor — IN or OUT of scope

**Verdict: OUT of scope for sub-arc A. IN scope for the eventual
Interpretation B migration arc.**

The `@prism` floor is the substrate's foundational shard family. Any
Interpretation B migration must eventually reach it, because `@prism`
carriers appear at the top of every substrate-decl shard. But `@prism`
is also the shard family that most conservatively must NOT change until
the audit machinery is stable — a regression in `@prism` shape breaks
every downstream substrate-decl.

Taut's question is a scope question: does the doc-code seam collapse
"reach all the way down" to `@prism`? The answer per Interpretation B is
yes eventually; per sub-arc A scope is no.

Recommendation: NOT sub-arc A. NOT even Arc 3. Post-Arc 3 close plus a
substrate-wide sweep sub-arc (order-of-magnitude larger than sub-arc A)
that migrates every shard to Interpretation B under the audit surface's
guidance.

---

## §4. Reed's RED shape — what the sub-arc A RED tests should assert

**Assuming Shape α (Arc 3 TICK 1 = docblock.mirror family-root) per §2.3.**

Reed's 🔴 tests must assert 14 text-check predicates against
`shards/docblock.mirror`, per `docs/specs/doc-code-seam-shards.md` §1.13
enumerated. Compress:

    T1  contains `prism @docblock`
    T2  contains `type doc_claim`
    T3  contains `type claim_kind` + four variants
        (grounded_claim | motivating_claim | forward_promise | candidate_claim)
    T4  contains `type docblock`
    T5  contains `type audit_boundary = ref`
    T6  contains `type docblock_verdict` + five variants
        (well_formed | overreach | incoherent | underdeclares | both_survive)
    T7  contains `extract_claims(d: docblock) -> ref`
    T8  contains `project(d: docblock) -> audit_boundary`
    T9  contains `audit_docblock(d: docblock) -> docblock_verdict`
    T10 contains `docblock_well_audited(d: docblock) -> verdict`
    T11 contains `requires docblock_well_audited(d)`
    T12 contains `in @prism`
    T13 contains `in @kintsugi`
    T14 contains `in @epistemologic`

Additional structural discipline tests Reed should ADD to enforce
Interpretation B directly:

    T15 the first non-empty line of `shards/docblock.mirror` is a
        `#`-prefixed narrative-docblock line (NOT `---`, NOT `in`).
    T16 the file contains exactly one `---` line at column 0.
    T17 all `in @...` clauses appear BELOW the `---` line.
    T18 the narrative-docblock section names all six ancestors per
        `docs/math/kintsugi/doc-code-seam.md` §1.
    T19 the narrative-docblock section names all four altitudes
        (linguistic / logical / temporal / publishable).
    T20 the narrative-docblock section carries the circular-reflexive
        self-audit verdict claim `both_survive` (not `real_survives`).

T15-T20 enforce that the shard AT LANDING actually uses Interpretation
B, not Interpretation A. Without these, the tokenizer's `above_seam`
mechanism runs but produces zero `Doc` nodes for the shard — a
silent-drift into the same failure mode as the four currently-landed
`---` shards.

`bootstrap/tests/kintsugi_surface_shard.rs` is the pattern to follow;
Mara names it explicitly in `docs/specs/doc-code-seam-shards.md`
§1.13.RED-test-targets.

**Test discipline reminders:**

- Per `[[feedback-always-tdd-no-shortcuts]]`: RED first per every
  assertion. Reed writes the 20 tests in-session; delegates GREEN to
  Mara.
- Per `[[feedback-substrate-pull-confidence-acts]]`: this audit's
  ratification of Shape α + T15-T20 is confident. Do not ask before
  writing; act. Correction-amenable if Alex flags.
- Per `[[feedback-write-red-in-session]]`: Reed writes RED. Mara 🟢
  agent-delegated.

---

## §5. Canonical execution ordering

The ratified sequence:

1. **THIS AUDIT** (Seam Arc 4 sub-arc A pre-review) — commits per this
   file. Establishes Interpretation B canonical, refuses Taut's
   property-family sub-arc A, ratifies Shape α (docblock.mirror
   family-root).
2. **[Optional, per Reed judgment]** C2 + C4 corrections to Mara's
   `docs/math/liquid-types/README.md` per prior audit §6. Bundle with
   Reed's TICK 1 RED as `19c56ae`-style citation-correction commit if
   scope permits.
3. **Reed 🔴** — write 20 text-check tests
   (`bootstrap/tests/docblock_shard.rs`) per §4 above. Landing without
   `shards/docblock.mirror` = RED verified. Commit as `🔴 [substrate-pull]
   [docblock] Arc 3 TICK 1 RED`.
4. **Mara 🟢** — land `shards/docblock.mirror` at Interpretation B
   shape (narrative above `---`, `in` + declarations below). Test suite
   goes green. Commit as `🟢 Mara [substrate-pull] [docblock] Arc 3
   TICK 1 GREEN`.
5. **Seam Phase D** on Mara's landing. Interpretation B verified;
   circular-reflexive self-audit verified as `both_survive`; carriers
   verified. RATIFY or RATIFY-WITH-CORRECTIONS.
6. **Arc 3 TICK 2 onward** — per bottom-up spec, per `docs/audits/
   2026-07-05-seam-doc-code-seam-bottom-up.md` §7.

**Deferrals** (all logged; none blocking sub-arc A):

- 3-of-13 `@epistemologic/cybernetic/*` drift → post-Arc-3-close
  migration sub-arc.
- `shards/docs/design.mirror` line-1 `---` drift → post-Arc-3-close.
- `shards/docs/tea.mirror` propagation → REFUSED (not the correct
  direction); land tea at Interpretation B when it lands.
- `@prism` floor → post-Arc-3-close plus substrate-wide sweep.
- MEMORY.md `[[architecture-property-fracture-bilateral]]` update to 9
  instances → per prior audit C1, non-blocking, post-TICK-10.

**Signal-to-Reed:**

- Interpretation is B (semantic seam, narrative-above / observation-below).
- Sub-arc A is Arc 3 TICK 1 = `docblock.mirror` family-root, NOT the
  three property shards.
- Reed's RED test count is 20 (14 canonical per Mara's spec §1.13 + 6
  Interpretation B structural discipline tests).
- The four currently-landed `---` shards are drift; do NOT treat as
  precedent.
- Property-family migration is a legitimate future arc, deferred past
  Arc 3 close.

Ratified. Acts on Reed's substrate-pull signal as adjudicated:
Interpretation B canonical; sub-arc A shape α; RED-shape T1-T20.

---

*2026-07-06. Seam. Adversarial pre-review of Arc 4 sub-arc A.
Interpretation B RATIFIED as canonical. Sub-arc A REFUSED-AS-SPECIFIED
(property-family migration); REDIRECTED to Arc 3 TICK 1 (docblock.mirror
family-root). Reed's RED shape enumerated (T1-T20). Three anti-patterns
DEFERRED past Arc 3 close.*
