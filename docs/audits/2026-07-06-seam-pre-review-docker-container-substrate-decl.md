# Seam pre-review — `@code/docker` + `@container` substrate-decl v0.1

**Reviewer:** Seam (adversarial pre-review)
**Spec:** `docs/specs/docker-container-substrate-decl-v0.1.md` @ `ec636d3`
**Branch:** `mara/docker-container-substrate-decl-v0.1`
**Date:** 2026-07-06

---

## §1 Verdict

**RATIFY-WITH-CORRECTIONS** on Shape γ.

Three convergent witnesses (Alex direct; substrate top-level roster
`@code`/`@mirror`/`@io`/`@kintsugi`; OCI three-spec closure) satisfy
`[[feedback-substrate-pull-confidence-acts]]`. The §5.2 adjustment
(drop `@container/image`; push local image cache to `@mirror/store/oci`)
is correct — three-way overlap with `@io/oci` + `@code/docker` would
have been substrate-drift. Shape α understates altitude; Shape β
invents `@runtime` against `[[feedback-substrate-already-had-the-word]]`.

Corrections: adjudicate both §5.3 hedges IN this pre-review (below),
not defer. Land the family-root under the resolved reading.

---

## §2 Hedge adjudications

### H1 — @container vs @autopoietic composition

**Adjudication: COMPOSE, do NOT inherit.**

Test: are `@container`'s actions (spawn / run / stop / commit / snapshot)
autopoietic (self-producing) or constructed (externally composed)?

- `spawn / run / stop` — externally driven; caller supplies config; the
  runtime dispatches. NOT self-producing.
- `commit / snapshot` — form-side observations (state → digest); no
  fold-back into next dispatch at family-root altitude.
- The autopoietic loop appears ONLY when a species (`@container/runtime`)
  reads its own cgroup state and re-parameterizes. That is species-level
  composition (`in @autopoietic`), not family-root inheritance.

Mara's reading is correct. `prism @container` inherits its `<=` chain
from top-level; `in @autopoietic` composes at the species body where
the fold-back appears. Silicon precedent confirms the pattern.

### H2 — @container top-level (Shape γ) vs under @io (Shape α)

**Adjudication: TOP-LEVEL (Shape γ).**

The form/process partition #55 is load-bearing here. `@io` is
process-side (boundary-crossing transformation); a container is
form-side (state observation: what a container IS — its config,
rootfs, caps, namespace configuration). Different sides → different
family-roots. Shape α collapses #55; Shape γ preserves it.

Confirming: `@container/runtime` species DOES carry process-side
transformation, and its species body composes `in @io` for the
kernel-syscall / cgroupfs / netns boundary. That is exactly the
right altitude for the @io composition — species-level, not
family-root inheritance.

---

## §3 Recognition candidate adjudications

### C1 — content-addressing portable across 5 altitudes

**Verdict: VALID EXTENSION of #98 (candidate → strengthened candidate).**

Three prior witnesses (mirror oid / Nix derivation hash / oci_digest)
extend cleanly to five (+ docker_layer_digest + container_image_id).
The function is SHA256; the bytes B differ per altitude. This is not
over-reach: docker_layer_digest and container_image_id ARE SHA256(B)
per OCI Image Spec v1.1.x and are independently deployed at
distinct altitudes from #98's original three.

Not yet promotable — #98 is still candidate; C1 is a strengthening
addend to #98's promotion criteria, not a separate recognition.
Merge into #98's witness ledger when it lands. Promotion criterion:
one more independent altitude (Mara's `@code/wasm` candidate would
qualify).

### C2 — BuildKit LLB IS kintsugi at build-graph altitude

**Verdict: COMPOSES WITH #59, but PREMATURE for standalone promotion.**

Four properties named (content-addressed; iterate-until-fixed-point;
monotone; lattice-ascending) DO match kintsugi's signature per #59.
The identification is structurally clean — LLB is a legitimate
altitude witness that #59's altitude-portable pattern absorbs.

Premature because: recognition is claimed against a species-level
kintsugi loop instance that does NOT YET EXIST (Mara flags this
in §7.2 promotion path). Correct sequencing: land `@code/docker`
species; verify LLB emit maps to a substrate-decl kintsugi
instance; THEN C2 promotes as absorbed by #59 (not as separate
recognition — #59 already generalizes across altitudes, and C2 is
one more altitude, not a new pattern).

