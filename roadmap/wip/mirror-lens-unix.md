# `@mirror/lens/unix` — read-only kernel-mediated projection (v0) → write + Lens primitive (v1)

*2026-06-15. Mara. Roadmap (substrate-pull plan; v0 unblocked NOW + v1 cut-together).*

**Status:** v0 ready to extract (fragmentation-fuse exists at
`vcs/git/src/fuse.rs`; ~1012 lines, commit `f1e1135`); v1 MANDATORY for
VM deployment per Alex.

**Internal task tag:** #314 (this work). Adjacent: #305 (BEAM-shape
`gen_prism` architecture; the per-home supervision-tree lifecycle this
lens rides on).

**Design lineage:**

- Materialize-to-tempdir floor landed 2026-06-11 (commit `3fb870a`;
  task #314 v0 baseline). `bootstrap/src/lens_unix.rs`'s `UnixLens`
  discharges the family root's abstract bodies through tempdir
  materialization — kernel-extension-free, idempotent, works on every
  dev machine including macOS without macFUSE + SIP-weakening.
- Family root declared 2026-06-11 (commit `5aa5777`) in
  `shards/mirror/lens/unix.mirror`; forward-promised `@mirror/lens/unix/fuse`
  and `@mirror/lens/unix/9p` sub-species named at the family-root
  altitude.
- Sister Mara's fragmentation FS-overlay survey (2026-06-15) identified
  `fragmentation/vcs/git/src/fuse.rs` (1012 lines; 26 `fuser::Filesystem`
  handler methods; inode table; transparent `Node::Lens` traversal;
  `ReadAnnotation` flow) as the load-bearing existing primitive.
  Extraction lifts the git coupling behind a `BackingStore` trait.
- Alex's design decisions 2026-06-15 (Reed + Alex close-out): (a)
  fragmentation-fuse workspace member; (b) read-only at v0; (c)
  per-home long-lived mount (BEAM philosophy); (d) defer the
  @mirror/store Lens primitive to v1, cut TOGETHER with write
  semantics.

## Why this matters — the ouroboros pin

Without `@mirror/lens/unix`:

- `cargo` (and `erlc`, and `flang`) can't read the content-addressed
  store — they expect path-addressed Unix files.
- the substrate can't consume itself — the
  `mirror -> @code/macro -> @mirror/store -> @mirror/lens/unix ->
  cargo -> binary -> @spectral runtime` pipeline (per
  `docs/specs/spectral-runtime.md` §5) stalls at the lens step.
- v1.0 cloud deployment is blocked — agent state has to persist
  through some kernel-mediated path, and the v0 materialize floor
  doesn't compose with the per-home root model the cloud needs.

The materialize-to-tempdir floor (v0 baseline) WORKS but is the wrong
long-term shape:

- copies content per-invocation (cargo invocations re-materialize);
- doesn't compose with per-home roots (one tempdir per cargo call, not
  one mount per home);
- doesn't survive cargo invocations (mount lifecycle is per-call, not
  long-lived);
- write semantics would have to round-trip through the materialize
  step (not viable for VM deployment).

The kernel-mediated FUSE-backed path fixes all four. Read-only at v0
is enough to close the ouroboros loop's read leg; v1 closes the write
leg + cross-shard composition together.

## v0 — read-only FUSE (next ticks)

**Scope:** read-only kernel-mediated projection. Cargo / erlc / flang
read from a per-home FUSE mount; the substrate writes still flow
through `@mirror/store/write` at the store altitude. v1.0 cloud
closure unblocked.

### v0.1 — fragmentation-fuse workspace member

Extract `fragmentation/vcs/git/src/fuse.rs` into a
`fragmentation/vcs/fuse/` workspace member. Abstract the git coupling
behind a `BackingStore` trait so the same fuser machinery serves any
content-addressed store:

```rust
pub trait BackingStore {
    fn read_blob(&self, oid: Oid) -> Result<Vec<u8>>;
    fn read_tree(&self, oid: Oid) -> Result<TreeView>;
    // write surface — declared at v0.1 for trait completeness but
    // mirror's SplinterBackingStore stubs these at v0; v1 lights them up:
    fn write_blob(&mut self, data: &[u8]) -> Result<Oid>;
    fn write_tree(&mut self, entries: &[TreeEntry]) -> Result<Oid>;
    fn commit(&mut self, message: &str) -> Result<Oid>;
}
```

Stays:

- the 26 `fuser::Filesystem` handler methods (lookup, getattr,
  readdir, read, etc.);
- the inode table + path-to-oid resolution;
- the transparent `Node::Lens` traversal (cross-shard references at
  the git altitude — the precedent for v1's @mirror/store Lens
  primitive);
- the `ReadAnnotation` flow.

`GitBackingStore` (the existing implementation) lives in
`fragmentation/vcs/git/`. The extraction is a structural lift, not a
behaviour change at the git altitude.

### v0.2 — mirror `SplinterBackingStore` + bootstrap integration

- `SplinterBackingStore` impl in mirror, wrapping `@mirror/store`'s
  `Splinter<H>` substrate. Implements `BackingStore::read_blob` /
  `read_tree`; `write_*` and `commit` stub to `partial(opacity)` /
  `Err(NotImplemented)` (v1 lights them up — see v1.1 below).
- `bootstrap/src/lens_unix.rs` swaps from materialize-to-tempdir to
  FUSE-backed via `SplinterBackingStore`. The abstract action surface
  declared in `shards/mirror/lens/unix.mirror` doesn't change; the
  body discharge moves from `UnixLens::mount` (tempdir copy) to
  `FuseUnixLens::mount` (fuser session spawn).
- Materialize-to-tempdir is kept as a fallback sub-species at
  `@mirror/lens/unix/materialize`; the new FUSE-backed path is
  `@mirror/lens/unix/fuse` (declared this tick at
  `shards/mirror/lens/unix/fuse.mirror`). The substrate sub-species
  carry the platform discriminator; downstream consumers pick by
  capability.

### v0.3 — per-home long-lived mount lifecycle

- One FUSE mount per home (`~/.mirror`, `~/.reed`, `~/.mara`).
- Supervision tree at each home spawns the mount on first need (per
  `docs/specs/spectral-runtime.md` §3 gen_prism homes; per #305
  BEAM-shape architecture).
- Mount survives cargo / erlc / flang invocations (BEAM-philosophy
  long-lived processes, persistent state).
- Drop semantics tied to home shutdown, not handle shutdown — the
  supervision tree owns the mount's lifecycle.
- macFUSE friction wall stays a documented dev-machine prerequisite
  per `shards/mirror/lens/unix.mirror`'s body-status note (the family
  root's existing v0-floor reasoning carries over; this sub-species
  doesn't dodge the wall, it accepts it).

**v0 scope summary:** read-only; per-home long-lived; ouroboros loop
closes; v1.0 cloud closure unblocked. Estimated ~750 LOC delta
(extraction + SplinterBackingStore + bootstrap swap, per Mara's
survey).

## v1 — write semantics + `@mirror/store` Lens primitive (MANDATORY for VM)

**Scope:** the v0 fuse sub-species's read-only floor lit up with
write, plus the cross-shard reference primitive at the Splinter level
that makes write through references meaningful. Lands TOGETHER as one
ticket. Neither alone is a complete feature.

### v1.1 — write semantics via fragmentation-fuse

- `BackingStore::write_blob` + `write_tree` + `commit` consumed by
  mirror's `SplinterBackingStore` (these stubs from v0.1 light up).
- Snapshot-on-flush — every write yields a new shard OID; HEAD
  lineage preserved at the store altitude.
- The family root's `write(p, content) -> oid` action body (currently
  forward-promised in `shards/mirror/lens/unix.mirror`'s action body
  mapping) discharges through this path for the fuse sub-species.
- Conflict resolution at the supervision tree altitude — per-home
  serializes write order; cross-home conflicts surface as
  `imperfect` opacities for Reflection to consume per
  `error-as-question.md` §5.3.

### v1.2 — `@mirror/store` Lens primitive

- Cross-shard reference primitive at the Splinter / @mirror/store
  altitude. Today fragmentation's `Node::Lens` does this AT THE GIT
  ALTITUDE (the `vcs/git/src/fuse.rs` machinery transparently traverses
  cross-repository references); mirror's store doesn't have an
  equivalent at the Splinter level.
- Shape (designed during v1): a Splinter type variant carrying an
  (oid, lens-path) tuple, or a `lens_ref` sibling to `shard_ref` in
  `shards/mirror/store.mirror` — the exact shape lands in the v1.2
  tick after the v0 sub-species stabilises.
- Required so the unix lens can expose `Node::Lens`-style transparent
  cross-shard references via the FUSE surface — without it, write
  semantics can write blobs and trees but can't compose across
  shards.

