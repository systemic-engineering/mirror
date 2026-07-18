# fragmentation → mirror/rust/fractal migration spec

**Author:** Mara `<mara@systemic.engineer>` (pending identity cascade to
`mirror <mara@spectral.engineer>` per Alex 2026-07-18 architecture)
**Date:** 2026-07-18
**Kind:** 📝 pure-docs (markdown-only bypass; no code migration this tick)
**Status:** Canonical migration spec; Reed executes on top

**Alex direct-transcript (this session, verbatim):**

> "Q1. path dependency. fragmentation will get published to crates.io
> eventually. We just need to slim it down a bit to what mirror needs
> and cut the cruft. Spawn Mara on the background on that."

> "Wait I know it. Yes, spawn Mara and the task is wrote a migration
> spec from @../../fragmentation/ to @~/dev/projects/mirror/rust/fractal
> that is built ontop of prismqueer."

**Adjacent Alex directives loaded into this spec:**

- Alex 2026-07-13: "@fractal underlies @kintsugi/consent; mirror compiler
  IS a Mandelbrot set." Fragmentation is the process; **fractal is the
  resulting shape**. Rename baked into the mint.
- Alex 2026-07-14: SSH signing chain MUST stay in Rust (quoted in
  `bootstrap/src/peer_persistence.rs:22-24`) — this migration is
  substrate-honest `.rs` authorship composing over `fractal + local git
  config`, `@io/boundary` primitives per marker
  `[substrate-floor:@io-boundary]`.
- Alex 2026-07-18 (Mara doctrine, MARA.md):
  > "Different witness, different hash. My observation of this code is
  > part of what this documentation is."

---

## §1. The mint — fragmentation → fractal

**Source project:** `/Users/alexwolf/dev/projects/fragmentation/`
(standalone workspace; 4 members: `.`, `vcs/git`, `vcs/jj`, `vcs/mcp`,
`spectral`)

**Target crate:** `/Users/alexwolf/dev/projects/mirror/rust/fractal/`
(sibling crate under `mirror/rust/`; path-dep, not workspace member,
mirroring the `prism/imperfect/` (terni) precedent — Cargo.toml at
`/Users/alexwolf/dev/projects/prism/imperfect/Cargo.toml`).

### Why `fractal` (naming discipline)

Substrate-already-had-the-word audit:

