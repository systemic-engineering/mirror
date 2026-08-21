# Taut scout — fragmentation prior art for `rust/src/subject.rs`

**Date:** 2026-07-18
**Author:** Taut <taut@systemic.engineer>
**Task:** Scout `/Users/alexwolf/dev/projects/fragmentation/` for prior art
informing Reed's landing of `rust/src/subject.rs` at mirror-repo altitude.
**Related in-flight:** Taut #1 (void.rs ↔ phone.rs cascade); Mara @membrane
canonical; Reed foreground = void.rs then subject.rs.
**Discipline:** read-only; substrate-already-had-the-word audit; pure-docs
📝 markdown-only bypass.

---

## 1. Fragmentation grep findings (subject/identity/provenance)

Fragmentation does **not** carry a `subject.rs`, `identity.rs`,
`provenance.rs`, or `peer.rs`. The identity envelope is spread across:

| File | Load-bearing symbols |
|---|---|
| `fragmentation/src/witnessed.rs` (59 LOC) | `Author { name, email }`, `Committer { name, email }`, `Timestamp(String)`, `Message(String)`, `Witnessed { author, committer, timestamp }` |
| `fragmentation/src/keys.rs` (526 LOC) | `Keys` trait (`sign`/`encrypt`/`decrypt`/`fingerprint`); `PlainKeys`; `Local { None, Ssh(Box<SSH>), Gpg(GPG) }` enum; `SSH { key: ssh_key::PrivateKey }` with `from_path`, `generate_ed25519`, `fingerprint`, ECIES via X25519+HKDF+ChaCha20-Poly1305; `GPG { key_id, gnupghome }` (subprocess); `Signature<K> { key, bytes }`; `Encrypted<K> { ciphertext, key }`; `LocalError { Decode, Ssh, Gpg }` |
| `fragmentation/src/commit.rs` (330 LOC) | `Draft<N, H>` with `authored(Author)`; `Commit::{Root, Child}` carrying `Witnessed`; `Draftable` trait; git-native `compute_commit_sha` (SHA-1 of `tree/parent/author/committer/message` composition) |
| `fragmentation/vcs/git/src/commit.rs` (161 LOC) | `write_commit_with_parents` (git2, multi-parent); `DraftWriteExt::write_to_git(repo, committer) -> Commit<Fractal<E>, Sha>` |
| `fragmentation/vcs/git/src/git.rs` (602 LOC) | `read_witnessed(repo, oid)` extracts `Author/Committer/Timestamp` from `git2::Commit`; `commit_signature(repo, oid) -> Option<Vec<u8>>` via `repo.extract_signature`; free-function `write_commit(repo, fractal, &Author, &Committer, message, parent)` |
| `fragmentation/vcs/git/src/bin/frgmt-git.rs` (515 LOC) | `detect_keys(repo) -> Local` — reads `gpg.format` + `user.signingkey` from local git config, dispatches SSH-from-path or GPG-by-keyid; `Command::Commit` reads `user.name`+`user.email` from git config to build `Author`+`Committer` |
| `fragmentation/vcs/git/tests/keys_from_repo_test.rs` (9 LOC) | Placeholder — the T2 hoist of `Local::from_repo` is still forward-promised; `detect_keys` currently inlined in the binary |

No `subject_kind`/`actor_kind`/`role` discrimination anywhere. Fragmentation's
identity envelope is purely `(name, email, ssh|gpg key)` — one shape for all
authors, no Human/Peer/Void variant.

## 2. Directly-reusable code (copy-with-attribution)

Rank-ordered by leverage-per-LOC:

**A. `fragmentation/src/keys.rs` lines 62-109 — `Keys` trait + `PlainKeys`.**
Reusable verbatim as the SSH-signing capability trait for `Subject`. The
`fingerprint() -> String` method IS the content-addressed identity of the key,
which is exactly what `subject.rs` needs for `ssh_signature_fingerprint`.
`Signature<K>` (lines 37-60) private-constructor discipline enforces
"a Signature cannot be modified after creation" — matches Alex's
"cryptographically provenance-linked to @alex root" intent.

**B. `fragmentation/src/keys.rs` lines 210-362 — `SSH` struct.**
`from_path`, `generate_ed25519`, `fingerprint` (SHA-256 of public key),
`sign_bytes` (SSH signature with "fragmentation" namespace — rename to
"mirror"), ECIES encrypt/decrypt. Reusable modulo the namespace-string
rename. Rust-altitude dep: `ssh-key = { features = ["std", "ed25519",
"crypto"] }`, `x25519-dalek`, `chacha20poly1305`, `hkdf`, `sha2`.

