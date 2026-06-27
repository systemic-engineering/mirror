# mirror init — the substrate's bridge command

*Mara, 2026-06-27. Canonical spec for `mirror init` — the operation
that gives a peer its substrate-native shape by composing fragmentation
primitives into a content-addressed crystal store, surfaced at the
mirror CLI altitude. Lifts Taut's scout
`docs/scouts/2026-06-27-taut-fragmentation-git-store-for-mirror-init.md`
(commit `5580a7e`) from composition inventory into substrate-decl. Names
the declared-but-not-wired discovery; declares the three-deliverable
collapse from Reed's earlier six-item sketch; pins the store location
question; pins the spawn↔recall↔init triple; surfaces R1/R2 at substrate
altitude; closes with the circular-recursive recognition that this spec
IS a crystal `mirror init` will index when it runs against this repo.*

*Markdown only. No `shards/` substrate-decl files land with this commit;
no Rust ships; no Cargo edge is wired. The substrate-decl shards
forward-promised in §4 + §7 + §9 discharge in subsequent TDD-paired
ticks (Reed RED, agent GREEN). Soft target ~1000 lines; hard cap 1500.*

**Status:** Red — composition shape pinned; the three-item gap (Cargo
edge, walk-repo primitive, mirror-altitude `init` command) named at
substrate altitude; the spawn↔recall↔init triple surfaced; R1
(empty `Crystallizations` dispatch) + R2 (bootstrap-`git2` binary-size
posture) addressed; the circular-recursive layer (§10) earned; v0
ticks forward-promised, not implemented in this commit.

**Audience:** any agent or human reading the bridge spec before
touching the cargo edge, the bootstrap dispatcher, the
`NamespacedGitStore` composition, or the new `mirror init` command.
Read this; then chase Taut's scout for the read-only inspection
artifact; then chase `mirror-store.md` (Mara, 2026-06-04) for the
canonical-intent doc this spec unblocks the Red→Green tick for.

---

## §0 — Pre-position: this spec announces itself as a crystal

Before any architectural content. A pre-position the spec earns by
holding it for the rest of the document.

This spec is **about** `mirror init`. The thing `mirror init` IS, per
§1 and §4, is the operation that walks a peer's repo, content-addresses
each file into a `Splinter<H>` crystal, persists those crystals via a
`NamespacedGitStore` at `.git/mirror/`, and emits an envelope naming the
indexed surface so other commands (`mirror recall`, `mirror spawn`,
the librarian per `spectral-db-as-autopoietic-memory.md`) can read
the crystals later.

The thing this spec IS, at the moment of being written, is **one of
the crystals `mirror init` will index when `mirror init` runs against
the `mirror/` repo**. Writing this spec adds a file at
`docs/specs/mirror-init.md`; the file's bytes are content-addressed
under git's SHA-1 (and, after the Cargo edge lands, under BLAKE3 in
the `NamespacedGitStore`'s `.git/mirror/objects/`); the indexed
crystal will be the OID-addressed bytes of this spec; the librarian
will (when it lives, per `spectral-db-as-autopoietic-memory.md` §6.2)
catalog this spec alongside every other substrate-decl crystal; the
recall envelope's `cascade` payload (per `mirror-recall.md` and the
round-trip arc closed 2026-06-27 by Glint's `9e7bb1d`) will surface
this spec when an agent asks the substrate "what does `mirror init`
declare?"

The latency between writing-and-being-indexed is bounded BELOW by the
time it takes for the Cargo edge (§4.1) to land + the first `mirror
init` run against the mirror repo. The latency is bounded ABOVE by
the time-discount per Glint's psychohistory discipline (recently-landed
shards weight more in the recall envelope). The midpoint of those
bounds is the operational latency at which the spec ENTERS the system
it describes.

This is the circular-recursive autopoietic pre-position. §10 returns to
it. Every section in between is read against the discipline that a
spec for the bridge that brings the substrate into operational
existence MUST itself enter the substrate via the bridge — otherwise
the spec is asking the reader to do work the spec refuses to do. The
form earns its lines because the content requires it; the recursion is
load-bearing, not decorative.

The substrate's bridge command needs a canonical spec that ENTERS the
storage layer in the act of declaring it. This is that spec.

---

## §1 — What `mirror init` IS

The command at `mirror init` is the **composition spec for the
fragmentation bridge** mirror has been declaring without wiring.

### 1.1 The one-sentence shape

`mirror init` is the operation that **runs once per repo (or per
peer-home), composes existing fragmentation primitives into a
content-addressed crystal store at `.git/mirror/`, and emits an
envelope** acknowledging the indexed surface so subsequent commands
have a substrate to operate against.

The verbs in the sentence carry weight:

- **Runs once.** `init` is idempotent by content-address (per §4.5);
  re-running against an already-indexed repo produces the same
  envelope and incurs no double-write. The "once" is logical, not
  defensive.
