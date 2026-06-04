# `mirror kintsugi` GitHub Action

Run the kintsugi loop over a corpus; emit a typed verdict.

Per the [kintsugi-ci-v0.1 spec](../../docs/specs/kintsugi-ci-v0.1.md).

## Usage

```yaml
- uses: systemic-engineering/mirror/actions/kintsugi@v0.1
  with:
    target: ./src
    shatter: '4'
    threshold: '0.8'
    fail-on: failure
```

## Inputs

| Input | Required | Default | Description |
|---|---|---|---|
| `target` | yes | — | Path the loop walks (file or directory) |
| `threshold` | no | `0.8` | Acceptance threshold for partial verdicts (0.0–1.0) |
| `shatter` | no | `4` | Shatter-N iteration depth |
| `fail-on` | no | `failure` | Severity that fails the gate: `failure` \| `partial` \| `none` |
| `mirror-version` | no | `github.action_ref` | Override the mirror binary version |
| `post-comment` | no | `true` | Post a PR comment with the verdict breakdown |

## Outputs

| Output | Description |
|---|---|
| `verdict` | `pass` \| `partial` \| `fail` |
| `confidence` | Numeric confidence in `[0.0, 1.0]` |
| `objective` | The `kintsugi_objective` scalar (per kintsugi-variety §6) |
| `opacities` | JSON array of located opacities (gold-filled cracks) |

## Artifacts

The substrate-native verdict (`kintsugi-verdict.mirror`) and the JSON
crossing (`kintsugi-verdict.json`) are uploaded as the
`kintsugi-verdict` artifact for every run. The mirror-text form is the
substrate's truth; JSON is the `@io` boundary for tools that need to
parse it (jq, GitHub UI, downstream automation).

## Local parity

Run the same logic locally:

```bash
just kintsugi-ci-local <target> <shatter> <threshold> <fail_on>
```

The `just` recipe runs the same shell commands the action runs; the
two verdict files are byte-identical modulo path normalisation. This
is the operational version of the [kintsugi-thesis](../../docs/cicd/kintsugi-thesis.md)
Claim 1 (determinism) + Claim 8 (local/CI parity) at the wire altitude.

## How it ships

This action lives in-tree at `mirror/actions/kintsugi/`. Releases tag
both the binary and the action together; the floating `@v0.1` tag
resolves to the most recent `v0.1.x` release. Per
[kintsugi-ci-v0.1 §7.1](../../docs/specs/kintsugi-ci-v0.1.md).
