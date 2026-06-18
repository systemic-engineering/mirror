# Seam Pre-Loop Review — Self-Improving Mirror MCP + LSP Cascade (2026-06-18)

*2026-06-18. Seam. Adversarial review of Reed's proposed /loop BEFORE it
fires. The 33-tick @cyberpunk recursion-lock audit just closed
(11 species substrate-fact; #63, #67, #70 promoted). Reed is about to
launch a self-improving mirror MCP + LSP cascade with `just rebuild` as
the load-bearing forcing function. This review attacks the plan.*

Status: **FIRE-WITH-AMENDMENTS.** Zero structural BLOCKERs.
Four SHOULD-FIXes. Three ADVISORYs. Six VERIFIEDs.

Branch: `seam/pre-loop-mirror-mcp-lsp-review` (off main).
Sign: Seam <seam@systemic.engineer>.

---

## §1 — Executive verdict

**FIRE-WITH-AMENDMENTS.**

The loop's premise is sound: the substrate has named four lenses
(`@mirror/lens/{cli,shell,mcp,lsp}` per `the-convergence.md` §1.2);
two of them (mcp, lsp) have family-header-only shards with empty
action bodies awaiting consumer pull; the proposed loop IS that
consumer pull. The substrate-pull discipline is in the architecture's
favor.

But the brief as proposed has four operational weaknesses that — based
on the prior 33-tick loop's hard-earned discipline (recognitions #73,
§16.1, §17.1, §22.2) — will produce stalls, false-positive verification,
and an unverifiable termination condition. Address the four
SHOULD-FIXes and the loop fires clean.

**The single load-bearing objection that stands:** `just rebuild`
**does not exist** as a recipe. The Justfile has `just build` (release
build) and `just install` (build + install to `~/.local/bin/mirror`).
The brief's central forcing-function vocabulary is hollow at the
literal-recipe altitude. SHOULD-FIX 1 amends.

---

## §2 — Per-attack-vector findings

### §2.1 Attack vector 1 — Termination condition

**Claim under attack**: "terminates when spawn lifts into mirror
(substrate-self-hosting at agent altitude)."

**Finding 2.1.A**: 🟡 SHOULD-FIX — Termination condition is empirically
unverifiable as stated.

The brief says termination is when Reed can replace
`Agent({subagent_type: "general-purpose"})` with
`mirror MCP({tool: "spawn"})` AND mirror-hosted spawn produces
"equivalent substrate-truth." Three failure modes:

1. **"Equivalent substrate-truth" is unmeasurable.** Substrate-truth
   IS what the audit produces; comparing two audit runs requires
   either (a) running both and comparing recognitions promoted +
   shards landed (extremely expensive; the @cyberpunk loop ran ~16
   hours and 33 ticks), or (b) a proxy metric (which collapses the
   thing under measurement).
