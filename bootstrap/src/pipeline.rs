//! mq pipeline parser + executor. Mirrors C `split_pipeline` / `execute_pipeline`.

use crate::ast::AstNode;
use crate::exec::io_exec;
use crate::grammar::{grammar_path_for_ref, load_grammar};
use crate::spectral::{compute_content_oid, render_ast};
use crate::tokenize::tokenize;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct Segment {
    pub r#ref: String,
    /// kintsugi the result of this segment before passing to the next?
    pub kintsugi_after: bool,
}

pub fn split_pipeline(query: &str) -> Vec<Segment> {
    let mut segs: Vec<Segment> = Vec::new();
    let bytes = query.as_bytes();
    let len = bytes.len();
    let mut start = 0usize;
    let mut p = 0usize;
    while p < len {
        if p + 2 < len && bytes[p] == b'|' && bytes[p + 1] == b'\\' && bytes[p + 2] == b'>' {
            let seg = query[start..p].trim();
            segs.push(Segment {
                r#ref: seg.to_string(),
                kintsugi_after: true,
            });
            p += 3;
            start = p;
        } else if p + 1 < len && bytes[p] == b'|' && bytes[p + 1] == b'>' {
            let seg = query[start..p].trim();
            segs.push(Segment {
                r#ref: seg.to_string(),
                kintsugi_after: false,
            });
            p += 2;
            start = p;
        } else {
            p += 1;
        }
    }
    let tail = query[start..].trim();
    if !tail.is_empty() {
        segs.push(Segment {
            r#ref: tail.to_string(),
            kintsugi_after: false,
        });
    }
    segs
}

fn tokenize_with_ref(r#ref: &str, source: &[u8]) -> Option<AstNode> {
    let path = grammar_path_for_ref(r#ref)?;
    let g = load_grammar(&path).ok()?;
    Some(tokenize(source, &g))
}

/// Apply kintsugi to the current AST using its grammar tag.
fn apply_implicit_kintsugi(ast: &mut AstNode, current_text: &mut Vec<u8>) -> Result<(), ()> {
    let tag = ast.grammar_tag.clone();
    if tag.is_empty() {
        return Ok(());
    }
    let mut out = Vec::new();
    render_ast(ast, 0, &mut out);
    *current_text = out;
    match tokenize_with_ref(&tag, current_text) {
        Some(new_ast) => {
            *ast = new_ast;
            Ok(())
        }
        None => Err(()),
    }
}

pub fn execute_pipeline(segs: &[Segment], source: &[u8]) -> i32 {
    if segs.is_empty() {
        return 1;
    }
    let first_ref = &segs[0].r#ref;
    let mut ast = match tokenize_with_ref(first_ref, source) {
        Some(a) => a,
        None => return 1,
    };
    let mut current_text: Vec<u8> = source.to_vec();

    if segs[0].kintsugi_after {
        if apply_implicit_kintsugi(&mut ast, &mut current_text).is_err() {
            return 1;
        }
    }

    for i in 1..segs.len() {
        let r#ref = segs[i].r#ref.as_str();
        if r#ref == "@mirror/kintsugi" || r#ref == "@kintsugi" {
            let mut out = Vec::new();
            render_ast(&ast, 0, &mut out);
            current_text = out;
            let tag = if !ast.grammar_tag.is_empty() {
                ast.grammar_tag.clone()
            } else {
                first_ref.clone()
            };
            ast = match tokenize_with_ref(&tag, &current_text) {
                Some(a) => a,
                None => return 1,
            };
        } else if r#ref == "@mirror/butterfly"
            || r#ref == "@mirror/butterfly.butterfly"
            || r#ref == "@mirror/butterfly.emit"
        {
            let emit_only = r#ref == "@mirror/butterfly.emit";
            if !emit_only {
                let args = ["-x", "ir", "-", "-O2", "-ffp-contract=off", "-o", "mirror-butterfly", "-lm"];
                match io_exec("clang", &args, &current_text) {
                    Ok((rc, out)) => {
                        if !out.is_empty() {
                            let _ = std::io::stderr().write_all(&out);
                        }
                        if rc != 0 {
                            eprintln!("butterfly: clang failed with exit {}", rc);
                            return rc;
                        }
                        eprintln!("butterfly: wrote ./mirror-butterfly");
                    }
                    Err(e) => {
                        eprintln!("butterfly: exec error: {}", e);
                        return 1;
                    }
                }
            }
        } else {
            let new_ast = match tokenize_with_ref(r#ref, &current_text) {
                Some(a) => a,
                None => {
                    eprintln!("pipeline: cannot dispatch {}", r#ref);
                    return 1;
                }
            };
            ast = new_ast;
        }

        if segs[i].kintsugi_after {
            if apply_implicit_kintsugi(&mut ast, &mut current_text).is_err() {
                return 1;
            }
        }
    }

    let last = segs.last().unwrap().r#ref.as_str();
    let nseg = segs.len();
    if nseg >= 2
        && (last == "@mirror/kintsugi" || last == "@kintsugi" || last == "@mirror/butterfly.emit")
    {
        let _ = std::io::stdout().write_all(&current_text);
    } else if nseg >= 2 && (last == "@mirror/butterfly" || last == "@mirror/butterfly.butterfly") {
        // no stdout
    } else {
        let oid = compute_content_oid(&ast);
        println!("{}", oid);
    }
    0
}

pub fn is_mq_query(arg: &str) -> bool {
    if arg.is_empty() {
        return false;
    }
    if arg.starts_with('@') {
        return true;
    }
    if arg.contains("|>") {
        return true;
    }
    if arg.contains("|\\>") {
        return true;
    }
    false
}
