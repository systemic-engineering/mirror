//! emit_code — generic code emitter driven by @code grammar templates.
//!
//! NO STRINGS for typed things. IoList for output. One function for any grammar.

// ---------------------------------------------------------------------------
// IoList — tree of byte slices
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum IoList {
    Chunk(Vec<u8>),
    Nested(Vec<IoList>),
    Empty,
}

impl IoList {
    pub fn text(s: &str) -> Self {
        IoList::Chunk(s.as_bytes().to_vec())
    }

    pub fn join(parts: Vec<IoList>) -> Self {
        IoList::Nested(parts)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.flatten_into(&mut out);
        out
    }

    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.to_bytes()).into_owned()
    }

    fn flatten_into(&self, out: &mut Vec<u8>) {
        match self {
            IoList::Chunk(bytes) => out.extend_from_slice(bytes),
            IoList::Nested(children) => {
                for child in children {
                    child.flatten_into(out);
                }
            }
            IoList::Empty => {}
        }
    }
}

// ---------------------------------------------------------------------------
// TemplateSet — closures that produce IoList
// ---------------------------------------------------------------------------

/// A set of code generation templates for a target language.
/// Each closure maps mirror constructs to IoList output.
pub struct TemplateSet {
    /// Map a mirror type name to a target type reference.
    pub map_type: Box<dyn Fn(&str) -> IoList>,
    /// Convert a name to PascalCase (or target equivalent).
    pub type_name: Box<dyn Fn(&str) -> String>,
    /// Convert a name to snake_case (or target equivalent).
    pub field_name: Box<dyn Fn(&str) -> String>,
    /// Emit an enum declaration.
    pub emit_enum: Box<dyn Fn(&str, &[String], &[String]) -> IoList>,
    /// Emit a struct declaration with typed fields.
    pub emit_struct: Box<dyn Fn(&str, &[(String, String)], &[String]) -> IoList>,
    /// Emit a unit type declaration (no fields, no variants).
    pub emit_unit_type: Box<dyn Fn(&str) -> IoList>,
    /// Emit a function declaration.
    pub emit_function: Box<dyn Fn(&str, &[(String, String)], Option<&str>) -> IoList>,
    /// Emit a property function declaration.
    pub emit_property: Box<dyn Fn(&str, &[(String, String)]) -> IoList>,
    /// Emit a module wrapper.
    pub emit_module: Box<dyn Fn(&str, Vec<IoList>) -> IoList>,
    /// Emit the file header.
    pub emit_header: Box<dyn Fn(&str) -> IoList>,
    /// Emit a comment for unrecognized declarations.
    pub emit_comment: Box<dyn Fn(&str, &str) -> IoList>,
    /// Emit a generic/parameterized struct.
    pub emit_generic_struct: Box<dyn Fn(&str, &[String]) -> IoList>,
    /// The todo expression for the target language.
    pub todo_expr: &'static str,
}

/// A code grammar: name + templates.
pub struct CodeGrammar {
    pub name: &'static str,
    pub templates: TemplateSet,
}

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

fn to_pascal_case(s: &str) -> String {
    s.split(['_', '-', '/'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let mut s = first.to_uppercase().to_string();
                    s.extend(chars);
                    s
                }
            }
        })
        .collect()
}

fn to_snake_case(s: &str) -> String {
    let s = s.replace(['-', '/'], "_");
    if s.chars()
        .all(|c| c.is_lowercase() || c == '_' || c.is_numeric())
    {
        return s;
    }

    let mut result = String::new();
    let mut prev_was_upper = false;
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && !prev_was_upper {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
            prev_was_upper = true;
        } else {
            prev_was_upper = false;
            result.push(c);
        }
    }
    result
}

fn strip_grammar_prefix(name: &str) -> String {
    let stripped = name.trim_start_matches('@');
    stripped.replace('/', "_")
}

fn indent_str(indent: usize) -> String {
    "    ".repeat(indent)
}

// ---------------------------------------------------------------------------
// Rust type mapping
// ---------------------------------------------------------------------------

fn map_type_rust(mirror_type: &str) -> IoList {
    IoList::text(&map_type_rust_string(mirror_type))
}

