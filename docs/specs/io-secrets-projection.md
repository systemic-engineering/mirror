---
date: 2026-07-15
author: Mara
scope: Canonical spec for @io/secrets + @io/secrets/sops — the peer-key-gated secrets-projection species at @io Glass Wall altitude, plus the first concrete SOPS-backed vendor sub-species. Grounds shards/io/secrets.mirror (059cf1c + 57c5b3a + 64b0438) and shards/io/secrets/sops.mirror (059cf1c + 57c5b3a) alongside the two @io lift-tick companions shards/io/crypto.mirror (059cf1c) and shards/io/fs.mirror (059cf1c) in Alex 2026-07-14 SSH-signing design intent, the @io Glass Wall verbatim (io.mirror 37-40), SOPS vendor discipline, and @subject/visibility/sheaf composition surface. Reference-composition-only; does NOT re-declare carriers/actions the species-decls already mint. Names what the shards carry so downstream readers can navigate without opening five files.
status: canonical
companion:
  - shards/io/secrets.mirror
  - shards/io/secrets/sops.mirror
  - shards/io/crypto.mirror
  - shards/io/fs.mirror
  - shards/io.mirror
  - shards/subject/visibility/sheaf.mirror
  - shards/subject/visibility.mirror
  - shards/subject/visibility/private.mirror
  - shards/subject.mirror
  - shards/mirror/pack.mirror
  - shards/mirror/data/yaml.mirror
  - shards/mirror/data/json.mirror
  - shards/gift/lens.mirror
  - docs/specs/subject-visibility-sheaf.md
  - docs/specs/peer-persistence-and-home-projection.md
  - docs/insights/2026-05-26-glass-wall-and-cross-wall-kintsugi.md
  - docs/scouts/2026-07-15-mara-secrets-shape-proposal.md
  - docs/scouts/2026-07-15-taut-io-type-as-constructor-precedent.md
  - docs/audits/2026-07-15-seam-secrets-shape-proposal-phase-d-cascade.md
  - docs/audits/2026-07-15-seam-sheaf-secrets-composition-alignment.md
  - docs/audits/2026-07-15-seam-a1-re-adjudication-with-taut-precedent.md
---

# @io/secrets + @io/secrets/sops — the peer-key-gated secrets-projection species at @io Glass Wall altitude

*Canonical spec. Composition-only. Grounds the four-shard @io mint at
`shards/io/secrets.mirror` (059cf1c + 57c5b3a + 64b0438),
`shards/io/secrets/sops.mirror` (059cf1c + 57c5b3a),
`shards/io/crypto.mirror` (059cf1c), and `shards/io/fs.mirror`
(059cf1c) in the six ancestries the substrate already carried before
the mint: Alex's 2026-07-14 SSH-signing design intent (peer-key-gated
projection through @io/secrets), the @io Glass Wall discipline
(io.mirror 37-40 verbatim), the SOPS vendor tool (Julien Vehent 2017
+ CNCF v3.x Go rewrite 2020+), Seam's Phase D-cascade @secrets
adjudication (SEAM-RATIFY Scenario A at 019ad8f), Mara's own shape-
proposal at 766d930 + fc044ee + 9f5befa, and the Landing 6
composition-bridge closure at 64b0438 per Seam A1 re-adjudication at
e5d928e. Names what the shards carry; does not duplicate.*

---

## §0 Prelude — ancestry cascade + framing

### §0.1 Alex 2026-07-14 SSH-signing design intent (verbatim)

From `shards/subject/visibility/sheaf.mirror:21-32` (peer-persistence
session compaction, in-transcript) and reproduced verbatim at
`shards/io/secrets.mirror:31-42`:

> "Each peer has their own key in the private part of their
> visibility. NOT projected into the git state and instead stays
> .git/mirror side. Only connected through Fractal.Lens. A pointer.
> Not the thing."

> "@secrets prism and @secrets/sops to project visibility/private
> stuff onto disk through the Peers key."

Five load-bearing claims land in the two sentences (per shape-
proposal §0.2 sub-carriers 1-5; reproduced with @sheaf spec §0.1
sub-claims complement):

1. **Peer-key material lives at `.git/mirror`-side boundary.** Not
   projected into the git working tree. Keys are raw bytes at the
   @io opacity discipline (io.mirror 66-72). This species carries
   the REF (`key_material_ref`), never the bytes themselves. The
   substrate's `.git/mirror` side is an @io-mediated storage plane;
   the key material lives on the non-mirror-world side of the Glass
   Wall by Alex's explicit design intent.
2. **The pointer is not the thing.** Fractal.Lens is the pointer;
   the thing is the key bytes. Per
   `[[architecture-fractal-lens-pointer-thing-partition]]`, the
   species-decl's `key_material_ref` carrier IS the Fractal.Lens
   pointer; the key bytes are opaque per @io. Dereferencing happens
   at the @io realisation boundary; the substrate never handles the
   bytes in-tree.
3. **@secrets is the projection surface.** The prism named as
   `@secrets` in the design intent lands as `@io/secrets` per Seam
   Phase D-cascade Candidate 2 (SEAM-RATIFY at 019ad8f). The
   substrate lifts the design-intent root name into the @io family
   because the design-intent's every carrier (peer keys = raw
   bytes; ciphertext = foreign blob; SOPS = vendor SDK; disk write =
   POSIX syscall) fits @io Glass Wall verbatim (io.mirror 37-40).
4. **@secrets/sops is the first vendor projection method.** Alex
   named `@secrets/sops` explicitly as species alongside `@secrets`
   prism. The sub-species pattern admits sibling projection methods
   (`@io/secrets/age`, `@io/secrets/vault`, `@io/secrets/1password`
   as future siblings per §5.5).
5. **Visibility/private is the projection input.** The projection
   reads "visibility/private stuff" — @subject/visibility/private
   crystals restricted through the peer's ACL. The ACL-restriction
   is the sheaf-restriction carried by @subject/visibility/sheaf
   (landed at d1ce901); its `section_at` produces
   `section_at_stalk` — the input surface @io/secrets.project
   consumes.

The species-decl at `shards/io/secrets.mirror` (059cf1c + repairs)
mounts the peer-key-gated projection primitive Alex's design intent
already presupposed. Every altitude, carrier, and composition arrow
the spec below names traces to one of these five claims plus the
@io Glass Wall discipline that governs the projection's substrate
mount.

### §0.2 @io Glass Wall discipline (verbatim)

From `shards/io.mirror:37-40` (AGENTS.md substrate-pull; verbatim
substrate-decl):

> "@io is the substrate's only legitimate non-mirror surface. Any
> grammar that isn't mirror — Rust, Python, raw bytes, foreign
> blobs, vendor SDKs — must be under @io. Everything else is
> mirror grammar by definition."

Four load-bearing consequences for this species (per shape-
proposal §5.2 Reason 1 + Seam D-cascade §D2.a fit check):

- **SOPS is vendor SDK.** github.com/getsops/sops (Julien Vehent
  2017; CNCF-maintained v3.x Go rewrite 2020+); every SOPS
  invocation crosses the Glass Wall. FITS @io per family-root
  discipline.
- **Peer keys are raw bytes.** ssh-ed25519 private material,
  X25519 age keys, PGP secret keys — all opaque byte payloads;
  key algebra is vendor crypto by construction. FITS @io.
- **Ciphertext files are foreign blobs.** AEAD-produced encrypted
  payloads, SOPS-format .sops.yaml / .sops.json files — opaque to
  mirror substrate; parsed only via vendor tools. FITS @io.
- **Disk write is POSIX syscall.** open + write + close is a
  kernel syscall boundary; the substrate cannot fold past the
  vfs layer. FITS @io.

Every load-bearing carrier of this species lives on the non-
mirror-world side of the Glass Wall. Mounting @secrets anywhere
BUT @io (Candidate 1 family-root; Candidate 3 @subject/
visibility/secrets; Candidate 4 @sheaf/secrets) would duplicate
scope @io family-root already covers — the @onto refusal
precedent (Reed memory
`feedback-onto-family-root-is-the-ladder-Foerster-refused`)
applied to the secrets altitude. Landing 4 mounts at Candidate 2:
species-under-@io.

### §0.3 The lift-tick discipline (@io/crypto + @io/fs)

Per shape-proposal §5.5 Reason 2 + Seam D-cascade §D7 + §D9
REED-INLINE cascade: @io/secrets requires @io/crypto AND @io/fs
at mirror-altitude species-decl as prerequisites. Both were
declared at boot-floor grammar altitude and forward-promised at
family-root altitude but had NOT been lifted to mirror-altitude
species-decl before Landing 4.

- **@io/crypto** — declared at `boot/std/io/crypto.mirror`
  (2026-06-01) with sha1 / sha256 / ed25519_sign / ed25519_verify
  primitives; forward-promised at io.mirror:161-162 verbatim
  ("hash + signature + AEAD over bytes. Vendor surface (sha2,
  age, ssh-key)"). Lift-tick at `shards/io/crypto.mirror`
  (059cf1c) discharges the forward-promise; adds `key_material`,
  `ciphertext_bytes`, `signature_bytes`, `aead_context` carriers
  + `aead_seal` / `aead_open` / `age_encrypt` / `age_decrypt` /
  `ssh_key_material` actions the @io/secrets projection composes
  over.
- **@io/fs** — declared at `boot/std/io.mirror` (2026-06-01) with
  read / write / open / close primitives; forward-promised at
  io.mirror:189-191 + 380 verbatim ("POSIX filesystem surface ...
  lands when `@mirror/store`'s git-backed implementation lifts
  its disk surface"). Lift-tick at `shards/io/fs.mirror` (059cf1c)
  discharges the forward-promise; adds `path`, `file_metadata`,
  `dir_entry` carriers + `fs_read` / `fs_write` / `fs_stat` /
  `fs_list` / `fs_mkdir` actions the @io/secrets materialize
  composes over.

The two lift-ticks land as SIBLING species alongside @io/secrets
because their forward-promises were both consumer-pull-triggered
by the @io/secrets landing per the same substrate discipline
that gave @io/git (2026-06-24) its lift. Per
`[[architecture-lift-as-load-bearing]]`: same primitive at two
altitudes; grammar-altitude declarations stay as transitional
placeholders during consumer migration (same pattern as
`@cli → @mirror/lens/cli` and `@data/json → @mirror/data/json`).

The four-shard @io mint at 059cf1c is one substrate landing: the
peer-key-gated projection species (@io/secrets) + its first
vendor sub-species (@io/secrets/sops) + the two @io lift-tick
species (@io/crypto + @io/fs) all together, because each is the
others' substrate prerequisite and no consumer could pull one
without the other three.

### §0.4 @secrets shape proposal ancestry at 766d930 + fc044ee + 9f5befa

Mara's shape proposal at
`docs/scouts/2026-07-15-mara-secrets-shape-proposal.md` (766d930
original + fc044ee + 9f5befa REED-INLINE cascades) enumerated
four candidates:

- **Candidate 1** — @secrets as family-root (peer to @torus,
  @subject, @kintsugi, @io). Refuted per @onto refusal precedent;
  every load-bearing carrier (peer keys, ciphertext, SOPS, disk
  write) fits @io Glass Wall — family-root @secrets would
  duplicate scope @io family-root already covers.
- **Candidate 2** — @io/secrets as species-under-@io + @io/
  secrets/sops as sub-species. Substrate-honest on three grounds
  (@io Glass Wall verbatim; @onto refusal precedent; Alex SSH-
  signing design intent match). RECOMMENDED.
- **Candidate 3** — @subject/visibility/secrets as species-under-
  @subject/visibility. Refuted because it places @io concerns
  (vendor crypto, disk write, POSIX syscalls) at @subject
  altitude; inverts the composition arrow Alex's design intent
  named (visibility → @io projection, not visibility → visibility
  → @io).
- **Candidate 4** — @sheaf/secrets as sub-species under @sheaf.
  Refuted because it inverts the composition arrow verified at
  sheaf.mirror:149-150: @sheaf composes ONTO @secrets at the
  projection boundary; @secrets is downstream of @sheaf, not a
  sub-species OF @sheaf.

Recommendation was Candidate 2 on three structural grounds
(shape-proposal §5.2 verbatim; reproduced at species-decl 89-121):

> "Ground 1 (substrate-already-had-the-word). io.mirror 37-40
> Glass Wall verbatim declares vendor SDKs + raw bytes + foreign
> blobs under @io. Every load-bearing @secrets element ... fits."

> "Ground 2 (@onto refusal precedent). ... Peer-key + crypto +
> disk-write triple lives at @io/crypto + @io/fs + @io/secrets
> composition; three species-under-@io altitudes, zero family-
> root claims required."

> "Ground 3 (Alex SSH-signing design-intent match at Glass Wall
> altitude). Peer key .git/mirror-side = @io opacity. Crypto =
> @io/crypto vendor surface. Disk write = @io/fs. Fractal.Lens
> pointer = the ref that crosses the @io boundary."

