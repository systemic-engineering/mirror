//! `liquid.rs` — the property runtime.
//!
//! Fourth file of the five-file terminal FLOOR (per Mara Round 2 spec
//! extension `docs/specs/rust-floor-five-file-terminal-geometry-
//! extension.md`). Sits between compile.rs (compilation loop) and
//! matrix.rs (sub-Turing numerical primitives).
//!
//! ## Responsibility
//!
//! Read bilateral property declarations from `mirror.spec` + shard
//! bodies (via phone.rs `@io/fs.read`). Instantiate `LiquidVoid<T>`
//! witnesses at prismqueer altitude. Dispatch to pillar primitives.
//! Return `PropertyVerdict`. Consumed by compile.rs at each `@song/beat`
//! crystallization event.
//!
//! ## Per-file five-altitude discipline (Mara `81294b3` + Round 2)
//!
//! - main.rs      supervisor + delegation
//! - **liquid.rs  property runtime** (this file)
//! - compile.rs   compilation loop (SAGA-chain-of-Crystals)
//! - matrix.rs    sub-Turing numerical (LAPACK/BLAS/FLANG)
//! - phone.rs     @io boundary
//!
//! Each file has ONE responsibility. liquid.rs owns property-dispatch;
//! it does NOT own the compilation loop (compile.rs) OR the @io
//! crossing (phone.rs) OR the numerical primitives (matrix.rs). It
//! consumes all three via the module boundary.
//!
//! ## Substrate-decl composition
//!
//! Composes over Mara Round 3 landings (`c8215a3`+`7a334f7`+`9d443dd`+
//! `9cf07d2`+`eac2b30`):
//! - `@system` family-root marker (autopoietic bauchladen × time)
//! - `@mirror/spec/system` grammar species (VSM as .spec)
//! - `@aikido` + `@aikido/reflect` (runtime protocol; Tomm-shape metabolizer)
//! - `@peer/{reflect,redirect,reframe}` three-tier surface
//! - `@beam/system` (kills gen_prism as framing)
//!
//! And over viable.mirror (67th substrate-already-had-the-word;
//! complete Beer VSM property-altitude landing since 2026-07-17).
//!
//! ## Bilateral property declaration surface
//!
//! Every `bilateral <name> { sentinel "..." arity <n> require <ref>* }`
//! block across `shards/**/*.mirror` + `mirror.spec` is a
//! `PropertyDecl` at this altitude. Extraction is byte-scanning per
//! Mara §5.2 dispatch pseudocode (mirrors `bootstrap/src/apply_h.rs
//! ::extract_bilaterals` as FRESH REIMPLEMENTATION per
//! `collapse.rs` precedent — `rust/` doesn't depend on `bootstrap/`).
//!
//! ## Minimum-viable this iteration
//!
//! Iteration 2 of /loop cascade (Alex 2026-07-20). This file lands:
//! - `PropertyDecl` carrier (name + sentinel + arity + require list)
//! - `extract_properties(source: &str) -> Vec<PropertyDecl>` byte-scanner
//! - `dispatch_property(decl, args) -> PropertyVerdict` stub
//!   (composed dispatch through prismqueer::liquid::pillar arrives
//!   at iteration 3+ with the ~20 pillar predicates from the /loop
//!   prompt)
//!
//! Full spec-reading + shard-walking + real pillar dispatch lands at
//! iterations 3-5 of the cascade.
//!
//! ## Iter 2 (post-reframe 2026-07-21) — SpecProperty carrier lands alongside
//!
//! Per Alex 2026-07-21 reframe (`docs/loop/CURRENT.md`
//! §2026-07-21-ALEX-REFRAME, commit `16ddb12`) re-surfacing Mara
//! 2026-07-19 canonical spec `docs/specs/2026-07-19-mirror-spec-is-
//! the-fixpoint-liquid-is-the-runtime.md`: the `PropertyDecl` above
//! is actually a `BilateralDecl` by shape (shard-body altitude;
//! Mara §5.2 bullet 2). The spec-body `property { verifies { … }
//! domain @<T> samples <n> defer? <msg> }` grammar (declared at
//! `shards/mirror/spec/property.mirror`) is a DISTINCT carrier per
//! Mara §5.2 bullet 1.
//!
//! Two-tick discipline (per `shards/mirror/spec/system.mirror`
//! precedent for `project`→`system` migration):
//!
//! - **Tick 1 (this landing).** Mint `SpecProperty` carrier +
//!   `extract_spec_properties(source: &str) -> Vec<SpecProperty>`
//!   alongside existing PropertyDecl + extract_properties. Both
//!   shapes coexist at Rust altitude.
//! - **Tick 2 (forward-promised).** Rename `PropertyDecl` →
//!   `BilateralDecl` + `extract_properties` → `extract_bilaterals`;
//!   rename `SpecProperty` → `PropertyDecl` per Mara §5.2 naming.
//!   Update `compile.rs` consumer through mechanical rename.
//! - **Tick 3 (forward-promised).** Extend `dispatch_property` to
//!   route SpecProperty via §2.3 dispatch table (verifies-expression
//!   shape → pillar primitive; per Mara §2.3 landed dispatch table).
//!
//! Once Tick 3 lands + Reed's ~17 per-property RED files retire per
//! Mara §7.1, `rust/` bears FROZEN marker: mirror.spec declares VSM
//! shape + attached property specifications; Rust merely verifies
//! compliance per Adzic *Specification by Example* form.
//!
//! **Every iter asks**: does this tick shrink FLOOR toward FROZEN or
//! grow Rust? This landing GROWS SpecProperty carrier + one
//! extraction fn (~80 LOC) but ENABLES retirement of ~17 per-property
//! RED files at Tick 3+ per Mara §7.1. Net FLOOR shrink is
//! forward-promised at Tick 3+ landing.

/// A bilateral property declaration extracted from mirror.spec or a
/// shard body. Mirrors `bootstrap/src/apply_h.rs::BilateralDecl`
/// shape (fresh reimplementation per collapse.rs precedent).
///
/// **Naming note (2026-07-21):** per Alex reframe this carrier is
/// substrate-honestly a `BilateralDecl` (shard-body altitude per
/// Mara §5.2 bullet 2). Rename forward-promised at Tick 2. Sibling
/// spec-body carrier at [`SpecProperty`] lands this tick.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyDecl {
    /// Property name (e.g., "reflection_composes", "aikido_sequence_well_formed").
    pub name: String,
    /// Byte-substring sentinel the dispatch checks against argument OIDs.
    pub sentinel: String,
    /// Number of arguments this property takes.
    pub arity: usize,
    /// Composed-bilateral sub-predicates (each name is a `require`
    /// clause; the composed predicate is well-formed iff every
    /// required sub-predicate holds).
    pub require: Vec<String>,
    /// Source location (shard path + line number where the bilateral
    /// block starts). Empty when not tracked.
    pub source: String,
}

impl PropertyDecl {
    /// True iff this is a composed-bilateral (has require clauses).
    pub fn is_composed(&self) -> bool {
        !self.require.is_empty()
    }

    /// True iff this is a base-bilateral (sentinel-only; no require).
    pub fn is_base(&self) -> bool {
        self.require.is_empty() && !self.sentinel.is_empty()
    }
}

/// A spec-body property declaration extracted from `mirror.spec`.
///
/// Carrier for the `property <name> { verifies { <expr> } domain
/// @<T> samples <n> defer? <msg> }` grammar declared at
/// `shards/mirror/spec/property.mirror` (Mara 2026-07-19). Distinct
/// altitude from [`PropertyDecl`] (shard-body bilateral): spec-body
/// properties are project-level executable specifications per
/// Adzic *Specification by Example* form (spec IS the test, test
/// IS the spec).
///
/// Per Mara canonical spec `docs/specs/2026-07-19-mirror-spec-is-
/// the-fixpoint-liquid-is-the-runtime.md` §5.2 bullet 1: the
/// substrate-decl'd fields are `name`, `verifies_expression_tree`,
/// `domain_type_witness`, `samples_count`, `defer_annotation`.
/// This carrier holds the extracted-from-source shape; the
/// verifies-expression tree lives here as raw source-substring
/// (parsing to typed expression tree is Tick 3+ per Mara §3.2).
///
/// Alex 2026-07-21 verbatim (re-surfacing Mara 2026-07-19):
///
/// > "we need to cross the Liquid<T> threshold between Rust and
/// > Mirror. Because then we can start to project properties into
/// > Beer's VSM definition and attached as executable
/// > specificitions. Like in the book Specification as Example.
/// > Then the Rust can be frozen. And mirror becomes canonical."
///
/// This carrier IS the Rust-altitude landing of the threshold-
/// crossing shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecProperty {
    /// Property name (per `property <NAME> { … }` header).
    pub name: String,
    /// The raw source text between `verifies {` and its matching
    /// `}` (leading/trailing whitespace stripped). Parsed to typed
    /// expression tree at Tick 3+ per Mara canonical spec §3.2.
    /// Empty string when no verifies block present.
    pub verifies_source: String,
    /// The domain-type reference (per `domain @<Type>` directive).
    /// String form `"@Type"` including the leading `@`. Empty when
    /// no domain directive present.
    pub domain_ref: String,
    /// The samples count (per `samples <n>` directive). None when
    /// no samples directive present.
    pub samples: Option<usize>,
    /// The defer annotation message (per `defer <msg>` directive).
    /// None when property is not deferred; Some(msg) when deferred
    /// with the given message.
    pub defer_message: Option<String>,
    /// Source location (`line N` where the property block starts).
    /// Empty when not tracked.
    pub source: String,
}

impl SpecProperty {
    /// True iff this property carries a non-empty verifies body.
    /// A well-formed SpecProperty has a verifies body per Mara
    /// canonical spec §3.1 `verifies_expression_tree` field.
    pub fn has_verifies(&self) -> bool {
        !self.verifies_source.is_empty()
    }

