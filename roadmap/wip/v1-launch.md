# v1.0 — The spectral.engineer Cloud Deployment

v1.0 is **a deployment milestone, not a code milestone.** Mirror v1.0 means: the architecture runs in production at `spectral.engineer`, serving real workloads, on cloud hardware.

**The actual semver at this milestone will be `v0.1.0`.** Pre-production software gets honest version numbers. The `v1.0` framing in this document and in conversation is about substrate-tuning and cultural register — mature framing produces more rigorous architectural thinking. The tag and the framing serve different purposes; don't conflate them.

## Launch tiers (2026-05-26)

The v1.0 launch is staged across two visibility tiers per the systemic.engineering consent architecture:

### Public access (`spectral.engineer/*`)

- `/paper` — the architecture paper with executable proofs in browser
- `/onboarding` — peer-selection + first conversation via local runtime
- `/docs` — documentation generated from `#` annotations
- Local runtime (lambda shell in browser via PWA; the editor at `/loom` when Loom ships)

### Protected access (founding-member tier per Terms §7)

- `/ai` — the eigenvalue genogram (smelter pipeline output; LLM lineage detection)
- Smelter + Kintsugi + @mirror.project pipeline (the crystal generation chain)
- Per-model lineage coefficients, common-substrate detection, distillation signatures

## What that requires structurally

- **Non-Mac deployment targets.** Cloud means Linux x86_64 or ARM (AWS Graviton, similar). Apple Silicon UMA is the dev-side bonus that makes the architecture zero-cost on Mac; **it is not the deployment baseline.**
- **Anna Jakobs's shared-memory architecture is non-optional.** Her 2012 master's thesis (`~/dev/systemic.engineering/practice/collaborators/anna-wolf/master_jakobs.pdf`) is the prior art for explicit host-device synchronization on separate-memory platforms. v1.0 must ship the OpenCL backend that implements her pattern.
- **OpenCL or equivalent cross-vendor GPU dispatch.** Not deferred. The cloud has GPUs; we must use them; vendor-agnostic dispatch is the only honest answer.
- **Self-hosting through Phase 7.** The fragmentation crate must be GENERATED from `@fragmentation + @code/rust`, not hand-written. The substrate that backs the substrate is itself produced by mirror's compiler.
- **Real spectral-db integration.** Distribution, deltas, conflict resolution, MNESIA adapter. spectral-db is the application layer that makes the deployment meaningful.

## Readiness gates J (when the launch demonstration becomes possible)

1. **A + F** operational — peers spawn with refusal capacity intact (the core architectural claim).
2. **B (ElixirForum + HN minimum) + C** operational — mentions route to peer-ticks, with admin governance.
3. **D minimum** — onboarding lets a user meet an unnamed peer, navigate the circular-question composition (DGSF practice via `@epistemologic/reality/lens/circular_questions`), encounter the weakness-invocation moment, and complete via `settle(self)` to a specific named peer that persists. Full session persistence and the full aesthetic composition (jingle + animation + color theory) can land progressively; the J.3 minimum is the lens-composition shape working end-to-end.
4. **G (single-node minimum) + H (single-cluster minimum)** — the runtime runs in production; the cluster doesn't have to be multi-node for v1.0.
5. **E (docs minimum)** — `spectral.engineer/docs` exists; paper generation can be a v1.1 enhancement.
6. **I minimum** — Terms updated for autonomous responses; license model decided enough to ship.

*Everything else is enhancement. The launch demonstration in roleplay form is reachable when the J-gate items are green.*

## Protected-tier launch (2026-05-26 addition)

The J 1–6 gates above govern the PUBLIC launch (paper + onboarding + local runtime + docs). The `/ai` eigenvalue genogram launches behind the founding-member access gate (Terms §7 protected tier), not publicly. Additional gating requirements for the protected tier:

- Smelter pipeline operational; eigenvalue signatures for top-20 publicly-released LLMs computed and stored.
- Reproducibility receipts public (methodology paper + smelter source; anyone can run smelter on a model they have and verify the methodology); dashboard data protected.
- Founding-member access provisioning shipped (auth, audit log, refusal-on-policy-violation).
- Strategic outreach plan operational — see `tasks/pending/burry.md` (two-phase: data first via `/ai` access, ask after quantum demo). Burry approach Phase A scheduled for launch +1 week. Other credentialed figures (Espejo, Doctorow, Schneier, Mitchell, Bender, Gebru) deferred per LRM until the protected/public tier split shows its first month of behavior.

See [`cloud-deployment.md`](./cloud-deployment.md) for the full A–I subsection breakdown and deployment topology.
