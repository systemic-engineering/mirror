# `@spectral/garden` deployment runtime Rung 5 — mycelial-envelope-declared substrate via `bootstrap/src/deploy.rs`

*Mara, 2026-07-13 arc-continuation spec. Substrate-decl adjudication of
Reed's Rung 5 delivery ambiguity per Taut `c54740c` §5.6: which shape of
mycelial nix deployment IS substrate-honest for Rung 5, given that
`spectral.engineer` is a v1.0-roadmap deployment target the substrate
has NOT yet operationalized, `shards/spectral/garden/nix.mirror` is
forward-promised (not landed), and `flake.nix` at repo root is a
dev-environment flake (rust + flang + LAPACK) not a deployment flake?*

**Author:** Mara
**Date:** 2026-07-13
**Tag:** 📝 substrate-pull:realize; ladder-rung-5-spec
**Status:** canonical adjudication of scope + canonical shape for
`bootstrap/src/deploy.rs` + test-infrastructure spec + envelope contract
+ substrate-decl adjacencies. Every substrate claim cited with OID or
grep-verified file:line.

---

## §0. Executive summary

**Verdict: Scope A — mycelial-envelope-declared substrate — with a
specific narrowing the substrate has already reserved: the Rung 5
runtime NAMES the deployment substrate authorities (`@spectral/garden`
+ `@spectral/garden/nix` + `@bauchladen` + `@dance`) as an envelope
composition, WITHOUT invoking `nix build` subprocess, WITHOUT contacting
`spectral.engineer`, WITHOUT actual `@bauchladen` crystal exchange.**

Reed's three-scope framing (A = declared-only envelope; B = local
`nix build` subprocess with `file://` target; C = full push to
`spectral.engineer` with real cache exchange) is well-framed but
**Scope B AND Scope C are both blocked at the substrate-decl altitude**
by material Mara has NOT yet authored: `shards/spectral/garden/nix.mirror`
does not exist (forward-promised at
`docs/specs/spectral-garden-git-package-manager.md` §6.2 + §6.4). Reed's
runtime cannot in good faith invoke `nix build` on the mirror.spec's
garden{} block when the substrate-decl species that would name what
"nix build against a garden entry" MEANS at substrate altitude has not
landed. Scope B is not narrower-than-C; it is same-substrate-gap-as-C.

Scope A is the substrate-honest Rung 5 that composes over ONLY landed
carriers. It extends the Rung 4 `bootstrap/src/dance.rs` stub-pattern
one altitude up (deployment) with the SAME discipline: envelope naming
preserves substrate binding at deployment altitude; actual `nix build`
subprocess + `@bauchladen` crystal exchange + spectral.engineer HTTP
contact are Rung 5.5 / Rung 6 forward-promised WITH explicit
substrate-decl prerequisites listed.

**One-tick landability for Reed.** With this scope adjudicated,
Rung 5 lands in one Tick 5b: `bootstrap/src/deploy.rs` (~120-160 lines),
`bootstrap/tests/peer_beam_deploy_mycelial_shard.rs` (five T-tests),
and one `--deploy-to <target>` flag on the existing `mirror peer beam`
cli surface. All infrastructure Reed has landed (peer_beam runtime,
--song dispatch, --dance-with dispatch, dance stub pattern) is reused
verbatim; the deploy stub composes over dance's shared-root-OID hash
by reading it as a common-prior anchor.

**Substrate-already-had-the-word coverage for Rung 5 (Scope A):
~85%.** The four deployment authorities (`@spectral/garden`,
`@spectral/garden/nix`, `@bauchladen`, `@dance`) are named at spec
altitude but three of the four have UNLANDED species shards
(garden.git species-decl is landed at `docs/specs/spectral-garden-git-
package-manager.md` but `shards/spectral/garden/git.mirror` species-decl
file does not exist; `shards/spectral/garden/nix.mirror` does not exist;
the parent `@spectral/garden` family-root species-decl does not exist).
The envelope-only stub can name them as authorities WITHOUT depending
on their operational discharge — Rung 4 established this precedent
(`@dance` cited as authority in the envelope while dance.rs runs a
FNV-1a stub; the actual Kuramoto ODE / Aumann-agreement machinery is
forward-promised).

