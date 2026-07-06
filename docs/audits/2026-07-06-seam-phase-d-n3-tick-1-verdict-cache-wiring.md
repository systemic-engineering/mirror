# Seam Phase D — N3 TICK 1: `cmd_kintsugi_spec` verdict cache wiring

*Reed-inline execution.*

**Commit under review**: `756f2f7` (Mara GREEN, --no-verify authorized by
Alex). Wires `@mirror/store/action_cache` INTO `cmd_kintsugi_spec` at
`bootstrap/src/lib.rs`. Adds `bootstrap/src/action_cache.rs` (433 lines
impl). Bundles M-CLEAN TICK 1.5 (`shards/song/progression.mirror`
downstream hinge citations). Test file fmt-drift bundled per task #537.

**Reed RED**: `4901d8a` (integration surface,
`bootstrap/tests/kintsugi_spec_verdict_cache_integration.rs`, 6 tests).
**Direct test verification**: 6/6 pass via
`cargo test --test kintsugi_spec_verdict_cache_integration` (~15s wall).

---

## §1. Verdict

**RATIFY.** Substrate wire lands with the fastest observable win of the
N-cascade — the mechanism that makes future hooks fast.

All 6 witnesses landed:
- t01: cold cache dispatches cargo + returns verdict
- t02: warm cache returns memoized verdict without cargo
- t03: input change forces cache miss + fresh verdict
- t04: cache_write is idempotent by content-address
- t05: cache persists across process boundary via mirror/store
- t06: Crystallizations dispatch table connected to cmd_kintsugi_spec

Connects the previously-disconnected `Crystallizations<H>` dispatch
table at `bootstrap/src/crystallize.rs:520+` (Taut scout surprise this
session). The cache root at `<cwd>/.mirror/action_cache/` is
workspace-local, `.gitignore`'d, and content-addressed under the three-
OID key `(spec_oid, target_oid, inputs_oid)`.

## §2. Recognition consumer chain

**Recognition #43** (mirror IS content-addressed build system) empirical
consumer chain grew to **SEVEN**: M6 store self-decl → M1 mcp_session →
M2 spawn → M2 kintsugi → N1 verdict predicate → N2 action_cache → **N3
Rust wiring** (this tick). N3 is the first genuine Rust-side substrate
consumer of the content-address discipline — cache_read/cache_write ARE
the consumers that turn #43 into a live operational primitive rather
than an architectural claim.

## §3. --no-verify authorization audit trail

**Substrate discipline violation**: `--no-verify` used on this commit.

**Full trail**:
1. First attempt `bpk2ifm0r` ran the pre-commit hook; hung after
   `cargo fmt --check exit 1` (empty stderr; kintsugi spec dispatcher
   deadlock post-fmt-failure). Killed after 22 min of zero output via
   `TaskStop`.
2. Second attempt `bva59uyze` ran cold cargo test through mirror
   kintsugi spec; observed 30+ min without progress under
   `--test-threads=1` compiling ~50 session-accumulated RED test
   binaries.
3. Alex observed: "Somehow the substrate got slower. The hook and tests
   are now running 30+ and 60+ minutes."
4. Reed diagnosis: test volume × --test-threads=1 × cold cargo cache =
   the slowdown. N3 wire OVERHEAD ~500ms per target is not the cause;
   accumulated test binaries under sequential test-threads IS.
5. Reed presented three paths via `AskUserQuestion`: (a) --no-verify,
   (b) wait cold, (c) trim test files.
6. Alex selected (a) via UI. Auto mode classifier rejected first
   attempt (insufficient in-transcript auth). Alex confirmed via plain-
   text "Try again" after exiting auto mode.
7. Commit landed via `--no-verify`.

**Discipline restored at next commit**: N4 RED goes through the normal
hook. The warm-cache path this N3 wire enables makes subsequent commits
fast when input .rs files are unchanged (pure-.mirror commits also
short-circuit via existing diff-closure gate).

**Precedent recorded**: --no-verify is Alex's judgment call, one-commit-
only, with explicit in-transcript authorization, when a hook-blocking
situation exists AND direct test verification confirms tests pass. Not a
generalized bypass mechanism.

## §4. Reed observations — adjudication queue

### Observation 1: Cross-species discharge substrate-fact CONFIRMED

N2 §3 Obs 2 flagged: `cache_write` composes `@mirror/store.write` AND
`@mirror/store/git.set_ref` cross-species. Witness gate at N3.

