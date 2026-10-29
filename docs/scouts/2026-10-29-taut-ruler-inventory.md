# Ruler inventory (Taut, 2026-10-29)

*Read-only scout. Branch `taut/ruler-inventory` off `reed/ship` (`d132f9d1`). Prism read at the pinned rev `26ed54b` (`git show`, no checkout) and at the cache `/Users/reed/.cache/mirror/prism-7e6b00f`. Ruler values are identical at both revs; only line numbers moved, from formatting (commutator `93/97 → 95/99`, `RotationConfig::default` `133–135 → 139–141`). Prism citations below are at `26ed54b`, the rev mirror pins. Test-only constants are excluded unless a spec cites them as product rulers.*

**Question (Alex, relayed):** make `Ruler` explicit; is a `Pact` a `Fractal<Ruler>`? **What this scout tests:** do the narrowings in the tree share laws (named? recorded? revocable? composable? reader-dependent?), or are several types sharing one word?

**Definition used:** a ruler is anything that narrows: a threshold, budget, ordering that picks a winner, quantization, weighting, tolerance, gate, approval or default that decides.

---

## §0 Headline numbers

81 rows: 64 in code, gates and build (prism 31, mirror Rust 25, hooks/Justfile/pins 8), plus 17 declared only in specs or shard slots.

| kind | count |
|---|---|
| threshold | 18 |
| gate | 15 |
| other (exit tables, defaults, identity roots, similarity, shard slots) | 11 |
| ordering / choice | 9 |
| quantization | 9 |
| weighting | 9 |
| budget | 6 |
| approval | 4 |