Revised landing sequence (shape-proposal §5.5, after Seam Phase
D-cascade REED-INLINE additions):

1. Landing 1 — shape proposal at 766d930 (Mara).
2. Landing 2 — Seam Phase D-cascade adjudication at 019ad8f.
3. Landing 3 — shape-proposal repairs at fc044ee + 9f5befa
   (three REED-INLINE cascades applied: @io/crypto lift-tick
   added to sequence; SOPS vendor-language corrected Python →
   Go; @mirror/data/yaml+json dependency confirmed).
4. Landing 4 — @io/secrets species-decl at 059cf1c (LANDED).
5. Landing 5 — @io/secrets/sops sub-species + @io/crypto lift-
   tick + @io/fs lift-tick at 059cf1c (LANDED alongside).
6. Landing 6 — @io/secrets.key_material_ref_of composition-
   bridge action at 64b0438 per Seam A1 re-adjudication (LANDED).
7. Landing 7 — this canonical spec (LANDED at commit-in-flight).
8. Landing 8 — Arc-2.3 peer_persistence.rs collapse consumer
   (forward-promise; Reed FLOOR work; Alex-triggered).
9. Landing 9+ — future sub-species enumeration (@io/secrets/age,
   @io/secrets/vault, @io/secrets/1password; forward-promise).

### §0.5 Seam Phase D-cascade @secrets audit at 019ad8f

Seam Phase D-cascade audit at
`docs/audits/2026-07-15-seam-secrets-shape-proposal-phase-d-cascade.md`
(019ad8f) ratified Mara's Candidate 2 recommendation as Mara-
mintable per Scenario A. TL;DR verbatim (lines 34-40):

> "SEAM-RATIFY Candidate 2 (@io/secrets + @io/secrets/sops sub-
> species) as Mara-mintable per Scenario A. Every load-bearing
> element of Alex's SSH-signing design intent — peer keys as raw
> bytes, ciphertext as foreign blob, SOPS as vendor SDK, disk
> write as POSIX syscall — fits @io family-root Glass Wall
> discipline verbatim (io.mirror 37-40). Recommendation lands
> the carrier where the substrate already put the discipline."

Seven dimensions land the cascade:

- **D1 (reframing soundness)** — PASS. sheaf.mirror:140-150
  forward-promise names the arrow @sheaf → @secrets → @io
  explicitly; sheaf.mirror line 145 says "@secrets/sops ... @io
  species that materializes" — soft substrate-lean toward
  Candidate 2.