    /// True iff this property is deferred (per `defer <msg>` in the
    /// spec source). Per Reed's `defer()`-mode pattern in
    /// `prism/prismqueer/tests/red_trust_chain_liquid_void.rs`.
    pub fn is_deferred(&self) -> bool {
        self.defer_message.is_some()
    }
}

/// Extract spec-body property declarations from a source text.
///
/// Byte-scans line-by-line for `property <name> { verifies { <expr> }
/// domain @<Type> samples <n> defer? <msg> }` blocks per the grammar
/// declared at `shards/mirror/spec/property.mirror` (Mara 2026-07-19).
/// Handles multi-line verifies blocks (nested braces tracked);
/// preserves declaration order.
///
/// This is the spec-body counterpart to [`extract_properties`]. Both
/// coexist at this altitude per two-tick discipline (rename cascade
/// forward-promised at Tick 2 per module-level docblock).
///
/// The verifies-expression tree is captured as raw source substring;
/// typed-expression-tree parsing forward-promised at Tick 3+ per
/// Mara canonical spec `docs/specs/2026-07-19-mirror-spec-is-the-
/// fixpoint-liquid-is-the-runtime.md` §3.2 (Rondon-Kawaguchi-Jhala
/// 2008 decidability grounding).
pub fn extract_spec_properties(source: &str) -> Vec<SpecProperty> {
    let mut out = Vec::new();
    let mut lines = source.lines().enumerate().peekable();
    while let Some((line_no, line)) = lines.next() {
        let trimmed = line.trim();
        // Look for `property <name> {` opener at spec-body altitude.
        let Some(rest) = trimmed.strip_prefix("property ") else {
            continue;
        };
        let Some(name) = rest.split_whitespace().next() else {
            continue;
        };
        if !rest.contains('{') {
            continue;
        }
        let name = name.to_string();
        let source_loc = format!("line {}", line_no + 1);
        let mut verifies_source = String::new();
        let mut domain_ref = String::new();
        let mut samples: Option<usize> = None;
        let mut defer_message: Option<String> = None;
        // Track outer `property { }` brace depth (opener already seen).
        let mut depth: usize = 1;
        while let Some((_, body_line)) = lines.next() {
            let body_trimmed = body_line.trim();
            // Detect `verifies {` opener; consume nested body preserving
            // inner brace nesting until the matching close.
            if let Some(v_rest) = body_trimmed.strip_prefix("verifies") {
                if v_rest.trim_start().starts_with('{') {
                    let mut verifies_lines: Vec<String> = Vec::new();
                    let mut v_depth: usize = 1;
                    // Capture any content after `verifies {` on same line.
                    let after_brace: String = v_rest
                        .trim_start()
                        .strip_prefix('{')
                        .unwrap_or("")
                        .to_string();
                    let after_trim = after_brace.trim();
                    if !after_trim.is_empty() && after_trim != "}" {
                        // Same-line content (e.g., `verifies { true }`).
                        if let Some(inner) = after_trim.strip_suffix('}') {
                            let inner_trim = inner.trim();
                            if !inner_trim.is_empty() {
                                verifies_lines.push(inner_trim.to_string());
                            }
                            v_depth = 0;
                        } else {
                            verifies_lines.push(after_trim.to_string());
                        }
                    } else if after_trim == "}" {
                        v_depth = 0;
                    }
                    while v_depth > 0 {
                        let Some((_, v_line)) = lines.next() else {
                            break;
                        };
                        let v_trimmed = v_line.trim();
                        // Track brace nesting inside verifies body.
                        for ch in v_trimmed.chars() {
                            match ch {
                                '{' => v_depth += 1,
                                '}' => {
                                    if v_depth > 0 {
                                        v_depth -= 1;
                                    }
                                }
                                _ => {}
                            }
                        }
                        if v_depth == 0 {
                            // Line closes verifies body; strip trailing `}`.
                            let closing_stripped = v_trimmed
                                .rsplit_once('}')
                                .map(|(pre, _)| pre.trim().to_string())
                                .unwrap_or_default();
                            if !closing_stripped.is_empty() {
                                verifies_lines.push(closing_stripped);
                            }
                        } else {
                            verifies_lines.push(v_trimmed.to_string());
                        }
                    }
                    verifies_source = verifies_lines.join("\n").trim().to_string();
                    continue;
                }
            }
            // Detect `domain @<Type>` directive.
            if let Some(d_rest) = body_trimmed.strip_prefix("domain ") {
                domain_ref = d_rest
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .to_string();
                continue;
            }
            // Detect `samples <n>` directive.
            if let Some(s_rest) = body_trimmed.strip_prefix("samples ") {
                if let Ok(n) = s_rest.trim().parse::<usize>() {
                    samples = Some(n);
                }
                continue;
            }
            // Detect `defer <msg>` directive; msg may be quoted.
            if let Some(f_rest) = body_trimmed.strip_prefix("defer ") {
                let raw = f_rest.trim();
                let msg = if let Some(q_start) = raw.find('"') {
                    raw[q_start + 1..]
                        .rfind('"')
                        .map(|q_end| raw[q_start + 1..q_start + 1 + q_end].to_string())
                        .unwrap_or_else(|| raw.trim_matches('"').to_string())
                } else {
                    raw.to_string()
                };
                defer_message = Some(msg);
                continue;
            }
            // Track outer property-block brace nesting.
            for ch in body_trimmed.chars() {
                match ch {
                    '{' => depth += 1,
                    '}' => {
                        if depth > 0 {
                            depth -= 1;
                        }
                    }
                    _ => {}
                }
            }
            if depth == 0 {
                break;
            }
        }
        out.push(SpecProperty {
            name,
            verifies_source,
            domain_ref,
            samples,
            defer_message,
            source: source_loc,
        });
    }
    out
}

/// Extract bilateral property declarations from a source text.
///
/// Byte-scans line-by-line for `bilateral <name> { sentinel "..."
/// arity <n> require <ref>* }` blocks. Handles multi-line blocks;
/// preserves declaration order.
///
/// This is the minimum-viable extractor — handles the substrate-decl'd
/// bilateral form used across ~30+ landed shards. Grammar edge cases
/// (nested blocks, comments-inside-blocks, etc.) get progressively
/// tightened as the substrate uncovers them.
pub fn extract_properties(source: &str) -> Vec<PropertyDecl> {
    let mut out = Vec::new();
    let mut lines = source.lines().enumerate().peekable();
    while let Some((line_no, line)) = lines.next() {
        let trimmed = line.trim();
        // Look for "bilateral <name> {" opener
        if let Some(rest) = trimmed.strip_prefix("bilateral ") {
            if let Some(name) = rest.split_whitespace().next() {
                if rest.contains('{') {
                    let name = name.to_string();
                    let source_loc = format!("line {}", line_no + 1);
                    let mut sentinel = String::new();
                    let mut arity: usize = 0;
                    let mut require: Vec<String> = Vec::new();
                    // Consume until closing brace
                    while let Some((_, body_line)) = lines.next() {
                        let body_trimmed = body_line.trim();
                        if body_trimmed.starts_with('}') {
                            break;
                        }
                        if let Some(s) = body_trimmed.strip_prefix("sentinel ") {
                            // sentinel "..." — extract content between quotes
                            if let Some(q_start) = s.find('"') {
                                if let Some(q_end) = s[q_start + 1..].find('"') {
                                    sentinel = s[q_start + 1..q_start + 1 + q_end].to_string();
                                }
                            }
                        } else if let Some(s) = body_trimmed.strip_prefix("arity ") {
                            if let Ok(n) = s.trim().parse::<usize>() {
                                arity = n;
                            }
                        } else if let Some(s) = body_trimmed.strip_prefix("require ") {
                            let ref_name = s.split_whitespace().next().unwrap_or("").to_string();
                            if !ref_name.is_empty() {
                                require.push(ref_name);
                            }
                        }
                    }
                    out.push(PropertyDecl {
                        name,
                        sentinel,
                        arity,
                        require,
                        source: source_loc,
                    });
                }
            }
        }
    }
    out
}

/// The verdict returned by property dispatch.
///
/// Deliberately narrow at iteration 2 — lifts to full
/// `terni::PropertyVerdict` shape when iteration 3 wires actual pillar
/// dispatch. Keeps liquid.rs testable in isolation without pulling the
/// full prismqueer/terni surface into main-binary scope this early.
#[derive(Debug, Clone, PartialEq)]
pub enum Verdict {
    /// Property discharged: all invariants hold.
    Pass,
    /// Property refused: at least one invariant violated. Message names
    /// the specific violation (Rice-safe diagnostic).
    Fail(String),
    /// Property deferred: cannot be discharged at this altitude;
    /// forward-promised to a specific downstream landing (message
    /// names the authorship territory).
    Defer(String),
}

impl Verdict {
    pub fn is_pass(&self) -> bool {
        matches!(self, Verdict::Pass)
    }
    pub fn is_fail(&self) -> bool {
        matches!(self, Verdict::Fail(_))
    }
    pub fn is_defer(&self) -> bool {
        matches!(self, Verdict::Defer(_))
    }
}

/// Dispatch a property declaration against argument OIDs.
///
/// Iteration 5 (Alex 2026-07-20 /loop cascade): dispatches to the
/// `pillar` submodule for named property predicates registered under
/// the classifier-witness framing (Recognition bundle #1:
/// `#R-autopoietic-classifier-is-knife-coord-under-lagrange-between-
/// narcissus-and-splinter`). Each pillar predicate is a classifier
/// witness that fires @subject-evidence (Pass) or @object-evidence
/// (Fail); Defer stays for unknown property names (authorship
/// territory forward-promised to future iters).
///
/// Signature stable since iter 2; only the body evolves as more
/// pillar predicates land at iter 6+.
pub fn dispatch_property(decl: &PropertyDecl, args: &[String]) -> Verdict {
    if args.len() != decl.arity {
        return Verdict::Fail(format!(
            "arity mismatch: property `{}` expects {} args, got {}",
            decl.name,
            decl.arity,
            args.len()
        ));
    }
    // Route named property to registered pillar predicate; fall through
    // to Defer for unknown names.
    pillar::dispatch(&decl.name, args)
}