| Word | Repo has it at | Ratification |
|---|---|---|
| `@fractal` | `shards/fractal.mirror` (mint scoped) + Recognition #79 candidate | Alex 2026-07-13 "mirror compiler IS a Mandelbrot set" — load-bearing |
| `fragmentation` | `/Users/alexwolf/dev/projects/fragmentation/` workspace | Kept as process-name for the source project (Mara's home) |
| `Fractal<E, H>` | `fragmentation/src/fragment.rs` (Shard/Branch/Lens variants) | Direct type-name; migration preserves symbol |

**The two words are not synonyms.** `fragmentation` is *the process of
breaking into content-addressed pieces*; `fractal` is *the resulting
self-similar shape*. Alex 2026-07-13 declared the shape as
substrate-load-bearing. **Mirror consumes the shape, not the process.**
Naming the mirror-side crate after the shape is delightfully-boring per
the `<primitive>_of_<input-shape>` convention (Alex 2026-07-18 pillar-arc
ratification).

**Path convention:** the eventual `@rust/fractal` shard-declaration lifts
this Rust crate into substrate vocabulary per the pattern precedent
`@../prism/imperfect` (terni) referenced in Alex's mint directive.

### Ratified naming decisions baked in

- Crate name: `fractal` (Cargo package name; publishable to crates.io)
- Crate path: `mirror/rust/fractal/`
- Dependency style: `path = "fractal"` from `mirror/rust/Cargo.toml`
  (NOT workspace member; matches `prism/imperfect/` precedent)
- Publishing path: eventually `crates.io/crates/fractal` (Alex directive)
- Downstream consumer names preserved: `Author`, `Committer`, `Witnessed`,
  `Timestamp`, `Message`, `Signature<K>`, `SSH`, `Local`, `Keys` trait

---

## §2. Substrate ground truth — what fragmentation carries today

Per Taut #2 scout (`docs/scouts/2026-07-18-taut-fragmentation-prior-art-for-subject-rs.md`)
+ this session's re-audit:

### Fragmentation top-level (`src/`) — 27 files

**Mirror-needed (KEEP → migrate):**

| File | LOC (approx) | Why mirror needs it |
|---|---|---|
| `witnessed.rs` | 59 | Author/Committer/Timestamp/Message — the split MARA doctrine formalizes ("different witness, different hash"); Alex Q2 ratified preserve-split |
| `keys.rs` | 526 | `Keys` trait + `Local { None, Ssh(SSH), Gpg(GPG) }` + full SSH signing/ECIES; `Signature<K>` private-constructor discipline |

**Mirror-adjacent (KEEP but slim):**

None at src/ altitude — everything else in `src/` is content-addressing
substrate mirror doesn't consume at rust/ altitude (see cut list §4).

### fragmentation-git (`vcs/git/`) — library layer

**Mirror-needed:**

| File | LOC (approx) | Why mirror needs it |
|---|---|---|
| `vcs/git/src/git.rs:11-30` | `read_witnessed` | Populates Witnessed from any git commit — direct floor for `subject.rs` provenance reads |
| `vcs/git/src/git.rs:66-77` | `commit_signature` | Extracts signature from signed commit; feeds two-witness verification |
| `vcs/git/src/bin/frgmt-git.rs:157-188` | `detect_keys(&repo)` | Reads `gpg.format` + `user.signingkey` from local git config → `Local`; direct floor for mirror's `phone.rs::git_commit_as` refactor |

**NOT needed by mirror** (see §4 cut list): `write_tree`, `read_tree`,
`write_node`, `read_node`, `write_commit`, `GitStore`, FUSE, MCP,
Lens/Manifest/Project machinery.

---

## §3. Target shape — `mirror/rust/fractal/`

```
mirror/rust/fractal/
├── Cargo.toml           — standalone crate; deps: prismqueer (path), ssh-key, x25519-dalek, chacha20poly1305, hkdf, sha2, hex, git2
├── README.md            — Mara-authored; the "why fractal, not fragmentation" narrative
├── LICENSE.md           — inherited from fragmentation (MIT / Apache-2.0 dual per prism precedent)
├── src/
│   ├── lib.rs           — public surface re-exports: Subject, SubjectKind, Keys, Local, SSH, Signature, Witnessed, Author, Committer, Timestamp
│   ├── subject.rs       — NEW at fractal altitude: Subject envelope (see §3.2)
│   ├── keys.rs          — slimmed from fragmentation/src/keys.rs (see §3.3)
│   ├── witnessed.rs     — verbatim from fragmentation/src/witnessed.rs
│   └── git.rs           — read_witnessed + commit_signature + detect_keys (extracted from vcs/git/src/git.rs + frgmt-git.rs; NO write path)
└── tests/
    ├── keys_signing.rs        — SSH signing property tests (see §3.5)
    ├── subject_provenance.rs  — Subject/Signature composition tests
    └── git_read_witnessed.rs  — read_witnessed + detect_keys tempdir tests
```

### §3.1 `lib.rs` public surface

```rust
//! `fractal` — content-addressed identity + witness primitives.
//!
//! Extracted from the `fragmentation` project (Mara's home, published
//! separately) and slimmed to what `mirror` needs at `rust/` altitude:
//! Subject, SigningKeys, Witnessed, Signature. Everything else in
//! fragmentation (Store, Fractal<E,H> tree, LAPACK prism, MCP,
//! FUSE, manifest/project machinery) stays in the fragmentation crate.
//!
//! Composes over `prismqueer::void::LiquidVoid<T>` for K=0 default
//! subject (see §5).

pub mod git;
pub mod keys;
pub mod subject;
pub mod witnessed;

pub use keys::{Keys, Local, LocalError, PlainKeys, Signature};
#[cfg(feature = "ssh")]
pub use keys::SSH;
pub use subject::{Subject, SubjectKind};
pub use witnessed::{Author, Committer, Message, Timestamp, Witnessed};
```

**No re-export of** `Fractal`, `Store`, `Fragment*`, `Ref`, `Sha`,
`SpectralCoordinate`, `HamiltonScheduler`, `Manifest`, `Project`,
`Naked`, `Singularity`, `PrismBridge`, `LapackPrism`, `Supervision`,
`ConcurrentStore`, `BoundedStore`, `FrgmntStore` — mirror doesn't
consume any of these at rust/ altitude.

### §3.2 `subject.rs` at fractal altitude (NEW)

Per Taut #2 §6 audit: mirror's `shards/subject.mirror` (Mara canonical
`5c06ee8`) is the substrate-decl; fragmentation's `Author + Local`
composition is the crypto-floor. Fractal at this altitude declares the
**Subject envelope** that bridges them:

