# `@mirror/recall` — the inbound surface family-root for returning agents

*2026-06-26. Mara. Canonical spec (family-root substrate-decl declaration).*
*Companion to Reed's observation `docs/observations/2026-06-26-reed-rehydration-gap-in-mirror-mcp.md` (228 lines; the motivating gap-naming) and Mara's insight `docs/insights/2026-06-26-spawn-is-substrate-leaving-ground-state.md` (`b10f00c`; the spawn-IS-leaving-ground-state structural claim §1 + the seven forbidden primitives §4). Composes WITH Mara's psychohistory-vector-as-sheaf insight (`d00f553`; the H¹-shape framing the rehydration gap admits per Taut's M2 scout finding) and Taut's graph-dependency-DAG scout (`d4749c0`; the `in <X>` dependency-direction pattern).*

*Status: **Red** at the shard altitude. This spec declares the family-root substrate-decl signature; Reed lands the Rust impl at Phase G of the round-trip loop (P3 of P1+P2+P3). The CLI surface, the four payloads, the dependency direction, the forbidden-primitives matrix, the name selection, the cross-altitude connections, the empirical consequences, and the open hedges land here for Pack review.*

Reads from / depends on:
- `docs/observations/2026-06-26-reed-rehydration-gap-in-mirror-mcp.md` (Reed; the motivating observation, four shapes named in §4)
- `docs/insights/2026-06-26-spawn-is-substrate-leaving-ground-state.md` (Mara, `b10f00c`; the spawn-IS-leaving-ground-state insight; §2 seven-piece composition; §4 seven forbidden primitives — the load-bearing gate)
- `docs/insights/2026-06-26-psychohistory-vector-as-sheaf.md` (Mara, `d00f553`; H¹ = candidate recognitions / two-witness rule = gluing axiom; §5.2; the rehydration gap is H¹-shaped per Taut's M2)
- `docs/scouts/2026-06-26-taut-graph-dependency-dag-scout.md` (Taut, `d4749c0`; the `in <X>` arrows point from consumer UP to grounding; the invariant `@mirror/recall` must respect)
- `docs/scouts/2026-06-26-taut-psychohistory-cohomology-scout.md` (Taut, `3a385fd`; the peer-ACL §10.1 explicit exclusion: lead→member is NOT a sheaf restriction map; flagged here, not collided with)
- `bootstrap/src/mcp.rs` (the current MCP surface — six tools: `compile`, `craft`, `kintsugi`, `prisms`, `verdict`, `spawn`; `prisms` is the existing introspection primitive this family-root composes WITH, not replaces)
- `shards/mirror/spawn.mirror` (the outbound family-root counterparty; signature pattern this spec mirrors symmetrically)
- `shards/mirror/store.mirror` (the OPEN content-addressed gate; the payload anchor surface)
- `shards/mirror/ref.mirror` (the navigable surface of A; recall composes WITH ref at the trajectory altitude)
- `shards/mirror/bench.mirror` (`bench_crystal`, `monotone_non_increasing`; recall's dogfood-state payload reads bench surface)
- `mirror.spec` (the dogfood instance; recall's pack-trail payload reads pack{} block; recall's dogfood-state payload reads settle_on block)
- `docs/specs/mirror-spec-schema.md` (the spec schema recall reads against)

Forward references (this spec unblocks):
- `shards/mirror/recall.mirror` (the substrate-decl family-root prism Reed lands; signature in §3.5)
- `shards/mirror/recall/{cascade,pack_trail,pull_frontier,dogfood}.mirror` (the four payload species; forward-promised per §3)
- `bootstrap/src/mcp.rs` (the MCP wire integration — one new tool `recall` advertised at `tools/list`; dispatch in `dispatch_tool_call`; integration test at `bootstrap/tests/mcp_handshake.rs::seven_tools_advertised`)
- `bootstrap/src/main.rs::cmd_recall` (the CLI dispatch; tick after the shard compiles)

---

## Table of contents

1. Statement — what the family-root IS in one paragraph
2. Motivation — Reed observation `c0acf41` + spawn↔recall symmetry
3. The four payloads — each formalized: what it returns, content-addressing anchor, composition with `prisms`
4. Dependency direction — what `in <X>` this family-root imports per Taut's pattern
5. Forbidden primitives check — explicit walkthrough of `b10f00c` §4 per payload
6. Name selection — final name picked + why; trade-offs named
7. Connections — `prisms`, spawn-and-probe relation, eigenboard-IS-sheaf, psychohistory H¹
8. Empirical consequences — what the round-trip test drive demonstrates
9. Honest hedges — what stays genuinely open
10. Pack trail

---

## 1. Statement — what the family-root IS

**`@mirror/recall` IS the substrate's inbound surface family-root — the typed family-root prism that exposes the substrate's own trajectory state to a returning agent as content-addressed views, composed across the substrate-decl, observation, probe, and Pack sheaves Mara's psychohistory insight names (`d00f553` §3.7), anchored at OID/commit/state content-addresses rather than synthesized at call time, and lifted through the MCP wire as the symmetric dual of `@mirror/spawn`'s outbound counterparty surface.**

Six load-bearing pieces in that sentence, each grounded in a landed substrate-decl, a Pack-banked insight, or a substrate discipline:

1. **Inbound surface family-root.** A family-root in the substrate-decl sense per the pattern Taut's `d4749c0` scout verified across 13 shards: declares vocabulary; species discharge details; `in @prism in @meta in @glass` at the family-root altitude. The four payloads (§3) are species under this family-root, not separate family-roots.

2. **Typed family-root prism.** Per `[[architecture-prism-as-trait-as-everything]]`: `prism` is the foundational keyword. `@mirror/recall` declares the five-operation surface (`focus / project / split / shift / settle`) over a `recall_request` carrier; the four payload species (`recall/cascade`, `recall/pack_trail`, `recall/pull_frontier`, `recall/dogfood`) discharge the typed responses.

3. **Substrate's trajectory state to a returning agent.** Trajectory in the sense of Mara's `d00f553` §3.7 — the joint state of four stacked sheaves over the substrate's development manifold. The returning agent is the spawn-and-probe counterparty (Mara `b10f00c` §2.5) at altitude N who left in an excited state and returns to a substrate that has continued without them.

4. **Content-addressed views, composed across the four sheaves.** Per recognition #98 (content-addressing across scopes): every payload anchors at a typed content-address — OID for git refs, commit SHA for cascade ticks, splinter hash for substrate-decl events, crystal hash for bench measurements. The recall surface RESOLVES the anchor; it does NOT synthesize fresh content at call time. (This is §5's forbidden-primitive gate; load-bearing for what the family-root IS NOT.)

5. **Anchored at OID/commit/state content-addresses rather than synthesized at call time.** The forbidden-primitives gate from `b10f00c` §4 rules out stateless-return-at-runtime, idempotent-at-runtime, and identity-mint. Every recall response is anchored — same git ref produces structurally equivalent recall payloads; the substrate's content-addressing discipline at every scope (recognition #98) provides the anchoring for free.

6. **Symmetric dual of `@mirror/spawn`.** Per Reed's `c0acf41` §5 forward-promise: spawn is the substrate's outbound-counterparty surface (excitation above λ₀ per #99); recall is the substrate's inbound-rehydrating-counterparty surface (returning to ground state with knowledge of what moved while away). Same architectural altitude — both compose at the @mirror family-root altitude with the same prism vocabulary; opposite direction — spawn instantiates a runtime peer outward; recall projects substrate trajectory inward to a returning peer.

Collapsing the six pieces back into one sentence: **`@mirror/recall` is the typed family-root prism a returning agent invokes to read the substrate's trajectory state — across the four stacked sheaves of its own development — as content-addressed views anchored at the substrate's existing OID/commit/state addresses, the symmetric dual of `@mirror/spawn` at the same architectural altitude, the opposite direction.**

This IS the statement. §§2-10 work out the motivation, the four payloads, the dependency direction, the forbidden-primitives gate, the name selection, the cross-altitude connections, the empirical consequences of the round-trip test drive, the open hedges, and the Pack trail.

---