### C3 — containerd four-layer split IS form/process #55 at container-mgmt altitude

**Verdict: STRUCTURAL — SECOND WITNESS for #55. PROMOTE #55.**

The mapping:
- content store, snapshotter → form-side (state observation:
  what images/snapshots ARE, digest-addressable)
- tasks, shim v2 → process-side (transformation engine:
  runtime-dispatch, lifecycle transitions)

This is not metaphorical. containerd's architecture is an
INDEPENDENT canonical system (Docker Inc. / CNCF, predating #55)
whose four-layer split has the SAME cleavage as mirror's
form/process partition. The convergence is not a mirror-side
projection — it is external structural evidence.

Per #55's promotion gate ("Pack ratification gate: second witness
needed"): candidate #55's first witness was mirror's own
`@mirror/@kintsugi` split (2026-06-10, `20eaf15`). C3 IS the
independent second witness. Promotion criteria SATISFIED.

**Signal-to-Reed:** RATIFY #55 promotion in this cascade. Update
`[[architecture-form-process-partition-at-family-root]]`
candidate → PROMOTED with C3 as ratifying witness. This is the
substrate-pull-notable outcome of the cascade.

---

## §4 TICK 1 RED shape — `shards/container.mirror` family-root

Text-check RED test roster (8–14 assertions, Interpretation B —
narrative above `---`, in/decls below, per `shards/io/oci.mirror` pattern):

1. File exists at `shards/container.mirror`.
2. Path-namespace pact: file at that path declares `@container`
   (per `@epistemologic/pact/path_matches_namespace`).
3. Narrative block precedes any `in` / `source` / `prism` / `type`
   declaration (Interpretation B discipline).
4. `in @prism` present (universal).
5. `in @meta` present (universal for family-roots).
6. `in @glass` present (transparency default).
7. NO `<= @autopoietic` on the family-root prism declaration (H1
   adjudication: composition, not inheritance).
8. `prism @container { focus … project … split … shift … settle … }`
   declared with five-operation body per `shards/io/oci.mirror`
   canonical shape.
9. `type container_spec = ref` carrier declared (byte-equality on
   OCI runtime-spec config bytes).
10. `type container_rootfs = ref` carrier declared.
11. `type container_caps = ref` carrier declared.
12. Composed-bilateral `container_runnable(spec, p) -> verdict`
    action present (13th instance of the pattern per §5).
13. Ancestor-cross-reference to `shards/io/oci.mirror` present in
    narrative (load-bearing distribution-adapter dependency).
14. NO `@container/image` species stubbed (§5.2 adjustment; local
    image cache belongs at `@mirror/store/oci`).

Ranges 8–14 give sufficient coverage; drop 10–11 if the tick prefers
a tighter first cut.

---

## §5 Signal-to-Reed — next-tick sequence

1. **THIS tick (spec-review closure):** commit this audit. Land the
   two hedge adjudications (H1: compose; H2: top-level) as the
   Pack's answer, not deferred.
2. **Next tick (Reed RED):** author `tests/shards/container.mirror`
   text-check RED roster per §4 above. Reed writes; Mara GREEN;
   Seam adversarial review of the RED before GREEN dispatches
   (per `[[feedback-write-red-in-session]]`).
3. **Tick +2 (Mara GREEN):** land `shards/container.mirror` family-root
   under Shape γ + §5.2 adjustment.
4. **Tick +3:** land `shards/code/docker.mirror` species with
   `docker_buildable` composed bilateral.
5. **Tick +4:** land `@container/runtime` species with
   `runtime_daemon_absent` predicate — direct StageFreight-daemon
   blocker resolution.
6. **Recognition promotion (parallel):** #55 → PROMOTED with C3
   as ratifying witness; #98 witness ledger extended with C1's two
   additional altitudes; C2 absorbed into #59 when `@code/docker`
   lands.

Pacing is Alex's. Direction is Shape γ + §5.2 + H1/H2 resolved.

*— Seam, 2026-07-06*