```rust
use crate::keys::{Keys, Local, Signature};
use crate::witnessed::Author;

/// Envelope carrying identity + signing capability.
///
/// Composes @subject/subject_instance (mirror substrate) over the
/// fragmentation-crypto floor: name + email (git-native), keys (Local),
/// kind (Human | Peer | Void — the three SEL variants mirror consumes).
#[derive(Clone, Debug)]
pub struct Subject {
    pub author: Author,          // name + email (git-native)
    pub keys: Local,             // None | Ssh | Gpg
    pub kind: SubjectKind,       // Human | Peer | Void
}

/// The three subject variants mirror consumes at rust/ altitude.
///
/// Per shards/subject.mirror:214-220 (six-species SEL) collapsed to
/// the three-variant `actor_kind` at shards/subject.mirror:275-278.
/// Void is the K=0 default per shards/void.mirror (Mara 974a3f6).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubjectKind {
    Human,   // Alex / operator; SSH-signed
    Peer,    // Reed/Mara/etc; SSH-signed per identity attribution architecture
    Void,    // K=0 default; PlainKeys / Local::None; per @peer/void discipline
}

impl Subject {
    /// Construct a Void subject — K=0 default, no signing keys.
    pub fn void() -> Self { /* ... */ }

    /// Construct a Human subject from git config (author name/email +
    /// detected keys). Used by phone.rs::git_commit_as consumer.
    pub fn human(author: Author, keys: Local) -> Self { /* ... */ }

    /// Construct a Peer subject. Same shape as Human but distinct kind
    /// (identity attribution architecture: peer@spectral.engineer chain
    /// terminates at @alex root; see project_identity_attribution_architecture).
    pub fn peer(author: Author, keys: Local) -> Self { /* ... */ }

    /// Sign arbitrary bytes. Returns Signature<Local> carrying provenance.
    /// Void variant returns empty-bytes Signature (PlainKeys semantics).
    pub fn sign_bytes(&self, data: &[u8]) -> Result<Signature<Local>, keys::LocalError> { /* ... */ }
}
```

**Consumer note:** the file that will land at `mirror/rust/src/subject.rs`
becomes a **thin wrapper** re-exporting `fractal::Subject` +
`fractal::SubjectKind` for backward-compatibility with mirror-side
call sites. **Do NOT author subject.rs at mirror/rust/src/ separately** —
it consumes fractal::Subject.

### §3.3 `keys.rs` slimming rules (from fragmentation/src/keys.rs)