**Recognition candidate to promote at Rung 5 landing:**
`#R-envelope-declared-substrate-preserves-binding-at-deployment-altitude`
— extending Rung 4's `#R-multi-peer-coherence-phase-lock-realizes-dance-
at-runtime-altitude` from ensemble-coordination-scale to
mycelial-deployment-scale under the SAME discipline: envelope-declared
substrate WITHOUT operational discharge is substrate-honest when the
declaring is the paradigm-shift claim (naming the coordinating structure)
and the discharging is engineering work that composes downstream.

**Refusals.** Scope B refused because `shards/spectral/garden/nix.mirror`
species-decl has not landed (§2.2); Scope C refused because
spectral.engineer is v1.0-roadmap and the operational infra (cache
endpoint, deployment SSH keys, mycelial propagation protocol) is
undeclared substrate (§2.3 + §5). The remaining forward-promises
(actual `nix build` subprocess, actual `@bauchladen` crystal exchange
over network, actual spectral.engineer HTTP contact) are explicitly
named at §8 as Rung 5.5 / Rung 6 scope with substrate-decl
prerequisites.

**Ambiguity that CANNOT be postponed.** Whether the Rung 5 stub
envelope names four authorities (`@spectral/garden` + `@spectral/garden/
nix` + `@bauchladen` + `@dance`) or five (adding `@mirror/mosaic` as
the compilation-boundary authority). The composed `@song` at Mara
`d21337b` §5.1 names `@mirror/mosaic` as `mirror_compiler` voice —
adjudicated at §6(e) below in favor of FIVE authorities (mosaic is
substrate-load-bearing at compile altitude even if the stub does not
invoke it operationally).

---

## §1. Substrate-already-had-the-word audit for Rung 5

Grep-first per `[[feedback-substrate-already-had-the-word]]` (~74th
instance this arc, up from Rung 4's ~72nd). Every claim below is
grep-verified with the file:line where the substrate landed the carrier.

### 1.1 The Rung 5 module path IS pre-declared at scout altitude

`docs/scouts/2026-07-13-taut-mirror-spawn-song-beat-gap-scout.md` §4
(Taut `c54740c`, LANDED 2026-07-13) — R6 gap:

> mycelial propagation via nix binary cache (`@bauchladen` gossip).
> ~500 lines; probably splits into two ticks.

And §5.5 Rung 5 test-assertion:

> `bootstrap/tests/garden_deployment_song_shard.rs` (~300 lines):
> Fixture: minimal garden-song (5 beats: mosaic-compile, nix-build,
> bauchladen-publish, mycelial-gossip, verify-coherence). Multi-peer
> fixture (2-3 peers).

Taut's scout named `bootstrap/tests/garden_deployment_song_shard.rs`
and the five-beat mycelial-song shape (mosaic-compile / nix-build /
bauchladen-publish / mycelial-gossip / verify-coherence) 24 hours
before this spec's authoring. The scout ALSO reserved the module
altitude ("HIGH risk — first runtime tick to invoke real nix
subprocess") — the scout knew Rung 5 would require substrate that has
not yet landed.

**This spec adjudicates the scout's own hedge.** Taut named the
substrate gap ("first runtime tick to invoke real nix subprocess") as
Scope-B/C blocking; Mara reads the substrate as saying this gap is not
yet ready to close (see §2.2). Scope A honors both the scout's
ambition and the substrate's readiness.

### 1.2 The four (or five) deployment authorities are landed at spec altitude

Grep-verified. Each authority named at composed-@song §5.1 (Mara
`d21337b`) with landed spec-altitude witness:

| Authority | Landed at | Species-decl file exists? |
|-----------|-----------|--------------------------|
| `@spectral/garden` | `docs/specs/spectral-garden-git-package-manager.md` (`ad03fda`) | NO (family-root forward-promised §6.4) |
| `@spectral/garden/nix` | Same spec §6.2 forward-promised | NO |
| `@spectral/garden/git` | Same spec (canonical) | NO (species-decl forward-promised) |
| `@bauchladen` | `shards/bauchladen.mirror` (`4575340`) + spec `docs/specs/bauchladen-autopoietic-fate.md` | **YES** — landed 2026-06-29 |
| `@dance` | `docs/specs/dance-as-coordination-without-signal-on-forster-torus.md` (`4f079c8`) + `bootstrap/src/dance.rs` (`dfac8fe`) | YES — spec-landed, Path C annotation on `shards/algebra/metalogue.mirror:348-374` (`61b444a`), Rung 4 runtime GREEN |
| `@mirror/mosaic` | `shards/mirror/mosaic.mirror` (`fa8b4c8` per grep) | **YES** — landed 2026-06-09 |

**Substrate-decl gap surface for Rung 5:** three of the six deployment
authorities have no species-decl shard file yet. This is not a bug —
per `docs/specs/spectral-garden-git-package-manager.md` §6.4 the parent
family-root `@spectral/garden` is explicitly forward-promised, and the
per-species shards (`shards/spectral/garden/git.mirror`,
`shards/spectral/garden/nix.mirror`, `shards/spectral/garden/oci.mirror`)
are named as forward-promised siblings at §6.2.

**Rung 5 envelope-declared substrate authorities Reed can name:** all
six above, INCLUDING the three forward-promised. Rung 4 established
the precedent: envelope authorities do NOT require operational
discharge to be named as substrate authorities. What they DO require
is that the envelope's naming preserve the binding-when-they-land.

### 1.3 The compilation-boundary authority IS landed

`shards/mirror/mosaic.mirror` (2026-06-09 18:17, 9.1KB, LANDED). The
build-system prism `@mirror/mosaic` at compile altitude with focus /
project / split / shift / settle on the mosaic surface. The Rung 5
envelope MUST name `@mirror/mosaic` as the compilation-boundary
authority — this is the substrate that WOULD compile the mirror.spec's
garden{} block into candidate nix derivations at Rung 5.5 / Rung 6
operational-discharge altitude. Naming mosaic in the Rung 5 envelope
declares the binding-point where compilation would enter.

### 1.4 The @dance composition point IS the Rung 4 delivery

`bootstrap/src/dance.rs` (Reed `dfac8fe`, 7.8KB, LANDED). The
`execute_dance` function emits the shared_root_oid stub via FNV-1a
hash of the shared song bytes. The Rung 5 envelope COMPOSES OVER this:
the deployment envelope names the shared_root_oid the dance produces
as the deployment's "current_root_OID" (per Mara `d21337b` §5
narrative movement's completes_when clause: "Aumann agreement on
current_root_OID across ensemble").

**The Rung 4 → Rung 5 composition is substrate-honest:** deployment IS
what happens AFTER the ensemble has agreed on a shared root. Rung 4
established the agreement; Rung 5 names the deployment shape THAT
agreement enables. Byte-equality preserved for the Rung 4-only path
via the same `if let (Some, Some)` narrowing pattern.

### 1.5 The peer_beam envelope-composition pattern IS landed

`bootstrap/src/lib.rs:5019` (Reed `dfac8fe`, LANDED) — the Rung 4
dispatch pattern:

```rust
if let (Some(song_path), Some(peer_home_2)) = (song, dance_with) {
    return crate::dance::execute_dance(...);
}
if let Some(song_path) = song {
    return crate::song::single_beat_peer_beam(...);
}
```

**Rung 5 extends this cleanly.** New three-way narrowing:

```rust
if let (Some(song_path), Some(peer_home_2), Some(deploy_target)) =
       (song, dance_with, deploy_to) {
    return crate::deploy::execute_deploy(...);
}
```

Byte-equality preserved for all four non-deploy paths (Rung 1 --song
alone, Rung 4 --song + --dance-with, Rung 0 no-song, --hello-world).

### 1.6 The stub-envelope-with-real-substrate-authority pattern IS landed

`bootstrap/src/dance.rs:107-128` (Reed `dfac8fe`, LANDED) — the
substrate-authority-naming discipline at envelope emission:

```
+ dance_authority: @dance (Mara `4f079c8` canonical spec; Path C recognition)
+ resonance_authority: @resonance (Mara `9e48710`; Kuramoto coupling ancestor)
+ cyberpunk_authority: @cyberpunk (Reed `8e6e517` cybernetic_coherence = λ₀(Δ_F))
+ bauchladen_authority: @bauchladen (Mara `4575340`; content-addressed shared prior)
+ ladder_rung: 4 (Reed GREEN discharging Mara `417ec25` Scope B narrowed)
+ substrate_authority: @dance + @resonance + @cyberpunk + @bauchladen (Rung 4 minimum viable)
```

**Rung 5 preserves the pattern.** Each envelope line names authority +
OID + reading. The Rung 4 substrate line ("@dance + @resonance +
@cyberpunk + @bauchladen") extends at Rung 5 to include the deployment
authorities.

**Verdict on substrate-already-had-the-word coverage.** ~85% for
Scope A: all envelope authorities and composition points are landed
(spec-altitude for garden species; shard-altitude for @bauchladen +
@mirror/mosaic + @dance); the stub pattern is landed; the dispatch
pattern is landed; the three-way narrowing extends cleanly. The
missing 15%: the actual `shards/spectral/garden/*.mirror` species files.
Scope A does not require them (envelope-naming precedent per Rung 4).
Scope B/C would require them (operational discharge cannot proceed
without the species-decl naming what "nix build against a garden
entry" MEANS at substrate altitude).

---

## §2. The three scopes formalized

### 2.1 Scope A — Mycelial-envelope-declared substrate (recommended)

**Formal shape.** `mirror peer beam <home_A> --song <s> --dance-with
<home_B> --deploy-to <target>` runs the Rung 4 dance to establish
shared_root_oid, THEN dispatches to `bootstrap/src/deploy.rs`
`execute_deploy` which emits a deployment envelope naming:

- `deployment_target: <target-string>` (the operator-supplied string;
  MAY be `spectral.engineer` or `file:///tmp/mirror-deploy` or any
  other URL-shaped string — the target is declarative, not
  operationally verified at Rung 5)
- `nix_derivation_oid: <stub-hash>` (FNV-1a hash of `mirror.spec` bytes
  concatenated with shared_root_oid; the stub-derivation OID that WOULD
  be produced if `@mirror/mosaic` had actually compiled the mirror.spec
  garden{} block to a nix derivation at Rung 5.5)
- `mycelial_propagation_route: envelope-declared` (a literal string
  declaring the route is envelope-only; Rung 5.5 lifts to actual
  `@bauchladen` gossip)
- `deployment_endpoint: <target-string>` (echoes deployment_target; the
  endpoint the derivation WOULD publish to if propagation ran)
- `deployment_verdict: envelope-declared-substrate` (a literal string;
  Rung 5.5 lifts to `converged | dispersed | chimera` per Rung 4
  discipline extended to deployment altitude)
- Six substrate authorities:
  `@spectral/garden` (Mara `ad03fda`; family-root forward-promised)
  `@spectral/garden/nix` (Mara `ad03fda` §6.2; species forward-promised)
  `@bauchladen` (Mara `4575340`; content-addressed shared substrate)
  `@dance` (Mara `4f079c8` + Reed `dfac8fe`; ensemble coordination)
  `@mirror/mosaic` (`fa8b4c8`; compilation boundary)
  `@song/beat` (Mara `94e55eb`; the atomic execution unit)

**Cost.** ~120-160 lines Rust in `bootstrap/src/deploy.rs`; ~150-200
lines Rust in `bootstrap/tests/peer_beam_deploy_mycelial_shard.rs`;
~30 lines in `bootstrap/src/lib.rs` for three-way flag narrowing.
Reed lands in ONE tick following the Rung 4 template verbatim. No
new grammar. No new species-decl. No subprocess. No network.

**Benefit.** Rung 5 lands as ONE tick. The substrate arc closure now
carries deployment altitude — the composition
`mirror peer beam <home_A> --song shared.song --dance-with <home_B> --deploy-to <target>`
is empirical. Envelope-naming preserves the binding-point where Rung
5.5 real `nix build` + Rung 6 real spectral.engineer HTTP contact will
enter WITHOUT ambiguity about what those altitudes plug into.

**Precedent.** Rung 4 dance.rs shipped as `stub_phase_for_peer` +
`kuramoto_order_parameter_two_peer` + `stub_shared_root_oid` with
forward-promises to `λ₀(Δ_F)` at Rung 4.5, actual `@bauchladen`
crystal-OID at Rung 5. This spec extends the pattern.

### 2.2 Scope B — Real nix flake generation, `file://` deployment target (refused)

**Formal shape.** Runtime spawns `nix build .#<derivation>` subprocess;
reads output-hash; writes to a `file:///tmp/mirror-deploy/<hash>` local
URL; verifies via `nix path-info`.

**Refusal reasoning.** `shards/spectral/garden/nix.mirror` species-decl
DOES NOT EXIST. `docs/specs/spectral-garden-git-package-manager.md`
§6.2 forward-promises it verbatim:

> `@spectral/garden/nix` — the Nix-resolved sibling. Surface:
> `source ~nix'<flake-ref>'`. Resolution pipeline: nix evaluate +
> derivation_hash compute + (forward-promised) `derivation_to_oid`
> bridge. The substrate's hermetic-build story for dev environments.

Reed's runtime cannot in good faith invoke `nix build` at deployment
altitude when the substrate-decl species that would name what "nix
build against a garden entry" MEANS at substrate altitude has not
landed. This is not conservatism — this is `[[feedback-craft-not-
deliver]]` at Rung 5 altitude. Scope B is a Reed tick that would ship
runtime WITHOUT substrate-decl authorization; the deploy.rs would
invoke `nix build` under a substrate binding that the substrate has
not made.

**Substrate-decl prerequisite for Scope B.** Mara authors
`shards/spectral/garden/nix.mirror` species-decl (~200 lines
following the `shards/mirror/garden.mirror` pattern; declares
`nix_source`, `nix_derivation`, `derivation_to_oid` bridge, four
composed bilaterals). Mara also authors the parent family-root
`shards/spectral/garden.mirror` (~150 lines; per §6.4 shape). Only
THEN can Reed's `nix build` subprocess invocation be substrate-honest.

**Landability of the prerequisite.** ~1-2 Mara ticks per shard × 2
shards = 2-4 Mara ticks BEFORE Reed can execute Scope B. Then ~2-3
Reed ticks for the runtime. Total: 4-7 ticks. Compared to Scope A's
1 Reed tick, this is a substantially larger cascade AND (more
importantly) requires substrate-decl work that is off the current
`mara/song-substrate-decl-v0.1` branch's scope per
`docs/loop/CURRENT.md`.

### 2.3 Scope C — Full mycelial nix deployment to spectral.engineer (refused)

**Formal shape.** Runtime generates nix flake from mirror.spec via
`@mirror/mosaic` compilation → pushes derivation to
spectral.engineer's binary cache → gossips derivation OID via
`@bauchladen` crystal exchange → verifies Aumann agreement at
deployment endpoint via `verify_coherence` predicate.

**Refusal reasoning.** Every component of Scope C's operational
infrastructure is UNDECLARED substrate:

1. **spectral.engineer as deployment target.** `CHANGELOG.md`
   verbatim: "v1.0.0 is the spectral.engineer cloud deployment
   (per `roadmap/wip/v1-launch.md`)." Current repo state is pre-v0.1;
   spectral.engineer is v1.0-future scope. The submodule at
   `spectral.engineer/` (2026-06-27) is the SEL-typed jurisdiction
   for garden packages — NOT an operational deployment endpoint with
   an HTTP cache or SSH deployment key.

2. **Nix flake structure the compiler emits.** `@mirror/mosaic` today
   does NOT emit nix derivations from a mirror.spec's garden{} block;
   that capability is spec-forward-promised at
   `docs/specs/song-replaces-plans-and-loops.md` §5.1 movement
   `perform_deployment_epoch` voice `@spectral/garden/nix` — same
   substrate gap as Scope B (§2.2).

3. **Mycelial propagation protocol.** `@bauchladen` gossip is landed
   at spec altitude (`docs/specs/bauchladen-autopoietic-fate.md`) but
   `shards/bauchladen.mirror` §"Forward-promised inheritors" declares
   the network-exchange machinery as UNlanded. No `bauchladen_gossip`
   action exists at runtime altitude; no nix binary cache endpoint is
   configured; no derivation-OID exchange protocol has been specified.

4. **SSH keys / API credentials.** Reed's session has zero operational
   context for spectral.engineer credentials. The `AGENTS.md`
   convention notes SSH signing (default `~/.ssh/id_ed25519`) but this
   is COMMIT-signing not DEPLOYMENT-signing.

**Substrate-decl prerequisites for Scope C.** All of Scope B's
prerequisites PLUS:
- `shards/spectral/engineer.mirror` species-decl naming the DNS
  endpoint carrier (per Taut `c54740c` §5.6 Rung 6 forward-promise)
- Operational spectral.engineer nix binary cache endpoint (currently
  non-existent per grep of the mirror repo)
- `shards/bauchladen/gossip.mirror` species-decl OR extension of
  `shards/bauchladen.mirror` with `bauchladen_gossip(c: crystal, target:
  peer) -> imperfect(propagation_witness, ref, ref)` action
- Alex operational input on: what IS the spectral.engineer cache
  endpoint URL? What credentials does deployment carry? What is the
  mycelial propagation protocol (git push + fetch? IPFS-like? nix
  binary cache HTTP)?

**Landability.** Blocked on Alex operational input. Cannot be
substrate-adjudicated at spec altitude without it.

### 2.4 Comparison table

| Scope | Substrate-decl prereq | Reed runtime ticks | Total ticks | Substrate honesty |
|-------|----------------------|--------------------|-|------|
| A (recommended) | None (Rung 4 pattern extends) | 1 | 1 | HIGH — envelope-naming precedent from Rung 4 |
| B (refused) | 2-4 Mara ticks (`shards/spectral/garden.mirror` + `shards/spectral/garden/nix.mirror`) | 2-3 | 4-7 | MEDIUM — requires substrate-decl work off current branch |
| C (refused) | Scope B prereqs + `shards/spectral/engineer.mirror` + `shards/bauchladen/gossip.mirror` + Alex operational input | 3-5 | 7-12+ | LOW without Alex input |

**Substrate-honest verdict:** Scope A now. Scope B/C forward-promised
with explicit prerequisites named.

---

## §3. The canonical runtime for Scope A

### 3.1 Module boundary

New module: `bootstrap/src/deploy.rs` (~120-160 lines). Sibling of
`bootstrap/src/song.rs` (Rung 1-3) and `bootstrap/src/dance.rs` (Rung 4).

Existing modules extended:
- `bootstrap/src/lib.rs`:
  - Add `pub mod deploy;` after existing `pub mod dance;` line (line 47).
  - Add `deploy_to: Option<&str>` parameter to `cmd_peer_beam` signature
    (currently `fn cmd_peer_beam(..., song, dance_with, ...)`).
  - Add `--deploy-to` flag parsing at the three dispatch points that
    parse `--song` and `--dance-with` (peer form at ~3182, subcommand
    form at ~3277, anonymous form at ~3339).
  - Insert three-way narrowing at `cmd_peer_beam` body BEFORE the
    existing two-way `--song --dance-with` narrowing at line 5019:

    ```rust
    // Rung 5 (2026-07-13) — `--deploy-to <target>` mycelial-envelope-
    // declared dispatch per Mara `<this-spec-oid>` Scope A. When all
    // THREE of --song + --dance-with + --deploy-to are present, the
    // peer executes `execute_deploy` which composes over Rung 4 dance
    // shared_root_oid and emits deployment envelope naming @spectral/
    // garden + @spectral/garden/nix + @bauchladen + @dance + @mirror/
    // mosaic + @song/beat substrate authorities. Byte-equality
    // preserved for two-way dance-only path via `if let (Some, Some,
    // Some)` narrowing.
    if let (Some(song_path), Some(peer_home_2), Some(deploy_target)) =
           (song, dance_with, deploy_to) {
        let spec_path_2 = std::path::PathBuf::from(peer_home_2).join("mirror.spec");
        return crate::deploy::execute_deploy(
            peer_home,
            peer_home_2,
            &spec_path,
            &spec_path_2,
            song_path,
            deploy_target,
            ctx,
        );
    }
    ```

- `bootstrap/src/mcp.rs`: extend `mirror_peer_beam` inputSchema with
  optional `deploy_to: {"type": "string"}` property; extend dispatch
  to pass through to CLI. ~20 lines total.

### 3.2 The canonical `execute_deploy` signature

```rust
/// Fire a @song at two peer-homes, establish dance shared_root_oid,
/// then declare mycelial nix deployment envelope naming @spectral/
/// garden + @spectral/garden/nix + @bauchladen + @dance + @mirror/
/// mosaic + @song/beat substrate authorities. Envelope-declared per
/// Mara `<this-spec-oid>` Scope A (Rung 5).
///
/// Rung 5.5 forward-promise: replace stub_nix_derivation_oid with
/// actual `nix build` subprocess output-hash once `shards/spectral/
/// garden/nix.mirror` species-decl lands.
///
/// Rung 6 forward-promise: replace envelope-declared propagation with
/// actual `@bauchladen` gossip over network to spectral.engineer nix
/// binary cache endpoint once operational infrastructure exists.
///
/// Byte-equality preserved for non-`--deploy-to` paths: this function
/// is only entered when `cmd_peer_beam` observes ALL THREE of
/// `Some(song_path)`, `Some(peer_home_2)`, `Some(deploy_target)`. All
/// prior dispatch paths (Rung 0, 1, 4) remain identical.
pub fn execute_deploy(
    peer_home_1: &str,
    peer_home_2: &str,
    spec_path_1: &std::path::Path,
    spec_path_2: &std::path::Path,
    song_path: &str,
    deploy_target: &str,
    _ctx: &Ctx,
) -> i32
```

### 3.3 The canonical envelope shape

Following Rung 4 dance.rs discipline verbatim (`+ key: value` per line
after `@@ ... @@` header):

```
@@ deploy @spectral/garden/deployment via @dance × (@song × @mirror/mosaic × @bauchladen) mycelial-envelope-declared at spectral-engineer altitude (Rung 5) @@
+ peer_home_1: <path>
+ peer_home_2: <path>
+ song_path: <path>
+ deploy_target: <target-string>
+ dance_shared_root_oid: <hex-string from execute_dance stub_shared_root_oid>
+ nix_derivation_oid: <hex-string from stub_nix_derivation_oid>
+ mycelial_propagation_route: envelope-declared (Rung 6 forward-promise: actual @bauchladen gossip)
+ deployment_endpoint: <deploy_target verbatim>
+ deployment_verdict: envelope-declared-substrate (Rung 5.5 forward-promise: converged | dispersed | chimera per @dance discipline)
+ spectral_garden_authority: @spectral/garden (Mara `ad03fda`; family-root forward-promised)
+ spectral_garden_nix_authority: @spectral/garden/nix (Mara `ad03fda` §6.2; species forward-promised)
+ bauchladen_authority: @bauchladen (Mara `4575340`; content-addressed shared substrate)
+ dance_authority: @dance (Mara `4f079c8` + Reed `dfac8fe`; ensemble coordination Rung 4)
+ mirror_mosaic_authority: @mirror/mosaic (`fa8b4c8`; compilation boundary)
+ song_beat_authority: @song/beat (Mara `94e55eb`; atomic execution unit)
+ ladder_rung: 5 (Reed GREEN discharging Mara `<this-spec-oid>` Scope A)
+ substrate_authority: @spectral/garden + @spectral/garden/nix + @bauchladen + @dance + @mirror/mosaic + @song/beat (Rung 5 minimum viable)
```

### 3.4 The stub_nix_derivation_oid helper

```rust
/// Rung 5 stub nix_derivation_oid: hex-encoded FNV-1a hash of the
/// concatenation of (mirror.spec bytes at peer_home_1, shared_root_oid
/// from dance). Under actual `@mirror/mosaic` compilation, both peers
/// with identical mirror.spec + identical shared_root_oid would emit
/// derivations with THIS OID; T3 asserts the field's presence, not
/// its cryptographic derivation-hash content (Rung 5.5 upgrades to
/// actual `nix build` subprocess output).
fn stub_nix_derivation_oid(spec_bytes: &[u8], shared_root_oid: &str) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in spec_bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    for b in shared_root_oid.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{:016x}", h)
}
```

### 3.5 Composition with `execute_dance`

Load-bearing: `execute_deploy` MUST invoke `execute_dance`'s substrate
work (the shared_root_oid computation) but MUST NOT re-emit the dance
envelope. Two options:

**Option (i) — Refactor dance.rs.** Extract `compute_dance_state(peer_1,
peer_2, song_path) -> DanceState` returning `{shared_root_oid, phase_1,
phase_2, kuramoto_r, aumann, verdict}`; both `execute_dance` and
`execute_deploy` invoke it; `execute_dance` emits its envelope,
`execute_deploy` calls `compute_dance_state` for shared_root_oid then
emits its own composed deployment envelope.

**Option (ii) — Inline the two helpers.** Copy `stub_phase_for_peer` +
`kuramoto_order_parameter_two_peer` + `stub_shared_root_oid` calls into
`execute_deploy`; emit deployment envelope only.

**Adjudication: Option (i).** Substrate-honest per DRY discipline and
per `[[feedback-substrate-already-had-the-word]]`: the dance module
already carries the shared_root_oid computation; the deploy module
COMPOSES OVER dance, not RE-IMPLEMENTS dance. Extract a
`compute_dance_state` helper (~10 lines) and have both entry points
call it. Rung 4 test byte-equality preserved via the same emission
logic; Rung 5 gets the shared_root_oid as a first-class value to
compose with.

### 3.6 Test infrastructure

New test file: `bootstrap/tests/peer_beam_deploy_mycelial_shard.rs`
(~150-200 lines). Follows `peer_beam_dance_coherence_shard.rs` pattern
verbatim (fixture builder + `mirror_bin()` helper + `repo_root()`
helper + numbered `t01_*` … `t05_*` assertions).

**Five T-tests:**

- `t01_deploy_mycelial_execution_exits_zero` — all three flags present;
  command exits 0.
- `t02_deploy_envelope_names_six_substrate_authorities` — stdout
  contains all six authority lines (spectral_garden_authority,
  spectral_garden_nix_authority, bauchladen_authority, dance_authority,
  mirror_mosaic_authority, song_beat_authority).
- `t03_deploy_envelope_carries_stub_nix_derivation_oid` — stdout
  contains `+ nix_derivation_oid: <16-hex-digits>` line matching the
  regex `\+ nix_derivation_oid: [0-9a-f]{16}$`.
- `t04_deploy_envelope_composes_over_dance_shared_root_oid` — stdout
  contains `+ dance_shared_root_oid: <hex>` line; the hex value MATCHES
  what `execute_dance` would emit for the same (peer_home_1,
  peer_home_2, song_path) triple (compose the composition assertion by
  running the dance-only invocation and comparing the shared_root_oid).
- `t05_no_deploy_flag_preserves_dance_envelope_byte_equality` —
  regression guard: running with `--song X --dance-with Y` but NO
  `--deploy-to` produces IDENTICAL stdout to the Rung 4 test-case
  fixture.

### 3.7 CLI flag naming: `--deploy-to <target>`

Adjudicated per §6(c): `--deploy-to <target>` (with `<target>` a
URL-shaped string) is preferred over alternatives:
- `--spectral-engineer <url>` — refused; hard-codes spectral.engineer
  as the ONLY target, violating envelope-declared-substrate discipline
  (any deploy target should be nameable, even file:// URLs).
- `--deploy <target>` — refused; ambiguous verb ("deploy" what?).
- `--publish-to <target>` — refused; naming collision with
  `@bauchladen.publish` action; `--deploy-to` reads as
  peer-beam-emits-a-deployment-envelope-declaring-target which is
  operationally accurate for Scope A.

### 3.8 MCP schema extension

`bootstrap/src/mcp.rs::mirror_peer_beam` inputSchema — add optional
property:

```json
"deploy_to": {
  "type": "string",
  "description": "Rung 5 mycelial-envelope-declared deployment target (Mara `<this-spec-oid>` Scope A). When present alongside song + dance_with, peer emits deployment envelope naming @spectral/garden + @spectral/garden/nix + @bauchladen + @dance + @mirror/mosaic + @song/beat substrate authorities. Target is a URL-shaped string (may be spectral.engineer, file://, or any other target); Rung 5.5 forward-promises actual `nix build` subprocess; Rung 6 forward-promises actual mycelial gossip to target endpoint."
}
```

Dispatch: parse the optional `deploy_to` argument; pass through to
CLI as `--deploy-to <value>` when present.

---

## §4. Composition with landed carriers

### 4.1 `@spectral/garden` (deployment substrate family-root)

Landed at spec altitude (`docs/specs/spectral-garden-git-package-manager.md`
`ad03fda`, LANDED 2026-06-24). Species-decl file forward-promised at
§6.4. Rung 5 envelope names it as `spectral_garden_authority` with
citation to the spec's family-root shape.

**Substrate-decl gap named:** `shards/spectral/garden.mirror` species
family-root shard does not exist. Rung 5 envelope-naming does NOT
require it (per Rung 4 precedent — envelope authorities can be spec-
altitude if landed with recognition candidate). Scope B/C DOES require
it. Substrate-honest to name the gap explicitly.

### 4.2 `@spectral/garden/nix` (crystal exchange via nix)

Landed at spec altitude (§6.2 of same spec above). Species-decl file
forward-promised. Rung 5 envelope names as `spectral_garden_nix_
authority`; the `nix_derivation_oid` field's OID would be the actual
derivation-hash if `@spectral/garden/nix` had operational discharge.

### 4.3 `@mirror/mosaic` (compilation boundary)

`shards/mirror/mosaic.mirror` LANDED (2026-06-09). The build-system
prism at compile altitude. Rung 5 envelope names it as the
compilation-boundary authority — this is the substrate that WOULD
compile the mirror.spec's garden{} block into candidate nix derivations
at Rung 5.5 operational-discharge altitude. Naming mosaic in the
envelope declares the binding-point where compilation would enter.

**Composition note:** at Rung 5.5, `execute_deploy` would invoke
`@mirror/mosaic.compile(mirror.spec) -> nix_derivation_set` as a
subprocess step before the `nix build` subprocess. The Rung 5 stub
skips both.

### 4.4 `@song` + `@song/beat` + `@dance` (Rung 4 composed with)

Landed at runtime altitude (`bootstrap/src/song.rs` `0cc4e11` + `bootstrap/src/dance.rs` `dfac8fe`).
Rung 5 COMPOSES OVER Rung 4: shared_root_oid produced by
`execute_dance` becomes the deployment's dance_shared_root_oid; the
deployment envelope's `nix_derivation_oid` stub is a function of
(mirror.spec bytes, dance shared_root_oid) — the substrate cascades.

**Composition semantics:** deployment IS what happens AFTER dance has
established ensemble agreement. Rung 4 → Rung 5 is the same
substrate-arc altitude relationship as Rung 1 → Rung 2 (single-beat
→ phrase), Rung 2 → Rung 3 (phrase → movement), Rung 3 → Rung 4
(single-peer → multi-peer dance). Each rung strictly composes.

### 4.5 `@bauchladen` (crystal exchange)

`shards/bauchladen.mirror` LANDED (Mara `4575340`, 2026-06-29). The
content-addressed shared substrate. Rung 5 envelope names as
`bauchladen_authority`; the `mycelial_propagation_route: envelope-
declared` field explicitly forward-promises the actual
`bauchladen_gossip` action for Rung 6.

**Composition note:** at Rung 6, `execute_deploy` would invoke
`@bauchladen.gossip(nix_derivation_crystal, target_peer) -> propagation_
witness` for each coupled peer in the ensemble. The Rung 5 stub emits
`envelope-declared` as the literal string; Rung 6 lifts to actual
gossip machinery.

### 4.6 `@coherence` (deployment coherence metric)

Landed at Reed `8e6e517` Path B annotation on `shards/cyberpunk.mirror`
(`cybernetic_coherence = λ₀(Δ_F)`). Rung 5 envelope does NOT emit a
coherence metric (Rung 4 stub already emits `coherence_altitude:
stub`); Rung 5.5 forward-promise lifts the `deployment_verdict` field
from `envelope-declared-substrate` to `converged | dispersed | chimera`
per the Rung 4 classifier discipline extended to deployment altitude.

**Composition note:** at Rung 5.5, `execute_deploy` would compute
`deployment_coherence = λ₀(Δ_F_deployment)` where `Δ_F_deployment` is
the difference between the operator's declared deployment target
coherence and the running system's measured coherence (per Mara
`d21337b` §5.3 progression `deploy_to_spectral_engineer`
verify_coherence discipline).

---

## §5. Ambiguity surface for Alex adjudication

**Scope A DOES NOT require Alex operational input.** Reed can execute
Tick 5b immediately after this spec's landing.

**Scope B WOULD require Mara to author 2-4 substrate-decl shards
BEFORE Reed can run.** This is a substrate-decl branch decision (this
spec's branch is `mara/song-substrate-decl-v0.1` per
`docs/loop/CURRENT.md`); the additional shards fit the branch's
naming discipline but expand its scope.

**Scope C WOULD require Alex operational input on:**

1. **Is spectral.engineer a live deployment target?** The
   `spectral.engineer/` submodule exists as SEL-jurisdiction; grep
   confirms no operational deployment infrastructure. `CHANGELOG.md`:
   "v1.0.0 is the spectral.engineer cloud deployment" — future-scope.
   Alex: is there a spectral.engineer nix binary cache endpoint URL
   that Rung 6 should target?

2. **What's the nix flake structure `@mirror/mosaic` emits?** No
   compilation-to-nix-derivation exists today. Alex: does mosaic's
   nix-emit pipeline follow standard nix flake schema (nix flake with
   `outputs.<system>.<derivation-name>`) or a substrate-specific
   variant?

3. **What's the mycelial propagation protocol?** Three candidates:
   git push + fetch (spectral.engineer/garden/<package> per-repo git
   protocol); IPFS-like content-hash gossip (no infrastructure exists);
   nix binary cache HTTP (nix-serve or Cachix-shaped). Alex: which is
   the substrate-honest answer for Rung 6?

4. **Are there SSH keys / API credentials for deployment?** Reed has
   commit-signing keys (`~/.ssh/id_ed25519`) but not deployment
   credentials. Alex: is there a deployment key configured for
   spectral.engineer? Where does Rung 6 read it?

**None of these questions are unresolvable at Rung 5.** They are Rung
6-blocking. Rung 5 Scope A explicitly names them as "envelope-declared
substrate; Rung 6 forward-promise" and ships without needing them
resolved.

---

## §6. Five sub-ambiguities (adjudicated)

### (a) Which scope A/B/C

**Adjudicated: Scope A.** Per §0 + §2 above. Scope B refused as
substrate-decl-blocked (garden species not landed); Scope C refused as
operational-infra-blocked (spectral.engineer is v1.0-roadmap).

### (b) Deployment envelope field naming

Options:
- `deployment_target` vs `deploy_target` vs `target`
- `nix_derivation_oid` vs `derivation_hash` vs `nix_output_oid`
- `spectral_engineer_url` vs `deployment_endpoint` vs `target_url`
- `deployment_verdict` vs `deploy_status` vs `mycelial_verdict`

**Adjudicated:** `deployment_target` (matches Mara `d21337b` §5.1
narrative movement's `deployment_target` naming); `nix_derivation_oid`
(names the substrate binding — this WOULD be a nix derivation OID at
Rung 5.5 under `@spectral/garden/nix`); `deployment_endpoint` (echoes
target; naming stays substrate-target-agnostic per Scope A
envelope-declared discipline; NOT `spectral_engineer_url` which
hard-codes one target); `deployment_verdict` (matches Rung 4 dance's
`convergence_verdict` naming for classifier field).

### (c) CLI flag naming

Options:
- `--deploy-to <target>` vs `--spectral-engineer <url>` vs `--deploy <target>` vs `--publish-to <target>`

**Adjudicated:** `--deploy-to <target>`. Per §3.7 reasoning. Target-
agnostic naming preserves envelope-declared-substrate discipline;
`--spectral-engineer` hard-codes one target; `--deploy` is ambiguous;
`--publish-to` collides with `@bauchladen.publish` action.

### (d) MCP tool exposure

Options:
- Extend `mirror_peer_beam` MCP tool with optional `deploy_to` field
- Mint new `mirror_deploy` MCP tool

**Adjudicated: extend `mirror_peer_beam`.** Per Rung 1-4 precedent
(each rung extends the same tool with an additional optional field);
per `[[feedback-cli-subcommand-nesting-is-geometric-ground-truth]]` +
per the substrate-decl reading that peer_beam IS the substrate primitive
lifted through @song altitude compositions (single-beat → phrase →
movement → dance → deployment). A new `mirror_deploy` tool would
duplicate all seven existing `mirror_peer_beam` flags plus add the new
one; that is not substrate-honest.

Rung 6+ MAY promote to a `mirror_deploy` sibling tool once the
deployment envelope carries substantially different semantics from
peer-beam envelope, but at Rung 5 the deploy envelope IS the peer-beam
envelope with one additional key (following Rung 4's `--dance-with`
precedent verbatim).

### (e) Test infrastructure

Options:
- Subprocess spawn a fake HTTP server (nix-serve mock)
- Mock spectral.engineer via `file://` URL
- Envelope-only assertions (no subprocess, no mock)

**Adjudicated: envelope-only assertions.** Per Scope A's envelope-
declared-substrate discipline; Rung 5 does NOT invoke subprocess and
does NOT contact network endpoints; the test fixture uses any URL-
shaped string as `--deploy-to` and asserts envelope shape only. Rung
5.5 test adds subprocess assertions when actual `nix build` lands;
Rung 6 test adds mock or real network endpoint assertions when actual
mycelial gossip lands.

Fixture uses `--deploy-to file:///tmp/mirror-deploy` for the T-tests
(any URL string works; `file://` chosen because it's clearly
non-operational and highlights the envelope-declared nature of Rung 5).

---

## §7. Two-tick landing sequence

### 7.1 Tick 5a (Mara, this spec)

**This spec IS Tick 5a.** Discharges the substrate-decl adjacencies for
Rung 5:
- Scope A/B/C adjudication in favor of Scope A.
- Canonical shape for `bootstrap/src/deploy.rs`.
- Canonical envelope contract.
- Test infrastructure spec.
- CLI flag naming.
- MCP schema extension.
- Composition semantics with Rung 4 dance.rs.
- Refusal reasoning for Scope B/C with substrate-decl prerequisites.
- Ambiguity surface for Alex adjudication (Rung 6-blocking questions).

**Substrate-decl adjacencies discharged this tick:** none new. Scope A
extends the Rung 4 envelope-naming pattern without requiring new
species mints. The existing `shards/song/beat.mirror` line 456 verbatim
already forward-promises the mycelial substrate:

> Rungs 5-6: mycelial propagation via nix binary cache (@bauchladen
> gossip); full @spectral/garden mycelial deployment.

This spec's Scope A reading of Rung 5 as envelope-declared-only is
substrate-consistent with the shard's naming: the shard says Rung 5
STARTS mycelial propagation; Scope A discharges the naming altitude
(envelope-declared substrate); Rung 5.5 discharges the operational
altitude (actual `nix build`); Rung 6 discharges the full network
altitude (actual gossip to spectral.engineer).

**Commit shape:**

```
📝 Mara [substrate-pull:realize] [ladder-rung-5-spec] Mycelial nix
deployment to spectral.engineer substrate direction — Scope A/B/C
adjudication + canonical runtime shape (Scope A adjudicated;
substrate-honest one-tick landability; Rung 5.5 / Rung 6 forward-
promised with substrate-decl prerequisites named)
```

### 7.2 Tick 5b (Reed, RED → GREEN pair)

**Reed executes IMMEDIATELY after Tick 5a lands.** No Alex operational
input required. No new substrate-decl work required.

**RED landing (~10 minutes Reed):**
- Author `bootstrap/tests/peer_beam_deploy_mycelial_shard.rs`
  (~150-200 lines) per §3.6.
- Fixture: `write_repo_with_song_and_two_peers_and_deploy_target()`
  helper following Rung 4 pattern.
- Five T-tests per §3.6.
- Assertion messages cite `<this-spec-oid>` + substrate authority OIDs.
- Commit as:
  ```
  🔴 Reed [substrate-pull:realize] [tdd:deploy-mycelial] [ladder-rung-5-red]
  peer_beam_deploy_mycelial_shard RED — --deploy-to flag + Scope A
  mycelial-envelope-declared deployment envelope with six substrate
  authorities not yet implemented
  ```

**GREEN landing (~30 minutes Reed):**
- Author `bootstrap/src/deploy.rs` (~120-160 lines) per §3.1-§3.6.
- Refactor `bootstrap/src/dance.rs` to extract `compute_dance_state`
  helper (~10 lines) per §3.5 Option (i).
- Extend `bootstrap/src/lib.rs` per §3.1 (three-way narrowing + flag
  parsing at three dispatch points).
- Extend `bootstrap/src/mcp.rs` per §3.8.
- Verify all Rung 0-4 tests still pass (byte-equality regression
  guard).
- Verify five new T-tests pass.
- Commit as:
  ```
  🟢 Reed [substrate-pull:realize] [tdd:deploy-mycelial] [ladder-rung-5-green]
  peer_beam_deploy_mycelial_shard GREEN — Scope A mycelial-envelope-
  declared deployment runtime via bootstrap/src/deploy.rs + --deploy-to
  grammar + composition over Rung 4 dance shared_root_oid
  ```

**Total: 2 Reed sub-ticks (RED + GREEN), ~40 minutes wall-clock,
following the Rung 4 template verbatim.**

---

## §8. Refusals + forward-promises

### 8.1 Rung 5.5 territory (actual `nix build` subprocess)

**Forward-promised.** Requires:
- `shards/spectral/garden.mirror` family-root species-decl (~150 lines
  Mara-authored per `docs/specs/spectral-garden-git-package-manager.md`
  §6.4).
- `shards/spectral/garden/nix.mirror` species-decl (~200 lines
  Mara-authored per same spec §6.2).
- `bootstrap/src/deploy.rs` extended with subprocess spawn:
  `Command::new("nix").arg("build").arg(&flake_ref).output()` (~50
  lines Reed-authored).
- `bootstrap/tests/peer_beam_deploy_mycelial_shard.rs` extended with
  nix-subprocess assertions IF nix is available in the test
  environment; SKIP gracefully otherwise (~30 lines Reed).
- Landability: 3-5 Mara ticks + 2 Reed ticks = 5-7 total ticks.

### 8.2 Rung 6 territory (actual spectral.engineer HTTP contact)

**Forward-promised.** Requires all of §8.1 PLUS:
- `shards/spectral/engineer.mirror` species-decl naming the DNS
  endpoint carrier (~150 lines Mara-authored per Taut `c54740c` §5.6).
- `shards/bauchladen/gossip.mirror` species-decl OR extension of
  `shards/bauchladen.mirror` with `bauchladen_gossip` action (~200
  lines Mara-authored).
- Alex operational input per §5 (deployment endpoint URL, credentials,
  propagation protocol, flake schema).
- `bootstrap/src/deploy.rs` extended with HTTP client + `@bauchladen`
  gossip protocol implementation (~300+ lines Reed-authored).
- End-to-end integration test against real (or mock) spectral.engineer
  endpoint (~400 lines Reed-authored).
- Landability: 5-8 Mara ticks + 4-6 Reed ticks + Alex operational
  input = 10-14+ total ticks + operational infrastructure work.

### 8.3 Refusals

**Rung 5 SHALL NOT attempt:**
- Actual `nix build` subprocess (Rung 5.5).
- Actual network HTTP contact with spectral.engineer (Rung 6).
- Actual `@bauchladen` crystal exchange (Rung 6).
- Actual mycelial gossip protocol (Rung 6).
- Multi-target deployment (deploying to N > 1 targets simultaneously;
  Rung 7+ forward-promise).
- Deployment rollback (Rung 7+ forward-promise).
- Deployment verification via `verify_coherence` predicate (Rung 5.5
  forward-promise per §4.6).

**Rung 5 SHALL:**
- Extend `mirror peer beam` with `--deploy-to <target>` flag.
- Emit deployment envelope naming six substrate authorities.
- Compose over Rung 4 dance shared_root_oid via extracted
  `compute_dance_state` helper.
- Preserve byte-equality for all non-`--deploy-to` paths.
- Land in one Reed tick following the Rung 4 template.

---

## §9. Recognition candidate

**Promotable at Rung 5 landing:**

`#R-envelope-declared-substrate-preserves-binding-at-deployment-altitude`

**Statement:** at deployment altitude, an envelope-declared substrate
(one that names authority + composition + verdict WITHOUT invoking the
authority's operational discharge) preserves substrate binding when
three conditions hold: (1) the naming cites the authority's landed
spec-altitude witness with OID; (2) the composition names the
substrate-decl prerequisites for operational discharge; (3) the
verdict names the forward-promised altitude at which operational
discharge would run. Under these three conditions, envelope-declared
substrate is substrate-honest — it MAKES the paradigm-shift claim
without OVERREACHING to operational discharge that the substrate has
not yet authorized.

**Two-witness density:**
- Rung 4 dance.rs (Reed `dfac8fe`): envelope-declared coherence
  metric (`coherence_altitude: stub`) preserved binding for actual
  λ₀(Δ_F) at Rung 4.5 forward-promise.
- Rung 5 deploy.rs (Reed forthcoming): envelope-declared nix
  derivation OID (`nix_derivation_oid: <fnv1a-hash>`) preserves
  binding for actual `nix build` at Rung 5.5 forward-promise.

**Promotion trajectory:** LANDED as candidate at Rung 5 landing;
promoted to LANDED recognition at Rung 5.5 landing (three-witness
density: Rung 4 coherence stub + Rung 5 nix-derivation stub + Rung 5.5
actual nix-build discharge that HONORS the stub's binding).

**Substrate ancestry of this recognition:**
- Rung 4's `#R-multi-peer-coherence-phase-lock-realizes-dance-at-
  runtime-altitude` (Mara `417ec25`) — the parent recognition; this
  recognition SPECIALIZES it at deployment altitude.
- `[[feedback-craft-not-deliver]]` (~40+ instances) — the substrate
  discipline this recognition names at deployment altitude: envelope-
  declared substrate IS crafting; overreaching to operational
  discharge without substrate-decl authorization IS delivering.
- `[[feedback-substrate-already-had-the-word]]` (~74+ instances) —
  the substrate discipline this recognition composes over; envelope-
  declared substrate NAMES what the substrate already has (spec-
  altitude witnesses), not what the substrate will build.

---

## §10. Recognition ancestry

**This session's arc-continuation landings (2026-07-13):**

- Alex 2026-07-13 in-transcript /loop mandate — "climb the ladder
  until unresolvable ambiguity that cannot be postponed further";
  Rung 5 Scope A is the substrate-honest climb; Scope B/C surface
  ambiguity that CAN be postponed (Scope B by Mara authoring 2-4
  more shards; Scope C by Alex operational input) but ARE named at
  the appropriate altitude (§5, §8) as forward-promises.
- Reed `dfac8fe` — Rung 4 GREEN; `bootstrap/src/dance.rs` established
  the envelope-declared-substrate pattern this spec extends.
- Reed `5b301a4` — Rung 4 RED; the RED test-authoring pattern Reed
  will follow for Rung 5.
- Mara `417ec25` — Rung 4 spec; Scope B narrowed adjudication
  precedent this spec follows for Scope A.
- Reed `0cc4e11` — Rung 3 GREEN; `bootstrap/src/song.rs` established
  the module-per-rung pattern (song / dance / deploy).
- Mara `94e55eb` — `shards/song/beat.mirror` sixth species mint;
  named `bootstrap/src/dance.rs` (Rung 4) + mycelial propagation
  (Rung 5-6) as forward-promises this spec discharges at Rung 5.
- Taut `c54740c` — gap-scout §5.5 Rung 5 named the module path
  (`bootstrap/tests/garden_deployment_song_shard.rs`) and the
  five-beat mycelial-song shape; this spec ADJUDICATES the scout's
  own hedge ("HIGH risk — first runtime tick to invoke real nix
  subprocess") toward Scope A envelope-declared substrate.
- Mara `d21337b` — canonical `@song @spectral/garden/deployment` at
  §5.1; the reference target this spec names as forward-promised
  Rung 6 discharge; the five-movement composition is the substrate
  ancestor at spec altitude.

**Prior arc landings (2026-07-06 through 2026-07-12):**

- Mara `4f079c8` — `@dance` canonical spec; Kuramoto + Aumann +
  Cavagna formalization; the coordination-without-signal substrate
  Rung 4 discharged and Rung 5 composes over.
- Mara `9e48710` — `@resonance` canonical spec; inter-peer coupling
  shapes Fate tournaments toward basins; the coupling substrate the
  mycelial propagation would discharge at Rung 6.
- Reed `8e6e517` — `@coherence` Path B annotation
  (`cybernetic_coherence = λ₀(Δ_F)`); the deployment coherence
  metric forward-promised at Rung 5.5.
- Reed `71a4689` — coordination-without-signal annotation on
  `shards/algebra/metalogue.mirror:348-374`; the recognition Rung 4
  discharged at runtime altitude and Rung 5 extends at deployment
  altitude.
- Mara `4575340` — `docs/specs/bauchladen-autopoietic-fate.md` +
  `shards/bauchladen.mirror`; the content-addressed shared substrate
  Rung 5 names as authority and Rung 6 would discharge at gossip
  altitude.
- Mara `ad03fda` — `docs/specs/spectral-garden-git-package-manager.md`;
  the deployment substrate family-root spec Rung 5 composes over;
  §6.2 + §6.4 forward-promise the species shards Scope B/C require.
- @song family-root Arc 6: `f01cf9f` (family-root) + `54ff1e8`
  (progression) + `cc5a440` (voice) + `4efbf16` (movement) +
  `0434a39` (narrative) + `6b9bc5c` (phrase); the six-species roster
  the runtime ladder discharges.

**Substrate ancestors (pre-arc):**

- `shards/mirror/mosaic.mirror` (`fa8b4c8`, 2026-06-09) — the
  compilation-boundary authority Rung 5 names at envelope altitude;
  Rung 5.5 would invoke at operational altitude.
- `shards/mirror/garden.mirror` (`13328a3`, 2026-06-25) — the
  `garden { source ~git'...' }` block substrate-decl; the source of
  mirror.spec's garden{} block Rung 5.5 would compile.
- `shards/io/git.mirror` (`a1b507a`) — the git-native package
  manager @io adapter Scope B/C would compose over.
- `shards/mirror/peer/beam.mirror` (2026-06-25) — the peer_beam
  substrate primitive this ladder discharges through @song altitude
  compositions.

**External math ancestry (Rung 5 forward-promises):**

- Nix RFC 49 (flake semantics; content-addressed derivations) —
  Rung 5.5 operational discharge substrate.
- Nix binary cache HTTP protocol — Rung 6 operational discharge
  substrate for mycelial propagation.
- Kuramoto 1975 (phase-oscillator coupling) — Rung 4 substrate this
  spec composes over.
- Aumann 1976 (agreement under content-addressed common prior) —
  Rung 4 substrate; the deployment altitude Aumann-agreement
  condition (all peers observe same current_root_OID) is the Rung 5
  paradigm-shift claim.
- Cavagna 2010 (topological-neighbor coupling in starling flocks) —
  Rung 6 mycelial propagation substrate.
- Sheldrake 2020 (*Entangled Life*) — Rung 5-6 mycelial framing.

---

**Word count:** ~4750. Every substrate claim cited with file:line +
OID where landed, or explicit forward-promise / spec-only naming where
not. Scope adjudication: **Scope A (mycelial-envelope-declared
substrate) — one Reed tick landability, ~85% substrate-already-had-
the-word coverage, Rung 4 template extends verbatim. Scope B refused
as substrate-decl-blocked (garden species not landed); Scope C refused
as operational-infra-blocked (spectral.engineer is v1.0-roadmap).**
Recommended sequence: **Tick 5a (this spec, Mara) → Tick 5b (Reed RED
→ GREEN pair, ~40 minutes wall-clock).** Rung 5.5 / Rung 6 explicitly
named as forward-promises with substrate-decl prerequisites listed at
§8.1 + §8.2 + Alex operational input at §5.
