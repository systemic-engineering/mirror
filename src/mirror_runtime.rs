//! MirrorRuntime — produces compiled `Shatter` artifacts from `.mirror` source.
//!
//! ## Recognition
//!
//! Each declaration in a `.mirror` file IS one beam in a content-addressed
//! trajectory. The compilation primitive is `MirrorFragment` (a
//! `Fractal<MirrorData, CoincidenceHash<5>>`), which lives in the
//! `coincidence` crate alongside `CoincidenceHash` itself.
//!
//! Five spectral dimensions: meets-and-exceeds the 3+1 of the cosmos. The
//! hash function has enough degrees of freedom to be a cosmic content
//! address — every structurally distinct form has a unique slot, with room.
//!
//! ## Pipeline
//!
//! - parse `.mirror` source → `MirrorFragment` tree (typed declaration node)
//! - wrap into `Shatter`, the runtime artifact
//! - `Shatter` implements the `Prism` trait: focus / project / refract are
//!   the operations that move a Form into and out of its content-addressed
//!   representation. `split` and `zoom` are TBD — their semantics will
//!   emerge when the trait is specced.
//!
//! Round-trip is exact: parse → emit text → parse again yields identical
//! content OIDs because the OID is derived from `MirrorData::encode()` and
//! recursive child OIDs via `fragmentation::fragment::content_oid()`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use coincidence::declaration::{
    fragment as build_fragment, DeclKind, MirrorData, MirrorFragment, MirrorFragmentExt, MirrorHash,
};
use fragmentation::frgmnt_store::FrgmntStore;
use fragmentation::sha::HashAlg;
use prism::{Beam, Precision, Prism};

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct MirrorRuntimeError(pub String);

impl std::fmt::Display for MirrorRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for MirrorRuntimeError {}

fn err(s: impl Into<String>) -> MirrorRuntimeError {
    MirrorRuntimeError(s.into())
}

#[derive(Debug)]
pub struct MirrorResolveError(pub String);

impl std::fmt::Display for MirrorResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for MirrorResolveError {}

// ---------------------------------------------------------------------------
// Form — the runtime structure that Shatter compiles to/from.
// ---------------------------------------------------------------------------

/// `Form` is the parsed-but-not-yet-content-addressed view: kind / name /
/// params / variants / nested children. The structural mirror of `MirrorData`
/// + recursive children. Used as `Prism::Input` and `Prism::Crystal`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Form {
    pub kind: DeclKind,
    pub name: String,
    pub params: Vec<String>,
    pub variants: Vec<String>,
    pub children: Vec<Form>,
}

impl Form {
    pub fn new(
        kind: DeclKind,
        name: impl Into<String>,
        params: Vec<String>,
        variants: Vec<String>,
        children: Vec<Form>,
    ) -> Self {
        Form {
            kind,
            name: name.into(),
            params,
            variants,
            children,
        }
    }

    fn to_fragment(&self) -> MirrorFragment {
        let data = MirrorData::new(
            self.kind.clone(),
            self.name.clone(),
            self.params.clone(),
            self.variants.clone(),
        );
        let children: Vec<MirrorFragment> = self.children.iter().map(|c| c.to_fragment()).collect();
        build_fragment(data, children)
    }

    fn from_fragment(frag: &MirrorFragment) -> Form {
        let d = frag.mirror_data();
        let children: Vec<Form> = frag
            .mirror_children()
            .iter()
            .map(Form::from_fragment)
            .collect();
        Form {
            kind: d.kind.clone(),
            name: d.name.clone(),
            params: d.params.clone(),
            variants: d.variants.clone(),
            children,
        }
    }
}

// ---------------------------------------------------------------------------
// Shatter — the compilation artifact, a Prism implementation.
// ---------------------------------------------------------------------------

/// `Shatter` is the compilation artifact of `MirrorRuntime`. It implements
/// the `Prism` trait: the five operations move a `Form` into and out of its
/// content-addressed representation.
///
/// `split` and `zoom` are TBD: their semantics will be specified when use
/// arrives. They are conservative no-ops that preserve compilation.
#[derive(Clone, Debug, Default)]
pub struct Shatter;