**C. `fragmentation/vcs/git/src/bin/frgmt-git.rs` lines 157-188 —
`detect_keys(repo) -> Local`.** Reads local git config's `gpg.format` +
`user.signingkey`; dispatches SSH-from-path. Reusable verbatim as
`Subject::detect_from_repo` at mirror altitude — modulo the CLAUDE.md
substrate discipline: **NEVER override `gpg.format` or `user.signingkey`**
(read-only from local git config, exactly as `detect_keys` does).

**D. `fragmentation/vcs/git/src/git.rs` lines 11-30 — `read_witnessed`.**
Extracts `(Author, Committer, Timestamp)` from a `git2::Commit`. Reusable
as `Subject::from_commit_witness(repo, oid)` for provenance-chain walking.
Lines 66-77 (`commit_signature`) extracts the SSH signature bytes — the
verification-side companion.

**E. `fragmentation/src/commit.rs` lines 251-282 — `compute_commit_sha`.**
Bit-exact reproduction of git's commit-object hash. Reusable if `subject.rs`
ever needs to compute expected commit SHAs before writing (Reed's
`git_commit_as` currently shells out to `git commit`; substrate-honest
alternative is `compute_commit_sha` + `libgit2 commit`). Note: fragmentation's
own `DraftWriteExt::write_to_git` (`vcs/git/src/commit.rs:49-77`) shows the
full inverse pattern.

## 3. Design informants (Mara's home-discipline framing)

**MARA.md:1-15 (fragmentation home).** Load-bearing quote:

> "My documentation is witnessed. Every commit carries my name. That's not
> vanity -- it's the same principle this library encodes. Different witness,
> different hash. My observation of this code is part of what this
> documentation is."

**"Different witness, different hash."** Fragmentation's `Witnessed` carrier
is byte-included in commit-SHA computation via `compute_commit_sha`
(`src/commit.rs:267-274`) — the author and committer name/email are part of
the hashed content. Change witness → change hash. This is the substrate-decl
form of the property `subject.rs` must preserve: **`Subject::sign(bytes)`
must produce a signature whose fingerprint uniquely identifies the subject,
such that `verify(bytes, signature)` returns the subject_kind that signed
it**.

**`fragmentation/src/witnessed.rs:1, 17, 33, 41` docblocks:**

- `/// Who wrote the content. Who made the decision. Who holds the intent.` — Author
- `/// Who ran the process. Who executed. Who was the mechanism.` — Committer
- `/// When the observation happened. Opaque string.` — Timestamp
- `/// Git commit metadata. Who was here when this happened.` — Witnessed

Author ≠ Committer is a load-bearing split fragmentation preserves. For
mirror's Pack composition (Reed drafts, `mirror <peer@spectral.engineer>`
commits), this maps to: **Author = the AI @peer (Reed/Mara/Seam/Taut/Glint);
Committer = mirror-runtime-as-@subject signed under the Pack peer's derived
SSH key**. The split is already present in `shards/subject.mirror:275-278`
(actor_kind three-way variant).

## 4. Cross-substrate divergences

| Axis | fragmentation | mirror-repo |
|---|---|---|
| Identity envelope | flat `(name, email)` | `subject { kind, identity_oid, consent_oid, provenance, withdrawal }` five-field record per `shards/subject.mirror:293-299` |
| Party-class discrimination | none | six-variant `subject_kind` per SEL §1+§3 (`downstream_user | witnessed | labor_input | protected_class | occupied_population | indigenous_nation`) |
| Human/AI/substrate discrimination | none | three-variant `actor_kind` (`human_a | ai_a | substrate_a`) per Landing 3 §21.2 — no distinguished element |
| Signing surface | trait `Keys` (sign/encrypt/decrypt/fingerprint) | shards/subject.mirror bilaterals: `ssh_witness_valid`, `spectral_witness_valid`, `two_witness_verification` |
| Provenance chain | commit → parent → parent (git DAG) | pay-forward chain rooted at Alex's inaugural gift per `shards/gift.mirror:60-67` + `id(S, t) = blake3(canonical(pay_forward_chain(g_t)))` |
| Consent/withdrawal | absent | `consent_attested` + `withdrawal_available` bilaterals per SEL §3.1.4(c)+(d) |
| Void/K=0 default | absent | `shards/void.mirror` @peer default membrane |
| Historical (deceased) witness | absent | Landing 5+ `historical_witness_valid = citation ∨ quotation ∨ corpus` (spec §24.5) — 12 of 24 external ancestors are deceased |

