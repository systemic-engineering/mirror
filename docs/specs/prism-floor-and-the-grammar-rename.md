# The prism floor + the grammar→prism rename — the nine-keyword floor and the ticks to close it

*2026-05-29. Mara. Spec — audit + design, not implementation. No Rust changes; read-only audit of the bootstrap, one markdown spec.*

Status: **Yellow** (the nine-keyword floor was locked with Alex 2026-05-29;
the gap between it and the bootstrap's *actual* recognition is audited below
and turns out to be narrower than the framing implies; the rename's blast
radius is measured; the kintsugi-fracture engine that would *ideally* run the
rename is confirmed a no-op scaffold, so the honest path is a runnable
mechanical migration realizing the fracture principle, not a literal fracture
pass.)

Depends on:
- `bootstrap/src/grammar.rs` (commit `c367512`) — `parse_grammar` (the
  keyword harvester + the hardcoded five-op match + the `grammar ` block
  opener), `load_grammar`, `companion_keyword_sources`, `is_mirror()`,
  `is_skip_word`.
- `bootstrap/src/tokenize.rs` (commit `c367512`) — the structural-form
  dispatch (`io` / `match` / `select`), the `pub` skip, the `in`/`out`
  refinement of an already-harvested `Project` kind, the action-decl and
  parametric-return walkers, the Dark fallback.
- `bootstrap/src/spectral.rs` (commit `c367512`) — the render side; the
  `"@mirror/grammar"` tag equality and the `b"grammar "` literal the renderer
  emits on round-trip. The rename's hidden coupling lives here.
- `bootstrap/src/pipeline.rs` (commit `c367512`) — `parse_rewrite` /
  `apply_rewrites` (whole-word-bounded byte rewrite, `/` and `@` as
  boundaries, sequential rule application) and `cmd_kintsugi_migrate`'s
  directory walk: the runnable mechanical rename engine.
- `bootstrap/src/main.rs` (commit `c367512`) — `kintsugi_tick`
  (`// no-op scaffold`), `cmd_kintsugi` / `cmd_kintsugi_migrate` /
  `cmd_kintsugi_single`. The kintsugi-fracture feasibility verdict lives
  here.
- `boot/std/mirror/grammar.mirror` (commit `c367512`) — the legacy meta-glass
  keyword source; the file that *would* become `prism.mirror`.
- `boot/std/mirror/glass/ast/token.mirror` (commit `c367512`) — the
  substrate-altitude keyword-surface declarations (`focus glass`,
  `focus lambda`, `focus fixed`, `focus property`, `focus shape`, plus the
  legacy `focus grammar` / `focus prism` / `split type` / ...).
- `boot/std/kintsugi/fracture.mirror` (commit `c367512`) — fracture as a
  closure operator on the AST lattice; the substrate shape the rename
  realizes (and which the engine does not yet evaluate).
- `docs/specs/mirror-grammar-self-hosted.md` — the meta-glass that wants to
  replace `tokenize.rs`; the keyword table the bootstrap reads on every parse.
- `docs/specs/surface-simplification.md` — the precedent: a vocabulary
  collapse written as ticks; the five-operation closure principle.
- AGENTS.md §"Keywords Are Substrate Declarations", §"The Glass Wall",
  §"Boundary Rust is not frozen capability" — the `[substrate-pull:realize]`
  marker discipline for every `.rs`-touching tick.

Unblocks:
- The `prism` crate's sharpened mandate ("the nine, nothing else; numerics
  behind a feature") — resolves the prism-naming "collision" the maintenance
  round flagged.
- The downstream doc-fracture ticks that rename `grammar`-terminology specs to
  `prism` (📝 follow-ons, §6).
- The (deferred) `lambda → glass` push once mirror self-tokenizes.

---

## 0. The headline finding (read this first)

The locked target is a **nine-keyword Rust language floor**:

> **focus, project, split, zoom, refract** (the five operations),
> **prism** (declares a prism — a namespace / optical space; replaces
> `grammar`), **glass** (extends/builds a prism with material), **in, out**
> (import/export). Plus ONE transitional keyword, **lambda**, kept in the
> floor only for bootstrap tokenization and dropped to glass the moment
> mirror tokenizes itself.

The audit's surprise: **the bootstrap's Rust floor is already much smaller
than "nine keywords."** Rust does not hardcode nine keyword strings. It
hardcodes:

1. the **five-operation match** (`focus/split/zoom/project/refract` as the
   *first* word of an `<op> <keyword>` declaration line — `grammar.rs:140`),
2. a single literal **`grammar ` block-opener** (`grammar.rs:80`),
3. a single literal **`"@mirror/grammar"` ref-equality** gate (`is_mirror()`,
   `grammar.rs:60`; mirrored on the render side, `spectral.rs:559`),
4. the render-side **`b"grammar "` emit** for round-trip (`spectral.rs:606`),
5. four `word == "..."` **structural-form** dispatches in the tokenizer
   (`pub`, `io`, `match`, `select`) plus an `in`/`out` *refinement* of an
   already-harvested `Project` node.

Everything else — `prism`, `glass`, `type`, `fixed`, `property`, `shape`,
`abstract`, `lambda` — is **not a Rust string at all.** Those words are
*harvested from the substrate*: the `<op> <keyword>` two-word lines in
`grammar.mirror` and `token.mirror`, read by `parse_grammar` and merged by
`companion_keyword_sources`. They are already glass in the sense the closure
principle wants.

Two consequences fall out:

- **"Floor-creep" is almost a non-issue at the keyword-table level.** `type`,
  `fixed`, `property`, `shape`, `abstract` are *not* recognized by hardcoded
  Rust; they are substrate declarations the harvester picks up. There is no
  per-keyword Rust `match` arm to delete for them. The only true Rust-resident
  recognition that should eventually move to glass is the `match`/`select`/`io`
  *structural dispatch* and the `is_skip_word` Rust-noise set (the shelved
  tick) — and those are structural forms, not the simple `<op> <keyword>`
  vocabulary the floor framing is about.
- **The `grammar → prism` rename is NOT a pure substrate rewrite.** Because
  Rust hardcodes the `grammar ` block-opener and the renderer emits `b"grammar "`,
  a content rewrite of the `.mirror` files alone would break the parse/round-trip:
  the harvester would no longer recognize `prism @x {` as a block opener, and
  the renderer would still emit `grammar `. The rename is a **coupled .rs +
  .mirror change** — exactly the kind of change `[substrate-pull:realize]`
  exists to mark.