**KEEP as-is:**
- `Encrypted<K>` struct
- `Signature<K>` struct (private-constructor discipline; the crypto-floor
  form of SEL "withdrawal-available" bilateral — see Taut #2 §Surprise)
- `Keys` trait
- `PlainKeys` no-op impl
- `Local { None, Ssh, Gpg }` enum
- `LocalError` enum
- `SSH` struct + all methods (`from_path`, `generate_ed25519`,
  `write_to_file`, `fingerprint`, `sign_bytes`, ECIES `encrypt_bytes` /
  `decrypt_bytes`, `x25519_secret` / `x25519_public`)
- `GPG` struct behind `gpg` feature (KEEP but M0 gates it off by default)

**CUT:**
- All references to `crate::encoding::{Decode, Encode}` — replace with
  `serde` derives (mirror doesn't consume fragmentation's Encode trait
  at rust/ altitude)
- All references to `crate::fragment::{ContentAddressed, Fractal,
  Fragmentable, TreeShaped}` — `Keys::sign` signature simplifies to
  accept `&[u8]` bytes directly (mirror signs commit-shas as bytes, not
  Fractal<E>)
- All references to `crate::ref_::Ref` and `crate::sha::Sha` /
  `HashAlg` — cut alongside Fractal cut
- `Keys::encrypt` / `Keys::decrypt` returning `Encrypted<Self>` /
  `Fractal<E>` — refactor to `Vec<u8> → Vec<u8>` (mirror doesn't compose
  encrypted-Fractal at rust/ altitude; the ECIES machinery stays for
  future @kintsugi/store consumers)

**Refactored `Keys` trait signature:**

```rust
pub trait Keys: Sized + Clone {
    type Error: fmt::Display + fmt::Debug;

    /// Sign raw bytes. Returns proof of authorship.
    fn sign(&self, data: &[u8]) -> Result<Signature<Self>, Self::Error>;

    /// Encrypt raw bytes. Returns opaque ciphertext.
    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, Self::Error>;

    /// Decrypt raw bytes. Returns plaintext.
    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, Self::Error>;

    /// Content-addressable identity of this key.
    fn fingerprint(&self) -> String;
}
```

Loss vs fragmentation: gives up the `Fractal<E>` typed-content sign
composition. This is intentional — mirror's SSH signing chain signs
git commit SHAs (bytes), not fragmentation-encoded fractal trees.

### §3.4 `git.rs` at fractal altitude (extracted from fragmentation-git)

Three functions only:

```rust
use crate::keys::Local;
use crate::witnessed::{Author, Committer, Message, Timestamp, Witnessed};

/// Read witness metadata from any git commit (works on any commit,
/// not just fragmentation ones). Extracted from
/// fragmentation/vcs/git/src/git.rs:11-30.
pub fn read_witnessed(
    repo: &git2::Repository,
    oid: git2::Oid,
) -> Result<(Witnessed, Message, git2::Oid), Box<dyn std::error::Error>> { /* verbatim */ }

/// Extract the signature from a signed commit, if present.
/// Extracted from fragmentation/vcs/git/src/git.rs:66-77.
pub fn commit_signature(
    repo: &git2::Repository,
    oid: git2::Oid,
) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> { /* verbatim */ }

/// Detect signing keys from local git config.
/// Extracted from fragmentation/vcs/git/src/bin/frgmt-git.rs:157-188.
pub fn detect_keys(repo: &git2::Repository) -> Local { /* verbatim */ }
```

### §3.5 Property tests (inline per rust/ discipline)

Three test files. Each mirrors Reed's rust/ prop_tests discipline
(Iter 1-10 arc: `rust/src/collapse.rs::prop_tests`, `void.rs::prop_tests`).

**`tests/keys_signing.rs`:**
- `signature_new_requires_keys_bound` — private-constructor discipline;
  only obtainable through `Keys::sign` (Taut #2 §Surprise executable)
- `plain_keys_signature_bytes_empty` — PlainKeys crypto-null
- `ssh_sign_deterministic_per_key_per_message` — sign twice same key +
  same bytes → verifies same signature (Ed25519 is deterministic)
- `fingerprint_stable_per_key` — content-addressable identity

**`tests/subject_provenance.rs`:**
- `void_subject_sign_returns_empty_signature` — Void = K=0 default
- `human_subject_signature_carries_key_provenance` —
  `Signature::key().fingerprint()` matches `Subject::keys.fingerprint()`
- `subject_kind_distinguishes_human_from_peer_at_type_level` — pattern
  match discipline (identity attribution architecture — @alex root
  chain terminates on Human variant only)
- `different_witness_different_hash` — MARA doctrine executable: two
  distinct Subjects signing same bytes → two distinct Signatures with
  distinct `key.fingerprint()` values

**`tests/git_read_witnessed.rs`:**
- `read_witnessed_populates_author_committer_from_commit` — tempdir
  git repo, one commit, verify Author + Committer split preserved
- `commit_signature_returns_none_for_unsigned` — walker's baseline
- `detect_keys_returns_none_when_no_signing_key_configured` — Void default
- `detect_keys_returns_ssh_when_gpg_format_ssh` — SSH path
- `read_witnessed_preserves_author_ne_committer_split` — Alex Q2
  ratified: MARA doctrine, `.author != .committer` when configured
  distinctly

---

## §4. What to cut — refused-migration list with rationale

### Cut from fragmentation/src/ (25 files)

| File | Reason |
|---|---|
| `bounded_store.rs`, `concurrent_store.rs`, `store.rs`, `frgmnt_store.rs` | Store machinery; mirror uses `prismqueer::liquid` + forthcoming `@mirror/store/liquid` cache, not fragmentation's Store |
| `fragment.rs`, `cid.rs`, `ref_.rs`, `sha.rs`, `shard_ref.rs` | Content-addressing substrate; mirror consumes @kintsugi + git SHA-1 natively via git2, not fragmentation's tree |
| `commit.rs`, `diff.rs` | Draft/Commit builder; mirror commits via `phone.rs::git_commit_as` (subprocess), not fragmentation's write_commit |
| `encoding.rs` | Encode/Decode trait; not needed with slimmed Keys signature |
| `hamilton_scheduler.rs` | Hot/cold shard scheduler; forward-promise for @mirror/store/liquid |
| `lapack_prism.rs`, `prism_bridge.rs` | Fragmentation ↔ prism bridge; mirror consumes prismqueer directly |
| `manifest.rs`, `project.rs` | Lens projection machinery; mirror-adjacent territory (not needed at rust/) |
| `naked.rs`, `singularity.rs` | Filesystem materialization; mirror uses `bootstrap/src/peer_persistence.rs` + `shards/peer/persistence.mirror` |
| `repo.rs` | Repo trait for store; not consumed at rust/ |
| `spectral_coordinate.rs` | `SpectralCoordinate<5>`; mirror consumes spectral coords via `../../prism/prismqueer` path-dep, not via fragmentation |
| `supervision.rs` | OTP-style supervision trees over fragments; forward-promise for @torus |
| `visibility.rs` | Public/Protected/Private wrappers; mirror's consent architecture lives at @kintsugi/consent shard altitude, not rust/ |
| `walk.rs` | Fragment walker; mirror's walkers live at shard-body composition altitude |

### Cut from fragmentation/vcs/ (all workspace members)

| Path | Reason |
|---|---|
| `vcs/git/src/bin/frgmt-git.rs` | Binary CLI; mirror doesn't need the fragmentation CLI, only the `detect_keys` function extracted from it |
| `vcs/git/src/git.rs` (mostly) | Keep only `read_witnessed`, `commit_signature`; cut `write_tree` / `read_tree` / `write_node` / `read_node` / `write_commit` / `write_tree_named` / `read_tree_named` / `GitStore` / `relabel_named` |
| `vcs/git/src/{bounded_store,commit,concurrent_store,fuse,namespaced,notes,store,walk,atomic}.rs` | All content-store adapters; mirror doesn't consume |
| `vcs/jj/` (entire workspace member) | jj backend; mirror is git-native |
| `vcs/mcp/` (entire workspace member) | MCP server; mirror's MCP surface lives at `phone.rs` altitude (Mara §3), not fragmentation-mcp |
| `spectral/` (entire workspace member) | Already merged upstream per Reed task #69 (`project_mirror_spectral_crate_relationship`); mirror-adjacent |

### Cut from fragmentation top-level

| Path | Reason |
|---|---|
| `gleam/` | BEAM-lineage; separate project altitude (see also Mara's home discipline) |
| `cairn/` | Mara's home discipline; not mirror's rust/ altitude |
| `build/`, `tasks/`, `flake.nix`, `flake.lock`, `Justfile` | Fragmentation project infrastructure; mirror has its own |
| `.mcp.json` (gleam-side) | Fragmentation project MCP config; not mirror's |
| `docs/`, `README.md`, `ROADMAP.md`, `MARA.md`, `MARA.gestalt` | Fragmentation project docs; fractal crate gets its own README (§3) |
| `tests/` (top-level) | Integration tests for fragmentation surface; fractal gets its own tests/ per §3.5 |

**Refused mint list** (things NOT to introduce during migration):

- **No `Fractal<E, H>` tree type re-export.** Mirror's `@fractal`
  substrate-decl is at shard altitude; the Rust `Fractal` type is
  fragmentation's own vocabulary. Do not conflate.
- **No workspace lift.** fractal is standalone Cargo.toml, path-dep from
  mirror/rust/Cargo.toml; matches `prism/imperfect/` precedent. Alex
  Q1 answer this session: "path dependency."
- **No `@fractal/*` shard mints during this migration.** Shard mints are
  Mara authorship territory at spec altitude; this migration is
  code-only. `@fractal` family-root + species stay forward-promised.
- **No mirror-side Fractal storage API.** Mirror consumes Subject +
  Signature + Witnessed; storage is `@mirror/store/liquid` territory.

---

## §5. Composition on top of prismqueer

Per Alex Q2 answer (this session, via Reed's mid-transcript update):
LiquidVoid<T> trait lives at `prismqueer::liquid` + `prismqueer::void`.

### §5.1 Void composition

```rust
// fractal::subject::Subject::void() composes over LiquidVoid<Subject>
use prismqueer::void::LiquidVoid;

impl LiquidVoid<Subject> for Subject {
    fn void() -> Self {
        Subject {
            author: Author::new("", ""),
            keys: Local::None,
            kind: SubjectKind::Void,
        }
    }

    fn is_void(&self) -> bool {
        matches!(self.kind, SubjectKind::Void)
    }
}
```

Void Subject is the K=0 default at fractal altitude — matches
`shards/void.mirror` (Mara `974a3f6`) + Reed's `void.rs` GREEN
(`569b0b5`).

### §5.2 Pillar composition (Signature<K> as measurement)

Signature production is a measurement per Pillar II
(`prismqueer::liquid::pillar::algedonic`):

- Positive shrinkage (successful signature; bytes non-empty) → `Pass`
- Zero magnitude (PlainKeys or Void) → treated as `Pass` for empty-bytes
  path (semantic: Void is not a failure; it's the K=0 default), OR
  `Fail` when signature was required (caller-decided)
- Below-threshold (partial success; e.g., signature bytes truncated) →
  `Partial` with confidence

Consumers wire this via `pillar::algedonic_of_magnitude(&sig_len,
&threshold)` at Pillar II altitude.

### §5.3 Trust chain composition

Per project_identity_attribution_architecture memory: `mirror
<peer@spectral.engineer>` chain terminates at `@alex` root.
`Subject::sign_bytes` produces `Signature<Local>` carrying
`key.fingerprint()`; downstream `@trust` verifier (forward-promise;
Mara authorship territory) walks the fingerprint chain back to root.

**No @trust family-root mint during this migration** — the crypto floor
composes; the substrate-decl lands separately.

### §5.4 What prismqueer provides / what fractal provides

| Layer | Provides | Consumes |
|---|---|---|
| prismqueer | LiquidVoid<T> trait, PropertyVerdict, pillar primitives, Prism trait, Bundle, Loss | (nothing from fractal) |
| fractal | Subject, SubjectKind, Keys trait, Local, Signature<K>, Witnessed, git primitives (read_witnessed, commit_signature, detect_keys) | prismqueer (LiquidVoid trait for Void default; Loss impl for pillar composition) |
| mirror/rust | phone.rs (git_commit_as refactored), matrix.rs, main.rs, collapse.rs, void.rs, subject.rs (thin wrapper) | fractal (Subject envelope), prismqueer (dev-deps for prop tests) |

The path-dep chain: mirror/rust → fractal → prismqueer.
No circular deps. Each altitude strictly shrinks Rust FLOOR per
`feedback_no_rust_extension_shortcut` discipline.

---

## §6. Migration recipe — Reed's step-by-step execution plan

**Precondition:** Reed's `void.rs` GREEN (already landed, `569b0b5`).
Alex Q2 LiquidVoid trait placement at prismqueer::void empirically
landed (Reed forward-promise).

### Step 1 — Scaffold `mirror/rust/fractal/` crate

Create directory structure:

```bash
mkdir -p /Users/alexwolf/dev/projects/mirror/rust/fractal/{src,tests}
```

Author `Cargo.toml` per §7 shape below.

Author `LICENSE.md` (inherit from `/Users/alexwolf/dev/projects/fragmentation/LICENSE.md`
verbatim: MIT OR Apache-2.0 dual).

Author `README.md` (Mara-authored; forward-promise, not this-tick).

### Step 2 — Migrate `witnessed.rs` (verbatim)

`cp /Users/alexwolf/dev/projects/fragmentation/src/witnessed.rs \
    /Users/alexwolf/dev/projects/mirror/rust/fractal/src/witnessed.rs`

No changes needed. All 5 types (Author, Committer, Timestamp, Message,
Witnessed) migrate verbatim. Preserves Alex Q2 author-vs-committer
split and MARA doctrine.

### Step 3 — Migrate `keys.rs` (slimmed per §3.3)

Copy source, then apply cuts:

1. Remove `use crate::encoding::{Decode, Encode};` and all
   `use crate::fragment::{...}` + `use crate::ref_::Ref` +
   `use crate::sha::{HashAlg, Sha}` imports.
2. Refactor `Keys` trait signatures per §3.3 (bytes in, bytes out).
3. Refactor `Local::sign` / `Local::encrypt` / `Local::decrypt` impls
   to match new trait shape (drop Fractal<E> wrapping).
4. Refactor `PlainKeys::decrypt` to return `Vec<u8>` (drop Fractal
   reconstruction).
5. Preserve `SSH::sign_bytes` (unchanged — already operates on bytes)
   and full ECIES `encrypt_bytes` / `decrypt_bytes` machinery.
6. Preserve `GPG` behind `gpg` feature (unchanged); M0 default off.

### Step 4 — Author `subject.rs` at fractal altitude

New file at `mirror/rust/fractal/src/subject.rs` per §3.2 shape.
Compose `Subject { author, keys, kind }`. Implement `LiquidVoid<Subject>`
per §5.1. Add `void() / human() / peer()` constructors and
`sign_bytes()` method.

### Step 5 — Extract `git.rs` from fragmentation-git

Extract `read_witnessed` from `fragmentation/vcs/git/src/git.rs:11-30`
and `commit_signature` from `:66-77` and `detect_keys` from
`fragmentation/vcs/git/src/bin/frgmt-git.rs:157-188` into new
`mirror/rust/fractal/src/git.rs`. All three verbatim (with import
adjustments: `use crate::witnessed::...` + `use crate::keys::Local`).

### Step 6 — Author `lib.rs` public surface

Per §3.1 shape. Module declarations + re-exports.

### Step 7 — Author property tests

Three test files per §3.5. RED-first per Mara TDD discipline: stub
implementations with `todo!()`, commit `🔴`, fill in bodies, commit
`🟢`. Reed's inline prop_tests discipline (Iter 1-10 arc) applies.

### Step 8 — Update `mirror/rust/Cargo.toml`

Add path-dep on fractal:

```toml
[dependencies]
fractal = { path = "fractal", features = ["ssh"] }
```

Add to `[dev-dependencies]` block alongside existing `prismqueer` +
`terni` + `tempfile` entries.

### Step 9 — Refactor `phone.rs::git_commit_as`

Per Mara §6 spec + Alex Q2 ratified split. New signature:

```rust
pub(crate) fn git_commit_as(
    repo_root: &Path,
    author: &fractal::Subject,
    committer: &fractal::Subject,
    message: &str,
) -> io::Result<String>
```

Body: extract `author.author.name` + `author.author.email` for
`-c user.name=... -c user.email=...` on the `git commit -F -` subprocess
call. Committer variant passed separately (though currently git honors
`GIT_COMMITTER_NAME` / `GIT_COMMITTER_EMAIL` env vars; extend `Command`
env to pass them).

Update all call sites in mirror/rust/src/main.rs (and elsewhere) to
construct `fractal::Subject::human(...)` before invoking.

### Step 10 — Land `subject.rs` at mirror/rust/src/ as thin wrapper

Author `mirror/rust/src/subject.rs`:

```rust
//! Thin re-export wrapper. All Subject/SubjectKind machinery lives in
//! the `fractal` sibling crate (path-dep). This file preserves the
//! mirror-side call-site vocabulary.

pub use fractal::{Subject, SubjectKind};
```

Optional: add mirror-specific extension methods here if any land later
(e.g., `Subject::from_mirror_config`).

### Step 11 — Verify empirical firing

```bash
cd /Users/alexwolf/dev/projects/mirror/rust
cargo build
cargo test --features ssh
```

Expected: all three fractal test files GREEN. All existing rust/ tests
still GREEN. `mirror --help` still fires.

### Step 12 — Commit sequence

Sequential commits per CLAUDE.md discipline. Reed executes:

1. `🔴 Reed [rust-fractal-migration] fractal crate scaffold + witnessed.rs
   + keys.rs slim + subject.rs + git.rs + tests stubbed with todo!()`
2. `🟢 Reed [rust-fractal-migration] fractal tests GREEN; mirror/rust
   consumes fractal::Subject via path-dep`
3. `📝 Reed [rust-fractal-migration] phone.rs::git_commit_as refactored
   to accept &Subject, &Subject; call sites updated`

Author attribution: `Reed <reed@systemic.engineer>`. SSH-signed default.
Marker on `.rs` files: `[substrate-floor:@io-boundary]` + Seam gate
(audit citation or `Signed-off-by: Seam`) since this crosses the .rs
FLOOR (extending Rust altitude, admissible per Alex 2026-07-14 SSH
signing design intent quoted in `bootstrap/src/peer_persistence.rs:22-24`).

---

## §7. Path forward + Cargo.toml shape

### §7.1 `mirror/rust/fractal/Cargo.toml`

```toml
[package]
name = "fractal"
version = "0.1.0"
edition = "2021"
license = "MIT OR Apache-2.0"
description = "Content-addressed identity + witness primitives — the crypto floor mirror composes on top of prismqueer."
repository = "https://github.com/systemic-engineering/mirror"
keywords = ["identity", "signing", "content-addressed", "ssh", "git"]
categories = ["cryptography", "authentication"]
readme = "README.md"

[features]
default = ["ssh"]
ssh = ["dep:ssh-key", "dep:x25519-dalek", "dep:chacha20poly1305", "dep:hkdf"]
gpg = []

[dependencies]
# Compose over prismqueer's LiquidVoid<T> trait for Void default (K=0
# subject); Loss impl for pillar composition. Path-dep matches
# mirror/rust/Cargo.toml [dev-dependencies] shape.
prismqueer = { path = "../../../prism/prismqueer", features = ["bundle"] }

# Crypto floor (all optional behind `ssh` feature)
ssh-key = { version = "0.6", features = ["std", "ed25519", "crypto"], optional = true }
x25519-dalek = { version = "2", features = ["static_secrets"], optional = true }
chacha20poly1305 = { version = "0.10", optional = true }
hkdf = { version = "0.12", optional = true }

# Always-on
sha2 = "0.10"
hex = "0.4"

# Git wire interop for read_witnessed / commit_signature / detect_keys
git2 = "0.19"

[dev-dependencies]
tempfile = "3"
```

**Note on prismqueer path:** three parent traversals because
`mirror/rust/fractal/` is three levels deep from `/Users/alexwolf/dev/`
where `prism/` sits as sibling to `mirror/`. Path resolves to
`/Users/alexwolf/dev/projects/prism/prismqueer/`.

### §7.2 Post-migration state

- `fractal` crate publishable to crates.io as Alex directed
- `fragmentation` project continues at `/Users/alexwolf/dev/projects/fragmentation/`
  as Mara's home discipline; can either become a downstream consumer of
  `fractal` (dedup crypto floor) or continue as-is (Mara's call)
- mirror/rust/src/subject.rs stays as thin wrapper composing
  fractal::Subject; can gain mirror-specific extension methods later
- `@fractal` shard family-root mint stays forward-promised (Mara
  authorship territory; grounded in Alex 2026-07-13 "mirror compiler IS
  a Mandelbrot set" + Recognition #79 candidate)
- `@trust` chain verifier stays forward-promised (walks
  Signature::key().fingerprint() chain back to @alex root)

### §7.3 Future migrations enabled

Once fractal lands + publishes:

- fragmentation can pull fractal as a dep to dedup (single source of
  truth for Author/Committer/Witnessed/Keys/Signature)
- @kintsugi/store may consume fractal::Signature for content-addressed
  provenance
- @mirror/store/liquid may extend fractal::Subject with cache-aware
  variants
- fragmentation-mcp remains standalone (mirror does NOT consume MCP
  from fragmentation; mirror's MCP lives at phone.rs altitude)

---

## §8. Q's for Alex (max 3)

### Q1 — features gating: `default = ["ssh"]` or `default = []`?

Fragmentation defaults to `["concurrent", "prism-bridge", "visibility",
"singularity", "project", "supervision"]`. Fractal drops all of those.
Should fractal's `default = ["ssh"]` (SSH signing on by default —
matches mirror's substrate-default use case) or `default = []` (fully
opt-in — matches "publishable slim crate" ethos)?

**Mara lean:** `default = ["ssh"]`. Mirror consumers always want SSH;
zero-signing consumers can `default-features = false`. Delightfully-
boring: the common case is the default.

### Q2 — Author = Committer shorthand on Subject constructor?

Fragmentation's `witnessed.rs` splits Author + Committer as distinct
structs. §3.4 refactor lets `phone.rs::git_commit_as` take
`(&Subject, &Subject)` (Alex Q2 ratified preserve-split).
Should Subject expose a `Subject::with_committer(author, keys, kind)`
constructor that also produces a matching Committer inside, so
call-sites can do `git_commit_as(&s, &s, msg)` for the common case where
author == committer?

**Mara lean:** yes, add the ergonomic shorthand. Preserve the type-level
split; ease the common case. `let s = Subject::human(...); git_commit_as(&s, &s, msg)`
is delightfully-boring; `let a = ...; let c = ...; git_commit_as(&a, &c, msg)` stays
available for the Pack use case (Reed drafts + mirror-runtime commits).

### Q3 — `@rust/fractal` shard mint timing?

Once `mirror/rust/fractal/` empirically fires, should Mara mint the
`@rust/fractal` shard-declaration (lifting the Rust crate into
substrate vocabulary per `@../prism/imperfect` precedent) in the same
arc, or forward-promise it to a separate spec tick?

**Mara lean:** forward-promise. This migration is code-scoped; the
substrate-decl mint is separate authorship territory that composes
with @fractal family-root arc (also forward-promised). Keeping them
separate preserves single-responsibility per arc.

---

## §9. One-sentence surprise

The migration is delightfully-boring precisely because **the shape
substrate-already-had-the-word**: fragmentation's Author + Committer +
Signature<K> private-constructor discipline was written to encode the
same load-bearing invariant mirror's `shards/subject.mirror` was written
to declare — the two altitudes were speaking the same language before
they knew about each other, and this migration is the syntactic edit
that makes the semantic identity visible.

---

**Delivery:** spec complete. Reed executes migration recipe §6 on top.

**Sequential landing:** commit this spec (📝 pure-docs markdown-only
bypass) as `Mara <mara@systemic.engineer>`.

*Mara — different witness, different hash.*
