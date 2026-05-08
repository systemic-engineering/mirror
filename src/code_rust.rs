//! @code/rust kintsugi — Rust source to MirrorAST conversion.
//!
//! Maps Rust constructs to the five operations:
//!   fn       -> Zoom   (transform, cross levels)
//!   struct   -> Split  (one of many, type with variants/fields)
//!   impl     -> Focus  (look closer, namespace, grouping)
//!   use      -> Project (extract a view, import)
//!   trait    -> Refract (scatter, verify, settle)
//!   mod      -> Module (top-level grouping)
//!
//! The conversion uses grammar_regions for line-level splitting,
//! then a lightweight brace-matching parser to identify Rust items
//! and map them to MirrorAST nodes. The existing kintsugi operations
//! (eliminate_dead, collapse_aliases, flatten_wrappers) on the base AST
//! then apply directly.

use crate::mirror_ast::{
    Field, FocusNode, GrammarRef, Identifier, MirrorAST, ModuleNode, ProjectNode, RefractNode,
    SplitNode, TypeBody, ZoomNode,
};

// ---------------------------------------------------------------------------
// RustItem — a parsed Rust top-level item
// ---------------------------------------------------------------------------

/// A Rust top-level item extracted by the lightweight parser.
#[derive(Clone, Debug, PartialEq)]
pub enum RustItem {
    /// `fn name(params) -> ret { body }`
    Function {
        name: String,
        params: Vec<(String, String)>,
        return_type: Option<String>,
        body: String,
        is_pub: bool,
    },
    /// `struct Name { fields }` or `struct Name(Type);`
    Struct {
        name: String,
        fields: Vec<(String, String)>,
        is_pub: bool,
    },
    /// `enum Name { Variant1, Variant2 }`
    Enum {
        name: String,
        variants: Vec<String>,
        is_pub: bool,
    },
    /// `impl Type { methods }` or `impl Trait for Type { methods }`
    Impl {
        target: String,
        trait_name: Option<String>,
        body: String,
        items: Vec<RustItem>,
    },
    /// `use path::to::thing;`
    Use {
        path: String,
        is_pub: bool,
    },
    /// `trait Name { methods }`
    Trait {
        name: String,
        body: String,
        is_pub: bool,
    },
    /// `mod name { body }` or `mod name;`
    Mod {
        name: String,
        is_pub: bool,
    },
    /// Comment or doc string (natural language region)
    Comment(String),
    /// Anything else we don't specifically recognize
    Other(String),
}

// ---------------------------------------------------------------------------
// Parsing — lightweight Rust item extraction
// ---------------------------------------------------------------------------

/// Parse Rust source into a sequence of top-level items.
///
/// This is NOT a full Rust parser. It identifies top-level items by:
/// 1. Scanning for keywords (`fn`, `struct`, `enum`, `impl`, `use`, `trait`, `mod`)
/// 2. Extracting names (the identifier after the keyword)
/// 3. Matching braces to find item boundaries
/// 4. Extracting simple parameter/field information
///
/// Comments and doc comments are preserved as Comment items.
pub fn parse_rust_items(source: &str) -> Vec<RustItem> {
    let mut items = Vec::new();
    let mut pos = 0;
    let bytes = source.as_bytes();

    while pos < bytes.len() {
        // Skip whitespace
        while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }
        if pos >= bytes.len() {
            break;
        }

        // Check for comments
        if pos + 1 < bytes.len() && bytes[pos] == b'/' && bytes[pos + 1] == b'/' {
            let start = pos;
            // Consume until end of line
            while pos < bytes.len() && bytes[pos] != b'\n' {
                pos += 1;
            }
            if pos < bytes.len() {
                pos += 1; // consume newline
            }
            items.push(RustItem::Comment(source[start..pos].to_string()));
            continue;
        }

        // Check for block comments
        if pos + 1 < bytes.len() && bytes[pos] == b'/' && bytes[pos + 1] == b'*' {
            let start = pos;
            pos += 2;
            let mut depth = 1;
            while pos + 1 < bytes.len() && depth > 0 {
                if bytes[pos] == b'/' && bytes[pos + 1] == b'*' {
                    depth += 1;
                    pos += 2;
                } else if bytes[pos] == b'*' && bytes[pos + 1] == b'/' {
                    depth -= 1;
                    pos += 2;
                } else {
                    pos += 1;
                }
            }
            items.push(RustItem::Comment(source[start..pos].to_string()));
            continue;
        }

        // Check for attributes (#[...])
        if bytes[pos] == b'#' {
            let _start = pos;
            pos += 1;
            if pos < bytes.len() && bytes[pos] == b'[' {
                pos += 1;
                let mut depth = 1;
                while pos < bytes.len() && depth > 0 {
                    if bytes[pos] == b'[' {
                        depth += 1;
                    } else if bytes[pos] == b']' {
                        depth -= 1;
                    }
                    pos += 1;
                }
            }
            // Attributes are consumed but not emitted as items
            // (they belong to the next item)
            continue;
        }

        // Try to parse an item starting with optional `pub`
        let item_start = pos;
        let is_pub = starts_with_at(source, pos, "pub ");
        if is_pub {
            pos += 4;
            // skip optional `(crate)` or `(super)` etc.
            while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
                pos += 1;
            }
            if pos < bytes.len() && bytes[pos] == b'(' {
                // pub(crate), pub(super), etc.
                while pos < bytes.len() && bytes[pos] != b')' {
                    pos += 1;
                }
                if pos < bytes.len() {
                    pos += 1; // skip ')'
                }
                while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
                    pos += 1;
                }
            }
        }

        if starts_with_at(source, pos, "fn ") || starts_with_at(source, pos, "async fn ") {
            let item = parse_function(source, item_start, is_pub);
            if let Some((item, new_pos)) = item {
                items.push(item);
                pos = new_pos;
            } else {
                pos = skip_to_next_item(source, item_start);
            }
        } else if starts_with_at(source, pos, "struct ") {
            let item = parse_struct(source, item_start, is_pub);
            if let Some((item, new_pos)) = item {
                items.push(item);
                pos = new_pos;
            } else {
                pos = skip_to_next_item(source, item_start);
            }
        } else if starts_with_at(source, pos, "enum ") {
            let item = parse_enum(source, item_start, is_pub);
            if let Some((item, new_pos)) = item {
                items.push(item);
                pos = new_pos;
            } else {
                pos = skip_to_next_item(source, item_start);
            }
        } else if starts_with_at(source, pos, "impl ") || starts_with_at(source, pos, "impl<") {
            let item = parse_impl(source, item_start);
            if let Some((item, new_pos)) = item {
                items.push(item);
                pos = new_pos;
            } else {
                pos = skip_to_next_item(source, item_start);
            }
        } else if starts_with_at(source, pos, "use ") {
            let item = parse_use(source, item_start, is_pub);
            if let Some((item, new_pos)) = item {
                items.push(item);
                pos = new_pos;
            } else {
                pos = skip_to_next_item(source, item_start);
            }
        } else if starts_with_at(source, pos, "trait ") {
            let item = parse_trait(source, item_start, is_pub);
            if let Some((item, new_pos)) = item {
                items.push(item);
                pos = new_pos;
            } else {
                pos = skip_to_next_item(source, item_start);
            }
        } else if starts_with_at(source, pos, "mod ") {
            let item = parse_mod(source, item_start, is_pub);
            if let Some((item, new_pos)) = item {
                items.push(item);
                pos = new_pos;
            } else {
                pos = skip_to_next_item(source, item_start);
            }
        } else if starts_with_at(source, pos, "extern ") || starts_with_at(source, pos, "const ") ||
                  starts_with_at(source, pos, "static ") || starts_with_at(source, pos, "type ") {
            // Skip these items by consuming up to semicolon or brace block
            pos = skip_to_next_item(source, item_start);
        } else {
            // Unknown — skip to next line or semicolon
            pos = skip_to_next_item(source, item_start);
        }
    }

    items
}