So the floor framing is *correct as a statement of intent* ("Rust recognizes
exactly the nine; the substrate defines the rest of its own vocabulary"), and
the work to close the gap is mostly (a) renaming the one loaner word the floor
hardcodes (`grammar` → `prism`) and (b) confirming the already-true
substrate-defined status of the rest. The big, risky floor-creep removal the
ticket worried about (pushing `type`/`fixed`/`property`/`shape` out of Rust)
**is already done** — those never lived in Rust.

---

## 1. The nine-keyword floor — stated, with the optical justification

A prism is a piece of glass cut to bend light. The floor is built from that
one physical picture:

```
prism     declares a prism — a namespace / optical space.
          (replaces the loaner word `grammar`.)
glass     the material a prism is made of. extends/builds a prism.
focus     \
project    \
split       >  the five operations — what a prism DOES to a beam.
zoom       /
refract   /
in        import — compose another prism into this one.
out       export — expose this prism's surfaces to others.
```

The justification is not decorative; it is the closure argument:

- **A prism is made of glass.** `prism` and `glass` are the two
  material/structural keywords. `prism` opens the optical space; `glass`
  builds the medium inside it. Everything the substrate later names
  (`type`, `fixed`, `property`, `shape`, `lambda`, `abstract`, …) is *a kind
  of glass* — substrate-defined material, declared with the nine, never
  hardcoded in Rust.
- **The five operations are what prisms do.** focus/project/split/zoom/refract
  are the Prism trait's methods. They are the irreducible verbs; nothing the
  substrate adds is a sixth operation (the precedent is
  `surface-simplification.md`'s five-verb collapse — `init`, `build`, `check`,
  `trace`, … all dissolved into compositions of the five).
- **in/out compose prisms.** Import and export are how one optical space is
  built from others. Rust already knows these words (and the bootstrap already
  refines a harvested `Project` into `In`/`Out` — `tokenize.rs:671`), so they
  are floor-natural.

**lambda is transitional.** It is floor-for-now purely so the hand-written
tokenizer can recognize the binding form fast during bootstrap. The moment
mirror tokenizes itself (the meta-glass of
`mirror-grammar-self-hosted.md`), `lambda` drops to glass like every other
binding-shape keyword. It is flagged, not deleted, by this spec.

**Everything-else-is-glass is the closure principle.** Rust recognizes exactly
the nine (+lambda transitionally). The substrate defines the rest of its own
vocabulary by *declaring* it with the nine. This is not aspirational for the
keyword table — as §0 found, it is already the bootstrap's behaviour: `type`,
`fixed`, `property`, `shape`, `abstract` are harvested from `.mirror`, not
matched in `.rs`.

---

## 2. The prism unification — one concept across four altitudes

`prism` is not four colliding names. It is one concept seen from four
altitudes:

| Altitude | What `prism` is |
|---|---|
| **Keyword** | the floor word that declares a prism (namespace / optical space). Replaces `grammar`. |
| **Crate** | the `prism` Rust crate — zero deps; the Prism trait, `Beam<T>`, the five operations, `ShannonLoss`. |
| **Trait** | the `Prism` trait itself — five methods, one for each operation. |
| **Thesis** | "everything is a Prism" — every grammar, every glass, every operation composes as one. |

These are the same idea at the language, library, type, and architecture
levels. The maintenance round flagged this as a "prism naming collision"; it
is not a collision, it is the unification working as designed. The keyword
*should* be the same word as the crate, the trait, and the thesis, because
they are the same concept.

The unification **sharpens the prism crate's mandate**: the crate IS the
floor. Its job is "provide the nine and nothing else." Numerics — the `d_s`
spectral-dimension kernel, the Dirac operator `D` of the spectral triple —
correctly sit *behind a Cargo feature*, not in the crate's core, because they
are the numerical floor the five ops are *read against* (per
`cosmos-mirror-scaffold.md` §3: eigendecomposition "is not one of the five
operations — it is the `D` of the spectral triple"). A prism core with
numerics in the default build would be capability-in-the-floor. Feature-gated,
the mandate holds: the nine, nothing else, by default.

---

## 3. The gap audit table

Read "recognized in Rust today?" precisely: it means *the literal word appears
in a Rust `match`/`starts_with`/`==`*, NOT *the word works as a keyword* (most
keywords work via substrate harvest, which is not Rust recognition).

| Keyword | Recognized in Rust today? (where) | Target | Gap action |
|---|---|---|---|
| `focus` | **yes** — five-op match `grammar.rs:140`; render `spectral.rs` reverse-lookup | permanent floor | keep |
| `project` | **yes** — five-op match `grammar.rs:140` | permanent floor | keep |
| `split` | **yes** — five-op match `grammar.rs:140` | permanent floor | keep |
| `zoom` | **yes** — five-op match `grammar.rs:140` | permanent floor | keep |
| `refract` | **yes** — five-op match `grammar.rs:140` | permanent floor | keep |
| `in` | **yes** — refines harvested `Project`→`In` `tokenize.rs:671` (Rust also knows `in` as `is_skip_word`) | permanent floor | keep |
| `out` | **yes** — refines harvested `Project`→`Out` `tokenize.rs:673` | permanent floor | keep |
| `prism` | **no** — substrate-only (`focus prism` in `token.mirror:32` / `grammar.mirror:14`, `00-prism.mirror:4`). NO `prism @x {` block-opener in Rust. | permanent floor | **rename target** — `grammar`'s recognition path moves to `prism` (see §4); the word must become a real Rust block-opener |
| `glass` | **no** — substrate-only (`focus glass`, `token.mirror:67`) | permanent floor | keep (already glass-declared; confirm it parses as a top-level form once `prism` opens the block) |
| `grammar` | **yes** — `grammar ` block-opener `grammar.rs:80`; `is_mirror()` ref `"@mirror/grammar"` `grammar.rs:60`; render emit `b"grammar "` `spectral.rs:606`; tag eq `spectral.rs:559`; combinator literals `spectral.rs:3020/3029` | **retired** (loaner word from parser theory) | **rename** → `prism` (the fracture, §4) |
| `lambda` | **no** — substrate-only (`focus lambda`, `token.mirror:68`) | **transitional floor** | keep-for-now; flag `lambda → glass` deferred to self-tokenize |
| `type` | **no** — substrate-only (`split type`, `grammar.mirror:15`, used in **82 files**) | glass (already) | none — already glass; do NOT touch (load-bearing, §7) |
| `fixed` | **no** — substrate-only (`focus fixed`, `token.mirror:69`) | glass (already) | none — already glass |
| `property` | **no** — substrate-only (`focus property`, `token.mirror:70`) | glass (already) | none — already glass |
| `shape` | **no** — substrate-only (`focus shape`, `token.mirror:71`) | glass (already) | none — already glass |
| `abstract` | **no** — substrate-only (`zoom abstract`, `grammar.mirror:18`) | glass (already) | none — already glass |
| `io` | **yes** — structural dispatch `tokenize.rs:421` (Spec A binding) | glass (eventually) | structural-form; defer with `match`/`select` to meta-glass self-host |
| `match` | **yes** — structural dispatch `tokenize.rs:494` (Spec B) | glass (eventually) | structural-form; defer to meta-glass self-host |
| `select` | **yes** — structural dispatch `tokenize.rs:548` (Spec B) | glass (eventually) | structural-form; defer to meta-glass self-host |
| `pub` | **yes** — Rust-noise skip `tokenize.rs:389` | glass/noise | leave (Rust-source noise, not a mirror keyword) |
| `is_skip_word` set (`async`, `let`, `where`, `self`, …) | **yes** — `grammar.rs::is_skip_word` | glass (the shelved tick) | defer — the shelved `is_skip_word` push-to-glass; round-trip-risky (§7) |

**Classification summary:**

- **Floor-correct (the nine):** `focus`, `project`, `split`, `zoom`,
  `refract`, `in`, `out`, plus `prism` and `glass` *as targets* (`prism`
  becomes a real Rust block-opener via the rename; `glass` is already
  substrate-declared and needs only to parse as a top-level form).
- **Transitional:** `lambda` — substrate-declared today, kept in the
  conceptual floor until self-tokenize; no Rust change needed now.
- **Floor-creep (recognized in Rust, should be glass):** *Not the simple
  vocabulary the ticket feared.* `type`/`fixed`/`property`/`shape`/`abstract`
  are **already glass** (substrate-harvested — no Rust to remove). The genuine
  Rust-resident recognition that is conceptually floor-creep is the
  **structural-form dispatch** (`io`/`match`/`select`) and the
  **`is_skip_word`** Rust-noise set. Both are deferred (they belong with the
  meta-glass self-hosting of `tokenize.rs`, per
  `mirror-grammar-self-hosted.md`), not with this floor-rename round.
- **Retired:** `grammar` — the loaner word; renamed to `prism`.

---

## 4. The grammar→prism rename — kintsugi fracture or the honest path

### 4.1 The intent: a kintsugi fracture

The rename is *meant* to be a kintsugi fracture: the system renames itself by
fracturing and healing, not by find-replace. The precedent is the earlier
`shape.grammar → glass` fracture. A fracture is a closure operator on the AST
lattice (`@kintsugi/fracture`): `detect(f, x) = (f(x) == x)`, `fill(f, x) =
f(x)`; one function, two laws (idempotence; canonical-at-fixpoint). The rename
`grammar => prism` is exactly such a fracture: a closure operator whose fixed
points are the prism-named forms.

### 4.2 The feasibility check (CRITICAL — do not assume)

**Verdict: the kintsugi-fracture *engine* is a no-op scaffold. A literal
fracture pass cannot run today.**

`kintsugi_tick` (`main.rs:536`) is explicitly a scaffold. Every stage is the
identity:

- Stage 1 (propose): `let candidates: Vec<()> = Vec::new();` — zero candidates.
- Stage 2 (measure): `let loss: f64 = 1.0;` — hardcoded full residue; the only
  real signal is `count_dark`, read-only.
- Stage 3 (elect): `None` (no proposal).
- Stage 4 (verify): `let verify_pass: bool = true;` — trivial pass.
- Stage 5 (fixed-point): `prior_oid == current_oid` — vacuously true on tick 1
  because nothing is spliced.

The doc comment says it outright: *"Every body is no-op for this scaffold."*
The `@kintsugi/fracture` substrate (`fracture.mirror`) declares `apply`,
`idempotent`, `canonical_at_fixpoint` — but their bodies are `\` obligation
holes, and the bootstrap does not evaluate grammar action bodies. So the
fracture-as-closure-operator engine does not exist as a runnable thing yet.

**Do not design a tick that pretends the stub can run the fracture.**

### 4.3 The honest path: a runnable mechanical migration that *realizes* the fracture principle

There IS a runnable rename engine in the bootstrap — it is just not the
closure-operator fracture. It is `cmd_kintsugi --out <dir> --transform
'grammar => prism'` (`main.rs::cmd_kintsugi_migrate`), built on
`pipeline.rs::apply_rewrites`:

- whole-word-bounded byte rewrite (the symbol must be bounded by non-word
  bytes; `/` and `@` are boundaries, so `@mirror/grammar`'s trailing path
  component rewrites and `grammars` does not),
- sequential rule application (rule N sees rule N−1's output),
- directory walk over every `.mirror` file, plus **basename rewrite**
  (`grammar.mirror` → `prism.mirror` when the rule is `grammar => prism`).

This is tested — `pipeline.rs` pins `grammar => glass` on `@mirror/grammar`
→ `@mirror/glass` and the `grammars`-doesn't-match boundary case. It is a real,
working content+path migration.

But on its own it is **insufficient and would break the build**, because of
§0's coupling: the Rust `parse_grammar` block-opener (`grammar.rs:80`) and the
renderer's `b"grammar "` emit (`spectral.rs:606`) still say `grammar`. So the
honest path is a **coupled change**:

1. The `.mirror`/path rewrite via the mechanical migration tool (runnable
   today), realizing the fracture *as a principle* — `grammar => prism` is a
   closure operator; running it twice is idempotent; its fixed points are the
   prism-named forms.
2. The matching `.rs` recognition change — block-opener `grammar ` → `prism `,
   ref `"@mirror/grammar"` → `"@mirror/prism"`, render emit `b"grammar "` →
   `b"prism "`, combinator literals — marked `[substrate-pull:realize]`.

The principle the fracture *names* is honoured; the literal fracture-engine
pass is **gated on the kintsugi engine landing** (deferred per LRM, see
`kintsugi-formatter.md` / `kintsugi-wiring.md`). When that engine becomes
runnable, this rename becomes the canonical worked example of a self-rename
fracture — but we do not block the floor cleanup on it.

### 4.4 The blast radius

| Surface | Count | Notes |
|---|---|---|
| `.mirror` files using the `grammar @x` / top-of-block `grammar` declaration | **158** | every grammar file in `boot/` opens with `grammar @name { … }` |
| `.mirror` files referencing `@mirror/grammar` (the meta-prism ref) | **9** | `glass_wall.mirror`, `glass.mirror`, `ast.mirror`, `ast/shape/in.mirror`, `ast/shape/out.mirror`, `ast/token.mirror`, `grammar.mirror`, `spec.mirror`, `peer.mirror` |
| `.rs` files hardcoding the literal `grammar` recognition/emit | **3** | `grammar.rs` (opener + `is_mirror` ref), `spectral.rs` (tag eq + emit + combinator literals), `pipeline.rs` (only test fixtures — not load-bearing) |
| The file rename itself | **1** | `boot/std/mirror/grammar.mirror` → `boot/std/mirror/prism.mirror` |

`@mirror/grammar` → `@mirror/prism` is the **meta-prism rename**: the
self-describing grammar's own ref. It is in the 9-file set and must move with
the keyword.

---

## 5. The ticks — an ordered sequence to close the gap

Each tick names scope, what moves, the marker, verification, and dependencies.
The `.rs`-touching ticks carry `[substrate-pull:realize]` per the FROZEN-`.rs`
discipline. **This spec writes the ticks; it implements none of them** (Mara's
mandate is the audit + this markdown).

### Tick 1 — `grammar → prism` rename (the fracture, realized mechanically)

- **Scope:** the coupled .mirror + .rs rename. The one loaner word leaves the
  floor; `prism` takes its recognition path.
- **What moves:**
  - `.mirror` (mechanical migration, `mirror kintsugi --out <tmp> --transform
    'grammar => prism' boot/`): all 158 `grammar @x {` openers → `prism @x {`;
    all 9 `@mirror/grammar` → `@mirror/prism`; file rename
    `boot/std/mirror/grammar.mirror` → `prism.mirror` (basename rewrite). The
    legacy `focus grammar` / `focus prism` declaration lines collapse — after
    the rewrite both become `focus prism`, which `merge_keyword_sources`
    treats as same-keyword-same-op (no conflict).
  - `.rs` (`[substrate-pull:realize]`): `grammar.rs:80` block-opener
    `"grammar "` → `"prism "`; `grammar.rs:60` / `spectral.rs:559` ref
    `"@mirror/grammar"` → `"@mirror/prism"`; `spectral.rs:606` emit
    `b"grammar "` → `b"prism "`; `spectral.rs:3020/3029` combinator literals;
    update `pipeline.rs` test fixtures.
- **Marker:** 🔴/🟢 + `[substrate-pull:realize]` (the .rs recognition change is
  boundary Rust; red-first against a round-trip OID fixture).
- **Verification:** `cargo test` green; **round-trip OID check** — for a
  representative sample of the 158 files, `compute_content_oid` before (with
  `grammar`/old .rs) must equal after (with `prism`/new .rs). The rename is a
  pure relabel; OIDs of the *renamed* corpus under the *renamed* recognizer
  must be stable across a second migration pass (idempotence — the fracture
  law). `mirror craft boot` must still produce a crystal with no new Dark.
- **Dependencies:** none (this is the first tick). Honest note: this is the
  *mechanical realization* of a fracture, not a fracture-engine pass — the
  engine is a stub (§4.2).

### Tick 2 — confirm the recognized floor

- **Scope:** assert, in tests, that the bootstrap recognizes exactly the
  floor: the five ops + `in` + `out` + `prism` (now a real block-opener) +
  `glass` (parses as a top-level form), and that `lambda` is present but
  flagged transitional.
- **What moves:** test-only (`.rs` tests) + a comment-level declaration in
  `token.mirror`/`prism.mirror` naming `lambda` transitional. No new Rust
  recognition.
- **Marker:** 🔴/🟢 (tests) — no `[substrate-pull:realize]` if it adds only
  assertions; 🔧 if it touches comments in `.mirror`.
- **Verification:** a test enumerating the harvested keyword table after
  `load_grammar("boot/std/mirror/prism.mirror")` and asserting the floor set;
  `glass @x { … }` round-trips.
- **Dependencies:** Tick 1 (the rename must land first so `prism` is the
  opener).

### Tick 3 (DEFERRED) — `lambda → glass`

- **Scope:** drop `lambda` from the conceptual floor once mirror tokenizes
  itself.
- **Trigger:** the meta-glass self-host of `tokenize.rs`
  (`mirror-grammar-self-hosted.md`) lands — i.e. when the hand-written
  tokenizer no longer needs the fast `lambda` recognition for bootstrap.
  `lambda` is substrate-declared already (`focus lambda`), so this is a
  documentation/closure change, not a Rust deletion.
- **Marker:** 🔧 + `[substrate-pull:realize]` if any Rust dispatch for the
  binding form remains by then; otherwise 📝.
- **Verification:** round-trip OIDs unchanged; the floor is exactly the nine.
- **Dependencies:** the meta-glass self-host (not this round).

### Tick 4 (DEFERRED) — structural-form + is_skip_word push-to-glass

- **Scope:** move the `io`/`match`/`select` structural dispatch and the
  `is_skip_word` Rust-noise set out of `.rs` into glass. This is the *real*
  floor-creep removal — and it is the shelved `is_skip_word` tick, not a new
  one.
- **Trigger:** the meta-glass self-host (same as Tick 3).
- **Marker:** 🔴/🟢 + `[substrate-pull:realize]` per family.
- **Verification:** corpus-wide round-trip OID check (§7 risk — `is_skip_word`
  carries load-bearing words like `self`/`where`/`type`/`in`; removing them
  must not change tokenization of any grammar).
- **Dependencies:** the meta-glass self-host. Out of scope for this round; noted
  so the floor story is complete.

**Ordering rationale:** Tick 1 is the only *active* tick this round and is the
whole point (rename the one loaner word). Tick 2 pins the result. Ticks 3–4 are
explicitly deferred to the meta-glass self-host and named only so the floor's
end-state is legible.

---

## 6. Downstream doc reconciliation

The rename makes "grammar"-terminology specs drift from the substrate. These
are **follow-on doc-fracture ticks (📝)** — not part of Tick 1's blast radius,
but they must be kintsugi'd to "prism" so the corpus stays coherent. Direct
consumers of the *keyword*/`@mirror/grammar` ref (highest priority):

| Spec | Why it must move |
|---|---|
| `docs/specs/mirror-new-command.md` | uses `grammar @x` framing and `@<name>` seed-grammar language throughout (HEAD `c367512`) |
| `docs/specs/cosmos-mirror-scaffold.md` | `@cosmos` "grammar family" framing; `the .mirror grammar declares the operation` |
| `docs/specs/mirror-grammar-self-hosted.md` | titled `@mirror/grammar`; the meta-glass spec — the **most** affected; its title and every `@mirror/grammar` ref move to `@mirror/prism` |
| `docs/specs/surface-simplification.md` | the `@mirror/grammar` row in the grammar-inventory table |
| `docs/specs/parser-as-prism-grammar.md`, `generated-parser-spec.md`, `autopoietic-grammar-spec.md`, `epistemologic-grammar.md`, `code-extension-grammar.md`, `graph-native-mirror-model.md` | "grammar"-as-concept titles/bodies; lower priority, fold opportunistically |

A `grep -rl 'grammar @\|@mirror/grammar\|\`grammar\`' docs/specs/*.md` surfaces
**~50 specs** mentioning the term in some form. Most use "grammar" as the
generic CS concept (acceptable to leave), so the doc-fracture is *not* a blanket
rewrite — it targets the keyword and the `@mirror/grammar` ref, not every prose
mention. Sequence: rename the **9 substrate-ref specs** first (they describe the
thing that moved), then the keyword-decl specs, then prose opportunistically.
Each is a 📝 commit; none touches `.rs`.

---

## 7. Honest dependencies / risks

- **The kintsugi-fracture engine is a stub (§4.2).** The rename is the
  *mechanical realization* of a fracture, gated for a literal fracture-engine
  pass on the kintsugi engine landing. Do not represent Tick 1 as a fracture
  pass; it is `apply_rewrites` + a coupled `.rs` change. The fracture is the
  *principle* it realizes.
- **FROZEN-`.rs` marker discipline.** Tick 1 and Tick 4 touch `.rs`
  recognition and MUST carry `[substrate-pull:realize]`. The `.rs` change is
  boundary Rust (a recognizer edit), not new capability — per AGENTS.md
  §"Boundary Rust is not frozen capability".
- **The rename is coupled, not pure-substrate.** §0/§4.3: rewriting the
  `.mirror` files without the matching `.rs` block-opener + render-emit change
  breaks the parse and the round-trip. The two halves must land in one tick (or
  the build is red between them). Red-first against a round-trip OID fixture.
- **Round-trip OID is the safety net.** The rename is a relabel; it must not
  change the *structure* of any AST, only the keyword/tag strings. The OID is
  content-addressed over the AST, so the renamed corpus under the renamed
  recognizer must produce stable OIDs, and a second migration pass must be a
  no-op (idempotence — the fracture's first law). This is the concrete witness
  that the "fracture principle" is honoured even without the engine.
- **Floor-creep removal is NOT this round, and `type` is the load-bearing
  trap.** The audit's relief: `type`/`fixed`/`property`/`shape`/`abstract`
  are already glass (substrate-harvested), so this round does **not** push them
  out of Rust — there is no Rust to push. `type` alone is used as a keyword
  decl across **82 files**; had it been a hardcoded Rust match, removing it
  would have been the same cross-grammar load-bearing surprise the
  `is_skip_word` audit found with `self`/`where`. It is not, so we leave it
  entirely. The genuinely risky push-to-glass — `is_skip_word` (carries
  `self`, `where`, `type`, `in`, `let`, `as`, `match`, …, all load-bearing
  across many grammars) — stays **shelved/deferred (Tick 4)** and requires a
  corpus-wide round-trip OID audit before any byte moves.
- **`prism` must become a real block-opener.** Today `parse_grammar` has NO
  `prism @x {` opener (only `grammar `). Tick 1's `.rs` half is what makes
  `prism` actually open a block; until it lands, `prism`-renamed files would
  not be harvested. This is the one place the rename adds Rust *recognition*
  (moving the opener from `grammar ` to `prism `), not just relabels — still
  `[substrate-pull:realize]`, since the *capability* (open a keyword block) is
  unchanged; only the trigger word moves.

---

*The loaner word leaves. `grammar` was borrowed from parser theory; `prism` is
mirror's own. The five ops are what a prism does; glass is what it's made of;
in and out are how prisms compose. Nine words, and the substrate names the
rest of itself. The rename wants to be a fracture — and one day, when the
engine lands, it will be the worked example. Today it is the principle, run by
hand, checked by the OID that does not move.*
