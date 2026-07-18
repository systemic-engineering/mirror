---
title: The @trust Family-Root — SSH-Compiler + Passkey-Garden Two-Altitude Bridge of One Chain-of-Trust
subtitle: Canonical spec landing @trust as family-root marker at duality-basis altitude (sibling to @void, @torus, @peer, @mirror, @kintsugi, @io, @fate, @tool, @subject, @spectral, @bauchladen, @autopoietic, @butterfly). Compose-side: @peer/registry species-decl for OID -> fractal::Subject resolution. Grounds Alex 2026-07-18 direct-transcript direction B ("The @trust floor is in the compiler. And it extends to the passkey.") + Reed + Alex 2026-04-03 passkey-spectral-bridge insight + SEL v1.1 licensable-party root. Content-addressed non-forgeable identity evolution at two altitudes of the SAME chain-of-trust.
status: canonical-spec
date: 2026-07-18
author: Mara
---

# The @trust Family-Root

> "B. In the garden. The @trust floor is in the compiler. And it extends to the passkey."
> --- Alex Wolf, 2026-07-18 direct-transcript, ratifying DIRECTION B (OID + registry) at
> `rust/src/main.rs::at_operator` dispatch surface (Reed `23cb7bb`) AND naming the extension
> to the garden's passkey/PRF altitude.

> "The passkey is the root. The spectral key is the tree.
> PRF proves the root authorized each growth ring.
> The tree grows. The root holds."
> --- Reed + Alex, 2026-04-03,
> `~/dev/systemic.engineering/practice/insights/cosmos/passkey-spectral-bridge.md`

---

## Substrate authority chain

- **Alex 2026-07-18 direct-transcript verbatim** (this session, in-transcript):
  "B. In the garden. The @trust floor is in the compiler. And it extends to the passkey."
  Ratifies DIRECTION B (OID + registry) at at_operator dispatch AND names the two-altitude
  extension the compiler-side and garden-side both discharge.

- **Reed + Alex 2026-04-03 passkey-spectral-bridge insight**
  (`~/dev/systemic.engineering/practice/insights/cosmos/passkey-spectral-bridge.md`, 2026-04-03):
  The bridge architecture. WebAuthn credential + PRF salt-chain produces content-addressed
  spectral-state evolution; SHA-512 over (prev_state || prf_output || entropy) is the evolution
  function; multiple passkeys can map to the same spectral identity (authorization gates, not
  identity sources); credential mapping + evolution chain constitute the trust chain.

- **Alex 2026-07-14 SSH-signing design intent** (peer-persistence session compaction, in-transcript;
  verbatim citation at `shards/subject/visibility/sheaf.mirror:21-32`,
  `shards/peer/persistence.mirror:31-42`, `shards/io/secrets.mirror:31-42`):
  "Each peer has their own key in the private part of their visibility. NOT projected into the
  git state and instead stays .git/mirror side. Only connected through Fractal.Lens. A pointer.
  Not the thing." The SSH altitude of the same @trust chain.

- **Mara `d39e852` closure canonical spec** (`docs/specs/2026-07-18-the-compiler-in-one-sentence.md`
  §6.2 + §16 forward-promise 1): named @trust family-root as Mara authorship territory
  next-tick discharge; the compiler-in-one-sentence chain routes tool_result signing THROUGH
  @trust chain terminating at @alex root.

- **Reed `23cb7bb` at_operator stub** (`rust/src/main.rs::at_operator` §@io/git.commit arm):
  Stakes the dispatch shape for `(author_oid, committer_oid, message)` at at_operator; returns
  substrate-honest Err naming @peer/registry species-decl + @trust family-root as the authorship
  territory this spec discharges. 7/7 tests GREEN.

- **Mara `22c803a` StageFreight tool_species_stagefreight_witnessed** (
  `shards/epistemologic/property/tool_species_stagefreight_witnessed.mirror` conjunct 4
  `tool_invocation_signed_at_alex_root`): DEFERRED-RED conjunct explicitly named this landing
  as its discharge point. The requires-clause inserts into the composed predicate without shape
  change per §4 forward-promise 1.

- **Systemic Engineering License (SEL) v1.1** (`docs/specs/subject-family-root-sel-licensable-party.md`,
  Mara `5c06ee8`, 2026-07-14; grounding for `shards/subject.mirror`): The licensable-party
  altitude that @trust chains attribute to. Every @trust chain terminates at a @subject with
  actor_kind classification; @alex is the first substrate-external human_a @subject-instance
  per §20 named-ancestor roster entry #1.

---

## §1 What @trust IS

@trust is the family-root for the SUBSTRATE-DECL CHAIN-OF-TRUST that governs identity-evolution
admissibility across every altitude the substrate ships identity at. The chain has FIVE
load-bearing properties:

1. **Content-addressed** --- every chain-step is a content-addressed OID; tampering with any
   step breaks every subsequent OID.
2. **Non-forgeable** --- only the root-holder can authorize the next evolution step (SSH-key
   holder for compiler-side; passkey-holder for garden-side).
3. **Root-terminating** --- every chain traversal terminates at a @subject root (typically @alex
   for the mirror-substrate case; any human_a @subject-instance per SEL §1 for general use).
4. **Two-altitude admissible** --- the same substrate-decl'd chain admits BOTH SSH-signing at
   compiler altitude AND passkey/PRF at garden altitude. Neither is privileged; both are
   instantiations of the family-root's substrate-decl'd invariant.
5. **Additive / append-only** --- chains only grow; no revocation-by-rewrite. Compromised roots
   can be superseded by new roots that map to the same @subject via multiple-passkey /
   multiple-key discipline per passkey-spectral-bridge §"The Mapping".

---

## §2 Two altitudes of the SAME chain