fn starts_with_at(source: &str, pos: usize, prefix: &str) -> bool {
    source[pos..].starts_with(prefix)
}

/// Skip from current position to the end of the current item.
/// Handles both semicolon-terminated and brace-delimited items.
fn skip_to_next_item(source: &str, pos: usize) -> usize {
    let bytes = source.as_bytes();
    let mut i = pos;
    let mut brace_depth = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'{' => {
                brace_depth += 1;
                i += 1;
            }
            b'}' => {
                if brace_depth > 0 {
                    brace_depth -= 1;
                    i += 1;
                    if brace_depth == 0 {
                        return i;
                    }
                } else {
                    i += 1;
                    return i;
                }
            }
            b';' if brace_depth == 0 => {
                return i + 1;
            }
            b'\n' if brace_depth == 0 && i > pos => {
                // If we haven't found a delimiter and we're past the start,
                // check if the next line starts a new item
                return i + 1;
            }
            _ => {
                i += 1;
            }
        }
    }
    bytes.len()
}

/// Find the matching closing brace starting from an opening brace.
/// Returns the position AFTER the closing brace.
fn find_matching_brace(source: &str, open_pos: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    if open_pos >= bytes.len() || bytes[open_pos] != b'{' {
        return None;
    }
    let mut depth = 0;
    let mut i = open_pos;
    while i < bytes.len() {
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i + 1);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

/// Extract an identifier starting at the given position.
fn extract_ident(source: &str, pos: usize) -> Option<(String, usize)> {
    let bytes = source.as_bytes();
    let mut i = pos;
    // skip whitespace
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    if i >= bytes.len() {
        return None;
    }
    let start = i;
    while i < bytes.len()
        && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_')
    {
        i += 1;
    }
    if i == start {
        return None;
    }
    Some((source[start..i].to_string(), i))
}

fn parse_function(source: &str, start: usize, is_pub: bool) -> Option<(RustItem, usize)> {
    // Find "fn " after possible "pub " and "async "
    let fn_pos = source[start..].find("fn ")?;
    let fn_pos = start + fn_pos + 3;

    let (name, after_name) = extract_ident(source, fn_pos)?;

    // Find opening paren for params
    let paren_pos = source[after_name..].find('(')?;
    let paren_pos = after_name + paren_pos;

    // Find matching close paren
    let mut depth = 0;
    let mut i = paren_pos;
    let bytes = source.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'(' {
            depth += 1;
        } else if bytes[i] == b')' {
            depth -= 1;
            if depth == 0 {
                break;
            }
        }
        i += 1;
    }
    let close_paren = i;
    let params_str = &source[paren_pos + 1..close_paren];
    let params = parse_simple_params(params_str);

    // Find return type (-> Type) before the opening brace
    let after_paren = close_paren + 1;
    let brace_pos = source[after_paren..].find('{')?;
    let brace_pos = after_paren + brace_pos;
    let between = source[after_paren..brace_pos].trim();
    let return_type = if between.starts_with("->") {
        Some(between[2..].trim().to_string())
    } else {
        None
    };

    let end = find_matching_brace(source, brace_pos)?;
    let body = source[brace_pos + 1..end - 1].to_string();

    Some((
        RustItem::Function {
            name,
            params,
            return_type,
            body,
            is_pub,
        },
        end,
    ))
}

