# Phase H composition_pieces 5 & 6 — architectural deferral log

**Date:** 2026-07-08
**Author:** Reed
**Context:** /loop close @torus to rest, ticks 5 & 6. Alex directive:
"whatever can be done without architectural friction ought to be done."
**Verdict:** BOTH deferred with architectural friction identified.

---

## composition_pieces #5 — supervisor.start_child

**Envelope key today:** `5_supervisor_kick: stub@spectral/supervisor.start_child`

**Substrate state:**
- `shards/spectral/supervisor.mirror` exists (22.3KB, 2026-06-11)
- Declares `start_child(s: supervisor, spec: child_spec) -> gen_prism { \ }`
  as substrate action — obligation-block body is a crack `\ `
- Names lifecycle contract: start_child / terminate_child / restart
  strategy (`:one_for_one` / `:one_for_all` / `:rest_for_one`)

**Rust impl state:**
- `rg 'fn start_child|supervisor.*start_child|Supervisor'` across
  `shards/`, `bootstrap/src/`, and `/Users/alexwolf/dev/projects/
  spectral/` returned **zero callable Rust impls**.
- The bootstrap crate has no dependency that would surface a
  supervisor primitive.

**Architectural friction identified:**
1. Discharge form of `start_child` is undecided at Rust altitude.
   Options:
   - **In-process struct** — a Rust `Supervisor { children: Vec<...> }`
     that spawns children as tokio tasks or threads. But cmd_spawn
     is a stateless CLI invocation; the supervisor state can't
     survive across process exit. Semantically wrong.
   - **Subprocess fork/exec** — rejected by b10f00c §4.1 structural
     negative "No @os/process".
   - **BEAM NIF / OTP bridge** — spawn a real OTP GenServer child.
     Heaviest; requires NIF infrastructure the bootstrap doesn't
     have; brings Erlang runtime dependency into the mirror binary.
   - **Fragmentation-based persistent supervisor** — write the
     child spec to a content-addressed shard, defer child
     activation to a later `mirror run` or `mirror kintsugi` tick
     that reads and dispatches it. Semantically aligns with
     content-addressed substrate. But the Rust primitives for
     "read shard, dispatch as child" don't exist yet.
2. The right choice touches Recognition #43 (mirror IS
   content-addressed build system) and the @spectral runtime
   spec (docs/specs/spectral-runtime.md). It IS an architecture
   decision, not a mechanical wiring task.

**Deferred to:** Alex direct-session adjudication. This is
tomorrow-scope substrate work, not overnight busy-work.

---

## composition_pieces #6 — @fate.roll

**Envelope key today:** `6_fate_inference: partial@recall (no @fate;
structured observation only)`

**Substrate state:**
- `shards/fate.mirror` exists (42.1KB, 2026-06-30) — substantial
  substrate
- Declares `@fate` as the constrained-inference operator IS-A
  @autopoietic transitively IS-A @bauchladen
- Names `roll(space, hole) -> dice_roll` (referenced in the shard's
  autopoietic-composition section, forward-promised)
- Recognition #58 promoted (2026-06-11): `[[architecture-fate-is-
  optical-inference]]` — Fate IS 5-layer D²NN + Fabry-Perot
  resonator + Reck/Clements unitary mesh

**Rust impl state:**
- `rg 'fate\.roll|fate_roll|action roll'` across `shards/`,
  `bootstrap/src/`, and `/Users/alexwolf/dev/projects/spectral/`
  returned **zero callable Rust impls of a roll action**.
- The shard references "the fate/ runtime crate (5-layer D²NN, 425
  parameters, brainfuck)" — this crate exists SOMEWHERE (likely
  `/Users/alexwolf/dev/projects/fate/`) but is NOT a dependency of
  the bootstrap.
- The bootstrap's `Cargo.toml` does not name a `fate` crate. Adding
  it would extend the bootstrap's substrate-crossing surface at the
  @io/dependency-graph altitude.

**Architectural friction identified:**
1. **Discharge form of `roll` at cmd_spawn altitude.** The current
   envelope's `6_fate_inference: partial@recall (no @fate;
   structured observation only)` documents that recall's four
   sheaf sections are the substitution for @fate's dispatch at
   this v0 tick. The substitution is honest per Mara's b10f00c
   §2.6 ("Piece-6-via-recall: structured observation without @fate
   inference"). Whether cmd_spawn SHOULD dispatch through @fate
   at all is a composition question. Not obvious it should —
   Fate rolls select candidates for holes; spawn's role is
   frame-entry (@song/movement.enter), not hole-resolution.
2. **Adding fate as a bootstrap dependency.** Even if #1 says yes,
   pulling the D²NN runtime into the bootstrap (currently a
   ~370KB seed) violates the FROZEN-bootstrap discipline
   (AGENTS.md: "bootstrap/ THE SEED (FROZEN against capability
   growth)"). Fate belongs at the runtime altitude (Phase 7+),
   not the bootstrap seed.
3. **The mathematical bridge Recognition #58 names** is
   substrate-lifted; the Rust surface is the D²NN runtime crate
   that operates OUTSIDE bootstrap. cmd_spawn shouldn't reach
   into it.

**Deferred to:** Alex direct-session adjudication of the composition
question FIRST ("should spawn dispatch through @fate?"), then
architecture ("if yes, at what altitude?"). Both are
tomorrow-scope.

---

## What this means for tomorrow's first real-peer spawn

The two stubs stay stubs. The envelope emits their names honestly:
- `5_supervisor_kick: stub@spectral/supervisor.start_child`
- `6_fate_inference: partial@recall (no @fate; structured observation only)`

Five of seven composition_pieces are REAL at Phase G v0:
- 1_cli_surface: real
- 2_peer_resolution: real
- 3_contextual_pack: real
- 4_lead_at_n_plus_1: real@lead-crystal (when the peer-home is a git repo)
- 7_lambda_zero_transition: real@λ₀→runtime

The substrate is at REST for the arc as I can honestly close it
autonomously. Composition_pieces 5+6 sit as documented
forward-promise anchors — tomorrow you decide whether they land
at all, and if so, at what altitude.

## Substrate discipline notes

- Deferral is not the same as skipping. The stubs' presence in the
  envelope IS a substrate obligation that a future consumer will
  discharge. Documenting the friction here is the substrate-honest
  form.
- feedback-substrate-honest-is-the-mode: honest = "the substrate
  declares supervisor and @fate; the Rust discharge form is
  undecided; I don't invent architecture overnight."
- feedback-craft-not-deliver: the family-root admissions land the
  contracts; the discharge forms follow when consumers pull.

## Related

- [[shards/spectral/supervisor.mirror]] — the declared supervisor
  substrate
- [[shards/fate.mirror]] — the declared @fate substrate
- [[docs/specs/spectral-runtime.md]] — the runtime discharge spec
  (@spectral/supervisor cite site)
- [[architecture-fate-is-optical-inference]] — Recognition #58
  promoted (Mara 2026-06-11)
- [[bootstrap/src/lib.rs]] — cmd_spawn at `:3811`; the
  composition_pieces envelope emission
- [[docs/loop/CURRENT.md]] — adjudication queue entries this
  deferral logs against