fn map_type_rust_string(mirror_type: &str) -> String {
    let t = mirror_type.trim();

    if let Some(idx) = t.find('(') {
        let base = &t[..idx];
        let inner = &t[idx + 1..t.len() - 1];
        let params: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
        let mapped_params: Vec<String> = params.iter().map(|p| map_type_rust_string(p)).collect();

        return match base {
            "option" => format!("Option<{}>", mapped_params.join(", ")),
            "result" => format!("Result<{}>", mapped_params.join(", ")),
            "vec" => format!("Vec<{}>", mapped_params.join(", ")),
            "hashmap" => format!("HashMap<{}>", mapped_params.join(", ")),
            "imperfect" => format!("Imperfect<{}>", mapped_params.join(", ")),
            _ => format!("{}<{}>", to_pascal_case(base), mapped_params.join(", ")),
        };
    }

    if t.starts_with('[') && t.ends_with(']') {
        let inner = &t[1..t.len() - 1];
        return format!("Vec<{}>", map_type_rust_string(inner));
    }

    match t {
        "text" | "string" => "String".to_string(),
        "nat" => "u64".to_string(),
        "f64" => "f64".to_string(),
        "f32" => "f32".to_string(),
        "u8" => "u8".to_string(),
        "u16" => "u16".to_string(),
        "u32" => "u32".to_string(),
        "u64" => "u64".to_string(),
        "usize" => "usize".to_string(),
        "i8" => "i8".to_string(),
        "i16" => "i16".to_string(),
        "i32" => "i32".to_string(),
        "i64" => "i64".to_string(),
        "bool" => "bool".to_string(),
        "str" => "&str".to_string(),
        "ref" => "String".to_string(),
        "oid" => "Oid".to_string(),
        "loss" => "Loss".to_string(),
        "prism" => "Prism".to_string(),
        "verdict" => "Verdict".to_string(),
        "imperfect" => "Imperfect<Crystal, Error, Loss>".to_string(),
        _ => to_pascal_case(t),
    }
}

// ---------------------------------------------------------------------------
// CodeGrammar::rust()
// ---------------------------------------------------------------------------