The passkey-spectral-bridge insight (2026-04-03) already named the parallel; this landing lifts
it to substrate-decl.

| Aspect | Compiler altitude (SSH) | Garden altitude (Passkey/PRF) |
|--------|--------------------------|-------------------------------|
| Root artifact | SSH private key on disk (`~/.ssh/id_ed25519`) | WebAuthn credential in secure enclave |
| Authorization proof | ed25519 signature over commit tree | HMAC-SHA-256 PRF over server salt |
| Chain step | git commit (SSH-signed) | spectral_state evolution (SHA-512 fold) |
| Chain traversal | git log --show-signature back to root | spectral_state chain back to genesis |
| Root-terminating @subject | @alex (or any pack-peer human_a) | @alex (or any authenticated reader) |
| Content-addressed hash | git SHA-1 (interop) / SHA-256 (canonical) | SHA-512 over (prev || prf_output || entropy) |
| Multiple-key policy | Multiple SSH keys per @subject via subject_visibility_sheaf | Multiple passkeys map to same spectral identity (bridge §"The Mapping") |
| Vendor surface | @io/crypto (ssh-key, ed25519-dalek) | @io/crypto (sha-512) + @io/webauthn (forward-promised species) |
| Anti-extraction | Key material stays .git/mirror-side per Alex 2026-07-14 | GDPR right-to-erasure via spectral chain deletion per bridge §GDPR |

**The two altitudes are structurally isomorphic under the family-root's substrate-decl'd
invariant.** The compiler-side SSH altitude has ~14 months of substrate history (bootstrap SSH
signing, `~/.ssh/id_ed25519` for Reed's identity per `~/.reed/04-TECH.md`, AGENTS.md
never-override-gpg discipline); the garden-side passkey altitude has the passkey-spectral-bridge
insight's architecture proposal + Alex 2026-07-18 in-transcript ratification. Both compose over
@io/crypto at the mechanism boundary.

**Why the isomorphism is load-bearing.** The compiler and the garden are the SAME substrate
deployed at two altitudes. The compiler runs on developer hardware and produces content-addressed
git commits SSH-signed by pack-peer keys. The garden runs on reader hardware (browser) and
produces content-addressed spectral evolution PRF-authenticated by reader passkey. When @alex
authors a shard, the SSH-signed commit IS the compiler-altitude @trust chain step. When a reader
annotates a paper on `garden.systemic.engineering`, the PRF-authenticated evolution IS the
garden-altitude @trust chain step. Both are `evolve(prev_state, authorization_proof, entropy)`
calls at their respective altitudes.

---

## §3 Family-root altitude declaration

**Decision: marker-primary with minimal 5-op prism inheritance from @void.**

The substrate-honest choice mirrors `shards/void.mirror` `974a3f6` (family-root marker per Alex
"deserves the recognition") + `shards/tool.mirror` `34ecd83` (5-op prism family-root). @trust
sits at the DUALITY-BASIS altitude sibling to @void / @torus / @peer / @mirror / @kintsugi /
@io / @fate / @tool / @subject / @spectral / @bauchladen / @autopoietic / @butterfly:

- **Marker-primary**: the family-root NAMES the recognition and inherits @void's 5-op basis; the
  operational discharge lives at species altitude (@peer/registry this tick; @tool.sign at
  @tool family-root; @trust/chain and @trust/signature forward-promised species).
- **5-op prism**: `prism @trust { focus/project/split/shift/settle trust_chain_step }` per
  Recognition #79 (5-op basis IS Void's tongue). Every family-root inherits Void's basis per
  `shards/void.mirror` §Composition inheritance discipline.
- **Minimal carrier**: `trust_chain_step` --- the substrate-decl'd typed record naming the
  content-addressed authorization step at either altitude. Concrete instantiation deferred to
  species-decl (SSH-side / passkey-side).

**NOT this tick:**
- The `@trust/chain` species (full chain traversal machinery); forward-promised per §12.
- The `@trust/passkey` species (WebAuthn/PRF composition surface); forward-promised per §12.
- The `@trust/ssh` species (git-signing composition surface); forward-promised per §12.
- The runtime chain-resolution mechanism (Rust-altitude authorship); Reed territory pending Mara
  species landings + Alex ratification.

**Two-tick discipline** per
[[feedback-legibility-over-foundation-when-collapsing]]: this tick lands the family-root marker +
minimal 5-op prism + one species-decl (@peer/registry) discharging the at_operator boundary
Reed staked at `23cb7bb`. Subsequent ticks land species specializations as consumer-realization
pulls them in.

---

## §4 @alex as first @subject with @trust root instantiation

Per SEL v1.1 §1 + `shards/subject.mirror` Landing 3 named-ancestor roster entry #1: @alex is
the first substrate-external `human_a` @subject-instance in the compiler. The @trust family-root
instantiates against @alex at TWO altitudes:

### Compiler altitude (SSH)

At compiler-boot time, the mirror binary carries an SSH public key embedded via the tools{}
block pin (per closure spec §4 tools{} grammar hint at `shards/mirror/spec.mirror` `67e8629`).
The key is `alex@systemic.engineer`'s ed25519 public half; the corresponding private half
never leaves @alex's disk (Reed's `~/.ssh/id_ed25519` for Reed's Pack-peer identity per
`~/.reed/04-TECH.md`; @alex's disk for @alex's authorship).

@trust chain traversal at compiler altitude:

```
  OID (author_oid or committer_oid)                     -- fed to at_operator
    ↓ (via @peer/registry.resolve; §6 below)
  fractal::Subject                                      -- typed carrier per fractal step 4 (82bc599)
    ↓ (via subject.email + subject.name via phone::git_commit_as)
  git commit --author="Name <email>" -S                 -- SSH-signed
    ↓ (git verify-commit ... resolves ssh_fingerprint)
  ssh_fingerprint SHA256                                -- witness field per @spectral/signature
    ↓ (via git config user.signingkey pointing at ~/.ssh/id_ed25519.pub)
  ed25519 public key                                    -- the SSH-side root artifact
    ↓ (via subject_visibility_sheaf: peer's private-scope binding)
  @alex (human_a @subject-instance per SEL §1)          -- chain terminates
```

### Garden altitude (Passkey/PRF)

At garden `garden.systemic.engineering` first-visit time, the reader's browser calls
`navigator.credentials.create()` and creates a passkey in the reader's device's secure enclave.
The passkey's `credential_id` becomes the root artifact for that reader's @trust chain instance
in the garden. Per bridge §"The Tick Button": `evolve(genesis, prf_output, beam)` produces the
first spectral_state; subsequent visits chain via `evolve(current, prf_output, entropy)`.

@trust chain traversal at garden altitude:

```
  spectral_state_current OID (per bridge §"The Mapping")
    ↓ (via SpectralBridge.credential_map lookup; bridge Rust example)
  spectral_identity_oid (the root; per bridge §"The Mapping")
    ↓ (append-only chain of evolve(prev, prf, entropy) SHA-512 hashes)
  credential_id (passkey; per bridge §"The Architecture")
    ↓ (via WebAuthn hmac-secret PRF; per bridge §"Why PRF")
  reader's device-side WebAuthn credential
    ↓ (multiple passkeys per spectral identity admissible per bridge §"The Mapping")
  human_a @subject-instance (reader; SEL §1 licensable party)
```

**@alex authoring in the garden**: when Alex uses `garden.systemic.engineering` from a browser
with a passkey, the garden-altitude chain terminates at the human_a @subject-instance `@alex`
(the SAME @subject as the compiler-altitude chain). Two altitude-distinct chains, ONE @subject
root. This IS the two-altitude bridge the family-root lands.

---

## §5 Chain traversal: OID → resolves through @trust chain → terminates at root

The chain-traversal invariant `chain_terminates_at_root` is the load-bearing substrate-decl'd
predicate the family-root ships. Every OID that appears at either altitude MUST resolve through
a finite number of chain-steps to a @subject-instance whose actor_kind is `human_a` (or
`historical_witness_r` per subject.mirror §Landing 5+ discharge for deceased-ancestor citation
via @arxiv).

**Predicate signature (species-decl at @trust family-root; body composed at species altitude):**

```
chain_terminates_at_root(step: trust_chain_step, root: subject) -> verdict
```

**Splinter-pole (honest)**: the chain terminates at `root` within finite steps; each intermediate
step is content-addressed; each transition is authorization-proof-verified (SSH signature or
PRF output); no cycle exists in the chain.

**Narcissus-pole (deceptive)**: the chain claims termination at `root` but a step's content-hash
doesn't verify OR the authorization proof is absent OR a cycle exists OR the terminating
@subject-instance's actor_kind is not admissible per SEL §1.

**Rice-safe bound**: reads only byte-visible state (content-hash comparison, signature
verification via @io/crypto, chain-step tag traversal); no program-semantics inspection.

---

## §6 Non-forgeability: only the root-holder can authorize evolution

The non-forgeability property is architecturally load-bearing across BOTH altitudes; it's the
reason the compiler ships SSH-signing-only per AGENTS.md discipline + the reason the garden
ships passkey-authenticated-PRF per bridge.

### Compiler altitude non-forgeability

The SSH-signing discipline at `shards/tool/git.mirror::commit_signing_ssh_only` (Mara `34ecd83`)
is the substrate-decl'd guarantee: no commit reaches the @trust chain without an SSH signature
from the caller's `~/.ssh/id_ed25519`. AGENTS.md "SSH signing default" NEVER-override-gpg.format
discipline is the pre-@trust bootstrap-tick guarantee at the Rust altitude; this shard lifts
that guarantee to substrate-decl.

### Garden altitude non-forgeability

The passkey-authenticated PRF discipline per bridge §"The Evolution Function": the chain is
non-forgeable because PRF outputs require the passkey holder. The `evolve(current, prf_output,
entropy)` function is a pure hash; the only mutating input is `prf_output`; PRF output requires
the secure-enclave-side hmac-secret; hmac-secret requires the passkey-holder's
credential-presence proof (biometric or PIN gesture at `navigator.credentials.get()`).

### Multiple-key non-forgeability preservation

Both altitudes admit multiple keys per @subject:
- Compiler: subject_visibility_sheaf's private-scope binding admits multiple SSH keys per
  @subject via peer.private per Alex 2026-07-14 design intent.
- Garden: bridge §"The Mapping" "Multiple passkeys can map to the same spectral identity. Any
  authorized passkey can approve the next evolution step."

Multiple keys per @subject IS the anti-fragility surface: losing one key doesn't lose identity;
compromising one key doesn't compromise identity (superseded by adding a new key). The @trust
chain's identity IS the @subject; the keys are authorization gates per bridge §"The Mapping".

---

## §7 @peer/registry species-decl --- the OID -> Subject resolution surface

This tick lands ONE species under @peer that discharges the at_operator boundary Reed staked at
`23cb7bb`. Full spec follows; substrate-decl at `shards/peer/registry.mirror` (companion shard
landed this tick).

### §7.1 What @peer/registry IS

The runtime **Subject registry**: a content-addressed store mapping `OID` → `fractal::Subject`
values, resolving through the @trust chain to a terminating @subject root. This is the
substrate-decl'd typed surface for `rust/src/main.rs::at_operator`'s `(author_oid,
committer_oid, message)` dispatch shape: the OIDs come in as strings; the registry resolves
them into typed `&fractal::Subject` values that phone.rs's `git_commit_as` consumes.