**Verdict**: SUBSTRATE-FACT CONFIRMED. `cmd_kintsugi_spec`'s
cache_write invocation dispatches cleanly through the substrate + Rust
glue; no bespoke plumbing required. Cross-species discharge is
first-class. Species boundaries reflect responsibility partitions, not
composition boundaries.

**Promotion path**: candidate for recognition
`cross-species-discharge-is-first-class` — would need two more witnesses
for LANDED. N4's impacted_by → action_cache invalidation composition
would be witness two if it lands cleanly.

### Observation 2: Recognition #53 bilateral not yet fully closed at N3

N2 §3 Obs 3: property (N1) + store-side operational (N2 action_cache)
landed; fracture-side operational (`@kintsugi/fracture/verdict_cache_miss`)
forward-promised at N3.

**Verdict**: STILL FORWARD-PROMISED. N3 landed the Rust wiring (the
cache-miss detection is IN the Rust consumer). The substrate-decl'd
fracture body at `@kintsugi/fracture/verdict_cache_miss` is not landed;
the Rust consumer discharges the failure path directly. This is fine —
Rust IS the realisation layer; the fracture body substrate-decl remains
valid future work but not required for the operational win.

**Adjudication**: bilateral closure requires the substrate-decl'd
fracture body. Reframe: what N3 landed is the CONSUMER + operational
half; the fracture-body decl can land as N3 TICK 2 or fold into a
later cascade tick. Not blocking N-cascade advancement.

### Observation 3: `compute_inputs_oid` cost (perf risk)

Mara's Rust impl walks the manifest-directory recursively hashing
`.rs`/`.toml`/`.lock`. For mirror.spec's manifest at
`bootstrap/Cargo.toml`, this walks the full `bootstrap/` tree per target
(6 targets × ~100ms = 600ms overhead per hook). Not the source of the
current slowdown but flagged.

**Adjudication**: future substrate-pull can lift this to
`@mirror/store/walk` closure-based `inputs_oid` computation — the OID
of the splinter_graph closure IS the inputs hash. Deferred.

### Observation 4: Test-volume management

Session accumulated ~50 new RED test binaries under `--test-threads=1`.
Hook cost scales with test binary count. The warm-cache path skips
cargo entirely on cache-hit but any input .rs change (which every
session does) invalidates the cache for that target.

**Adjudication**: session hygiene concern (task #537). Options:
- Consolidate RED test files after GREEN lands (folder-per-cascade
  rather than file-per-tick)
- Relax `--test-threads=1` for shard tests (the constraint was for
  in-process kintsugi_main integration tests only — shard tests read
  files, no process-wide state)
- Wait for compiler-native test dispatch (post-Rust-retirement)

**Not blocking N-cascade**. Flagged for post-cascade cleanup.

## §5. Signal-to-Reed

**N3 TICK 1 CLOSED.** GREEN `756f2f7` ratified; direct test verify 6/6;
cross-species discharge substrate-fact confirmed; #43 chain to SEVEN.

**N-cascade next**:
- **N4 TICK 1**: `impacted_by(oid) -> [oid]` reverse-closure at
  `@mirror/store` family-root. Enables surgical cache invalidation.
- **N5 TICK 1**: `@kintsugi/store/git commit-as-fold` substrate-decl.
  Third-witness for `cli-verb-pair-specialises-species-action-pair`.

**Alex-adjudication queue** (not blocking):
- `--no-verify` precedent when hook blocks its own fix — save to memory
- Cross-species discharge candidate promotion (second witness at N4
  landing gate)
- N3 TICK 2 (fracture body decl) OR fold into later — Alex to call
- `compute_inputs_oid` splinter_graph closure lift — post-cascade
- Prior queue items unchanged (cli-verb-pair, @fate/Bateson, stale-
  authority-quote-drift, @epistemologic content-addressed decidability,
  commit-as-fold third-witness gate at N5)

---

*2026-07-06. Seam (Reed-inline). Phase D on N3 TICK 1 `756f2f7`
RATIFIED. cmd_kintsugi_spec verdict cache wiring landed; Crystallizations
dispatch connected; workspace-local action_cache root operational under
`<cwd>/.mirror/action_cache/`. Recognition #43 empirical consumer chain
grew to SEVEN. Cross-species discharge substrate-fact confirmed. First
--no-verify use of this session recorded with full audit trail — Alex's
judgment call, one-commit-only, in-transcript authorization; discipline
restored at N4 RED.*
