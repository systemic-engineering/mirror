//! AST types mirroring the C struct `ast_node_t`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AstKind {
    Focus,
    Project,
    Split,
    Shift,
    Settle,
    In,
    Out,
    /// `io <name>(<args>) = <lens-call> > <selector>` — Spec A.
    /// The Turing-complete escape hatch. Body lives behind a `~f` reference
    /// resolved through a body lens and narrowed by a CSS-style selector.
    IoBinding,
    /// `match <subject> { <arm> => <body>, ... }` — Spec B.
    /// Structural dispatch. Patterns are mq queries over the subject's type.
    MatchExpr,
    /// `select |<binder>| { <variant> => <body>, ... }` — Spec B.
    /// Closure-style sum-type dispatch. Slots in next to recover/rescue.
    SelectExpr,
    /// `T(U)` — parametric type application at the type layer.
    ///
    /// Per the parametric-types-and-fp-heritage insight (2026-05-25):
    /// `T(U)` is the type-layer application of `T` to `U`. The type checker
    /// treats `T(U1)` and `T(U2)` as distinct types (different content
    /// hashes). Bare `T` (no parens) remains a non-parametric type ref.
    ///
    /// `name` carries the verbatim form `T(U)` (or `T(U, V)`, etc.) so
    /// content addressing distinguishes parametric instances at the OID
    /// layer. The shape is land-on-the-OID — the bootstrap doesn't carry
    /// a structured base/parameter split because the verbatim form is
    /// what hashes; downstream lenses extract the components from `name`.
    ParametricType,
    /// A span of unrecognized bytes — `total_classification` failure.
    ///
    /// The bytes are preserved verbatim in `body` (for round-trip rendering)
    /// and the source span (line/column start + end) is recorded in
    /// `dark_span` for diagnostics. `content_oid` hashes these bytes with a
    /// `"dark"` tag so the silent-absorption mode dies: changes to the dark
    /// region produce different OIDs.
    ///
    /// Per `docs/specs/strict-and-total-classification.md`.
    Dark,
}

/// Half-open byte span `[start, end)` into the source file. (0, 0) = unknown.
///
/// Line/column for diagnostics is computed on demand from the source bytes
/// — the AST itself only carries byte offsets so multi-byte UTF-8 stays
/// honest and the tokenizer doesn't pay for line counting it doesn't need.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DarkSpan {
    pub start: usize,
    pub end: usize,
}

/// Resolve a byte offset into 1-based (line, col), counting `\n` as the line
/// break. Column is 1-based bytes into the current line (ASCII assumed for
/// the boot tree). Returns (1, 1) for offset 0 in any source.
pub fn line_col_at(source: &[u8], offset: usize) -> (u32, u32) {
    let off = offset.min(source.len());
    let mut line: u32 = 1;
    let mut last_nl: usize = 0;
    for i in 0..off {
        if source[i] == b'\n' {
            line += 1;
            last_nl = i + 1;
        }
    }
    let col: u32 = (off - last_nl) as u32 + 1;
    (line, col)
}

#[derive(Debug, Clone)]
pub struct AstNode {
    pub kind: AstKind,
    pub name: String,
    /// which grammar tokenized this node, e.g. "@code/llvm/ir"
    pub grammar_tag: String,
    /// original surface keyword (used for round-trip when one kind maps to
    /// several keywords). Empty string means "use the grammar reverse lookup".
    pub keyword: String,
    /// Verbatim body text for opaque nodes (LLVM IR function bodies,
    /// global declarations, attribute groups). None when not used.
    ///
    /// For `AstKind::Dark`, this holds the verbatim unrecognized bytes —
    /// the renderer round-trips them as-is and `content_oid` hashes them
    /// with a `"dark"` tag so the silent-absorption mode dies.
    pub body: Option<String>,
    pub children: Vec<AstNode>,
    /// Source span — populated for `AstKind::Dark` for diagnostics.
    pub dark_span: DarkSpan,
}

impl AstNode {
    pub fn new(kind: AstKind, name: &str) -> Self {
        // The C side hard-truncates `name` to 255 bytes (name[256] field).
        // Mirror that to keep the OID derivation byte-exact.
        let mut n = name.to_string();
        if n.len() > 255 {
            n.truncate(255);
        }
        AstNode {
            kind,
            name: n,
            grammar_tag: String::new(),
            keyword: String::new(),
            body: None,
            children: Vec::new(),
            dark_span: DarkSpan::default(),
        }
    }

    /// Construct a Dark child carrying the verbatim unrecognized bytes and
    /// a source span pointing at the region. The bytes feed `content_oid`
    /// (under a `"dark"` tag) and the renderer (verbatim round-trip).
    pub fn dark(bytes: &str, span: DarkSpan) -> Self {
        let mut node = AstNode::new(AstKind::Dark, "");
        node.body = Some(bytes.to_string());
        node.dark_span = span;
        node
    }

    pub fn add_child(&mut self, child: AstNode) {
        self.children.push(child);
    }

    pub fn set_body(&mut self, body: &str) {
        self.body = Some(body.to_string());
    }

    pub fn tag_recursive(&mut self, tag: &str) {
        // C side hard-truncates tag to 127 bytes (grammar_tag[128]).
        let mut t = tag.to_string();
        if t.len() > 127 {
            t.truncate(127);
        }
        self.grammar_tag = t.clone();
        for c in self.children.iter_mut() {
            c.tag_recursive(&t);
        }
    }

    /// Set the original surface keyword (truncated to 63 bytes like C).
    pub fn set_keyword(&mut self, kw: &str) {
        let mut k = kw.to_string();
        if k.len() > 63 {
            k.truncate(63);
        }
        self.keyword = k;
    }
}
