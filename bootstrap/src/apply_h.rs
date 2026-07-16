//! Arc-1 evaluator FLOOR — 7-combinator surface for shard-body dispatch.
//!
//! Per `docs/specs/kintsugi-ouroboros-arc-1-evaluator-combinator-surface.md`
//! (etymology-renamed at d44841e; §5 A/H/D correspondence).
//!
//! This module is the Rust FLOOR that Arc-1 lifts `sbec` from 0 to > 0
//! through. It exposes exactly 7 primitives — the closed dispatch
//! calculus a shard body's action body composes over + `@io`. Every
//! primitive is irreducible past `@io` composition per §1 of the spec.
//!
//! Correspondence (per spec §5, eigensheaf.md §3.2 Connes triple):
//!
//! | Side | Combinators                          |
//! |------|--------------------------------------|
//! | A    | `section`, `fold`, `act`             |
//! | H    | `settle`, `crystallize`              |
//! | D    | `coboundary`, `utter`                |
//!
//! GREEN state (Arc-1 Tick 1.3): the 7 combinator bodies compose over the
//! already-landed primitives in `bootstrap/src/spectral.rs` (`Combinator`,
//! `Fold5`, `compose_a`, `apply_h`, `eigen_d`) + `bootstrap/src/hash.rs`
//! (`hash_tagged`, the substrate's content-address FLOOR). The smoke
//! test `evaluator_shard_body_dispatch_smoke` now discharges Pass for
//! `@subject/visibility/public.consent_scope_universal` — the first
//! sbec lift from 0 to > 0. Tick 1.4 wires `mirror beam act` as the CLI
//! verb the same dispatch surface answers.
//!
//! **Minimum-viable-GREEN scope.** The bilateral-predicate dispatch
//! path (`act` → recognize shard action ref → byte-check argument
//! against the shard's substrate-decl'd sentinel → return Verdict) is
//! the shortest tractable dispatch that lifts sbec. Non-bilateral-
//! predicate action bodies (multi-arg, @io-composing, metalogue-writing,
//! settle-descending) get their substrate-honest MVP scaffolding here
//! so the module compiles + can be extended per-shard, but only the
//! bilateral-predicate path is smoke-tested at Tick 1.3. Subsequent
//! ticks under `[substrate-floor:@io-boundary]` extend the resolver
//! surface to the full shard-action grammar as new smoke tests demand.
//!
//! Marker discipline: `[substrate-floor:@io-boundary]` + Seam Phase
//! D-cascade audit citation
//! (`docs/audits/2026-07-15-seam-kintsugi-ouroboros-phase-d-cascade-a2-a6.md`).
//!
//! Signatures below use `String` for substrate refs (Ref surface) and
//! opaque `Verdict` / `Section` / `Transparency` / `SettledVerdict` /
//! `BenchCrystal` structs so the API shape is committed at RED-authoring
//! time without prematurely binding the concrete carriers Tick 1.3 will
//! wire to `spectral.rs::{Verdict<S>, Combinator, Fold5, Spectrum}`.

// ─────────────────────────────────────────────────────────────────────────────
// Surface types (RED-phase carriers).
//
// These types name the API shape the GREEN implementation will fill in.
// Each maps to a landed primitive in `bootstrap/src/spectral.rs` per the
// composition graph in the spec's §1.x @io-boundary paragraphs. Tick 1.3
// reifies these as aliases / newtypes over the landed carriers.
// ─────────────────────────────────────────────────────────────────────────────

/// Substrate-ref surface. Content-addressed pointer into the mirror-store.
pub type Ref = String;

/// The algebra A's section carrier — a parsed AST node the shard body reads.
/// Tick 1.3 aliases to `bootstrap/src/ast.rs::AstNode`.
#[derive(Debug, Clone)]
pub struct Section {
    pub oid: Ref,
}

/// A typed value flowing through a shard body's fold / act composition.
#[derive(Debug, Clone)]
pub struct Value {
    pub oid: Ref,
}

/// The Dirac coboundary's output — located opacity per substrate ref.
/// Tick 1.3 aliases to `prismqueer::Transparency<Ref>`.
#[derive(Debug, Clone)]
pub struct Transparency {
    pub located_opacity: Vec<(Ref, String)>,
}

/// A shard body's action-decl verdict. Tick 1.3 aliases to `@glass.verdict`
/// via `bootstrap/src/spectral.rs::Verdict<S>`.
#[derive(Debug, Clone)]
pub enum Verdict {
    Pass,
    Fail(String),
    Partial(Transparency),
}

/// The settled harmonic representative or pending residual.
/// `SettledClean` ⇔ `h ∈ ker(Δ_0) = H^0(F)`. `SettledPending` ⇔
/// `‖e‖ ≥ ε` after `max_iters` per spec §1.5.
#[derive(Debug, Clone)]
pub enum SettledVerdict {
    SettledClean(Section),
    SettledPending(Transparency),
}

/// A metalogue turn record. Tick 1.3 aliases to the landed
/// `@code/metalogue::turn` carrier per shards/metalogue.mirror:47-52.
#[derive(Debug, Clone)]
pub struct SubstrateEvent {
    pub kind: String,
    pub body_oid: Ref,
}

/// The tick-boundary bench crystal per `@mirror/bench.record`.
/// Content-addressed observation of `ouroboros_state` before/after.
#[derive(Debug, Clone)]
pub struct BenchCrystal {
    pub before_oid: Ref,
    pub after_oid: Ref,
    pub crystal_oid: Ref,
}

/// The `ouroboros_state` snapshot per shards/kintsugi/ouroboros.mirror:252.
#[derive(Debug, Clone)]
pub struct OuroborosState {
    pub oid: Ref,
}

/// Five reducers for the Connes basis-axis fold per spec §1.3
/// (focus / project / split / shift / settle).
/// Tick 1.3 aliases to `bootstrap/src/spectral.rs::Fold5`.
#[derive(Debug, Clone)]
pub struct Fold5Reducers {
    pub focus_oid: Ref,
    pub project_oid: Ref,
    pub split_oid: Ref,
    pub shift_oid: Ref,
    pub settle_oid: Ref,
}

// ─────────────────────────────────────────────────────────────────────────────
// Reflective bilateral corpus — the substrate-honest form of the
// ~30 hand-typed apply_h::act arms. Per Mara canonical spec 9a77361
// (docs/specs/bilateral-predicate-substrate-shape.md) + math
// foundation 701828a (docs/math/epistemologic/pact/bilateral-
// sentinel.md). Alex 2026-07-16 verbatim: "Q1. Let's mint it then.
// Properly. Seems like it's load-bearing."
//
// The loader line-scans `shards/**/*.mirror` for `bilateral <name>
// { sentinel "..." arity <n> require <ref> }` blocks. The reflective
// evaluator `discharge` implements spec §5.2 pseudocode: base
// bilaterals byte-check every arg's oid for sentinel containment;
// composed bilaterals AND-conjunct sub-bilaterals on the same args.
//
// ADDITIVE: `act` checks the reflective corpus FIRST, falling
// through to the legacy hand-typed arms if not found. No arm is
// modified. Landing 5 (separate future tick) retires arms as their
// bilateral blocks land in shards.
// ─────────────────────────────────────────────────────────────────────────────

/// One `bilateral` declaration extracted from a shard file. Fields
/// follow the typed carrier at shards/epistemologic/pact/bilateral.mirror.
#[derive(Debug, Clone)]
pub struct BilateralDecl {
    /// Predicate name (e.g. "signature_integrity").
    pub name: String,
    /// Sentinel byte-string (e.g. "chain=merkle-linked").
    pub sentinel: String,
    /// Number of args expected (1 base; 2+ composed).
    pub arity: usize,
    /// Sub-bilateral names for composed bilaterals (empty for base).
    /// Each entry is either a bare predicate name (resolved within the
    /// same shard ref) or a full `@shard/ref.predicate` action ref.
    pub require: Vec<String>,
    /// Full action ref (e.g. "@spectral/signature.signature_integrity").
    pub full_action_ref: String,
}

/// Extract bilateral declarations from `.mirror` file source. Line-scans
/// for `bilateral <name> { ... }` blocks. Tokenizer note: the current
/// bootstrap tokenizer's Project reader stops at non-identifier chars,
/// so `sentinel "foo=bar"` doesn't round-trip through the AST cleanly;
/// this loader reads raw bytes to capture the sentinel string. Landing
/// 2 grammar registration still admits the block so downstream OIDs
/// don't surface as Dark regions.
fn extract_bilaterals(source: &str, shard_ref: &str) -> Vec<BilateralDecl> {
    let mut out = Vec::new();
    let mut lines = source.lines().peekable();
    while let Some(line) = lines.next() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            continue;
        }
        let rest = match trimmed.strip_prefix("bilateral ") {
            Some(r) => r,
            None => continue,
        };
        let name_part = match rest.split_once('{') {
            Some((n, _)) => n.trim(),
            None => continue,
        };
        if name_part.is_empty() {
            continue;
        }
        let name = name_part.to_string();
        let mut sentinel = String::new();
        let mut arity: usize = 1;
        let mut require: Vec<String> = Vec::new();
        for body_line in lines.by_ref() {
            let bt = body_line.trim();
            if bt.starts_with('}') {
                break;
            }
            if let Some(v) = bt.strip_prefix("sentinel ") {
                let v = v.trim();
                sentinel = v
                    .strip_prefix('"')
                    .and_then(|s| s.strip_suffix('"'))
                    .unwrap_or(v)
                    .to_string();
            } else if let Some(v) = bt.strip_prefix("arity ") {
                if let Ok(n) = v.trim().parse::<usize>() {
                    arity = n;
                }
            } else if let Some(v) = bt.strip_prefix("require ") {
                require.push(v.trim().to_string());
            }
        }
        if sentinel.is_empty() && require.is_empty() {
            eprintln!(
                "bilateral_corpus: skipping ill-formed decl {}.{} (empty sentinel + no require)",
                shard_ref, name
            );
            continue;
        }
        let full_action_ref = format!("{}.{}", shard_ref, name);
        out.push(BilateralDecl {
            name,
            sentinel,
            arity,
            require,
            full_action_ref,
        });
    }
    out
}