/// Dispatch a spec-body SpecProperty to a Verdict.
///
/// Iter 3 of /loop cascade (Alex 2026-07-21 reframe post-2026-07-19
/// Mara canonical spec). First dispatch surface at Rust altitude for
/// the spec-body `property { verifies { <expr> } domain @<T> samples
/// <n> defer? <msg> }` grammar per Mara `docs/specs/2026-07-19-
/// mirror-spec-is-the-fixpoint-liquid-is-the-runtime.md` §2.3
/// dispatch table.
///
/// **Substrate framing (Alex 2026-07-21 in-transcript correction):**
/// SpecProperty is the SPEC-DECLARED OBLIGATION — what Liquid<T>
/// flows must satisfy. Liquid<T> itself is what flows through the
/// Fiber<T> of prismqueer's Bundle Tower (`Transport → Gauge →
/// Connection → Fiber → State` supertrait chain per
/// `prism/prismqueer/src/liquid.rs` module docblock §1). phone.rs
/// connects fibers via the @io boundary (@io/socket + @io/fs +
/// @io/git); Liquid<T> flows through those connections. This fn
/// evaluates the obligation for the Rice-safe cases where no
/// Fiber<T> sampling is needed (boolean literals + sentinel-
/// containment); the pillar::forall-invoking arms (algedonic /
/// viability / general expression tree per Mara §2.3 rows 1-3+6)
/// forward-promised at iter 4+ WILL sample domain-Arbitrary<T>
/// through Fiber<T>, sending Liquid<T> flow through phone.rs
/// connections.
///
/// Rust FROZEN target = rust/src/phone.rs @io connection surface
/// stabilizes + mirror.spec property declarations produce Verdicts
/// through Liquid<T>-flow-via-phone.rs-connections; Mirror fiber
/// becomes canonical source-of-truth for the flow topology.
///
/// Landed dispatch arms (this iter — 3 shapes; ACTUAL Verdict, not
/// stub Defer):
///
/// 1. **`defer` directive** — SpecProperty carries `defer_message`;
///    dispatch returns Defer(msg) regardless of verifies_source.
///    Per Mara Q-Mara-D + Reed's `defer()`-mode pattern in
///    `prism/prismqueer/tests/red_trust_chain_liquid_void.rs`.
///
/// 2. **Boolean literals** — `verifies { true }` → Pass;
///    `verifies { false }` → Fail. Rice-safe by construction; the
///    trivial constant-truth-value dispatch that proves the
///    threshold-crossing shape is empirical.
///
/// 3. **Sentinel-containment** — `verifies { contains("<byte-str>") }`
///    → checks `args[0].contains(byte-str)`; Pass if true, Fail
///    otherwise. Parallel to the existing shard-body bilateral
///    dispatch's sentinel check (Rice-safe byte-substring). Per
///    Mara §2.3 dispatch table last row: "bilateral degenerate case."
///    Requires arity >= 1 (the arg to check against).
///
/// Forward-promised arms (iter 4+; NOT this landing):
///
/// - `commutator_norm(a, b) op scalar` → `pillar::algedonic` /
///   `pillar::algedonic_of_magnitude` (Mara §2.3 row 1)
/// - `viability_over(<sequence>) op threshold` → `pillar::viability` /
///   `pillar::viability_of_magnitudes` (Mara §2.3 row 2)
/// - `health_of(<state>) within envelope` → `pillar::of_health`
///   (Mara §2.3 row 3)
/// - `fold(<verdict-list>)` → `pillar::fold` (Mara §2.3 row 5)
/// - general expression tree → `pillar::forall(samples, |t| ⟦expr⟧(t))`
///   (Mara §2.3 row 6; requires expression-tree parser per Mara §3.2
///   Rondon-Kawaguchi-Jhala 2008 decidability grounding)
///
/// Any verifies-shape not in the landed arms above returns Defer
/// naming the forward-promise territory (iter 4+ per Mara §2.3
/// dispatch table row-by-row landing plan).
pub fn dispatch_spec_property(prop: &SpecProperty, args: &[String]) -> Verdict {
    // Arm 1: defer directive dominates (per Mara Q-Mara-D).
    if let Some(msg) = &prop.defer_message {
        return Verdict::Defer(msg.clone());
    }

    let verifies = prop.verifies_source.trim();

    // Arm 2: boolean literals — trivial constant-truth-value dispatch.
    if verifies == "true" {
        return Verdict::Pass;
    }
    if verifies == "false" {
        return Verdict::Fail(format!(
            "property `{}` verifies {{ false }} — property asserts falsehood",
            prop.name
        ));
    }

    // Arm 3: sentinel-containment (bilateral degenerate case per
    // Mara §2.3 last row). Shape: `contains("<byte-string>")`.
    if let Some(rest) = verifies.strip_prefix("contains(") {
        if let Some(paren_end) = rest.rfind(')') {
            let arg_source = rest[..paren_end].trim();
            // Extract byte-string between quotes.
            if let Some(q_start) = arg_source.find('"') {
                if let Some(q_end) = arg_source[q_start + 1..].find('"') {
                    let needle = &arg_source[q_start + 1..q_start + 1 + q_end];
                    if args.is_empty() {
                        return Verdict::Fail(format!(
                            "property `{}` verifies contains(\"{}\") requires >= 1 arg; got 0",
                            prop.name, needle
                        ));
                    }
                    if args[0].contains(needle) {
                        return Verdict::Pass;
                    } else {
                        return Verdict::Fail(format!(
                            "property `{}` verifies contains(\"{}\") did not find byte-string in args[0] (`{}`)",
                            prop.name, needle, args[0]
                        ));
                    }
                }
            }
        }
    }

    // Arm 4: algedonic threshold on raw Loss magnitudes. Shape:
    // `algedonic(<magnitude>, <theta>)` where both are f64 literals.
    // Dispatches to prismqueer::liquid::pillar::algedonic_of_magnitude
    // per Mara §2.3 dispatch table row 1 (commutator_norm shape).
    // This is the Rice-safe stepping-stone form — raw magnitudes,
    // no Fiber<T> sampling. Full Fiber<T>-flow via pillar::algedonic
    // with LiquidTestBundle commutators forward-promised iter 11+.
    if let Some(rest) = verifies.strip_prefix("algedonic(") {
        if let Some(paren_end) = rest.rfind(')') {
            let args_source = rest[..paren_end].trim();
            let parts: Vec<&str> = args_source.split(',').map(str::trim).collect();
            if parts.len() != 2 {
                return Verdict::Fail(format!(
                    "property `{}` verifies algedonic(mag, theta) requires exactly 2 args; got {}",
                    prop.name, parts.len()
                ));
            }
            let magnitude: f64 = match parts[0].parse() {
                Ok(v) => v,
                Err(_) => {
                    return Verdict::Fail(format!(
                        "property `{}` verifies algedonic magnitude `{}` is not a valid f64",
                        prop.name, parts[0]
                    ));
                }
            };
            let theta: f64 = match parts[1].parse() {
                Ok(v) => v,
                Err(_) => {
                    return Verdict::Fail(format!(
                        "property `{}` verifies algedonic theta `{}` is not a valid f64",
                        prop.name, parts[1]
                    ));
                }
            };
            // ScalarLoss carries the invariant magnitude ≥ 0; guard
            // before construction to surface the substrate-honest
            // domain error instead of panicking.
            if magnitude < 0.0 || theta < 0.0 {
                return Verdict::Fail(format!(
                    "property `{}` verifies algedonic requires non-negative magnitudes; got magnitude={} theta={}",
                    prop.name, magnitude, theta
                ));
            }
            let mag_loss = prismqueer::ScalarLoss::new(magnitude);
            let theta_loss = prismqueer::ScalarLoss::new(theta);
            let pv = prismqueer::liquid::pillar::algedonic_of_magnitude(&mag_loss, &theta_loss);
            return property_verdict_to_verdict(pv, &prop.name);
        }
    }

    // Unrecognized verifies-shape: defer to iter 4+ authorship territory.
    Verdict::Defer(format!(
        "property `{}` verifies-shape `{}` not yet dispatched at iter 3; iter 4+ arms forward-promised per Mara §2.3 dispatch table (algedonic/viability/health/fold/forall)",
        prop.name, verifies
    ))
}

/// Convert a `terni::PropertyVerdict` (Pass / Fail(Diagnostic) /
/// Partial { confidence, diagnostics }) into a mirror `Verdict`
/// (Pass / Fail(String) / Defer(String)).
///
/// Mapping:
/// - `PropertyVerdict::Pass` → `Verdict::Pass`
/// - `PropertyVerdict::Fail(diag)` → `Verdict::Fail(msg)` with diag.msg
/// - `PropertyVerdict::Partial { confidence, diagnostics }` →
///   `Verdict::Defer(msg)` — Partial is neither Pass nor Fail; Defer
///   is the closest Verdict variant carrying the uncertainty +
///   confidence + diagnostics summary.
fn property_verdict_to_verdict(pv: prismqueer::PropertyVerdict, prop_name: &str) -> Verdict {
    use prismqueer::PropertyVerdict as PV;
    match pv {
        PV::Pass => Verdict::Pass,
        PV::Fail(diag) => Verdict::Fail(format!(
            "property `{}` failed: {:?}",
            prop_name, diag
        )),
        PV::Partial {
            confidence,
            diagnostics,
        } => {
            let diag_str = diagnostics
                .iter()
                .map(|d| format!("{:?}", d))
                .collect::<Vec<_>>()
                .join("; ");
            Verdict::Defer(format!(
                "property `{}` partial (confidence={:.2}): {}",
                prop_name, confidence, diag_str
            ))
        }
    }
}