impl CodeGrammar {
    pub fn rust() -> Self {
        CodeGrammar {
            name: "rust",
            templates: TemplateSet {
                map_type: Box::new(map_type_rust),
                type_name: Box::new(to_pascal_case),
                field_name: Box::new(to_snake_case),
                emit_enum: Box::new(|name, _params, variants| {
                    let pascal = to_pascal_case(name);
                    let mut parts = vec![IoList::text(&format!("pub enum {} {{\n", pascal))];
                    for v in variants {
                        parts.push(IoList::text(&format!("    {},\n", to_pascal_case(v))));
                    }
                    parts.push(IoList::text("}\n"));
                    IoList::join(parts)
                }),
                emit_struct: Box::new(|name, fields, _params| {
                    let pascal = to_pascal_case(name);
                    let mut parts = vec![IoList::text(&format!("pub struct {} {{\n", pascal))];
                    for (fname, ftype) in fields {
                        parts.push(IoList::text(&format!(
                            "    pub {}: {},\n",
                            to_snake_case(fname),
                            map_type_rust_string(ftype)
                        )));
                    }
                    parts.push(IoList::text("}\n"));
                    IoList::join(parts)
                }),
                emit_unit_type: Box::new(|name| {
                    IoList::text(&format!("pub struct {};\n", to_pascal_case(name)))
                }),
                emit_function: Box::new(|name, params, return_type| {
                    let fn_name = to_snake_case(name);
                    let params_str = if params.is_empty() {
                        String::new()
                    } else {
                        params
                            .iter()
                            .map(|(n, t)| {
                                format!("{}: {}", to_snake_case(n), map_type_rust_string(t))
                            })
                            .collect::<Vec<_>>()
                            .join(", ")
                    };
                    let ret = match return_type {
                        Some(rt) => format!(" -> {}", map_type_rust_string(rt)),
                        None => String::new(),
                    };
                    IoList::join(vec![
                        IoList::text(&format!("pub fn {}({}){} {{\n", fn_name, params_str, ret)),
                        IoList::text("    todo!()\n"),
                        IoList::text("}\n"),
                    ])
                }),
                emit_property: Box::new(|name, params| {
                    let fn_name = to_snake_case(name);
                    let params_str = if params.is_empty() {
                        String::new()
                    } else {
                        params
                            .iter()
                            .map(|(n, t)| {
                                format!("{}: &{}", to_snake_case(n), to_pascal_case(t))
                            })
                            .collect::<Vec<_>>()
                            .join(", ")
                    };
                    IoList::join(vec![
                        IoList::text(&format!(
                            "pub fn {}({}) -> Imperfect<(), PropertyError, PropertyLoss> {{\n",
                            fn_name, params_str
                        )),
                        IoList::text("    todo!()\n"),
                        IoList::text("}\n"),
                    ])
                }),
                emit_module: Box::new(|name, children| {
                    let mod_name = to_snake_case(&strip_grammar_prefix(name));
                    let mut parts = vec![IoList::text(&format!("pub mod {} {{\n", mod_name))];
                    for child in children {
                        parts.push(child);
                    }
                    parts.push(IoList::text("}\n"));
                    IoList::join(parts)
                }),
                emit_header: Box::new(|target| {
                    IoList::join(vec![
                        IoList::text(&format!(
                            "// Generated by mirror craft --target {}\n",
                            target
                        )),
                        IoList::text(
                            "// Do not edit \u{2014} this file is derived from .mirror source\n\n",
                        ),
                    ])
                }),
                emit_comment: Box::new(|kind, name| {
                    IoList::text(&format!("// {}: {}\n", kind, name))
                }),
                emit_generic_struct: Box::new(|name, params| {
                    let pascal = to_pascal_case(name);
                    let generics: Vec<String> = params.iter().map(|p| to_pascal_case(p)).collect();
                    IoList::text(&format!("pub struct {}<{}>;\n", pascal, generics.join(", ")))
                }),
                todo_expr: "todo!()",
            },
        }
    }
}

// ---------------------------------------------------------------------------
// Gleam type mapping
// ---------------------------------------------------------------------------

fn map_type_gleam(mirror_type: &str) -> IoList {
    IoList::text(&map_type_gleam_string(mirror_type))
}

fn map_type_gleam_string(mirror_type: &str) -> String {
    let t = mirror_type.trim();

    if let Some(idx) = t.find('(') {
        let base = &t[..idx];
        let inner = &t[idx + 1..t.len() - 1];
        let params: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
        let mapped_params: Vec<String> = params.iter().map(|p| map_type_gleam_string(p)).collect();

        return match base {
            "option" => format!("Option({}))", mapped_params.join(", ")),
            "result" => format!("Result({}))", mapped_params.join(", ")),
            "list" => format!("List({})", mapped_params.join(", ")),
            "vec" => format!("List({})", mapped_params.join(", ")),
            _ => format!("{}({})", to_pascal_case(base), mapped_params.join(", ")),
        };
    }

    if t.starts_with('[') && t.ends_with(']') {
        let inner = &t[1..t.len() - 1];
        return format!("List({})", map_type_gleam_string(inner));
    }

    match t {
        "text" | "string" | "str" | "ref" => "String".to_string(),
        "nat" | "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "usize" => {
            "Int".to_string()
        }
        "f32" | "f64" => "Float".to_string(),
        "bool" => "Bool".to_string(),
        "nil" => "Nil".to_string(),
        "dynamic" => "Dynamic".to_string(),
        "bit_array" => "BitArray".to_string(),
        _ => to_pascal_case(t),
    }
}

// ---------------------------------------------------------------------------
// CodeGrammar::gleam()
// ---------------------------------------------------------------------------

