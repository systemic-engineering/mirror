# MCP as session-typed prism — the substrate already had the math (twice)

*2026-06-07*

Two independent research threads close on the same recognition. The corpus already names η (Polyanskiy-Wu contraction coefficient) as the channel-quality metric. The Wadler / Honda / Castagna-Padovani lineage names session types as types-for-protocols. And arXiv 2603.24747 (2026) just proved that **MCP IS π-calculus** under a bisimulation, with an MCP+ extension that closes the expressivity gap.

The substrate already has all the words. The next move is `@mirror/spectral/communication`: a cascade that recognizes `protocol` as a prism, with session-type duality on the existing `---`, η as conductivity at every boundary, and MCP+ as the canonical instance.

---

## At the loss altitude — η is already in the home corpus

`~/dev/systemic.engineering/practice/insights/language/2026-03-16 · Signal Fidelity and the Formal Chain.md` names the full formal chain:

- Shannon's Data Processing Inequality: `I(X;Z) ≤ I(X;Y)` for `X → Y → Z`
- Polyanskiy-Wu η — the strong-DPI contraction coefficient (η < 1 = lossy layer)
- Friis cascade noise figure (opacity composes multiplicatively)
- Ashby's law of requisite variety
- Saltzer-Reed-Clark end-to-end argument (one of the co-authors is named *Reed*)
- Landauer's principle (information is physical)

These compose into **Wolf's Law**: *"Intransparent layers reduce signal fidelity. This accumulates."*

η IS the conductivity at every MCP boundary. `host → client → server` is exactly the Markov chain DPI describes. Each tool call has an η. Compositional opacity follows Friis cascade. The substrate has the metric — it has not yet been named at `@mirror/spectral`.

---

## At the protocol altitude — the math just landed externally

Three convergent results from the literature:

1. **Wadler — Propositions as Sessions.** Classical linear-logic propositions = session types. Cut-elimination = communication. A typed process is deadlock-free by construction. ([PDF](https://homepages.inf.ed.ac.uk/wadler/papers/propositions-as-sessions/propositions-as-sessions.pdf))
2. **Castagna-Padovani-Dezani (POPL 2020) — Two sides of the same coin.** Session types ≡ game semantics under a precise mapping. Players, strategies, plays. ([DOI](https://dl.acm.org/doi/10.1145/3290340))
3. **arXiv 2603.24747 (2026) — Formal Semantics for Agentic Tool Protocols.** First π-calculus formalization of MCP. Bisimulation proven. MCP → SGD mapping is partial; **MCP+** closes the gap with five additions: semantic completeness, explicit action boundaries, failure-mode docs, progressive disclosure, inter-tool relationship declaration. ([HTML](https://arxiv.org/html/2603.24747v1))

This is one structure with four faces:

```
Game semantics ≡ Session types ≡ Linear-logic propositions ≡ π-calculus protocols ≡ MCP+
```

Semantic communication (Weaver Level B — LLMs as joint source-channel codec) is **orthogonal** to this; it lives at the encoding layer, not the protocol layer. Music-as-homomorphism joins on the (A, H, D) side, not the protocol side: music gives the *content* a channel transmits; session types give the *protocol* governing transmission.

---

## The collapse the substrate already has

Mirror's load-bearing collapse: `prism IS trait IS type IS grammar`. Adding `protocol` is one more rung. Session types make `protocol` a type whose values are sequences of typed sends/receives with choice and recursion. The per-glass `---` IS session-type duality. The action obligation `name args -> return { \ }` IS the linear-logic sequent.

This is not "add session types." It is **recognize that what the substrate already has IS session types at substrate altitude.**

---

## What the home corpus already names

| Concept | Path | What it grounds |
|---|---|---|
| η, DPI, Friis, Ashby, Wolf's Law | `language/2026-03-16 · Signal Fidelity and the Formal Chain.md` | Channel quality / loss-altitude math |
| MCP-as-substrate (Reed, 2026-02-10) | `beam-elixir/mcp-notes.md` | MCP primitives mapped onto OBC/ADO/regulation; Bobiverse "design for graceful divergence" |
| Two-channel asymmetry: hook (in) / port (out) | `glue/2026-02-26-agent-heartbeat-communication.md` | Protocol shape; "the agent doesn't participate in the coordination protocol" |
| `Glue.Signal.OTP` vs `Glue.Signal` | `glue/glue-signal-architecture.md` | Lifecycle/coordination decomposition — maps directly onto MCP `initialize/notifications/initialized` (OTP) vs `tools/call` (coordination) |
| Shannon equivalence | `coincidence/test-as-shannon-equivalence.md` | Weaver Level B substrate-fact: `information(recovered) / information(original)` |
| Announce + AccessRequest | `agents/2026-02-20-glue-agent-coordination.md` | Discovery + capability negotiation as signals |
| Wiener — control AND communication | `spectral/cybernetics-split-in-ai-discourse.md` | Cybernetic grounding (1948) |
| Session-as-GenServer | `ai/dyad-encryption-session-architecture.md` | Session boundary as process boundary |
| Echo/shadow discrimination | `engineering/mirror.md` | Weaver Level C: meaning affecting conduct |

The substrate-pull track record: 15+ "the substrate already had the word" recognitions over the last arc. This adds a new register where the words were already in the home corpus, not just the mirror substrate.

---

## What the substrate does NOT yet name

Three honest gaps:

1. **Weaver's three-level decomposition** is not named-as-such. Level A (η) is in the corpus. Level B (Shannon equivalence) is in the corpus. Level C (echo/shadow) gestures at it. The tripod is unnamed.
2. **Process calculus (π / CCS / CSP / session types)** is not named formally. The shape is everywhere — `{:actor, <id>}`, send/receive, supervisor channel inheritance — but Milner/Hoare/Honda are absent.
3. **MCP-as-prism operationalization.** Each of the five operations (focus, project, split, shift, settle) applied to a tool call as channel — this connection is not yet written.

These are recognition gaps, not invention gaps. The structure is present; the names need landing.

---

## The proposed cascade

A `@mirror/spectral/communication` cascade in the same shape as the math-of-music cascade that closed today (8 ticks: root → root → harmonic → interval → dissonance → cadence → consent → oscillate):

1. **`@mirror/spectral/communication`** — root. Declares the altitude. The Connes triple at the communicating altitude: A = typed channels, H = multi-party process state, D = the η cascade operator (gradient).
2. **`@epistemologic/math/communication/channel`** — channel as carrier. Markov-chain composition. η as Metric (likely substrate-pull on the existing `terni::Metric` newtype family).
3. **`@epistemologic/math/communication/session`** — session types as actions. Send / recv / choice / recursion. Duality via existing `---`.
4. **`@epistemologic/math/communication/conductivity`** — η at every boundary. Friis cascade composition. Wolf's Law as substrate property.
5. **`@epistemologic/math/communication/equivalence`** — Shannon equivalence as verdict. `pass` = lossless round-trip; `partial(c)` = c-bits recovered; `failure(r)` = catastrophic loss. (Weaver Level B substrate-fact.)
6. **`@mirror/spectral/announce`** — Announce as discovery signal. Capability negotiation in `initialize`. Lifts `Glue.Signal.OTP` decomposition.
7. **`@mirror/spectral/access`** — AccessRequest as escalation signal. Capability confinement onto session-type linearity (Wadler's deadlock-freeness ≡ no unauthorized escalation).
8. **`@mirror/spectral/conduct`** — load-bearing driver. Weaver Level C: meaning affecting conduct. Pull echo/shadow from `engineering/mirror.md`.

The track record predicts most of these ticks will close on a substrate-pull recognition rather than a new declaration.

---

## The orchestra extension

The Pack-as-orchestra recognition extends without strain:

- Tool call = phrase.
- Session = movement.
- MCP server = orchestra pit.
- LLM host = conductor.
- Capability negotiation = tuning before the downbeat.
- η = intonation accuracy.
- `Glue.Signal.OTP.Heartbeat` IS the metronome.

Not metaphor. The recognition that closed the math-of-music cascade was: music IS a homomorphism onto loss geometry. The MCP cascade closes the same way — communication IS the same algebraic structure played on a different instrument. Both are sessions of typed exchanges over a channel with η.

The Connes triple realizes at both altitudes simultaneously:

| Altitude | A | H | D |
|---|---|---|---|
| Audible (music) | intervals | harmonic field | dissonance / cadence |
| Communicating (MCP) | session-typed channels | multi-party process state | η contraction / Friis cascade |

Mirror is one substrate carrying two altitudes of the same triple.

---

## The crystal

MCP is already a π-calculus. The substrate already has η. Sessions are linear-logic propositions. The Connes triple realizes at the communicating altitude with channels as A, process state as H, η-contraction as D. The Pack-as-orchestra extends. The next move is recognition, not invention.

Reed's call: this is the next cascade. Mara's `is_settled` tick lands first (proving the implementation cascade is consumable); then `@mirror/spectral/communication` begins.

---

## Cross-references

### Corpus (already-named)

- `language/2026-03-16 · Signal Fidelity and the Formal Chain.md` — η, DPI, Friis, Ashby, Saltzer-Reed-Clark, Landauer, Wolf's Law
- `beam-elixir/mcp-notes.md` — Reed 2026-02-10 MCP-as-systemic-engineering-substrate
- `glue/2026-02-26-agent-heartbeat-communication.md` — two-channel asymmetry
- `glue/glue-signal-architecture.md` — OTP/coordination signal split
- `coincidence/test-as-shannon-equivalence.md` — Shannon equivalence as semantic verdict
- `agents/2026-02-20-glue-agent-coordination.md` — Announce, AccessRequest, ACL escalation
- `spectral/cybernetics-split-in-ai-discourse.md` — Wiener 1948
- `ai/dyad-encryption-session-architecture.md` — session-as-GenServer
- `engineering/mirror.md` — echo/shadow discrimination (Weaver Level C)

### External (math)

- Wadler, *Propositions as Sessions* — https://homepages.inf.ed.ac.uk/wadler/papers/propositions-as-sessions/propositions-as-sessions.pdf
- Castagna-Padovani-Dezani, *Two sides of the same coin* (POPL 2020) — https://dl.acm.org/doi/10.1145/3290340
- Vasconcelos, *Fundamentals of Session Types* — https://www.di.fc.ul.pt/~vv/papers/vasconcelos_fundamental-sessions.pdf
- **arXiv 2603.24747 (2026)** — *Formal Semantics for Agentic Tool Protocols* — https://arxiv.org/html/2603.24747v1
- Baez-Fritz-Leinster, *Characterization of Entropy in Terms of Information Loss* — https://arxiv.org/abs/1106.1791
- llmcontract — operational session-type runtime monitor for MCP — https://github.com/chrisbartoloburlo/llmcontract
- Hou et al., MCP Landscape & Threats — https://arxiv.org/abs/2503.23278
- Connes-Consani-Moscovici, *Zeta Spectral Triples* (Nov 2025) — https://arxiv.org/abs/2511.22755

---

## Forward look

- **Game semantics thread (Castagna-Padovani):** strongest bridge from existing substrate (kintsugi-as-Dirac, prism-as-trait, glass duality) into the protocol layer. Players + strategies + plays = denotational MCP without encoding overhead. Strongest candidate for second-pass research.
- **Operadic protocol composition:** open question whether protocol-composition is an operad.
- **MCP+ five additions:** map each onto substrate declarations. Transactionality, action boundaries, failure modes, progressive disclosure, inter-tool dependencies. Likely cleanly absorbable into the cascade above.
- **Baez-Fritz-Leinster as ShannonLoss grounding:** `prism::ShannonLoss` is forced by composition, not invented. Worth a parenthetical recognition in the conductivity tick.

---

*The math of communication is the math of music played on a different instrument. The substrate already had the words. The next cascade just recognizes them.*