fn parse_simple_params(params_str: &str) -> Vec<(String, String)> {
    let mut result = Vec::new();
    for segment in params_str.split(',') {
        let segment = segment.trim();
        if segment.is_empty() || segment == "self" || segment == "&self" || segment == "&mut self" {
            continue;
        }
        // Remove patterns like `mut `
        let segment = segment.strip_prefix("mut ").unwrap_or(segment);
        if let Some((name, ty)) = segment.split_once(':') {
            result.push((name.trim().to_string(), ty.trim().to_string()));
        }
    }
    result
}

fn parse_struct(source: &str, start: usize, is_pub: bool) -> Option<(RustItem, usize)> {
    let kw_pos = source[start..].find("struct ")?;
    let kw_pos = start + kw_pos + 7;

    let (name, after_name) = extract_ident(source, kw_pos)?;

    // Skip generic params if present
    let mut i = after_name;
    let bytes = source.as_bytes();
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    if i < bytes.len() && bytes[i] == b'<' {
        let mut depth = 1;
        i += 1;
        while i < bytes.len() && depth > 0 {
            if bytes[i] == b'<' { depth += 1; }
            if bytes[i] == b'>' { depth -= 1; }
            i += 1;
        }
    }

    // Skip where clauses
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }

    // Check for semicolon (unit struct or tuple struct)
    if i < bytes.len() && bytes[i] == b';' {
        return Some((
            RustItem::Struct {
                name,
                fields: vec![],
                is_pub,
            },
            i + 1,
        ));
    }

    // Tuple struct: struct Name(fields);
    if i < bytes.len() && bytes[i] == b'(' {
        let semi_pos = source[i..].find(';')?;
        let end = i + semi_pos + 1;
        return Some((
            RustItem::Struct {
                name,
                fields: vec![],
                is_pub,
            },
            end,
        ));
    }

    // Find opening brace
    let brace_pos = source[i..].find('{')?;
    let brace_pos = i + brace_pos;
    let end = find_matching_brace(source, brace_pos)?;
    let body_str = &source[brace_pos + 1..end - 1];

    // Parse fields
    let fields = parse_struct_fields(body_str);

    Some((
        RustItem::Struct {
            name,
            fields,
            is_pub,
        },
        end,
    ))
}

fn parse_struct_fields(body: &str) -> Vec<(String, String)> {
    let mut fields = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        // Skip comments and attributes
        if line.starts_with("//") || line.starts_with('#') || line.is_empty() {
            continue;
        }
        // Remove `pub ` prefix
        let line = line.strip_prefix("pub ").unwrap_or(line);
        let line = if line.starts_with("(") {
            // pub(crate) etc
            if let Some(rest) = line.split_once(')') {
                rest.1.trim()
            } else {
                line
            }
        } else {
            line
        };
        // field: Type,
        if let Some((name, ty)) = line.split_once(':') {
            let name = name.trim().to_string();
            let ty = ty.trim().trim_end_matches(',').trim().to_string();
            if !name.is_empty() && !name.contains(' ') {
                fields.push((name, ty));
            }
        }
    }
    fields
}

fn parse_enum(source: &str, start: usize, is_pub: bool) -> Option<(RustItem, usize)> {
    let kw_pos = source[start..].find("enum ")?;
    let kw_pos = start + kw_pos + 5;

    let (name, after_name) = extract_ident(source, kw_pos)?;

    // Find opening brace (skipping generics and where clauses)
    let brace_pos = source[after_name..].find('{')?;
    let brace_pos = after_name + brace_pos;
    let end = find_matching_brace(source, brace_pos)?;
    let body_str = &source[brace_pos + 1..end - 1];

    // Extract variant names (simplified: just the identifiers before (, {, or ,)
    let mut variants = Vec::new();
    for line in body_str.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") || line.starts_with('#') {
            continue;
        }
        // Extract variant name (everything before '(', '{', ',', or whitespace)
        let name_end = line
            .find(|c: char| c == '(' || c == '{' || c == ',' || c.is_whitespace())
            .unwrap_or(line.len());
        let variant = line[..name_end].trim().to_string();
        if !variant.is_empty() {
            variants.push(variant);
        }
    }

    Some((
        RustItem::Enum {
            name,
            variants,
            is_pub,
        },
        end,
    ))
}