impl Prism for Shatter {
    type Input = Form;
    type Eigenvalues = MirrorData;
    type Projection = MirrorFragment;
    type Node = Form;
    type Convergence = MirrorFragment;
    type Crystal = Form;
    type Precision = Precision;

    /// Focus: read the top-level eigenvalues (kind/name/params/variants).
    fn focus(&self, input: &Form) -> Beam<MirrorData> {
        Beam::new(MirrorData::new(
            input.kind.clone(),
            input.name.clone(),
            input.params.clone(),
            input.variants.clone(),
        ))
    }

    /// Project: turn the eigenvalues + a fresh form into a content-addressed
    /// MirrorFragment. Precision is honored as part of the trait contract
    /// but the projection is structurally lossless.
    fn project(&self, _eigenvalues: &MirrorData, _precision: Precision) -> Beam<MirrorFragment> {
        // The eigenvalues alone don't carry children. Project against an
        // empty form so the call is meaningful at the trait surface; full
        // structural projection happens via `compile_form()` below.
        let frag = build_fragment(_eigenvalues.clone(), Vec::new());
        Beam::new(frag)
    }

    /// Split — TBD. Conservative no-op: yield the projection back as one
    /// node beam. The semantics will be specified when use arrives.
    fn split(&self, projection: &MirrorFragment) -> Vec<Beam<Form>> {
        vec![Beam::new(Form::from_fragment(projection))]
    }

    /// Zoom — TBD. Conservative pass-through over the contained projection.
    fn zoom(
        &self,
        beam: Beam<MirrorFragment>,
        f: &dyn Fn(MirrorFragment) -> MirrorFragment,
    ) -> Beam<MirrorFragment> {
        beam.map(f)
    }

    /// Refract: settle a content-addressed projection back into a Form.
    fn refract(&self, beam: Beam<MirrorFragment>) -> Form {
        Form::from_fragment(&beam.result)
    }
}

impl Shatter {
    /// Full structural compile: Form → MirrorFragment with all children
    /// content-addressed. Used by the boot pipeline.
    pub fn compile_form(&self, form: &Form) -> MirrorFragment {
        form.to_fragment()
    }

    /// Inverse: MirrorFragment → Form.
    pub fn decompile(&self, frag: &MirrorFragment) -> Form {
        Form::from_fragment(frag)
    }
}

// ---------------------------------------------------------------------------
// Parser — line-oriented, brace-balanced.
// ---------------------------------------------------------------------------

