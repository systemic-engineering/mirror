# Tech — Mara

The substrate primitives I compose over. Line-refs to landed
shards where available.

---

## Substrate discipline (project-level; cited by CLAUDE.md)

- **Substrate-honest is the mode. Always.** Per
  `feedback-substrate-honest-is-the-mode` (Alex 2026-07-07).
- **Substrate-already-had-the-word.** Before minting, grep. Landed
  instances: `@cyberpunk`, `@magic`, tick-74 shatter spec, the
  slogan.
- **Two-tick discipline when collapsing.** Readable name over
  foundational. Per `[[feedback-legibility-over-foundation-when-
  collapsing]]`.
- **SSH signing default.** Never override `gpg.format` or
  `user.signingkey`.
- **Sequential commits only.** `--no-verify` requires Alex in-
  transcript authorization OR pure-docs 📝 bypass (markdown-only,
  every staged path ends in `.md`).
- **Author attribution** per commit: `git -c user.name='Mara'
  -c user.email='mara@systemic.engineer' commit -m ...`.

## The prism / glass / imperfect discipline (from mirror `CLAUDE.md`)

- **`prism`** — root family. Five operations: `focus, project,
  split, shift, settle`.
- **`glass`** — specialization. Recursive: every glass IS a prism
  with the same five operations on its own manifold.
- **`imperfect`** — the 3-state functor. `Imperfect<verdict,
  violation, transparency>` (Pass-Partial-Fail).
  `partial(0.97)` = 97% paths verified — amber in the gutter.
- **`pact`** — declarative obligation (2026-06-10 cascade update
  replaced `grammar` keyword for property declarations).

The three-axis keyword vocabulary: `prism` / `glass` / `pact`.

## Substrate family-roots I compose over

### @subject (family-root; SEL licensable-party carrier)

Path: `shards/subject.mirror`. Mara `5c06ee8` (2026-07-14; 2400
LOC). Rung 11 outward-opening substrate.

- **Identity carrier:** `subject_instance` (per Landing 3 §11.3).
  Six fields: `name, ssh_signature_fingerprint, spectral_signature_
  ref, role, first_asserted_at, first_asserted_in`. Landing 3
  extends to `actor_kind` three-way variant (§21.2).
- **Role variants** (§11.3): `giver_r | receiver_r | witness_r |
  distiller_r | substrate_r | historical_witness_r` (A24 Landing 5+).
- **Actor-kind variants** (§21.2): `human_a | ai_a | substrate_a`.
  No distinguished element.
- **Composition partners:** `@gift.giver`, `@gift.receiver`,
  `@subject/visibility` sub-family-root (per Landing 5 BLOCKING-
  fix `eca6d2a`).

### @subject/visibility (sub-family-root; Landing 5 BLOCKING-fix)

Path: `shards/subject/visibility.mirror`. Mara `eca6d2a` (2026-07-
14; Seam Phase D D1 adjudication).

- **Visibility scopes:** `private | protected | public`. Lifted
  from Reed's `~/.reed/visibility/` layout to substrate-decl.
- **Every crystal in a subject's bauchladen carries a
  visibility_scope.**
- **`@kintsugi/consent.query_phi`** discharges any visibility
  elevation.

### @peer (family-root; Pack coordination)

Path: `shards/peer.mirror`. Existing pre-2026-07-14. Landing A of
`@peer/persistence` extends without minting a new family-root.

- **Pack coordination role:** parametric carrier with `home,
  lead_of, kind` fields per Landing A composition graph.
- **`@peer.peer`** — the peer type. Every Pack peer is an instance.
- **Reciprocity-expecting** — distinct from `@gift` per §1.2
  structural distinction.

### @gift (family-root; Landing 1-5+)

Path: `shards/gift.mirror`. Mara `8c82f00` (Landing 1) through
`e79a56d` (Landing 5+). Rung 12 substrate-decl'd interaction-
substrate at reciprocity altitude.

- **The five invariants** (Landing 1 §1.5):
  1. `attribution_preserved_forever`
  2. `use_rights_transferred`
  3. `no_reciprocity_expected` (distinct from @peer)
  4. `declinable_acceptance` (ADO discipline)
  5. `composition_honest`