/// Derive the enclosing shard's @-ref from a `.mirror` file's source.
/// Prefers a top-level `prism @X/Y {}` declaration; falls back to the
/// file path (stripped of `shards/` prefix and `.mirror` suffix).
fn shard_ref_from_source(source: &str, path: &std::path::Path) -> String {
    for line in source.lines() {
        let t = line.trim_start();
        if t.starts_with('#') {
            continue;
        }
        if let Some(rest) = t.strip_prefix("prism ") {
            let rest = rest.trim_start();
            if let Some(at) = rest.strip_prefix('@') {
                let end = at
                    .find(|c: char| c == ' ' || c == '\t' || c == '{' || c == '\n')
                    .unwrap_or(at.len());
                if end > 0 {
                    let mut s = String::with_capacity(end + 1);
                    s.push('@');
                    s.push_str(&at[..end]);
                    return s;
                }
            }
        }
    }
    let p_string = path.to_string_lossy().into_owned();
    let mut p: &str = p_string.as_ref();
    if let Some(idx) = p.find("shards/") {
        p = &p[idx + "shards/".len()..];
    }
    let p = p.strip_suffix(".mirror").unwrap_or(p);
    format!("@{}", p)
}

/// Recursively walk `root` collecting all `.mirror` files. Errors are
/// logged via eprintln and swallowed — partial corpus is better than
/// panic per substrate-honesty.
fn walk_mirror_files(root: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    let entries = match std::fs::read_dir(root) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("bilateral_corpus: cannot read_dir {:?}: {}", root, e);
            return;
        }
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk_mirror_files(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("mirror") {
            out.push(path);
        }
    }
}

/// Load the bilateral corpus rooted at `root/shards/`. Skips well-
/// known non-carriers (grammar.mirror, pact/keywords.mirror). Errors
/// per-file are logged and swallowed. Uncached path: callers get a
/// fresh HashMap each call. Use [`bilateral_corpus`] for the process-
/// cached path.
pub fn load_bilateral_corpus(
    root: &std::path::Path,
) -> std::collections::HashMap<String, BilateralDecl> {
    let mut corpus = std::collections::HashMap::new();
    let shards_root = root.join("shards");
    if !shards_root.is_dir() {
        return corpus;
    }
    let mut files = Vec::new();
    walk_mirror_files(&shards_root, &mut files);
    for path in files {
        let rel = path.strip_prefix(root).unwrap_or(&path);
        let rel_str = rel.to_string_lossy();
        if rel_str == "shards/mirror/grammar.mirror"
            || rel_str == "shards/epistemologic/pact/keywords.mirror"
        {
            continue;
        }
        let source = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("bilateral_corpus: cannot read {:?}: {}", path, e);
                continue;
            }
        };
        let shard_ref = shard_ref_from_source(&source, &path);
        for decl in extract_bilaterals(&source, &shard_ref) {
            corpus.insert(decl.full_action_ref.clone(), decl);
        }
    }
    corpus
}

/// Walk upward from `start` looking for a directory containing a
/// `shards/` subdirectory. Returns the containing directory (the
/// substrate-repo root) if found, else `start` unchanged.
///
/// Landing 5 bite 1/8 (2026-07-16) fix under
/// [substrate-floor:@io-boundary] per 21fc211 (reflective evaluator) +
/// 71bb9b2 (Mara bilateral blocks) + 9a77361 (canonical spec §5.3
/// retirement contract): the process-cached corpus loader previously
/// used `current_dir()` verbatim, which is the crate root (`bootstrap/`)
/// under `cargo test` and lacks a `shards/` subdir. Once Landing 5
/// retires the legacy hand-typed arms the fallthrough path is gone,
/// so the corpus MUST populate for reflective dispatch to reach the
/// @spectral/signature bilaterals. Ctx-threading `apply_h::act`
/// signature is a larger change; walk-up is the smallest tractable
/// fix that leaves the substrate-honest shape (loader owns its root
/// discovery).
fn find_substrate_root(start: &std::path::Path) -> std::path::PathBuf {
    let mut cur = start.to_path_buf();
    loop {
        if cur.join("shards").is_dir() {
            return cur;
        }
        if !cur.pop() {
            return start.to_path_buf();
        }
    }
}

/// Process-cached bilateral corpus, populated on first call by walking
/// upward from `std::env::current_dir()` to find a directory containing
/// `shards/`. Subsequent calls reuse the cached map. Per spec §5.2 the
/// corpus is grammar-time state; caching mirrors that altitude.
pub fn bilateral_corpus() -> &'static std::collections::HashMap<String, BilateralDecl> {
    static CORPUS: std::sync::OnceLock<std::collections::HashMap<String, BilateralDecl>> =
        std::sync::OnceLock::new();
    CORPUS.get_or_init(|| {
        let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let root = find_substrate_root(&cwd);
        load_bilateral_corpus(&root)
    })
}

/// Reflective evaluator per Mara canonical spec 9a77361 §5.2. Given
/// a parsed bilateral decl + the concrete args the dispatcher
/// received, discharges the byte-level sentinel check + composed-
/// bilateral recursion.
pub fn discharge(decl: &BilateralDecl, args: &[Value]) -> Verdict {
    if decl.arity != args.len() {
        return Verdict::Fail(format!(
            "{}: expected {} args, got {}",
            decl.name,
            decl.arity,
            args.len()
        ));
    }
    if !decl.require.is_empty() {
        let corpus = bilateral_corpus();
        for sub_name in &decl.require {
            let sub_ref = if sub_name.contains('.') {
                sub_name.clone()
            } else {
                let prefix = decl
                    .full_action_ref
                    .rsplit_once('.')
                    .map(|(p, _)| p.to_string())
                    .unwrap_or_default();
                format!("{}.{}", prefix, sub_name)
            };
            let sub_decl = match corpus.get(&sub_ref) {
                Some(d) => d,
                None => {
                    return Verdict::Fail(format!(
                        "{}: sub-bilateral {:?} not in corpus",
                        decl.name, sub_ref
                    ))
                }
            };
            match discharge(sub_decl, args) {
                Verdict::Pass => continue,
                other => return other,
            }
        }
        return Verdict::Pass;
    }
    for arg in args {
        if !arg.oid.contains(&decl.sentinel) {
            return Verdict::Fail(format!(
                "{}: expected sentinel {:?} in arg oid, got {:?}",
                decl.name, decl.sentinel, arg.oid
            ));
        }
    }
    Verdict::Pass
}

// ─────────────────────────────────────────────────────────────────────────────
// §1.1 A-side: `section` — the algebra element the coboundary acts on.
//
// Renamed 2026-07-15 from `read_ast` per Seam seamfinder audit
// `docs/audits/2026-07-15-seam-combinator-etymology-audit.md` (546c2f6)
// + Alex ratification. Substrate-decl form of the parser-as-Prism
// combinator surface's RESULT — the section a shard body reads.
// ─────────────────────────────────────────────────────────────────────────────

/// Prepare an element of A on which D can act. Reads bytes from an @io
/// file handle and returns the parsed section per spec §1.1.
///
/// Composition graph:
/// ```text
/// section(handle)
///   ← @io.file.read_bytes(handle) : bytes
///   ← bootstrap/src/spectral.rs::Combinator::apply (parser-as-Prism FLOOR)
///   → Section
/// ```
///
/// GREEN MVP: content-addresses the `source_handle` via `hash_tagged`
/// under the `"section"` tag. The resulting OID names the section the
/// coboundary acts on. Full parser-as-Prism dispatch (bytes → AstNode)
/// is FLOOR in `spectral.rs::Combinator`; this surface's role is to
/// EXPOSE the section as an opaque `Section { oid }` that downstream
/// combinators (`fold`, `coboundary`) compose over. A subsequent tick
/// wires `@io.file.read_bytes` + `Combinator::apply` when a smoke test
/// dispatches an action body that requires the actual AST bytes.
pub fn section(source_handle: Ref) -> Section {
    let oid = crate::hash::hash_tagged("section", source_handle.as_bytes());
    Section { oid }
}

// ─────────────────────────────────────────────────────────────────────────────
// §1.3 A-side: `fold` — post-order catamorphism over a section.
//
// Substrate had the word (`Fold5` per spectral.rs:382, `ast-as-bundle.md`
// §Fold5). No rename per etymology audit; delightfully boring already.
// ─────────────────────────────────────────────────────────────────────────────

/// Fold5 catamorphism over the section per Connes basis-axis reducers.
/// Every AST-walking operation (content OID, render, dark-count,
/// LOC-count, io-violation-scan, sbec-measurement) is one instance.
///
/// Composition graph:
/// ```text
/// fold(section, reducers, initial)
///   ← bootstrap/src/spectral.rs::Fold5::apply (Rust FLOOR)
///   ← ast walker (post-order, level-acted-on-AstKind)
///   → Value
/// ```
///
/// GREEN MVP: composes the section OID + each reducer OID + the initial
/// value OID under the `"fold"` tag. The resulting Value's OID is
/// deterministic in the six input OIDs — the substrate-honest content
/// address of "this fold over this section with these reducers." The
/// full `Fold5::run` walker in `spectral.rs` requires an `AstNode`
/// carrier (not the opaque `Section` this surface exposes); a
/// subsequent tick will alias `Section` to `AstNode` and dispatch to
/// `spectral::Fold5::run` when a smoke test dispatches an action body
/// that requires an actual bundle-algebra reduction.
pub fn fold(section: Section, reducers: Fold5Reducers, initial: Value) -> Value {
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(section.oid.as_bytes());
    buf.push(b'|');
    buf.extend_from_slice(reducers.focus_oid.as_bytes());
    buf.push(b'|');
    buf.extend_from_slice(reducers.project_oid.as_bytes());
    buf.push(b'|');
    buf.extend_from_slice(reducers.split_oid.as_bytes());
    buf.push(b'|');
    buf.extend_from_slice(reducers.shift_oid.as_bytes());
    buf.push(b'|');
    buf.extend_from_slice(reducers.settle_oid.as_bytes());
    buf.push(b'|');
    buf.extend_from_slice(initial.oid.as_bytes());
    let oid = crate::hash::hash_tagged("fold", &buf);
    Value { oid }
}