/// Parse a `.mirror` source string. The top-level may contain one or more
/// declarations. If there is exactly one, return it as-is. If there are
/// multiple, wrap them in a synthetic file-level Form.
pub fn parse_form(source: &str) -> Result<Form, MirrorRuntimeError> {
    let tokens = tokenize(source);
    let mut cursor = 0usize;
    let mut decls = Vec::new();

    loop {
        skip_trivia(&tokens, &mut cursor);
        if cursor >= tokens.len() {
            break;
        }
        decls.push(parse_decl(&tokens, &mut cursor)?);
    }

    if decls.is_empty() {
        Err(err("no declarations found".to_string()))
    } else if decls.len() == 1 {
        Ok(decls.into_iter().next().unwrap())
    } else {
        // Multiple declarations: wrap in a synthetic file-level Form
        let wrapped = Form::new(
            DeclKind::Form,
            "".to_string(),
            Vec::new(),
            Vec::new(),
            decls,
        );
        Ok(wrapped)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tok {
    Word(String),
    LBrace,
    RBrace,
    LParen,
    RParen,
    Comma,
    Equals,
    Newline,
}

fn tokenize(source: &str) -> Vec<Tok> {
    let mut out = Vec::new();
    let bytes = source.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        match c {
            ' ' | '\t' | '\r' => {
                i += 1;
            }
            '\n' => {
                out.push(Tok::Newline);
                i += 1;
            }
            '#' => {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
            }
            '{' => {
                out.push(Tok::LBrace);
                i += 1;
            }
            '}' => {
                out.push(Tok::RBrace);
                i += 1;
            }
            '(' => {
                out.push(Tok::LParen);
                i += 1;
            }
            ')' => {
                out.push(Tok::RParen);
                i += 1;
            }
            ',' => {
                out.push(Tok::Comma);
                i += 1;
            }
            '|' | '.' | '/' | '<' | '>' | ':' | '-' => {
                // Operator sequences like |, |>, <|, /, .., etc. can be declaration names.
                // Try to collect them as a word if they form a contiguous symbol sequence.
                let start = i;
                while i < bytes.len() {
                    let cc = bytes[i] as char;
                    if cc == '|'
                        || cc == '.'
                        || cc == '/'
                        || cc == '<'
                        || cc == '>'
                        || cc == ':'
                        || cc == '-'
                    {
                        i += 1;
                    } else {
                        break;
                    }
                }
                if i == start {
                    i += 1;
                } else {
                    let sym = source[start..i].to_string();
                    out.push(Tok::Word(sym));
                }
            }
            '=' => {
                out.push(Tok::Equals);
                i += 1;
            }
            _ => {
                let start = i;
                while i < bytes.len() {
                    let cc = bytes[i] as char;
                    if cc.is_alphanumeric() || cc == '_' || cc == '@' {
                        i += 1;
                    } else {
                        break;
                    }
                }
                if i == start {
                    i += 1;
                } else {
                    out.push(Tok::Word(source[start..i].to_string()));
                }
            }
        }
    }
    out
}

fn skip_trivia(tokens: &[Tok], cursor: &mut usize) {
    while *cursor < tokens.len() && matches!(tokens[*cursor], Tok::Newline) {
        *cursor += 1;
    }
}

fn parse_decl(tokens: &[Tok], cursor: &mut usize) -> Result<Form, MirrorRuntimeError> {
    skip_trivia(tokens, cursor);
    let kind_word = match tokens.get(*cursor) {
        Some(Tok::Word(w)) => w.clone(),
        other => {
            return Err(err(format!(
                "expected declaration keyword, got {:?}",
                other
            )))
        }
    };
    *cursor += 1;
    let kind = DeclKind::parse(&kind_word)
        .ok_or_else(|| err(format!("unknown declaration kind: {}", kind_word)))?;

    let name = match tokens.get(*cursor) {
        Some(Tok::Word(w)) => {
            *cursor += 1;
            w.clone()
        }
        _ => String::new(),
    };

    let mut params = Vec::new();
    if matches!(tokens.get(*cursor), Some(Tok::LParen)) {
        *cursor += 1;
        loop {
            match tokens.get(*cursor) {
                Some(Tok::RParen) => {
                    *cursor += 1;
                    break;
                }
                Some(Tok::Word(w)) => {
                    params.push(w.clone());
                    *cursor += 1;
                }
                Some(Tok::Comma) => {
                    *cursor += 1;
                }
                other => return Err(err(format!("malformed params: {:?}", other))),
            }
        }
    }

    let mut variants = Vec::new();
    if matches!(tokens.get(*cursor), Some(Tok::Equals)) {
        *cursor += 1;
        loop {
            // Don't skip newlines here - they terminate the variant list
            match tokens.get(*cursor) {
                Some(Tok::Newline) => {
                    *cursor += 1;
                    break;
                }
                Some(Tok::Word(w)) if w == "|" => {
                    // Pipe separator in variant list
                    *cursor += 1;
                }
                Some(Tok::Word(w)) => {
                    variants.push(w.clone());
                    *cursor += 1;
                    // If variant is followed by params like call(...), consume them
                    if matches!(tokens.get(*cursor), Some(Tok::LParen)) {
                        *cursor += 1;
                        let mut paren_depth = 1;
                        while *cursor < tokens.len() && paren_depth > 0 {
                            match tokens.get(*cursor) {
                                Some(Tok::LParen) => paren_depth += 1,
                                Some(Tok::RParen) => paren_depth -= 1,
                                _ => {}
                            }
                            *cursor += 1;
                        }
                    }
                }
                _ => break,
            }
        }
    }

    let mut children = Vec::new();
    skip_inline_trivia(tokens, cursor);
    if matches!(tokens.get(*cursor), Some(Tok::LBrace)) {
        *cursor += 1;
        loop {
            skip_trivia(tokens, cursor);
            match tokens.get(*cursor) {
                Some(Tok::RBrace) => {
                    *cursor += 1;
                    break;
                }
                None => return Err(err("unterminated block".to_string())),
                Some(Tok::Word(w)) => {
                    // Try to parse as a declaration. If the word is not a recognized
                    // declaration kind, skip it and any following tokens until the
                    // next recognized declaration or closing brace.
                    if DeclKind::parse(w).is_some() {
                        let child = parse_decl(tokens, cursor)?;
                        children.push(child);
                    } else {
                        // Unrecognized keyword - skip tokens until we find a newline
                        // or something that looks like the start of a new declaration
                        while *cursor < tokens.len() {
                            match tokens.get(*cursor) {
                                Some(Tok::RBrace) | Some(Tok::Newline) => break,
                                _ => {
                                    *cursor += 1;
                                }
                            }
                        }
                        // Consume the newline if present
                        if matches!(tokens.get(*cursor), Some(Tok::Newline)) {
                            *cursor += 1;
                        }
                    }
                }
                _ => {
                    // Unexpected token - skip to next line
                    while *cursor < tokens.len()
                        && !matches!(tokens.get(*cursor), Some(Tok::Newline | Tok::RBrace))
                    {
                        *cursor += 1;
                    }
                    if matches!(tokens.get(*cursor), Some(Tok::Newline)) {
                        *cursor += 1;
                    }
                }
            }
        }
    }

    Ok(Form::new(kind, name, params, variants, children))
}

fn skip_inline_trivia(tokens: &[Tok], cursor: &mut usize) {
    while matches!(tokens.get(*cursor), Some(Tok::Newline)) {
        *cursor += 1;
    }
}

// ---------------------------------------------------------------------------
// Emitter — Form → text. Round-trip stable.
// ---------------------------------------------------------------------------

pub fn emit_form(form: &Form) -> String {
    let mut out = String::new();
    emit_form_into(form, 0, &mut out);
    out
}

fn emit_form_into(form: &Form, indent: usize, out: &mut String) {
    for _ in 0..indent {
        out.push_str("  ");
    }
    out.push_str(form.kind.as_str());
    if !form.name.is_empty() {
        out.push(' ');
        out.push_str(&form.name);
    }
    if !form.params.is_empty() {
        out.push('(');
        for (i, p) in form.params.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str(p);
        }
        out.push(')');
    }
    if !form.variants.is_empty() {
        out.push_str(" = ");
        for (i, v) in form.variants.iter().enumerate() {
            if i > 0 {
                out.push_str(" | ");
            }
            out.push_str(v);
        }
    }
    if !form.children.is_empty() {
        out.push_str(" {\n");
        for child in &form.children {
            emit_form_into(child, indent + 1, out);
        }
        for _ in 0..indent {
            out.push_str("  ");
        }
        out.push_str("}\n");
    } else {
        out.push('\n');
    }
}

