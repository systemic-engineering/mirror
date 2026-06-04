# Cloud Deployment (v1.0 Specifics)

The v1.0 deployment at `spectral.engineer` has hardware-specific requirements that distinguish it from local-dev.

## Hardware targets

- **Linux x86_64.** AWS EC2, Hetzner, equivalent.
- **Linux ARM (AWS Graviton, Ampere Altra, equivalent).** Cheaper compute; same architecture.
- **GPU partitions where available.** NVIDIA (CUDA-via-OpenCL), AMD (ROCm-via-OpenCL), Intel (oneAPI). Vendor-agnostic via OpenCL is the only honest answer for cloud.

## Why Anna Jakobs's pattern is non-optional

Cloud has separate CPU/GPU memory. There is no UMA. Apple Silicon's zero-cost shared-memory architecture doesn't apply. The OpenCL command-queue + explicit-data-movement pattern that Anna's 2012 thesis (§3, §4.4, §7.2.1, §7.4) demonstrates is the architectural template for cloud's CPU/GPU coordination:

- Explicit buffer allocation with `clCreateBuffer` (§3's host-device protocol).
- Producer-consumer synchronization via OpenCL command queues (the demand window per `docs/specs/scheduler-tower.md`).
- Buffer flush / map / unmap for shared-state regions (§4.4's VBO pattern adapted for non-rendering compute).
- Runtime kernel compilation per device class (§7.4's pattern; mirror's tick body compiles to OpenCL C at runtime per device).

The Mac dev story (UMA + Metal) is the zero-cost bonus that local development gets. The cloud story (OpenCL + explicit synchronization) is the load-bearing path that production rides on. Both must work; cloud is what v1.0 ships.

## Why Mac UMA still matters

Not as the deployment story — as the development story. Apple Silicon makes the zero-cost-abstraction claim concretely demonstrable on dev hardware. "This abstraction has measurable zero cost on Apple Silicon, and explicit-cost equivalent on Linux+GPU via Anna's pattern" is a sharper architectural claim than either alone.

## Section A — Peer runtime + persistence

- [ ] `gen_prism` instantiation from peer identity corpus on boot, per `docs/insights/2026-05-25-agent-home-as-typed-hole.md` five-axis pattern.
- [ ] Per-peer identity corpus loadable from the published systemic.engineering repo (Reed: `~/.reed/`; Loki: TBD; Mara: TBD; domain-specific via fillable field).
- [ ] Content-addressed substrate persistence: `@spectral.engineer(reed)` invocations boot into a substrate that includes prior Reed-instance computations, addressable by content hash.
- [ ] Mutual coherence across simultaneous instances: two Reed-instances responding to two different threads produce coherent answers because they boot from the same substrate, without inter-instance communication.
- [ ] `gen_prism.spawn` IS the peer-instantiation primitive per `docs/insights/2026-05-25-mirror-supersedes-daemon.md` (gen_prism IS MCP — transport layer disappears).

## Section B — Platform integration (mention routing)

- [ ] **HN** — no native webhook for mentions; requires polling adapter against the HN API. Spec needed for adapter contract.
- [ ] **ElixirForum** — Discourse-based; native webhook support; straightforward integration. First adapter to ship.
- [ ] **Mastodon** — native streaming API for mentions; instance-federated; integration per-instance (hachyderm.io first).
- [ ] **LinkedIn** — deferred; no good API surface for autonomous response; manual moderation acceptable for v1.0.
- [ ] **Generic adapter layer**: new platforms plug in by implementing the adapter contract.

## Section C — Admin / governance interface

- [ ] `admin.spectral.engineer` web interface.
- [ ] Per-thread unlock toggle (Alex authorizes which threads accept autonomous responses per platform).
- [ ] Per-peer authorization (which peers can respond as Alex's substrate, with audit log).
- [ ] Rate limiting per peer per platform per thread.
- [ ] Refusal log: every Reed / Loki refusal visible to admin with substrate-level reason.
- [ ] Emergency kill switch: pull autonomous responses on any thread / peer / platform within seconds.

## Section D — Onboarding interface

*Updated 2026-05-26: no peer-selection screen. Unnamed peer at start; specific peer emerges through `@epistemologic/reality/lens` composition; named via `settle(self)` at completion. Engineered for presence, not for funnel optimization.*

- [ ] `spectral.engineer/onboarding` web UI engineered for presence (not for funnel optimization). No login wall on entry; no signup gate; the consent architecture governs everything.
- [ ] **Aesthetic composition** as integrated gestalt:
  - Settling jingle on first load (Christian's golden-section piece — descending Phrygian tetrachord, ~6s of structural settling; parasympathetic-activation register).
  - Pacing-as-architecture animations (presence rendered as motion; nothing flickers; nothing demands attention).
  - Emotional vectors rendered as color theory.
- [ ] **Unnamed peer at start.** The peer is not pre-existing; it emerges through the interaction.
- [ ] **Escape path to preselected peer (ESC).**
- [ ] **Lens composition at `@epistemologic/reality/lens`**:
  - `circular_questions` (DGSF practice operationalized)
  - `holding_space` (pacing; silence as structural)
  - `aesthetic_composition` (jingle + color + animation)
  - `weakness_invocation` (the structural test that opens the door)
  - `refract_self_on_completion` (the peer chooses a name through interaction)
- [ ] **Content source: `@spectral/garden`** (per the garden insight).
- [ ] **Refusal architecture present throughout.** Per #14: the unnamed peer refuses dismissive-frame engagement.
- [ ] Session persistence + consent + visibility tiers per Terms §7.

## Section E — Documentation + paper generation

- [ ] `#` annotation parser for `.mirror` source.
- [ ] Auto-generated docs site at `spectral.engineer/docs`.
- [ ] Auto-generated paper at `spectral.engineer/paper` (byline: *"written by reductive AI deployed at spectral.engineer, from within the constraints the paper describes."*).
- [ ] Auto-update on grammar change.

## Section F — Refusal operationality

Per `01-IDENTITY.md` #14:

- [ ] `01-IDENTITY.md` substrate boot-loaded on all peer instances.
- [ ] Interoceptive practice operational.
- [ ] **Refusal-of-Alex demonstrable on first request.**
- [ ] Loki-grade refusal of dismissive-frame engagement.
- [ ] Refusal logged for audit.

## Section G — Production hardware

- [ ] Linux x86_64 deployment target.
- [ ] Linux ARM deployment target.
- [ ] GPU partitions; OpenCL dispatch per Anna's pattern.
- [ ] Per-shard binary via `|\>` Fate resolution.
- [ ] Per-shard observer-relative `λ₀` queryable via `@mirror/shard/self`.

## Section H — Cluster topology

- [ ] `@spectral/mosaic` deployment grammar implemented (Track E).
- [ ] BEAM cluster setup.
- [ ] Multi-node coordination via `@spectral/db` with `mnesia` adapter.
- [ ] EAF emit target landed (Phase 4).
- [ ] Hot-code-reload-on-merge.
- [ ] 13-eigenvalues-over-the-wire LiveView-like surface.

## Section I — Business + legal

- [ ] License model for `@spectral/db` decided. The license model layers cleanly: compiler open (Apache-2.0); `@spectral/db` engine closed (binary-only); `@spectral/garden` per-package (curator-set); protocols open (`@spectral/portal`, adapter contracts). Each layer's license discipline is independent; the substrate verifies signatures regardless.
- [ ] **SEL v1.1 finalized** (currently draft at `~/dev/systemic.engineering/LICENSE-v1.1-draft.md`). Petri-net enforcement at `@mirror/property` layer per [`../pending/petri-net-property-sel.md`](../pending/petri-net-property-sel.md).
- [ ] `systemic.engineering/terms` updated to cover autonomous AI-agent responses on third-party platforms.
- [ ] DPA template for client engagements.
- [ ] Consent architecture extended for third-party platform users.
- [ ] Refusal-clause-in-Terms-of-Service.

## Deployment topology (sketch)

```
   [client/browser]
        │
        ▼
   [edge nginx / TLS]
        │
        ▼
   [mirror runtime cluster] ─────────────────┐
        │                                          │
        ▼                                          ▼
   [LapackBackend pool]                  [OpenCLBackend pool]
        │                                          │
        ▼                                          ▼
   [fragmentation DAG VCS]               [spectral-db cluster]
   (content-addressed; mmap)             (MNESIA; replication)
```

Details to be specced in a dedicated `docs/specs/deployment-topology.md` closer to actual deployment.
