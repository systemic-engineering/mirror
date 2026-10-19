# mirror

mirror is a research compiler for content-addressed, graph-native reasoning. Programs are written in a small declarative grammar (`.mirror` files, called *shards*). Every compiled artifact is identified by a hash of its normalised syntax tree (an *OID*): same source, same OID. Uncertainty is part of the language. A typed hole `\` marks a place the author has not resolved yet, and compilation returns a ternary result, Success, Partial (with a measured loss) or Failure, rather than pass/fail. The numerical core (spectral graph operators, fixed points, loss accounting) lives in the sibling library [prism](https://github.com/systemic-engineering/prism). mirror composes over it.

The project is built through an agentic loop: named AI agents with signed commit identities write the specs, the implementation, the adversarial reviews and the verification reports. Alex Wolf makes the architectural decisions. Every step is a signed commit, so the provenance can be audited (see [How it's built](#how-its-built)).

## Status

**Research project, in the middle of a bottom-up rewrite. No stable release, no semver.**

The original Rust compiler (`bootstrap/`) was deleted on 2026-08-26. Its replacement, `rust/`, is being grown one primitive at a time. Most of the recent work (September–October 2026) is mathematics: docs in `docs/math/` with adversarial audits in `docs/audits/`. The code in `rust/` was last changed on 2026-09-11.

Verified on 2026-10-19 (macOS arm64, `nix develop`):

| | State |
|---|---|
| `rust/` (the `mirror` binary) | **Builds** against prism commit `62a9fa5` (2026-09-05), tagged `mirror-compat-2026-09`. **Does not build** against prism's current `main`: prism renamed `SheafOfShardGraph`, `ComposedSheaf` and `Harmonics` after 2026-09-19, and from 2026-09-16 it links against `gfortran`, which mirror's dev shell does not provide. |
| `mirror` binary unit tests | 175 passed, 0 failed, 1 ignored |
| `rust/tests/red_spec_claims.rs` | 9 passed, 10 failed. Deliberate: each test asserts a claim from a spec that the code does not implement yet. The failures list the gap between spec and code. |
| `rust/tests/red_liquid_pillar_i_commutator_antisymmetric.rs` | Does not compile (it imports `prismqueer::liquid`, which has since been renamed `flux`) |
| `rust/spectral` crate | 97 passed |
| `rust/fractal` crate | 32 passed. `fire_d_measure`: 2 passed, 2 failed (a RED scaffold, failing by design) |
| `rust/matrix`, `rust/roomba` crates | The libraries build. Their test modules do not compile (same stale `prismqueer::liquid` import). |

What the binary does today:

- `mirror love <a> <b> <composed>` works. It compares the Laplacian spectra of two graphs and their composition (e.g. `mirror love K2 K3 K5`), and exits `0`/`1`/`255` for Green/Yellow/Red.
- `mirror compile <file>` runs. On the shards in this repo it currently reports a chain of 0 crystals, so it does not yet produce useful output.
- `roomba`, `serve --mcp`, `bumblebee` and `bench` are wired up but were not exercised in this verification pass.
- `craft boot` fails (it tries to read a directory as a file). `kintsugi`, `index`, `shatter`, `init`, `recall`, `beam` and `peer` are listed in `--help`, but they print "dispatch lands at M3+" and exit 2.
- The `Justfile` still points at `bootstrap/Cargo.toml`, so `just build` and `just test` do not work. Use the cargo commands below.

The active line of work is tracked in [`docs/loop/CURRENT.md`](./docs/loop/CURRENT.md).

## Quickstart

mirror depends on prism by relative path (`../../prism/prismqueer` from `rust/`), so the two repositories must sit side by side. The prism checkout must be at a compatible revision (see Status).

```sh
git clone https://github.com/systemic-engineering/mirror
git clone https://github.com/systemic-engineering/prism
git -C prism checkout mirror-compat-2026-09 && git -C prism submodule update --init

cd mirror
nix develop     # Rust toolchain, flang, LAPACK/BLAS

cargo build --release --manifest-path rust/Cargo.toml
MIRROR=${CARGO_TARGET_DIR:-rust/target}/release/mirror
$MIRROR --help
$MIRROR love K2 K3 K5          # composed spectrum; exits 1 (Yellow)
$MIRROR love K5 K5 K5          # self-composition is refused; exits 255 (Red)

cargo test --manifest-path rust/Cargo.toml --bin mirror    # 175 passed, 1 ignored
```

## Structure

```
rust/            the Rust floor: the `mirror` binary (src/) plus four path crates
  src/           main.rs (CLI), compile.rs, love.rs, phone.rs (I/O), wire.rs (JSON), apply_h.rs
  fractal/       OIDs, crystals, signing and provenance (Witnessed)
  matrix/        LAPACK-backed numerics, registry
  spectral/      property and pillar checks over prismqueer
  roomba/        "mend": finds and collapses redundant grammar arms