// ---------------------------------------------------------------------------
// MirrorRuntime — the operation.
// ---------------------------------------------------------------------------

/// Compiled artifact: a top-level Form, its content-addressed MirrorFragment,
/// and the crystal hash (root OID).
#[derive(Clone, Debug)]
pub struct CompiledShatter {
    pub form: Form,
    pub fragment: MirrorFragment,
}

impl CompiledShatter {
    pub fn crystal(&self) -> &MirrorHash {
        self.fragment.oid()
    }
    pub fn form_name(&self) -> &str {
        &self.form.name
    }
}

#[derive(Default)]
pub struct MirrorRuntime;

impl MirrorRuntime {
    pub fn new() -> Self {
        MirrorRuntime
    }

    pub fn compile_source(&self, source: &str) -> Result<CompiledShatter, MirrorRuntimeError> {
        let form = parse_form(source)?;
        let shatter = Shatter;
        let fragment = shatter.compile_form(&form);
        Ok(CompiledShatter { form, fragment })
    }

    pub fn compile_file(&self, path: &Path) -> Result<CompiledShatter, MirrorRuntimeError> {
        let src = std::fs::read_to_string(path)
            .map_err(|e| err(format!("read {}: {}", path.display(), e)))?;
        self.compile_source(&src)
    }

    pub fn compile_boot_dir(&self, dir: &Path) -> Result<BootShatter, MirrorRuntimeError> {
        let mut entries: Vec<_> = std::fs::read_dir(dir)
            .map_err(|e| err(format!("read_dir {}: {}", dir.display(), e)))?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("mirror"))
            .collect();
        entries.sort();