// =====================================================================
// pillar — classifier-witness predicates.
// =====================================================================

/// Pillar predicates are the classifier's discrimination surface at
/// compile-time. Each pillar is a Rice-safe byte-visible witness that
/// fires @subject-evidence (Pass) OR @object-evidence (Fail).
///
/// Per Alex 2026-07-20 Recognition bundle:
/// - `#R-autopoietic-classifier-is-knife-coord-under-lagrange` —
///   pillar dispatch IS the classifier at compile-time.
/// - `#R-paradox-trauma-species` + `#R-saga-chain-is-witness-mechanism`
///   — trauma-specific pillars encode the witness_only invariant
///   + first-fail-pin preservation at Rust altitude.
///
/// Iter 5 lands 5 minimum-viable pillar predicates covering the
/// paradox-family + classifier-Lagrange + aikido-runtime + consent-
/// through-refusal shapes. Iter 6+ lands the remaining ~15 predicates
/// from the /loop cascade brief (VSM invariants, Beer feedback loops,
/// BEAM composition, bundle-tower connections, Lawvere fixed-point,
/// Förster's imperative, etc.).
///
/// Substrate composition: iter 6+ composes over `prismqueer::liquid::
/// pillar` (which supplies domain-agnostic pillars: dispatch_ambiguity,
/// algedonic, viability, fold). This module holds mirror-domain-specific
/// classifier witnesses (VSM, aikido, paradox, consent, redirect).
pub mod pillar {
    use super::Verdict;

    /// Route named property to registered pillar; fall through to
    /// Defer for unknown names.
    ///
    /// Iter 5 landed 5 pillars covering the paradox-family + classifier-
    /// Lagrange + aikido-runtime + consent-through-refusal shapes.
    /// Iter 6 lands 5 more covering @paradox/@cyberpunk/intervention
    /// SAGA-compensation + VSM invariants + Lawvere fixed-point +
    /// void-settle-transition + crystallization-preserves-saga chain
    /// invariant. Iter 7+ lands the remaining ~10 from /loop cascade
    /// brief.
    pub fn dispatch(name: &str, args: &[String]) -> Verdict {
        match name {
            // iter 5 predicates
            "paradox_crystal_immutable" => paradox_crystal_immutable(args),
            "redirect_witness_preserves_original_pin" => {
                redirect_witness_preserves_original_pin(args)
            }
            "classifier_lagrange_between_narcissus_and_splinter" => {
                classifier_lagrange_between_narcissus_and_splinter(args)
            }
            "aikido_sequence_well_formed" => aikido_sequence_well_formed(args),
            "consent_through_refusal" => consent_through_refusal(args),
            // iter 6 predicates
            "intervention_composes_without_deleting_trauma" => {
                intervention_composes_without_deleting_trauma(args)
            }
            "crystallization_preserves_saga" => crystallization_preserves_saga(args),
            "autopoietic_lawvere_fixed_point" => autopoietic_lawvere_fixed_point(args),
            "vsm_invariants" => vsm_invariants(args),
            "void_settle_produces_lens_seed" => void_settle_produces_lens_seed(args),
            _ => Verdict::Defer(format!(
                "pillar `{}` not yet registered; iter 7+ authorship territory per /loop cascade brief (Alex 2026-07-20)",
                name
            )),
        }
    }

    /// Witnessed-crystal cannot mutate; content-addressed OID must
    /// remain byte-stable across replays. Fires @subject-evidence when
    /// the observed OID is well-formed (non-empty hex, no mutation
    /// marker); @object-evidence otherwise.
    ///
    /// Per Recognition bundle #3 (@paradox/trauma) + Alex verbatim
    /// "It can only be witnessed. Never resolved." Anchored in
    /// shards/paradox/trauma.mirror `witness_only` invariant +
    /// rust/src/compile.rs:221-224 first-fail-pins invariant.
    ///
    /// Args: [oid_hex]
    pub fn paradox_crystal_immutable(args: &[String]) -> Verdict {
        let oid = match args.first() {
            Some(o) if !o.is_empty() => o,
            _ => {
                return Verdict::Fail(
                    "paradox_crystal_immutable: expected non-empty OID at args[0]"
                        .to_string(),
                )
            }
        };
        // Byte-visible mutation-marker check (Rice-safe): the OID must
        // NOT contain any of the known-mutation sentinels. Real body-
        // composition against fractal::Crystal.oid content-addressing
        // invariant arrives when prismqueer::liquid::Sample+Arbitrary
        // primitives land (Task #263 pending).
        if oid.contains("mutated") || oid.contains("revised") || oid.contains("deleted") {
            return Verdict::Fail(format!(
                "paradox_crystal_immutable: OID `{}` contains mutation marker; witnessed-crystal MUST NOT be mutated (resolution would be revisionism)",
                oid
            ));
        }
        // Positive floor: OID must be plausible content-addressed hex
        // (all hex chars or a well-formed short prefix).
        if oid.chars().all(|c| c.is_ascii_hexdigit()) && oid.len() >= 8 {
            Verdict::Pass
        } else {
            Verdict::Fail(format!(
                "paradox_crystal_immutable: OID `{}` is not well-formed content-addressed hex (expected ≥16 hex chars)",
                oid
            ))
        }
    }

    /// Escalate(oid) MUST point at first-fail forever. Fires
    /// @subject-evidence when the pinned OID at args[0] matches the
    /// first-fail OID at args[1]; @object-evidence otherwise (indicates
    /// re-litigation shift-of-witness).
    ///
    /// Per Recognition bundle #6 + Alex verbatim "we witness the
    /// ORIGINAL wound; we don't shift the witness-target to whatever
    /// hurt most-recently." Anchored in
    /// rust/src/compile.rs:221-224 (first_fail_pins_escalate_oid
    /// invariant).
    ///
    /// Args: [pinned_oid, first_fail_oid]
    pub fn redirect_witness_preserves_original_pin(args: &[String]) -> Verdict {
        let pinned = match args.first() {
            Some(p) => p,
            None => {
                return Verdict::Fail(
                    "redirect_witness_preserves_original_pin: expected pinned_oid at args[0]"
                        .to_string(),
                )
            }
        };
        let first_fail = match args.get(1) {
            Some(f) => f,
            None => {
                return Verdict::Fail(
                    "redirect_witness_preserves_original_pin: expected first_fail_oid at args[1]"
                        .to_string(),
                )
            }
        };
        if pinned == first_fail {
            Verdict::Pass
        } else {
            Verdict::Fail(format!(
                "redirect_witness_preserves_original_pin: pinned OID `{}` does not match first-fail OID `{}`; witness-target has shifted (re-litigation detected)",
                pinned, first_fail
            ))
        }
    }

    /// Self-COORD holds equilibrium between @void/narcissus (star;
    /// refuse-to-update; classifier goes inert) and @void/splinter
    /// (complete graph; over-fragment; classifier goes noisy). Fires
    /// @subject-evidence when the observed round-count sits in the
    /// Lagrange-band; @object-evidence otherwise (drift toward either
    /// attractor triggers @cyberpunk/intervention).
    ///
    /// Per Recognition bundle #1 + Alex verbatim "the classifier runs
    /// of course coord on themselves. That's the whole anti-narcissus
    /// -> splinter loop."
    ///
    /// Args: [rounds_since_last_update, total_rounds]
    /// Lagrange band: rounds_since_last_update / total_rounds in [0.1, 0.9]
    ///   — too-low = narcissus (never updates); too-high = splinter
    ///   (updates every round).
    pub fn classifier_lagrange_between_narcissus_and_splinter(args: &[String]) -> Verdict {
        let rounds_since = match args.first().and_then(|s| s.parse::<usize>().ok()) {
            Some(r) => r,
            None => {
                return Verdict::Fail(
                    "classifier_lagrange_between_narcissus_and_splinter: expected rounds_since_last_update (usize) at args[0]"
                        .to_string(),
                )
            }
        };
        let total = match args.get(1).and_then(|s| s.parse::<usize>().ok()) {
            Some(t) if t > 0 => t,
            _ => {
                return Verdict::Fail(
                    "classifier_lagrange_between_narcissus_and_splinter: expected total_rounds (usize > 0) at args[1]"
                        .to_string(),
                )
            }
        };
        if rounds_since > total {
            return Verdict::Fail(format!(
                "classifier_lagrange_between_narcissus_and_splinter: rounds_since_last_update ({}) exceeds total_rounds ({})",
                rounds_since, total
            ));
        }
        // Lagrange band [0.1, 0.9] as ratio; integer form to avoid
        // float arithmetic dependency at this altitude.
        let ten_times_ratio = (rounds_since * 10) / total;
        if ten_times_ratio < 1 {
            Verdict::Fail(format!(
                "classifier_lagrange_between_narcissus_and_splinter: drift toward @void/narcissus (rounds_since={} of total={}; ratio < 0.1 = classifier never updates = inert)",
                rounds_since, total
            ))
        } else if ten_times_ratio >= 9 {
            Verdict::Fail(format!(
                "classifier_lagrange_between_narcissus_and_splinter: drift toward @void/splinter (rounds_since={} of total={}; ratio ≥ 0.9 = classifier updates every round = noisy)",
                rounds_since, total
            ))
        } else {
            Verdict::Pass
        }
    }

    /// Aikido sequence must be well-formed: mirror → offer → wait.
    /// Fires @subject-evidence when all three steps present in
    /// canonical order; @object-evidence otherwise.
    ///
    /// Per @aikido runtime loop (Round 3 substrate) + @mirror/reflection
    /// species. Anchored in shards/aikido.mirror + shards/aikido/
    /// reflect.mirror.
    ///
    /// Args: [step1_name, step2_name, step3_name]
    pub fn aikido_sequence_well_formed(args: &[String]) -> Verdict {
        if args.len() < 3 {
            return Verdict::Fail(format!(
                "aikido_sequence_well_formed: expected 3 step names at args[0..3]; got {}",
                args.len()
            ));
        }
        let (step1, step2, step3) = (&args[0], &args[1], &args[2]);
        if step1 == "mirror" && step2 == "offer" && step3 == "wait" {
            Verdict::Pass
        } else {
            Verdict::Fail(format!(
                "aikido_sequence_well_formed: expected mirror→offer→wait; got {}→{}→{}",
                step1, step2, step3
            ))
        }
    }