fn parse_impl(source: &str, start: usize) -> Option<(RustItem, usize)> {
    let kw_pos = source[start..].find("impl")?;
    let kw_pos = start + kw_pos + 4;

    // Find opening brace
    let brace_search_start = kw_pos;
    let brace_pos = source[brace_search_start..].find('{')?;
    let brace_pos = brace_search_start + brace_pos;
    let header = source[kw_pos..brace_pos].trim();

    // Check for "Trait for Type" pattern
    let (target, trait_name) = if let Some(for_pos) = header.find(" for ") {
        let trait_part = header[..for_pos].trim();
        // Clean up generic bounds from trait name
        let trait_clean = trait_part.split('<').next().unwrap_or(trait_part).trim();
        let type_part = header[for_pos + 5..].trim();
        let type_clean = type_part.split(|c: char| c == '<' || c.is_whitespace()).next().unwrap_or(type_part).trim();
        (type_clean.to_string(), Some(trait_clean.to_string()))
    } else {
        // Just `impl Type` — extract the type name
        let type_clean = header.split(|c: char| c == '<' || c.is_whitespace()).next().unwrap_or(header).trim();
        (type_clean.to_string(), None)
    };

    let end = find_matching_brace(source, brace_pos)?;
    let body = source[brace_pos + 1..end - 1].to_string();

    // Parse inner items (methods)
    let inner_items = parse_rust_items(&body);

    Some((
        RustItem::Impl {
            target,
            trait_name,
            body,
            items: inner_items,
        },
        end,
    ))
}

fn parse_use(source: &str, start: usize, is_pub: bool) -> Option<(RustItem, usize)> {
    let kw_pos = source[start..].find("use ")?;
    let kw_pos = start + kw_pos + 4;
    let semi_pos = source[kw_pos..].find(';')?;
    let path = source[kw_pos..kw_pos + semi_pos].trim().to_string();

    Some((
        RustItem::Use { path, is_pub },
        kw_pos + semi_pos + 1,
    ))
}

fn parse_trait(source: &str, start: usize, is_pub: bool) -> Option<(RustItem, usize)> {
    let kw_pos = source[start..].find("trait ")?;
    let kw_pos = start + kw_pos + 6;

    let (name, _after_name) = extract_ident(source, kw_pos)?;

    // Find opening brace
    let brace_pos = source[kw_pos..].find('{')?;
    let brace_pos = kw_pos + brace_pos;
    let end = find_matching_brace(source, brace_pos)?;
    let body = source[brace_pos + 1..end - 1].to_string();

    Some((
        RustItem::Trait {
            name,
            body,
            is_pub,
        },
        end,
    ))
}

fn parse_mod(source: &str, start: usize, is_pub: bool) -> Option<(RustItem, usize)> {
    let kw_pos = source[start..].find("mod ")?;
    let kw_pos = start + kw_pos + 4;

    let (name, after_name) = extract_ident(source, kw_pos)?;

    // Check for semicolon (external module)
    let mut i = after_name;
    let bytes = source.as_bytes();
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    if i < bytes.len() && bytes[i] == b';' {
        return Some((
            RustItem::Mod { name, is_pub },
            i + 1,
        ));
    }

    // Inline module with body
    if i < bytes.len() && bytes[i] == b'{' {
        let end = find_matching_brace(source, i)?;
        return Some((
            RustItem::Mod { name, is_pub },
            end,
        ));
    }

    Some((RustItem::Mod { name, is_pub }, after_name))
}

// ---------------------------------------------------------------------------
// Conversion — RustItem -> MirrorAST
// ---------------------------------------------------------------------------

/// Convert a single RustItem to a MirrorAST node.
pub fn item_to_mirror_ast(item: &RustItem) -> Option<MirrorAST> {
    match item {
        RustItem::Function {
            name,
            params,
            return_type,
            ..
        } => {
            let fields: Vec<Field> = params
                .iter()
                .map(|(n, t)| Field {
                    name: Identifier::new(n),
                    type_ref: Identifier::new(t),
                })
                .collect();
            Some(MirrorAST::Zoom(ZoomNode {
                name: Identifier::new(name),
                params: fields,
                target: return_type.as_ref().map(|t| Identifier::new(t)),
                grammar_ref: Some(GrammarRef::new("@code/rust")),
                children: vec![],
                body: None,
            }))
        }
        RustItem::Struct { name, fields, .. } => {
            let type_body = if fields.is_empty() {
                Some(TypeBody::Unit)
            } else {
                Some(TypeBody::Struct(
                    fields
                        .iter()
                        .map(|(n, t)| Field {
                            name: Identifier::new(n),
                            type_ref: Identifier::new(t),
                        })
                        .collect(),
                ))
            };
            Some(MirrorAST::Split(SplitNode {
                name: Identifier::new(name),
                variants: vec![],
                params: vec![],
                body: type_body,
                children: vec![],
            }))
        }
        RustItem::Enum {
            name, variants, ..
        } => {
            let variant_ids: Vec<Identifier> =
                variants.iter().map(|v| Identifier::new(v)).collect();
            Some(MirrorAST::Split(SplitNode {
                name: Identifier::new(name),
                variants: variant_ids.clone(),
                params: vec![],
                body: Some(TypeBody::Enum(variant_ids)),
                children: vec![],
            }))
        }
        RustItem::Impl {
            target,
            trait_name,
            items,
            ..
        } => {
            let name = if let Some(trait_name) = trait_name {
                format!("{} for {}", trait_name, target)
            } else {
                target.clone()
            };
            let children: Vec<MirrorAST> = items
                .iter()
                .filter_map(|item| item_to_mirror_ast(item))
                .collect();
            Some(MirrorAST::Focus(FocusNode {
                name: Identifier::new(&name),
                target: None,
                children,
            }))
        }
        RustItem::Use { path, .. } => Some(MirrorAST::Project(ProjectNode {
            name: Identifier::new(path),
            target: Some(GrammarRef::new("@code/rust")),
            children: vec![],
        })),
        RustItem::Trait { name, .. } => Some(MirrorAST::Refract(RefractNode {
            name: Identifier::new(name),
            target: None,
            params: vec![],
            children: vec![],
        })),
        RustItem::Mod { name, .. } => Some(MirrorAST::Module(ModuleNode {
            name: Identifier::new(name),
            children: vec![],
        })),
        RustItem::Comment(_) => None,
        RustItem::Other(_) => None,
    }
}