        let mut per_file: BTreeMap<String, CompiledShatter> = BTreeMap::new();
        let mut all_forms: Vec<Form> = Vec::new();

        for path in entries {
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            let compiled = self.compile_file(&path)?;
            all_forms.push(compiled.form.clone());
            per_file.insert(stem, compiled);
        }

        let collapsed_form = Form::new(DeclKind::Form, "mirror", Vec::new(), Vec::new(), all_forms);
        let shatter = Shatter;
        let collapsed_fragment = shatter.compile_form(&collapsed_form);
        let collapsed = CompiledShatter {
            form: collapsed_form,
            fragment: collapsed_fragment,
        };

        Ok(BootShatter {
            per_file,
            collapsed,
        })
    }
}

#[derive(Debug)]
pub struct BootShatter {
    pub per_file: BTreeMap<String, CompiledShatter>,
    pub collapsed: CompiledShatter,
}

// ---------------------------------------------------------------------------
// MirrorRegistry — content-addressed store backed by FrgmntStore
// ---------------------------------------------------------------------------

const REGISTRY_CACHE_BYTES: usize = 16 * 1024 * 1024;

/// MirrorRegistry holds compiled fragments in a content-addressed store.
/// Backed by FrgmntStore<MirrorFragment>, which manages both in-memory cache
/// and persistent disk storage via the `.frgmnt/` directory structure.
pub struct MirrorRegistry {
    store: FrgmntStore<MirrorFragment>,
    ops: std::collections::BTreeSet<String>,
    root: PathBuf,
}

impl MirrorRegistry {
    /// Open or create a registry at the given path. Creates `.frgmnt/objects`
    /// and `.frgmnt/refs` subdirectories if they don't exist. Initializes
    /// builtin operations ("in", "out").
    pub fn open(path: &Path) -> Result<Self, MirrorRuntimeError> {
        let path_str = path
            .to_str()
            .ok_or_else(|| err(format!("non-utf8 registry path: {}", path.display())))?;
        let store = FrgmntStore::<MirrorFragment>::open(path_str, REGISTRY_CACHE_BYTES)
            .map_err(|e| err(format!("open frgmnt store at {}: {}", path.display(), e)))?;
        let mut ops = std::collections::BTreeSet::new();
        ops.insert("in".to_string());
        ops.insert("out".to_string());
        Ok(MirrorRegistry {
            store,
            ops,
            root: path.to_path_buf(),
        })
    }

    /// Check if an operation name is registered (builtin or custom).
    pub fn has_op(&self, name: &str) -> bool {
        self.ops.contains(name)
    }

    /// Look up a named fragment in the registry. Returns None if the name
    /// doesn't exist or the Oid it references isn't in the cache or on disk.
    pub fn lookup(&self, name: &str) -> Option<MirrorFragment> {
        let oid = self.store.get_ref(name)?;
        self.store.get_persistent(&oid)
    }

    /// Root path of the registry.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Flush all cached fragments to disk and clear the in-memory cache.
    /// Call this before dropping the registry to ensure all fragments are persisted.
    pub fn flush(&self) {
        self.store.flush();
    }

    /// Register forms into the store. If the form's name is empty (synthetic file-level form),
    /// recurse into children. For forms starting with `@`, compile to a MirrorFragment,
    /// store persistently, and map the name to its OID. Forms without `@` prefix are ignored.
    /// Returns OIDs of newly-registered forms.
    pub fn register(&mut self, form: &Form) -> Vec<String> {
        let mut oids = Vec::new();
        if form.name.is_empty() {
            // Synthetic file-level form: recurse into children
            for child in &form.children {
                oids.extend(self.register_decl(child));
            }
        } else {
            // Single named form: try to register it
            oids.extend(self.register_decl(form));
        }
        oids
    }