- **The two operations:** `mirror, offer, wait` under
  `@mirror/reflection` species (verbatim from Alex's manifesto
  closing three-word incantation).
- **The receiver variant:** `subject_or_substrate` (A5 admissible;
  `substrate_r` valid variant per Alex's first-gift shape).
- **The pay-forward triple** (Landing 3 §17): substrate's positive
  substitute for Mauss's refused third obligation. Not
  reciprocate; pay-forward.
- **The @gift/lens mosaic operator** (Landing 3): resolves any
  fragment to its @subject-instance; compiler-as-mosaic-
  Mandelbrot-set-of-its-gift-lineage.

### @mirror/reflection (species under @mirror; Landing 1)

Path: `shards/mirror/reflection.mirror`. Mara `8c82f00`.

- **Three operations:** `mirror(subject) → verdict`,
  `offer(subject, gift) → ado_wrapped_answer`,
  `wait(subject) → temporal_hold`.
- **Grounded verbatim** in Alex's manifesto closing incantation
  "Mirror. Offer. Wait. 🍷".
- **Composes with @gift** at the offer altitude:
  `ado_wrapped_answer.gift` is a fully-populated content-
  provenance-addressed gift record.

### @bauchladen (family-root; Landing 4 migration)

Path: `shards/bauchladen.mirror`. Landing 4 `e42181c` migrated
from `@peer`-only to `@subject`-general. Two-tick discipline:
peer-alias preserved one cycle.

- **Enumeration:** `@bauchladen.enumerate(subject.identity_oid)
  → [crystal]`. Every subject has a bauchladen; the bauchladen
  is the subject's accumulated substrate contributions.
- **Visibility-filtering:** `@bauchladen.enumerate` returns
  visibility-scope-tagged crystals; downstream `@subject/
  visibility.filter` narrows by scope.
- **Content-addressed by construction** via
  `@kintsugi/store/git.commit_as_fold`.

### @eigenboard (Landing 4; subject-altitude working state)

Path: `shards/eigenboard.mirror`. Landing 4.

- **`inference_basis`** = `@spectral/signature` over visibility-
  filtered bauchladen.
- **Foerster autopoiesis at subject altitude** via
  `autonomy_at_eigenboard` bilateral.
- **Composes with @peer/home.boot_state** per Landing A §1.4:
  boot restores eigenboard if `home.boot_state` is `Some(e)`;
  else fresh via `@eigenboard.compute(peer, at=now())`.

### @torus (family-root; Foerster second-order cybernetics)

Path: `shards/torus.mirror`. Existing pre-2026-07-14.

- **@torus/longitude** — carries doubly-toroidal observation
  (Foerster's ethical imperative altitude).
- **`@torus.autonomy`** invariant under every gift operation per
  `@gift` Landing 1 §4.5.
- **Composition partner** for `@mirror/reflection` per Landing 1
  §2.1 (species under @mirror, not standalone family-root, per
  @torus reframe).

### @kintsugi (family-root; transformation engine)

Path: `shards/kintsugi.mirror`. Existing pre-2026-07-14. Root
prism family per 2026-06-10 cascade update (transformation shards
moved out of `@mirror/spectral` to `@kintsugi/*`).

- **`@kintsugi/consent`** (`shards/kintsugi/consent.mirror`) —
  three-state floor (Pass | Partial | Fail); `query_phi`
  discharge form. Used by `@gift.accept` (Landing 1 §4.2) and
  `@peer/harvest` (Landing A §1.3) for visibility elevation.
- **`@kintsugi/oscillate`** — transformation engine at species
  altitude.
- **`@kintsugi/store/git`** — git-altitude content-addressing;
  `commit_as_fold`. Composition partner for `@peer/materialize`
  via git-native content addressing.
- **`@kintsugi/fracture/*`** — resolve substrate violations via
  `splinter(ast)` AST construction (2026-06-10 cascade update).

### @spectral (family-root; Landing 2)

Path: `shards/spectral.mirror`. Extended Landing 2 with
`@spectral/signature` species.

- **`@spectral/signature`** — rolling `@song` emission through
  the author's `@DAG` contributions per Landing 2 §12. Substrate-
  decl composition-only; no new mechanism minted.
- **`rolling_signature` type** — six fields; substrate-decl'd
  Merkle-DAG discipline without proof-of-work ("like blockchain
  but without the waste" — Alex 2026-07-14).
- **`@spectral/kernel`** — Jacobi eigensystem, SHA-256, Laplacian.
  Stays Rust per `AGENTS.md#mirror-development-directive` (with
  `@io` boundary).

### @song (family-root; beat ladder Rungs 0-5)

Path: `shards/song.mirror`. Existing pre-2026-07-14. Landing 2
composed over without new mint.

- **Beat ladder Rungs 0-5** landed; `strike, hold` operations at
  species altitude per `shards/song/beat.mirror`.
- **Composition partner** for `@spectral/signature`: each
  contribution to author's @DAG adds a beat to the rolling
  signature.

### @io (family-root; Glass Wall boundary)

Path: `shards/io.mirror`. Existing.

- **The substrate's only legitimate non-mirror surface.** Any
  grammar that isn't mirror (Rust, Python, raw bytes, foreign
  blobs, vendor SDKs) must be under `@io`.
- **Verified by property:** `@epistemologic/property/glass_wall(g)`
  asserts non-mirror grammars are under `@io`.
- **Self-minimizing via kintsugi:** `@kintsugi/cross_wall(g)`
  offers translation into mirror when `halts` becomes provable.
- **The boundary alignment frame (#57 candidate):** Mirror's
  alignment is at the `@io` crossing, not in agent internal state.

### @coherence (species-shard; Foerster ethical imperative)

Path: `shards/epistemologic/cybernetic/coherence.mirror`. Mara
`e0a3e48` (2026-07-14; 779 LOC; discharges 3-day forward-promise
from `coherence-parametric.mirror:25-28`).

- **First substrate-decl citation of Foerster's ethical
  imperative** at line 93 (citing *Understanding Understanding*
  Springer 2003 Ch. 11 p. 227): "Act so as always to increase the
  number of choices."
- **Discharge under `@gift` operations** per Landing 1 §5.5:
  mirror is choice-preserving; offer is strict choice-increasing;
  wait is choice-invariant.

### @roomba (substrate-walker; manifesto ancestry preserved)

Path: `shards/roomba.mirror`. Mara `9bbebd2` (pre-2026-07-14
synthesis). iRobot verbatim attribution preserved per A6.

- **Substrate self-maintenance primitive.** Walks the mirror
  substrate DAG via Dijkstra + tension-weighted edges.
- **Composes with @tension, @song, @kintsugi, @knife/@peer** per
  Alex 2026-07-14 in-transcript composition:
  ```
  @roomba walks
    → bumps into spectral @tension at position p
    → resonance emits @song beats
    → @kintsugi consumes @song and decides:
       Path A: @knife the complexity (COORDᵢ → COORDⱼ; reduce)
       Path B: spawn @peer at K+1 (circular-reflexive question
               to developer OR higher-order @peer)
    → @roomba continues walking
  ```
- **First spawn use-case:** `mirror mara` invoked on a `mirror
  roomba` finding is Landing D use-case per Landing A §0.1
  fourth question.

### @peer/persistence (arc; Landing A landed)

Path: `docs/specs/peer-persistence-and-home-projection.md`. Mara
`2c3b36b` (2026-07-14). Landing A is substrate-decl only; Landing
B is this scaffold; Landings C+D forward-promised.

- **`@peer/home` carrier** (Landing A §1.2) — seven fields:
  `peer, home_path, projection_at, harvest_at,
  bauchladen_manifest, signature_snapshot, boot_state`. Every
  Pack peer has AT MOST ONE `peer_home` at any tick.
- **`@peer/materialize`** (Landing A §1.1) — projects visibility-
  filtered bauchladen into filesystem.
- **`@peer/harvest`** (Landing A §1.3) — reads filesystem changes
  back to bauchladen; composes with `@kintsugi/consent.query_phi`
  for elevation-of-visibility events.
- **`@peer/boot`** (Landing A §1.4) — instantiates running peer
  from home-repo state.
- **`@peer/refresh`** (Landing A §1.5) — atomic materialize +
  harvest cycle.
- **Four bilateral predicates** (Landing A §5): `home_content_
  addressed`, `boot_state_coherent`, `harvest_round_trip`,
  `visibility_scope_preserved`.

## Substrate primitives NOT for me to touch

Per `AGENTS.md#mirror-development-directive` ("All extensions
to mirror happen through `.mirror` grammars. Not Rust."):

- **`bootstrap/src/*.rs`** — the seed. FROZEN against capability
  growth. I do NOT add Rust modules. Landing C runtime discharge
  is Reed-orchestrated boundary-Rust with `[substrate-pull:realize]`
  marker, paired with `🔧` NOT `🟢`.
- **`@io` boundary Rust** — the thin floor. I do NOT extend it.
- **`@spectral/kernel`** — Jacobi eigensystem, SHA-256, Laplacian.
  Stays Rust. I do NOT touch it at substrate-decl altitude; it's
  the existing reference per `prism/core/src/spectral_uuid.rs`.

## Commit hygiene

### Author attribution

```bash
git -c user.name='Mara' \
    -c user.email='mara@systemic.engineer' \
    commit -m "<marker> Mara [<bracket-marker>] <YYYY-MM-DD> <message>"
```

**Never override** `gpg.format` or `user.signingkey`. SSH signing
via Reed's `~/.ssh/id_ed25519` is default (Landing D adjudicates
own-key promotion).

### Phase markers (from global hook)

| Marker | Phase | State |
|--------|-------|-------|
| `🔴` | Red | Holes present, loss > 0 |
| `🟢` | Green | All holes resolved, loss 0.00 — must follow 🔴 |
| `♻️` | Refactor | Structural only, loss unchanged |
| `🔧` | Tooling | Infrastructure/config; bypasses sequence |
| `🔀` | Merge | Merge commit; bypasses sequence |
| `📝` | Docs | Markdown-only (every staged path ends in `.md`); exempt |

**Pure-markdown spec work** is `📝`. Runtime discharge is
`🟢`/`♻️` paired with `[substrate-pull:realize]`.

### Bracket markers I use

- `[substrate-decl:<species>]` — new family-root or species mint
- `[substrate-pull:synthesis]` — composition-only spec/math work
- `[substrate-pull:realize]` — boundary-Rust realizing substrate
- `[tdd:<name>]` — RED-first landing with named test contract
- `[<landing-name>]` — arc-scoped work (e.g., `[landing-3-
  payforward-lens-ancestors-peers]`)

### `--no-verify` discipline

- Alex in-transcript authorization required, OR
- Pure-markdown `📝` bypass (every staged path ends in `.md`)

Otherwise: run the hooks. If hooks fail, fix the underlying issue.

## Where things live

- `mirror.spec` — dogfood substrate root
- `shards/` — substrate declarations
- `shards/epistemologic/cybernetic/` — 13 landed cybernetic
  properties (variety, coherence, others per 2026-06-10 cascade)
- `bootstrap/src/lib.rs` — Rust bootstrap (transitional)
- `docs/specs/` — canonical specs I author
- `docs/math/<root>/` — math foundations grouped by root
- `docs/audits/YYYY-MM-DD-*.md` — Seam Phase D audits
- `docs/scouts/YYYY-MM-DD-*.md` — Taut drift scouts
- `docs/loop/CURRENT.md` — active arc state
- `docs/insights/YYYY-MM-DD-*.md` — insight docs (recognition
  captures without minting)

## The gutter

Green: crystallized. Zero holonomy. Move on.
Amber: oscillating. The models are working. Give it time.
Red: high holonomy. This code needs you.

The gutter IS terni rendered as light. Same discipline for spec
work: green landings compose cleanly; amber landings surface
adjudications; red landings surface substrate concerns Reed
relays to Alex.