impl CodeGrammar {
    pub fn gleam() -> Self {
        CodeGrammar {
            name: "gleam",
            templates: TemplateSet {
                map_type: Box::new(map_type_gleam),
                type_name: Box::new(to_pascal_case),
                field_name: Box::new(to_snake_case),
                emit_enum: Box::new(|name, _params, variants| {
                    let pascal = to_pascal_case(name);
                    let mut parts = vec![IoList::text(&format!("pub type {} {{\n", pascal))];
                    for v in variants {
                        parts.push(IoList::text(&format!("  {}\n", to_pascal_case(v))));
                    }
                    parts.push(IoList::text("}\n"));
                    IoList::join(parts)
                }),
                emit_struct: Box::new(|name, fields, _params| {
                    let pascal = to_pascal_case(name);
                    let mut parts = vec![IoList::text(&format!("pub type {} {{\n", pascal))];
                    parts.push(IoList::text(&format!("  {}(\n", pascal)));
                    for (fname, ftype) in fields {
                        parts.push(IoList::text(&format!(
                            "    {}: {},\n",
                            to_snake_case(fname),
                            map_type_gleam_string(ftype)
                        )));
                    }
                    parts.push(IoList::text("  )\n"));
                    parts.push(IoList::text("}\n"));
                    IoList::join(parts)
                }),
                emit_unit_type: Box::new(|name| {
                    let pascal = to_pascal_case(name);
                    IoList::text(&format!("pub type {} {{\n  {}\n}}\n", pascal, pascal))
                }),
                emit_function: Box::new(|name, params, return_type| {
                    let fn_name = to_snake_case(name);
                    let params_str = if params.is_empty() {
                        String::new()
                    } else {
                        params
                            .iter()
                            .map(|(n, t)| {
                                format!("{}: {}", to_snake_case(n), map_type_gleam_string(t))
                            })
                            .collect::<Vec<_>>()
                            .join(", ")
                    };
                    let ret = match return_type {
                        Some(rt) => format!(" -> {}", map_type_gleam_string(rt)),
                        None => String::new(),
                    };
                    IoList::join(vec![
                        IoList::text(&format!(
                            "pub fn {}({}){} {{\n",
                            fn_name, params_str, ret
                        )),
                        IoList::text("  todo\n"),
                        IoList::text("}\n"),
                    ])
                }),
                emit_property: Box::new(|name, params| {
                    let fn_name = to_snake_case(name);
                    let params_str = if params.is_empty() {
                        String::new()
                    } else {
                        params
                            .iter()
                            .map(|(n, t)| {
                                format!("{}: {}", to_snake_case(n), to_pascal_case(t))
                            })
                            .collect::<Vec<_>>()
                            .join(", ")
                    };
                    IoList::join(vec![
                        IoList::text(&format!(
                            "pub fn {}({}) -> Result(Nil, PropertyError) {{\n",
                            fn_name, params_str
                        )),
                        IoList::text("  todo\n"),
                        IoList::text("}\n"),
                    ])
                }),
                emit_module: Box::new(|name, children| {
                    // Gleam doesn't have inline modules, emit as a section comment
                    let mod_name = to_snake_case(&strip_grammar_prefix(name));
                    let mut parts = vec![IoList::text(&format!(
                        "// --- module: {} ---\n\n",
                        mod_name
                    ))];
                    for child in children {
                        parts.push(child);
                    }
                    IoList::join(parts)
                }),
                emit_header: Box::new(|target| {
                    IoList::join(vec![
                        IoList::text(&format!(
                            "// Generated by mirror craft --target {}\n",
                            target
                        )),
                        IoList::text(
                            "// Do not edit \u{2014} this file is derived from .mirror source\n\n",
                        ),
                    ])
                }),
                emit_comment: Box::new(|kind, name| {
                    IoList::text(&format!("// {}: {}\n", kind, name))
                }),
                emit_generic_struct: Box::new(|name, params| {
                    let pascal = to_pascal_case(name);
                    let generics: Vec<String> =
                        params.iter().map(|p| to_snake_case(p)).collect();
                    IoList::text(&format!(
                        "pub type {}({}) {{\n  {}({})\n}}\n",
                        pascal,
                        generics.join(", "),
                        pascal,
                        generics.join(", ")
                    ))
                }),
                todo_expr: "todo",
            },
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // IoList tests
    #[test]
    fn iolist_empty() {
        let empty: Vec<u8> = vec![];
        assert_eq!(IoList::Empty.to_bytes(), empty);
    }

    #[test]
    fn iolist_chunk() {
        assert_eq!(IoList::text("hello").to_bytes(), b"hello".to_vec());
    }

    #[test]
    fn iolist_nested() {
        let list = IoList::join(vec![
            IoList::text("pub "),
            IoList::text("struct "),
            IoList::text("Foo;\n"),
        ]);
        assert_eq!(list.to_bytes(), b"pub struct Foo;\n".to_vec());
    }

    #[test]
    fn iolist_deep() {
        let inner = IoList::join(vec![IoList::text("a"), IoList::text("b")]);
        let outer = IoList::join(vec![inner, IoList::text("c")]);
        assert_eq!(outer.to_bytes(), b"abc".to_vec());
    }

    #[test]
    fn iolist_to_string() {
        let list = IoList::join(vec![IoList::text("hello "), IoList::text("world")]);
        assert_eq!(list.to_string_lossy(), "hello world");
    }

    #[test]
    fn iolist_empty_nested() {
        let list = IoList::join(vec![IoList::Empty, IoList::text("x"), IoList::Empty]);
        assert_eq!(list.to_bytes(), b"x".to_vec());
    }

    // CodeGrammar::rust() tests
    #[test]
    fn rust_map_type_text() {
        let g = CodeGrammar::rust();
        assert_eq!((g.templates.map_type)("text").to_string_lossy(), "String");
    }

    #[test]
    fn rust_map_type_option() {
        let g = CodeGrammar::rust();
        assert_eq!(
            (g.templates.map_type)("option(text)").to_string_lossy(),
            "Option<String>"
        );
    }

    #[test]
    fn rust_emit_enum() {
        let g = CodeGrammar::rust();
        let r = (g.templates.emit_enum)("color", &[], &["red".into(), "blue".into()]);
        let s = r.to_string_lossy();
        assert!(s.contains("pub enum Color"), "got:\n{}", s);
        assert!(s.contains("Red,"), "got:\n{}", s);
        assert!(s.contains("Blue,"), "got:\n{}", s);
    }

    #[test]
    fn rust_emit_unit() {
        let g = CodeGrammar::rust();
        let r = (g.templates.emit_unit_type)("point");
        assert_eq!(r.to_string_lossy(), "pub struct Point;\n");
    }

    // CodeGrammar::gleam() tests
    #[test]
    fn gleam_map_type_nat() {
        let g = CodeGrammar::gleam();
        assert_eq!((g.templates.map_type)("nat").to_string_lossy(), "Int");
    }

    #[test]
    fn gleam_all_ints_are_int() {
        let g = CodeGrammar::gleam();
        for t in ["u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "usize"] {
            assert_eq!((g.templates.map_type)(t).to_string_lossy(), "Int");
        }
    }

    #[test]
    fn gleam_emit_enum() {
        let g = CodeGrammar::gleam();
        let r = (g.templates.emit_enum)("color", &[], &["red".into(), "blue".into()]);
        let s = r.to_string_lossy();
        assert!(s.contains("pub type Color"), "got:\n{}", s);
        assert!(s.contains("Red"), "got:\n{}", s);
        assert!(!s.contains("enum"), "got:\n{}", s);
    }

    #[test]
    fn gleam_emit_struct() {
        let g = CodeGrammar::gleam();
        let r = (g.templates.emit_struct)("user", &[("name".into(), "text".into())], &[]);
        let s = r.to_string_lossy();
        assert!(s.contains("pub type User"), "got:\n{}", s);
        assert!(s.contains("User("), "got:\n{}", s);
        assert!(s.contains("name: String"), "got:\n{}", s);
    }

    #[test]
    fn gleam_emit_function() {
        let g = CodeGrammar::gleam();
        let r = (g.templates.emit_function)("boot", &[("id".into(), "text".into())], None);
        let s = r.to_string_lossy();
        assert!(s.contains("pub fn boot"), "got:\n{}", s);
        assert!(s.contains("todo"), "got:\n{}", s);
        assert!(!s.contains("todo!()"), "got:\n{}", s);
    }
}