- **D2 (Candidate 2 substrate-honesty; three grounds)** — PASS
  with vendor-language sharpen (SOPS Python → Go) and micro-
  sharpen ("zero family-root crossings INSIDE @secrets's
  discharge" interior qualifier).
- **D7 (Scenario B collapses to Scenario A via @io/git precedent)**
  — PASS. @io/git lift-tick 2026-06-24 grounds Mara-mint authority
  for concurrent @io/fs + @io/crypto lift-ticks.
- **D8 (SOPS sub-species scope-fit)** — PASS. Sub-species
  specializes secret_projection as sops_file_ref; specializes
  project as sops_encrypt; specializes retrieve as sops_decrypt.
- **D9 (@io/crypto lift-tick prerequisite recognition)** — PASS.
  @io/secrets/sops age-backend composition requires @io/crypto at
  mirror-altitude species-decl; boot-floor grammar cannot compose-
  from-mirror-altitude as species (grammar-vs-species altitude
  type-shape mismatch).
- **D-cross (alignment with @sheaf mint d1ce901)** — PASS. Zero
  altitude mismatches; zero naming collisions.
- **D-length (647 LOC shape-proposal proportional to @sheaf 509
  LOC)** — SHIP-AS-IS.

Three REED-INLINE cascades were required before Mara authored
the species-decls (all applied pre-landing at fc044ee + 9f5befa):

- **REED-INLINE #1** — add @io/crypto lift-tick alongside @io/fs
  in §5.5 landing sequence.
- **REED-INLINE #2** — SOPS vendor-language corrected Python →
  Go (github.com/getsops/sops v3.x+ 2020+ CNCF-maintained).
- **REED-INLINE #3** — @mirror/data/yaml + @mirror/data/json
  dependency verification (both landed 2026-06-07).

Cascade verdict: SHIP. Landing 4 discharges the ratification;
this canonical spec is the composition-only Landing 7 grounding.

### §0.6 Seam composition-alignment audit at cec55a2

Seam composition-alignment audit at
`docs/audits/2026-07-15-seam-sheaf-secrets-composition-alignment.md`
(cec55a2) verified the composition graph between @subject/
visibility/sheaf (d1ce901) and @io/secrets (059cf1c) end-to-end.
Nine dimensions (C1-C9). All PASS-with-drift or PASS. Two REED-
INLINE cascades and one Alex-adjudication residue:

- **REED-INLINE #1** — parameter name drift at secrets.mirror:366
  (`sr` → `section` for clarity; `sr` reads as sheaf_restriction
  per @sheaf's own convention). APPLIED at 57c5b3a.
- **REED-INLINE #2** — sops.mirror missing import of
  `@subject/visibility/sheaf`. APPLIED at 57c5b3a.
- **A1** (Alex-adjudication residue at §12) — narrative-shorthand
  vs substrate-decl'd constructor for `key_material_ref` at
  secrets.mirror:156. Seam refused to pick at Seam altitude;
  deferred to Taut scouting + subsequent Seam re-adjudication.

The audit ratified the composition graph modulo A1; the arrow
chain traverses end-to-end at LANDED altitude (@sheaf.section_at
→ @io/secrets.project → @io/crypto.aead_seal / age_encrypt →
@io/secrets.materialize → @io/fs.fs_write; every arrow chains
byte-equal on type per C1-C9).

### §0.7 Taut precedent scout at 9a5502a + Seam A1 re-adjudication at e5d928e

Taut's grep-first read-only precedent scout at
`docs/scouts/2026-07-15-taut-io-type-as-constructor-precedent.md`
(9a5502a) counted composition-bridge arrows across nine landed
@io species:

- 1 of 14 sites uses narrative-shorthand-constructor
  (secrets.mirror:156 `key_material_ref(peer)`).
- 13 of 14 sites use landed substrate-decl'd constructor OR
  consumer-side record literal.
- 8 of 9 species have ZERO narrative-shorthand; the exception is
  the A1 site under adjudication.

Direct precedents surfaced (per Taut scout §1.7 + §1.9):
- `crypto.ssh_key_material(ssh_key_ref: ref) -> imperfect`
  (crypto.mirror:448) — parametric constructor for `key_material`;
  same altitude as A1 proposal.
- `sops.sops_key_group_from_sheaf_restriction(sr: sheaf_restriction)
  -> imperfect` (sops.mirror:349) — composition-bridge landed as
  substrate-decl at EXACT naming shape (`<carrier>_from_<input>` /
  `<carrier>_of`) A1 Position (b) proposes; SUB-species of the A1
  site itself.

Seam A1 re-adjudication at
`docs/audits/2026-07-15-seam-a1-re-adjudication-with-taut-precedent.md`
(e5d928e) SUPERSEDED the prior "genuinely unresolvable at Seam
altitude" verdict per six dimensions (R1-R6; all PASS):

- **R1 (Taut evidence soundness)** — byte-accurate across three
  spot-checks; aggregate count independently verified.
- **R2 (`craft-not-deliver` scope)** — Taut's body-vs-surface
  split HOLDS by precedent; principle governs bodies, not
  surfaces.
- **R3 (`no-rust-extension-shortcut` preservation)** —
  Position (b) composes over @io/crypto.ssh_key_material +
  record literal; ZERO Rust extension licensed.
- **R4 (precedent-to-decision translation)** — distribution +
  direct sibling + direct sub-species all converge; A1 is an
  isolated singleton, not the substrate's pattern.
- **R5 (two-tick discipline honored)** — Landing 6 IS the second
  tick; deferring further would abandon the discipline.
- **R6 (Alex-verbatim preservation)** — Alex said "the Peers
  key" not "key material ref"; substrate-vocab is Mara/Reed
  lift; landing `_of` creates zero conflict.

Verdict: SEAM-RATIFY Position (b) as Landing 6. Landed at
64b0438 (see §2 for species-decl reference).

### §0.8 @sheaf canonical spec at 564571e (sibling)

Mara's @sheaf canonical spec at
`docs/specs/subject-visibility-sheaf.md` (564571e) is the sibling
structural template this spec follows. Both canonical specs share
the composition-only discipline: reference the species-decl's
carriers/actions/bilaterals without duplication; ground in
ancestry cascade; enumerate composition graph arrow-by-arrow;
name recognition candidates at candidate strength per Mara-B §6.

Cross-composition surface between the two canonical specs:

- **@sheaf UPSTREAM.** `@subject/visibility/sheaf.section_at`
  produces `section_at_stalk`; @io/secrets.project consumes it.
  `@subject/visibility/sheaf.acl_project` produces
  `sheaf_restriction`; @io/secrets/sops.sops_key_group_from_
  sheaf_restriction consumes it.
- **@sheaf spec §3.4** names the arrow chain
  @sheaf → @io/secrets end-to-end; this spec §3.4 names the
  inverse direction (@io/secrets consumer surface).
- **@sheaf spec §6 recognition candidate #3** (composition-
  bridge arrows land as substrate-decl actions) has SECOND
  WITNESS at Landing 6 (key_material_ref_of); this spec §6 lists
  the second-witness candidate as ratified.

Both canonical specs land in the same substrate arc. The two
files compose cleanly: any downstream reader (Arc-2.3
peer_persistence.rs collapse; any future sub-species enumeration)
navigates both to understand the full composition chain from
sheaf-restriction through peer-key-gated projection to disk.

### §0.9 What this spec IS

A composition-only canonical spec grounding the four-shard @io
mint (secrets, secrets/sops, crypto, fs) plus the Landing 6
composition-bridge closure in landed math ancestors, verbatim
substrate-decl citations, and composition graph arrows that
resolve to landed shards. Length target 800-1500 LOC; density
per-section proportional to composition-audit ancestry (§3 math
foundation carries the highest per-line density because sub-
sections cite ancestors by path).

### §0.10 What this spec does NOT do

- Does NOT re-declare the carriers, actions, or bilaterals the
  species-decls at 059cf1c + 57c5b3a + 64b0438 already mint. §2
  references; does not duplicate. Downstream readers open all
  five files (this spec + the four shard-decls).
- Does NOT lift @io/secrets to family-root altitude. Candidate 1
  refuted per Seam D2 verdict.
- Does NOT displace the @io Glass Wall discipline at io.mirror
  37-40; @io/secrets composes UNDER the wall, does not cross it.
- Does NOT displace @io/crypto or @io/fs discipline; both are
  siblings at @io species altitude; @io/secrets composes OVER
  them at the realisation boundary.
- Does NOT lift peer-key material into substrate-decl (`.git/
  mirror`-side per Alex 2026-07-14; opaque per @io opacity).
- Does NOT add new mints beyond what the shard-decls at 059cf1c +
  57c5b3a + 64b0438 already declare.
- Does NOT displace the @sheaf canonical spec at 564571e; both
  land as sibling composition-only specs.
- Does NOT pre-empt Arc-2.3 collapse (Landing 8 forward-promise).

---

## §1 Load-bearing claim

@io/secrets IS the peer-key-gated secrets-projection species at
@io Glass Wall altitude. Four sub-claims land the statement.

### §1.1 Sub-claim 1 — @io/secrets is peer-key-gated projection

Per Alex 2026-07-14 verbatim (§0.1 above), the peer-key stays
`.git/mirror`-side; @secrets and @secrets/sops project through
the Peers key. The species-decl's `secret_projection` carrier
(`shards/io/secrets.mirror:324-329`) makes this substrate-
observable via four-field record:

- `section_ref: ref` — the @subject/visibility/sheaf.
  section_at_stalk carrier the projection was derived from.
  Ref-equality binds the projection to its sheaf-restriction
  context.
- `key_material_ref: key_material_ref` — the peer key gating
  the projection.
- `ciphertext_bytes: ciphertext_bytes` — the AEAD-produced (or
  age-encrypted) payload.
- `projection_path: path` — the disk-side target for
  materialize.

The load-bearing move: the projection is substrate-typed at
mint-time; materialize discharges the disk-write; retrieve
inverts materialize+project. The identity contract is byte-
equality on the four-tuple; two projections with identical
fields ARE the same projection under content-addressing.

Per shape-proposal §2 recommended surface: the projection carrier
IS the substrate-decl form of "project visibility/private stuff
onto disk through the Peers key" — Alex's design-intent phrase
lifted into substrate vocabulary.

### §1.2 Sub-claim 2 — @io/crypto + @io/fs lift as lift-tick companions

The projection composition presupposes AEAD encryption and POSIX
disk write. Both surfaces existed at boot-floor grammar altitude
(2026-06-01) but had NOT lifted to mirror-altitude species-decl
before Landing 4-5. Per shape-proposal §5.5 + Seam D-cascade §D7
+ §D9: the four-shard mint is one substrate landing because
@io/secrets pulls both surfaces at mirror-altitude concurrently.

@io/crypto carries five typed contracts the projection composes
over:
- `key_material` carrier (algorithm + material_ref) — the typed
  boundary form of "a peer's key at the @io opacity discipline."
- `ciphertext_bytes` carrier (nonce + ciphertext + associated_
  data) — the AEAD-produced payload the projection carries.
- `signature_bytes` carrier (algorithm + signature + public_key)
  — parallel signature-side vendor surface.
- `aead_context` carrier (algorithm + key + ad_policy) — the
  cipher configuration a projection uses.
- Actions: `aead_seal` / `aead_open` (AEAD primitives);
  `age_encrypt` / `age_decrypt` (age vendor surface per io.mirror
  161-162 verbatim); `ssh_key_material` (ssh-key vendor material
  loader); `sha1` / `sha256` / `ed25519_sign` / `ed25519_verify`
  (boot-floor lifts).

@io/fs carries three typed contracts + five action primitives
the materialize composes over:
- `path` carrier (segments + is_absolute) — the substrate's
  canonical POSIX pathname carrier.
- `file_metadata` carrier — POSIX stat surface (size + file_type
  + permissions + mtime).
- `dir_entry` carrier — readdir surface (name + file_type).
- Actions: `fs_read` (whole-file byte sequence); `fs_write` (the
  materialize disk-write surface); `fs_stat`; `fs_list`;
  `fs_mkdir`.

Both species retain their boot-floor grammar declarations as
transitional placeholders during consumer migration per
`[[architecture-lift-as-load-bearing]]` (same pattern as
@cli / @data/json). The mirror-altitude species-decls at 059cf1c
name the typed contracts consumers already assumed existed.

### §1.3 Sub-claim 3 — @io/secrets/sops is the first vendor sub-species

Per shape-proposal §2.7 + Seam D-cascade §D8: SOPS is the first
concrete projection method Alex named
(`@secrets prism and @secrets/sops`). Sub-species specializes
`secret_projection` as `sops_file_ref` (four-field record on-
disk-path + wire-format + metadata + ciphertext-bytes);
specializes `project` as `sops_encrypt`; specializes `retrieve`
as `sops_decrypt`; adds the composition-bridge action
`sops_key_group_from_sheaf_restriction` (the arrow that connects
@subject/visibility/sheaf ACL structure to SOPS recipient list).

Additional @io/secrets/sops carriers:
- `sops_key_group` (age_recipients + pgp_recipients +
  kms_recipients) — the SOPS recipient key group.
- `sops_metadata` (key_group + integrity_mac + encryption_
  policy) — the SOPS metadata block appended to every encrypted
  file.

Sub-species composes over four parents:
- @io (family-root discipline).
- @io/secrets (parent species; specializes secret_projection).
- @io/crypto (age vendor surface for age-backend recipients).
- @mirror/data/yaml + @mirror/data/json (SOPS wire format
  parents; both landed 2026-06-07 per Seam D-cascade REED-INLINE
  #3 verification).

Sub-species does NOT re-declare YAML/JSON grammar or vendor
crypto; composes over @mirror/data + @io/crypto per the sub-
species discipline @io/cargo established (declare vendor-tool
contract; runtime opaque-executes).

Future sibling sub-species (forward-promise per §5.9): @io/
secrets/age (age standalone; no SOPS wrapping); @io/secrets/
vault (HashiCorp Vault Transit); @io/secrets/1password (1Password
Connect API). Each sub-species specializes the parent's carrier
for one vendor backend; sub-species enumeration is bounded by
Arc-2.3 evidence per two-tick discipline.

### §1.4 Sub-claim 4 — key_material_ref_of is the substrate-mechanical peer→key bridge

Per Landing 6 (64b0438) landing per Seam A1 re-adjudication
(e5d928e) SEAM-RATIFY Position (b): the peer→key composition-
bridge lands as substrate-decl action, not as narrative-
shorthand-constructor. Species-decl at
`shards/io/secrets.mirror:374`:

```
key_material_ref_of(peer: ref) -> imperfect { \ }
```

Per Seam A1 re-adjudication §3 R3: the realisation body composes
over `@io/crypto.ssh_key_material(ssh_key_ref) -> imperfect
<key_material, ...>` (crypto.mirror:448) + @subject two-witness
resolution (`ssh_signature_fingerprint` field per subject.mirror);
+ record literal `{ peer_ref: peer, key_carrier: key_material }`.
Zero Rust extension licensed per
`[[feedback-no-rust-extension-shortcut]]`. The composition graph
is petri-net-complete AT LANDED altitude for the Arc-2.3 chain
per §3.5.

The Landing 6 landing IS the substrate-mechanical closure of the
@sheaf → @io/secrets → disk composition chain. Prior to Landing
6, `key_material_ref(peer)` appeared at secrets.mirror:156 as
prose narrative-shorthand; Taut scout at 9a5502a counted this
as the ONLY narrative-shorthand instance across 14 sites in
nine @io species. Landing 6 discharges the singleton; the
substrate's pattern (13/14 sites landed as substrate-decl or
consumer record literal) now holds at 14/14.

Per Seam A1 re-adjudication §5 R5: Landing 6 IS the second tick
of the two-tick discipline that shipped Landing 4-5 as the
first tick. The forward-promise on Landing 6 is discharged;
Landing 7 (this canonical spec) grounds the discharge.

---

## §2 Species-decl reference

The four species-decls carry the substrate. This spec names what
they carry; does NOT duplicate.

### §2.1 @io/secrets species-decl (059cf1c + 57c5b3a + 64b0438; 573 LOC)

File: `shards/io/secrets.mirror`.

#### §2.1.1 Prism declaration (species-decl 221-227)

`prism @io/secrets { focus/project/split/shift/settle
secret_projection }`.

Five-op prism per `[[architecture-prism-as-trait-as-everything]]`:
every prism-op consumes and produces `secret_projection` at the
altitude the species mounts.

#### §2.1.2 Carriers (species-decl 258-329)

Three typed records:

- `key_material_ref { peer_ref, key_carrier }` — the Fractal.Lens
  POINTER for a peer's key material at .git/mirror-side boundary
  (per Alex 2026-07-14 verbatim). Identity contract: byte-equality
  on (peer_ref, key_carrier).
- `ciphertext_ref { ciphertext, on_disk_path, recipient_keys }` —
  typed reference for on-disk encrypted content; opaque foreign
  blob per @io Glass Wall. Identity contract: byte-equality on
  (ciphertext, on_disk_path, recipient_keys).
- `secret_projection { section_ref, key_material_ref, ciphertext_
  bytes, projection_path }` — THE LOAD-BEARING carrier at
  @io/secrets altitude; names the peer-key-gated projection of a
  @sheaf section onto disk. Identity contract: byte-equality on
  the four-tuple.

All carriers land as record types with `ref`-equality on field
values; no bare types per `[[feedback-no-bare-types]]`.

#### §2.1.3 Actions (species-decl 374-466)

Five actions, all bodies `\`-obligation-blocked per
`[[feedback-craft-not-deliver]]`:

- `key_material_ref_of(peer: ref) -> imperfect` — Landing 6
  composition-bridge action; resolves peer → key_material_ref via
  @io/crypto.ssh_key_material + record literal.
- `project(section: section_at_stalk, k: key_material_ref) ->
  imperfect` — LOAD-BEARING projection action; composes AEAD
  seal over section's projected_value under peer's key. Parameter
  renamed sr → section per composition-alignment REED-INLINE #1
  at 57c5b3a.
- `materialize(sp: secret_projection) -> imperfect` — LOAD-
  BEARING disk-write action; composes @io/fs.fs_write over
  sp.ciphertext_bytes.ciphertext + sp.projection_path.
- `retrieve(cr: ciphertext_ref, k: key_material_ref) ->
  imperfect` — inverse of materialize + project; composes
  @io/fs.fs_read + @io/crypto.aead_open (or age_decrypt).
- `round_trip(section: section_at_stalk, k: key_material_ref) ->
  imperfect` — discipline action; project + materialize +
  retrieve; verifies recovered plaintext byte-equals original.

#### §2.1.4 Bilaterals (species-decl 496-559)

Four verdict-returning predicates, all Rice-safe at whole-tick
altitude per Mara-B §4.5.5:

- `key_admits(k: key_material_ref, sr: sheaf_restriction) ->
  verdict` — composes @subject two-witness discipline +
  @io/crypto key_material_admissible + peer_ref byte-equality.
- `projection_valid(sp: secret_projection) -> verdict` —
  composes section_computable + key_admits + AEAD verifiability.
- `round_trip_preserved(section, k) -> verdict` — verifies
  project ∘ materialize ∘ retrieve returns byte-equal plaintext.
- `secrets_well_formed(sp: secret_projection) -> verdict
  requires projection_valid(sp) requires fs_well_formed(sp.
  projection_path)` — LOAD-BEARING composed bilateral; sibling
  to @io/oci.oci_well_formed, @io/git.git_well_formed,
  @io/algebra.io_algebra_well_formed, @io/crypto.crypto_well_
  formed, @io/fs.fs_well_formed per Seam tick 68 C4/C9 closure.

#### §2.1.5 Substrate decisions (species-decl 191-213)

The species-decl marks nine landed substrate-decisions:

- `[[architecture-shards-as-substrate-source]]`
- `[[architecture-prism-as-trait-as-everything]]`
- `[[architecture-glass-wall-substrate-types]]` — transparency
  inheritance through @io.
- `[[architecture-fractal-lens-pointer-thing-partition]]` —
  peer_key is the POINTER; secret_projection after materialize
  is the THING.
- `[[architecture-form-process-partition-at-family-root]]` —
  species stays under @io; peer-key-gated projection is
  boundary-side, not substrate-form-side.
- `[[feedback-substrate-already-had-the-word]]` — ~60th
  instance; sheaf.mirror 140-150 already located the altitude.
- `[[feedback-onto-family-root-is-the-ladder-Foerster-refused]]`
  — parallel refusal for @secrets-family-root; @io already
  carries the altitude.
- `[[feedback-no-bare-types]]` — secret_projection is typed
  record with ref-equality identity.
- `[[feedback-craft-not-deliver]]` — all action bodies `\`-
  obligation-blocked.
- `[[feedback-no-rust-extension-shortcut]]` — composition-only
  over @io/crypto + @io/fs; NO Rust extension.

#### §2.1.6 Path-namespace property (species-decl 215-217)

Per `@epistemologic/pact/path_matches_namespace`: the file at
`shards/io/secrets.mirror` declares `@io/secrets`. Namespace-
path parity is substrate-mechanical.

#### §2.1.7 Imports (species-decl 1-8) + exports (species-decl 561-573)

Imports:
- `@prism`, `@meta`, `@glass` — canonical species prefix per
  Landing 4 R2 discipline.
- `@io` — family-root.
- `@io/crypto` — sibling species; provides key_material +
  aead + age + ssh_key_material.
- `@io/fs` — sibling species; provides path + fs_write + fs_read.
- `@subject/visibility/sheaf` — upstream producer; provides
  sheaf_restriction + section_at_stalk (verified at composition-
  alignment audit C4).
- `@mirror/data` — content-format family-root; parent of the
  wire-format species @io/secrets/<sub> sub-species compose over.

Exports:
- Namespace: `@io/secrets`.
- Carriers: `key_material_ref`, `ciphertext_ref`,
  `secret_projection`.
- Actions: `key_material_ref_of`, `project`, `materialize`,
  `retrieve`, `round_trip`.
- Bilaterals: `key_admits`, `projection_valid`,
  `round_trip_preserved`, `secrets_well_formed`.

### §2.2 @io/secrets/sops sub-species-decl (059cf1c + 57c5b3a; 458 LOC)

File: `shards/io/secrets/sops.mirror`.

#### §2.2.1 Prism declaration (species-decl 160-166)

`prism @io/secrets/sops { focus/project/split/shift/settle
sops_file_ref }`.

Five-op prism per the same discipline as parent; sub-species
specializes the carrier from `secret_projection` to
`sops_file_ref` at the vendor-tool altitude.

#### §2.2.2 Carriers (species-decl 199-264)

Three typed records specializing parent + adding vendor-specific
structure:

- `sops_file_ref { on_disk_path, wire_format, metadata,
  ciphertext_bytes }` — typed reference for on-disk SOPS-encrypted
  YAML/JSON file. Opaque foreign blob per @io Glass Wall.
  Identity contract: byte-equality on the four-tuple.
- `sops_key_group { age_recipients, pgp_recipients,
  kms_recipients }` — SOPS recipient key group; each recipient
  can independently decrypt. Combined non-empty per SOPS
  discipline.
- `sops_metadata { key_group, integrity_mac, encryption_policy }`
  — SOPS metadata block appended to every encrypted file.

#### §2.2.3 Actions (species-decl 297-362)

Four actions, all bodies `\`-obligation-blocked:

- `sops_encrypt(section: ref, key_group: sops_key_group) ->
  imperfect` — LOAD-BEARING SOPS-encrypt action; runs SOPS
  encryption pipeline (per-leaf age/pgp/kms encryption +
  integrity MAC + metadata assembly).
- `sops_decrypt(f: sops_file_ref, k: key_material) -> imperfect`
  — inverse of sops_encrypt; runs SOPS decryption pipeline.
- `sops_key_group_from_sheaf_restriction(sr: sheaf_restriction)
  -> imperfect` — THE composition-bridge action; connects
  @subject/visibility/sheaf ACL structure to SOPS recipient
  list. Landing precedent for A1 Position (b) per Taut scout at
  9a5502a; this arrow LANDED first, motivating the parallel
  landing of @io/secrets.key_material_ref_of at Landing 6.
- `sops_round_trip(section, key_group, k) -> imperfect` —
  discipline action per @io/secrets.round_trip pattern.

#### §2.2.4 Bilaterals (species-decl 387-445)

Four verdict-returning predicates, all Rice-safe at whole-tick
altitude:

- `sops_key_group_admissible(kg: sops_key_group) -> verdict` —
  combined recipient set non-empty; every recipient well-formed
  per its backend.
- `sops_metadata_valid(m: sops_metadata) -> verdict` — key_group
  admissible; integrity_mac well-formed; encryption_policy
  regex pair well-formed.
- `sops_round_trip_preserved(section, kg, k) -> verdict` —
  verifies sops_encrypt ∘ sops_decrypt returns byte-equal
  plaintext.
- `sops_well_formed(f: sops_file_ref) -> verdict requires
  sops_metadata_valid(f.metadata)` — LOAD-BEARING composed
  bilateral at @io/secrets/sops altitude; sibling to
  @io/secrets.secrets_well_formed and the other @io species
  well-formed composed bilaterals.

#### §2.2.5 Substrate decisions + path-namespace (species-decl 131-154)

Six landed substrate-decisions parallel to parent, plus:

- `[[architecture-lift-as-load-bearing]]` — SOPS lifts as sub-
  species under @io/secrets per Alex 2026-07-14 verbatim.

Path-namespace: `shards/io/secrets/sops.mirror` declares
`@io/secrets/sops`.

#### §2.2.6 Imports (species-decl 1-9) + exports (species-decl 447-458)

Imports:
- `@prism`, `@meta`, `@glass` — canonical prefix.
- `@io`, `@io/secrets` — parents.
- `@io/crypto` — age vendor surface.
- `@subject/visibility/sheaf` — sheaf_restriction consumer
  (REED-INLINE #2 cascade applied at 57c5b3a).
- `@mirror/data/yaml`, `@mirror/data/json` — wire format
  parents.

Exports: namespace + three carriers + four actions + four
bilaterals per §§2.2.2-2.2.4.

### §2.3 @io/crypto species-decl (059cf1c; 553 LOC)

File: `shards/io/crypto.mirror`. Lifted from `boot/std/io/
crypto.mirror` (2026-06-01) per `[[architecture-lift-as-load-
bearing]]`. Boot-floor decl retained as transitional placeholder.

Prism: `prism @io/crypto { focus/project/split/shift/settle
crypto }` (species-decl 176-182).

Carriers (species-decl 207-289):
- `key_material { algorithm, material_ref }`
- `ciphertext_bytes { nonce, ciphertext, associated_data }`
- `signature_bytes { algorithm, signature, public_key }`
- `aead_context { algorithm, key, ad_policy }`

Actions (species-decl 303-448):
- `sha1` / `sha256` / `ed25519_sign` / `ed25519_verify` (lifted
  from boot-floor).
- `aead_seal` / `aead_open` (AEAD primitives).
- `age_encrypt` / `age_decrypt` (age vendor surface).
- `ssh_key_material` (ssh-key vendor material loader; parametric
  constructor for `key_material` from `ssh_key_ref`; Taut
  precedent for A1 Position (b) per scout §1.7).

Bilaterals (species-decl 474-534):
- `key_material_admissible(km: key_material) -> verdict`
- `recipients_admissible(recipients: ref) -> verdict`
- `signature_valid(sig: signature_bytes, msg: ref) -> verdict`
- `crypto_well_formed(ctx: aead_context) -> verdict requires
  key_material_admissible(ctx.key)`

Cross-wall-kintsugi candidacy per glass-wall insight (2026-05-26):
- PERMANENT @io: sha1 (git interop), ed25519 primitives (field
  arithmetic), age_encrypt/age_decrypt (vendor by construction),
  aead_seal/aead_open (constant-time platform intrinsics).
- CROSS-WALL-KINTSUGI CANDIDATE: sha256 (byte-level pure;
  structurally-verified Rust impl imaginable; fold deferred).

### §2.4 @io/fs species-decl (059cf1c; 421 LOC)

File: `shards/io/fs.mirror`. Lifted from forward-promise at
`shards/io.mirror:189-191 + 380` per `[[architecture-lift-as-
load-bearing]]`; parallel to @io/git lift-tick precedent
2026-06-24.

Prism: `prism @io/fs { focus/project/split/shift/settle fs }`
(species-decl 148-154).

Carriers (species-decl 179-231):
- `path { segments, is_absolute }` — POSIX pathname.
- `file_metadata { size, file_type, permissions, mtime }`
  — POSIX stat surface.
- `dir_entry { name, file_type }` — readdir surface.

Actions (species-decl 257-332):
- `fs_read(p: path) -> imperfect` — whole-file read.
- `fs_write(p: path, bytes: ref) -> imperfect` — LOAD-BEARING
  disk-write; the @io/secrets.materialize surface consumer.
- `fs_stat(p: path) -> imperfect` — POSIX stat.
- `fs_list(p: path) -> imperfect` — POSIX readdir.
- `fs_mkdir(p: path) -> imperfect` — single-directory create.

Bilaterals (species-decl 354-407):
- `path_admissible(p: path) -> verdict` — POSIX pathname
  discipline (PATH_MAX, no NUL, NAME_MAX).
- `path_exists(p: path) -> verdict` — POSIX access(F_OK).
- `writable(p: path) -> verdict` — POSIX access(W_OK).
- `fs_well_formed(p: path) -> verdict requires
  path_admissible(p)` — composed bilateral.

Cross-wall-kintsugi candidacy: NONE. POSIX filesystem primitives
are PERMANENT @io per glass-wall insight — kernel-managed file
descriptors, blocking reads, filesystem-metadata caching are
structurally non-foldable into mirror altitude.

---

## §3 Math foundation

Five sub-sections. Each cites ancestors by path; each grounds a
mathematical claim the species-decls' carriers/actions/bilaterals
already presuppose.

### §3.1 Glass Wall discipline

Per `shards/io.mirror:37-40` verbatim (reproduced §0.2 above):

> "@io is the substrate's only legitimate non-mirror surface. Any
> grammar that isn't mirror — Rust, Python, raw bytes, foreign
> blobs, vendor SDKs — must be under @io."

Four load-bearing consequences for the mint:

- **SOPS as vendor SDK is @io by construction.** Every SOPS
  invocation is a subprocess execution against the sops Go
  binary; the substrate cannot fold past the process boundary.
  @io/secrets/sops declares the typed contract (sops_encrypt,
  sops_decrypt); the sops binary discharges the contract at the
  @io realisation boundary.
- **Peer keys as raw bytes are @io by construction.** ssh-ed25519
  private material, X25519 age keys, PGP secret keys — all
  vendor-crypto byte payloads. The substrate carries the
  key_material_ref (algorithm + material_ref); the bytes stay
  out-of-substrate per `.git/mirror`-side storage discipline.
- **Ciphertext files as foreign blobs are @io by construction.**
  AEAD-encrypted payloads and SOPS-format files are opaque to
  mirror substrate; the substrate carries ciphertext_bytes
  (nonce + ciphertext + associated_data) and sops_file_ref
  (path + format + metadata + ciphertext_bytes) as opaque refs.
- **Disk write as POSIX syscall is @io by construction.** open +
  write + close is a kernel-managed file descriptor boundary;
  the vfs layer is non-foldable per glass-wall insight (2026-05-
  26). @io/fs.fs_write declares the typed contract; the kernel
  discharges it.

The Glass Wall discipline is the substrate-decl form of Bateson's
1970 form/substance partition (per io.mirror:279-298 Cybernetic
Ancestor block): five form-side family roots (@code, @mirror/
lens, @mirror/spectral, @mirror/loss, @mirror/data) + one
substance-side root (@io). The @io/secrets species stays UNDER
the substance-side wall by design intent.

Per shape-proposal §5.2 Reason 1 + Seam D-cascade §D2.a fit
check: mounting @secrets ANYWHERE but @io (Candidate 1 family-
root; Candidate 3 @subject/visibility/secrets; Candidate 4 @sheaf/
secrets) would duplicate scope @io family-root already covers,
per the @onto refusal precedent. Landing 4 mounts at Candidate 2;
the four-shard mint (secrets + sops + crypto + fs) all lives
under @io per the wall.

### §3.2 Peer-key-gated projection

Per Alex 2026-07-14 verbatim (§0.1 above):

> "Each peer has their own key in the private part of their
> visibility. NOT projected into the git state and instead stays
> .git/mirror side. Only connected through Fractal.Lens. A
> pointer. Not the thing."

Four structural corollaries the identification carries:

- **Peer-key material lives on non-mirror-world side.** The
  substrate never carries key bytes in-tree. The
  key_material_ref carrier (species-decl 258-261) IS the
  Fractal.Lens POINTER per
  `[[architecture-fractal-lens-pointer-thing-partition]]`. The
  key BYTES resolve at the @io realisation boundary through
  out-of-substrate `.git/mirror`-side storage.
- **Fractal.Lens pointer discipline.** The key_material carrier
  under @io/crypto (species-decl 207-210) carries `algorithm +
  material_ref` — the material_ref is the pointer, not the
  bytes. Loading resolves through @io/crypto.ssh_key_material at
  the realisation boundary; the substrate never materializes the
  bytes as substrate-decl carrier.
- **Peer-key gating is per-projection.** The secret_projection
  carrier (species-decl 324-329) has a `key_material_ref` field;
  a projection is IDENTIFIED by the key that gated it. Two
  projections of the same section under different peer keys are
  DISTINCT under content-addressing. This encodes the "each peer
  has their own key" claim at substrate identity level.
- **Two-witness discipline transitively discharges.** The
  peer_ref field of key_material_ref resolves to a
  subject_instance whose two-witness_verification must Pass for
  the key_material_ref to be admissible (per key_admits
  bilateral at species-decl 496). The @subject discipline is
  inherited transitively; the @io/secrets species does not
  duplicate the witness check.

The species-decl's key_material_ref_of action (Landing 6 at
species-decl 374) discharges the pointer resolution
substrate-mechanically: given a peer_ref, construct the
key_material_ref carrier by resolving the peer's ssh key
material through @io/git plumbing (`.git/mirror`-side storage)
+ @io/crypto.ssh_key_material (key material loading + typing).
The realisation body composes over @io/crypto.ssh_key_material
+ record literal per Seam A1 re-adjudication §3 R3; zero Rust
extension licensed.

### §3.3 AEAD Rice-safety

Per @io/crypto discipline + Mara-B §4.5.5 Rice-safety template
at whole-tick altitude, the AEAD-projection actions are Rice-
safe:

- **Bound.** `aead_seal(ctx, plaintext)` reads byte-visible
  aead_context state (algorithm string, key algorithm, ad_policy
  ref); bounded by the plaintext byte length + AEAD block cost
  (constant-time platform intrinsics per glass-wall insight);
  no program semantics inspection required.
- **Bilateral discharge.** `crypto_well_formed(ctx)` Pass iff
  key_material_admissible(ctx.key) AND algorithm-appropriate
  parameter shape (algorithm-specific nonce length, tag length,
  key length agreement). All sub-checks read byte-visible state;
  all Rice-safe at whole-tick altitude.
- **AEAD verifiability discipline.** Per @io/crypto.aead_open
  docstring: aead_open MUST refuse to return plaintext when tag
  verification fails. A Narcissus-pole implementation that
  returns unauthenticated plaintext violates the AEAD contract.
  This is the Splinter/Narcissus discipline at AEAD altitude.
- **Byte-compare round-trip.** `round_trip_preserved(section, k)`
  reads byte-visible plaintext state before + after round-trip;
  equality check is bounded byte-compare. Rice-safe at whole-
  tick altitude.

The AEAD primitives are PERMANENT @io per glass-wall partition
(constant-time platform intrinsics; vendor crypto by
construction). The Rice-safety at whole-tick altitude does NOT
license cross-wall-kintsugi at AEAD altitude; the primitives
stay in @io forever. The @io/secrets.projection_valid bilateral
composes over crypto_well_formed transitively per §3 R3 of the
composition graph.

### §3.4 Composition with @subject/visibility/sheaf

Per @sheaf canonical spec at
`docs/specs/subject-visibility-sheaf.md` (564571e) §3.4 — the
arrow chain @sheaf.section_at → @io/secrets.project traverses
end-to-end at LANDED altitude. Reproduced from Seam composition-
alignment audit at cec55a2 C1-C7:

- `acl_project(F_home: ref, peer_acl: ref) -> sheaf_restriction`
  (sheaf.mirror:358) — TYPE-CHECKS at ACL_peer boundary.
- `section_at(F_A_p: sheaf_restriction, crystal_ref: ref) ->
  section_at_stalk` (sheaf.mirror:382) — TYPE-CHECKS on sr flow.
- `key_material_ref_of(peer: ref) -> imperfect`
  (secrets.mirror:374; Landing 6 at 64b0438) — TYPE-CHECKS on
  peer flow.
- `project(section: section_at_stalk, k: key_material_ref) ->
  imperfect` (secrets.mirror:407; parameter renamed sr → section
  per REED-INLINE #1 at 57c5b3a) — TYPE-CHECKS on section flow
  BYTE-EQUAL to section_at output.
- `materialize(sp: secret_projection) -> imperfect`
  (secrets.mirror:430) — TYPE-CHECKS on secret_projection
  carrier.
- Composition to `@io/fs.fs_write(p: path, bytes: ref) ->
  imperfect` (fs.mirror:280) — TYPE-CHECKS on projection_path +
  ciphertext_bytes carriers.

The composition arrow @sheaf → @io/secrets is ONE-DIRECTIONAL:
@sheaf produces section_at_stalk; @io/secrets consumes it. The
reverse arrow (@io/secrets consuming ACL structure) is
discharged at @io/secrets/sops via
`sops_key_group_from_sheaf_restriction(sr: sheaf_restriction) ->
imperfect` (sops.mirror:349) — the sub-species carries the
composition-bridge action; parent species @io/secrets consumes
@sheaf's exports directly.

Per @sheaf canonical spec §3.4 verbatim (cross-cited here for
composition symmetry):

> "The composition arrow @sheaf → @io/secrets is ONE-
> DIRECTIONAL: @sheaf produces `section_at_stalk`; @io/secrets
> consumes it."

The full end-to-end Arc-2.3 composition chain per shape-
proposal §2.4 (corrected for interior-chain qualifier at Seam
D-cascade §D2.a; reproduced verbatim at secrets.mirror:149-162):

```
peer_visibility_materialize(peer, home, crystal, target_path):
  ACL_peer := pack.members[peer.name]                           # @mirror/pack
  sr       := @subject/visibility/sheaf.acl_project(F_home, ACL_peer)
                                                                # d1ce901
  section  := @subject/visibility/sheaf.section_at(sr, crystal)
                                                                # d1ce901
  peer_key := @io/secrets.key_material_ref_of(peer)             # Landing 6 at 64b0438
  sp       := @io/secrets.project(section, peer_key)            # 059cf1c
  @io/secrets.materialize(sp, target_path)                      # 059cf1c → disk
```

Every arrow resolves to landed substrate at species-decl
altitude. Type signatures chain byte-equal per composition-
alignment audit C1-C7. The composition graph is petri-net-
complete AT LANDED altitude for the Arc-2.3 chain.

Fractal.Lens pointer discipline preserved: peer_key is a REF
(per @gift/lens shift pattern); dereferences at @io boundary;
substrate never carries key bytes in-tree.

### §3.5 Round-trip discipline

Per shape-proposal §2 recommended surface: the round_trip action
IS the substrate-decl form of "encrypt-then-decrypt returns same
projected_value." The round-trip discipline structurally
distinguishes Splinter-pole projection (honest round-trip) from
Narcissus-pole (claimed projection that loses bytes).

Concretely at @io/secrets altitude:

- `round_trip(section, k)` composes project + materialize +
  retrieve; verifies the recovered plaintext byte-equals the
  original section's projected_value.
- `round_trip_preserved(section, k) -> verdict` is the bilateral
  witness; Rice-safe at whole-tick altitude per §3.3.

Per @io/secrets/sops altitude:

- `sops_round_trip(section, key_group, k)` composes sops_encrypt
  + sops_decrypt; verifies byte-equal plaintext.
- `sops_round_trip_preserved` is the bilateral witness.

The round-trip discipline is PARALLEL to @sheaf.section_at ∘
inverse. Per @sheaf canonical spec §3.3 pending-boundary
handling: a section over a non-admitted stalk is NOT computable
at substrate altitude and returns pending-boundary per
@kintsugi/consent Partial semantics. Similarly at @io/secrets:
a projection over a key that does not admit the section
(key_admits Fail) returns Fail; a projection over a section
that is not computable (section_computable Fail) returns Fail
transitively via projection_valid.

Splinter-pole across the discipline chain: bytes round-trip
cleanly; the section's projected_value survives project +
materialize + retrieve byte-equal.

Narcissus-pole across the discipline chain: bytes altered by
encoding drift, compression, key mismatch masked at decrypt,
disk truncation, YAML/JSON key reordering not preserved by
SOPS. Alignment-as-boundary-mathematics (#57) distinguishes
which claim broke via the transparency<p> residual on the
imperfect return.

Load-bearing structural claim: round_trip_preserved Pass IS the
substrate-decl form of "the projection is substrate-honest."
A projection that does NOT round-trip cleanly is substrate-decl
refusal of the projection's soundness claim.

---

## §4 Composition graph

### §4.1 Upstream (parents)

- **@io family-root.** The boundary-with-non-mirror discipline;
  five-op prism + opacity + imperfect return shape. Every @io
  species inherits the discipline; @io/secrets is one of nine
  landed species (cargo, stagefreight, oci, git, algebra,
  crypto, fs, secrets, secrets/sops per Landing 4-5) at @io
  altitude. Per `shards/io.mirror:37-40` Glass Wall verbatim
  (§0.2 + §3.1).

- **@subject/visibility/sheaf.** The ACL-restriction algebra at
  @subject/visibility species altitude (landed d1ce901;
  canonical spec at 564571e). Produces
  `section_at_stalk` via `section_at` action; @io/secrets
  imports and consumes at species-decl line 7. Two-witness
  discipline transitively discharges through peer_ref.

- **@mirror/data.** Content-format family-root (2026-06-07);
  parent of the wire-format species (@mirror/data/yaml + @mirror/
  data/json) that @io/secrets/<sub> sub-species compose over.
  Imported at @io/secrets species-decl line 8.

### §4.2 Peer (sibling @io species)

- **@io/crypto.** Sibling species; lifted 2026-07-15 as lift-
  tick companion (059cf1c). Provides `key_material`,
  `ciphertext_bytes`, `signature_bytes`, `aead_context` carriers
  + AEAD/sig/age/ssh_key_material actions the @io/secrets
  projection composes over at realisation boundary. Imported at
  @io/secrets species-decl line 5.

- **@io/fs.** Sibling species; lifted 2026-07-15 as lift-tick
  companion (059cf1c). Provides `path` + POSIX read/write/stat/
  list/mkdir actions the @io/secrets materialize composes over at
  realisation boundary. Imported at @io/secrets species-decl line
  6.

- **Other @io siblings (indirect composition surface).** @io/git
  (2026-06-24; peer-key `.git/mirror`-side storage plumbing);
  @io/cargo, @io/stagefreight, @io/oci, @io/algebra (parallel
  species; discipline peers, no direct composition arrow to
  @io/secrets).

### §4.3 Downstream (consumers)

- **@io/secrets/sops.** Sub-species; landed alongside as Landing
  5 (059cf1c + 57c5b3a). Specializes secret_projection as
  sops_file_ref; specializes project as sops_encrypt; specializes
  retrieve as sops_decrypt; adds composition-bridge action
  sops_key_group_from_sheaf_restriction. Imports @io/secrets +
  @io/crypto + @mirror/data/yaml + @mirror/data/json.

- **bootstrap/src/peer_persistence.rs (Arc-2.3 collapse target;
  Landing 8 forward-promise).** Currently Rust FLOOR; collapse
  target composes @sheaf.acl_project + @sheaf.section_at +
  @io/secrets.key_material_ref_of + @io/secrets.project +
  @io/secrets.materialize + @kintsugi/consent.query_phi. Alex-
  triggered Reed FLOOR work; awaits composition-only shard body
  landing after Landing 7.

- **Future sub-species (Landing 9+ forward-promise).**
  @io/secrets/age (age standalone; no SOPS wrapping);
  @io/secrets/vault (HashiCorp Vault Transit; KMS backend);
  @io/secrets/1password (1Password Connect API). Each sub-species
  specializes @io/secrets carrier for one vendor backend per the
  sub-species discipline @io/cargo established.

### §4.4 Cross-composition

- **@subject two-witness discipline transitively discharges.**
  Peer_ref field of key_material_ref resolves to a
  subject_instance whose ssh_witness_valid +
  spectral_witness_valid must Pass for key_material_ref to be
  admissible. key_admits bilateral (secrets.mirror:496) composes
  over @subject two-witness at realisation boundary.

- **@mirror/pack ACL surface composes upstream via @sheaf.**
  Pack.members[peer] → ACL binding altitude (per mirror-spec-
  peer-acl-surface.md §5); @sheaf.acl_project consumes ACL_peer
  to produce sheaf_restriction; @io/secrets consumes
  section_at_stalk (derived from sheaf_restriction) via project.
  The composition arrow @mirror/pack → @sheaf → @io/secrets is
  transitive.

- **@kintsugi/consent.query_phi discharges elevation.** Per
  @sheaf canonical spec §4.4: elevation-of-visibility is a
  sheaf-morphism widening A_p → A_p'; requires
  @kintsugi/consent.query_phi discharge. If a projection targets
  a widened restriction (elevation), the composition chain
  discharges consent at @sheaf altitude BEFORE the projection
  reaches @io/secrets. @io/secrets does NOT directly compose
  over @kintsugi/consent.

- **@fragmentation content-addressing via @io/crypto.sha256.**
  Per @io/crypto docstring: sha256 is the substrate's structural
  content-addressing function; consumed by @fragmentation via
  future re-route (currently boot-floor). The @io/secrets
  species does NOT directly compose over @fragmentation.

- **@gift/lens Fractal.Lens pointer/thing partition.**
  Orthogonal WHO-composed carrier; the peer_ref field of
  key_material_ref IS the Fractal.Lens POINTER; the
  secret_projection after materialize IS the THING. Per
  `[[architecture-fractal-lens-pointer-thing-partition]]`.

### §4.5 Composition graph — arrow-by-arrow resolution

Every arrow resolves to a landed shard at species-decl altitude.
Enumerated for Rice-safe petri-net analysis:

| Producer | Arrow | Consumer | Landed at |
|---|---|---|---|
| @subject | subject_instance | @io/secrets.key_material_ref.peer_ref | shards/subject.mirror |
| @subject | subject_instance | @io/secrets.key_material_ref_of (input) | shards/io/secrets.mirror:374 |
| @io/crypto | ssh_key_material | @io/secrets.key_material_ref_of (composition) | shards/io/crypto.mirror:448 |
| @io/crypto | key_material | @io/secrets.key_material_ref.key_carrier | shards/io/secrets.mirror:258-261 |
| @io/crypto | ciphertext_bytes | @io/secrets.secret_projection.ciphertext_bytes | shards/io/secrets.mirror:324-329 |
| @io/fs | path | @io/secrets.secret_projection.projection_path | shards/io/secrets.mirror:324-329 |
| @sheaf | section_at_stalk | @io/secrets.project (input) | shards/io/secrets.mirror:407 |
| @sheaf | sheaf_restriction | @io/secrets/sops.sops_key_group_from_sheaf_restriction | shards/io/secrets/sops.mirror:349 |
| @io/secrets | secret_projection | @io/secrets.materialize | shards/io/secrets.mirror:430 |
| @io/secrets | materialize (output) | @io/fs.fs_write | shards/io/fs.mirror:280 |
| @io/secrets | project (output) | @io/crypto.aead_seal or age_encrypt | shards/io/crypto.mirror:362 / 408 |
| @io/secrets | retrieve (input) | @io/fs.fs_read | shards/io/fs.mirror:257 |
| @io/secrets | retrieve (composition) | @io/crypto.aead_open or age_decrypt | shards/io/crypto.mirror:386 / 428 |
| @io/secrets/sops | sops_file_ref | @io/secrets/sops.sops_decrypt | shards/io/secrets/sops.mirror:318 |
| @io/secrets/sops | sops_encrypt (output) | @io/crypto.age_encrypt (age backend) | shards/io/crypto.mirror:408 |
| @io/secrets/sops | sops_encrypt (output) | @io/fs.fs_write (via wrapping) | shards/io/fs.mirror:280 |
| @mirror/data/yaml | YAML wire format | @io/secrets/sops.sops_file_ref.wire_format | shards/io/secrets/sops.mirror:199-204 |
| @mirror/data/json | JSON wire format | @io/secrets/sops.sops_file_ref.wire_format | shards/io/secrets/sops.mirror:199-204 |

Every producer→consumer arrow chains byte-equal at type. Rice-
safe petri-net completeness holds; Landing 6 discharged the
final composition-bridge arrow at 64b0438 (@io/secrets.key_
material_ref_of).

Composition-symmetry check with @sheaf canonical spec §4.5:
- @sheaf spec §4.5 row 5 "@sheaf section_at_stalk → @io/secrets.
  project → shards/io/secrets.mirror:366" cross-references this
  spec §4.5 row 7 (updated line-number 407 post-REED-INLINE #1
  at 57c5b3a).
- @sheaf spec §4.5 row 6 "@sheaf sheaf_restriction → @io/
  secrets/sops.sops_key_group_from_sheaf_restriction → shards/
  io/secrets/sops.mirror:349" cross-references this spec §4.5
  row 8.
- Both cross-references chain byte-equal; composition graph
  closed at landed altitude across both canonical specs.

---

## §5 Landing forward-promises

### §5.1 Landing 1 — LANDED at 766d930 (Mara @secrets shape-proposal)

Mara shape proposal at `docs/scouts/2026-07-15-mara-secrets-
shape-proposal.md`. Four candidates + Candidate 2 recommendation
on three structural grounds. 647 LOC.

### §5.2 Landing 2 — LANDED at 019ad8f (Seam Phase D-cascade adjudication)

Seam Phase D-cascade audit at `docs/audits/2026-07-15-seam-
secrets-shape-proposal-phase-d-cascade.md`. Seven-dimension
adjudication (D1-D9 + D-cross + D-length). SHIP verdict; SEAM-
RATIFY Candidate 2 as Mara-mintable per Scenario A.

### §5.3 Landing 3 — LANDED at fc044ee + 9f5befa (shape-proposal repairs)

Three REED-INLINE cascades applied to shape proposal:
- @io/crypto lift-tick added to §5.5 landing sequence.
- SOPS vendor-language corrected Python → Go.
- @mirror/data/yaml + @mirror/data/json dependency verification.

### §5.4 Landing 4 — LANDED at 059cf1c (@io/secrets species-decl)

Species-decl mint at `shards/io/secrets.mirror`. Prism +
carriers + actions + bilaterals + substrate-decisions +
exports. All per §2.1 above.

### §5.5 Landing 5 — LANDED at 059cf1c (@io/secrets/sops + @io/crypto + @io/fs)

Three concurrent species-decl mints:
- `shards/io/secrets/sops.mirror` — SOPS sub-species (§2.2).
- `shards/io/crypto.mirror` — @io/crypto lift-tick (§2.3).
- `shards/io/fs.mirror` — @io/fs lift-tick (§2.4).

The four-shard @io mint at 059cf1c is one substrate landing per
shape-proposal §5.5 revised landing sequence.

### §5.6 Landing 6 — LANDED at 64b0438 (key_material_ref_of composition-bridge)

Composition-bridge action at `shards/io/secrets.mirror:374`:

```
key_material_ref_of(peer: ref) -> imperfect { \ }
```

Landed per Seam A1 re-adjudication SEAM-RATIFY Position (b) at
e5d928e. Discharges the singleton narrative-shorthand instance
Taut scout at 9a5502a counted (1/14 sites); the substrate's
pattern (14/14 sites landed as substrate-decl or consumer record
literal) now holds.

### §5.7 Landing 7 — LANDED at commit-in-flight (this canonical spec)

This canonical spec at `docs/specs/io-secrets-projection.md`.
Composition-only; zero new mints; grounds the four-shard mint
+ Landing 6 composition-bridge in the ancestry cascade of §0
through §4.

### §5.8 Landing 8 — Arc-2.3 peer_persistence.rs collapse (forward-promise)

`bootstrap/src/peer_persistence.rs` currently Rust FLOOR;
collapse target composes:
- @sheaf.acl_project — the Arc-2.3 sheaf primitive.
- @sheaf.section_at — the per-crystal section reader.
- @io/secrets.key_material_ref_of — Landing 6 composition-bridge.
- @io/secrets.project — the peer-key-gated projection.
- @io/secrets.materialize — the disk-side materialization.
- @kintsugi/consent.query_phi — the elevation discharge (per
  @sheaf spec §4.4).
- @subject/visibility.filter — the bauchladen sub-tray
  restriction (per @sheaf spec §5.3).

Per Seam A1 re-adjudication §5 + composition-alignment audit C9:
the composition chain is petri-net-complete AT LANDED altitude;
the Arc-2.3 collapse composes over the existing landed substrate
with zero new Rust FLOOR authorship licensed per
`[[feedback-no-rust-extension-shortcut]]`.

Alex-triggered Reed FLOOR work per Alex's persistent /loop
directive; awaits explicit Alex trigger to open Landing 8.

Test candidate (RED-first per TDD discipline): a
peer_persistence_arc_2_3_smoke.rs test that materializes a
peer's sub-tray of visibility crystals through the composition
chain; verifies:
- secrets_well_formed Pass across the admitted stalks (positive
  case).
- projection_valid Fail (with transparency<p> residual naming
  which claim broke) across the non-admitted stalks OR across
  the wrong-key-gated projections (negative case).
- round_trip_preserved Pass across the admitted-stalk +
  admitted-key positive case (discipline case).

### §5.9 Landing 9+ — future sub-species enumeration (forward-promise)

Candidate sub-species enumeration bounded by Arc-2.3 evidence
per two-tick discipline:

- **@io/secrets/age** — age standalone; no SOPS wrapping.
  Sibling to @io/secrets/sops at the vendor-tool altitude.
  Composes over @io/crypto.age_encrypt / age_decrypt directly.
  Landing warranted if Arc-2.3 collapse surfaces age-only
  projection cases outside SOPS.
- **@io/secrets/vault** — HashiCorp Vault Transit backend
  (KMS-style key management). Composes over @io/crypto.aead_seal
  + Vault Transit HTTP API at realisation boundary. Landing
  warranted if pack.members includes Vault-backed peers.
- **@io/secrets/1password** — 1Password Connect API backend.
  Sibling to vault at the KMS altitude; different vendor.
  Landing warranted if pack.members includes 1Password-backed
  peers.

Sub-species enumeration is only warranted if a distinct vendor
backend surfaces from Landing 8 evidence; sub-species proliferate
per consumer pull, not per speculation.

The two-tick discipline requires that Landing 9+ not be pre-
empted by speculation; land 8, observe, enumerate 9+ from 8's
evidence.

---

## §6 Recognition candidates

Per Mara-B §6 discipline (candidate strength; awaits second-
witness for ratification per Pack conventions), the four-shard
mint + Landing 6 composition-bridge admits recognition
candidates naming the substrate patterns the mint crystallizes.

### §6.1 #R-secrets-projection-composes-through-glass-wall-with-zero-family-root-crossings

**Candidate strength.** The peer-key-gated secrets projection
species composes ENTIRELY through @io Glass Wall with zero
family-root crossings inside @io/secrets's discharge. Every
load-bearing carrier (peer keys, ciphertext, disk write, vendor
SDK) lives at @io altitude; the species does not require any
non-@io family-root crossing to discharge its algebra. The @io
family-root Glass Wall discipline is SUFFICIENT for peer-key-
gated projection at substrate-decl altitude.

**First witness.** Shape-proposal §5.2 Ground 1 (io.mirror 37-40
Glass Wall verbatim fit check); Seam D-cascade §D2.a fit
verification (SOPS = vendor SDK; peer keys = raw bytes;
ciphertext = foreign blob; disk write = POSIX syscall — all
FITS @io).

**Second witness (awaited).** Landing 8 Arc-2.3
peer_persistence.rs collapse discharges the composition chain
at operational altitude; verifies the composition graph
traverses end-to-end with zero avoidable family-root crossings
inside @io/secrets's discharge (interior chain qualifier per
Seam D-cascade §D2.a micro-sharpen).

**Ancestry.** Alex 2026-07-14 SSH-signing design intent + @io
Glass Wall discipline (2026-06-08, T21 io.mirror family-root
landing). The recognition candidate is the substrate-decl form
of the Glass Wall discipline applied to peer-key-gated
projection.

### §6.2 #R-composition-bridge-arrows-land-as-substrate-decl-actions (SECOND WITNESS)

**Candidate strength.** Composition-bridge arrows at @io
altitude (and per Seam A1 re-adjudication, at any composition-
bridge site) land as substrate-decl actions with `\`-obligation-
blocked bodies; narrative-shorthand-constructor is NOT the
substrate's established pattern.

**First witness.** Per @sheaf canonical spec §6.3 (564571e):
Taut precedent scout at 9a5502a (2026-07-15) counted 1/14 sites
narrative-shorthand; 13/14 landed constructor or consumer
record literal; 8/9 species zero narrative-shorthand. Seam A1
re-adjudication at e5d928e ratified Position (b).

**SECOND WITNESS (LANDED at 64b0438).** Landing 6
@io/secrets.key_material_ref_of at species-decl 374 discharges
the singleton narrative-shorthand instance; the substrate's
pattern now holds at 14/14 sites landed. The recognition
candidate extends from @sheaf canonical spec §6.3 (first-
witness candidate strength) to this canonical spec §6.2
(second-witness ratified strength).

**Ancestry.** Eight of nine @io species already land carrier-
production actions substrate-decl'd BEFORE Landing 6; Landing 6
closes the pattern. The recognition is now ratified at
substrate-mechanical altitude.

### §6.3 #R-secret-projection-round-trip-is-splinter-narcissus-discipline

**Candidate strength.** The round_trip / round_trip_preserved
discipline at @io/secrets altitude structurally distinguishes
Splinter-pole projection (honest round-trip; bytes survive
project + materialize + retrieve byte-equal) from Narcissus-
pole (claimed projection that loses bytes; encoding drift,
compression, key mismatch masked at decrypt, disk truncation,
YAML/JSON key reordering not preserved). The discipline is
PARALLEL to @sheaf.section_at ∘ inverse per §3.5.

**First witness.** shape-proposal §2 recommended surface;
species-decl at secrets.mirror:466-533 (round_trip action +
round_trip_preserved bilateral); sops.mirror:361-422
(sops_round_trip + sops_round_trip_preserved).

**Second witness (awaited).** Arc-2.3 collapse Test candidate
per §5.8 discharge: peer_persistence_arc_2_3_smoke.rs verifies
round_trip_preserved Pass across the admitted-stalk +
admitted-key positive case AND round_trip_preserved Fail with
transparency<p> residual across the wrong-key-gated negative
case.

**Ancestry.** AEAD verifiability discipline (aead_open MUST
refuse unauthenticated plaintext per crypto.mirror:378-383);
SOPS round-trip fidelity discipline (sops.mirror:56-64 "sops_
decrypt ∘ sops_encrypt IS byte-identity on the section modulo
key ordering which SOPS preserves"); shape-proposal §2.4
verbatim round_trip discipline.

### §6.4 #R-fractal-lens-pointer-partitions-peer-key-from-secret-projection

**Candidate strength.** The Fractal.Lens pointer/thing partition
per `[[architecture-fractal-lens-pointer-thing-partition]]`
applies at @io/secrets altitude: the peer_ref field of
key_material_ref IS the POINTER; the secret_projection after
materialize IS the THING. The @io/secrets species substrate-
decls the partition at the peer-key-gated-projection altitude;
extends the pattern from @gift/lens.

**First witness.** Alex 2026-07-14 verbatim ("Fractal.Lens. A
pointer. Not the thing."); species-decl 194-196 substrate-
decisions block naming
`[[architecture-fractal-lens-pointer-thing-partition]]` as
landed decision.

**Second witness (awaited).** Landing 8 Arc-2.3 collapse
demonstrates the partition operationally: peer_key resolves
through @io/git plumbing at realisation boundary; substrate
never handles key bytes; the materialize action writes the
projection (the THING) to disk; the projection carries the
key_material_ref (the POINTER) as its identity field.

**Ancestry.** @gift/lens landing (Alex 2026-07-14 same-week
design intent phase) established the WHO-composed pointer
carrier; @io/secrets landing establishes the HOW-projection-
materializes pointer/thing carrier at the disk-projection
altitude.

### §6.5 #R-glass-wall-lift-tick-triggers-concurrent-sibling-species

**Candidate strength.** When a consumer pulls an @io species at
mirror-altitude and that consumer requires two other @io
species also at mirror-altitude AND both prerequisites had only
been declared at boot-floor grammar altitude, all three species
lift concurrently as one substrate landing. The Glass Wall lift-
tick discipline governs: no species can be lifted alone if its
composition graph requires sibling species not-yet-lifted.

**First witness.** Landing 4-5 four-shard mint at 059cf1c:
@io/secrets consumer-pulled @io/crypto + @io/fs at mirror-
altitude; both lifted concurrently per shape-proposal §5.5 +
Seam D-cascade §D9. Precedent from @io/git lift-tick 2026-06-24
(single-species lift; no concurrent siblings required).

**Second witness (awaited).** Future @io species lift-ticks
(@io/llvm; @io/process; @io/uri mirror-altitude lift; etc.)
either confirm the pattern (concurrent sibling lift if
composition graph requires) or refine it (some single-species
lifts admissible if composition graph closed at boot-floor).

**Ancestry.** `[[architecture-lift-as-load-bearing]]` discipline
(same primitive at two altitudes; grammar-altitude declarations
stay as transitional placeholders during consumer migration);
@cli → @mirror/lens/cli + @data/json → @mirror/data/json
precedents.

### §6.6 Awaited-witness discipline

Per Mara-B §6, recognition candidates crystallize at candidate
strength; second-witness ratification comes from operational
composition (Landing 8) or from a subsequent Taut scout
confirming the pattern across additional substrate sites. This
spec does not pre-empt Alex-adjudication on any candidate; Alex-
naming authority per Pack conventions governs recognition
ratification at candidate → landed altitude.

Recognition candidate #6.2 (composition-bridge arrows land as
substrate-decl actions) is the FIRST recognition candidate
across the two canonical specs (this + @sheaf at 564571e) to
achieve second-witness ratification WITHOUT waiting for Arc-2.3
operational discharge — the Landing 6 substrate-decl at 64b0438
IS the operational second-witness at substrate altitude.

---

## §7 A-series discharges

### §7.1 Adjudication residue from @secrets shape proposal

Per Seam Phase D-cascade §D2 + §D8 verdict: Candidate 2 SHIPS
as Mara-mintable species-decl + SOPS sub-species. Three REED-
INLINE cascades were required before Mara authored the species-
decls:

- **Cascade 1** (Seam D-cascade REED-INLINE #1) — shape-
  proposal §5.5 landing sequence add @io/crypto lift-tick
  alongside @io/fs. APPLIED at fc044ee.
- **Cascade 2** (Seam D-cascade REED-INLINE #2) — shape-
  proposal §5.2 Reason 1 vendor-language SOPS Python → Go
  (github.com/getsops/sops v3.x+ 2020+ CNCF-maintained).
  APPLIED at fc044ee.
- **Cascade 3** (Seam D-cascade REED-INLINE #3) — shape-
  proposal §2.7 dependency verification for @mirror/data/yaml +
  @mirror/data/json. APPLIED at 9f5befa.

Optional micro-sharpen per D2.a — qualify "zero family-root
crossings" as "zero family-root crossings INSIDE @secrets's
discharge (interior chain)" to disambiguate from consumer-side
@subject → @io crossings. APPLIED at species-decl 119-120 +
this spec §6.1 verbatim.

All cascades applied before Landing 4-5 (species-decl mints at
059cf1c); this canonical spec inherits the corrected ancestry.
Verification per §0.4 (three-grounds recommendation matches
ratified shape).

### §7.2 Adjudication residue from Seam composition-alignment audit

Per Seam composition-alignment audit at cec55a2: SHIP with two
REED-INLINE cascades and one Alex-adjudication residue.
Status:

- **REED-INLINE #1** (secrets.mirror:366 parameter name drift
  `sr` → `section`) — APPLIED at 57c5b3a. Verified at Seam A1
  re-adjudication R1.3.
- **REED-INLINE #2** (sops.mirror missing import of
  @subject/visibility/sheaf) — APPLIED at 57c5b3a. Verified
  at Seam A1 re-adjudication R1.3.
- **A1** (Alex-adjudication residue at composition-alignment
  §12) — RESOLVED via Seam A1 re-adjudication at e5d928e:
  SEAM-RATIFY Position (b) as Landing 6.

### §7.3 Adjudication residue from Seam A1 re-adjudication

Per Seam A1 re-adjudication at e5d928e: SEAM-RATIFY Position
(b) as Landing 6. Landed at 64b0438. Sub-choices deferred to
Landing 6 (NOT Alex-adjudication per re-adjudication §8):

- Exact name (`key_material_ref_of` vs `key_material_ref_
  from_peer` vs `peer_key_material_ref`) — RESOLVED at Landing
  6 as `key_material_ref_of` per readable-over-foundational
  Pack convention (attested composition-arrow suffix `_of` per
  precedent `manifest_for`, `oid_to_digest`).
- Whether to update composition-chain narrative prose at
  secrets.mirror:156 — ORTHOGONAL per re-adjudication §8;
  narrative retains prose form pending Arc-2.3 evidence.
- Whether ciphertext_ref ALSO needs a landed constructor —
  DEFERRED per re-adjudication §8; scout §5.2 out of scope.
  Awaits Arc-2.3 evidence.

None Alex-un-adjudicable; all Seam-adjudicable at Landing 6
review OR future Taut scouting.

### §7.4 No adjudication residue from this canonical spec

This spec is composition-only. Zero new mints. Zero new Alex-
adjudication questions. The recognition candidates in §6 sit at
candidate strength (§6.1, §6.3, §6.4, §6.5) OR second-witness
ratified strength (§6.2) per Mara-B §6 discipline; second-
witness ratification for candidate-strength items is Arc-2.3
discharge (Landing 8) or additional Taut scouting. Neither is
Alex-adjudication under Pack conventions.

**Combined Alex-adjudication residue after this canonical spec:
ZERO from @secrets cascade.** All prior Alex-adjudication items
(A1 from composition-alignment; A2/A4 from Phase D-cascade)
either RESOLVED via Seam re-adjudication or ORTHOGONAL to this
canonical spec scope. This spec introduces no A-series items.

---

## §8 Substrate-honest bounds

### §8.1 Rice-safety at whole-tick altitude

All five @io/secrets actions (key_material_ref_of, project,
materialize, retrieve, round_trip) are Rice-safe at whole-tick
altitude per Mara-B §4.5.5 template. All four bilaterals
(key_admits, projection_valid, round_trip_preserved, secrets_
well_formed) are Rice-safe at whole-tick altitude. Bounded
inspection of byte-visible substrate state (subject_instance
witness records, ACL byte structure, key algorithm strings,
ciphertext bytes, path structure). No program semantics
inspection. No unbounded recursion.

Concrete bounds per-action:

- `key_material_ref_of(peer)`: O(1) subject_instance witness
  lookup + O(|ssh_key|) key material load at realisation
  boundary (bounded by ssh-key vendor library key size).
- `project(section, k)`: O(|section.projected_value|) AEAD
  seal cost (bounded by plaintext byte length + constant-time
  platform intrinsics per glass-wall insight).
- `materialize(sp)`: O(|sp.ciphertext_bytes|) POSIX write cost
  (bounded by ciphertext byte length + kernel write throughput).
- `retrieve(cr, k)`: O(|cr.ciphertext|) POSIX read + AEAD open
  cost (bounded parallel to project).
- `round_trip(section, k)`: O(|section.projected_value|)
  composite cost (bounded sum of project + materialize +
  retrieve).
- `key_admits(k, sr)`: O(1) subject_instance witness reads +
  O(1) key algorithm lookup + O(1) peer_ref byte-equality.
- `projection_valid(sp)`: O(1) transitive section_computable +
  O(1) transitive key_admits + O(|sp.ciphertext_bytes|) AEAD
  verifiability decrypt-and-compare.
- `round_trip_preserved(section, k)`: O(|section.projected_
  value|) byte-compare after round_trip discharge.
- `secrets_well_formed(sp)`: O(1) transitive projection_valid +
  O(depth) transitive fs_well_formed (bounded by path depth per
  @io/fs writable bilateral).

@io/secrets/sops actions parallel per sub-species discipline;
all Rice-safe at whole-tick altitude per shape-proposal §2.7
sub-species scope-fit + Seam D-cascade §D8 verification.

### §8.2 Composition-only

This canonical spec:
- Does NOT re-declare carriers, actions, or bilaterals the
  species-decls mint. §2 references; does not duplicate.
- Does NOT introduce new type mints beyond what species-decls
  059cf1c + 57c5b3a + 64b0438 already declare.
- Does NOT introduce new @io boundaries beyond @io/secrets +
  @io/secrets/sops + @io/crypto + @io/fs (all landed at 059cf1c;
  Landing 6 at 64b0438 is a composition-bridge action, not a
  new boundary).
- Does NOT lift @io/secrets to family-root altitude; species-
  under-@io per Seam D2 verdict.
- Does NOT displace @io/crypto or @io/fs discipline; both are
  siblings at @io species altitude.
- Does NOT displace @sheaf canonical spec at 564571e; both land
  as sibling composition-only specs.

### §8.3 Two-tick discipline

Per shape-proposal §5.5 revised landing sequence: chosen mount
`@io/secrets` (species-under-@io) reads cleanly as "the peer-
key-gated projection species at @io altitude." Two-tick
alternatives rejected:

- `@secrets` (family-root) — foundational but duplicates @io
  scope. Fails substrate-already-had-the-word.
- `@subject/visibility/secrets` — inverts composition arrow;
  puts @io concerns at @subject altitude.
- `@sheaf/secrets` — inverts composition arrow verified at
  sheaf.mirror:149-150; @secrets is downstream of @sheaf.

The chosen mount honors readable-over-foundational per Pack
conventions. `@io/secrets` reads to any composer who has
grepped sheaf.mirror:140-150; the parent altitude `@io` is the
Glass Wall the projection composes under.

Landing 6 discharge honors two-tick discipline per Seam A1 re-
adjudication §5 R5: Landing 4-5 shipped as the first tick;
Landing 6 IS the second tick that closed the singleton
narrative-shorthand instance. Deferring Landing 6 further
without new adverse evidence would break the discipline.

### §8.4 @io Glass Wall preserved

Every load-bearing carrier of the four-shard mint stays UNDER
the @io Glass Wall:

- **Vendor tools UNDER @io.** SOPS binary invocation at
  @io/secrets/sops.sops_encrypt / sops_decrypt realisation
  boundary. age crate at @io/crypto.age_encrypt / age_decrypt.
  ssh-key crate at @io/crypto.ssh_key_material.
- **Raw bytes UNDER @io.** Peer keys (algorithm + material_ref
  in key_material carrier). AEAD nonces + ciphertexts (in
  ciphertext_bytes). Signature bytes (in signature_bytes).
- **Foreign blobs UNDER @io.** Ciphertext files (in
  ciphertext_ref). SOPS-format files (in sops_file_ref). All
  parsed only via vendor tools at realisation boundary.
- **Vendor SDKs UNDER @io.** SOPS Go binary. age crate. ssh-key
  crate. aes-gcm + chacha20poly1305 crates. std::fs (POSIX
  syscalls). All under the Glass Wall.

The @io Glass Wall discipline holds; @io/secrets composes
UNDER the wall, does not cross it. Per
`[[architecture-glass-wall-substrate-types]]`: transparency
inheritance through @io; every action returns imperfect<a, e,
l> per the family-root carrier discipline.

### §8.5 No family-root crossings inside @secrets's discharge

Per shape-proposal §2.4 + Seam D-cascade §D2.a micro-sharpen
(interior chain qualifier): @io/secrets's INTERIOR discharge
crosses ZERO family-root boundaries. Composition graph inside
@secrets's discharge:

- @io/secrets → @io/crypto (species-sibling; no family-root
  crossing).
- @io/secrets → @io/fs (species-sibling; no family-root
  crossing).
- @io/secrets → @io (family-root; internal composition, not a
  crossing).
- @io/secrets/sops → @io/secrets (species → parent; no crossing).
- @io/secrets/sops → @mirror/data/yaml + @mirror/data/json
  (species → external family-root; ONE consumer-side crossing
  at wire-format boundary — necessary per @io Glass Wall for
  vendor-format parsing).

The consumer-side @subject → @io crossing (at @sheaf.section_at
→ @io/secrets.project boundary) is UPSTREAM of @secrets's
discharge; per D-cascade micro-sharpen, this crossing is
external to the interior chain qualifier. Zero avoidable
family-root crossings per composition arrow.

Candidate 1 (@secrets family-root) would have added crossings;
per Seam D2 verdict, Candidate 2 minimizes them.

### §8.6 Bootstrap Rust FLOOR preserved

Per `[[feedback-no-rust-extension-shortcut]]` + Reed memory
`feedback-no-rust-extension-shortcut` (2026-07-14) + audit at
`docs/audits/2026-07-15-reed-substrate-dishonest-rust-
extensions-during-gift-arc.md`: the four-shard mint composes
ENTIRELY over @io at realisation boundary; zero Rust extension
authorship licensed. Realisation paths per Seam A1 re-
adjudication §3 R3:

- `key_material_ref_of` realisation: @io/crypto.ssh_key_material
  + record literal. NO Rust extension.
- `project` realisation: @io/crypto.aead_seal (or age_encrypt)
  over section.projected_value. NO Rust extension.
- `materialize` realisation: @io/fs.fs_write over sp.ciphertext_
  bytes.ciphertext + sp.projection_path. NO Rust extension.
- `retrieve` realisation: @io/fs.fs_read + @io/crypto.aead_open
  (or age_decrypt). NO Rust extension.
- `round_trip` realisation: project + materialize + retrieve
  composition. NO Rust extension.
- sops_* actions realisation: sops binary invocation + @io/
  crypto age backend + @mirror/data/yaml or json parsing.
  Vendor tool invocation is opaque-@io per Glass Wall; no shim
  Rust extension.

The Arc-2.3 collapse (Landing 8) composes over the four-shard
mint + Landing 6 composition-bridge + @sheaf composition chain
per §5.8; zero new Rust FLOOR authorship licensed. Per
[[substrate-floor:@io-boundary]] marker (Reed 2026-07-14 memory
tightening): if the realisation crosses @io, cite audit or
Signed-off-by: Seam trailer. This spec's Arc-2.3 forward-promise
COMPOSES over existing @io realisations; no new @io boundary
crossings required.

---

## §9 Witnesses

### §9.1 Alex verbatim

Per §0.1: Alex 2026-07-14 SSH-signing design intent in-
transcript peer-persistence session compaction. Two sentences
reproduced at species-decl 31-42 verbatim; five load-bearing
claims land the altitude for @io/secrets mount + @io/secrets/
sops sub-species mount per §0.1 sub-claims 1-5.

Alex is NOT directly witnessing this canonical spec; Alex is
witnessing through the ancestry cascade (design intent → shape
proposal → Seam ratification → species-decls → composition-
alignment audit → A1 re-adjudication → Landing 6 → this spec).
Alex's persistent /loop directive (collapse until unresolvable
ambiguity that cannot be adjudicated with a Seam spawn) is the
firing pattern that triggered this canonical spec landing per
Mara @sheaf shape proposal §5.5 sequence step 3 designation.

Per Reed's Fractal.Lens partition reading of Alex 2026-07-14
verbatim: the peer_ref field of key_material_ref IS the
Fractal.Lens POINTER Alex named; the substrate-vocab lift
("key_material_ref") is Mara/Reed shard-authoring per Seam A1
re-adjudication §6 R6 (Alex said "the Peers key"; substrate-
vocab is Mara/Reed lift; landing `_of` creates zero conflict
with Alex-verbatim preservation).

### §9.2 @secrets shape proposal ancestry

Per §0.4: Mara @secrets shape proposal at 766d930 + fc044ee +
9f5befa. Four candidates + Candidate 2 recommendation on three
structural grounds (@io Glass Wall + @onto refusal + Alex SSH-
signing design-intent match). REED-INLINE cascades applied
before Landing 4 discharge per §7.1.

Three-grounds recommendation crystallized the shape at
substrate-honest altitude before Seam adjudication; Seam D-
cascade §D2 ratified the three grounds without reversal.

### §9.3 Seam Phase D-cascade @secrets audit

Per §0.5: Seam Phase D-cascade audit at 019ad8f ratified
Candidate 2 as Mara-mintable per Scenario A. Seven-dimension
adjudication (D1-D9 + D-cross + D-length). Combined Alex-
adjudication residue for @secrets collapsed from N items to A4-
only (recognition candidate ratification per prior Phase D §D8;
orthogonal to @secrets shape).

Additional Seam witnesses at Landing 4-6:
- `docs/audits/2026-07-15-seam-sheaf-secrets-composition-
  alignment.md` (cec55a2) — composition-alignment audit; SHIP
  verdict.
- `docs/audits/2026-07-15-seam-a1-re-adjudication-with-taut-
  precedent.md` (e5d928e) — A1 re-adjudication; Landing 6
  ratification.

### §9.4 Composition-alignment audit

Per §0.6: Seam composition-alignment audit at cec55a2 verified
composition graph @sheaf → @io/secrets end-to-end. Nine
dimensions (C1-C9); all PASS. Two REED-INLINE cascades applied
at 57c5b3a; one A1 residue resolved via re-adjudication at
e5d928e.

The composition graph is Rice-safe petri-net-complete at
LANDED altitude per §4.5 arrow-by-arrow resolution table.

### §9.5 Taut precedent scout

Per `docs/scouts/2026-07-15-taut-io-type-as-constructor-
precedent.md` (9a5502a): grep-first read-only scout confirming
the substrate's established @io family pattern for composition-
bridge arrows. 1/14 sites narrative-shorthand; 13/14 landed
constructor or consumer record literal; 8/9 species zero
narrative-shorthand. Direct precedent for Landing 6
substrate-decl of @io/secrets.key_material_ref_of.

Taut's read-only discipline (grep-first; no shard authorship;
no Rust) grounded the A1 re-adjudication empirically. The
substrate's pattern was ALREADY there; Taut named it.

### §9.6 Seam A1 re-adjudication

Per `docs/audits/2026-07-15-seam-a1-re-adjudication-with-taut-
precedent.md` (e5d928e): SEAM-RATIFY Position (b) as Landing
6. Six-dimension re-adjudication (R1-R6); all PASS. Prior
"genuinely unresolvable at Seam altitude" verdict SUPERSEDED
by Taut precedent scout evidence.

Landing 6 landing at 64b0438 IS the operational discharge of
the re-adjudication verdict; §5.6 documents the landing;
§6.2 records the second-witness ratification for recognition
candidate #R-composition-bridge-arrows-land-as-substrate-decl-
actions.

### §9.7 The full cascade ancestry

The witness cascade for this canonical spec:

1. **Alex 2026-07-14 SSH-signing design intent** (verbatim) —
   the origin claim.
2. **@io Glass Wall discipline** (T21, 2026-06-08; io.mirror
   37-40) — the family-root wall the projection composes under.
3. **Mara @sheaf shape proposal** (60d9f5f + fc044ee; 2026-07-
   15) — the @sheaf carrier the projection composes over.
4. **Seam Phase D-cascade A2 ratification** (7d46f32) — @sheaf
   Candidate 2 Mara-mintable.
5. **Mara @sheaf species-decl mint** (d1ce901) — @sheaf
   carrier landed.
6. **Mara @secrets shape proposal** (766d930 + fc044ee +
   9f5befa) — four candidates + Candidate 2 recommendation.
7. **Seam Phase D-cascade @secrets ratification** (019ad8f) —
   Candidate 2 + SOPS sub-species Mara-mintable per Scenario
   A.
8. **Mara four-shard mint** (059cf1c) — @io/secrets +
   @io/secrets/sops + @io/crypto + @io/fs.
9. **Composition-alignment audit** (cec55a2) — end-to-end
   composition graph verification.
10. **Post-audit REED-INLINE cascades** (57c5b3a) — parameter
    name drift + sops import drift APPLIED.
11. **Taut precedent scout** (9a5502a) — @io family type-as-
    constructor precedent evidence.
12. **Seam A1 re-adjudication** (e5d928e) — A1 SUPERSEDED;
    SEAM-RATIFY Position (b) as Landing 6.
13. **Mara @sheaf canonical spec** (564571e) — sibling
    canonical spec ancestor.
14. **Landing 6 composition-bridge landing** (64b0438) —
    key_material_ref_of substrate-decl'd.
15. **This canonical spec** (commit-in-flight) — Landing 7.
    Composition-only; grounds the four-shard mint + Landing 6
    in the ancestry chain above.

Landing sequence continues at Landing 8 (Arc-2.3 peer_
persistence.rs collapse); Landing 9+ (sub-species enumeration,
if warranted).

---

## §10 Substrate-honest closure

This spec invented nothing. Every altitude, carrier name,
composition edge, math primitive, and witness cited above is
already landed:

- Glass Wall discipline — shards/io.mirror:37-40 (verbatim;
  2026-06-08 T21).
- Peer-key-gated projection design intent — Alex 2026-07-14
  verbatim (reproduced at species-decl 31-42).
- Fractal.Lens pointer/thing partition — shards/gift/lens.
  mirror + `[[architecture-fractal-lens-pointer-thing-
  partition]]`.
- AEAD Rice-safety — Mara-B §4.5.5 + shards/io/crypto.mirror
  bilaterals (2026-07-15 lift-tick).
- POSIX filesystem primitives — shards/io/fs.mirror (2026-07-
  15 lift-tick from boot/std/io.mirror boot-floor).
- SOPS vendor tool discipline — github.com/getsops/sops
  (Vehent 2017 + CNCF Go v3.x 2020+).
- age vendor tool discipline — age-encryption.org (Valsorda
  2019+).
- ssh-key vendor tool discipline — Alex 2026-07-14 SSH-signing
  design intent + Pack commit convention (ed25519 default).
- Sheaf-restriction as ACL — docs/specs/peer-persistence-and-
  home-projection.md §12.3 (verbatim; via @sheaf canonical
  spec at 564571e §3.2).
- Composition-bridge landing pattern — Taut precedent scout
  (9a5502a) + Seam A1 re-adjudication (e5d928e).
- Two-tick discipline — Pack conventions per CLAUDE.md
  substrate-honest is the mode.
- No-Rust-extension-shortcut discipline — Reed memory
  `feedback-no-rust-extension-shortcut` (2026-07-14) + audit
  at docs/audits/2026-07-15-reed-substrate-dishonest-rust-
  extensions-during-gift-arc.md.
- Lift-as-load-bearing discipline —
  `[[architecture-lift-as-load-bearing]]` + @cli / @data/json
  lift-tick precedents.

The four-shard mint at 059cf1c + Landing 6 at 64b0438 landed
the carriers where the substrate already put the discipline.
This canonical spec grounds the mint in the ancestry chain above
and names the composition graph explicitly so downstream readers
(Arc-2.3 peer_persistence.rs collapse; @sheaf canonical spec
consumers; any future sub-species enumeration; any downstream
Taut drift scout) can navigate without re-deriving the ancestry.

The ~60th instance of `[[feedback-substrate-already-had-the-
word]]` per species-decl 82-83, now with second-tick ratification
by canonical spec grounding. The word `@secrets` was substrate-
decl'd across FIVE carriers before Landing 4 landed the shard at
the sixth (species-under-@io):

1. `shards/subject/visibility/sheaf.mirror:140-150` — the
   forward-composition surface reference (15 references).
2. `shards/io.mirror:37-40` — the Glass Wall discipline naming
   vendor SDKs + raw bytes + foreign blobs under @io.
3. `shards/io.mirror:161-162` — @io/crypto vendor-surface
   naming (sha2, age, ssh-key).
4. `shards/io.mirror:189-191` — @io/fs forward-promise (POSIX
   filesystem surface).
5. `shards/subject/visibility/private.mirror` — Reed ancestry
   (~/.reed/visibility/private/ operational since ~2026-02-07).

Plus, now landed:

6. `shards/io/secrets.mirror` — the peer-key-gated projection
   species at @io Glass Wall altitude.
7. `shards/io/secrets/sops.mirror` — the first vendor sub-
   species (SOPS-backed).
8. `shards/io/crypto.mirror` — the mirror-altitude lift-tick
   companion.
9. `shards/io/fs.mirror` — the mirror-altitude lift-tick
   companion.
10. `docs/specs/io-secrets-projection.md` — this canonical spec
    grounding the four-shard mint + Landing 6 in the ancestry
    chain.

The ~60th instance of substrate-already-had-the-word for
@io/secrets; the ~61st for @io/crypto; the ~62nd for @io/fs;
the ~63rd for @io/secrets/sops. Four substrate-already-had-the-
word instances in one landing arc, all discharged as substrate-
decl carriers at species-under-@io altitude.

Landing 8 (Arc-2.3 peer_persistence.rs collapse) discharges the
composition chain at operational altitude. The substrate has the
word; the substrate now also has the carriers and the grounding.
Peer-key-gated projection through @io/secrets is substrate-
mechanical from Alex 2026-07-14 design intent to disk write per
@io/fs.fs_write, arrow-by-arrow, without intermediate narrative-
shorthand.

The bowl is one thing. The Peers key stays .git/mirror side.
Fractal.Lens is the pointer. The projection is the thing.

---

*Mara. Canonical spec. Composition-only. Reed commits as Mara
after review.*
