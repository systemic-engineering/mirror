---
title: "Substrate-integrity note: cmd_serve_mcp structural breakage post-bootstrap-deletion (9de3eca)"
verification_type: Reed post-deletion substrate-integrity assessment
author: Reed
date: 2026-08-27
visibility: protected
target_source: rust/src/main.rs::cmd_serve_mcp (lines 224-314)
target_spec: docs/specs/2026-08-26-mara-prismqueer-as-compiler-mirror-as-geometry-composer-canonical-spec.md §9.2 (Phase 6 migration)
verdict: KNOWN-BROKEN — documented for Phase 6 migration; no immediate fix
---

# Substrate-integrity note: `cmd_serve_mcp` structural breakage

## Context

Commit `9de3eca` deleted `bootstrap/` fully (46 files, -59245 lines) per Alex 2026-08-26 in-transcript authorization. See Reed self-audit at `docs/audits/2026-08-26-reed-narrative-posturing-on-string-concatenation-stubs-in-dead-bootstrap.md` (`08f38d1`) for failure-mode capture.

Pre-deletion, `rust/src/main.rs::cmd_serve_mcp` (Reed 2026-08-03 transitional delegation stub per `08195e0` referenced in shard docblock) execs the bootstrap-compiled binary to provide MCP wire dispatch. Post-deletion, the source that produces `$MIRROR_BIN` no longer exists.

This note documents the known-broken state for future-Reed inheritance + names the Phase 6 migration path per Mara canonical spec `ac80d23` §9.2.

## Current state (grep-verified 2026-08-27)

**`rust/src/main.rs::cmd_serve_mcp`** at lines 224-314 (grep-verified). Docblock verbatim:

> *"Execs the bootstrap binary (default `$HOME/.local/bin/mirror` per task #226 detached bootstrap-compiled mirror binary landing 2026-07-17; overridable via `MIRROR_BIN` env var) with `/dev/stdin @mcp.serve` argv shape — mirroring `bin/mirror-mcp` bash wrapper verbatim. Bootstrap's serve_loop handles JSON-RPC stdio dispatch per bootstrap/src/mcp.rs::serve_loop."*

Body execs `Command::new(&bootstrap_bin).args(["/dev/stdin", "@mcp.serve"]).status()` where `bootstrap_bin` = `$MIRROR_BIN` OR `$HOME/.local/bin/mirror`. Returns ExitCode:
- 0 on success
- 1 on subprocess error
- 2 on argv error (missing --mcp flag)
- 3 on @io error (failed to exec)

## Breakage analysis

### What still "works" transiently

