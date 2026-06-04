# v0.1.0 — kintsugi CI release

**Status:** ready to cut (T11.7 gate). All scaffolding landed; awaiting workflow validation.

**Spec:** [`docs/specs/kintsugi-ci-v0.1.md`](../../docs/specs/kintsugi-ci-v0.1.md)

---

## What v0.1.0 ships

A GitHub Action that runs the kintsugi loop over a corpus and emits a typed verdict:

```yaml
- uses: systemic-engineering/mirror/actions/kintsugi@v0.1
  with:
    target: ./src
    shatter: '4'
    threshold: '0.8'
    fail-on: failure
```

Users of this action get pass/partial/fail gates on their own repos
from day one. The substrate ships its own thesis at the wire altitude.

## Tick status

| Tick | Status | Commit | What |
|---|---|---|---|
| T11.1 | ✅ | pq spec | pq typed DSL (Target/Filter/Output) in prism_core |
| T11.2 | ✅ | | `mirror kintsugi --ci` flag + JSON serialiser |
| T11.2.5 | ✅ | | substrate-pull correction: mirror-text default + `--format=json` flag |
| T11.2.6 | ✅ | | typed Verdict / CorpusVerdict in `boot/std/kintsugi.mirror` |
| T11.3 | ✅ | | `--target <dir>` corpus walker |
| T11.4 | ✅ | `952f622` | `actions/kintsugi/action.yml` composite + bin scripts + Justfile recipe |
| T11.5 | ✅ | `f61a5ca` | `.github/workflows/kintsugi.yml` (self-host gate) |
| T11.6 | ✅ | `f61a5ca` | `.github/workflows/release.yml` (build + sign + attach + tag) |
| T11.7 | 🔴 | — | cut `v0.1.0`; floating `v0.1` resolves |
| T11.8 | ⬜ | post-release | switch internal call site to `@v0.1` published reference |

## Cut criterion (T11.7)

All three must hold:

1. **T11.5 green.** Mirror's own `kintsugi.yml` runs green against `boot/std` via the in-repo `./actions/kintsugi` path on `reed/gap-substrate-fold` (or whichever branch lands the workflows to main).
2. **T11.6 produces artifacts.** Tag push triggers the release workflow; binaries for all four targets land attached to the GitHub Release for `v0.1.0` with SHA-256 checksums.
3. **Floating tag resolves.** `git fetch && git checkout v0.1` resolves to `v0.1.0`'s commit.

Execution: tag `v0.1.0` on the commit where T11.5 + T11.6 are green; force-push `v0.1` floating tag.

## v0.1 vs v1.0 framing

From [the roadmap README](../README.md): **v1.0 = the spectral.engineer cloud deployment.** Actual semver: `v0.1.0`.

- **v0.1.0** = kintsugi wired up in GitHub Actions, mirror as the Actions provider. **Ships now.**
- **v0.2 … v0.N** = capability accretion as Phase 2–6 land. Each phase enables substrate features that compose into the eventual cloud surface.
- **v1.0** = spectral.engineer cloud deployment. The release surface that's *deliberately framed* as v1.0 for substrate-tuning purposes; the semver counter says otherwise.

This distinction matters because v0.1.0 doesn't gate Phase 0–7 progression. The phases continue post-tag; v0.1 just lets the substrate ship a real artifact users can consume while the bootstrap retirement unfolds. See [`phase-0-current-state.md`](./phase-0-current-state.md) for the orthogonal progression.

## Cross-references

- Spec: [`docs/specs/kintsugi-ci-v0.1.md`](../../docs/specs/kintsugi-ci-v0.1.md)
- Composite action: [`actions/kintsugi/`](../../actions/kintsugi/)
- Self-host workflow: [`.github/workflows/kintsugi.yml`](../../.github/workflows/kintsugi.yml)
- Release workflow: [`.github/workflows/release.yml`](../../.github/workflows/release.yml)
- Justfile local parity: `just kintsugi-ci-local <target> <shatter> <threshold> <fail_on>`
- Local/CI parity discipline: [`docs/cicd/kintsugi-thesis.md`](../../docs/cicd/kintsugi-thesis.md) (Claim 1 + Claim 8)
- Phase progression context: [`phase-0-current-state.md`](./phase-0-current-state.md)
- v1.0 framing: [`v1-launch.md`](./v1-launch.md)