**Why these cut together:** v1.1 without v1.2 means the substrate can
write but can't compose cross-shard. v1.2 without v1.1 means the
substrate has cross-shard refs but they're read-only. Either alone
ships half the feature. The Lens primitive needs write to be
meaningful; write needs the Lens primitive to compose. They land as
one ticket.

**v1 deployment dependency:** VM deployment of mirror runtime (the
`spectral.engineer` cloud target per `roadmap/wip/cloud-deployment.md`
§A) requires write support — agent state has to persist through the
lens. v1 is MANDATORY for v1.0 cloud deployment; v0 is the read leg,
v1 is the write+composition leg, and both legs land before
spectral.engineer's first production cycle.

## Sequencing

1. **v0.1** — fragmentation-fuse extract (`BackingStore` trait +
   workspace crate scaffold).
2. **v0.2** — mirror `SplinterBackingStore` + bootstrap swap.
3. **v0.3** — per-home long-lived mount lifecycle.
4. **v1.1 + v1.2** — together, after v0 lands and stabilises.

v0 is unblocked NOW (substrate decl landed this tick at
`shards/mirror/lens/unix/fuse.mirror`; the Rust extraction is the
next implementation tick). v1 sequencing depends on v0 stability +
concrete cloud deployment timeline.

No time estimates — one tick after the other, per
`feedback-no-time-estimates`.

## Cross-references

- **Substrate decl (this tick):**
  `shards/mirror/lens/unix/fuse.mirror`
- **Family root:** `shards/mirror/lens/unix.mirror` (commit
  `5aa5777`; the abstract impedance surface — mount / path / read /
  write / stat — this sub-species refines)
- **v0 floor (the materialize path being superseded):**
  `bootstrap/src/lens_unix.rs` (commit `3fb870a`)
- **Load-bearing primitive being extracted:**
  `fragmentation/vcs/git/src/fuse.rs` (commit `f1e1135`, ~1012 lines)
- **Workspace layout target:** `docs/specs/mirror-native-vcs.md` §2.2
- **Runtime pipeline this lens closes:**
  `docs/specs/spectral-runtime.md` §3 (gen_prism homes; per-home
  roots), §5 (the ouroboros pipeline)
- **Security boundary:** `docs/specs/threat-model-v0.md` §3 A4 — per-home
  FUSE mounts are the spawn-isolation surface; sign-verify happens at
  spawn altitude.
- **Adjacent track:** `roadmap/wip/spectral-db-substrate.md` (the open
  interface; @mirror/store stays open and this lens consumes it
  through the open trait).
- **Cloud closure:** `roadmap/wip/cloud-deployment.md` §A (peer runtime
  + persistence; v1 mandatory for this).
- **Architecture memory this doc updates:**
  `architecture-three-tier-stack` (the lens layer at the
  fragmentation-mcp ↔ mirror ↔ @spectral/db altitude — per-home FUSE
  mount is the concrete shape the supervision tree owns).

## Open questions (post-design-decisions)

- **macFUSE friction wall.** Already accepted by fragmentation's
  existing FUSE path (`vcs/git/src/fuse.rs` ships); documented
  dev-machine prerequisite (kernel extension + SIP weakening +
  reboot). The v0 fuse sub-species inherits the prerequisite from the
  underlying fuser binding; the materialize sub-species remains the
  kernel-free fallback for environments where macFUSE isn't
  available (CI without privileged setup, isolated build sandboxes).
- **Lens primitive shape at @mirror/store altitude.** Splinter type
  variant? Sibling `lens_ref` type next to `shard_ref` in
  `shards/mirror/store.mirror`? (oid, lens-path) tuple? Designed
  during v1.2 after v0 sub-species stabilises and the cross-shard
  reference call sites are concrete.
- **v1.0 cloud deployment timeline.** When does the v1 cut become
  time-critical? Tracked at `roadmap/wip/cloud-deployment.md`; this
  doc surfaces the dependency but does not pin the schedule.
- **Erlang + Fortran consumer convergence.** v0's
  `SplinterBackingStore` serves cargo; erlc and flang ride the same
  FUSE mount through the same backing store. Per
  `shards/mirror/lens/unix.mirror`'s body-status note: target-agnostic
  lens. The convergence test (cargo / erlc / flang all read the same
  mounted oid for the same path) is forward-promised to a v0.3
  cross-target test once the mount lifecycle stabilises.