Species under @peer. Sibling to `@peer/void` (Mara `9c7de83`, K=0 default) and
`@peer/persistence` (Reed `2026-07-15`, K≥1 persistent peer with home). @peer/registry is
ORTHOGONAL to both: it's not a peer species (a KIND of peer); it's the resolution SURFACE that
maps content-addressed identity references TO peer values.

**Substrate-already-had-the-word altitude collision (audit clean):**

`@spectral/registry` (`shards/spectral/registry.mirror`, 2026-06-11) exists at the
supervisor-child-index altitude (BEAM Registry analogue for gen_prism children indexed by
`uuid_spectral`). @peer/registry sits at DIFFERENT altitude:

| Aspect | @spectral/registry | @peer/registry (this tick) |
|--------|--------------------|----------------------------|
| Family-root | @spectral (BEAM-runtime altitude) | @peer (identity altitude) |
| Key type | uuid_spectral (128-bit, dark 80-bit portion for lookup) | OID (content-addressed hash string) |
| Value type | gen_prism child handle | fractal::Subject |
| Purpose | Supervisor's typed child index | at_operator's OID-taking dispatch surface |
| Mutation discipline | Supervisor's restart-strategy invariant | Content-addressed append-only per @trust chain |

The word "registry" appearing at both altitudes is analogous to the word "git" appearing at
`@io/git` (mechanism altitude, LANDED) and `@tool/git` (identity/version/contract altitude,
LANDED) --- same word, different family-root, no substrate collision. `feedback-substrate-
already-had-the-word` respected: this landing NAMES the co-existence explicitly and cites the
spectral-side sibling.

### §7.2 Well-known Subjects

The registry ships with FOUR well-known Subject constructors that consumers pull without needing
registry-lookup:

1. **Subject::mirror()** --- the compiler's self-identity. `name="mirror"`,
   `email="mirror@spectral.engineer"`, `home=None`, `kind=Peer`. Constructor landed at fractal
   step 9 (Reed `73aeb8a`). Used as `committer` in Author≠Committer split when the compiler
   itself commits on behalf of a pack-peer author.

2. **Subject::void()** --- the K=0 default peer per Mara `974a3f6` + `9c7de83` + Alex "I am
   void." `name="void"`, `email="void@spectral.engineer"`, `home=None`, `kind=Void`.
   Constructor landed at fractal step 4 (Reed `82bc599`). Used as pre-character-crystallization
   substrate-observation identity.

3. **Subject::human(alex, alex@spectral.engineer)** --- the @alex human_a @subject-instance.
   `home=None` (Human @subject has no compiler-managed home). Used as `author` (with
   Subject::mirror() as committer) when the compiler commits on @alex's behalf under peer-ACL
   discipline. The @trust root at both altitudes.

4. **Subject::peer(<name>, <email>, <home>)** --- constructor for named pack-peers (Reed, Mara,
   Seam, Taut, Glint, Loki, Glint, ...). Each pack-peer has their own SSH key per Alex
   2026-07-14 design intent + their own home per @peer/persistence species-decl.

### §7.3 Current-peer resolution: how does mirror know "who am I now?" at runtime?

**Substrate-honest answer**: currently NO single canonical resolution path; the compiler reads
several admissible sources with a documented priority. This section names the FLOOR admissible
surface + forward-promises the resolution-canonicalization tick.

**Priority ladder (admissible sources; higher = winner):**

1. `mirror.spec` `pack{}.lead` block (per Alex 2026-06-25 "Reed is lead, yeah"): the compiler's
   current-peer at substrate altitude for the running project. `~peer'~/.reed'` resolves via
   @peer glass altitude + @io/git to the peer's home-repo `mirror.spec` `pack{}.lead`
   self-declaration per peer-ACL §6.2.
2. `GLUE_ACTOR` env var: transient per-invocation override (set by session glue-bus per Reed
   session convention `reed[$PWD][$UUID]`). Used for per-tool-invocation attribution.
3. `git config user.email` + `git config user.name`: fallback for non-substrate-pull-honest
   invocations (pre-mirror.spec projects, e.g. system tooling that reads git config directly).
4. `~/.reed/` / `~/.mara/` / etc. peer-home directory presence: last-resort inference (Reed's
   `~/.reed/CLAUDE.md` "Every session starts from zero" per Void discipline; identity-file bundle
   determines current character-specialization of the K=0 Void substrate).

**Forward-promised canonicalization tick**: as at_operator dispatch matures, the current-peer
resolution IS a species-decl'd typed action `@peer/registry.current_peer() -> subject`; the
priority ladder above becomes the substrate-decl'd body. Held at forward-promise to next Mara
tick per two-tick discipline.

### §7.4 Registry evolution: SSH-side chain and passkey-side chain

Each Subject entry in the registry can GROW. Growth = @trust chain evolution. Two altitudes,
two chain-step shapes:

**SSH-side chain evolution** (compiler altitude):

- Each SSH-signed git commit authored by the @subject IS a chain step.
- Chain step OID = git commit OID (SHA-1 today per interop; SHA-256 forward-promised per
  @io/git canonical).
- Chain-step authorization proof = ed25519 signature over commit tree.
- Chain-step content = commit tree (`.git/objects/`) + parent commit OID.
- Evolution admissibility per @spectral/signature discipline (Mara `d1ce901` + Alex 2026-07-14
  "rolling @song through the graph space? Like blockchain but without the waste?"): each
  contribution beat = one chain step; signature IS accumulated beat-sequence.

**Passkey-side chain evolution** (garden altitude):

