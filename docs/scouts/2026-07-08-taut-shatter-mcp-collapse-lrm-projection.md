# Taut Scout — @shatter × MCP wrapper collapse: LRM projection into the beam-refactor cascade

*2026-07-08 evening. Grep-first, read-only. Under-300-word summary at bottom.*

## VERDICT

**LRM CONFIRMED — FOLD REQUIRES ONE PREREQUISITE.**

The LRM is real: tick 3 of the beam-refactor cascade (`bd837cd` §Task 3
"tick 3") is already editing `bin/mirror-mcp` for the schema rename
(`mirror_spawn` → `mirror_peer_beam` + `mirror_beam`). Folding the
`@shatter --target @data/json` collapse INTO that tick is marginally
cheap — the wrapper is 125 lines, mostly one giant TOOLS_LIST JSON
blob + a case dispatch. Custom JSON-formatting logic is 1 line (line
121, a sed-based escape). No refactor-target of size 100 or 500 lines
exists to collapse.

**The prerequisite:** the `@shatter --target <codomain-ref>` cli surface
does NOT exist today, at either substrate-decl or Rust altitude.
`mirror.spec:95-98` declares `command shatter { arg oid: content_address;
arg out: ~f }` — the `out` argument is a **filesystem path** (`~f`),
not a codomain ref. `bootstrap/src/lib.rs` has NO `"shatter"` dispatch
arm (only `compile`, `craft`, `kintsugi`, `init`, `recall`, `spawn`).
The `--out @data/json` mechanism (`bootstrap/src/lib.rs:1826-1832`,
`parse_substrate_ref_to_format`) is **kintsugi-scoped**, not
shatter-scoped. Landing the LRM fold in tick 3 requires either:
- (α) `mirror.spec` cli-block extension: `command shatter { arg oid; flag
  target: ref = @data/shatter-projection-format; arg out: ~f }` +
  bootstrap dispatch arm + `parse_substrate_ref_to_format` lift to
  shatter altitude, OR
