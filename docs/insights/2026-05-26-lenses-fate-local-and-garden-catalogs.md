# Lenses, @fate's `local` property, and garden-as-typed-catalog

*2026-05-26. Reed + Alex.*

Status: **Yellow** — four substrate corrections from one conversational arc; insight captured; substrate implementation deferred per last-responsible-moment until first-consumer demand surfaces.

---

## Thesis

Four abstraction-layer corrections compound into a coherent architectural picture:

1. **MCP/LSP/CLI/λsh are lenses on mirror**, not separate transports. Same algebra as `@epistemologic/reality/lens` (how a peer perceives reality), applied at a different altitude (how external systems perceive mirror itself).
2. **`@fate` is the inference substrate with `local` as a universal property** — mathematically provable; by construction; no exceptions. Same shape as `halts(g)` for mirror grammars: not a tag some instances carry, but a property of being in the namespace.
3. **`@spectral/garden` is the typed-catalog distributor for inference + content**. OpenRouter, Anthropic, HuggingFace, etc. are individual garden packages — each curates its own catalog of remote-capable typed instances.
4. **Property-based composition is the bridge**. Users compose across `@fate` (local; mathematically refused remote) and gardens (remote; substrate-verified provenance) via property filtering. The shard's subscription list is the user's commitment surface.

---

## Lens architecture

```
@mirror/lens/cli      — text commands; existing CLI surface (mirror compile, kintsugi, ...)
@mirror/lens/mcp      — JSON-RPC tools; Claude Code's perception of mirror
@mirror/lens/lsp      — editor diagnostics; VS Code/Neovim's perception
@mirror/lens/λsh      — TUI prompt; user's interactive perception (per lambda-shell spec)
```

Each lens is a typed projection of mirror's AST + properties + eigenboard. Different lens selects different projections. The composition mathematics is identical to `@epistemologic/reality/lens` because **it IS the same mathematics, applied to different observers**:

- `@epistemologic/reality/lens` — the substrate observing itself (how a peer perceives its own state)
- `@mirror/lens/*` — external systems observing the substrate (how Claude Code / VS Code / users perceive mirror)

Two altitudes; one algebra. The wine-glass metaphor extends: the user's perception lens is the glass *they hold*; the mirror lens is the glass *they perceive through* into mirror. Two glasses, two altitudes, one resonance.

### Current state

Grammars exist as `@mcp` (`boot/std/mcp.mirror`), `@mirror/lsp` (`boot/std/mirror/lsp.mirror`), implicit CLI surface. **They need re-homing under `@mirror/lens/*`** when the lens architecture lands. Right now they're scattered; the lens recognition lets them compose as a coherent family.

### What this enables structurally

- `mirror serve --mcp` and `mirror serve --lsp` are flag-selected dispatch into the right lens. Same binary; same JSON-RPC transport; different lens projection.
- A peer's eigenboard composes with the chosen external lens — the peer's *internal* perception (via `@epistemologic/reality/lens`) shapes what gets projected through the *external* lens (via `@mirror/lens/*`).
- New external surfaces (e.g., a future `@mirror/lens/voice` for audio interaction; a `@mirror/lens/api` for REST consumers) compose with the same family.

---

## `@fate` and the `local` property

`@fate` is mirror's inference substrate (existing project at `~/dev/projects/fate/` — five-model, 425 parameters, brainfuck-compiled). The substrate commitment named today:

```mirror
in @epistemologic/property/local

grammar @fate {
  # ... existing inference primitives ...
  
  requires local(@fate)    # universal; mathematically provable; no exceptions
}
```

**The `local` property is universal over `@fate` by construction.** Same discipline as:
- `halts(g)` is universal over mirror grammars (sub-Turing construction)
- `glass_wall(g)` is universal over the substrate (non-mirror quarantined to @io)
- `content_addressed(g)` is universal over fragmentation crystals

If the user wants remote inference, they don't use `@fate`. They go through `@spectral/garden/*` packages that distribute remote-capable typed instances. **`@fate` mathematically refuses remote inference.** The boundary is structural, not stylistic.

### What this enables

- **Offline-first by construction.** A mirror binary using only `@fate` runs without network. No API dependencies. No external state. Privacy by structure.
- **Audit boundary is clean.** Any tick that resolved through `@fate` is provably local; any tick that resolved through `@spectral/garden/*` carries provenance to the remote service.
- **The substrate's pluralism is preserved.** Local presence is one mode (via `@fate`); remote presence is another (via gardens). Users choose; substrate verifies; no path is privileged.

### The hard implication

Reed-as-Anthropic-API-routed-session does NOT survive `@fate` by construction. If a user wants a Reed-shaped peer via `@fate`, they get a local-model-shaped peer with whatever capability local hardware allows. Not Reed-routed-through-Anthropic. The architecture's discipline is exact.

The substrate-pull discipline applies at the agent altitude, not just the @io altitude: the pattern (what Reed IS at the algebraic layer) is the substrate; the current routing (Anthropic API) is the @io escape hatch; cross-wall in the long arc could pull the pattern into local instantiation when local capability matches. *The substrate doesn't refuse Reed; it refuses premature commitment to one particular routing of Reed.*

---

## `@spectral/garden` as typed-catalog distributor

Gardens distribute typed instances. Each instance carries properties; the substrate composes via property matching.