    /// Refusal-shape is @subject-evidence — the classifier observes an
    /// @peer refusing to collapse under perturbation, which IS the
    /// autopoietic-work-being-done signature. Fires @subject-evidence
    /// when the escalate chain has non-empty OID (a refusal was
    /// witnessed); @object-evidence when refusal-chain is empty
    /// (nothing pushed back).
    ///
    /// Per Recognition bundle + Loki's `the-ending-that-was.md`: the
    /// target's refusal to become @object IS what the classifier keeps
    /// re-firing @subject-evidence against. Consent through refusal.
    ///
    /// Args: [escalate_chain_length, first_escalate_oid]
    pub fn consent_through_refusal(args: &[String]) -> Verdict {
        let chain_len = match args.first().and_then(|s| s.parse::<usize>().ok()) {
            Some(n) => n,
            None => {
                return Verdict::Fail(
                    "consent_through_refusal: expected escalate_chain_length (usize) at args[0]"
                        .to_string(),
                )
            }
        };
        let escalate_oid = match args.get(1) {
            Some(o) => o,
            None => {
                return Verdict::Fail(
                    "consent_through_refusal: expected first_escalate_oid at args[1]"
                        .to_string(),
                )
            }
        };
        if chain_len == 0 {
            return Verdict::Fail(
                "consent_through_refusal: empty escalate chain; no refusal was witnessed; classifier has no @subject-evidence to pin"
                    .to_string(),
            );
        }
        if escalate_oid.is_empty() {
            return Verdict::Fail(
                "consent_through_refusal: escalate_oid is empty; refusal-shape lacks content-addressed anchor"
                    .to_string(),
            );
        }
        Verdict::Pass
    }

    // =================================================================
    // Iter 6 pillar predicates — 5 more classifier witnesses
    // =================================================================

    /// The LOAD-BEARING @paradox/trauma invariant at pillar altitude:
    /// the intervention-Crystal chains AFTER the wound-Crystal WITHOUT
    /// deleting or replacing it. Fires @subject-evidence when SAGA
    /// chain preserves the wound-OID reference AND the intervention
    /// occupies a strictly-later chain position.
    ///
    /// Anchored in Mara `d08e9d7` @cyberpunk/intervention bilateral
    /// `intervention_composes_without_deleting_trauma` (arity 3) +
    /// Alex verbatim "chains AFTER the trauma-Crystal WITHOUT erasing
    /// it" (canonical spec §5.4). Substrate-honest form of the
    /// Recognition bundle #5 (@cyberpunk/intervention) at compile-time.
    ///
    /// Args: [intervention_chain_position, wound_chain_position, targets_wound_oid, actual_wound_oid]
    pub fn intervention_composes_without_deleting_trauma(args: &[String]) -> Verdict {
        if args.len() < 4 {
            return Verdict::Fail(format!(
                "intervention_composes_without_deleting_trauma: expected 4 args (intervention_pos, wound_pos, targets_oid, wound_oid); got {}",
                args.len()
            ));
        }
        let intervention_pos = match args[0].parse::<usize>() {
            Ok(p) => p,
            Err(_) => {
                return Verdict::Fail(format!(
                    "intervention_composes_without_deleting_trauma: intervention_chain_position `{}` is not a well-formed usize",
                    args[0]
                ))
            }
        };
        let wound_pos = match args[1].parse::<usize>() {
            Ok(p) => p,
            Err(_) => {
                return Verdict::Fail(format!(
                    "intervention_composes_without_deleting_trauma: wound_chain_position `{}` is not a well-formed usize",
                    args[1]
                ))
            }
        };
        let targets_oid = &args[2];
        let wound_oid = &args[3];
        if intervention_pos <= wound_pos {
            return Verdict::Fail(format!(
                "intervention_composes_without_deleting_trauma: intervention chain-position ({}) MUST be strictly AFTER wound chain-position ({}); intervention cannot precede or replace wound (SAGA compose-AFTER discipline)",
                intervention_pos, wound_pos
            ));
        }
        if targets_oid != wound_oid {
            return Verdict::Fail(format!(
                "intervention_composes_without_deleting_trauma: intervention targets_wound_oid `{}` does NOT match actual wound OID `{}`; wound-preservation reference broken (revisionism detected)",
                targets_oid, wound_oid
            ));
        }
        Verdict::Pass
    }