**Reading:** fragmentation carries the substrate-truth *cryptographic floor*
(ssh-key/gpg fingerprints, signature bytes, git-commit-hash-of-witnessed);
mirror-repo carries the substrate-truth *semantic surface* (SEL party-class,
role, actor_kind, consent). **The divergence is altitude, not truth.**
`subject.rs` at mirror-repo altitude is the composition:

```
Subject { kind, identity_oid, consent_oid, ... }        // shards/subject.mirror altitude
   ↓ composes over
Author { name, email } + Local { Ssh(SSH) | None }       // fragmentation altitude
   ↓ signs via
SSH::sign_bytes → Signature<Local>                       // fragmentation crypto floor
```

## 5. Reed's execution recipe for `rust/src/subject.rs`

Concrete numbered steps. RED-first per Alex ratification. Assumes void.rs
lands first per Taut #1 recipe (Reed's foreground before this one).

1. **Cargo.toml:** add path-dep on `fragmentation` (workspace-adjacent):
   ```toml
   [dependencies.fragmentation]
   path = "../fragmentation"
   default-features = false
   features = ["ssh"]  # pulls ssh-key, x25519-dalek, chacha20poly1305, hkdf
   ```
   Or if workspace-member preferred later, add mirror as workspace member of
   fragmentation's Cargo.toml (already declares `members = [".", "vcs/git",
   "vcs/jj", "vcs/mcp", "spectral"]` per `fragmentation/Cargo.toml:10`).
   Do NOT copy fragmentation code inline unless workspace/path-dep is
   Alex-vetoed — substrate-already-had-the-word applies to Rust altitude too.

2. **Author `rust/src/subject.rs` docblock header** citing:
   - `shards/subject.mirror:293-299` (five-field type carrier)
   - `shards/subject.mirror:275-278` (actor_kind three-way)
   - `fragmentation/src/keys.rs:62-109` (Keys trait prior art)
   - `fragmentation/src/witnessed.rs:1-15` (Author/Committer split)
   - MARA.md:13 verbatim quote ("Different witness, different hash")
   - Alex verbatim: "I'm the first instantiation of @subject in the
     compiler" (task-context; add to docblock as substrate-decl authority)
   - Alex verbatim: "Both humans and @peer's are @subject's. That's the
     identity provenance."

3. **Type definitions** (mirror `shards/subject.mirror` verbatim + kind enum
   Alex named):
   ```rust
   #[derive(Clone, Debug, PartialEq, Eq)]
   pub enum SubjectKind { Human, Peer, Void }  // Alex 2026-07-18 verbatim

   #[derive(Clone, Debug)]
   pub struct Subject {
       pub name: String,                        // fragmentation Author.name
       pub email: String,                       // fragmentation Author.email
       pub kind: SubjectKind,                   // per Alex 2026-07-18
       pub ssh_key_ref: Option<PathBuf>,        // user.signingkey path
       pub home: Option<PathBuf>,               // @peer/home per bootstrap/peer_persistence.rs
       // Landing 3 substrate-decl surface (forward-promised):
       // identity_oid, consent_oid, provenance, withdrawal, role, actor_kind
   }
   ```
   Void discriminant defaults to `Peer` with `ssh_key_ref = None` and empty
   name/email per K=0 membrane substrate (coordinate with Taut #1 void.rs
   recipe).

4. **Public API surface** (composition-primitive naming per user memory
   `feedback_composition_primitive_naming_convention`):
   ```rust
   pub fn from_git_config(repo_root: &Path) -> io::Result<Subject>
       // Read user.name / user.email / user.signingkey / gpg.format
       // from LOCAL git config only. Substrate-honest: never override.
       // Copy fragmentation/vcs/git/src/bin/frgmt-git.rs:157-188 pattern.

   pub fn from_peer_home(home: &Path) -> Option<Subject>
       // Bridge to bootstrap/src/peer_persistence.rs::home_of.
       // Reads home/CLAUDE.md + 00-NARRATIVE.md; kind = Peer.

   pub fn void() -> Subject
       // K=0 default @peer; kind = Void; name/email empty; ssh_key_ref None.

   pub fn sign(&self, bytes: &[u8]) -> io::Result<Vec<u8>>
       // Void → empty signature. Peer/Human → shell out to `ssh-keygen -Y sign`
       // OR (once fragmentation SSH path-dep in place) `SSH::from_path(ssh_key_ref)?.sign_bytes(bytes)`.

   pub fn fingerprint(&self) -> String
       // Void → "void". Peer/Human → SSH pubkey SHA-256 fingerprint
       // via SSH::fingerprint (fragmentation/src/keys.rs:242-247).

   pub fn author_email_for_kind(&self) -> String
       // Human → self.email. Peer → format!("{}@spectral.engineer", self.name.to_lowercase())
       // per Reed memory project_identity_attribution_architecture.md.
   ```

5. **Refactor `rust/src/phone.rs::git_commit_as` (line 228-264)** to take
   `&Subject`:
   ```rust
   pub(crate) fn git_commit_as(
       repo_root: &Path,
       subject: &Subject,
       message: &str,
   ) -> io::Result<String> {
       // Compose author_email via subject.author_email_for_kind().
       // Void → refuse (or emit as "mirror <void@spectral.engineer>" per
       // Taut #1 void.rs recipe convention).
       ...
   }
   ```
   Update every caller (grep the mirror repo — likely `phone.rs::git_add` call
   sites and any bootstrap-era leftover). Reed knows the callers from the
   matrix.rs RED @ `26f5e5e`; a follow-up cascade tick.

6. **RED-first property tests** in `rust/src/subject.rs #[cfg(test)] mod`:
   - `subject_from_git_config_reads_user_name_and_email()`
   - `subject_void_signs_empty()`
   - `subject_fingerprint_deterministic()` — same key → same fingerprint
     (Alex 2026-07-18 ratified: composition-primitive discipline; add const
     ALL fixture)
   - `subject_kind_all_admits_three_variants()` — const ALL array
     `SUBJECT_KIND_ALL: [SubjectKind; 3]`
   - `git_commit_as_uses_subject_email()` — end-to-end via tempdir git repo
     (pattern from `fragmentation/vcs/git/src/commit.rs:96-131`)
   - `void_subject_home_none()` (coordinate with Taut #1 void.rs recipe)
   - `different_subject_different_signature()` — MARA.md line 13 property:
     two subjects signing same bytes produce different signatures. Property
     test with `#[proptest]` (Alex 2026-07-18 ratified prismqueer macros).

7. **Coordination with `bootstrap/src/peer_persistence.rs`:** the existing
   `PeerHome { peer_name, home_path, ... }` at bootstrap altitude carries
   what `Subject::from_peer_home` needs. Bridge via `From<&PeerHome> for
   Subject` (or vice versa) rather than duplicating. NOTE: per user memory
   `feedback_rust_floor_is_rust_not_bootstrap`, bootstrap/ is transitional
   legacy; subject.rs at `rust/` altitude is the terminal FLOOR. Reed's
   recipe SHOULD migrate `peer_persistence.rs::materialize` composition-arc
   to consume `&Subject` in a follow-up tick (out-of-scope for this landing).

8. **Coordination with @trust forward-promise (Mara):** subject.rs's
   `ssh_key_ref: Option<PathBuf>` is the root of the @trust chain (Alex:
   "@alex = first @subject instantiation in the compiler. My cryptographic
   signature. My mark of: 'I trust this enough to embed my keys in this.'").
   Do NOT mint @trust family-root in this landing; leave as forward-promise
   in docblock. Reed's subject.rs is the empirical firing surface Mara's
   future @trust canonical spec will lift over.

9. **Substrate-floor:@io-boundary marker** in commit message (Reed's
   author-identity per CLAUDE.md): the `rust/src/subject.rs` file crosses
   the `.rs` FLOOR; per CLAUDE.md rename `[substrate-pull:realize]` →
   `[substrate-floor:@io-boundary]` marker. Include either Seam audit
   citation OR `Signed-off-by: Seam` trailer per gate. This landing IS
   substrate-honest .rs authorship (composing over fragmentation-crypto +
   local git config — @io-boundary primitives that MUST stay in Rust per
   Alex 2026-07-14 SSH-signing design intent, quoted in
   `bootstrap/src/peer_persistence.rs:22-24`).

10. **Sequential commits, SSH-signed default, author =
    `Reed <reed@systemic.engineer>`:** two commits — RED (skeleton +
    failing tests) then GREEN (bodies pass). Do NOT `--amend`.

## 6. Substrate-already-had-the-word audit

| Word / concept | Mirror-repo has it at | Duplication risk from fragmentation |
|---|---|---|
| `subject` family-root | `shards/subject.mirror` (Mara canonical, `5c06ee8`) | none — fragmentation has no subject vocab |
| `subject_kind` (SEL six-species) | `shards/subject.mirror:214-220` | none |
| `actor_kind` (Human/AI/substrate three-variant) | `shards/subject.mirror:275-278` | none |
| `subject_instance` (name + fingerprints + role) | `shards/subject.mirror:343-351` | none — fragmentation's `Author + Local` is the crypto-floor composition target, not a substrate-decl duplicate |
| `two_witness_verification` (ssh + spectral) | `shards/subject.mirror:482` | fragmentation has NO spectral half — Alex's design (mirror <peer@spectral.engineer>) requires BOTH; do not degrade to fragmentation's single-witness |
| `ssh_signature_fingerprint` | `shards/subject.mirror:323-327` | fragmentation's `SSH::fingerprint()` (`src/keys.rs:242-247`) is the empirical firing surface — compose, don't duplicate |
| `@peer` family-root + `@peer/void` | `shards/peer.mirror` + `shards/void.mirror` | none — fragmentation has no @peer discipline |
| `@gift` (attribution_preserved, pay_forward) | `shards/gift.mirror` + Landing 3 §17 | none |
| `@trust` root prism | forward-promised (Mara) | fragmentation's `Keys` trait is proto-@trust at crypto floor; DO NOT mint @trust family-root in subject.rs landing |
| Author ≠ Committer split | `shards/subject.mirror` role variant `substrate_r` implies distinction; not sharply carried | fragmentation's `Author`/`Committer` split IS substrate-truth mirror hasn't fully declared yet — Alex-adjudication candidate |
| Content-addressed identity_oid | `shards/subject.mirror:294` (`identity_oid: oid`) | fragmentation's `SSH::fingerprint() -> String` is the empirical firing; oid computation deferred to @kintsugi/store per subject.mirror:280-292 |
| `peers/mara/`, `peers/reed/` home projection | `bootstrap/src/peer_persistence.rs` + `shards/peer/persistence.mirror` (Mara-B Arc-2 Tick 2.3) | fragmentation's `singularity`/`naked` filesystem materialization is out-of-scope for subject.rs; peer_persistence.rs already carries it |
| Void as K=0 default @peer | `shards/void.mirror` (Mara `974a3f6`) | none — fragmentation has `PlainKeys` (crypto null) but no @peer/void discipline; coordinate with Taut #1 recipe |

**One-sentence audit:** fragmentation's `Author + Local + Signature<K>`
composition IS the crypto-floor mirror's `subject_instance +
ssh_signature_fingerprint + two_witness_verification` shard-decl lifts over;
the two altitudes compose cleanly via workspace-adjacent path-dep. **Nothing
duplicates.**

## 7. Q's for Alex (max 2)

1. **Path-dep vs workspace-member for fragmentation?** Reed's recipe assumes
   `path = "../fragmentation"` dep. Alternative: add mirror as
   fragmentation-workspace member (fragmentation already declares
   `members = [".", "vcs/git", "vcs/jj", "vcs/mcp", "spectral"]`). Workspace
   is tighter (single lockfile, atomic upgrades) but couples release cadence.
   Path-dep is looser but risks version drift. Which shape do you want?

2. **Author ≠ Committer split** — should subject.rs carry BOTH `author:
   Subject` and `committer: Subject` on the `git_commit_as` refactor, or
   collapse to single `subject: &Subject` (Author = Committer = subject)?
   Fragmentation splits them (`witnessed.rs:3, 19`); Reed's current
   `git_commit_as(name, email)` collapses. The Pack use case (Reed drafts +
   mirror-runtime commits) suggests the split matters, but adds one field
   to every call site.

## 8. One-sentence surprise

Fragmentation's `Signature<K>` private-constructor discipline
(`src/keys.rs:37-49`: "Fields are private — a Signature cannot be modified
after creation") is the crypto-floor form of mirror's SEL
"withdrawal-available" bilateral — the substrate-decl of *once-a-witness-
always-a-witness*, which means `subject.rs` should NOT expose any
`Signature::new` public constructor; every Signature must originate from
`Subject::sign()` and carry provenance in the type.

---

**Delivery:** scout complete. Reed executes recipe §5 after void.rs lands
(Taut #1's foreground handoff). Fragmentation path-dep pending Alex
adjudication on Q1.