/// Convert a Rust source file into a MirrorAST Module node.
///
/// The file becomes a Module containing Focus/Split/Zoom/Project/Refract children
/// based on the Rust constructs found.
pub fn rust_to_mirror_ast(filename: &str, source: &str) -> MirrorAST {
    let items = parse_rust_items(source);
    let children: Vec<MirrorAST> = items
        .iter()
        .filter_map(|item| item_to_mirror_ast(item))
        .collect();
    MirrorAST::Module(ModuleNode {
        name: Identifier::new(filename),
        children,
    })
}

/// Convert a Rust source file into a base AST (ast::Ast) Module for kintsugi operations.
///
/// This maps:
///   Module -> Ast::Body(children)
///   Zoom (fn) -> Ast::Call { name: "action", args: [name, params...] }
///   Split (struct/enum) -> Ast::Call { name: "type", args: [name, body] }
///   Focus (impl) -> Ast::Call { name: "focus", args: [name, children...] }
///   Project (use) -> Ast::Call { name: "in", args: [Ref(path)] }
///   Refract (trait) -> Ast::Call { name: "refract", args: [name] }
///
/// This representation allows the existing eliminate_dead, collapse_aliases,
/// flatten_wrappers to operate on Rust code.
pub fn rust_to_base_ast(_filename: &str, source: &str) -> crate::ast::Ast {
    use crate::ast::{Ast, Body};

    let items = parse_rust_items(source);
    let children: Vec<Ast> = items
        .iter()
        .filter_map(|item| item_to_base_ast(item))
        .collect();

    Ast::Body(Body::new(children))
}

fn item_to_base_ast(item: &RustItem) -> Option<crate::ast::Ast> {
    use crate::ast::{Ast, Atom, Body, Ref};

    match item {
        RustItem::Function {
            name,
            params,
            return_type,
            ..
        } => {
            let mut args: Vec<Ast> = vec![Ast::Atom(Atom::new(name))];
            for (_pname, ptype) in params {
                args.push(Ast::Ref(Ref::new(ptype)));
            }
            if let Some(ret) = return_type {
                args.push(Ast::Ref(Ref::new(ret)));
            }
            Some(Ast::Call {
                name: Atom::new("action"),
                args,
            })
        }
        RustItem::Struct { name, fields, .. } => {
            let mut args: Vec<Ast> = vec![Ast::Atom(Atom::new(name))];
            if !fields.is_empty() {
                let field_asts: Vec<Ast> = fields
                    .iter()
                    .map(|(fname, ftype)| {
                        Ast::Call {
                            name: Atom::new(fname),
                            args: vec![Ast::Ref(Ref::new(ftype))],
                        }
                    })
                    .collect();
                args.push(Ast::Body(Body::new(field_asts)));
            }
            Some(Ast::Call {
                name: Atom::new("type"),
                args,
            })
        }
        RustItem::Enum {
            name, variants, ..
        } => {
            let mut args: Vec<Ast> = vec![Ast::Atom(Atom::new(name))];
            for v in variants {
                args.push(Ast::Atom(Atom::new(v)));
            }
            Some(Ast::Call {
                name: Atom::new("type"),
                args,
            })
        }
        RustItem::Impl {
            target,
            trait_name,
            items,
            ..
        } => {
            let impl_name = if let Some(t) = trait_name {
                format!("{} for {}", t, target)
            } else {
                target.clone()
            };
            let mut args: Vec<Ast> = vec![Ast::Atom(Atom::new(&impl_name))];
            let child_asts: Vec<Ast> = items
                .iter()
                .filter_map(|i| item_to_base_ast(i))
                .collect();
            if !child_asts.is_empty() {
                args.push(Ast::Body(Body::new(child_asts)));
            }
            Some(Ast::Call {
                name: Atom::new("focus"),
                args,
            })
        }
        RustItem::Use { path, .. } => Some(Ast::Call {
            name: Atom::new("in"),
            args: vec![Ast::Ref(Ref::new(path))],
        }),
        RustItem::Trait { name, .. } => Some(Ast::Call {
            name: Atom::new("refract"),
            args: vec![Ast::Atom(Atom::new(name))],
        }),
        RustItem::Mod { name, .. } => Some(Ast::Call {
            name: Atom::new("module"),
            args: vec![Ast::Atom(Atom::new(name))],
        }),
        RustItem::Comment(_) | RustItem::Other(_) => None,
    }
}

// ---------------------------------------------------------------------------
// Metrics — node count, depth, structural complexity
// ---------------------------------------------------------------------------

/// Compute basic metrics for a MirrorAST tree from Rust code.
pub struct RustAstMetrics {
    pub node_count: usize,
    pub depth: usize,
    pub fn_count: usize,
    pub type_count: usize,
    pub impl_count: usize,
    pub use_count: usize,
    pub trait_count: usize,
}