    /// Register a single declaration if its name starts with `@`.
    fn register_decl(&mut self, decl: &Form) -> Option<String> {
        if !decl.name.starts_with('@') {
            return None;
        }
        let shatter = Shatter;
        let fragment = shatter.compile_form(decl);
        let oid = fragment.oid().as_str().to_string();
        let size = self.estimate_size(decl);
        self.store.insert_persistent(oid.clone(), fragment, size);
        if let Err(e) = self.store.set_ref(&decl.name, &oid) {
            eprintln!("warning: set_ref({} -> {}) failed: {}", decl.name, oid, e);
        }
        Some(oid)
    }

    /// Estimate the byte size of a form for cache accounting.
    fn estimate_size(&self, form: &Form) -> usize {
        let mut bytes = form.name.len()
            + form.params.iter().map(|s| s.len()).sum::<usize>()
            + form.variants.iter().map(|s| s.len()).sum::<usize>()
            + 64; // Base overhead for Kind and structure
        for child in &form.children {
            bytes += self.estimate_size(child);
        }
        bytes
    }

    /// Resolve a Form tree by checking that every `in @X` reference exists in the store.
    /// Returns the first unresolved reference as an error, or Ok(()) if all resolve.
    /// Resolution goes through store.get_ref() to ensure it works after a reopen
    /// (disk-backed, not in-memory shadow).
    pub fn resolve(&self, form: &Form) -> Result<(), MirrorResolveError> {
        self.resolve_node(form)
    }