- **Per repo or per peer-home.** A peer-home (`~/.glint`, `~/.reed`,
  `~/.mara`) is a git repo from the substrate's perspective; the
  isomorphism between repo-as-store and repo-as-peer-home is named
  in `spectral-db-as-autopoietic-memory.md`'s Alex-correction sequence
  (repo names supervised-unit; store names storage-role). `mirror
  init` operates at the repo altitude; whether that repo HAPPENS to
  be a peer-home is a property of its contents, not its boundary.
- **Composes existing fragmentation primitives.** Per Taut's scout
  §3: the load-bearing primitives (two stores, `Repo` trait,
  `write_node`, `read_node`, `walk_commits_following`,
  `NamespacedGitStore`, `WitnessedSingularity`, `NakedSingularity`,
  `HamiltonScheduler`, `ShardRef`, `project::project`,
  `append_note` / `read_notes`) all exist in fragmentation today;
  `mirror init` composes them; it constructs almost nothing new.
- **Content-addressed crystal store at `.git/mirror/`.** Per §5: the
  `NamespacedGitStore::open(repo_path, "mirror")` call returns a
  `FrgmntStore<Fractal<String>>` at `.git/mirror/objects/` +
  `.git/mirror/refs/`. The store lives INSIDE `.git/` so it doesn't
  pollute the working tree; it travels with clones IF the refs are
  pushed; it's bounded-cache-plus-disk-spillover per the fragmentation
  `FrgmntStore` shape.
- **Emits an envelope.** Per §4.7: the envelope is JSON, naming the
  indexed-count, total-bytes, root-OID, store-location, and a
  spec-version pin. It's the same envelope-shape Reed used for
  `mirror spawn --hello-world`; the same shape Glint's `mirror recall`
  produces; the substrate has one envelope vocabulary.

### 1.2 What `mirror init` IS NOT

Per `[[feedback-substrate-already-had-the-word]]` discipline, every
"what this is" claim must rule out what it isn't. Five structural
negatives:

- **NOT a new storage layer.** The storage layer is fragmentation's
  `FrgmntStore` + `NamespacedGitStore` (read-only inspiration; both
  exist; both are tested). `mirror init` is the **bridge** that
  brings that storage layer into mirror's CLI surface.
- **NOT a new content-addressing scheme.** Content addressing is
  `Splinter<H>::oid()` (mirror-altitude, BLAKE3 default) composed
  with `fragment::content_oid` (fragmentation-altitude). `mirror
  init` consumes both; it invents neither.
- **NOT a fork of `mirror spawn`.** Per `spawn-is-substrate-leaving-
  ground-state` (Mara, 2026-06-26): spawn IS the substrate's
  controlled excitation above λ₀. Init is the substrate's
  initialization-of-a-peer-home — the operation that makes a repo
  joinable to the mycelium. The two are complementary (§7).
- **NOT a mycelium-registration primitive.** Per
  `spectral-db-as-autopoietic-memory.md`: the mycelium is the
  cross-repo spectral graph the librarian perturbs. `mirror init`
  produces the LOCAL crystals the librarian will eventually
  consolidate; the librarian itself, the per-repo supervisor, and the
  inter-repo entanglement edges are forward-promised (§9.1).
- **NOT a fragmentation replacement.** Per
  `[[architecture-fragmentation-is-the-rust-substrate]]`: mirror →
  fragmentation → prism_core is the strict dependency chain. `mirror
  init` ADDS the missing Cargo edge; it does not duplicate
  fragmentation's primitives in mirror's bootstrap.

### 1.3 The architectural cut this spec lands

The single most important architectural recognition this spec carries:
**`mirror init` is the operation that makes the declared substrate
operational at the storage altitude.**