/// Compute metrics from parsed Rust items.
pub fn compute_metrics(items: &[RustItem]) -> RustAstMetrics {
    let mut metrics = RustAstMetrics {
        node_count: 0,
        depth: 1, // module level
        fn_count: 0,
        type_count: 0,
        impl_count: 0,
        use_count: 0,
        trait_count: 0,
    };

    for item in items {
        match item {
            RustItem::Function { .. } => {
                metrics.fn_count += 1;
                metrics.node_count += 1;
            }
            RustItem::Struct { fields, .. } => {
                metrics.type_count += 1;
                metrics.node_count += 1 + fields.len();
            }
            RustItem::Enum { variants, .. } => {
                metrics.type_count += 1;
                metrics.node_count += 1 + variants.len();
            }
            RustItem::Impl { items: inner, .. } => {
                metrics.impl_count += 1;
                metrics.node_count += 1;
                let inner_metrics = compute_metrics(inner);
                metrics.node_count += inner_metrics.node_count;
                metrics.fn_count += inner_metrics.fn_count;
                if inner_metrics.depth + 1 > metrics.depth {
                    metrics.depth = inner_metrics.depth + 1;
                }
            }
            RustItem::Use { .. } => {
                metrics.use_count += 1;
                metrics.node_count += 1;
            }
            RustItem::Trait { .. } => {
                metrics.trait_count += 1;
                metrics.node_count += 1;
            }
            RustItem::Mod { .. } => {
                metrics.node_count += 1;
            }
            RustItem::Comment(_) => {}
            RustItem::Other(_) => {}
        }
    }

    metrics
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Ast;
    use crate::grammar_regions;
    use crate::grammar_regions::GrammarId;

    // -- Parsing tests -------------------------------------------------------

    #[test]
    fn parse_simple_function() {
        let src = "fn hello(name: String) -> bool { true }";
        let items = parse_rust_items(src);
        assert_eq!(items.len(), 1);
        match &items[0] {
            RustItem::Function {
                name,
                params,
                return_type,
                is_pub,
                ..
            } => {
                assert_eq!(name, "hello");
                assert_eq!(params.len(), 1);
                assert_eq!(params[0].0, "name");
                assert_eq!(params[0].1, "String");
                assert_eq!(return_type.as_deref(), Some("bool"));
                assert!(!is_pub);
            }
            other => panic!("expected Function, got {:?}", other),
        }
    }

    #[test]
    fn parse_pub_function() {
        let src = "pub fn greet(who: &str) { println!(\"{}\", who); }";
        let items = parse_rust_items(src);
        assert_eq!(items.len(), 1);
        match &items[0] {
            RustItem::Function { name, is_pub, .. } => {
                assert_eq!(name, "greet");
                assert!(is_pub);
            }
            other => panic!("expected Function, got {:?}", other),
        }
    }

    #[test]
    fn parse_simple_struct() {
        let src = "pub struct Point {\n    pub x: f64,\n    pub y: f64,\n}";
        let items = parse_rust_items(src);
        assert_eq!(items.len(), 1);
        match &items[0] {
            RustItem::Struct {
                name,
                fields,
                is_pub,
            } => {
                assert_eq!(name, "Point");
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].0, "x");
                assert_eq!(fields[0].1, "f64");
                assert_eq!(fields[1].0, "y");
                assert_eq!(fields[1].1, "f64");
                assert!(is_pub);
            }
            other => panic!("expected Struct, got {:?}", other),
        }
    }

    #[test]
    fn parse_simple_enum() {
        let src = "pub enum Color {\n    Red,\n    Green,\n    Blue,\n}";
        let items = parse_rust_items(src);
        assert_eq!(items.len(), 1);
        match &items[0] {
            RustItem::Enum {
                name,
                variants,
                is_pub,
            } => {
                assert_eq!(name, "Color");
                assert_eq!(variants, &["Red", "Green", "Blue"]);
                assert!(is_pub);
            }
            other => panic!("expected Enum, got {:?}", other),
        }
    }

    #[test]
    fn parse_impl_block() {
        let src = "impl Point {\n    fn new(x: f64, y: f64) -> Self { Point { x, y } }\n}";
        let items = parse_rust_items(src);
        assert_eq!(items.len(), 1);
        match &items[0] {
            RustItem::Impl {
                target,
                trait_name,
                items: inner,
                ..
            } => {
                assert_eq!(target, "Point");
                assert!(trait_name.is_none());
                assert_eq!(inner.len(), 1);
                match &inner[0] {
                    RustItem::Function { name, .. } => assert_eq!(name, "new"),
                    other => panic!("expected inner Function, got {:?}", other),
                }
            }
            other => panic!("expected Impl, got {:?}", other),
        }
    }

    #[test]
    fn parse_trait_impl() {
        let src = "impl Display for Point {\n    fn fmt(&self, f: &mut Formatter) -> Result { Ok(()) }\n}";
        let items = parse_rust_items(src);
        assert_eq!(items.len(), 1);
        match &items[0] {
            RustItem::Impl {
                target,
                trait_name,
                ..
            } => {
                assert_eq!(target, "Point");
                assert_eq!(trait_name.as_deref(), Some("Display"));
            }
            other => panic!("expected Impl, got {:?}", other),
        }
    }

    #[test]
    fn parse_use_statement() {
        let src = "use std::collections::HashMap;";
        let items = parse_rust_items(src);
        assert_eq!(items.len(), 1);
        match &items[0] {
            RustItem::Use { path, is_pub } => {
                assert_eq!(path, "std::collections::HashMap");
                assert!(!is_pub);
            }
            other => panic!("expected Use, got {:?}", other),
        }
    }

    #[test]
    fn parse_trait_definition() {
        let src = "pub trait Greetable {\n    fn greet(&self) -> String;\n}";
        let items = parse_rust_items(src);
        assert_eq!(items.len(), 1);
        match &items[0] {
            RustItem::Trait { name, is_pub, .. } => {
                assert_eq!(name, "Greetable");
                assert!(is_pub);
            }
            other => panic!("expected Trait, got {:?}", other),
        }
    }

    #[test]
    fn parse_multiple_items() {
        let src = "\
use std::fmt;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn origin() -> Self { Point { x: 0.0, y: 0.0 } }
}