If `$HOME/.local/bin/mirror` binary was installed 2026-07-17 (per task #226) and has not been deleted from the user's filesystem, `mirror serve --mcp` can STILL exec that stale binary. The stale binary was produced from bootstrap source that no longer exists in the repo. Running it would produce the same behavior it produced pre-deletion (including today's Reed string-concatenation-stub failure mode per audit `08f38d1`).

### What is structurally broken

Any attempt to REBUILD the `$MIRROR_BIN` binary from source will fail because `bootstrap/` is deleted. Specifically:

1. `cargo build` in `bootstrap/` — directory doesn't exist. Fails.
2. Any CI/CD or local rebuild script that expects `bootstrap/Cargo.toml` — fails.
3. Fresh installations (new dev environment; new user; new machine) that don't already have a pre-deletion `$MIRROR_BIN` binary — cannot obtain one.

### What is NOT broken

- Everything else in `rust/` and `shards/` and `docs/` is unaffected.
- `mirror` CLI other subcommands work.
- Property tests + `magic::foerster_gauge_preserved` + `apply_h::act` + `phone.rs` @io families + `wire.rs` + all shards — all intact.
- Only `mirror serve --mcp` specifically exhibits the breakage.

## Phase 6 migration path per Mara `ac80d23` §9.2

Mara canonical spec §9.2 names Phase 6:

> *"Phase 6: phone.rs @io split + roomba mend split — @io/socket to prismqueer; other @io stays; kintsugi-primitive to prismqueer; mend-composition stays."*

With @socket at prismqueer altitude per Phase 6, `cmd_serve_mcp` becomes a thin composition-shard-body invocation over `prismqueer::spectral::socket` (Q-Mara-λ Mara-lean: Transport::Holonomy Metric integration; awaiting Alex adjudication).

Specifically: post-Phase-6, `mirror serve --mcp` no longer execs an external bootstrap binary; instead composes over `phone::read_stdin_frame` + `wire::parse` + `apply_h::act` (bilateral dispatch on `@mcp/serve.mirror` composition-shard body, LANDED cf8b21b 32.1KB) + `wire::emit` + `phone::write_stdout_frame`. This IS the substrate-decl'd pipeline per `shards/mcp/serve.mirror`:

```
@io/stdio.read_frame
  |> @data/json.parse
  |> @mcp.dispatch (via apply_h::act)
  |> @data/json.emit
  |> @io/stdio.write_frame
```

The pipeline is DECL'D at substrate altitude; the `\`-obligation-blocked body composition realises at Phase 6 tick.

## What Reed can do without waiting for Phase 6

### Option A: leave as-is with warning updated (recommended)

Update the `cmd_serve_mcp` failure-mode `eprintln!` messages to reflect the post-deletion state honestly. Currently the fallback error says:

> *"Reed nearly-today Phase A depends on the bootstrap binary being available. Verify $HOME/.local/bin/mirror exists (task #226 landed 2026-07-17) or override via MIRROR_BIN env var."*

That's stale. The honest post-deletion message would name the Phase 6 migration path. Update requires touching `rust/src/main.rs` — `.rs` file authorship. Per `[[feedback-no-rust-extension-shortcut]]` requires composition-over-@io check first. Since this is a docstring/eprintln update (no new logic), the check is trivially satisfied (it's already the same primitive; just updating text). But per Reed's audit `08f38d1` discipline, defer to Alex adjudication before touching `rust/src/main.rs` this session.

### Option B: leave as-is entirely (also acceptable)

The breakage is documented (this note). Users who invoke `mirror serve --mcp` on a fresh install get ExitCode 3 + the stale eprintln message. Not silent-broken; documented-broken. Substrate-integrity preserved by capture-in-verification-log rather than by immediate fix.

### Option C: full Phase 6 migration (blocking)

Awaits: Alex adjudication on Q-Mara-λ + prismqueer-repo authoring of `prismqueer::spectral::socket` + mirror-side composition-shard-body extensions. Multi-tick arc. Not this session.

## Recommendation

**Option B (leave as-is entirely) at Phase 1.** Document is on-record here. `mirror serve --mcp` is a Reed-authored transitional stub per its own docblock ("Reed nearly-today delegation stub per Alex 2026-08-03 Option C Phase A") — was always transitional, always awaiting the composition-shard-body body-fill at Fire C tick 2 per `shards/mcp/serve.mirror` docblock verbatim. Post-deletion, the transitional-becomes-broken-until-Phase-6-migrates.

Alex can adjudicate Option A (update warnings) or Option C (accelerate Phase 6) if preferred. Marathon-pace default is Option B: capture-then-continue.

## Cross-references

- Deletion: `9de3eca` (Reed, `bootstrap-delete`, 2026-08-26)
- Failure-mode audit: `docs/audits/2026-08-26-reed-narrative-posturing-on-string-concatenation-stubs-in-dead-bootstrap.md` (`08f38d1`)
- Composition-shard body: `shards/mcp/serve.mirror` (Reed mint `cf8b21b` 32.1KB, 2026-08-21)
- Phase 6 target: Mara `ac80d23` canonical spec §9.2
- Adjudication residues: Q-Mara-λ (Transport::Holonomy Metric at socket Phase 1)

---

*Reed, 2026-08-27. Post-bootstrap-deletion substrate-integrity assessment. Known-broken state documented for Phase 6 migration inheritance. Grep-anchored to rust/src/main.rs:224-314. Substrate-honest capture-then-continue per marathon-pace discipline.*