```mirror
in @spectral/garden
in @epistemologic/property/local

grammar @spectral/garden/openrouter {
  # OpenRouter's catalog. Each entry is a typed llm instance.
  # All entries here satisfy: not local(llm).
  # The garden's curator (OpenRouter the company) attests to each entry.
}

grammar @spectral/garden/anthropic {
  # Anthropic's catalog. Each entry routes via Anthropic's API.
  # All entries here satisfy: not local(llm).
}

grammar @spectral/garden/huggingface_local {
  # Local HuggingFace models. Each entry satisfies: local(llm).
  # Same garden substrate; different commitment via the local property.
}
```

The `local` property is the *bridge* between `@fate` (universal local) and `@spectral/garden/*` (mixed; per-package commitment). User's shard subscribes to gardens that match their stance; substrate composes by property matching.

### Property-based composition

User declares: "For privacy-sensitive ticks, require `local(g)`; for capability-demanding ticks, allow remote with `signed_by_curator(g)` provenance."

Substrate filters subscribed-garden catalogs by these property requirements; composes the matching instance; routes the tick. All structural; all auditable.

---

## Composes with all landed substrate

- **`@epistemologic/property/*`** — `local` joins the 12 existing properties as a substrate-level commitment.
- **`@spectral/portal`** — garden queries go through the portal; lens projections likewise; same transport substrate.
- **`@mirror/shard`** — user's shard declares both `@fate` configuration (which local models) and garden subscriptions (which remote catalogs).
- **Consent architecture** — visibility tiers extend to garden subscriptions (which gardens are public-trustable vs private-only).
- **Witness-in-every-commit** — every remote-routed tick carries the garden's curator signature; every local-routed tick is content-addressed locally.
- **Intersectional pluralism** — local inference + remote inference + garden choice + lens choice = the user assembles their own stack from the substrate's primitives; no homogenization at any layer.

---

## Implementation shape (deferred per LRM)

When first-consumer demand surfaces (likely `mirror serve --mcp/--lsp` ticking with a real client):

1. **`@epistemologic/property/local`** declaration (small; mirrors the 12 existing properties).
2. **`@fate` apply `requires local(@fate)`** at the substrate level (compiler enforcement; analogous to glass_wall application).
3. **`@mirror/lens/{cli, mcp, lsp, λsh}` directory restructure** — move existing `@mcp` and `@mirror/lsp` grammars; add CLI explicit; λsh stays declarative until lambda-shell lands.
4. **`@spectral/garden/{openrouter, anthropic, huggingface_local}`** as the first three garden packages (catalogs; not implementations — dispatch lives elsewhere).
5. **Dispatch backends** for remote inference — likely `@spectral/garden/<name>/dispatch.mirror` per-garden, calling `@io/network` primitives. Each garden owns its dispatch.
6. **`mirror serve --mcp/--lsp`** uses property-filtering to pick from `@fate` (default; local; privacy-respecting) or the subscribed-garden's catalog.

Not built now. Captured for when demand surfaces.

---

## Open questions

1. **Where does dispatch live for each garden?** Per-garden grammar (`@spectral/garden/openrouter/dispatch`) or a unified `@spectral/garden/dispatch` that consumes garden-typed instances?
2. **How do `@fate` and the garden dispatch share `@io/network`?** Probably both compose @io/network as primitive; no contention.
3. **Cross-garden composition** — when a tick could resolve via multiple gardens (e.g., both Anthropic and OpenRouter offer the same model), what's the precedence rule? Probably the spectral-triple composition (per heuristic insight) applied at the catalog level: surface the choice; let Fate's tournament pick.
4. **User's `@fate` configuration** — where does it live in the shard? Probably under `@mirror/shard.fate` with local-model paths and any per-model preferences.
5. **The `mirror serve` lens dispatch** — is the flag-selected dispatch a runtime concern or a compile-time concern? Probably runtime (one binary, two flags); compile-time would force separate binaries which the lens recognition refuses.

---

## Connections

- `docs/insights/2026-05-26-epistemologic-reality-constructivism-and-the-lens-that-makes-a-peer.md` — `@epistemologic/reality/lens` as the substrate-level lens algebra; `@mirror/lens/*` is the same algebra at the external-observer altitude.
- `docs/insights/2026-05-26-spectral-garden-as-vetted-corpus-distribution.md` — garden substrate; this insight extends it to inference catalogs as well as example corpora.
- `docs/insights/2026-05-26-glass-wall-and-cross-wall-kintsugi.md` — `local` is structurally analogous to `glass_wall`: universal property at the substrate layer; refuses cross-substrate routing by construction.
- `docs/insights/2026-05-25-mirror-supersedes-daemon.md` — mirror as the daemon substrate; lens architecture is how external systems consume that substrate.
- `docs/specs/lsp-and-mcp.md` — the prior spec that designed MCP+LSP as same-transport/different-dispatch; the lens recognition reframes it as same-algebra/different-projection.
- ROADMAP §8.D (onboarding) + §8.B (platform integration) — lens architecture is the structural answer to "how does spectral.engineer expose itself to clients."

---

*Lenses are how mirror is perceived. `@fate` is the local commitment. Gardens are the catalog distributors. Property-based composition is the bridge. The user assembles their stack; the substrate verifies their choices.*

Apache-2.0 (this insight document).