- Each `navigator.credentials.get()` visit IS a chain step.
- Chain step OID = spectral_state SHA-512 hash per bridge §"The Evolution Function".
- Chain-step authorization proof = PRF output (hmac-secret HMAC-SHA-256 over server salt).
- Chain-step content = (prev_state || prf_output || interaction_entropy).
- Evolution admissibility per bridge §"The Salt Strategy": salt_n = SHA-256(spectral_state_{n-1});
  the chain IS the spectral history.

**One @subject, two altitude-distinct chains, ONE @trust family-root invariant.** Both
altitudes discharge `chain_terminates_at_root(step, root)` per §5 predicate.

### §7.5 Bilateral: `peer_registered_at_trust_root`

The registry's LOAD-BEARING bilateral. Asserts:

```
peer_registered_at_trust_root(oid: ref, root: subject) -> verdict
```

- Splinter-pole: `oid` resolves via `@peer/registry.resolve(oid) -> Some(subject)`; the resolved
  subject's @trust chain terminates at `root` per `chain_terminates_at_root(chain_head, root)`;
  the root's actor_kind is human_a (or historical_witness_r per subject.mirror §Landing 5+); the
  root is admissible per SEL §1 licensable-party contract.
- Narcissus-pole: `oid` resolves to a subject whose chain traversal fails (broken chain, wrong
  root, cycle, unadmissible actor_kind) OR `oid` doesn't resolve at all (registry has no entry).

Body discharges at realization boundary per [[feedback-craft-not-deliver]]; sentinel per shard
docblock: `peer=registered-at-trust-root-<altitude>` (altitude ∈ {ssh, passkey}).

---

## §8 Relationship to existing landings

| Existing landing | @trust chain composition |
|------------------|-----|
| `shards/subject.mirror` (Landing 3 lift: every @peer is ALSO a @subject) | @trust chain terminates at @subject; @peer/registry resolves OID → Subject; Subject carries actor_kind classifying the chain root |
| `shards/peer.mirror` (family-root: parametric peer type) | @peer/registry is the resolution surface; @peer.load(dir) reads mirror.spec.pack{}.lead per §7.3 priority 1 |
| `shards/peer/void.mirror` (K=0 default peer) | Subject::void() is a well-known Subject per §7.2 #2; Void has no chain (K=0, no character loaded, no authorization proof needed for pure substrate-observation) |
| `shards/peer/persistence.mirror` (K≥1 with home) | @peer/registry.resolve(oid) returns Subject::peer(name, email, home); persistence species-decl's four bilaterals compose with @trust chain admissibility |
| `shards/io/crypto.mirror` (ssh-key + ed25519_sign + ed25519_verify + sha2 + age) | @trust chain SSH-side authorization proof discharges through @io/crypto.ed25519_sign / ed25519_verify at mechanism boundary |
| `shards/io/secrets.mirror` + `shards/io/secrets/sops.mirror` | Peer's SSH key material stays .git/mirror-side per Alex 2026-07-14; @io/secrets is the peer-key-gated projection primitive; @trust chain composes over @io/secrets for private-scope binding |
| `shards/tool.mirror` `sign(invocation, signer) -> tool` action | The action that discharges through @trust chain; this landing NAMES the chain @tool.sign discharges INTO |
| `shards/tool/git.mirror` `commit_signed` + `commit_signing_ssh_only` | @tool/git.commit_signed IS the compiler-altitude @trust chain step-generating action; its species-decl'd contract enforces SSH-signing per AGENTS.md |
| `shards/epistemologic/property/tool_species_stagefreight_witnessed.mirror` conjunct 4 | `tool_invocation_signed_at_alex_root` DEFERRED-RED discharge point per Mara `22c803a` §4 forward-promise 1; conjunct 4 requires-clause insertion follows this landing |
| `shards/spectral/signature.mirror` (rolling @song of author contributions) | SSH-side @trust chain per §7.4; each @spectral/signature beat IS one @trust chain step at compiler altitude |
| `shards/mirror/spec.mirror` `pack{}.lead` + `pack{}.members` | @peer/registry §7.3 priority 1 source; lead's identity per @pack.peer variant declares the current-peer at substrate altitude |
| `docs/specs/2026-07-18-the-compiler-in-one-sentence.md` §6.2 + §16 forward-promise 1 | THIS landing discharges the forward-promise; the @trust family-root arrow in the closure spec's compiler-in-one-sentence chain lands here |
| `docs/specs/2026-07-18-stagefreight-delivery.md` (Marcus PR delivery chain) | The @trust chain signing the StageFreight PR delivery IS the compiler-altitude @trust chain terminating at @alex; conjunct 4 requires-clause activation depends on this landing |
| `docs/specs/subject-family-root-sel-licensable-party.md` (SEL v1.1) | @trust chain root-terminating @subject discipline grounds in SEL §1 licensable-party contract |
| `~/dev/systemic.engineering/practice/insights/cosmos/passkey-spectral-bridge.md` (Reed + Alex, 2026-04-03) | Garden-altitude @trust chain architecture; PRF-authenticated evolution IS the passkey-altitude chain-step discipline |

---

## §9 Recognition candidates surfaced (DO NOT RATIFY at this landing)

### #R-trust-is-one-chain-at-two-altitudes

> The @trust chain is ONE substrate-decl'd invariant discharged at TWO altitudes (SSH-signing at
> compiler, PRF-authenticated at garden). The isomorphism between the altitudes IS what makes the
> compiler and the garden the same substrate deployed twice, not two separate systems.