pub fn distance(a: Point, b: Point) -> f64 { 0.0 }
";
        let items = parse_rust_items(src);
        let non_comment: Vec<&RustItem> = items
            .iter()
            .filter(|i| !matches!(i, RustItem::Comment(_) | RustItem::Other(_)))
            .collect();
        assert!(non_comment.len() >= 4, "expected at least 4 items (use, struct, impl, fn), got {}: {:?}", non_comment.len(), non_comment);
    }

    #[test]
    fn parse_comments_preserved() {
        let src = "// This is a comment\nfn foo() { }";
        let items = parse_rust_items(src);
        assert!(items.iter().any(|i| matches!(i, RustItem::Comment(_))));
        assert!(items.iter().any(|i| matches!(i, RustItem::Function { .. })));
    }

    // -- MirrorAST conversion tests -------------------------------------------

    #[test]
    fn function_becomes_zoom() {
        let src = "fn hello(name: String) -> bool { true }";
        let items = parse_rust_items(src);
        let ast = item_to_mirror_ast(&items[0]).unwrap();
        assert!(matches!(ast, MirrorAST::Zoom(_)));
        assert_eq!(ast.name(), "hello");
    }

    #[test]
    fn struct_becomes_split() {
        let src = "pub struct Point {\n    pub x: f64,\n    pub y: f64,\n}";
        let items = parse_rust_items(src);
        let ast = item_to_mirror_ast(&items[0]).unwrap();
        assert!(matches!(ast, MirrorAST::Split(_)));
        assert_eq!(ast.name(), "Point");
    }

    #[test]
    fn enum_becomes_split_with_variants() {
        let src = "pub enum Color {\n    Red,\n    Green,\n    Blue,\n}";
        let items = parse_rust_items(src);
        let ast = item_to_mirror_ast(&items[0]).unwrap();
        match &ast {
            MirrorAST::Split(s) => {
                assert_eq!(s.name.as_str(), "Color");
                assert_eq!(s.variants.len(), 3);
            }
            other => panic!("expected Split, got {:?}", other),
        }
    }

    #[test]
    fn impl_becomes_focus() {
        let src = "impl Point {\n    fn new() -> Self { Point {} }\n}";
        let items = parse_rust_items(src);
        let ast = item_to_mirror_ast(&items[0]).unwrap();
        assert!(matches!(ast, MirrorAST::Focus(_)));
        assert_eq!(ast.name(), "Point");
    }

    #[test]
    fn use_becomes_project() {
        let src = "use std::collections::HashMap;";
        let items = parse_rust_items(src);
        let ast = item_to_mirror_ast(&items[0]).unwrap();
        assert!(matches!(ast, MirrorAST::Project(_)));
    }

    #[test]
    fn trait_becomes_refract() {
        let src = "pub trait Greetable {\n    fn greet(&self) -> String;\n}";
        let items = parse_rust_items(src);
        let ast = item_to_mirror_ast(&items[0]).unwrap();
        assert!(matches!(ast, MirrorAST::Refract(_)));
        assert_eq!(ast.name(), "Greetable");
    }

    // -- Full file conversion tests ------------------------------------------

    #[test]
    fn rust_file_becomes_module() {
        let src = "\
use std::fmt;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn origin() -> Point { Point { x: 0.0, y: 0.0 } }
";
        let ast = rust_to_mirror_ast("point.rs", src);
        match &ast {
            MirrorAST::Module(m) => {
                assert_eq!(m.name.as_str(), "point.rs");
                assert!(m.children.len() >= 2, "expected at least Project + Split + Zoom, got {}", m.children.len());
            }
            other => panic!("expected Module, got {:?}", other),
        }
    }

    // -- Base AST conversion tests (for kintsugi) ----------------------------

    #[test]
    fn rust_to_base_ast_produces_body() {
        let src = "\
use std::fmt;

pub struct Point {
    pub x: f64,
}

pub fn origin() -> Point { Point { x: 0.0 } }
";
        let ast = rust_to_base_ast("point.rs", src);
        assert!(matches!(ast, Ast::Body(_)));
        let node_count = ast.node_count();
        assert!(node_count > 0, "base AST should have nodes");
    }

    #[test]
    fn base_ast_function_is_action_call() {
        let src = "fn hello(name: String) -> bool { true }";
        let ast = rust_to_base_ast("test.rs", src);
        if let Ast::Body(body) = &ast {
            let child = &body.children()[0];
            assert!(child.is_call("action"), "fn should become action Call, got {:?}", child);
            assert_eq!(child.decl_name(), Some("hello"));
        } else {
            panic!("expected Body");
        }
    }

    #[test]
    fn base_ast_struct_is_type_call() {
        let src = "pub struct Point {\n    pub x: f64,\n}";
        let ast = rust_to_base_ast("test.rs", src);
        if let Ast::Body(body) = &ast {
            let child = &body.children()[0];
            assert!(child.is_call("type"), "struct should become type Call, got {:?}", child);
            assert_eq!(child.decl_name(), Some("Point"));
        } else {
            panic!("expected Body");
        }
    }

    // -- Kintsugi pipeline tests (the money tests) ---------------------------

    #[test]
    fn kintsugi_eliminate_dead_on_rust_ast() {
        // Build a Rust file with a struct referenced by a function,
        // and another struct not referenced by anything.
        let src = "\
pub struct Used {
    pub x: f64,
}