- (β) accept that the MCP wrapper's collapse target is `mirror kintsugi
  --out @data/json` (which exists today at `bootstrap/src/lib.rs:1826`)
  rather than `mirror shatter --target @data/json` (which does not).

Reading (β) is substrate-honestly what Mara §5.4 already says: "`mirror
kintsugi --out @data/json` IS `@shatter(kintsugi_result, @data/json)` at
kintsugi altitude." The wrapper's tool-name → binary-cli-verb mapping
becomes coherent under this reading without any new substrate landing.

---

## TASK 1 — Current `bin/mirror-mcp` shape and change surface

**File size:** 125 lines total (`bin/mirror-mcp`, 8.9KB, 2026-07-07
23:20 refresh per Reed `79d3433` sequence).

**Structural anatomy:**
- Lines 1-22: header + comments (task #386 forward-promise, substrate-
  honest flag notes).
- Lines 24-27: environment setup (`MIRROR_HOME`, `MIRROR`).
- Lines 29-39: `TOOLS_LIST` heredoc — the six-tool JSON schema
  advertisement, single-line compacted at 39.
- Lines 41-125: main dispatch loop.
  - Lines 45-57: `initialize` / `notifications/initialized` / `tools/list`
    RPC framings (`initialize` at 50 is a bare JSON string echo; no
    formatting logic).
  - Lines 58-115: `tools/call` dispatch — arg extraction (lines 65-76,
    one grep-per-key), tool match (78-114), binary invocation.
  - Lines 117-122: **envelope escape** — the ONLY custom
    JSON-formatting logic in the file. One `sed` + `tr` pipeline at
    line 121:
    ```
    sed 's/\\/\\\\/g; s/"/\\"/g; s/\t/\\t/g' | tr '\n' '\r' | sed 's/\r/\\n/g'
    ```
    This is generic-string-to-JSON-string escape, NOT structured JSON
    emission. The binary's stdout is wrapped verbatim into
    `{"result":{"content":[{"type":"text","text":"<escaped>"}]}}`.

**What could collapse to `@shatter --target @data/json` dispatch:**

**Nothing at the envelope-formatting altitude.** The wrapper does NOT
construct structured JSON envelopes from mirror substrate; it wraps
mirror's stdout (whatever text or JSON the binary emitted) into the
MCP JSON-RPC transport frame. The `@shatter --target @data/json`
collapse would only kick in if the mirror binary itself started emitting
`.shatter`-formatted or `@data/json`-formatted structured payloads that
the wrapper CURRENTLY hand-formats. **It doesn't hand-format anything
substrate-structural today.**

**Real change surface, line count:**
- The TOOLS_LIST JSON blob (lines 34-36, one long line): the tool
  schema. Rename `mirror_spawn` → `mirror_peer_beam` + add new
  `mirror_beam`. **~40-column additions to a single-line JSON**, ~2
  logical additions.
- The tools/call case dispatch (lines 104-113, `"mirror_spawn"`): rename
  case label, add case for `mirror_beam`. **~10 lines mechanical**.
- Arg extraction (lines 74-76): peer_home / mission stays; if
  `mirror_beam` takes different args, add ~2 lines.

**LRM fold marginal cost estimate:** the beam-refactor tick 3 edit
touches lines ~34-36 + 104-113 (~2 case blocks, 1 schema block). Folding
the `@shatter --target @data/json` collapse in adds either:
- (α) if we chase the substrate landing: 0 lines here, but pull
  in the mirror.spec + bootstrap prerequisite (§Task 3 below).
- (β) if we collapse `mirror_kintsugi` dispatch to pass `--out
  @data/json` explicitly: ~2 lines added at line 91-94
  (`kintsugi_args+=("--out" "@data/json")`).

Reading (β) is a 2-line addition to tick 3. Reading (α) is a
substrate-decl landing that's larger than tick 3's scope.

---

## TASK 2 — Beam refactor's MCP wrapper touch surface (α vs β delta)

From `bd837cd` §Task 3, tick 3 already touches `bin/mirror-mcp`:

**(α) tick 3 without LRM fold — mechanical rename:**
- TOOLS_LIST rename `mirror_spawn` → `mirror_peer_beam`: 1 label change
  in the JSON blob at line 35.
- Add `mirror_beam` tool schema: ~40 columns added to line 35 (or a
  new line if we break formatting).
- Dispatch case rename `"mirror_spawn"` → `"mirror_peer_beam"`: 1 line
  (104).
- Add `"mirror_beam"` case: ~5-8 lines (arg extraction reuse + binary
  invocation with different positional).
- Arg-parse update if `mirror_beam` takes different args (mission-only,
  no peer_home): ~3-5 lines.

**Total ~30-line diff at wrapper altitude.** Consistent with Taut's
`bd837cd` §Task 3 estimate.

**(β) tick 3 + @shatter-collapse combined:**
- Everything in α above (~30 lines).
- Plus: pass `--out @data/json` in `mirror_kintsugi` dispatch (line 91):
  ~2 lines added.
- Plus: pass `--out @data/json` in `mirror_spawn` → `mirror_peer_beam`
  dispatch (line 105) if spawn's structured JSON envelope (`--hello-
  world` mode) collapses to substrate emission: **NOT applicable
  today** — `bootstrap/src/lib.rs cmd_spawn` at line 3835+ constructs
  the `@song` envelope itself as text, and `--hello-world` toggles
  structured JSON emission at the binary altitude, not the wrapper.
  The wrapper is already transport-transparent for spawn's JSON. Zero
  additional lines here.

**Combined tick 3 diff estimate: ~32 lines (~30 rename + ~2
`--out @data/json`).** Well under 100; well under 500.

**Substrate-honest verdict on tick-3 split:** the fold does NOT split
tick 3. The mechanical additions are of the same shape (JSON blob
edits + case dispatch edits) as the rename. Cognitive load is
comparable. **Combined tick 3 is one substrate tick.**

**Caveat:** the fold under reading (β) is "collapse dispatch verbs to
already-substrate-honest binary flags" — a *rename cascade at the flag
altitude*, NOT the deeper collapse Mara §5.1 names ("`bin/mirror-mcp`
becomes JSON-RPC framing + `@shatter` dispatch. The custom formatting
code collapses to substrate."). The deep collapse is prerequisite-
bound; the shallow collapse fits in tick 3.

---

## TASK 3 — Is @shatter callable via CLI today?

**`mirror shatter` cli surface — SUBSTRATE-DECL'd, NOT WIRED.**

At `mirror.spec:95-98`:
```
command shatter {
  # Project a settled shard to .shatter format.
  arg oid: content_address
  arg out: ~f
}
```

Two args, both positional. **No `--target` flag.** The `out` positional
is a **filesystem path** (`~f`), not a substrate ref. Substrate declares
the command; it does NOT declare a target-format-species selector.

At `bootstrap/src/lib.rs`, the dispatch match (`args[1].as_str()`, lines
3002-3050) has arms for: `compile`, `craft`, `kintsugi`, `init`,
`recall`, `spawn`. **NO `"shatter"` arm.** The binary today does not
respond to `mirror shatter <oid> <out>` at all — the substrate has
declared it in mirror.spec (Reed's cli-block tick landed 2026-07-07),
but `cmd_shatter` is not implemented.

**`shards/mirror/lens/cli/shatter.mirror`** (7.5KB, 2026-06-12) declares
the `stage @mirror/lens/cli/shatter` prism at substrate altitude. The
action signature at the stage altitude carries `(oid: content_address,
out: ~f)` — matching `mirror.spec`. No `target: ref` parameter.

**The `@shatter --target @data/json` cli surface does NOT exist today.**
Neither at substrate-decl nor at Rust altitude. To get it, the substrate
landing is:

1. `mirror.spec` cli-block extension: `command shatter { arg oid; flag
   target: ref = @data/shatter-projection-format; arg out: ~f }` (default-
   valued flag preserves backward-compat for existing shatter callers).
2. `bootstrap/src/lib.rs` adds `"shatter" =>` dispatch arm, lifts
   `parse_substrate_ref_to_format` (currently kintsugi-scoped at
   `:1826-1832`) to a general target-format-species resolver.
3. `shards/mirror/lens/cli/shatter.mirror` extends its action signature
   to include `target: ref`.

**This is a substrate landing DEPENDENCY on the LRM fold.** Under
reading (α) of the LRM (deep collapse), tick 3 requires this
prerequisite. Under reading (β) (shallow collapse via kintsugi
`--out @data/json`), no prerequisite — the substrate landing is
substrate-pull-correct when the second consumer needs a non-identity
target at shatter altitude (Mara §9 Q4 forward-promise).

**Adjacent existing wiring:** `mirror kintsugi --out @data/json` at
`bootstrap/src/lib.rs:2942-2955` DOES accept substrate refs today (via
`dispatch_out_substrate_ref` at :1873-1899); `parse_substrate_ref_to_
format` maps `@data/json` → `CiFormat::Json`. This is Mara §5.4's LANDED
example. Reading (β) leverages this existing surface without new
substrate.

---

## TASK 4 — @mcp.serve deferral status

**Reading `docs/loop/phase-h-deferral-2026-07-08.md`:** the deferral log
covers composition_pieces #5 (`spectral/supervisor.start_child`) and #6
(`@fate.roll`). It does NOT explicitly defer `@mcp.serve` as a
composition piece — that's a task-tracker item (task #386, referenced
in `docs/loop/CURRENT.md:246-249`: "Bash wrapper: `bin/mirror-mcp` still
bash (task #386 lift partial); MCP tool list advertises
`mirror_compile`/`mirror_craft`/`mirror_kintsugi` but NOT
`mirror_spawn`").

**CURRENT.md line 340-344 states the two-tick target:**
> "Task #386's `@mcp.serve` lift reads the cli-block to synthesize this
> — two-tick target."

**Substrate state of `@mcp.serve`:** LANDED at `boot/std/mcp.mirror`
(723B, 2026-05-20 22:21). The grammar declares:
```
grammar @mcp {
  type request = { method: text, params: json, id: json }
  type response = { result: json, id: json }
  serve -> imperfect {
    @io.read(stdin) |> @data/json.parse |> dispatch
                    |> @data/json.emit |> @io.write(stdout)
  }
  dispatch(request) -> response { \ }
  tools -> json { \ }
}
```

The substrate ALREADY declares:
- `serve` action as a pipeline: read → parse → dispatch → emit → write.
- `dispatch(request) -> response` as a `\`-crack (substrate-decl'd but
  discharge-form-undecided).
- `@data/json.parse` and `@data/json.emit` as the linearization pair
  IN the serve pipeline itself.

**This IS Mara §4.2's `@shatter(_, @data/json)` shape at the substrate-
decl altitude — it just uses the pre-recognition-#113 vocabulary
(`@data/json.parse` / `@data/json.emit`) rather than `@shatter(shard,
@data/json)`.**

**Two candidate readings from the task prompt:**

- **(Complement) reading.** The LRM tick collapses the wrapper's tool-
  name-to-binary-flag translation surface (~32 lines per §Task 2); the
  transport lift (bash → binary `@mcp.serve`) happens later at task
  #386.

- **(Preempt) reading.** The LRM tick collapses so much of the wrapper
  that the transport lift is trivial and can happen this tick.

**Verdict: complement holds.** The wrapper's 125-line size splits into:
- ~40 lines schema advertisement (TOOLS_LIST heredoc) — NOT reducible
  without substrate-side schema synthesis (task #386's "reads the
  cli-block to synthesize this").
- ~30 lines JSON-RPC framing (initialize / tools/list / tools/call
  response envelopes) — substrate-decl'd at `@mcp.serve` pipeline but
  not implemented in bootstrap.
- ~45 lines dispatch (arg extraction + case match + binary invocation)
  — the substrate-decl for this IS the `dispatch(request) -> response`
  `\`-crack at `boot/std/mcp.mirror`; discharge-form is
  cli-block-synthesized (task #386).
- ~10 lines envelope escape (line 121) — collapses under `@shatter(_,
  @data/json)` at the wrapper altitude, but requires the substrate to
  have an escape primitive.

The LRM tick can flip **~2 lines** (pass `--out @data/json` to
`mirror_kintsugi`) without preempting task #386. It cannot flip the
40+30+45+10 = 125 lines without the task #386 lift.

**Mara §5.1 refined:** the "clearer" from Mara §5.1 lands as a naming
recognition, not a wrapper collapse this tick. Task #386 remains the
lift moment; the LRM fold is a naming-shift + a 2-line dispatch clean-up.

---

## TASK 5 — Silent conflicts

### 5a. Fate crate wiring (task #38 clean; Reed `b4bf336`) — PRESERVED

`bootstrap/Cargo.toml` (7.3KB) does NOT declare a `fate` crate
dependency. Search for `use fate|extern crate fate|CompiledFateRuntime|
fate::select` across `bootstrap/**/*.rs` returned ZERO matches. The
fate crate reached clean end-to-end at `/Users/alexwolf/dev/projects/
fate/` per prompt reference, but it is NOT plumbed into the bootstrap.
The @shatter fold in the MCP wrapper does not touch this plumbing —
the wrapper dispatches to `~/.local/bin/mirror`, and mirror does not
call into fate today. `mirror_beam` tool's forward-promised eventual
dispatch to `fate::CompiledFateRuntime::select` for Shape B substance
is downstream of both the beam refactor AND the fate crate becoming a
bootstrap dep. **No conflict; no interaction.**

### 5b. `mirror craft --target-kind` / `mirror kintsugi --out` existing surfaces — PROMOTED

Per `mirror.spec:107-110`:
```
command craft {
  arg target: ~d
  flag target_kind: str = "binary"
  flag reflect: bool = false
}
```

Per `mirror.spec:87-90`:
```
command kintsugi {
  arg spec: ~f = ~f'./mirror.spec'
  flag target: list(str) = []
  flag emit_shatter: bool = false
}
```

Mara §5.3 names `mirror craft --target-kind` as an EXISTING
`@shatter(craft_shard, @code/{binary,rust,gleam})` at cli altitude;
§5.4 names `mirror kintsugi --out @data/json` as EXISTING
`@shatter(kintsugi_result, @data/json)` at kintsugi altitude. Both
LANDED per Reed `59c7fd0` (target-kind rename) and Reed 2026-06-16
substrate-pull (`--out` accepts substrate refs).

**The LRM fold PROMOTES both cli-block declarations semantically:**
their existing flag/arg-space maps onto @shatter's target-format-species
codomain (Mara §4.1 Claim 1). No cli surface change. The naming
recognition adds coherence retroactively. **No conflict; consistency
improves.**

### 5c. Second-instance witness for `@data-and-@code-namespaces-ARE-@shatter-codomains` (§5.5)

Per Mara §5.5 sub-recognition and §9 Q4: promotion trigger is "the
second concrete `shards/data/*.mirror` file appears (small-consolidation
rule)."

The LRM fold's proposed wrapper collapse (whether reading α or β) does
NOT land a new `shards/data/*.mirror` file. It touches:
- (β): `bin/mirror-mcp` (2-line addition, non-substrate).
- (α, if pursued): `mirror.spec` + `bootstrap/src/lib.rs` +
  `shards/mirror/lens/cli/shatter.mirror` — substrate-decl motion at
  the cli-block altitude and stage altitude, but no new `shards/data/*
  .mirror` species.

**The LRM fold is NOT a second-instance witness for #5.5 sub-recognition
promotion.** It is a same-altitude repetition of the MCP-wrapper witness
(`bin/mirror-mcp`'s `--out @data/json` addition is the SAME MCP altitude
that Mara §5.1 already cites). The second-instance witness that would
trigger #5.5 promotion is a NEW altitude — e.g., the LSP wrapper
(§5.2) or the HTTP wrapper (future) or a concrete `shards/data/lsp.mirror`
landing.

**Substrate-honest posture:** LRM fold stays a first-instance
witness at MCP altitude. #5.5 sub-recognition remains forward-
promised; small-consolidation rule NOT triggered.

---

## TASK 6 — Cascade impact projection

**Original 5-tick cascade per `bd837cd` §Task 3:**

- Tick 1: subcommand nesting admissibility in `shards/mirror/lens/
  cli.mirror` grammar (grammar-extension).
- Tick 2: `shards/mirror/spawn.mirror` → `shards/mirror/peer/beam.mirror`
  (path-namespace move + composition binding updates).
- Tick 3: `mirror.spec` cli-block + `bootstrap/src/lib.rs cmd_spawn` →
  `cmd_peer_beam` dispatch + tests rename + `bin/mirror-mcp` tool
  schema rename.
- Tick 4: docs sweep across 12+ consumers.
- Tick 5 (optional): `mirror beam <mission>` top-level anonymous variant.

**Modified with LRM fold (reading β, shallow):**
Same 5 ticks. **Tick 3 gains ~2 lines** (`kintsugi_args+=("--out"
"@data/json")` in `bin/mirror-mcp` at line ~91). Tick count unchanged;
sequence unchanged; no new prerequisite.

**Modified with LRM fold (reading α, deep):**
Requires a NEW prerequisite tick **before** the 5-tick cascade starts:

- Tick 0 (NEW prerequisite): `mirror.spec` cli-block extends `command
  shatter` with `flag target: ref = @data/shatter-projection-format`.
  `bootstrap/src/lib.rs` gains `"shatter" =>` dispatch arm +
  `parse_substrate_ref_to_format` lifted to shatter altitude.
  `shards/mirror/lens/cli/shatter.mirror` stage-altitude action gains
  `target: ref` parameter. `bootstrap/tests/shatter_*.rs` — new RED-
  first tests. Documented cascade: `docs/specs/shatter-cli-target-
  parametric.md`.
- Then the 5-tick cascade resumes.

**Ticks 1-5 as-is + tick 0 new = 6-tick cascade under reading α.**

**Reading α trade-off:** wraps the LRM fold in a substrate landing that
IS the deep-collapse Mara §5.1 names. **Substrate-honestly discharges
the recognition candidate.** But adds a tick and expands the scope
beyond the beam refactor's original grammar concerns.

**Reading β trade-off:** the shallow fold in tick 3 is coherent but does
NOT discharge Mara §5.1's deep-collapse promise. `bin/mirror-mcp` remains
a bash wrapper carrying JSON-RPC framing + escape logic UNTIL task #386's
`@mcp.serve` lift lands. **Two-tick discipline preferred:** shallow now,
deep at task #386 vicinity.

---

## Top-3 signals

1. **The `@shatter --target @data/json` cli surface does NOT exist
   today.** `mirror.spec:95-98` declares `command shatter { arg oid; arg
   out: ~f }` — no `target: ref` flag. `bootstrap/src/lib.rs` has NO
   `"shatter"` dispatch arm (only compile/craft/kintsugi/init/recall/
   spawn). The `--out @data/json` substrate-ref parser
   (`parse_substrate_ref_to_format` at `:1826-1832`) is kintsugi-
   scoped. Deep collapse (Mara §5.1) requires a substrate landing
   prerequisite; shallow collapse leverages `mirror kintsugi
   --out @data/json` today.

2. **`bin/mirror-mcp` custom formatting is 1 line (line 121), not
   40/100/500 lines.** The wrapper is transport-transparent for
   binary stdout; the ONE custom formatting site is a generic
   string-to-JSON-string escape (sed + tr). Deep collapse under Mara
   §5.1 requires substrate-side JSON emission via `@data/json.parse`/
   `@data/json.emit` per `boot/std/mcp.mirror:12` — that is task
   #386's `@mcp.serve` lift, not tick 3's scope.

3. **`@mcp.serve` substrate IS declared at `boot/std/mcp.mirror`
   (2026-05-20, 723B).** The grammar declares `serve` as a pipeline
   using `@data/json.parse |> dispatch |> @data/json.emit`. The
   dispatch action is a `\`-crack (undischarged). The wrapper's
   collapse target is real substrate; the discharge is task #386's
   lift. LRM fold is complement to task #386, not preempt: tick 3
   flips ~2 lines of kintsugi `--out` addition; task #386 flips the
   rest.

---

## Under-300-word summary

**LRM CONFIRMED — FOLD REQUIRES ONE PREREQUISITE.** Alex's Last-
Responsible-Moment framing is honest: the beam-refactor cascade's
tick 3 already touches `bin/mirror-mcp` for the `mirror_spawn` →
`mirror_peer_beam` + `mirror_beam` schema rename. Folding the `@shatter
× MCP` collapse INTO that tick has TWO readings:

**Reading β (shallow, no prerequisite):** pass `--out @data/json` to
`mirror_kintsugi` dispatch in the wrapper (2-line addition at line ~91
of `bin/mirror-mcp`). Substrate-honestly uses the LANDED `parse_
substrate_ref_to_format` at `bootstrap/src/lib.rs:1826-1832` (kintsugi-
scoped). Coherent with Mara §5.4. **Fits tick 3; does NOT discharge
Mara §5.1's deep collapse.**

**Reading α (deep, one prerequisite):** requires a NEW tick 0 before
the 5-tick cascade — `mirror.spec` cli-block extension to add
`flag target: ref` on `command shatter`, plus `bootstrap/src/lib.rs`
adding the missing `"shatter"` dispatch arm, plus lifting `parse_
substrate_ref_to_format` from kintsugi-scoped to shatter-scoped, plus
`shards/mirror/lens/cli/shatter.mirror` stage-altitude signature
extension, plus RED-first tests, plus a spec doc.
`bootstrap/src/lib.rs` currently has NO `"shatter"` dispatch (only
compile/craft/kintsugi/init/recall/spawn). **Extends cascade to
6 ticks; discharges Mara §5.1's deep collapse; second-instance witness
for #5.5 is NOT triggered because it's still MCP altitude.**

Taut-recommendation: **reading β for this cascade.** Two-tick
discipline: shallow collapse now (tick 3 +2 lines), deep collapse at
task #386's `@mcp.serve` lift (already substrate-declared at
`boot/std/mcp.mirror:12`; the wrapper's 125 lines split cleanly
into ~40 schema + ~30 framing + ~45 dispatch + ~10 escape, all
substrate-decl'd or forward-promised at that lift). Fate crate
plumbing unaffected (`bootstrap/Cargo.toml` has NO `fate` dep);
`mirror craft --target-kind` and `mirror kintsugi --out` are PROMOTED
retroactively as existing @shatter-dispatch-shaped consumers per Mara
§5.3 + §5.4 (no cli-surface change).

Not a substrate re-scout: subcommand nesting per Recognition #35
cli-as-prism is landable ground truth. `@onto` NOT proposed as
family-root. No `--no-verify`.