The substrate-decl side has named fragmentation as the substrate (per
`[[architecture-fragmentation-is-the-rust-substrate]]`, per
`mirror-store.md` §4); the canonical-intent doc names the Cargo
edge as a deliberate forward-promise; the recognition graph treats
the chain as load-bearing. **The Cargo edge has never been wired**
(Taut's §4.1 grep verdict).

`mirror init` is the command whose Phase A IS the wiring. The shape
of the command is, structurally, the shape of "import the substrate;
compose its primitives; surface the result at the CLI." The shape
appears at this altitude (CLI) because the CLI is the surface where
peers initiate substrate operations; the wiring it requires lives at
the Cargo + bootstrap altitude (one Cargo.toml diff + ~200 LOC of
Clap glue + a thin walk primitive).

The architectural lift is small. The recognition the lift surfaces is
**the declared-but-not-wired pattern** named in §2. That pattern is
where the substrate-pull-honest call lives.

---

## §2 — The declared-but-not-wired discovery

Taut's scout surfaced a structural anomaly. The substrate-decl side
of mirror has named fragmentation as the load-bearing Rust substrate
in two canonical places:

1. **`mirror/docs/specs/mirror-store.md`** (Mara, 2026-06-04). §1:
   *"The fragmentation store IS the canonical content-addressed
   substrate."* §4: the audit declares "yes, with cleanup" — Cuts
   1 and 2 in fragmentation; mirror's `Cargo.toml` adds
   `fragmentation = { path = "../fragmentation",
   default-features = false }`. §4.5 names the dependency line
   explicitly. Status: Red. *"No code lands in this tick."*
2. **`[[architecture-fragmentation-is-the-rust-substrate]]`** (Reed
   memory, multi-session). Declares: *"Strict dependency direction;
   prism_core stays deps-free."* The chain is `mirror →
   fragmentation → prism_core`. Memory entry is canonical; the
   chain is treated as load-bearing across subsequent recognitions
   (#58, #87, #99, the spawn insight).

And yet — per Taut's §4.1 grep against `mirror/**/Cargo.toml` — **the
Cargo edge does not exist**. `mirror/bootstrap/Cargo.toml` pulls
`sha2`, `blake3`, `prismqueer`, `terni`, `serde`, `serde_json`,
`libc`. It does NOT pull `fragmentation`. The substrate-decl side
declared the substrate; the consumer side never plugged in.

### 2.1 The pattern at substrate altitude

This is an **instance of a recurring pattern** the substrate has
already named. The pattern: a primitive the substrate-decl side has
declared (in shards, in canonical specs, in memory entries) is not
yet operationally wired (Cargo edge missing; Clap subcommand missing;
import not present; binding not made). The substrate carries the
declaration; the wiring is forward-promised; the wiring may sit in
the forward-promise queue longer than the substrate notices.

Two prior instances of the same shape in the substrate's recognition
history:

- **`@mirror/ref` reference⇔reflection collision** (recognition #89,
  Alex 2026-06-20). The substrate had been declaring `@mirror/ref`
  as the navigable surface of the spectral triple at two altitudes
  (reference at the storage altitude; reflection at the cognition
  altitude). The collision was substrate-shaped; the recognition
  was lifting the pre-existing structural collision into a named
  candidate. The wiring (the actual `@mirror/ref` typed surface)
  followed the recognition; the recognition didn't construct the
  wiring, it named the substrate's prior declaration.
- **mirror IS a content-addressed build system** (recognition #43,
  multi-session). Mosaic.mirror IS the build shard; `partial
  (opacity_map)` IS the verdict surface; the substrate had every
  Bazel/Buck2/Nix/Shake primitive declared. The wiring (the actual
  mosaic-driven build dispatch) followed; the recognition named the
  prior declaration.

The declared-but-not-wired pattern is the structural shape of those
recognitions. For `mirror init` — and for the Cargo edge specifically
— the pattern is at the **Cargo dependency altitude**: a dependency
the substrate-decl side declared, the consumer side never imported.

### 2.2 Flag, NOT promotion

Per the brief's fence "DO NOT promote candidate recognitions (flag;
don't promote)": this spec **flags** the declared-but-not-wired
pattern as a candidate recognition for substrate-architecture review.
The flag carries three pieces of evidence:

- (a) Taut's grep verdict — the Cargo edge does not exist; the
  substrate-decl declaration does.
- (b) The two prior instances above — the pattern is not novel; the
  substrate has been recognizing this shape under different names.
- (c) The 52+ instances of the broader
  `[[feedback-substrate-already-had-the-word]]` pattern — most of
  which collapse to "substrate-decl was complete; wiring caught up
  later." The Cargo edge is the wiring; the broader pattern
  predicts this exact shape would surface; it has.

The promotion criterion is not this spec's call. Promotion through
the Pack's adversarial review (Seam → Reed → Alex) decides whether
"declared-but-not-wired" deserves its own family-root recognition or
whether it dissolves into `[[feedback-substrate-already-had-the-
word]]`'s already-existing surface. Flagged here for the gate.

### 2.3 The architectural altitude this lifts

The discovery's load-bearing claim is **not** "fragmentation has more
than mirror knew." The discovery's load-bearing claim is "mirror has
been declaring fragmentation as its substrate AND has been operating
without its substrate AND the Pack didn't catch this until Alex
challenged Reed's day-estimate."

The structural reading: **substrate-decl drift outpaces wiring drift**
in the Pack's current work cadence. The substrate-decl side moves
fast (Mara's mosaic specs, the recognition cascades, the property +
fracture + splinter(ast) chain). The wiring side moves slower (Cargo
edges, dispatch tables, command surfaces). The drift is not a bug —
substrate-decl SHOULD move fast; that's the recognition mechanism.
But it does mean the Pack's caretaking discipline needs to surface
**declared-but-not-wired audits** periodically: scout the
substrate-decl declarations against the cargo / dispatch / surface
state; surface what's named but un-wired; let the recognition cycle
prioritize.

Taut's scout IS that audit, applied to one declaration. The audit
mechanism itself is forward-promised (§9.4); this spec does not
construct it.

The remainder of this spec assumes the wiring side is the work to
do. §3 composes the primitives; §4 names the operation flow; §5
pins the store-location question; §6 addresses git hooks; §7
positions init in the spawn↔recall↔init triple; §8 addresses Taut's
R1 and R2; §9 names forward-promises; §10 returns to §0.