**First-witness gate landed retroactively across:** 14 months of SSH-signing bootstrap history
(git commits signed per AGENTS.md discipline); the passkey-spectral-bridge insight (2026-04-03,
Reed + Alex; three months of dormant architecture waiting for the compiler-side to name it);
Alex 2026-07-18 direct-transcript naming the extension ("the @trust floor is in the compiler.
And it extends to the passkey.").

**Second-witness gate**: this landing IS the second witness (family-root marker naming the
isomorphism at substrate-decl altitude).

**Mara lean**: PROMOTE THIS TICK per Alex's direct-transcript verbatim as promotion-track
naming. Alex adjudicates.

### #R-registry-is-the-at-operator-dispatch-surface

> The at_operator's OID-taking dispatch surface (Reed `23cb7bb`) requires a registry species
> because content-addressed identity references CANNOT be inlined at dispatch time. The registry
> is the SUBSTRATE-ARCHITECTURAL correspondence between the compiler-side content-addressed
> identity model and the runtime type resolution the phone.rs `git_commit_as` refactor needs.

**First-witness gate**: Reed `23cb7bb` at_operator @io/git.commit stub (2026-07-18); the Err
variant NAMES this registry surface as the resolution mechanism.

**Second-witness gate**: this landing IS the second witness (@peer/registry species-decl at
substrate-decl altitude).

**Mara lean**: hold at CANDIDATE strength; promote only after Reed's runtime resolution
empirical firing per @peer/registry.resolve implementation.

### #R-passkey-is-the-garden-side-void

> Per bridge §"The Tick Button": reader's first-visit `credentials.create()` IS the reader's
> character-crystallization event at garden altitude, structurally isomorphic to Pack peer
> boot-sequence identity-file-load per γ(Void, c) discipline. The passkey IS to the garden what
> the SSH key IS to the compiler: the authorization-proof-holder that admits @subject-attribution
> to the substrate.

**First-witness gate**: bridge §"The Tick Button" narrative + `shards/peer/void.mirror`
§"Consumer hardware --- Void runs locally" ("Every deployment of the paper is one Void-instance
per reader.").

**Second-witness gate**: this landing NAMES the isomorphism explicitly.

**Mara lean**: hold at CANDIDATE strength; second-order recognition; deep composition with
#R-void-is-the-default-peer (Mara 2026-07-18 promotion track).

---

## §10 Refused mints (audit clean)

Refused per substrate-already-had-the-word discipline:

1. **@passkey family-root** --- REFUSED. @trust family-root ALREADY carries the two-altitude
   discipline per §2; passkey is the garden-altitude AUTHORIZATION-PROOF-HOLDER, not a
   family-root. Forward-promised: @trust/passkey species (when garden-side WebAuthn integration
   authorship pulls it in).

2. **@webauthn family-root** --- REFUSED. @io/crypto family-species surface (per `shards/io.mirror`
   161-162 vendor-surface line: "hash + signature + AEAD over bytes. Vendor surface (sha2, age,
   ssh-key).") absorbs WebAuthn as a vendor primitive under @io. Forward-promised: @io/webauthn
   species (mechanism altitude, sibling to @io/crypto).

3. **@prf family-root** --- REFUSED. Same reasoning as @webauthn; PRF is a WebAuthn extension
   primitive under @io/crypto → forward-promised @io/webauthn species.

4. **@identity family-root** --- REFUSED. @subject family-root ALREADY carries the identity-
   attribution altitude per Mara `5c06ee8` + "every @peer is ALSO a @subject" Landing 3 lift;
   @trust is orthogonal (the CHAIN OF AUTHORIZATION, not the identity ITSELF).

5. **@registry family-root** --- REFUSED. @spectral/registry exists at supervisor-child-index
   altitude; @peer/registry lands at Subject-OID resolution altitude; multiplying the word to
   family-root altitude fails substrate-already-had-the-word discipline. Species-level
   specialization suffices.

6. **@chain family-root** --- REFUSED. @trust family-root ALREADY carries chain-traversal per
   §5; the chain IS the @trust family-root's ontology; a separate @chain family-root would
   duplicate.

7. **@signature family-root** --- REFUSED. @spectral/signature (Mara `d1ce901`) already carries
   the rolling-@song signature altitude; @trust.sign action + @tool.sign action + @spectral/
   signature species-decl cover the composition surface without a new family-root.

---

## §11 Empirical firing surfaces

When realized this arc admits four empirical firing surfaces (Reed's territory after Mara species
land):

1. **Registry lookup fires at at_operator boundary**: `at_operator("@io/git.commit",
   [author_oid, committer_oid, message])` resolves OIDs via @peer/registry.resolve, hands typed
   `&fractal::Subject` values to phone::git_commit_as. The Err-STUB at `rust/src/main.rs`
   `23cb7bb` becomes a substrate-decl'd success path.

2. **SSH-chain step generation fires at @tool/git.commit_signed**: each SSH-signed commit
   generates one @trust chain step at compiler altitude; @spectral/signature.signature_beat OID
   round-trips through the chain per §7.4.

3. **Passkey-chain step generation fires at garden `garden.systemic.engineering`**: each reader
   `credentials.get()` generates one @trust chain step at garden altitude per bridge §"Every
   subsequent visit"; the resulting spectral_state chain IS the garden-altitude @trust chain
   witnessed.

4. **Conjunct 4 discharge fires**: `tool_species_stagefreight_witnessed` conjunct 4
   (`tool_invocation_signed_at_alex_root`) requires-clause insertion at
   `shards/epistemologic/property/tool_species_stagefreight_witnessed.mirror` per Mara `22c803a`
   §4 forward-promise 1. All six @tool species (@tool/cargo, /git, /nix, /go, /docker,
   /gitlab_ci) become admissible-signed at empirical firing altitude.

---

## §12 Forward-promised (NOT landed this tick)

1. **`@trust/chain` species** --- typed chain-traversal machinery + full `chain_terminates_at_root`
   body. Species-decl next tick or when Reed's runtime resolution pulls it in.

2. **`@trust/ssh` species** --- SSH-side chain-step composition surface. Composes @tool/git.
   commit_signed + @io/crypto.ed25519_sign + @spectral/signature.

3. **`@trust/passkey` species** --- Passkey-side chain-step composition surface. Composes
   @io/webauthn (forward-promised @io species) + @io/crypto.sha512 + bridge's
   SpectralBridge.on_auth.

4. **`@io/webauthn` species** --- Mechanism altitude for WebAuthn CTAP2.1 + hmac-secret PRF.
   Sibling of @io/crypto. Composes with @trust/passkey species per bridge Rust example.

5. **`@peer/registry` runtime dispatch** --- Reed territory. Runtime resolution against `mirror.
   spec pack{}.lead` + `GLUE_ACTOR` env + `git config` per §7.3 priority ladder; typed action
   `current_peer() -> subject`.

6. **Multi-passkey / multi-SSH-key discipline formalization** --- Per bridge §"The Mapping" +
   Alex 2026-07-14 SSH-signing design intent; adds `subject_visibility_sheaf.private_keys:
   [key_material_ref]` species-decl'd list carrier + admissibility bilateral.

7. **Conjunct 4 requires-clause insertion** --- `shards/epistemologic/property/
   tool_species_stagefreight_witnessed.mirror` per Mara `22c803a` §4 forward-promise 1. Insert
   `requires tool_invocation_signed_at_alex_root(species, p)` between conjunct 3 and conjunct 5
   in the composed bilateral. Two-tick discipline: land after @trust/ssh species discharges the
   body.

8. **Passkey-spectral-bridge integration** --- garden-side @trust chain empirical firing.
   Discharges the garden-altitude @trust chain in the compiler binary that ships the garden
   endpoint. Library commitment (webauthn-rs kanidm per bridge Rust example) deferred to
   empirical-firing tick per Q4.

9. **@trust/chain content-addressability across altitudes** --- Prove (empirically) that a Subject
   authored at compiler-altitude and later engaged with the garden-altitude produces a bridge
   chain-step that composes cleanly. The isomorphism is architectural; the empirical firing
   waits for garden endpoint deployment.

---

## §13 Q's for Alex (non-blocking; surface at candidate strength)

**Q1 (naming)**: `chain_terminates_at_root` OR `chain_at_root_terminates` per composition-
primitive naming convention `<primitive>_of_<input-shape>`? Mara lean: `chain_terminates_at_root`
reads more naturally at splinter-pole discharge ("the chain terminates at root") than the
`_of_` construction ("chain-at-root-terminates" fights English). Composition-primitive
convention holds when the pattern is a value-type generalization (per feedback ratification
2026-07-18); this predicate is a chain-property, not a value-type generalization. Lean:
EXEMPTION admissible per feedback pattern's specific applicability. Alex adjudicates.

**Q2 (species vs family-root for @trust)**: this tick lands marker-primary with minimal 5-op
prism. Alternative: full-species-decl at family-root (like `shards/subject.mirror`'s 24KB
richness). Mara lean: MARKER-PRIMARY THIS TICK per legibility-over-foundation + two-tick
discipline; the family-root NAMES the recognition, species discharge operational surface next
tick. Alex adjudicates.

**Q3 (@peer/registry vs @trust/registry placement)**: this tick lands under @peer per
substrate-already-had-the-word (`@peer.load(dir)` resolution surface already at @peer altitude).
Alternative: under @trust as @trust/registry (registry IS the @trust chain's typed resolution
surface). Mara lean: UNDER @PEER THIS TICK per Reed's `23cb7bb` at_operator dispatch stub
naming @peer/registry specifically as the authorship territory; @trust/registry admissible as
an ALIAS-species (re-export) if Alex prefers the @trust altitude. Alex adjudicates.

**Q4 (WebAuthn library commitment)**: bridge Rust example uses `webauthn-rs` (kanidm). Mara
lean: HOLD LIBRARY DECISION at forward-promise until garden endpoint work begins;
naming a library at spec altitude before empirical firing violates substrate-honesty. Alex
adjudicates.

**Q5 (@trust chain compression policy)**: SSH-side chain grows unbounded (one step per commit);
passkey-side chain grows unbounded (one step per garden visit). Bridge §GDPR names
right-to-erasure but doesn't name compression. Mara lean: FORWARD-PROMISE compression policy;
start with unbounded append-only per anti-fragility discipline; compression is a Kagi-informed
decision (Merkle-DAG log compression is well-studied prior art). Alex adjudicates.

---

## §14 Composition surprises (as authored)

1. **The word "registry" collides admissibly with @spectral/registry at different altitude.**
   This is a POSITIVE surprise: it means the substrate's naming for typed content-addressed
   resolution surfaces is stable across altitudes. @spectral/registry indexes gen_prism children
   by uuid_spectral; @peer/registry indexes Subjects by OID; both are content-addressed typed
   surfaces at their respective family-root altitudes.

2. **The passkey-spectral-bridge insight (2026-04-03) is architecture that waited for the
   compiler-side to catch up.** The bridge was authored three months before Alex named the
   @trust extension direction; when the extension was named, the entire bridge architecture
   composed cleanly against @trust family-root with zero refactoring. This is
   substrate-already-had-the-word at cross-repository altitude: the systemic.engineering garden
   corpus carried the passkey-side @trust chain architecture; mirror's compiler-side pulled it
   in when Alex ratified the extension.

3. **@spectral/signature (Mara `d1ce901`, 2026-07-14) is already the SSH-side @trust chain at
   compiler altitude.** The rolling-@song architecture Alex named 2026-07-14 ("like blockchain
   but without the waste") IS the compiler-altitude @trust chain per §7.4; this landing NAMES
   the correspondence explicitly. First-witness for compiler-side @trust chain lands
   retroactively at `d1ce901`.

4. **The five-op prism inheritance from @void makes @trust's family-root shape trivial to author.**
   Per Recognition #79 (5-op basis IS Void's tongue) + `shards/void.mirror` §Composition
   inheritance discipline: every family-root inherits Void's basis; @trust needs only NAME the
   inheritance + declare the carrier. The family-root marker-primary shape is architectural
   consequence.

5. **The conjunct-4 uniform DEFERRED-RED across six @tool species (per Mara `22c803a` +
   `734035b`) is one-time-fix.** When @trust chain discharges the SSH-signature validation,
   ALL SIX species' conjunct 4 requires-clauses discharge simultaneously; no per-species
   authorship needed. Cascade-tick discipline: this landing enables ONE requires-clause
   insertion tick to close six deferred conjuncts.

---

## §15 One-sentence surprise

**The compiler and the garden are the same substrate ran twice at two different altitudes; the
@trust chain is what makes them the same, and it always was --- the passkey-spectral-bridge
insight authored 2026-04-03 was compiling itself against a family-root that didn't yet exist,
waiting for Alex to name the extension so the substrate could recognize what it had already
authored.**

---

## §16 Substrate decisions ancestry

- [[architecture-shards-as-substrate-source]]
- [[architecture-prism-as-trait-as-everything]] (`prism @trust`)
- [[architecture-glass-wall-substrate-types]] (transparency inheritance through @trust)
- [[architecture-form-process-partition-at-family-root]] (@trust stays at form-side; chain-
  step generation is process-side at species altitude)
- [[architecture-alignment-as-boundary-mathematics]] (splinter/narcissus poles per §5 + §7.5)
- [[feedback-substrate-already-had-the-word]] (~62nd instance; passkey-spectral-bridge carried
  the passkey-side @trust chain architecture 3+ months before family-root naming; @spectral/
  signature carried the SSH-side @trust chain 4+ days before)
- [[feedback-legibility-over-foundation-when-collapsing]] (two-tick discipline; marker-primary
  family-root this tick; full-species-decl if Alex ratifies deeper form)
- [[feedback-no-rust-extension-shortcut]] (pure-substrate + pure-docs authorship; NO Rust
  extension authored; @peer/registry runtime dispatch is Reed territory next tick)
- [[feedback-detector-inadequacy-answer-is-never-rust]] (chain-verification is bilateral
  resolver-arm sentinel-check; NO Rust extension for chain-verify)
- [[feedback-composition-primitive-naming-convention]] (Q1 audits the pattern's applicability
  to the chain predicate; requests exemption for chain-property)
- [[feedback-craft-not-deliver]] (`\`-obligation-blocked bilateral bodies; consumers pull
  realization via apply_h::act bilateral resolver-arm dispatch)
- [[feedback-reed-inflates-stub-empirical-firings]] (this spec + @peer/registry shard-decl are
  CANONICAL SPEC + SHARD-DECL, NOT runtime dispatch; Reed's territory)
- [[feedback-onto-family-root-is-the-ladder-Foerster-refused]] (@onto refused; @trust is a
  chain-of-authorization ontology, not a foundational-ladder ontology)

---

## §17 Cross-refs

- `docs/specs/2026-07-18-peer-registry-oid-resolution.md` --- companion @peer/registry deep
  spec (this tick).
- `docs/specs/2026-07-18-the-compiler-in-one-sentence.md` §6.2 + §16 forward-promise 1 ---
  the closure spec whose forward-promise this landing discharges.
- `docs/specs/2026-07-18-stagefreight-delivery.md` §6 --- StageFreight PR delivery chain
  signed at @alex root; conjunct 4 dependency chain.
- `docs/specs/subject-family-root-sel-licensable-party.md` --- SEL v1.1 grounding.
- `docs/specs/peer-persistence-and-home-projection.md` §12.3 --- ACL IS the sheaf structure;
  peer's private-scope binding admits key material.
- `~/dev/systemic.engineering/practice/insights/cosmos/passkey-spectral-bridge.md` --- Reed +
  Alex 2026-04-03; garden-altitude @trust chain architecture.
- `shards/trust.mirror` (this tick) --- family-root marker.
- `shards/peer/registry.mirror` (this tick) --- species-decl for at_operator resolution surface.
- `shards/subject.mirror` --- @subject family-root; @trust chain terminates at @subject-
  instance.
- `shards/peer.mirror` --- @peer family-root; parametric peer type.
- `shards/peer/void.mirror` --- K=0 default peer; Subject::void() well-known constructor.
- `shards/peer/persistence.mirror` --- K≥1 persistent peer; @peer/registry composes.
- `shards/tool.mirror` --- `sign` action discharges through @trust.
- `shards/tool/git.mirror` --- `commit_signed` action + `commit_signing_ssh_only` sub-bilateral.
- `shards/io/crypto.mirror` --- SSH-side authorization proof mechanism.
- `shards/io/secrets.mirror` --- Peer-key-gated projection; SSH key material private-scope.
- `shards/spectral/signature.mirror` --- SSH-side @trust chain per §7.4 correspondence.
- `shards/mirror/spec.mirror` `pack{}` --- current-peer resolution priority 1 source per §7.3.
- `rust/src/main.rs::at_operator` @io/git.commit stub --- the boundary this landing discharges.
- `rust/fractal/src/subject.rs` --- Subject envelope + Subject::mirror() + Subject::void() well-
  known constructors per §7.2.

---

*Landed 2026-07-18 by Mara. Pure-docs canonical spec (📝 markdown-only 📝 bypass). Discharges
Alex 2026-07-18 direct-transcript DIRECTION B + closure spec §16 forward-promise 1 + passkey-
spectral-bridge insight architecture at family-root altitude.*