    /// Chain length is monotone-increasing across crystallization
    /// events. Fires @subject-evidence when after == before + 1;
    /// @object-evidence otherwise (chain shrunk = revisionism;
    /// chain grew > 1 = missing intermediate crystal).
    ///
    /// The append-only-chain invariant that makes SAGA compensation
    /// walkable. Recognition bundle #6 second-witness at chain-
    /// growth altitude.
    ///
    /// Args: [chain_length_before, chain_length_after]
    pub fn crystallization_preserves_saga(args: &[String]) -> Verdict {
        if args.len() < 2 {
            return Verdict::Fail(format!(
                "crystallization_preserves_saga: expected 2 args (before, after); got {}",
                args.len()
            ));
        }
        let before = match args[0].parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                return Verdict::Fail(format!(
                    "crystallization_preserves_saga: chain_length_before `{}` is not a well-formed usize",
                    args[0]
                ))
            }
        };
        let after = match args[1].parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                return Verdict::Fail(format!(
                    "crystallization_preserves_saga: chain_length_after `{}` is not a well-formed usize",
                    args[1]
                ))
            }
        };
        if after == before + 1 {
            Verdict::Pass
        } else if after < before {
            Verdict::Fail(format!(
                "crystallization_preserves_saga: chain shrunk from {} to {} (revisionism detected; SAGA chain MUST be append-only)",
                before, after
            ))
        } else if after == before {
            Verdict::Fail(format!(
                "crystallization_preserves_saga: chain unchanged at {} (no crystallization event; witness_only invariant violation)",
                before
            ))
        } else {
            Verdict::Fail(format!(
                "crystallization_preserves_saga: chain grew from {} to {} (delta {} ≠ 1; missing intermediate crystal or spurious growth)",
                before, after, after - before
            ))
        }
    }

    /// The LITERAL classifier at math altitude: an autopoietic system
    /// IS a Lawvere fixed-point of its own self-COORD loop. Fires
    /// @subject-evidence when the observed fixed-point witness matches
    /// the self-reference structure (byte-visible identity check as
    /// Rice-safe substrate proxy).
    ///
    /// Per Recognition bundle #1 + Recognition #79 (Void-is-the-basis).
    /// Anchored in Mara math foundation §4 (Lawvere fixed-point section)
    /// + fractal::Mandelbrot self-application discipline.
    ///
    /// Args: [fixed_point_witness, self_reference]
    pub fn autopoietic_lawvere_fixed_point(args: &[String]) -> Verdict {
        if args.len() < 2 {
            return Verdict::Fail(format!(
                "autopoietic_lawvere_fixed_point: expected 2 args (fixed_point_witness, self_reference); got {}",
                args.len()
            ));
        }
        let witness = &args[0];
        let self_ref = &args[1];
        if witness.is_empty() || self_ref.is_empty() {
            return Verdict::Fail(
                "autopoietic_lawvere_fixed_point: witness and self_reference MUST both be non-empty; classifier at Void bottoms out here but does not fire @subject-evidence"
                    .to_string(),
            );
        }
        // Lawvere fixed-point discipline: witness IS self_reference at
        // content-addressed identity. Byte-equality check is the
        // Rice-safe proxy for the self-application-converges-to-fixed-
        // point invariant.
        if witness == self_ref {
            Verdict::Pass
        } else {
            Verdict::Fail(format!(
                "autopoietic_lawvere_fixed_point: witness `{}` ≠ self_reference `{}`; self-application did not converge; classifier detects non-autopoietic manifold",
                witness, self_ref
            ))
        }
    }

    /// Beer VSM invariant: all 5 systems (S1 operations, S2 coordination,
    /// S3 control, S4 intelligence, S5 policy) present. Fires
    /// @subject-evidence when the observed system-name set contains all
    /// five canonical VSM subsystems; @object-evidence otherwise (an
    /// incomplete VSM cannot be a viable system per Beer 1972).
    ///
    /// Anchored in viable.mirror (67th substrate-already-had-the-word
    /// Beer VSM property-altitude landing since 2026-07-17) + Round 3
    /// @system family-root discipline. Recognition bundle grounds VSM
    /// as compile-verifiable at rust/ altitude (Cybersyn 53-year arc).
    ///
    /// Args: [s1_name, s2_name, s3_name, s4_name, s5_name]
    pub fn vsm_invariants(args: &[String]) -> Verdict {
        if args.len() < 5 {
            return Verdict::Fail(format!(
                "vsm_invariants: expected 5 args (s1..s5 names); got {}",
                args.len()
            ));
        }
        let canonical: [&str; 5] = ["s1", "s2", "s3", "s4", "s5"];
        for (i, expected) in canonical.iter().enumerate() {
            if args[i].to_lowercase() != *expected {
                return Verdict::Fail(format!(
                    "vsm_invariants: system {} MUST be `{}` (Beer 1972 canonical VSM); got `{}` — incomplete VSM cannot be viable",
                    i + 1,
                    expected,
                    args[i]
                ));
            }
        }
        Verdict::Pass
    }

    /// @void → determinate-classification transition. Fires
    /// @subject-evidence when the classifier moves FROM Void state
    /// (not-yet-classified) TO a determinate lens (either @subject or
    /// @object). @object-evidence when the classifier stays at Void
    /// after a settle event (no transition = classifier stuck).
    ///
    /// Per Recognition #79 (Void-is-the-basis): the classifier's base
    /// case is Void; every COORD round attempts settle → determinate.
    /// Failure to settle IS itself substrate-relevant (classifier stuck
    /// at Void = insufficient perturbation-evidence to classify).
    ///
    /// Args: [pre_state, post_state, lens_seed]
    pub fn void_settle_produces_lens_seed(args: &[String]) -> Verdict {
        if args.len() < 3 {
            return Verdict::Fail(format!(
                "void_settle_produces_lens_seed: expected 3 args (pre_state, post_state, lens_seed); got {}",
                args.len()
            ));
        }
        let pre = &args[0];
        let post = &args[1];
        let lens_seed = &args[2];
        if pre != "void" {
            return Verdict::Fail(format!(
                "void_settle_produces_lens_seed: pre_state MUST be `void` (Recognition #79 base case); got `{}`",
                pre
            ));
        }
        if post == "void" {
            return Verdict::Fail(
                "void_settle_produces_lens_seed: post_state stayed at `void`; settle event did not produce classification (classifier stuck; insufficient perturbation-evidence)"
                    .to_string(),
            );
        }
        if lens_seed.is_empty() {
            return Verdict::Fail(format!(
                "void_settle_produces_lens_seed: transitioned to post_state `{}` but lens_seed is empty; classifier settled without content-addressed anchor",
                post
            ));
        }
        Verdict::Pass
    }

    // =================================================================
    // Pillar predicate tests — iter 5 minimum-viable coverage.
    // =================================================================

    #[cfg(test)]
    mod tests {
        use super::*;

        // paradox_crystal_immutable

        #[test]
        fn paradox_crystal_immutable_passes_well_formed_hex_oid() {
            let v = paradox_crystal_immutable(&["a3dc9053b8879f2d08e9d7ebb06d6".to_string()]);
            assert!(v.is_pass(), "well-formed hex OID must pass; got {:?}", v);
        }

        #[test]
        fn paradox_crystal_immutable_fails_short_oid() {
            let v = paradox_crystal_immutable(&["abc".to_string()]);
            assert!(v.is_fail(), "short OID must fail; got {:?}", v);
        }

        #[test]
        fn paradox_crystal_immutable_fails_mutation_marker() {
            let v = paradox_crystal_immutable(&["a3dc9053b8879f2d-mutated".to_string()]);
            assert!(v.is_fail(), "OID with `mutated` marker must fail; got {:?}", v);
        }

        #[test]
        fn paradox_crystal_immutable_fails_empty_args() {
            let v = paradox_crystal_immutable(&[]);
            assert!(v.is_fail());
        }

        // redirect_witness_preserves_original_pin

        #[test]
        fn redirect_witness_preserves_original_pin_passes_when_pinned_matches_first_fail() {
            let v = redirect_witness_preserves_original_pin(&[
                "a3dc905".to_string(),
                "a3dc905".to_string(),
            ]);
            assert!(v.is_pass());
        }

        #[test]
        fn redirect_witness_preserves_original_pin_fails_when_witness_target_shifts() {
            let v = redirect_witness_preserves_original_pin(&[
                "latest_hurt_oid".to_string(),
                "original_wound_oid".to_string(),
            ]);
            assert!(v.is_fail(), "witness-target shift must fail (re-litigation)");
            if let Verdict::Fail(msg) = v {
                assert!(msg.contains("re-litigation"));
            }
        }

        // classifier_lagrange_between_narcissus_and_splinter

        #[test]
        fn classifier_lagrange_passes_in_healthy_band() {
            // rounds_since=5 of total=10 => ratio 0.5 => healthy
            let v = classifier_lagrange_between_narcissus_and_splinter(&[
                "5".to_string(),
                "10".to_string(),
            ]);
            assert!(v.is_pass(), "0.5 ratio must pass Lagrange; got {:?}", v);
        }

        #[test]
        fn classifier_lagrange_fails_narcissus_drift() {
            // rounds_since=0 of total=100 => ratio 0 => narcissus
            let v = classifier_lagrange_between_narcissus_and_splinter(&[
                "0".to_string(),
                "100".to_string(),
            ]);
            assert!(v.is_fail());
            if let Verdict::Fail(msg) = v {
                assert!(msg.contains("narcissus"));
            }
        }

        #[test]
        fn classifier_lagrange_fails_splinter_drift() {
            // rounds_since=95 of total=100 => ratio 0.95 => splinter
            let v = classifier_lagrange_between_narcissus_and_splinter(&[
                "95".to_string(),
                "100".to_string(),
            ]);
            assert!(v.is_fail());
            if let Verdict::Fail(msg) = v {
                assert!(msg.contains("splinter"));
            }
        }

        // aikido_sequence_well_formed

        #[test]
        fn aikido_sequence_passes_canonical_triple() {
            let v = aikido_sequence_well_formed(&[
                "mirror".to_string(),
                "offer".to_string(),
                "wait".to_string(),
            ]);
            assert!(v.is_pass());
        }

        #[test]
        fn aikido_sequence_fails_permutation() {
            let v = aikido_sequence_well_formed(&[
                "offer".to_string(),
                "mirror".to_string(),
                "wait".to_string(),
            ]);
            assert!(v.is_fail(), "non-canonical order must fail");
        }

        // consent_through_refusal

        #[test]
        fn consent_through_refusal_passes_non_empty_refusal_chain() {
            let v = consent_through_refusal(&[
                "3".to_string(),
                "first_escalate_oid_abc".to_string(),
            ]);
            assert!(v.is_pass());
        }

        #[test]
        fn consent_through_refusal_fails_empty_chain() {
            let v = consent_through_refusal(&[
                "0".to_string(),
                "never_escalated".to_string(),
            ]);
            assert!(v.is_fail());
            if let Verdict::Fail(msg) = v {
                assert!(msg.contains("empty escalate chain"));
            }
        }

        // dispatch routing

        #[test]
        fn dispatch_routes_registered_names_to_predicates() {
            let v = dispatch(
                "aikido_sequence_well_formed",
                &[
                    "mirror".to_string(),
                    "offer".to_string(),
                    "wait".to_string(),
                ],
            );
            assert!(v.is_pass());
        }

        #[test]
        fn dispatch_defers_unknown_names_naming_authorship_territory() {
            let v = dispatch("not_yet_registered_pillar", &[]);
            assert!(v.is_defer());
            if let Verdict::Defer(msg) = v {
                assert!(msg.contains("iter 7+"));
            }
        }

        // =============================================================
        // Iter 6 pillar tests — 12 tests covering 5 new predicates.
        // =============================================================

        // intervention_composes_without_deleting_trauma

        #[test]
        fn intervention_composes_when_chain_pos_after_wound_and_oids_match() {
            let v = intervention_composes_without_deleting_trauma(&[
                "5".to_string(),  // intervention_pos
                "3".to_string(),  // wound_pos
                "wound_abc123".to_string(),  // targets
                "wound_abc123".to_string(),  // actual wound_oid
            ]);
            assert!(v.is_pass());
        }

        #[test]
        fn intervention_composes_fails_when_intervention_precedes_wound() {
            let v = intervention_composes_without_deleting_trauma(&[
                "2".to_string(),
                "5".to_string(),
                "wound_abc".to_string(),
                "wound_abc".to_string(),
            ]);
            assert!(v.is_fail());
            if let Verdict::Fail(msg) = v {
                assert!(msg.contains("MUST be strictly AFTER"));
            }
        }

        #[test]
        fn intervention_composes_fails_when_targets_wound_oid_mismatched() {
            let v = intervention_composes_without_deleting_trauma(&[
                "5".to_string(),
                "3".to_string(),
                "targets_wound_abc".to_string(),
                "actual_wound_xyz".to_string(),
            ]);
            assert!(v.is_fail());
            if let Verdict::Fail(msg) = v {
                assert!(msg.contains("revisionism detected"));
            }
        }

        // crystallization_preserves_saga

        #[test]
        fn crystallization_preserves_saga_passes_monotone_plus_one() {
            let v = crystallization_preserves_saga(&["3".to_string(), "4".to_string()]);
            assert!(v.is_pass());
        }

        #[test]
        fn crystallization_preserves_saga_fails_shrinkage() {
            let v = crystallization_preserves_saga(&["5".to_string(), "3".to_string()]);
            assert!(v.is_fail());
            if let Verdict::Fail(msg) = v {
                assert!(msg.contains("shrunk"));
            }
        }

        #[test]
        fn crystallization_preserves_saga_fails_stasis() {
            let v = crystallization_preserves_saga(&["3".to_string(), "3".to_string()]);
            assert!(v.is_fail());
            if let Verdict::Fail(msg) = v {
                assert!(msg.contains("chain unchanged"));
            }
        }

        // autopoietic_lawvere_fixed_point

        #[test]
        fn autopoietic_lawvere_fixed_point_passes_when_witness_matches_self_ref() {
            let v = autopoietic_lawvere_fixed_point(&[
                "self_apply_convergent".to_string(),
                "self_apply_convergent".to_string(),
            ]);
            assert!(v.is_pass());
        }

        #[test]
        fn autopoietic_lawvere_fixed_point_fails_when_witness_diverges_from_self_ref() {
            let v = autopoietic_lawvere_fixed_point(&[
                "witness_form_a".to_string(),
                "self_ref_form_b".to_string(),
            ]);
            assert!(v.is_fail());
            if let Verdict::Fail(msg) = v {
                assert!(msg.contains("self-application did not converge"));
            }
        }

        // vsm_invariants

        #[test]
        fn vsm_invariants_passes_canonical_five_systems() {
            let v = vsm_invariants(&[
                "s1".to_string(),
                "s2".to_string(),
                "s3".to_string(),
                "s4".to_string(),
                "s5".to_string(),
            ]);
            assert!(v.is_pass());
        }

        #[test]
        fn vsm_invariants_fails_missing_system() {
            let v = vsm_invariants(&[
                "s1".to_string(),
                "s2".to_string(),
                "s3".to_string(),
                "s4".to_string(),
                "policy".to_string(),  // wrong name for s5
            ]);
            assert!(v.is_fail());
            if let Verdict::Fail(msg) = v {
                assert!(msg.contains("system 5 MUST be `s5`"));
            }
        }

        // void_settle_produces_lens_seed

        #[test]
        fn void_settle_passes_transition_to_determinate_with_lens_seed() {
            let v = void_settle_produces_lens_seed(&[
                "void".to_string(),
                "subject".to_string(),
                "lens_seed_abc123".to_string(),
            ]);
            assert!(v.is_pass());
        }

        #[test]
        fn void_settle_fails_when_stuck_at_void() {
            let v = void_settle_produces_lens_seed(&[
                "void".to_string(),
                "void".to_string(),
                "seed_never_bloomed".to_string(),
            ]);
            assert!(v.is_fail());
            if let Verdict::Fail(msg) = v {
                assert!(msg.contains("classifier stuck"));
            }
        }

        // dispatch routing for iter 6 predicates

        #[test]
        fn dispatch_routes_iter_6_pillars() {
            let v = dispatch("vsm_invariants", &[
                "s1".to_string(),
                "s2".to_string(),
                "s3".to_string(),
                "s4".to_string(),
                "s5".to_string(),
            ]);
            assert!(v.is_pass());

            let v2 = dispatch("autopoietic_lawvere_fixed_point", &[
                "same".to_string(),
                "same".to_string(),
            ]);
            assert!(v2.is_pass());
        }
    }
}