    fn resolve_node(&self, node: &Form) -> Result<(), MirrorResolveError> {
        if node.kind == DeclKind::In {
            let target = &node.name;
            if self.store.get_ref(target).is_none() {
                return Err(MirrorResolveError(format!(
                    "unresolved `in {}`: no such ref in registry store at {}",
                    target,
                    self.root.display()
                )));
            }
        }
        for child in &node.children {
            self.resolve_node(child)?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use fragmentation::sha::HashAlg;
    use std::path::PathBuf;

    fn boot_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("boot")
    }

    fn tempdir_for_test(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("mirror-test-{}-{}", name, std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn mirror_runtime_parses_atom_decl() {
        let src = "form @form {\n  prism focus\n}\n";
        let form = parse_form(src).unwrap();
        assert_eq!(form.kind, DeclKind::Form);
        assert_eq!(form.name, "@form");
        assert_eq!(form.children.len(), 1);
        assert_eq!(form.children[0].kind, DeclKind::Prism);
        assert_eq!(form.children[0].name, "focus");
    }

    #[test]
    fn mirror_runtime_parses_params_and_variants() {
        let src = "form @x {\n  prism eigenvalues(precision)\n  traversal kind = a | b | c\n}\n";
        let form = parse_form(src).unwrap();
        assert_eq!(form.children[0].params, vec!["precision".to_string()]);
        assert_eq!(
            form.children[1].variants,
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }

    #[test]
    fn mirror_runtime_parses_nested_property() {
        let src = "form @property {\n  property unique_variants(form) {\n    fold input\n  }\n}\n";
        let form = parse_form(src).unwrap();
        assert_eq!(form.children.len(), 1);
        let prop = &form.children[0];
        assert_eq!(prop.kind, DeclKind::Property);
        assert_eq!(prop.name, "unique_variants");
        assert_eq!(prop.params, vec!["form".to_string()]);
        assert_eq!(prop.children.len(), 1);
        assert_eq!(prop.children[0].kind, DeclKind::Fold);
    }

    #[test]
    fn mirror_runtime_compile_form_file() {
        let runtime = MirrorRuntime::new();
        let compiled = runtime
            .compile_file(&boot_dir().join("00-prism.mirror"))
            .unwrap();
        // 00-prism.mirror has multiple declarations, so they're wrapped in a
        // synthetic file-level Form.
        assert_eq!(compiled.form.kind, DeclKind::Form);
        assert!(compiled.form.children.len() >= 2);
        // Look for @prism declaration
        let prism_decl = compiled
            .form
            .children
            .iter()
            .find(|f| f.name == "@prism")
            .expect("@prism declaration present");
        assert_eq!(prism_decl.kind, DeclKind::Prism);
        assert_eq!(prism_decl.children.len(), 5);
    }

    #[test]
    fn mirror_runtime_round_trip_oids_match() {
        let runtime = MirrorRuntime::new();
        for entry in std::fs::read_dir(boot_dir()).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|s| s.to_str()) != Some("mirror") {
                continue;
            }
            let s1 = runtime.compile_file(&path).unwrap();
            let text = emit_form(&s1.form);
            let s2 = runtime.compile_source(&text).unwrap();
            assert_eq!(
                s1.crystal(),
                s2.crystal(),
                "round-trip crystal mismatch for {}",
                path.display()
            );
        }
    }

    #[test]
    fn mirror_runtime_compiles_full_boot_dir() {
        let runtime = MirrorRuntime::new();
        let boot = runtime.compile_boot_dir(&boot_dir()).unwrap();
        assert_eq!(boot.per_file.len(), 5);
        assert!(boot.per_file.contains_key("00-prism"));
        assert!(boot.per_file.contains_key("01-meta"));
        assert!(boot.per_file.contains_key("02-actor"));
        assert!(boot.per_file.contains_key("03-property"));
        assert!(boot.per_file.contains_key("10-mirror"));
        assert_eq!(boot.collapsed.form_name(), "mirror");
        assert_eq!(boot.collapsed.form.children.len(), 5);
        let again = runtime.compile_boot_dir(&boot_dir()).unwrap();
        assert_eq!(boot.collapsed.crystal(), again.collapsed.crystal());
    }

    #[test]
    fn mirror_runtime_property_file_compiles() {
        let runtime = MirrorRuntime::new();
        let compiled = runtime
            .compile_file(&boot_dir().join("03-property.mirror"))
            .unwrap();
        assert_eq!(compiled.form_name(), "@property");
        let prop_count = compiled
            .form
            .children
            .iter()
            .filter(|f| f.kind == DeclKind::Property)
            .count();
        assert_eq!(prop_count, 9);
    }

    #[test]
    fn mirror_runtime_mirror_form_has_property_applications() {
        let runtime = MirrorRuntime::new();
        let compiled = runtime
            .compile_file(&boot_dir().join("10-mirror.mirror"))
            .unwrap();
        let kinds: Vec<&DeclKind> = compiled.form.children.iter().map(|f| &f.kind).collect();
        assert!(kinds.contains(&&DeclKind::Requires));
        assert!(kinds.contains(&&DeclKind::Invariant));
        assert!(kinds.contains(&&DeclKind::Ensures));
        assert!(kinds.contains(&&DeclKind::In));
    }

    #[test]
    fn mirror_runtime_shatter_prism_round_trip() {
        // Exercise the Prism impl on Shatter: focus → project → refract.
        // The full structural round-trip uses compile_form/decompile because
        // project on the trait surface only carries the top eigenvalues.
        let runtime = MirrorRuntime::new();
        let compiled = runtime
            .compile_file(&boot_dir().join("00-prism.mirror"))
            .unwrap();
        let shatter = Shatter;

        // Trait-level focus carries the top eigenvalues.
        let eigen_beam = shatter.focus(&compiled.form);
        assert_eq!(eigen_beam.result.kind, DeclKind::Form);
        // 00-prism.mirror wraps multiple declarations in a synthetic Form with empty name
        assert_eq!(eigen_beam.result.name, "");

        // Trait-level project produces a content-addressed (childless) frag.
        let proj_beam = shatter.project(&eigen_beam.result, Precision::new(1.0));
        assert!(!proj_beam.result.oid().as_str().is_empty());

        // Full structural projection via compile_form, then refract back.
        let frag = shatter.compile_form(&compiled.form);
        let restored = shatter.refract(Beam::new(frag.clone()));
        assert_eq!(restored, compiled.form);

        // Stable OID across runs (CoincidenceHash<5> determinism).
        let frag2 = shatter.compile_form(&compiled.form);
        assert_eq!(frag.oid(), frag2.oid());
    }

    #[test]
    fn registry_opens_at_path_with_in_and_out_builtins() {
        let tmp = tempdir_for_test("registry_opens");
        let registry = MirrorRegistry::open(&tmp).expect("open registry");
        assert!(registry.has_op("in"), "in must be a builtin op");
        assert!(registry.has_op("out"), "out must be a builtin op");
        assert!(registry.lookup("@prism").is_none());
        assert!(tmp.join("objects").exists());
        assert!(tmp.join("refs").exists());
    }

    #[test]
    fn registry_registers_named_form_into_store() {
        let tmp = tempdir_for_test("registry_registers_named");
        let mut registry = MirrorRegistry::open(&tmp).unwrap();

        let form = Form::new(
            DeclKind::Prism,
            "@prism",
            Vec::new(),
            Vec::new(),
            vec![Form::new(
                DeclKind::Prism,
                "focus",
                Vec::new(),
                Vec::new(),
                Vec::new(),
            )],
        );
        registry.register(&form);

        let stored = registry.lookup("@prism").expect("@prism in registry");
        let shatter = Shatter;
        let restored = shatter.decompile(&stored);
        assert_eq!(restored.name, "@prism");
        assert_eq!(restored.children.len(), 1);
        assert_eq!(restored.children[0].name, "focus");
    }

    #[test]
    fn registry_registers_only_at_named_top_level_forms() {
        let tmp = tempdir_for_test("registry_registers_only_at");
        let mut registry = MirrorRegistry::open(&tmp).unwrap();

        // A name without @-prefix should NOT become a form binding.
        let form = Form::new(DeclKind::Prism, "id", Vec::new(), Vec::new(), Vec::new());
        registry.register(&form);
        assert!(registry.lookup("id").is_none());
        assert!(registry.lookup("@id").is_none());
    }

    #[test]
    fn registry_persists_across_reopen() {
        let tmp = tempdir_for_test("registry_persists");
        {
            let mut registry = MirrorRegistry::open(&tmp).unwrap();
            let form = Form::new(
                DeclKind::Prism,
                "@prism",
                Vec::new(),
                Vec::new(),
                Vec::new(),
            );
            registry.register(&form);
            registry.flush();
        }
        // Reopen — cache is gone, but the disk + refs persist.
        let registry = MirrorRegistry::open(&tmp).unwrap();
        let stored = registry
            .lookup("@prism")
            .expect("@prism survives reopen via disk");
        let shatter = Shatter;
        let restored = shatter.decompile(&stored);
        assert_eq!(restored.name, "@prism");
    }

    #[test]
    fn registry_resolves_in_reference_when_target_in_store() {
        let tmp = tempdir_for_test("registry_resolves_in");
        let mut registry = MirrorRegistry::open(&tmp).unwrap();

        let prism_form = Form::new(
            DeclKind::Prism,
            "@prism",
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        registry.register(&prism_form);

        let file = Form::new(
            DeclKind::Form,
            "",
            Vec::new(),
            Vec::new(),
            vec![Form::new(
                DeclKind::In,
                "@prism",
                Vec::new(),
                Vec::new(),
                Vec::new(),
            )],
        );
        assert!(registry.resolve(&file).is_ok());
    }

    #[test]
    fn registry_resolve_fails_when_in_target_missing() {
        let tmp = tempdir_for_test("registry_resolve_missing");
        let registry = MirrorRegistry::open(&tmp).unwrap();
        let file = Form::new(
            DeclKind::Form,
            "",
            Vec::new(),
            Vec::new(),
            vec![Form::new(
                DeclKind::In,
                "@nonexistent",
                Vec::new(),
                Vec::new(),
                Vec::new(),
            )],
        );
        let err = registry.resolve(&file).unwrap_err();
        assert!(
            err.0.contains("@nonexistent"),
            "error message should mention the missing form: {}",
            err.0
        );
    }

    #[test]
    fn registry_resolve_uses_disk_after_reopen() {
        let tmp = tempdir_for_test("registry_resolve_disk");
        {
            let mut registry = MirrorRegistry::open(&tmp).unwrap();
            let prism_form = Form::new(
                DeclKind::Prism,
                "@prism",
                Vec::new(),
                Vec::new(),
                Vec::new(),
            );
            registry.register(&prism_form);
            registry.flush();
        }
        let registry = MirrorRegistry::open(&tmp).unwrap();
        let file = Form::new(
            DeclKind::Form,
            "",
            Vec::new(),
            Vec::new(),
            vec![Form::new(
                DeclKind::In,
                "@prism",
                Vec::new(),
                Vec::new(),
                Vec::new(),
            )],
        );
        assert!(
            registry.resolve(&file).is_ok(),
            "resolve must use store ref lookup, not in-memory state"
        );
    }
}