shards/          ~410 .mirror grammar declarations (the language and its library)
boot/            ~150 older boot grammars; shards/ supersedes them
mirror.spec      mirror's description of itself, written in its own grammar
docs/math/       mathematical foundations (math defines; specs cite)
docs/specs/      design specs (~256)
docs/audits/     adversarial reviews (~129)
docs/scouts/     read-only "floor truth" reports on what the code actually does (~100)
docs/loop/       CURRENT.md: the active arc
spectral.engineer/  submodule: the curated corpus and package garden (SEL-licensed)
```

## Core ideas

Each idea has a one-line gloss and a link to the doc that develops it. The docs are research notes: the claims in them are argued, not machine-checked, unless a test says so.

- **Content addressing.** An artifact's identity is a 32-byte hash of its β-normalised syntax tree, and compiled results chain by OID (`rust/fractal`). See [`docs/math/2026-08-10-mara-beta-normal-ast-content-addressing-math-foundation.md`](./docs/math/2026-08-10-mara-beta-normal-ast-content-addressing-math-foundation.md).
- **Sub-Turing by design.** The grammar is meant to stay decidable: inference runs inside a state space that is bounded up front, not in a general-purpose interpreter. This is a design constraint the code is meant to respect. It has not been proven. See [`docs/math/2026-07-25-sub-turing-geometric-compiler-floor.md`](./docs/math/2026-07-25-sub-turing-geometric-compiler-floor.md) and §5.1 of [`docs/math/FLOOR.md`](./docs/math/FLOOR.md).
- **Typed holes `\`.** `\` marks an unresolved inference site in a shard body, e.g. `\splinter(ast)` in [`shards/epistemologic/cybernetic/algedonic.mirror`](./shards/epistemologic/cybernetic/algedonic.mirror). A grammar with holes compiles, but its result is Partial, not Success.
- **Settle and fixed points.** "Settle" means running a graph's heat flow until it stops moving. Concretely, it projects onto the kernel of the graph Laplacian, and what remains moving is reported as Partial. This is implemented in prism (`prismqueer::spectral::settle`). See [`docs/math/sheaf/laplacian.md`](./docs/math/sheaf/laplacian.md) and [`docs/math/FLOOR.md`](./docs/math/FLOOR.md).
- **Measurable loss.** Every stage returns `Imperfect<T, E, L>` from [terni / imperfect](https://github.com/systemic-engineering/imperfect): Success, Partial(value, loss) or Failure(error, loss). Loss is a value with laws, not a log line.
- **`fate`, the model selector.** A small, fixed classifier picks which of five inference modes runs next. It is 425 `u8` parameters: five weight sets (one per current mode), each a 5-way linear classifier over 16 spectral features (5×16 weights + 5 biases). The f64 training form has 450 parameters. It is executed by a fixed Brainfuck program (`fate.bf`) interpreted over a 256-byte tape. The code is in prism (`prismqueer/src/fate/`; the library builds, but its test module currently does not compile), and the grammar is in [`shards/fate.mirror`](./shards/fate.mirror).
- **Observers that disagree.** The latest math asks what two partial views of one graph can and cannot agree on (cohomology of covers). See [`docs/math/2026-10-01-mara-the-comma-between-observers-math-foundation.md`](./docs/math/2026-10-01-mara-the-comma-between-observers-math-foundation.md) and [`docs/math/2026-10-10-mara-the-chord-math-foundation.md`](./docs/math/2026-10-10-mara-the-chord-math-foundation.md), with Seam's audits in [`docs/audits/`](./docs/audits/).

Index of the math: [`docs/math/README.md`](./docs/math/README.md).

## How it's built

The work runs through a fixed rotation of agents (Claude Code sessions with distinct names, roles and SSH-signed commit identities) and one human:

| Role | Who | Commits |
|------|-----|---------|
| Architecture decisions, ratification | **Alex Wolf** (human). Decisions are made in conversation and quoted in the commit messages that act on them (442 of 2,983 commit subjects cite Alex). | 7 |
| Orchestration, RED-first tests, implementation | **Reed** `reed@systemic.engineer` | 1,631 |
| Math foundations and specs | **Mara** `mara@systemic.engineer` | 978 |
| Adversarial review (`docs/audits/`) | **Seam** `seam@systemic.engineer` | 145 |
| Read-only verification against the code (`docs/scouts/`) | **Taut** `taut@systemic.engineer` | 117 |
| Essays and prose | **Glint** `glint@systemic.engineer` | 78 |
| Other (automation identity `mirror@…`, Loki) | | 27 |

Counts come from `git shortlog -sne`, merging each name's addresses, as of 2026-10-19. About 97% of commits carry an SSH signature.

The handoffs are explicit: Taut reports what is actually in the code → Mara writes the math and spec → Seam audits it and issues corrections, which Mara folds in (visible in the log as alternating `Seam §N` / `Mara fold §N` commits) → Reed implements against a committed RED test → Alex decides. Commits carry phase markers (🔴 red, 🟢 green, ♻️ refactor, 🔧 tooling, 📝 docs), and a 🟢 must follow a 🔴. Full conventions: [`AGENTS.md`](./AGENTS.md) and [`CONTRIBUTING.md`](./CONTRIBUTING.md).

## License

Layered, per [`LICENSE.md`](./LICENSE.md):

- The compiler, the grammars (`boot/`, `shards/`), the Prism algebra and the open adapters are **Apache-2.0** ([`license/APACHE2.md`](./license/APACHE2.md)).
- The curated corpus, the `@spectral/garden` reviewed packages and operational deployments built on mirror are under the **systemic.engineering License (SEL)**, v1.1 effective 2026-05-29 ([`license/SEL.md`](./license/SEL.md)).
- The `@spectral/db` engine is closed source. Garden packages are licensed per curator.

## Related

- [prism](https://github.com/systemic-engineering/prism): `prismqueer` (the numerical and algebraic core mirror composes over), `terni` and `prismqueer-projections`. Apache-2.0.
- [imperfect / terni](https://github.com/systemic-engineering/imperfect): the `Imperfect<T, E, L>` type.
- Essays: [*Imperfect*](https://systemic.engineering/imperfect), [*Arrival*](https://systemic.engineering/arrival), [*Manifesto*](https://systemic.engineering/manifesto), [*Damn, Failed*](https://systemic.engineering/damn-failed). More at [systemic.engineering](https://systemic.engineering).