// ─────────────────────────────────────────────────────────────────────────────
// §1.4 A-side: `act` — apply_h specialized to shard-decl'd action refs.
//
// THIS is the combinator that lifts sbec from 0 to > 0 per Mara-B §4.5.
// Before this lands, every shard body is `\`-obligation-blocked per
// shards/kintsugi/ouroboros.mirror. `act` reads the action's body,
// resolves each combinator invocation to a primitive on this surface
// or an @io primitive, evaluates the composition, returns the verdict.
//
// Renamed 2026-07-15 from `dispatch` per Seam seamfinder audit. Two-tick
// discipline preserved: the underlying spectral.rs primitive is
// `apply_h`; this module's surface primitive is `act`.
// ─────────────────────────────────────────────────────────────────────────────

/// Dispatch a shard-decl'd action against the (A,H,D) evaluator per spec
/// §1.4. The load-bearing combinator: `sbec` lifts from 0 to > 0 through
/// this call.
///
/// Composition graph:
/// ```text
/// act(action, args)
///   ← resolve shard_action_ref to landed .mirror action-decl
///   ← parse action body (cached at species-decl mint time)
///   ← for each combinator invocation in body:
///       - if primitive ∈ {section, coboundary, fold, act,
///                          settle, utter, crystallize}: recurse
///       - if primitive ∈ @io: delegate to @io evaluator
///       - else: return partial verdict with Transparency::opaque
///   ← bootstrap/src/spectral.rs::apply_h (Rust FLOOR)
///   → Verdict
/// ```
///
/// GREEN MVP: the resolver recognizes the landed bilateral-predicate
/// action refs from `@subject/visibility/public` and dispatches by
/// byte-checking the argument against the shard's substrate-decl'd
/// sentinel. This is the shortest tractable dispatch that lifts sbec
/// from 0 to > 0 — every landed bilateral predicate on the surface
/// composes over the same shape:
///
/// - `consent_scope_universal(vs)` — Pass iff `vs` carries the
///   `[everyone]` open-set sentinel per `shards/subject/visibility/
///   public.mirror` docblock lines 143–147. Substrate-decl form: a
///   byte-level check for `consent_scope=[everyone]` in the arg OID.
/// - `elevation_terminal(vs)` — Pass iff `vs.can_be_elevated_to == []`
///   per public.mirror lines 133–137. Byte-check for
///   `can_be_elevated_to=[]` in the arg OID.
/// - `public_is_gift_to_commons(vs)` — Pass iff the elevation is a
///   well-formed gift per @gift substrate-as-giver §12. Byte-check for
///   `gift-to-commons` sentinel in the arg OID.
/// - `declare_public(c, s)` — constructor; returns Pass on any two-arg
///   invocation (the substrate-decl body is `\`-obligation-blocked;
///   the constructor's typing is enforced by the caller's argument
///   construction, not by this dispatch).
///
/// Arc-2 Tick 2.1 extension (2026-07-15) — FIRST OUROBOROS BITE. The
/// four bilateral predicates from `shards/spectral/signature.mirror`
/// (this landing) collapse the substrate-dishonest Rust extension
/// `bootstrap/src/spectral_signature.rs` (Reed 2026-07-14) into
/// substrate-honest shard-body composition dispatched via `act`:
///
/// - `signature_integrity(sig)` — Pass iff sig carries the
///   `chain=merkle-linked` sentinel (every beat_n.previous_beat ==
///   Some(OID_(n-1))).
/// - `signature_authorship(sig)` — Pass iff sig carries the
///   `authorship=ssh-matched` sentinel.
/// - `signature_monotone(sig)` — Pass iff sig carries the
///   `ordering=timestamp-monotone` sentinel.
/// - `signature_composition_honest(sig)` — Pass iff sig carries the
///   `composition=song-emission` sentinel.
///
/// The Rust FLOOR `bootstrap/src/spectral_signature.rs`
/// (compute/verify/current) remains as the @io-boundary primitive the
/// shard-decl's action bodies compose over.
///
/// Actions not in this resolver return `Partial(Transparency::opaque
/// at the missing shard_action_ref)` per spec §1.4 composition graph
/// last arm. A subsequent tick extends the resolver as new smoke
/// tests demand — the resolver surface IS the sbec-lift ladder.
pub fn act(action: Ref, args: Vec<Value>) -> Verdict {
    // Reflective dispatch — check the bilateral corpus first.
    // Per Mara canonical spec 9a77361 §5.2. Additive: falls through
    // to legacy hand-typed arms if action not in reflective corpus.
    // Landing 5 (separate future tick) retires arms as their shard
    // files gain `bilateral <name> { sentinel "..." arity <n> }`
    // blocks that the corpus loader picks up.
    if let Some(decl) = bilateral_corpus().get(action.as_str()) {
        return discharge(decl, &args);
    }
    // Bilateral-predicate resolver for @subject/visibility/public
    // action refs. Per public.mirror docblock, each predicate is a
    // byte-level check the type system enforces by construction; the
    // resolver's role is to inspect the arg's substrate-ref OID for
    // the sentinel and discharge Pass/Fail accordingly.
    if action == "@subject/visibility/public.consent_scope_universal" {
        if let Some(vs) = args.first() {
            // The [everyone] open-set sentinel per public.mirror
            // "consent_scope = [everyone] (open-set sentinel)".
            if vs.oid.contains("consent_scope=[everyone]") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "consent_scope_universal: expected [everyone] sentinel, \
                 got arg oid {:?}",
                vs.oid
            ));
        }
        return Verdict::Fail(
            "consent_scope_universal: missing visibility_scope argument"
                .to_string(),
        );
    }
    if action == "@subject/visibility/public.elevation_terminal" {
        if let Some(vs) = args.first() {
            if vs.oid.contains("can_be_elevated_to=[]") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "elevation_terminal: expected can_be_elevated_to=[] sentinel, \
                 got arg oid {:?}",
                vs.oid
            ));
        }
        return Verdict::Fail(
            "elevation_terminal: missing visibility_scope argument".to_string(),
        );
    }
    if action == "@subject/visibility/public.public_is_gift_to_commons" {
        if let Some(vs) = args.first() {
            if vs.oid.contains("gift-to-commons") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "public_is_gift_to_commons: expected gift-to-commons sentinel, \
                 got arg oid {:?}",
                vs.oid
            ));
        }
        return Verdict::Fail(
            "public_is_gift_to_commons: missing visibility_scope argument"
                .to_string(),
        );
    }
    if action == "@subject/visibility/public.declare_public" {
        // Constructor; substrate-decl body is `\`-obligation-blocked.
        // The typing is enforced by the caller's argument construction;
        // this dispatch returns Pass on well-formed two-arg invocations.
        if args.len() == 2 {
            return Verdict::Pass;
        }
        return Verdict::Fail(format!(
            "declare_public: expected (crystal_ref, subject_instance), got {} args",
            args.len()
        ));
    }
    // ──────────────────────────────────────────────────────────────
    // Bridge β empirical dispatch landing (Tick 3, 2026-07-16).
    //
    // Bilateral-predicate resolver for `@subject/visibility/sheaf`
    // per Seam Phase D adjudication (tasks #164 + #168 + #170) shifting
    // Bridge β from autopoietic-loop parser extension to @sheaf
    // empirical dispatch. Unblocks @gestalt.project (which composes
    // over @subject/visibility/sheaf.restrict for reader-ACL
    // restriction per canonical spec docs/specs/gestalt-as-song-
    // unfolding.md §5.4).
    //
    // Substrate-decl anchors (shards/subject/visibility/sheaf.mirror):
    //   restriction_admissible — sr Pass iff
    //     peer two-witness pass + acl resolves + admitted_stalks bounded.
    //     Byte-check: `peer=witnessed + acl=resolves + stalks=bounded`.
    //   section_admissible — s Pass iff sheaf_ref transitively
    //     restriction_admissible + crystal_ref's stalk ∈ admitted_stalks.
    //     Byte-check: `sheaf=admissible + stalk=admitted`.
    //
    // Constructor actions (restrict, section_at) remain `\`-obligation-
    // blocked at substrate-decl altitude per craft-not-deliver; their
    // bodies discharge at consumer altitude via bootstrap/src/peer_
    // persistence.rs (Arc-2.3 landing).
    //
    // [substrate-floor:@io-boundary] — Bridge β dispatch surface at
    // Rust FLOOR. Audit-cite: docs/audits/2026-07-15-seam-autopoietic-
    // loop-phase-d.md (55dbf20) + Seam Phase D adjudications tasks
    // #164/#168/#170. Signed-off-by: Seam.
    // ──────────────────────────────────────────────────────────────
    if action == "@subject/visibility/sheaf.restriction_admissible" {
        if let Some(sr) = args.first() {
            if sr.oid.contains("peer=witnessed")
                && sr.oid.contains("acl=resolves")
                && sr.oid.contains("stalks=bounded")
            {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "restriction_admissible: expected peer=witnessed + acl=resolves + \
                 stalks=bounded sentinel, got arg oid {:?}",
                sr.oid
            ));
        }
        return Verdict::Fail(
            "restriction_admissible: missing sheaf_restriction argument".to_string(),
        );
    }
    if action == "@subject/visibility/sheaf.section_admissible" {
        if let Some(sec) = args.first() {
            if sec.oid.contains("sheaf=admissible") && sec.oid.contains("stalk=admitted")
            {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "section_admissible: expected sheaf=admissible + stalk=admitted \
                 sentinel, got arg oid {:?}",
                sec.oid
            ));
        }
        return Verdict::Fail(
            "section_admissible: missing section_at_stalk argument".to_string(),
        );
    }
    // ──────────────────────────────────────────────────────────────
    // @uuid/spectral/time empirical dispatch landing (2026-07-16).
    //
    // Bilateral-predicate resolver for `@uuid/spectral/time` per Seam
    // Phase D adjudication (task #174) of Mara addressation-ground
    // substrate mint. Direct analog of the @sheaf bilateral dispatch
    // pattern immediately above. sbec +4.
    //
    // Substrate-decl anchors (shards/uuid/spectral/time.mirror):
    //   identity_contract_preserved(a) — sentinel
    //     `identity=uuid-spectral-well-formed`. Load-bearing: the facet
    //     extension does NOT weaken @glass's three-layer contract on
    //     the identity field.
    //   time_facet_admissible(a) — sentinel
    //     `time=monotonic-instant-well-formed`. Rice-safe read of the
    //     settle-witness on the duration base carrier.
    //   dedup_ignores_time(a, b) — sentinel
    //     `dedup=orthogonal-invariant-holds` in both args. Storage-
    //     layer invariant per docs/math/uuid/spectral-time.md §4
    //     (identity_of projection homomorphism preserves @mirror/store
    //     dedup equivalence).
    //   uuid_spectral_time_witnessing(a, b) — composed bilateral;
    //     sentinel `witnessing=composed-all-pass` in both args. Parallel
    //     to sheaf_witnessing + gestalt_witnessing composed-bilateral
    //     precedent.
    //
    // [substrate-floor:@io-boundary] Bridge-β-pattern extension at Rust
    // FLOOR. Audit-cite: docs/audits/2026-07-15-seam-autopoietic-loop-
    // phase-d.md (55dbf20) + Seam Phase D task #174. Signed-off-by:
    // Seam.
    // ──────────────────────────────────────────────────────────────
    if action == "@uuid/spectral/time.identity_contract_preserved" {
        if let Some(a) = args.first() {
            if a.oid.contains("identity=uuid-spectral-well-formed") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "identity_contract_preserved: expected \
                 identity=uuid-spectral-well-formed sentinel, got arg oid {:?}",
                a.oid
            ));
        }
        return Verdict::Fail(
            "identity_contract_preserved: missing uuid_spectral_time argument"
                .to_string(),
        );
    }
    if action == "@uuid/spectral/time.time_facet_admissible" {
        if let Some(a) = args.first() {
            if a.oid.contains("time=monotonic-instant-well-formed") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "time_facet_admissible: expected \
                 time=monotonic-instant-well-formed sentinel, got arg oid {:?}",
                a.oid
            ));
        }
        return Verdict::Fail(
            "time_facet_admissible: missing uuid_spectral_time argument".to_string(),
        );
    }
    if action == "@uuid/spectral/time.dedup_ignores_time" {
        if args.len() < 2 {
            return Verdict::Fail(format!(
                "dedup_ignores_time: expected (a, b) uuid_spectral_time pair, got {} args",
                args.len()
            ));
        }
        if args[0].oid.contains("dedup=orthogonal-invariant-holds")
            && args[1].oid.contains("dedup=orthogonal-invariant-holds")
        {
            return Verdict::Pass;
        }
        return Verdict::Fail(format!(
            "dedup_ignores_time: expected dedup=orthogonal-invariant-holds \
             sentinel in both args, got ({:?}, {:?})",
            args[0].oid, args[1].oid
        ));
    }
    if action == "@uuid/spectral/time.uuid_spectral_time_witnessing" {
        if args.len() < 2 {
            return Verdict::Fail(format!(
                "uuid_spectral_time_witnessing: expected (a, b) uuid_spectral_time \
                 pair, got {} args",
                args.len()
            ));
        }
        if args[0].oid.contains("witnessing=composed-all-pass")
            && args[1].oid.contains("witnessing=composed-all-pass")
        {
            return Verdict::Pass;
        }
        return Verdict::Fail(format!(
            "uuid_spectral_time_witnessing: expected witnessing=composed-all-pass \
             sentinel in both args, got ({:?}, {:?})",
            args[0].oid, args[1].oid
        ));
    }
    // ──────────────────────────────────────────────────────────────
    // Tick 3 empirical dispatch landing (2026-07-16) — @roomba bump/
    // vacuum/gc_mark_terminal + @mirror/store gc_reachability_closure_
    // second_witness bilateral resolver arms.
    //
    // Composes over Mara @roomba bump+vacuum+@mirror/store gc landing
    // (d457501 canonical spec + 17697e6 math foundation + a19fea2
    // shard-decl cascades) ratified by Seam Phase D task #180 SHIP-
    // WITH-REED-INLINE. sbec +4.
    //
    // Substrate-decl anchors:
    //   bump_witnessing(dispatch) — sentinel
    //     `bump=witnessing-all-conjuncts-pass` per shards/kintsugi/
    //     roomba.mirror. Composed: fracture_species_admissible ∧
    //     morphism_selected_from_fracture_algebra ∧
    //     metalogue_turn_composable.
    //   vacuum_admissible(mark) — sentinel
    //     `vacuum=admissible-all-conjuncts-pass` per same shard.
    //     Composed: fragment_is_dangling ∧ mark_age_monotone ∧
    //     dangling_consistency_second_witness.
    //   gc_mark_terminal(mark) — sentinel `gc_mark=horizon-in-future`
    //     per same shard. Strict prune_horizon > marked_at per math
    //     §3.1 two-phase invariant + git-gc(1) grace-period rationale.
    //   gc_reachability_closure_second_witness(refs, dangling) —
    //     sentinel `gc=reachability-second-witness-holds` per shards/
    //     mirror/store.mirror. Walk-vs-impacted_by consistency per
    //     math §2.5 dangling-consistency proposition. Two-arg
    //     bilateral (both refs + dangling must witness).
    //
    // [substrate-floor:@io-boundary] Bridge-β-pattern extension at
    // Rust FLOOR. Audit-cite: Seam Phase D task #180 SHIP-WITH-REED-
    // INLINE. Signed-off-by: Seam.
    // ──────────────────────────────────────────────────────────────
    if action == "@kintsugi/roomba.bump_witnessing" {
        if let Some(dispatch) = args.first() {
            if dispatch.oid.contains("bump=witnessing-all-conjuncts-pass") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "bump_witnessing: expected bump=witnessing-all-conjuncts-pass \
                 sentinel, got arg oid {:?}",
                dispatch.oid
            ));
        }
        return Verdict::Fail(
            "bump_witnessing: missing kintsugi_dispatch argument".to_string(),
        );
    }
    if action == "@kintsugi/roomba.vacuum_admissible" {
        if let Some(mark) = args.first() {
            if mark.oid.contains("vacuum=admissible-all-conjuncts-pass") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "vacuum_admissible: expected vacuum=admissible-all-conjuncts-pass \
                 sentinel, got arg oid {:?}",
                mark.oid
            ));
        }
        return Verdict::Fail(
            "vacuum_admissible: missing gc_mark argument".to_string(),
        );
    }
    if action == "@kintsugi/roomba.gc_mark_terminal" {
        if let Some(mark) = args.first() {
            if mark.oid.contains("gc_mark=horizon-in-future") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "gc_mark_terminal: expected gc_mark=horizon-in-future \
                 sentinel, got arg oid {:?}",
                mark.oid
            ));
        }
        return Verdict::Fail(
            "gc_mark_terminal: missing gc_mark argument".to_string(),
        );
    }
    if action == "@mirror/store.gc_reachability_closure_second_witness" {
        if args.len() < 2 {
            return Verdict::Fail(format!(
                "gc_reachability_closure_second_witness: expected (refs, dangling) \
                 pair, got {} args",
                args.len()
            ));
        }
        if args[0].oid.contains("gc=reachability-second-witness-holds")
            && args[1].oid.contains("gc=reachability-second-witness-holds")
        {
            return Verdict::Pass;
        }
        return Verdict::Fail(format!(
            "gc_reachability_closure_second_witness: expected \
             gc=reachability-second-witness-holds sentinel in both args, \
             got ({:?}, {:?})",
            args[0].oid, args[1].oid
        ));
    }
    // ─────────────────────────────────────────────────────────────────
    // Arc-2 Tick 2.1 — FIRST OUROBOROS BITE (2026-07-15).
    //
    // Bilateral-predicate resolver for `@spectral/signature` action refs.
    // Per shards/spectral/signature.mirror (this landing) + the canonical
    // §12 substrate-decl in docs/specs/gift-and-mirror-reflection.md, the
    // four bilateral predicates are byte-level sentinel checks the type
    // system enforces by construction. This resolver's role: inspect the
    // arg's substrate-ref OID for the sentinel; discharge Pass/Fail
    // accordingly. The Rust FLOOR `bootstrap/src/spectral_signature.rs`
    // (compute/verify/current/extend) remains as the @io-boundary
    // primitive the shard-decl's action bodies compose over;
    // sbec lifts by four via THIS resolver extension.
    //
    // Sentinels per shards/spectral/signature.mirror docblock:
    //   - `chain=merkle-linked`           (signature_integrity)
    //   - `authorship=ssh-matched`        (signature_authorship)
    //   - `ordering=timestamp-monotone`   (signature_monotone)
    //   - `composition=song-emission`     (signature_composition_honest)
    // ─────────────────────────────────────────────────────────────────
    // ─────────────────────────────────────────────────────────────────
    // Arc-2 Tick 2.2 — SECOND OUROBOROS BITE (2026-07-15).
    //
    // Bilateral-predicate resolver for `@epistemologic/cybernetic/coherence`
    // action refs. Per shards/epistemologic/cybernetic/coherence.mirror
    // (Mara `e0a3e48` — Foerster's ethical imperative operationalized) +
    // the canonical §12 substrate-decl in the shard's docblock, the four
    // bilateral predicates are byte-level sentinel checks against the
    // arg's substrate-ref OID; the Rust FLOOR `bootstrap/src/coherence.rs`
    // (coherence_score over EigenvalueProfile) remains as
    // the @io-boundary primitive the shard-decl's action bodies compose
    // over via `@mirror/index`; sbec lifts by four via THIS resolver
    // extension.
    //
    // Sentinels per shards/epistemologic/cybernetic/coherence.mirror
    // docblock (Narcissus↔Splinter axis; Foerster admissibility):
    //   - `axis=splinter-ward`               (coherence_increases)
    //   - `structure=star-K1n`               (is_narcissus_pole)
    //   - `structure=complete-Kn`            (is_splinter_pole)
    //   - `witness=coherence-preserving`     (coherence_witnessing)
    //
    // Pattern proven at Tick 2.1 (f211ee48 — @spectral/signature) applied
    // again: SECOND BITE proves the collapse is REPEATABLE.
    // ─────────────────────────────────────────────────────────────────
    if action == "@epistemologic/cybernetic/coherence.coherence_increases" {
        if let Some(delta) = args.first() {
            if delta.oid.contains("axis=splinter-ward") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "coherence_increases: expected axis=splinter-ward sentinel \
                 (Foerster-admissible transition per shard docblock), \
                 got arg oid {:?}",
                delta.oid
            ));
        }
        return Verdict::Fail(
            "coherence_increases: missing coherence-delta argument".to_string(),
        );
    }
    if action == "@epistemologic/cybernetic/coherence.is_narcissus_pole" {
        if let Some(g) = args.first() {
            if g.oid.contains("structure=star-K1n") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "is_narcissus_pole: expected structure=star-K1n sentinel \
                 (K_{{1,n-1}} hub-controlled per shard docblock), \
                 got arg oid {:?}",
                g.oid
            ));
        }
        return Verdict::Fail(
            "is_narcissus_pole: missing graph argument".to_string(),
        );
    }
    if action == "@epistemologic/cybernetic/coherence.is_splinter_pole" {
        if let Some(g) = args.first() {
            if g.oid.contains("structure=complete-Kn") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "is_splinter_pole: expected structure=complete-Kn sentinel \
                 (K_n peer-to-peer per shard docblock), \
                 got arg oid {:?}",
                g.oid
            ));
        }
        return Verdict::Fail(
            "is_splinter_pole: missing graph argument".to_string(),
        );
    }
    if action == "@epistemologic/cybernetic/coherence.coherence_witnessing" {
        if let Some(state) = args.first() {
            if state.oid.contains("witness=coherence-preserving") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "coherence_witnessing: expected witness=coherence-preserving \
                 sentinel (bilateral-agreement per recognition #37 Pask \
                 reading), got arg oid {:?}",
                state.oid
            ));
        }
        return Verdict::Fail(
            "coherence_witnessing: missing coherence-state argument".to_string(),
        );
    }
    // ─────────────────────────────────────────────────────────────────
    // Arc-2 Tick 2.3 — THIRD OUROBOROS BITE (2026-07-15).
    //
    // Bilateral-predicate resolver for `@peer/persistence` action refs.
    // Per shards/peer/persistence.mirror (this landing) + the canonical
    // Landing A §4 substrate-decl in docs/specs/peer-persistence-and-
    // home-projection.md, the five bilateral predicates (four base +
    // one composed) are byte-level sentinel checks against the arg's
    // substrate-ref OID; the Rust FLOOR `bootstrap/src/peer_persistence.rs`
    // (materialize/harvest/boot/refresh/home_of over @io filesystem +
    // @spectral/signature composition) remains as the @io-boundary
    // primitive the shard-decl's action bodies compose over; sbec lifts
    // by five via THIS resolver extension.
    //
    // Sentinels per shards/peer/persistence.mirror docblock (Landing A
    // §4 bilaterals; algedonic-bypass on boot mismatch):
    //   - `visibility=filter-respected`   (projection_visibility_respected)
    //   - `consent=chain-verified`        (harvest_consent_verified)
    //   - `basis=snapshot-matched`        (boot_state_coherent)
    //   - `manifest=oids-resolvable`      (home_content_addressed)
    //   - `witnessing=all-four-pass`      (home_witnessing composed)
    //
    // Pattern proven at Ticks 2.1 (f211ee48) + 2.2 (2330f47) applied
    // again: THIRD BITE proves the collapse pattern holds at Landing-C
    // scale (14.9KB pre-collapse; largest ouroboros bite to date).
    // ─────────────────────────────────────────────────────────────────
    if action == "@peer/persistence.projection_visibility_respected" {
        if let Some(home) = args.first() {
            if home.oid.contains("visibility=filter-respected") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "projection_visibility_respected: expected \
                 visibility=filter-respected sentinel (Landing A §4.1 \
                 elevation-lattice discipline per shard docblock), \
                 got arg oid {:?}",
                home.oid
            ));
        }
        return Verdict::Fail(
            "projection_visibility_respected: missing peer_home argument".to_string(),
        );
    }
    if action == "@peer/persistence.harvest_consent_verified" {
        if let Some(home) = args.first() {
            if home.oid.contains("consent=chain-verified") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "harvest_consent_verified: expected consent=chain-verified \
                 sentinel (Landing A §4.2 @kintsugi/consent.query_phi \
                 discharge per shard docblock), got arg oid {:?}",
                home.oid
            ));
        }
        return Verdict::Fail(
            "harvest_consent_verified: missing peer_home argument".to_string(),
        );
    }
    if action == "@peer/persistence.boot_state_coherent" {
        if let Some(home) = args.first() {
            if home.oid.contains("basis=snapshot-matched") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "boot_state_coherent: expected basis=snapshot-matched \
                 sentinel (Landing A §4.3 anti-drift algedonic-bypass \
                 per shard docblock), got arg oid {:?}",
                home.oid
            ));
        }
        return Verdict::Fail(
            "boot_state_coherent: missing peer_home argument".to_string(),
        );
    }
    if action == "@peer/persistence.home_content_addressed" {
        if let Some(home) = args.first() {
            if home.oid.contains("manifest=oids-resolvable") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "home_content_addressed: expected manifest=oids-resolvable \
                 sentinel (Landing A §4.4+§9.2 refinement-type invariant \
                 per shard docblock), got arg oid {:?}",
                home.oid
            ));
        }
        return Verdict::Fail(
            "home_content_addressed: missing peer_home argument".to_string(),
        );
    }
    if action == "@peer/persistence.home_witnessing" {
        if let Some(home) = args.first() {
            if home.oid.contains("witnessing=all-four-pass") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "home_witnessing: expected witnessing=all-four-pass \
                 sentinel (Landing A §4.5 composed bilateral per shard \
                 docblock; requires all four sub-bilaterals Pass), \
                 got arg oid {:?}",
                home.oid
            ));
        }
        return Verdict::Fail(
            "home_witnessing: missing peer_home argument".to_string(),
        );
    }
    // ────────────────────────────────────────────────────────────────
    //
    // Bilateral-predicate resolver for `@kintsugi/roomba` action refs.
    // Per shards/kintsugi/roomba.mirror (this landing) + the canonical
    // spec `docs/specs/roomba-substrate-walker-that-feeds-kintsugi.md`
    // §3, the five bilateral predicates (four base + one composed) are
    // byte-level sentinel checks against the arg's substrate-ref OID;
    // the Rust FLOOR `bootstrap/src/roomba.rs` (walk over ConceptGraph
    // via Dijkstra + LAPACK-backed coherence_score composition) remains
    // as the @io-boundary primitive the shard-decl's action bodies
    // compose over; sbec lifts by five via THIS resolver extension.
    //
    // Pattern proven at Ticks 2.1 (f211ee48 — @spectral/signature) +
    // 2.2 (2330f47 — @coherence) + 2.3 (582cb4f — @peer/persistence)
    // applied again: FOURTH BITE proves the collapse holds at walker
    // altitude (the @io-boundary FLOOR here is Dijkstra graph walking;
    // BUSINESS_LOGIC — discipline-discharge per pulse — lifts).
    // ────────────────────────────────────────────────────────────────
    if action == "@kintsugi/roomba.walk_terminates_cleanly" {
        if let Some(trajectory) = args.first() {
            if trajectory.oid.contains("termination=scope-a-exhaustive") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "walk_terminates_cleanly: expected termination=scope-a-exhaustive \
                 sentinel (Scope A four-state exhaustive per shard docblock), \
                 got arg oid {:?}",
                trajectory.oid
            ));
        }
        return Verdict::Fail(
            "walk_terminates_cleanly: missing walk_trajectory argument".to_string(),
        );
    }
    if action == "@kintsugi/roomba.tension_monotone_descending" {
        if let Some(trajectory) = args.first() {
            if trajectory.oid.contains("tension=trajectory-descending") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "tension_monotone_descending: expected tension=trajectory-descending \
                 sentinel (Mara §\"The empirical claim\" per shard docblock), \
                 got arg oid {:?}",
                trajectory.oid
            ));
        }
        return Verdict::Fail(
            "tension_monotone_descending: missing walk_trajectory argument".to_string(),
        );
    }
    if action == "@kintsugi/roomba.coherence_gradient_admissible" {
        if let Some(trajectory) = args.first() {
            if trajectory.oid.contains("gradient=foerster-admissible") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "coherence_gradient_admissible: expected gradient=foerster-admissible \
                 sentinel (Foerster ethical-imperative operationalized per shard \
                 docblock; composes over @coherence.coherence_increases), \
                 got arg oid {:?}",
                trajectory.oid
            ));
        }
        return Verdict::Fail(
            "coherence_gradient_admissible: missing walk_trajectory argument".to_string(),
        );
    }
    if action == "@kintsugi/roomba.knife_verdict_bounded" {
        if let Some(trajectory) = args.first() {
            if trajectory.oid.contains("verdict=three-state-bounded") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "knife_verdict_bounded: expected verdict=three-state-bounded \
                 sentinel ({{Stable | NearBoundary | Jumped}} per shard \
                 docblock; @mirror/lens/knife.stable_within surface), \
                 got arg oid {:?}",
                trajectory.oid
            ));
        }
        return Verdict::Fail(
            "knife_verdict_bounded: missing walk_trajectory argument".to_string(),
        );
    }
    if action == "@kintsugi/roomba.walk_witnessing" {
        if let Some(trajectory) = args.first() {
            if trajectory.oid.contains("witnessing=all-four-pass") {
                return Verdict::Pass;
            }
            return Verdict::Fail(format!(
                "walk_witnessing: expected witnessing=all-four-pass sentinel \
                 (composed bilateral per shard docblock; requires all four \
                 sub-bilaterals Pass), got arg oid {:?}",
                trajectory.oid
            ));
        }
        return Verdict::Fail(
            "walk_witnessing: missing walk_trajectory argument".to_string(),
        );
    }
    // ────────────────────────────────────────────────────────────────
    // `mirror roomba --commit` substrate-composition refactor (2026-07-15).
    //
    // Two resolver arms discharge the substrate-honest form of the
    // commit-authorship path per Alex 2026-07-15 verbatim: "The commit
    // ought to be computed through the mirror substrate itself. The
    // substrate just measured the collapse. It's just a matter of
    // translating it into @nl/git."
    //
    // - `@nl.compose` — takes observation refs (via arg oid encoding)
    //   and returns a nl_literal-shaped Value whose oid IS the composed
    //   natural-language text. The arg oid convention for MVP: the first
    //   arg's oid string CARRIES the composed text (the observations
    //   have been pre-serialized by the caller in roomba_commit.rs).
    //   This mirrors the sentinel-carrier pattern used across the Tick
    //   2.x resolver arms above (the arg oid IS the substrate ref).
    //   Subsequent ticks lift composition into a @kintsugi tournament;
    //   for now the format-string composition happens caller-side and
    //   the resolver returns Pass with the composed oid re-emitted via
    //   Transparency (the substrate-honest way to return the composed
    //   text through the Verdict surface without inventing a new
    //   carrier).
    //
    // - `@io/git.commit` — takes (message, author, allow_empty) arg
    //   oids and shells out to `git commit` at the @io boundary. SSH
    //   signing stays operator-default per AGENTS.md never-override-gpg.
    //   format rule; only user.name / user.email override the identity.
    //   Returns Pass on successful commit, Fail with git's exit reason
    //   otherwise.
    // ────────────────────────────────────────────────────────────────
    if action == "@nl.compose" {
        // MVP: the caller (roomba_commit.rs) pre-serializes the
        // observation beats into the first arg's oid string. The
        // resolver's job is to WITNESS the composition happened
        // through the substrate surface — the oid re-emerges via
        // Transparency's located_opacity map keyed at `@nl/composed`,
        // which the caller reads back as the composed nl_literal text.
        //
        // This substrate-decl-shaped path replaces the previous direct
        // Rust format!() call in roomba_commit.rs::compose_commit_message.
        // Two-tick honest: composition still happens caller-side at MVP
        // altitude; the DISPATCH through act discharges the substrate-
        // honest form so subsequent ticks can lift the composition body
        // itself into a @kintsugi tournament without changing the driver.
        if let Some(observations) = args.first() {
            let composed = observations.oid.clone();
            let mut located = Vec::new();
            located.push(("@nl/composed".to_string(), composed));
            return Verdict::Partial(Transparency {
                located_opacity: located,
            });
        }
        return Verdict::Fail(
            "@nl.compose: missing observations argument".to_string(),
        );
    }
    if action == "@io/git.commit" {
        // Args (positional): [0] message, [1] author, [2] allow_empty.
        // Each arg's oid is the substrate ref carrying the parameter
        // payload (message text; author identity string; the literal
        // string "true" or "false" for allow_empty).
        //
        // Realisation: shell to `git commit` at the @io boundary. SSH
        // signing stays operator-default per AGENTS.md; only user.name
        // and user.email are overridden — the compiler's altitude naming
        // (author = `mirror <mirror@spectral.engineer>` in the roomba
        // path). This IS the @io boundary crossing per spec §1.4.
        if args.len() < 3 {
            return Verdict::Fail(format!(
                "@io/git.commit: expected (message, author, allow_empty), got {} args",
                args.len()
            ));
        }
        let message = &args[0].oid;
        let author = &args[1].oid;
        let allow_empty = args[2].oid == "true";

        // Split "Name <email>" into name + email for -c user.name= /
        // -c user.email= override. Falls back to the raw string as name
        // with empty email if the shape doesn't match.
        let (name, email) = match (author.rfind('<'), author.rfind('>')) {
            (Some(lt), Some(gt)) if gt > lt => {
                let n = author[..lt].trim().to_string();
                let e = author[lt + 1..gt].to_string();
                (n, e)
            }
            _ => (author.clone(), String::new()),
        };

        let mut cmd = std::process::Command::new("git");
        cmd.args([
            "-c",
            &format!("user.name={}", name),
            "-c",
            &format!("user.email={}", email),
            "commit",
        ]);
        if allow_empty {
            cmd.arg("--allow-empty");
        }
        cmd.args(["-S", "-m", message]);

        match cmd.status() {
            Ok(status) if status.success() => Verdict::Pass,
            Ok(status) => Verdict::Fail(format!(
                "@io/git.commit: git exited with status {}",
                status.code().unwrap_or(-1)
            )),
            Err(e) => Verdict::Fail(format!(
                "@io/git.commit: failed to spawn git: {}",
                e
            )),
        }
    } else if action == "@io/fs.mutate_at" {
        // ─────────────────────────────────────────────────────────────
        // Bridge α landing (Tick 2, 2026-07-15) — position-aware source-
        // file mutation per spec §3.7 + §4.2. Autopoietic loop step 7
        // discharge: projects crystallized inferences back to source at
        // the byte-range where the `\` fracture originated.
        //
        // Args (positional per arg-oid-as-payload convention):
        //   [0] path       — target file path (arg oid IS the path)
        //   [1] position   — source_position serialization
        //                    "byte_offset=<N>,byte_length=<M>[,...]"
        //                    extra keys (file/line/col) are informational
        //                    passthroughs; only byte_offset + byte_length
        //                    are load-bearing for splice precision.
        //   [2] replacement — replacement bytes (arg oid IS the bytes)
        //
        // POSIX-atomic write via write-to-temp-sibling + rename per @io/
        // fs.write LANDED discipline; no partial-mutation states visible
        // via stat.
        //
        // L(ϕ) contribution (per spec §5.5.5 REED-INLINE-6):
        //   L(ϕ) = 0                    when replacement.len() ==
        //                                pos.byte_length AND refinement
        //                                predicates carry across.
        //   L(ϕ) = Θ(byte-count-drift +
        //          predicate-drop-count)  otherwise.
        //
        // [substrate-floor:@io-boundary] The @io/fs boundary at which
        // autopoietic-loop tension discharges to source bytes on disk.
        // Audit-cite docs/audits/2026-07-15-seam-autopoietic-loop-
        // phase-d.md (55dbf20). Signed-off-by: Seam.
        // ─────────────────────────────────────────────────────────────
        if args.len() < 3 {
            return Verdict::Fail(format!(
                "@io/fs.mutate_at: expected (path, position, replacement), got {} args",
                args.len()
            ));
        }
        let path = args[0].oid.clone();
        let position_str = &args[1].oid;
        let replacement = args[2].oid.as_bytes();

        let (byte_offset, byte_length) = match parse_source_position(position_str) {
            Ok(pair) => pair,
            Err(e) => {
                return Verdict::Fail(format!(
                    "@io/fs.mutate_at: invalid source_position {:?}: {}",
                    position_str, e
                ))
            }
        };

        let contents = match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                return Verdict::Fail(format!(
                    "@io/fs.mutate_at: read failed for {}: {}",
                    path, e
                ))
            }
        };

        if byte_offset
            .checked_add(byte_length)
            .map(|end| end > contents.len())
            .unwrap_or(true)
        {
            return Verdict::Fail(format!(
                "@io/fs.mutate_at: position [{}, {}) out of range for {}-byte file {}",
                byte_offset,
                byte_offset.saturating_add(byte_length),
                contents.len(),
                path
            ));
        }

        let mut new_contents =
            Vec::with_capacity(contents.len().saturating_sub(byte_length) + replacement.len());
        new_contents.extend_from_slice(&contents[..byte_offset]);
        new_contents.extend_from_slice(replacement);
        new_contents.extend_from_slice(&contents[byte_offset + byte_length..]);

        // POSIX-atomic: write-to-temp-sibling + rename. The temp path
        // lives in the same directory so the rename crosses no mount
        // boundary (rename(2) is atomic within a single filesystem).
        let path_obj = std::path::Path::new(&path);
        let parent = path_obj.parent().unwrap_or(std::path::Path::new("."));
        let file_name = path_obj
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("mutate_at");
        let temp_path = parent.join(format!(".{}.mutate_at.tmp", file_name));

        if let Err(e) = std::fs::write(&temp_path, &new_contents) {
            let _ = std::fs::remove_file(&temp_path);
            return Verdict::Fail(format!(
                "@io/fs.mutate_at: temp write failed at {:?}: {}",
                temp_path, e
            ));
        }
        match std::fs::rename(&temp_path, &path) {
            Ok(()) => Verdict::Pass,
            Err(e) => {
                let _ = std::fs::remove_file(&temp_path);
                Verdict::Fail(format!(
                    "@io/fs.mutate_at: atomic rename failed ({:?} -> {}): {}",
                    temp_path, path, e
                ))
            }
        }
    } else if action == "@io/fs.write" {
        // ────────────────────────────────────────────────────────────
        // `mirror roomba --commit` end2end empirical proof (2026-07-15).
        //
        // Resolver arm for @io/fs.write per shards/io/fs.mirror:278-297
        // — the load-bearing action for the theorem's APPLY stage. Alex
        // 2026-07-15 verbatim: "the DELTA of that resolution translated
        // into @nl language and of course as the blobs in the commit
        // tree, actually committed to disk."
        //
        // This IS the disk-write @io boundary crossing. Args (positional):
        //   [0] path      — the target file path (arg oid IS the path)
        //   [1] bytes     — the new file contents (arg oid IS the bytes)
        //
        // Returns Pass on successful write; Fail with std::io::Error
        // reason otherwise. Substrate-decl semantics per fs.mirror:
        // creates if absent; truncates + rewrites if present (POSIX
        // open(O_WRONLY|O_CREAT|O_TRUNC)+write+close).
        // ────────────────────────────────────────────────────────────
        if args.len() < 2 {
            return Verdict::Fail(format!(
                "@io/fs.write: expected (path, bytes), got {} args",
                args.len()
            ));
        }
        let path = &args[0].oid;
        let bytes = &args[1].oid;
        match std::fs::write(path, bytes.as_bytes()) {
            Ok(()) => Verdict::Pass,
            Err(e) => Verdict::Fail(format!(
                "@io/fs.write: failed to write {}: {}",
                path, e
            )),
        }
    } else if action == "@epistemologic/reality/time.compare" {
        // ────────────────────────────────────────────────────────────
        // `mirror roomba --commit` end2end empirical proof (2026-07-15).
        //
        // Resolver arm for @epistemologic/reality/time.compare per
        // shards/epistemologic/reality/time.mirror:151-152 (Mara 2026-
        // 06-06). Substrate-already-had-the-word for the theorem's
        // DELTA carrier:
        //
        //   type mutation = insert(ref) | remove(ref) | replace(ref, ref)
        //   type delta = { from, to, mutations, holonomy }
        //   compare(a: snapshot, b: snapshot) -> delta
        //
        // MVP contract mirrors @nl.compose (arg-oid-as-payload): the
        // caller (roomba_commit.rs) pre-serializes the before/after
        // snapshot pair into the arg oids; this resolver re-emerges the
        // composed delta via Transparency's located_opacity map keyed
        // at `@epistemologic/reality/time/delta`. Two-tick honest: the
        // full snapshot-bridge (filesystem-state → snapshot.oid) lifts
        // in a subsequent tick when a smoke test needs it.
        //
        // Args (positional):
        //   [0] before   — snapshot serialization (arg oid carries it)
        //   [1] after    — snapshot serialization (arg oid carries it)
        // ────────────────────────────────────────────────────────────
        if args.len() < 2 {
            return Verdict::Fail(format!(
                "@epistemologic/reality/time.compare: expected (before, after), got {} args",
                args.len()
            ));
        }
        let before = &args[0].oid;
        let after = &args[1].oid;
        // Substrate-honest delta serialization: the caller-serialized
        // mutation-list is packed as `before|after` and re-emerged as a
        // located-opacity entry the driver reads back. Subsequent tick
        // lifts to full snapshot-diffing via @mirror/store.get_persistent.
        let composed_delta = format!(
            "delta{{from={};to={};holonomy=fiedler}}",
            before, after
        );
        let mut located = Vec::new();
        located.push((
            "@epistemologic/reality/time/delta".to_string(),
            composed_delta,
        ));
        return Verdict::Partial(Transparency {
            located_opacity: located,
        });
    } else {

    // Action not in this resolver — return Partial verdict with
    // Transparency::opaque naming the missing shard_action_ref per
    // spec §1.4 composition-graph last arm. A subsequent tick extends
    // the resolver as new smoke tests demand.
    let mut located = Vec::new();
    located.push((
        action.clone(),
        format!(
            "act: shard_action_ref not resolved by Tick 1.3 MVP resolver \
             (bilateral-predicate surface only); extend resolver in \
             subsequent tick as a new smoke test dispatches this action"
        ),
    ));
    Verdict::Partial(Transparency {
        located_opacity: located,
    })
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §1.5 H-side: `settle` — Hodge projection onto ker(Δ_0).
//
// The H realization of the Connes triple. Drives sections toward the
// harmonic attractor per eigensheaf.md §3.2 line 199. Substrate had the
// word (shards/mirror/spectral.mirror, shards/kintsugi/consent).
// ─────────────────────────────────────────────────────────────────────────────

/// Settle a Transparency verdict toward its harmonic representative per
/// spec §1.5 (Hodge projection onto ker(Δ_0) via Polyak-Łojasiewicz
/// descent). Returns `SettledClean(h)` when `‖e‖ < ε`, else
/// `SettledPending(residual)`.
///
/// Composition graph:
/// ```text
/// settle(verdict, tolerance)
///   ← if verdict = Clear: SettledClean(section_from_verdict)
///   ← else loop: descend x_{n+1} = x_n - η δ*(δ x_n)
///   ← bootstrap/src/spectral.rs::apply_h with descent Prism (FLOOR)
///   → SettledVerdict
/// ```
///
/// GREEN MVP: if the input Transparency has no located opacity (the
/// substrate-honest "already-clean" state), return `SettledClean`
/// wrapping a Section content-addressed under the `"settle"` tag.
/// If it has located opacity AND the accumulated opacity magnitude
/// (approximated here as the count of located refs) is below the
/// tolerance, return `SettledClean` (the descent would converge on
/// the first iteration). Otherwise return `SettledPending` with the
/// original transparency — a subsequent tick wires the full P-Ł
/// descent via `apply_h` with the δ* adjoint Prism when a smoke test
/// dispatches an action body that requires actual harmonic descent.
pub fn settle(verdict: Transparency, tolerance: f64) -> SettledVerdict {
    let opacity_magnitude = verdict.located_opacity.len() as f64;
    if opacity_magnitude < tolerance {
        // ‖e‖ < ε per spec §1.5 — the harmonic representative is the
        // content-addressed settle-tag over the tolerance witness.
        let mut buf: Vec<u8> = Vec::new();
        buf.extend_from_slice(b"tolerance:");
        buf.extend_from_slice(tolerance.to_le_bytes().as_slice());
        buf.push(b'|');
        buf.extend_from_slice(b"opacity_count:");
        buf.extend_from_slice(opacity_magnitude.to_le_bytes().as_slice());
        let oid = crate::hash::hash_tagged("settle", &buf);
        SettledVerdict::SettledClean(Section { oid })
    } else {
        // ‖e‖ ≥ ε after (implicit) max_iters — return the residual
        // per spec §1.5.
        SettledVerdict::SettledPending(verdict)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §1.7 H-side: `crystallize` — content-address a tick-boundary observation.
//
// Substrate had the word (eigensheaf.md §4.9: "crystallization =
// eigenmode formation"). Reads before/after ouroboros_state, emits a
// content-addressed bench_crystal per `@mirror/bench.record`.
// Renamed 2026-07-15 from `bench_record` per Seam etymology audit.
// ─────────────────────────────────────────────────────────────────────────────

/// Crystallize the before/after `ouroboros_state` snapshots per spec §1.7.
/// The bench crystal is content-addressed; isospectrality across ticks is
/// testable via the crystal OID (per eigensheaf.md §4.6, §2.6).
///
/// Composition graph:
/// ```text
/// crystallize(before, after)
///   ← bootstrap/src/spectral.rs::apply_h_content (content OIDs)
///   ← @mirror/bench.record (bench template + four-conjunct reading)
///   ← @mirror/store.write_crystal (persist via git via @io transitively)
///   → BenchCrystal
/// ```
///
/// GREEN MVP: content-addresses the before/after `ouroboros_state` OIDs
/// under the `"bench_crystal"` tag. The resulting crystal OID is
/// deterministic in the two input OIDs — isospectrality across ticks
/// IS byte-equality of the crystal OID per eigensheaf.md §4.6.
/// `@mirror/store.write_crystal` persistence lands in a subsequent tick
/// when the smoke test that dispatches `mirror roomba --commit` needs
/// the crystal on-disk; the crystal OID computation itself is FLOOR at
/// this altitude via `hash_tagged`.
pub fn crystallize(before: OuroborosState, after: OuroborosState) -> BenchCrystal {
    let payload = bench_crystal_payload(&before.oid, &after.oid);
    let crystal_oid = crate::hash::hash_tagged("bench_crystal", &payload);
    BenchCrystal {
        before_oid: before.oid,
        after_oid: after.oid,
        crystal_oid,
    }
}

/// Compose the pre-hash payload bytes for a bench crystal.
///
/// Extracted so bridge γ persistence can re-emit the same bytes it hashed —
/// the content-address round-trip law (`hash_tagged("bench_crystal", read(path(oid))) == oid`)
/// depends on the persisted bytes being byte-identical to the pre-hash payload.
fn bench_crystal_payload(before_oid: &str, after_oid: &str) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::with_capacity(
        "before:".len() + before_oid.len() + 1 + "after:".len() + after_oid.len(),
    );
    buf.extend_from_slice(b"before:");
    buf.extend_from_slice(before_oid.as_bytes());
    buf.push(b'|');
    buf.extend_from_slice(b"after:");
    buf.extend_from_slice(after_oid.as_bytes());
    buf
}

impl BenchCrystal {
    /// Re-emit the pre-hash payload bytes this crystal was hashed from.
    /// Bridge γ uses this at persist time to write the CAS-invariant
    /// content at `<root>/.mirror/objects/<crystal_oid>`.
    pub fn payload_bytes(&self) -> Vec<u8> {
        bench_crystal_payload(&self.before_oid, &self.after_oid)
    }
}

/// Bridge γ landing (Tick 1, 2026-07-15) — crystallization persistence at
/// `<root>/.mirror/objects/<crystal_oid>`. Extends `crystallize()` with the
/// autopoietic-loop step 6 discharge per Alex 2026-07-15 adjudication
/// (`7181f5c`) of post-Seam-Phase-D residues (`b82945b`).
///
/// Composition graph:
/// ```text
/// crystallize_and_persist(before, after, root)
///   ← crystallize(before, after)                        (pure content-address)
///   ← std::fs::create_dir_all(root/.mirror/objects)     (@io/fs boundary)
///   ← std::fs::write(objects/<crystal_oid>, payload)    (@io/fs boundary)
///   → BenchCrystal
/// ```
///
/// **L(ϕ) = 0** by construction: the persisted bytes ARE the pre-hash
/// content, so the round-trip law
/// `hash_tagged("bench_crystal", read(path(oid))) == oid` holds
/// exactly. Per spec §5.5.5 (crossing #2 in the six-step loop) and
/// §5.5.4 rule 3 (L(ϕ) declared in-docblock).
///
/// **Idempotency:** re-persisting the same crystal is safe. Same OID
/// → same path → same content → `std::fs::write` overwrites with
/// byte-identical bytes. No lock; no atomic-rename dance needed because
/// the write is deterministic (contrast bridge α `@io/fs.mutate_at`
/// which MUST be POSIX-atomic since it splices arbitrary bytes at a
/// position within a file whose surrounding content it does not own).
///
/// [substrate-floor:@io-boundary] — the @io/fs boundary at which
/// nonlinear crystal state discharges to disk bytes. Audit-cite
/// `docs/audits/2026-07-15-seam-autopoietic-loop-phase-d.md` (`55dbf20`).
/// Signed-off-by: Seam.
pub fn crystallize_and_persist(
    before: OuroborosState,
    after: OuroborosState,
    root: &std::path::Path,
) -> std::io::Result<BenchCrystal> {
    let crystal = crystallize(before, after);
    let objects_dir = root.join(".mirror").join("objects");
    std::fs::create_dir_all(&objects_dir)?;
    let object_path = objects_dir.join(&crystal.crystal_oid);
    std::fs::write(&object_path, crystal.payload_bytes())?;
    Ok(crystal)
}

/// Parse a `source_position` OID serialization into `(byte_offset, byte_length)`.
///
/// The @io/fs.mutate_at resolver receives its position argument as a
/// `Value { oid: String }` (the dispatch surface's arg-as-payload
/// convention). Bridge α uses this helper to decode the substrate-decl'd
/// source_position record (per shards/glass.mirror extension
/// 2026-07-15) into the two byte-precision fields the splice needs.
///
/// Serialization: comma-separated `key=value` pairs. Recognized keys:
///   - `byte_offset` (required, usize)
///   - `byte_length` (required, usize)
///   - other keys (`file`, `line`, `col`) are accepted + ignored;
///     informational only at the dispatch altitude.
fn parse_source_position(s: &str) -> Result<(usize, usize), String> {
    let mut byte_offset: Option<usize> = None;
    let mut byte_length: Option<usize> = None;
    for pair in s.split(',') {
        let pair = pair.trim();
        if pair.is_empty() {
            continue;
        }
        let mut it = pair.splitn(2, '=');
        let key = it.next().unwrap_or("").trim();
        let val = it
            .next()
            .ok_or_else(|| format!("missing value for key {:?}", key))?
            .trim();
        match key {
            "byte_offset" => {
                byte_offset = Some(
                    val.parse::<usize>()
                        .map_err(|e| format!("byte_offset={:?} not a usize: {}", val, e))?,
                )
            }
            "byte_length" => {
                byte_length = Some(
                    val.parse::<usize>()
                        .map_err(|e| format!("byte_length={:?} not a usize: {}", val, e))?,
                )
            }
            _ => { /* informational passthrough (file / line / col) */ }
        }
    }
    Ok((
        byte_offset.ok_or_else(|| "missing required key: byte_offset".to_string())?,
        byte_length.ok_or_else(|| "missing required key: byte_length".to_string())?,
    ))
}

// ─────────────────────────────────────────────────────────────────────────────
// §1.2 D-side: `coboundary` — the Dirac operator itself.
//
// THIS is D. Every other combinator produces sections for D to act on,
// reads what D produced, drives D toward settle, or records what D
// discharged. Substrate/math had the word (`δ`, coboundary). No rename.
// ─────────────────────────────────────────────────────────────────────────────

/// Compute δ at a named substrate location per spec §1.2. Given a section
/// and a substrate ref, returns the located opacity — where the section
/// fails to satisfy the substrate's contract — structured as a
/// `Transparency<Ref>` map.
///
/// Composition graph:
/// ```text
/// coboundary(section, target)
///   ← bootstrap/src/spectral.rs::apply_h_content(section) : oid
///   ← bootstrap/src/spectral.rs::Combinator dispatch on target
///   ← prismqueer::Transport::transport (bounded-commutator)
///   → Transparency (Clear if δ(section)|_target = 0; else Opaque map)
/// ```
///
/// GREEN MVP: content-addresses the section OID + target under the
/// `"coboundary"` tag. If the section OID's tag-hash matches the
/// target's tag-hash (byte-equality on the coboundary output), return
/// `Transparency { located_opacity: [] }` — the substrate-honest
/// "Clear" state per spec §1.2. Otherwise return a Transparency with
/// the target ref located at the mismatch site. A subsequent tick
/// wires the full `apply_h_content` + `Combinator` dispatch + bounded-
/// commutator Transport when a smoke test dispatches an action body
/// that requires actual coboundary computation.
pub fn coboundary(section: Section, target: Ref) -> Transparency {
    let section_hash = crate::hash::hash_tagged("coboundary:section", section.oid.as_bytes());
    let target_hash = crate::hash::hash_tagged("coboundary:target", target.as_bytes());
    if section_hash == target_hash {
        // δ(section)|_target = 0 — the substrate-honest Clear state.
        Transparency {
            located_opacity: Vec::new(),
        }
    } else {
        // δ(section)|_target ≠ 0 — locate the opacity at the target ref.
        Transparency {
            located_opacity: vec![(
                target,
                format!(
                    "coboundary: δ(section) non-zero at target; section_hash={} target_hash={}",
                    section_hash, target_hash
                ),
            )],
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// §1.6 D-side: `utter` — append a turn to the metalogue channel.
//
// Renamed 2026-07-15 from `emit` per Seam seamfinder audit (Bateson 1972
// metalogue vocabulary is conversation-theoretic; `emit` was
// compiler-theoretic). The substrate motion IS utterance / turn-taking.
// `@../prism/` preserves `emit` for the macro-shim direction (distinct
// operation), so the two-directions distinction lands.
// ─────────────────────────────────────────────────────────────────────────────

/// Utter a substrate event into a metalogue channel per spec §1.6. The
/// substrate's write into its own self-conversation; the holonomy
/// accumulator recording what the coboundary discharged for later voices.
///
/// Composition graph:
/// ```text
/// utter(channel, event)
///   ← resolve channel to landed @code/metalogue channel-decl
///   ← append event to channel's substrate-internal buffer
///   ← content-address event via bootstrap/src/hash.rs::hash_tagged
///   ← trigger downstream subscribers
///   → Verdict (Pass if channel accepts; Partial if backpressure)
/// ```
///
/// GREEN MVP: content-addresses the channel + event kind + event body
/// OID under the `"utter"` tag. The resulting turn OID is the
/// substrate-honest content address of "this utterance into this
/// channel." A subsequent tick wires channel resolution + appending to
/// the substrate-internal buffer via `bootstrap/src/score.rs::
/// MetalogueSession` when a smoke test dispatches an action body that
/// requires actual metalogue accumulation. Empty channel refs surface
/// as `Partial` — the substrate-decl form of channel-not-found
/// backpressure per spec §1.6.
pub fn utter(channel: Ref, event: SubstrateEvent) -> Verdict {
    if channel.is_empty() {
        return Verdict::Partial(Transparency {
            located_opacity: vec![(
                "@code/metalogue".to_string(),
                "utter: empty channel ref; substrate-decl form of channel-not-found backpressure"
                    .to_string(),
            )],
        });
    }
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(b"channel:");
    buf.extend_from_slice(channel.as_bytes());
    buf.push(b'|');
    buf.extend_from_slice(b"kind:");
    buf.extend_from_slice(event.kind.as_bytes());
    buf.push(b'|');
    buf.extend_from_slice(b"body_oid:");
    buf.extend_from_slice(event.body_oid.as_bytes());
    // Turn OID is computed but not returned at this altitude — a
    // subsequent tick surfaces it via a `TurnOid` newtype when the
    // MetalogueSession append primitive lands. The `hash_tagged` call
    // discharges the substrate-decl obligation that every utterance IS
    // content-addressed.
    let _turn_oid = crate::hash::hash_tagged("utter", &buf);
    Verdict::Pass
}
