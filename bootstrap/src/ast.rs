//! AST types mirroring the C struct `ast_node_t`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AstKind {
    Focus,
    Project,
    Split,
    Zoom,
    Refract,
    In,
    Out,
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
    pub body: Option<String>,
    pub children: Vec<AstNode>,
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
        }
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