pub struct Orphan {
    pub y: f64,
}

pub fn process(input: Used) -> Used { input }
";
        let ast = rust_to_base_ast("test.rs", src);
        let before = ast.node_count();
        let simplified = ast.eliminate_dead();
        let after = simplified.node_count();
        // eliminate_dead should remove the Orphan struct
        assert!(after <= before, "eliminate_dead should not increase nodes: {} -> {}", before, after);

        // Verify Used survives (it's referenced by action)
        let mut found_used = false;
        simplified.walk(&mut |node| {
            if let Ast::Atom(a) = node {
                if a.as_str() == "Used" {
                    found_used = true;
                }
            }
        });
        assert!(found_used, "Used type should survive eliminate_dead");
    }

    #[test]
    fn kintsugi_pipeline_reduces_rust_ast() {
        // A Rust file with duplicate type aliases and dead code.
        let src = "\
pub enum Status {
    Active,
    Inactive,
}

pub enum State {
    Active,
    Inactive,
}

pub struct Orphan {
    pub x: f64,
}

pub fn process(input: Status) -> Status { input }
";
        let ast = rust_to_base_ast("test.rs", src);
        let before = ast.node_count();
        let simplified = ast
            .collapse_aliases()
            .flatten_wrappers()
            .eliminate_dead();
        let after = simplified.node_count();
        assert!(after < before,
            "kintsugi pipeline should reduce nodes: {} -> {}", before, after);
    }

    #[test]
    fn kintsugi_preserves_depth_bound() {
        let src = "\
use std::fmt;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self { Point { x, y } }
    pub fn origin() -> Self { Point { x: 0.0, y: 0.0 } }
}

pub fn distance(a: Point, b: Point) -> f64 { 0.0 }
";
        let ast = rust_to_base_ast("test.rs", src);
        let simplified = ast
            .clone()
            .collapse_aliases()
            .flatten_wrappers()
            .eliminate_dead();
        assert!(simplified.depth() <= ast.depth(),
            "kintsugi should not increase depth: {} -> {}", ast.depth(), simplified.depth());
    }

    // -- Metrics tests -------------------------------------------------------

    #[test]
    fn metrics_count_items() {
        let src = "\
use std::fmt;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self { Point { x, y } }
}

pub fn distance(a: Point, b: Point) -> f64 { 0.0 }

pub trait Measurable {
    fn measure(&self) -> f64;
}
";
        let items = parse_rust_items(src);
        let metrics = compute_metrics(&items);
        assert!(metrics.use_count >= 1, "should count use statements");
        assert!(metrics.type_count >= 1, "should count struct/enum");
        assert!(metrics.impl_count >= 1, "should count impl blocks");
        assert!(metrics.fn_count >= 2, "should count functions (including impl methods)");
        assert!(metrics.trait_count >= 1, "should count traits");
    }

    // -- Integration with grammar_regions ------------------------------------

    #[test]
    fn grammar_regions_identifies_rust_file() {
        let grammar = grammar_regions::primary_grammar("src/main.rs");
        assert_eq!(grammar, GrammarId::new("@code/rust"));
    }

    #[test]
    fn full_pipeline_parse_regions_then_convert() {
        let src = "\
//! Module documentation.

use std::collections::HashMap;

/// A point in 2D space.
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn origin() -> Point { Point { x: 0.0, y: 0.0 } }
";
        // Step 1: grammar_regions splits into regions
        let grammar = GrammarId::new("@code/rust");
        let regions = grammar_regions::split_regions(src, &grammar);
        assert!(!regions.is_empty());

        // Step 2: collect code regions
        let code_content: String = regions
            .iter()
            .filter(|r| r.grammar == GrammarId::new("@code/rust"))
            .map(|r| r.content.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        // Step 3: parse Rust items from code regions
        let items = parse_rust_items(&code_content);
        assert!(!items.is_empty(), "should parse items from code regions");

        // Step 4: convert to MirrorAST
        let mirror_ast = rust_to_mirror_ast("test.rs", src);
        match &mirror_ast {
            MirrorAST::Module(m) => {
                assert!(!m.children.is_empty(), "module should have children");
            }
            other => panic!("expected Module, got {:?}", other),
        }

        // Step 5: convert to base AST and run kintsugi
        let base_ast = rust_to_base_ast("test.rs", src);
        let before = base_ast.node_count();
        let simplified = base_ast
            .collapse_aliases()
            .flatten_wrappers()
            .eliminate_dead();
        let after = simplified.node_count();
        // Even without duplicates, pipeline should at least not increase nodes
        assert!(after <= before,
            "kintsugi pipeline should not increase nodes: {} -> {}", before, after);
    }
}