| status | all 81 | code only (64) |
|---|---|---|
| **declared** (named, addressable: a type, a parameter, a documented choice) | 38 | 21 |
| **derived** (a theorem's bound, not a choice) | 3 | 3 |
| **disguised** (bare constant, inline literal, implicit order) | 40 | 40 |

- **Every disguised ruler is in code.** All 17 spec rows are declared, and none has landed as `@ruler`. There is no `@ruler` shard, and no `ruler` keyword in `mirror.spec` (grep: zero hits in `shards/`, `boot/`, `mirror.spec`; the one mention is the comment at `Justfile:184`).
- **Outcome recorded vs. ruler recorded.** 34 of 64 code rulers leave their *outcome* somewhere (a verdict, a `Wall`, a commit). Only **5 of 64 put their own identity or value next to that outcome**: `k_max` (`Dark::Budget { k_max }`), the two FROZEN bypass markers (in the commit message), the prism pin (`Cargo.lock`), and the default start rule (`start` field). For every other ruler you can see what got narrowed but not which ruler did it.
- **Cross vs. up:** 6 of 81 rulers have a route *across* to a peer who can check them. 5 more have nobody who can overrule them, or only the actor itself. Everything else routes *up* (§3).

---

## §1 Inventory

Columns: **site** · **what it narrows** · **kind** · **D/d** (declared / disguised / derived) · **overrule route** (across = a peer can check it; up = only a higher authority; caller = whoever invokes; nobody) · **recorded?** (does the result carry the ruler's identity or value, the outcome only, or nothing).

### 1a. Prism (pinned `26ed54b`)

| # | site | narrows | kind | D/d | overrule | recorded |
|---|---|---|---|---|---|---|
| P1 | `prismqueer/src/spectral/instrument.rs:160` `ZERO = 1e-9` (× scale) | which eigenvalues of D count as zero (harmonic dimension) | threshold | disguised (private const) | up (prism edit + mirror pin bump) | silent |
| P2 | `spectral/commutator.rs:95` `NUMERICAL_ZERO = 1e-12` | singular value is zero | threshold | disguised | up | outcome: values in the band go to `Partial` |
| P3 | `spectral/commutator.rs:99` `CLEARLY_NONZERO = 1e-6` | singular value is surely nonzero (the band 1e-12…1e-6 means "don't know") | threshold | disguised | up | outcome (`Partial`) |
| P4 | `spectral/commutator.rs:111` `scale.max(1.0)` | floors the Frobenius scale at 1 | threshold | disguised | up | silent |
| P5 | `spectral/spectrum.rs:109,115` `1e-12` (absolute) | Refract: trace treated as zero; eigenvalues dropped | threshold | disguised (inline literal ×2) | up | silent |
| P6 | `spectral/settle.rs:168` `16·ε·scale·n` | kernel certification error | threshold | disguised (inline formula) | up | outcome (`Wall::Unseparated`) |
| P7 | `spectral/settle.rs:173` gap `1/(n(n−1))` | requires the error to resolve the spectral gap | threshold | **derived** (λ₂ bound) | nobody (a theorem) | outcome (`Wall`) |
| P8 | `spectral/settle.rs:48` `Noise(pub f64)` | observer resolution: modes below it are not visible | threshold | declared (newtype param) | caller | outcome (`Tail.moving/faded`); the Noise value itself isn't carried |
| P9 | `spectral/tension.rs:177` `zero_tol = 1e-9` (absolute) | zero eigenvalues when counting components / b₁ | threshold | disguised | up | silent |
| P10 | `spectral/tension.rs:212` `fiedler < expected_min * 0.5` | whether a weak-Fiedler tension is reported | threshold | disguised | up | silent (the 0.5 doesn't appear in the tension) |
| P11 | `spectral/tension.rs:182,196,213-219` normalisations, `.min(1.0)` | tension magnitudes | weighting | disguised | up | silent |
| P12 | `coincidence.rs:96,161,167,193,198,238` `f64::EPSILON` (×6) | coefficients dropped **inside `Oid::hash`** | threshold | disguised | up | silent; **touches identity** |
| P13 | `coincidence.rs:256` `FRAGILE_THRESHOLD = 1e-6` | "fragile" detection; **inert**: both branches at `:306-311` return the same `Agreed` | threshold | disguised | nobody needed | silent |
| P14 | `coincidence.rs:253,470` `DEFAULT_DIMENSION = 16` | projection dimension of the canonical content detector | quantization | disguised | up | silent; touches identity |
| P15 | `coincidence.rs:317` `quantize_weight` [-1,1] → u8 | projection weights | quantization | disguised | up | silent |
| P16 | `spectral/rotation.rs:139-141` `RotationConfig::default` K=1.0, dt=0.05, ω=0.3 | Kuramoto coupling, step, spread | weighting | declared (named struct, overridable) | caller | **silent**: the splinter address is over (phases, order_r, fiedler, tick) (`:85`), not the config |
| P17 | `spectral/rotation.rs:222` `ω·(i·0.1 + 1.0)` | per-vertex natural frequency | weighting | disguised | up | silent |
| P18 | `spectral/rotation.rs:179` `ticks: usize` | rotation budget | budget | declared (param) | caller | outcome (splinter count) |
| P19 | `spectral/rotation.rs:258` `fiedler < previous_fiedler` | Foerster gauge: strict, **no tolerance** on a float | gate | disguised (the zero tolerance is unnamed) | up | outcome (verdict) |
| P20 | `precision.rs:10` `Precision(f64)` | eigenvalues below it are zero in `project` | threshold | declared (newtype) | caller | recorded (`KernelSpec` carries it) |
| P21 | `kernel.rs:59` `l > threshold` | which logit dimensions survive | threshold | declared (param) | caller | outcome (dims list) |
| P22 | `kernel.rs:62-70` empty → keep the argmax | fallback when nothing passes | ordering | disguised | up | silent |
| P23 | `fate/mod.rs:168-176` `argmax5`, strict `>`: **first index wins ties**; `MODELS` order `:178` | which model fate picks | ordering | disguised | up | partial (distribution kept, tie not flagged) |
| P24 | `fate/mod.rs:76` `dist[4] = 0.0` | excludes Fate from `best_non_fate` | ordering | declared (fn name) | up | recorded (zeroed distribution) |
| P25 | `fate/feature.rs:62,84` `CASIMIR_EIGENVALUES`, `BERRY_PHASE = 0.847` | fate feature weights | weighting | disguised (pub const, no provenance) | up | silent |
| P26 | `flux.rs:250,311,346` `confidence: 0.5` | the confidence of every `Partial` from these arms | weighting | disguised | up | silent (looks like a measurement but is a constant) |
| P27 | `flux.rs:301,329` `theta` (algedonic) | signal vs. no signal | threshold | declared (typed param) | caller | outcome (verdict) |
| P28 | `spectral_uuid.rs:55-56,93` 48/80 split | active vs. dark bits of the uuid | quantization | declared in docs, constant in code | up | silent; `active` unsigned |
| P29 | `docs/spec/gate.md:28,60` budget 4, patience 1 | review rounds | budget | declared ("choice for bullet-up", vector-tested) | **across** (bullet-up decided; `gate-vectors.json`) | recorded (`moving` vs `kept`) |
| P30 | `docs/spec/gate.md:59` NFC word-level id | what counts as "the same objection" | quantization | declared ("the observer's boundary, written down") | **across** | recorded as rule text |
| P31 | `docs/spec/gate.md:7,62-65` fail closed only if *all* fail; failed observer = open item; no majority | gate verdict | gate | declared | **across** | recorded (items) |

### 1b. Mirror Rust (`rust/**`)

| # | site | narrows | kind | D/d | overrule | recorded |
|---|---|---|---|---|---|---|
| M1 | `engine/src/lib.rs:207,598,614` `k_max` | E's rounds | budget | declared (param) | caller | **ruler recorded**: `Stop::Budget(k_max)`, `Dark::Budget { k_max }` (`:151`) |
| M2 | `engine/src/lib.rs:764-770` Herbrand bound | ticks may not exceed \|B\| | gate | **derived** (Prop. 5.4) | nobody | recorded (`Dark::Bound`) |
| M3 | `engine/src/lib.rs:65-72` `Functional` declarations | at most one value per key | gate (a clause) | declared | program author | recorded (findings / `Dark::Functional`) |
| M4 | `engine/src/lib.rs:224-231` stop on consecutive `under` equality | well-founded termination | other (stop rule) | **derived** (Prop. 1.9) | nobody | recorded (retractions) |
| M5 | `boot/std/mirror/grammar.mirror:76,81,87` `choice(...)` | PEG alternative order: **first match wins** | ordering | disguised (by position; spec says "declared by position") | grammar author | silent (only via τ's grammar address) |
| M6 | `src/main.rs:2191` `cmd_compile` → `ExitCode::SUCCESS` | **exit 0 on every verdict, `fail` included** | gate | disguised | up | silent (escalation is printed; the exit code ignores it) |
| M7 | `src/compile.rs:237-240` first `Fail` sets `Escalate` | which failure escalates | ordering | disguised (comment only) | up | recorded (`Escalate(oid)`) |
| M8 | `src/main.rs:2509-2517` love verdict → 0 / 1 / 255 | Green/Yellow/Red | gate | declared (comment + spec) | up | printed |
| M9 | `src/love.rs:257,268` strict `λ_c > max(λ_a, λ_b)`; truncation to `min_len` | per-axis climb; **extra axes silently dropped** | threshold + quantization | disguised | up | silent (truncation unreported) |
| M10 | `src/main.rs:414,422,752,766` etc.: argv error → 2; `:453,777,…` IO → 3 | the CLI's exit table | other (exit table) | disguised | up | **collides**: spec `dark = 2`; 3 is not in the spec table |
| M11 | `src/main.rs:629-632` MCP: `ticks = 1` + `RotationConfig::default()` | one rotation per query | budget | disguised | up | silent |
| M12 | `src/main.rs:238-245` `classify` by extension | file kind | quantization | disguised | up | silent |
| M13 | `src/main.rs:264` `build_shards_sheaf`: sibling-edge-only | the substrate graph rotation reads | weighting | disguised | up | silent |
| M14 | `src/main.rs:1308-1355` `mend_at`: writes and commits `.rs` as `mirror` | **auto-approval of program edits, no human** | approval | disguised | nobody before the fact (revert after) | commit; its message (`:1397`) carries `[substrate-floor:@io-boundary]` with **no `docs/audits` path and no Seam trailer**, so `.githooks/commit-msg:80-102` would refuse it: two rulers that disagree, both silent about it |
| M15 | `roomba/src/mend.rs:273-283` overlapping arms: earliest `byte_start` wins | which arm is deleted | ordering | disguised | up | silent (dropped arm unreported) |
| M16 | `roomba/src/mend.rs:240-262` `.contains("<sentinel>")` | "this arm is redundant" (substring similarity) | other (similarity) | disguised | up | recorded (arms in the commit message) |
| M17 | `src/main.rs:1092-1093` pheromone deposit auto-commit | auto-approval of an observation commit | approval | disguised | nobody | recorded (commit) |
| M18 | `spectral/src/liquid.rs:1124-1137` Lagrange band [0.1, 0.9] in integer tenths | narcissus / splinter / pass | threshold + quantization | disguised | up | recorded (Fail message) |
| M19 | `spectral/src/liquid.rs:797` `defer` → `confidence: 0.5` | the confidence of a deferred verdict | weighting | disguised | up | silent |
| M20 | `spectral/src/liquid.rs:637,732,872` non-negative theta | domain gate | gate | disguised | up | recorded (Fail message) |
| M21 | `matrix/src/void.rs:192,196-198` 12-char OID prefix + `append(true)` | beat file identity: **a prefix collision appends two beats into one file** | quantization | disguised | up | silent |
| M22 | `src/bench.rs:73,117-119` `--samples` default 10⁶; percentile index truncation | bench sampling | budget | declared (flag) | caller | printed |
| M23 | `src/apply_h.rs:427,447` 7-line window; `:633` ≤ 2 newlines | prism-boilerplate detection; blank-line collapse | quantization | disguised | up | silent |
| M24 | `fractal/src/mandelbrot.rs:52` `GENESIS` | the first `prev` | other (default) | declared | nobody | recorded |
| M25 | `src/main.rs:2138` `Subject::mirror()` as author and committer | who witnessed a compile | other (default identity) | disguised | up | recorded (`Witnessed`) |

### 1c. Gates, hooks, pins

| # | site | narrows | kind | D/d | overrule | recorded |
|---|---|---|---|---|---|---|
| G1 | `Justfile:148-194` pre-commit INTERIM GATE: `cargo test`, default features (red-claims excluded); diff-closure short-circuit `:161-170` | what blocks a commit | gate | declared (comment, Alex 2026-10-28) | up (Alex) | silent (the commit doesn't record which tolerance applied) |
| G2 | `Justfile:197-202` pre-push: coverage "TODO" | coverage floor (absent) | gate | disguised | up | silent |
| G3 | `Justfile:256` `git merge --no-ff --no-gpg-sign` | **turns signing off for merge commits** | gate | disguised | up | silent; conflicts with "SSH signing default" (CLAUDE.md) |
| G4 | `.githooks/commit-msg:65-66,80-102` FROZEN `.rs` (tests exempt); `[substrate-floor:@io-boundary]` needs Seam | `.rs` authorship | gate | declared | **across** (Seam), but the check is a regex for *any* `docs/audits/YYYY-MM-DD-*.md` string (`:82`): the file's existence and contents aren't checked | recorded (marker in the message) |
| G5 | `.githooks/commit-msg:73-76` `[bugfix:restore]` | FROZEN bypass | approval | declared (marker) | **nobody: self-authorized**, no peer gate | recorded (marker) |
| G6 | global `commit-msg` (`/nix/store/…-git-hooks`, from `~/.os`) phase markers / sequence / 📝 | commit phase discipline | gate | declared | up (`~/.os` owner) | recorded (emoji) |
| G7 | prism rev `26ed54b` at `rust/Cargo.toml:79`, `rust/fractal/Cargo.toml:49`, `rust/matrix/Cargo.toml:34,40`, `rust/spectral/Cargo.toml:20,23`, `rust/roomba/Cargo.toml:45,49`; `flake.nix:39` hash | the floor that runs E (trust root) | other (identity pin) | declared | up (Alex 2026-10-27) | recorded (`Cargo.lock`) |
| G8 | `rust/tests/fixpoint_t1.rs:39` default start `"shard"` | which rule parses | other (default) | declared (spec §1.2′) | up | recorded (`start` field) |

### 1d. Declared in specs or shard slots, not landed as `@ruler`

| # | site | narrows | kind | overrule | recorded |
|---|---|---|---|---|---|
| S1 | `docs/specs/mirror-fixpoint-parser.md:95-101` exit table 0/1/255/2, "tolerance is a declared ruler" | exits | gate | up (Alex decided, row 20 `:522`) | n/a |
| S2 | `docs/specs/mirror-kintsugi.md:189-200` `ruler @mirror/gate` | what the gate tolerates | gate | up (human-approved) | n/a (unlanded) |
| S3 | `mirror-fixpoint-parser.md:191` stage-iteration `k_max` | `--fixpoint` diagnostic | budget | up | reports `fixed at k` / `cycle(p)` |
| S4 | `mirror-fixpoint-parser.md:445-447,514` `active` quantizer, operator, bit split | locate, never identify | quantization | **across: "signed by every issuer"** | `active unsigned` until signed |
| S5 | `mirror-fixpoint-parser.md:169,516` root name `@mirror` signed by Alex | the one identity root | other (identity root) | up (single root, by design) | signed |
| S6 | `mirror-fixpoint-parser.md:161,481` PEG alternative order as a named ruler inside dark | meaning of choice | ordering | grammar author | via τ |
| S7 | `mirror-fixpoint-parser.md:456-468` `μ_json` NFC; number precision (row 18, Alex-owned) | what a number/string means | quantization | up | the spelling goes to the erasure record |
| S8 | `docs/specs/mirror-settle.md:98-105` `settle(…, &Rulers, …)` + `Erasure` `#[must_use]` | which `Cycle` phase becomes a crystal | ordering (choice) | up (signed `@ruler`) | **ruler addr in every Erasure entry** |
| S9 | `mirror-settle.md:112-113` `similar(…, radius: &Ruler)` | rerere similarity | other (similarity) | up | `Suggestion { ruler }` |
| S10 | `mirror-settle.md:182,201` `Temperament { kernel, ruler }` | tempering out a comma | ordering (choice / quotient) | up | kernel returned as erasure |
| S11 | `mirror-settle.md:191-195,202` `Futures::weigh(&Ruler)` | weights directions, **drops none** | weighting | up | `Weighted { ruler }` |
| S12 | `docs/specs/mirror-coupling.md:38-41,52` `Offered { value, ruler, to: (Party, Party) }` | coupling strength | weighting | **across (the parties are named)** | ruler + parties |
| S13 | `docs/specs/mirror-convergence.md:39,80,179-185` advance ruler (auto-approval) | which mends skip the human | approval | up (human-signed) | `Approved-by: ruler <addr>` trailer |
| S14 | `mirror-convergence.md:93-96` rerere "similar" radius | offer scope | other (similarity) | up | distance + ruler shown |
| S15 | `mirror-fixpoint-parser.md:590` T14 | bare number in a ruler position → Opaque | gate (on rulers) | n/a | n/a |
| S16 | `mirror.spec:138` `total_transparency.weight == 0` | settle_on predicate | threshold | up | via `settle_on` |
| S17 | shard parameter slots, bodies all `\`: `tolerance: metric` (`shards/mirror/bench.mirror:270,292,363,381`; `shards/epistemologic/property/cold_compile_within_tolerance.mirror:153`), `tolerance: ref` (`shards/loop.mirror:269`), `budget: u64` (`shards/mirror/spectral/portal/stream.mirror:115`), `budget: nat` (`shards/kintsugi/surface.mirror:615`), `budget: ref` (`shards/spectral/restart_intensity.mirror:129`), `threshold: u64` (`shards/epistemologic/pact/benchmark.mirror:101`), `radius: ref` (`shards/ui/mote.mirror:154`) | various | other (slots) | n/a | n/a |

---

## §2 Duplicates (one ruler, defined more than once)

1. **Numerical zero: six definitions, three scale conventions.** `instrument.rs:160` 1e-9 × scale; `commutator.rs:95/99` 1e-12 / 1e-6 × max(Frobenius, 1) with a don't-know band; `spectrum.rs:109,115` 1e-12 absolute; `settle.rs:168` 16·ε·scale·n; `tension.rs:177` 1e-9 absolute; `coincidence.rs` `f64::EPSILON` absolute. The same graph can be "connected" to `tension` and "not separated" to `settle`. Only the commutator and settle have a third value ("don't know"). The others are two-valued and say nothing.
2. **`SpectralCoordinate<5>`: three spec definitions plus a fourth landed one** (math `2026-10-28…foundation.md:506`: fragmentation, rung-8-9, prism `spectrum.rs`; the landed one is SHA-seeded random projections). Not re-scouted; confirmed still unresolved (`mirror-fixpoint-parser.md:491`).
3. **Exit-code table: four versions.** Spec 0/1/255/2 (S1); `cmd_love` implements 0/1/255 (M8); `cmd_compile` is always 0 (M6); every verb uses 2 for argv errors and 3 for IO (M10). **2 means both "usage error" and "dark".**
4. **"Gate": five rulers under one word.** Justfile interim gate (G1), FROZEN hook (G4), the global phase hook (G6), the spec's `ruler @mirror/gate` (S2), and prism's convergence gate (P29–P31). Only prism's has written-down choices, vectors, and a route across.
5. **`confidence: 0.5` as a constant**, four times: `flux.rs:250,311,346` and `liquid.rs:797`. It reads like a measurement but is a fixed number.
6. **Prism pin: eight copies** (6 manifests + `Cargo.lock` + `flake.nix:39` hash). Consistent today.
7. **Rotation defaults** reach production twice: through `RotationConfig::default` (P16) and through the MCP hard-coded `ticks = 1` (M11).
8. **Math §9's "settle's `Noise(1e-6)`"** (`docs/math/2026-10-28-…foundation.md` §9, and `mirror-fixpoint-parser.md:484`) is a **test constant** (`settle.rs:295`, inside `#[cfg(test)]`; also `:535`). Production `settle` takes `Noise` from the caller and has no default. The ruler to declare is the *caller's* choice, and there's no production caller yet.

**Word collision: "pact" is already taken.** `shards/epistemologic/pact/` holds 16 predicate shards (`path_matches_namespace`, `gate_matches_diff_closure`, `bilateral`, …). These are verdict-returning checks the substrate holds itself to, which makes them clauses. The engine tests use `@pact/fact` as a fact altitude (`rust/engine/tests/e5_well_founded.rs:24`), and T13 says `mirror pact eval` (`mirror-fixpoint-parser.md:588`). Neither is `Fractal<Ruler>`. If `Pact = Fractal<Ruler>` lands, the current `@epistemologic/pact/*` is on the clause side of Reed's line, not the policy side.

---

## §3 Cross vs. up

| route | rows | count |
|---|---|---|
| **across** (a peer can check or co-sign) | P29, P30, P31 (bullet-up's documented choices + vectors), G4 (Seam gate), S4 (`active`: every issuer), S12 (`Offered`: both parties named) | **6** |
| **caller** (whoever invokes picks; no peer check) | P8, P16, P18, P20, P21, P27, M1, M22 | 8 |
| **program / grammar author** | M3, M5, S6 | 3 |
| **nobody, or self** | P7, P13, M2, M4 (theorems / inert); **G5** (`[bugfix:restore]`, self-authorized); **M14, M17** (mirror auto-commits; overrule only by revert after the fact); M24 | 8 |
| **up** (edit the code or spec; for prism, a pin bump through FROZEN's Seam gate) | everything else | 56 |

- In code, the only route across is the Seam gate (G4), and it checks whether a path *string* is present (`.githooks/commit-msg:82`), not the audit itself.
- **Overruling any disguised ruler in `rust/src` goes through FROZEN.** So the route to change a bare constant is the same route as adding capability: up to Seam, or sideways through the self-authorized `[bugfix:restore]`. In practice the cheapest way past a ruler is the one ruler nobody else checks.
- The spec side mostly routes up too: "approved by a human like any ruler" (`mirror-kintsugi.md:189`), "signed `@ruler`" (`mirror-settle.md:104`), `@mirror` signed by Alex. The two across routes in the specs are `active`, signed by every issuer, and `Offered`, which names both parties. Those two are also the only rulers whose spec says who is *on the other side*.

---

## §4 Grouped by the laws they obey

| family | members | named? | recorded? | revocable? | composes how | reader-dependent? |
|---|---|---|---|---|---|---|
| **Resolution** (is it zero / visible?) | P1–P10, P12, P13, P19, P20, P27, M9, M18, S16 | mostly no | outcome sometimes (commutator, settle have a don't-know band); value never | per call only for Noise, Precision, theta | **max-of-coarseness**; three-valued where honest | **yes by design** ("the observer's resolution", `settle.rs:46`) |
| **Budget** (how long) | P18, P29, M1, M11, M22, S3 | yes except M11 | **yes when done right** (`Dark::Budget{k_max}`, gate `moving`) | per call | **min**, commutative; more budget never changes a settled answer, only turns "moving" into settled | no |
| **Choice** (which one wins) | M5/S6 PEG order, P22, P23, P24, M7, M15, S8, S10 | spec yes, code no | spec: `Erasure` with ruler addr; code: silent | spec: by re-settling under another ruler | **does not commute**: order of application is the meaning (PEG is exactly this) | yes |
| **Equivalence / quantization** (what counts as the same) | P14, P15, P28, P30, M12, M16, M21, M23, S4, S7, S9, S14 | mixed | mostly silent | no (baked into identity in P12/P14/P28) | **partition meet/join** (refinement lattice) | the spec says `active` must *not* be (one ruler, every issuer) |
| **Weighting** (how much) | P11, P16, P17, P25, P26, M13, M19, S11, S12 | spec yes, code no | silent in code | — | linear-ish; **`weigh` drops nothing** (`mirror-settle.md:202`) | yes |
| **Authority / gate / approval** (who may pass) | G1–G6, M6, M8, M10, M14, M17, M20, P31, S1, S2, S13, S15 | mostly yes | marker or trailer when declared; silent in M6, M14, G3 | yes (a new signed ruler) | **conjunction**; signer-bearing | yes (who signs) |
| **Identity roots / defaults** | G7, G8, M24, M25, S5 | yes | yes | only up | don't compose: there's one of each | no |

**Meaning vs. policy** (Reed's line: meaning lives in clauses, policy in pacts):

- **Meaning** (would change what a grammar or program means, or what something *is*): PEG order (M5/S6), `μ_json` NFC and precision (S7), `@mirror` root (S5), the prism pin (G7: the floor that runs E), `Functional` declarations (M3), `settle … by:` phase choice (S8) and `Temperament` (S10), the default start rule (G8), the `Oid::hash` filters and quantizers (P12, P14, P15), the 48/80 split's dark half (P28), the beat-file prefix (M21), and **mend's sentinel match and auto-commit (M16, M14), which delete program text**.
- **Measurement** (changes what a *reading* reports, not what a program means): the Resolution family (P1–P10, P13, P19, P20, P27, M9, M18). **This is a third bin that Reed's two-way line doesn't have.** Math §9 already splits it: machine epsilon is *declared*, the multiplier on top is *signed*.
- **Policy**: budgets, gates and exits, approvals (M17, S13, G5), weightings, similarity radii (S9, S14), reporting order (M7, M15, P22–P24), `active` (S4: "locates, never identifies"), the shard slots (S17).
- **Straddling:** `settle … by:` is a policy choice that changes meaning. That's what `Erasure` is for, and it's the one place the specs already pay for crossing the line.

---

## §5 Verdict: several types under one word

**One envelope, at least five algebras.**

- **What they share:** the *envelope* the specs already give a ruler: "an `Addr` of a signed `@ruler` crystal, never a closure or a number" (`mirror-settle.md:104`), plus a record that it was applied. Every family can wear that envelope. As provenance, one `Ruler` type covers all 81 rows.
- **What they don't share:** the laws of the body.
  - **Budgets** compose by `min` and commute.
  - **Choices** don't commute; PEG order *is* the non-commutation.
  - **Resolutions** compose by coarseness, and the honest ones have three values.
  - **Equivalences** compose as partitions.
  - **Authorities** compose by conjunction and carry a signer.
  - **Weightings** narrow nothing until a choice consumes them (`weigh` "drops nothing", T4), so by Alex's definition ("anything that narrows") they aren't rulers on their own. They're inputs to a Choice.
- **The tree already disagrees with a single type.** The shard slots type the same role five ways (`metric`, `ref`, `u64`, `nat`, `ref`; S17). The code has six incompatible numerical zeros (§2.1).
- **For `Pact = Fractal<Ruler>`, observed, not designed:**
  - Rulers that "don't need to agree" fit the Budget, Resolution and Authority families: a disagreement there is a comma, and both sides can be recorded.
  - Two disagreeing **Choice** or **Equivalence** rulers don't make a comma *inside* one third. They make two results. The fixpoint spec already says this: "the same meaning settled by two rulers gives two results, each saying which ruler made it" (`mirror-fixpoint-parser.md:481`).
  - So a fractal of meaning-rulers forks the third rather than holding a comma in it. That lines up with Reed's line only if Choice and Equivalence rulers stay out of pacts and live in clauses. Today they don't: PEG order is in the grammar, and NFC and number precision are in `μ`, which is clause-side. Settle's `by:` is the exception, and it's paid for by `Erasure`.

## §6 The eight disguised rulers that matter most

1. `prism coincidence.rs:96,161,167,193,198,238`: `f64::EPSILON` filters **inside `Oid::hash`**. Identity depends on an unnamed threshold.
2. `prism spectral/instrument.rs:160` `ZERO = 1e-9`, and its five siblings (§2.1): the instrument's zero, defined six ways.
3. `rust/src/main.rs:2191`: `mirror compile` exits 0 on `fail`.
4. `rust/src/main.rs:1308-1355` (and `:1092`): `mend_at` deletes `.rs` arms and commits as `mirror` with no approval. Its own commit message would fail FROZEN's Seam gate (`:1397` vs `.githooks/commit-msg:80-102`).
5. `boot/std/mirror/grammar.mirror:76,81,87`: PEG ordered choice, a meaning ruler named only by position.
6. `prism fate/mod.rs:168-176`: `argmax5`, where the first index wins ties (with `:76` zeroing Fate).
7. `rust/src/love.rs:257,268`: strict float `>` with no tolerance, and silent truncation to `min_len` (twin: `prism rotation.rs:258`).
8. `rust/matrix/src/void.rs:192,196-198`: 12-char OID prefix in append mode, so two beats can share one file.

Next in line: `confidence: 0.5` ×4 (§2.5), and `Justfile:256` `--no-gpg-sign`.

## §7 What this scout does not do

- It doesn't define `Ruler` or `Pact` (Mara's, in parallel).
- It doesn't fix any of the rows.
- It doesn't re-derive the `SpectralCoordinate` conflict (math `:506` has it).