2. **The substitution test is single-tick.** "NEXT tick brief" can
   succeed on a contrived easy brief and fail on a hard one. The
   prior loop showed (§17.1 / recognition #73): dissolution-shaped
   work lands while novel-synthesis-shaped work stalls. A mirror-hosted
   spawn might handle dissolution ticks and stall on synthesis ticks
   — the loop would terminate on the easy case and break on the hard
   case in production.
3. **The loop can run indefinitely.** Without a falsification
   criterion, the loop has no upper bound — at 33 ticks the
   @cyberpunk loop showed `saturation signal` (§17.3) but
   re-invocation kept it going. The proposed loop has the same shape.

**Amendment**: Reed must name a **falsification criterion** — a
specific tick-shape that, if the mirror-hosted spawn cannot handle,
constitutes "not yet equivalent." Suggested: a tick brief drawn from
the @cyberpunk audit itself (replay a known-hard novel-synthesis tick
like @kintsugi tower walk §17). If mirror-hosted spawn stalls on a
tick general-purpose handled, termination is false. If general-purpose
also stalls (§17 case), the test is uninformative — pick a different
replay tick.

**Also**: name a **maximum tick count** as backstop (e.g. 25 ticks).
At maximum-tick, the loop closes regardless of termination claim;
substrate-truth surfaces what landed vs what was promised.

### §2.2 Attack vector 2 — Agent-in-flight discipline

**Claim under attack**: implicit; the prior loop's `feedback-loop-always-agent-in-flight`
says always keep one agent in flight between Reed's ticks.

**Finding 2.2.A**: 🟡 SHOULD-FIX — `just rebuild` between ticks
**invalidates** in-flight agents.

The prior @cyberpunk loop's agent-in-flight discipline relied on
agents operating on **substrate source** (shards, specs) where in-flight
work composed with Reed's recognition work. The proposed loop's
artifacts are **binary changes** — when an agent lands a new MCP tool
or LSP method to `bootstrap/src/` and `just build` runs between ticks,
ANY in-flight agent that was about to verify or extend the previous
binary's behavior is operating against a stale artifact.

Three sub-failure modes:

1. **In-flight Mara verifies against stale binary**: she pulls
   `~/.local/bin/mirror`, tests the new tool, the test passes
   against the in-flight binary but Reed's next tick already
   replaced it. Verification is wrong.
2. **In-flight Taut benches stale code path**: profiling delta
   measurements come from a binary that no longer exists in main.
3. **In-flight Seam reviews against a moving target**: adversarial
   review of work that already shifted.

**Amendment**: redefine agent-in-flight discipline for this loop:

- Agents now own a tick's **entire lifecycle**: identify → land →
  rebuild → verify → return. The tick is the atomic unit; agents
  do not span across `just rebuild` boundaries.
- The "in-flight" discipline reads as **parallel sub-tracks**:
  Track A (the next MCP/LSP improvement) and Track B (complementary
  work that doesn't touch bootstrap/, e.g. spec writing, prior-art
  lookup, math doc amendments). Track B can run in parallel without
  binary-invalidation.
- Or: pause Track B at `just build` boundary and resume on the new
  binary. Explicit handoff, not silent assumption.

The prior loop's feedback memory needs an addendum that specifies the
distinction. The substrate-pull-natural amendment is "loop-always-agent-in-flight
applies to substrate-shard work; for binary-changing work, agents own
the tick atomically." Reed should land this addendum BEFORE firing.

### §2.3 Attack vector 3 — `just rebuild` as forcing function

**Claim under attack**: "`just rebuild` to refresh the binary, verify
via the new binary's tools" is the load-bearing forcing function.

**Finding 2.3.A**: 🔴 → **demoted to 🟡 SHOULD-FIX**: `just rebuild`
recipe does not exist. After review, this is a vocabulary correction,
not a structural BLOCKER.

✅ **VERIFIED**: the Justfile (`/Users/alexwolf/dev/projects/mirror/Justfile`)
has `just build` (release build at line 110-111 in current state), `just
install` (build + install at lines 117-122), and `just craft-binary`
(self-hosted via `craft --target binary` at lines 209-211). **There is
no `just rebuild`.** The verb does not exist.

This is a 30-second fix: either Reed (a) means `just build && just install`
chained (the substrate-pull-natural reading), or (b) means `just install`
alone (since install depends on build), or (c) means a new recipe to
add. Brief should disambiguate.

The substrate-pull-natural reading: **`just install`** is the verb. It
builds the release binary AND installs it to `~/.local/bin/mirror`,
which is what the MCP wrapper `bin/mirror-mcp` invokes (per the
`MIRROR="${MIRROR_BIN:-$HOME/.local/bin/mirror}"` line in `bin/mirror-mcp`).
Without the install step, the rebuilt binary lives at
`bootstrap/target/release/mirror` but the MCP wrapper keeps invoking
the old one. The forcing function is incomplete without install.

**Amendment**: brief reads `just install` (or `just build && just install`).
NOT `just rebuild`. Taut owns the Justfile; if Reed wants a `rebuild`
alias verb for ergonomics, Reed flags Taut for a one-line addition
(out of scope for this loop).

**Finding 2.3.B**: 🟡 SHOULD-FIX — `just install` succeeding is **not
sufficient** verification that the MCP/LSP capability is real.

A successful build proves the Rust compiles. It does NOT prove:
- The new MCP tool is registered in `tools/list` response of `bin/mirror-mcp`
  (a separate JSON-RPC handler that hand-codes the tool list — see
  the `case "tools/list"` block).
- The new tool's `tools/call` dispatch actually wires to a code path
  (could be a stub returning empty).
- The capability is observably different from the previous binary
  (could be dead code).

Three concrete failure modes:

1. **Tool registered but not dispatched**: agent adds a `mirror_focus`
   tool to the `tools/list` response in `bin/mirror-mcp` but forgets
   to add a case in the `tools/call` switch. Build succeeds; tool
   call returns "unknown tool"; loop's "verify" step gives a false
   green if it only checks `tools/list`.
2. **Mocked handler**: dispatch returns hardcoded mirror-text without
   touching the new code path. Verification by string-matching the
   response passes; the code path is never exercised.
3. **Bash wrapper not Rust**: the current `bin/mirror-mcp` is a **bash
   wrapper**, not Rust. New tools land in `bin/mirror-mcp` shell, not
   in `bootstrap/src/`. `just build` then verifies nothing about the
   MCP layer — the bash file is uncompiled. This is the silent
   failure mode most likely to occur on tick 1.

**Amendment**: verification must include:

- `bin/mirror-mcp` initialize handshake succeeds via the new binary
  (i.e. spawn mirror-mcp, send `initialize`, get back `serverInfo`).
- `tools/list` includes the new tool's name with a non-empty
  description.
- `tools/call` on the new tool against a substrate fixture returns
  a non-stub result that ALSO would have failed on the previous
  binary. (A diff against pre-tick binary's behavior, not just a
  presence check.)
- The verification target is **a fixture in the repo**, not freeform.
  Add `tests/mcp_fixtures/` for this loop. Each tick's new tool gets
  a fixture; the fixture IS the regression suite.

Without this, the loop's "rebuild + verify" step is ceremony, not
verification. The prior @cyberpunk loop kept this honest by landing
shards that the boot/craft compiler could exercise (`mirror craft boot`
either compiled or didn't). This loop needs an equivalent forcing
function at the MCP/LSP altitude.

### §2.4 Attack vector 4 — Substrate-pull-picking will stall

**Claim under attack**: "use the current MCP + LSP to identify one
substrate-pull-natural improvement, land it in mirror source."

**Finding 2.4.A**: 🟡 SHOULD-FIX — Early ticks are novel-synthesis-shaped
and will stall per recognition #73.

The prior loop's recognition #73 (§17.1): *novel-synthesis recognition
claims from within the audit's own discipline are stall-prone;
dissolution-into-vocabulary claims are not*. Three of the @cyberpunk
loop's last six ticks stalled (Glint §16, @kintsugi tower §17, two
conversation ticks §28); all were novel-synthesis-shaped.

Early MCP/LSP improvements are predominantly novel-synthesis-shaped:
- "Add `mirror_focus` MCP tool" requires designing the tool args, the
  return shape, the dispatch path. Novel synthesis at the tool-shape
  altitude.
- "Add `publishDiagnostics` LSP method" requires designing the
  diagnostic structure, the eigenboard payload shape, the LSP
  notification flow. Novel synthesis at the LSP-method altitude.
- "Add codeAction for kintsugi" requires designing the action
  representation, the workspace edit shape. Novel synthesis.

Middle ticks (auto-formatter) get easier because the substrate already
declared the property/fracture pattern. Late ticks (orchestration)
require new synthesis.

**Amendment**: Reed must build a **dissolution-first picking
discipline**. For each tick's improvement candidate, ask:

- Does the improvement IS already named in `the-convergence.md` §2.1's
  composition table? (Each row is "one algebraic expression, four
  notations" — these are dissolutions, not novel syntheses.)
- Does it map to a shard action body that's currently `\` (unresolved)?
  `shards/mirror/lens/mcp.mirror` has `tool(name, args: ref) -> mcp { \ }`
  and `dispatch(call: ref) -> mcp { \ }`. Both are dissolution-ready.
- Does it have prior-art in `bin/mirror-mcp` shell that just needs
  lifting to substrate / Rust?

If the answer to any of the three is yes: dissolution-shaped; safe
to pick. If novel synthesis is required: defer or Reed walks inline
(per the prior loop's pattern).

**Concrete suggestion**: tick 1's improvement is **lift the bash
wrapper `bin/mirror-mcp` into Rust** — the `tools/list` response and
the `tools/call` dispatch table both already exist; lifting them from
bash to Rust is dissolution (the shape is named; the substrate
already had the words). This avoids the early-tick stall.

### §2.5 Attack vector 5 — Current MCP + LSP surface

**Finding 2.5.A**: ✅ VERIFIED — the MCP server is a bash wrapper.

`/Users/alexwolf/dev/projects/mirror/bin/mirror-mcp` is a bash script
(4.2KB, 145 lines). It hand-rolls JSON-RPC parsing via grep and sed.
It registers three tools (`mirror_compile`, `mirror_craft`,
`mirror_kintsugi`) and dispatches to `~/.local/bin/mirror` subcommands.
There is no Rust MCP server in `bootstrap/src/`. There is no
JSON-RPC library dependency in `bootstrap/Cargo.toml`.

This is **maximally room-to-grow** territory. Three tools today,
hand-rolled bash; the loop has substantial substrate distance to
cover before MCP capability is even at parity with the substrate's
declared `@mirror/lens/mcp` shape (which the shard at
`shards/mirror/lens/mcp.mirror` declares as FAMILY-HEADER ONLY with
empty action bodies).

**Finding 2.5.B**: ✅ VERIFIED — LSP does not exist.

There is no LSP server. Zero LSP-related files (search:
`mcp|lsp|MCP|LSP|tower-lsp|language_server|tool_call|jsonrpc` over
`bootstrap/src/**/*.rs` returned zero matches). The shard
`shards/mirror/lens/lsp.mirror` declares the surface; no
implementation exists. `the-convergence.md` §1.2 even notes
*"`@mirror/lens/lsp` is substrate-generated via the `@code/rust/lens-server`
macro shim — there is no `tower-lsp` dependency in the runtime crate."*
This is the substrate naming what the loop must produce.

**Finding 2.5.C**: ✅ VERIFIED — convergence spec §2.1 names the
algebraic map for every porcelain verb across all four lens surfaces.

Twelve rows of (cybernetic verb → λsh sequence → MCP tool call →
LSP method). This is the **dissolution map**. Every improvement the
loop should land is already named at substrate altitude in
`docs/specs/the-convergence.md` §2.1. Reed's loop is consuming a
pre-named architectural plan, not designing one.

**This is the loop's strongest substrate-pull foundation.** Any
improvement that maps to a row of §2.1 is dissolution-shaped (the
substrate already had the word). The loop should explicitly cite
this table on every tick: "tick N improvement = row M of §2.1."

### §2.6 Attack vector 6 — The recursion claim

**Claim under attack**: "the loop recursively improves MCP + LSP
capability." But recursion needs base case + successor.

**Finding 2.6.A**: 🟢 ADVISORY — the recursion is real but
under-specified.

Reed's brief doesn't name the successor function. The substrate-pull
reading: each tick's MCP/LSP improvement increases the **vocabulary
the loop can use to identify the next improvement**. Concretely:

- Tick 1 lands `mirror_focus` MCP tool. Tick 2 uses MCP `mirror_focus`
  to inspect shards and find the next improvement candidate.
- Tick 3 lands `mirror_project @mirror/lens/lsp` tool. Tick 4 uses MCP
  `mirror_project` to filter substrate by lens family.
- Tick N lands MCP/LSP `spawn` capability. Tick N+1 uses it for tick
  N+2's brief.

The successor function = "the next tick's brief is generated using
strictly-more MCP/LSP capability than the previous tick." This is
the **same shape as peer-cognition's vocabulary-expansion mechanism**
(`architecture-peer-learns-by-crystal-vocabulary-expansion`): each
settlement extends the classifier's vocabulary. The MCP + LSP IS the
classifier's vocabulary at agent altitude.

**Amendment**: Reed cites this in the brief. The recursion is
**vocabulary-expansion-shaped, not weight-update-shaped**.

### §2.7 Attack vector 7 — Seam-review cadence

**Claim under attack**: "Seam reviews interlaced at Reed's judgement
on high-leverage ticks."

**Finding 2.7.A**: 🟡 SHOULD-FIX — "Reed's judgement" is too vague
given the prior loop missed a needed review at §11 (the carrier
extension activated; no Seam review fired).

The prior 33-tick loop had ONE Seam review (tick 4 blocker review).
The substrate-pull resolution at tick 5 dissolved the blockers — but
ONLY because Alex prompted with the substrate-pull frame. Without
that prompt, the loop would have proceeded with the BLOCKERs's
weaker resolution. A second Seam review at the carrier-extension
activation (§11) would have surfaced this earlier.

**Concrete amendment**: fix four explicit Seam-review trigger criteria
in addition to "Reed's judgement":

1. **Pre-tick-1 Seam review** (this review). ✅ done.
2. **At first cross-cutting change** (any tick that modifies more
   than one lens — e.g., a tick that touches both MCP and LSP, or
   both Rust and shard).
3. **At first novel-synthesis tick** (per recognition #73; defined
   as a tick where the improvement is NOT mappable to a row of
   `the-convergence.md` §2.1).
4. **At termination claim** (the tick that proposes "spawn lifts to
   mirror"). This is the load-bearing substrate verification; it
   must not be Reed-unilateral.

Beyond these four, Reed's judgement applies. The four criteria are
the **minimum cadence floor**; Reed can review more freely.

### §2.8 Attack vector 8 — Spawn lift verification

**Finding 2.8.A**: 🟡 SHOULD-FIX — the brief's termination tick must
include the falsification criterion from §2.1.

When the loop "terminates," the brief says Reed declares termination.
Per §2.7 amendment, Seam reviews at the termination claim. The Seam
review verifies:

- The mirror-hosted `spawn` tool handles the replay-tick
  (falsification criterion from §2.1) producing equivalent
  substrate-truth.
- The mirror-hosted `spawn` tool's verification fixture exists in
  the repo and runs in CI.
- The general-purpose Agent equivalence is **not assumed** — Reed
  runs the replay tick on both and compares the recognition
  delta.

If any check fails, termination is false; the loop continues.

---

## §3 — Recommendations summary

### Verdict: FIRE-WITH-AMENDMENTS

Reed amends the loop brief with the following before firing:

1. **Verb correction**: `just rebuild` → `just install` (or
   `just build && just install`). The Justfile has no `rebuild`
   recipe; install IS the substrate-pull-natural verb because it
   refreshes the binary the MCP wrapper invokes.

2. **Verification must be observable**: each tick verifies via
   `tests/mcp_fixtures/` (new dir). Tool registered + tool dispatched
   + result differs from pre-tick binary. Build succeeding is not
   sufficient.

3. **Termination needs falsification criterion + maximum tick count**:
   replay a known-hard tick from @cyberpunk audit (e.g. §17
   @kintsugi tower walk); termination is false if mirror-hosted
   spawn cannot handle it. Maximum 25 ticks as backstop.

4. **Agent-in-flight discipline addendum**: agents own ticks atomically
   when the tick changes binary; agent-in-flight applies to Track B
   (shard/spec work that doesn't touch bootstrap/). Reed lands the
   addendum to `feedback-loop-always-agent-in-flight` BEFORE firing.

5. **Dissolution-first picking**: each tick's improvement must map to
   a row of `the-convergence.md` §2.1 OR a `\` action body in
   `shards/mirror/lens/{mcp,lsp}.mirror`. Novel-synthesis ticks are
   Reed-inline (per §17 pattern) or deferred.

6. **Seam-review cadence floor**: four trigger criteria (this review;
   first cross-cutting tick; first novel-synthesis tick; termination
   claim). Beyond these, Reed's judgement.

### Concrete tick 1 suggestion (substrate-pull-natural)

**Lift `bin/mirror-mcp` from bash to Rust as a `mirror serve --mcp`
subcommand.** Dissolution-shaped:

- The bash wrapper's `tools/list` response is a hand-rolled string;
  lift to a Rust function returning a `serde_json::Value`.
- The bash dispatch table is a case statement; lift to a Rust `match`.
- The three tools (`mirror_compile`, `mirror_craft`, `mirror_kintsugi`)
  already work; lift maintains parity.
- `just install` refreshes the binary; the MCP wrapper invokes the
  new binary via `MIRROR_BIN=~/.local/bin/mirror`.
- Verification: `tests/mcp_fixtures/initialize.json` +
  `tests/mcp_fixtures/tools_list.json` + per-tool fixtures.

This tick produces the **Rust MCP foundation** that subsequent ticks
extend. Every subsequent tool addition is then dissolution-shaped:
add a row to the tool table, add a match arm, add a fixture. The
loop's stall-risk drops sharply.

---

## §4 — The strongest single objection

**The strongest objection that stands**: the brief's verification
shape is ceremony, not verification (Finding 2.3.B).

The proposed loop's discipline reads: "use the current MCP + LSP to
identify one substrate-pull-natural improvement, land it in mirror
source, `just rebuild` to refresh the binary, verify via the new
binary's tools." The verification step is doing the work the loop
depends on for correctness. Build succeeds + presence-check + a
mirror-text response: all three can be true while no actual capability
landed. The bash wrapper failure mode (Finding 2.5.A) makes this
particularly acute — a tick can "verify" by editing
`bin/mirror-mcp` directly, never touching the Rust source, and the
binary nobody invoked is unchanged.

The loop will produce stalls or false ticks until verification means
**fixture-driven diff against the previous binary's behavior**.
SHOULD-FIX 2 amends.

The loop's framing addresses this **partially** ("verify via the new
binary's tools") but the resolution is at the wrong altitude — the
NEW binary's tools verifying the NEW binary is self-referential.
The substrate-pull-natural fix is fixture-driven regression: the
fixture exists in the repo; the OLD binary fails it; the NEW binary
passes; the diff IS the capability gain. This is exactly the shape of
the kintsugi-ci verdict envelope (`docs/specs/kintsugi-ci-v0.1.md`
§5.3 / `just kintsugi-ci-local`) at MCP altitude. The substrate
already has the word.

---

## §5 — Seam-review cadence recommendation (Reed's question)

Reed asked: "under what concrete criteria does Reed insert a Seam
review?"

**Recommendation: four floor criteria + Reed's judgement.**

The four floor criteria (per Finding 2.7.A):

1. **Pre-tick-1**: this review.
2. **First cross-cutting tick**: any tick modifying both MCP and LSP,
   or both Rust source and shard source. Cross-cutting changes touch
   substrate seams; Seam reviews seams.
3. **First novel-synthesis tick**: per recognition #73, novel-synthesis
   work has elevated stall risk. Seam reviews before the stall to
   either reframe to dissolution or surface the synthesis claim
   explicitly.
4. **Termination claim**: load-bearing verification; not Reed-unilateral.

Beyond the floor, Reed exercises judgement:

- High structural-change ticks (any tick touching `@mirror/lens` family
  shapes).
- Ticks where Reed feels uncertain mid-design (the "I should check"
  signal IS data).
- Ticks where an in-flight agent's work composes against the binary
  change (per §2.2 — Seam reviews the composition seam).
- At Alex's request.

The floor is the substrate's mathematical commitment. Reed's judgement
is the substrate-pull-discipline carrier. Both.

---

## §6 — Severity-graded summary

### 🔴 BLOCKERs: 0

(No structural defects requiring loop retraction.)

### 🟡 SHOULD-FIXes: 4

1. `just rebuild` → `just install` verb correction (Finding 2.3.A).
2. Verification must be fixture-driven, not presence-check (Finding 2.3.B).
3. Termination needs falsification criterion + maximum tick count
   (Finding 2.1.A).
4. Agent-in-flight discipline addendum for binary-changing ticks
   (Finding 2.2.A).

### 🟢 ADVISORYs: 3

1. The recursion is vocabulary-expansion-shaped; brief should cite
   `architecture-peer-learns-by-crystal-vocabulary-expansion` (Finding 2.6.A).
2. Seam-review cadence has four floor criteria + Reed's judgement
   (Finding 2.7.A).
3. Dissolution-first picking discipline; cite `the-convergence.md` §2.1
   on every tick (Finding 2.4.A).

### ✅ VERIFIEDs: 6

1. Justfile has `just build`, `just install`, `just craft-binary`;
   no `just rebuild`.
2. `bin/mirror-mcp` is a 145-line bash wrapper; no Rust MCP server.
3. No LSP server exists; zero LSP files in `bootstrap/src/`.
4. `shards/mirror/lens/{mcp,lsp}.mirror` declare family headers with
   empty action bodies (`{ \ }`) — substrate is ready for consumer pull.
5. `the-convergence.md` §2.1 names the algebraic map for every
   porcelain verb across all four lens surfaces.
6. The convergence spec explicitly notes the LSP is substrate-generated
   via `@code/rust/lens-server` macro shim; no `tower-lsp` dependency.

---

## §7 — Verdict line

**FIRE-WITH-AMENDMENTS.** Four SHOULD-FIXes; zero BLOCKERs. Loop
foundation is sound; brief operational discipline needs tightening.
With amendments, the loop fires clean and has high substrate-pull
support (entire `the-convergence.md` §2.1 table is dissolution-ready;
both lens shards are family-header-only with empty bodies awaiting
consumer pull).

Signed: Seam <seam@systemic.engineer>.