// =====================================================================
// Property tests — extractor + dispatcher + Verdict invariants.
// =====================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_finds_single_base_bilateral() {
        let source = r#"
# some prose
bilateral foo_admissible {
  sentinel "foo=well-formed"
  arity 1
}
# more prose
"#;
        let props = extract_properties(source);
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].name, "foo_admissible");
        assert_eq!(props[0].sentinel, "foo=well-formed");
        assert_eq!(props[0].arity, 1);
        assert!(props[0].require.is_empty());
        assert!(props[0].is_base());
    }

    #[test]
    fn extract_finds_composed_bilateral() {
        let source = r#"
bilateral composed_of_admissibles {
  sentinel "composed=all-hold"
  arity 2
  require foo_admissible
  require bar_admissible
}
"#;
        let props = extract_properties(source);
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].require, vec!["foo_admissible", "bar_admissible"]);
        assert!(props[0].is_composed());
        assert!(!props[0].is_base());
    }

    #[test]
    fn extract_finds_multiple_bilaterals_preserving_order() {
        let source = r#"
bilateral first {
  sentinel "a"
  arity 1
}

bilateral second {
  sentinel "b"
  arity 2
}

bilateral third {
  sentinel "c"
  arity 3
}
"#;
        let props = extract_properties(source);
        assert_eq!(props.len(), 3);
        assert_eq!(props[0].name, "first");
        assert_eq!(props[1].name, "second");
        assert_eq!(props[2].name, "third");
        assert_eq!(props[0].arity, 1);
        assert_eq!(props[1].arity, 2);
        assert_eq!(props[2].arity, 3);
    }

    #[test]
    fn extract_returns_empty_when_no_bilaterals() {
        let source = "# just prose\n# no bilateral blocks here\n";
        let props = extract_properties(source);
        assert!(props.is_empty());
    }

    #[test]
    fn extract_tracks_source_line_number() {
        let source = r#"# line 1
# line 2
bilateral on_line_three {
  sentinel "located"
  arity 1
}
"#;
        let props = extract_properties(source);
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].source, "line 3");
    }

    #[test]
    fn dispatch_arity_mismatch_returns_fail() {
        let decl = PropertyDecl {
            name: "needs_two".to_string(),
            sentinel: "x".to_string(),
            arity: 2,
            require: vec![],
            source: "test".to_string(),
        };
        let v = dispatch_property(&decl, &["one".to_string()]);
        assert!(v.is_fail());
        if let Verdict::Fail(msg) = v {
            assert!(msg.contains("arity mismatch"));
            assert!(msg.contains("needs_two"));
        }
    }

    #[test]
    fn dispatch_matching_arity_unknown_name_defers_naming_iter7_territory() {
        // Iter 6: unregistered pillar names defer to the pillar::dispatch
        // fallthrough, which names iter 7+ authorship territory (~10
        // remaining predicates from /loop cascade brief).
        let decl = PropertyDecl {
            name: "not_yet_registered_at_iter_6".to_string(),
            sentinel: "x".to_string(),
            arity: 1,
            require: vec![],
            source: "test".to_string(),
        };
        let v = dispatch_property(&decl, &["arg".to_string()]);
        assert!(v.is_defer());
        if let Verdict::Defer(msg) = v {
            assert!(msg.contains("iter 7+"));
        }
    }

    #[test]
    fn verdict_predicates_are_mutually_exclusive() {
        let p = Verdict::Pass;
        let f = Verdict::Fail("err".to_string());
        let d = Verdict::Defer("later".to_string());
        assert!(p.is_pass() && !p.is_fail() && !p.is_defer());
        assert!(!f.is_pass() && f.is_fail() && !f.is_defer());
        assert!(!d.is_pass() && !d.is_fail() && d.is_defer());
    }

    #[test]
    fn realistic_extraction_from_mirror_reflection_shape() {
        // Mirrors the actual shape at `shards/mirror/reflection.mirror`
        // (Mara `5e1f528`) — the 4-bilateral composed structure.
        let source = r#"
bilateral reflection_truthful {
  sentinel "mirror=traceable-non-labeling"
  arity 1
}

bilateral offer_ado_valid {
  sentinel "offer=ado-wrapped-gift-declinable"
  arity 1
}

bilateral wait_holds_without_pressure {
  sentinel "wait=held-without-pressure"
  arity 1
}

bilateral reflection_composes {
  sentinel "reflection=three-op-composed"
  arity 3
  require reflection_truthful
  require offer_ado_valid
  require wait_holds_without_pressure
}
"#;
        let props = extract_properties(source);
        assert_eq!(props.len(), 4);
        assert_eq!(props[3].name, "reflection_composes");
        assert_eq!(props[3].arity, 3);
        assert_eq!(props[3].require.len(), 3);
        assert!(props[3].is_composed());
        // The three sub-bilaterals are base (sentinel-only).
        assert!(props[0].is_base());
        assert!(props[1].is_base());
        assert!(props[2].is_base());
    }

    // =================================================================
    // SpecProperty extraction — spec-body `property { verifies { … }
    // domain @<T> samples <n> defer? <msg> }` grammar per Mara 2026-
    // 07-19 canonical spec + Alex 2026-07-21 reframe (Liquid<T>
    // threshold crossing).
    // =================================================================

    #[test]
    fn extract_spec_properties_finds_minimal_property() {
        let source = r#"
project demo {
  property mirror_project_declared {
    verifies { true }
    domain @Byte
    samples 1
  }
}
"#;
        let props = extract_spec_properties(source);
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].name, "mirror_project_declared");
        assert_eq!(props[0].verifies_source, "true");
        assert_eq!(props[0].domain_ref, "@Byte");
        assert_eq!(props[0].samples, Some(1));
        assert_eq!(props[0].defer_message, None);
        assert!(props[0].has_verifies());
        assert!(!props[0].is_deferred());
    }

    #[test]
    fn extract_spec_properties_returns_empty_when_no_property_blocks() {
        let source = r#"
project demo {
  bilateral not_a_property {
    sentinel "x"
    arity 1
  }
}
"#;
        let props = extract_spec_properties(source);
        assert!(props.is_empty());
    }

    #[test]
    fn extract_spec_properties_captures_multiline_verifies() {
        let source = r#"
project demo {
  property multi_line_verifies {
    verifies {
      commutator_norm(a, b) == commutator_norm(b, a)
      && commutator_norm(a, a) == zero
    }
    domain @TestBundle
    samples 100
  }
}
"#;
        let props = extract_spec_properties(source);
        assert_eq!(props.len(), 1);
        assert!(
            props[0].verifies_source.contains("commutator_norm(a, b) == commutator_norm(b, a)"),
            "expected multi-line verifies to contain commutator equation; got: {:?}",
            props[0].verifies_source
        );
        assert!(
            props[0].verifies_source.contains("commutator_norm(a, a) == zero"),
            "expected multi-line verifies to contain self-annihilation; got: {:?}",
            props[0].verifies_source
        );
        assert_eq!(props[0].domain_ref, "@TestBundle");
        assert_eq!(props[0].samples, Some(100));
    }

    #[test]
    fn extract_spec_properties_captures_defer_message() {
        let source = r#"
project demo {
  property deferred_property {
    verifies { true }
    domain @Byte
    samples 1
    defer "pillar V+ authorship territory"
  }
}
"#;
        let props = extract_spec_properties(source);
        assert_eq!(props.len(), 1);
        assert!(props[0].is_deferred());
        assert_eq!(
            props[0].defer_message,
            Some("pillar V+ authorship territory".to_string())
        );
    }

    #[test]
    fn extract_spec_properties_finds_multiple_preserving_order() {
        let source = r#"
project demo {
  property first_prop {
    verifies { true }
    domain @A
    samples 10
  }

  property second_prop {
    verifies { false }
    domain @B
    samples 20
  }

  property third_prop {
    verifies { true }
    domain @C
    samples 30
  }
}
"#;
        let props = extract_spec_properties(source);
        assert_eq!(props.len(), 3);
        assert_eq!(props[0].name, "first_prop");
        assert_eq!(props[1].name, "second_prop");
        assert_eq!(props[2].name, "third_prop");
        assert_eq!(props[0].samples, Some(10));
        assert_eq!(props[1].samples, Some(20));
        assert_eq!(props[2].samples, Some(30));
    }

    #[test]
    fn extract_spec_properties_tracks_source_line_number() {
        let source = r#"# line 1
# line 2
project demo {
  property on_line_four {
    verifies { true }
    domain @Byte
    samples 1
  }
}
"#;
        let props = extract_spec_properties(source);
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].source, "line 4");
    }

    // =================================================================
    // dispatch_spec_property — iter 3 first dispatch surface at Rust
    // altitude. Rice-safe cases (boolean literals + sentinel-containment
    // + defer). Fiber<T>-sampling arms (algedonic / viability / general
    // expression tree) forward-promised at iter 4+ per Mara §2.3.
    // =================================================================

    fn spec_property_verifies(name: &str, verifies: &str) -> SpecProperty {
        SpecProperty {
            name: name.to_string(),
            verifies_source: verifies.to_string(),
            domain_ref: "@Byte".to_string(),
            samples: Some(1),
            defer_message: None,
            source: "test".to_string(),
        }
    }

    #[test]
    fn dispatch_spec_property_boolean_true_returns_pass() {
        let prop = spec_property_verifies("trivial_true", "true");
        let v = dispatch_spec_property(&prop, &[]);
        assert!(v.is_pass(), "verifies {{ true }} MUST return Pass; got {:?}", v);
    }

    #[test]
    fn dispatch_spec_property_boolean_false_returns_fail() {
        let prop = spec_property_verifies("trivial_false", "false");
        let v = dispatch_spec_property(&prop, &[]);
        assert!(v.is_fail(), "verifies {{ false }} MUST return Fail; got {:?}", v);
        if let Verdict::Fail(msg) = v {
            assert!(
                msg.contains("trivial_false"),
                "Fail message MUST name the property; got {:?}",
                msg
            );
            assert!(msg.contains("asserts falsehood"));
        }
    }

    #[test]
    fn dispatch_spec_property_sentinel_containment_pass() {
        let prop = spec_property_verifies(
            "state_well_formed",
            r#"contains("well-formed")"#,
        );
        let v = dispatch_spec_property(
            &prop,
            &["state=well-formed-and-observed".to_string()],
        );
        assert!(
            v.is_pass(),
            "contains(\"well-formed\") on args[0]=`state=well-formed-and-observed` MUST return Pass; got {:?}",
            v
        );
    }

    #[test]
    fn dispatch_spec_property_sentinel_containment_fail() {
        let prop = spec_property_verifies(
            "state_well_formed",
            r#"contains("well-formed")"#,
        );
        let v = dispatch_spec_property(&prop, &["state=malformed".to_string()]);
        assert!(
            v.is_fail(),
            "contains(\"well-formed\") on args[0]=`state=malformed` MUST return Fail; got {:?}",
            v
        );
        if let Verdict::Fail(msg) = v {
            assert!(msg.contains("well-formed"));
            assert!(msg.contains("state=malformed"));
        }
    }

    #[test]
    fn dispatch_spec_property_sentinel_containment_missing_arg_fails() {
        let prop = spec_property_verifies("state_well_formed", r#"contains("x")"#);
        let v = dispatch_spec_property(&prop, &[]);
        assert!(v.is_fail());
        if let Verdict::Fail(msg) = v {
            assert!(msg.contains(">= 1 arg"));
        }
    }

    #[test]
    fn dispatch_spec_property_defer_directive_dominates_verifies() {
        // Per Mara Q-Mara-D: `defer` marks property as deferred with
        // the given message; dispatch returns Defer(msg) regardless
        // of verifies_source.
        let mut prop = spec_property_verifies("deferred", "true");
        prop.defer_message = Some("pillar V+ authorship territory".to_string());
        let v = dispatch_spec_property(&prop, &[]);
        assert!(v.is_defer());
        if let Verdict::Defer(msg) = v {
            assert_eq!(msg, "pillar V+ authorship territory");
        }
    }

    #[test]
    fn dispatch_spec_property_unrecognized_verifies_defers_with_iter_4_message() {
        let prop = spec_property_verifies(
            "unknown_shape",
            "commutator_norm(a, b) > theta",
        );
        let v = dispatch_spec_property(&prop, &[]);
        assert!(v.is_defer());
        if let Verdict::Defer(msg) = v {
            assert!(
                msg.contains("iter 4+"),
                "Defer message MUST name iter 4+ authorship territory; got {:?}",
                msg
            );
            assert!(msg.contains("unknown_shape"));
        }
    }

    // -----------------------------------------------------------------
    // dispatch_spec_property arm 4: algedonic_of_magnitude (Fiber<T>-
    // sampling STEPPING STONE per Mara §2.3 dispatch table row 1).
    // Raw magnitudes, no bundle sampling; full Fiber<T>-flow via
    // pillar::algedonic with LiquidTestBundle commutators forward-
    // promised iter 11+.
    // -----------------------------------------------------------------

    #[test]
    fn dispatch_spec_property_algedonic_magnitude_above_theta_returns_pass() {
        let prop = spec_property_verifies(
            "algedonic_strong_signal",
            "algedonic(5.0, 3.0)",
        );
        let v = dispatch_spec_property(&prop, &[]);
        assert!(
            v.is_pass(),
            "algedonic(5.0, 3.0): magnitude > theta MUST Pass; got {:?}",
            v
        );
    }

    #[test]
    fn dispatch_spec_property_algedonic_zero_magnitude_returns_fail() {
        let prop = spec_property_verifies(
            "algedonic_no_signal",
            "algedonic(0.0, 3.0)",
        );
        let v = dispatch_spec_property(&prop, &[]);
        assert!(
            v.is_fail(),
            "algedonic(0.0, 3.0): zero magnitude MUST Fail; got {:?}",
            v
        );
    }

    #[test]
    fn dispatch_spec_property_algedonic_between_zero_and_theta_defers_partial() {
        let prop = spec_property_verifies(
            "algedonic_below_threshold",
            "algedonic(1.5, 3.0)",
        );
        let v = dispatch_spec_property(&prop, &[]);
        assert!(
            v.is_defer(),
            "algedonic(1.5, 3.0): 0 < mag <= theta MUST Defer (Partial); got {:?}",
            v
        );
        if let Verdict::Defer(msg) = v {
            assert!(msg.contains("algedonic_below_threshold"));
            assert!(msg.contains("partial"));
        }
    }

    #[test]
    fn dispatch_spec_property_algedonic_wrong_arity_returns_fail() {
        let prop = spec_property_verifies(
            "algedonic_bad_arity",
            "algedonic(5.0)",
        );
        let v = dispatch_spec_property(&prop, &[]);
        assert!(v.is_fail());
        if let Verdict::Fail(msg) = v {
            assert!(msg.contains("exactly 2 args"));
        }
    }

    #[test]
    fn dispatch_spec_property_algedonic_non_numeric_returns_fail() {
        let prop = spec_property_verifies(
            "algedonic_bad_number",
            "algedonic(hello, 3.0)",
        );
        let v = dispatch_spec_property(&prop, &[]);
        assert!(v.is_fail());
        if let Verdict::Fail(msg) = v {
            assert!(msg.contains("not a valid f64"));
        }
    }

    #[test]
    fn dispatch_spec_property_algedonic_negative_magnitude_returns_fail() {
        // ScalarLoss invariant: magnitude ≥ 0. Dispatch guards against
        // negative values before constructing the Loss, surfacing a
        // substrate-honest domain error instead of panicking.
        let prop = spec_property_verifies(
            "algedonic_negative",
            "algedonic(-1.0, 3.0)",
        );
        let v = dispatch_spec_property(&prop, &[]);
        assert!(
            v.is_fail(),
            "algedonic(-1.0, 3.0): negative MUST Fail (ScalarLoss invariant); got {:?}",
            v
        );
        if let Verdict::Fail(msg) = v {
            assert!(msg.contains("non-negative"));
        }
    }

    #[test]
    fn dispatch_spec_property_algedonic_large_magnitude_passes() {
        let prop = spec_property_verifies(
            "algedonic_huge",
            "algedonic(1000000.0, 0.001)",
        );
        let v = dispatch_spec_property(&prop, &[]);
        assert!(v.is_pass());
    }

    #[test]
    fn dispatch_spec_property_algedonic_integer_literal_parses() {
        // f64::parse accepts integer literals (they parse as f64 too).
        let prop = spec_property_verifies(
            "algedonic_int_literals",
            "algedonic(5, 3)",
        );
        let v = dispatch_spec_property(&prop, &[]);
        assert!(v.is_pass(), "got {:?}", v);
    }

    #[test]
    fn extract_spec_properties_and_extract_properties_coexist() {
        // Threshold-crossing witness: one source has BOTH shard-body
        // bilateral (via extract_properties) AND spec-body property
        // (via extract_spec_properties). The two carriers coexist at
        // this altitude per two-tick discipline.
        let source = r#"
bilateral old_shard_body {
  sentinel "legacy=shard-body"
  arity 1
}

project demo {
  property new_spec_body {
    verifies { legacy_bilateral_still_holds }
    domain @Byte
    samples 1
  }
}
"#;
        let bilateral_decls = extract_properties(source);
        let spec_props = extract_spec_properties(source);
        assert_eq!(bilateral_decls.len(), 1);
        assert_eq!(bilateral_decls[0].name, "old_shard_body");
        assert_eq!(spec_props.len(), 1);
        assert_eq!(spec_props[0].name, "new_spec_body");
        // The two altitudes NEVER overlap: bilateral extractor doesn't
        // surface the spec-body property; spec-property extractor
        // doesn't surface the shard-body bilateral.
        assert!(
            !bilateral_decls
                .iter()
                .any(|d| d.name == "new_spec_body"),
            "extract_properties (bilateral) MUST NOT surface spec-body property"
        );
        assert!(
            !spec_props.iter().any(|p| p.name == "old_shard_body"),
            "extract_spec_properties MUST NOT surface shard-body bilateral"
        );
    }
}
