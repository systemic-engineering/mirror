//! Grammar-aware region detection and splitting.
//!
//! A file is a superposition of grammars. This module detects which grammar
//! owns which region of a file, splitting content into grammar-typed regions
//! with byte spans. The detection is line-based: scan lines, detect grammar
//! transitions, merge adjacent same-grammar lines.
//!
//! Supported primary grammars:
//! - `@code/rust` (.rs) -- detects `///`, `//!`, `//`, `/* */` as `@nl` regions
//! - `@code/markdown` (.md) -- detects fenced code blocks and YAML frontmatter
//! - `@mirror` (.mirror) -- detects `in @code/rust { }` and comments
//! - `@code/toml`, `@code/yaml`, `@code/json`, `@code/nix` -- leaf grammars (no recursion)
//! - `@nl` -- natural language (default for unknown extensions)

/// A grammar identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GrammarId(pub String);

impl GrammarId {
    pub fn new(s: &str) -> Self {
        GrammarId(s.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Byte span within the source file.
#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

/// A region of a file belonging to a single grammar.
#[derive(Clone, Debug)]
pub struct Region {
    pub grammar: GrammarId,
    pub content: String,
    pub span: Span,
    pub children: Vec<Region>,
}

/// Detect the primary grammar from a file path (extension-based).
pub fn primary_grammar(path: &str) -> GrammarId {
    let ext = path.rsplit('.').next().unwrap_or("");
    match ext {
        "rs" => GrammarId::new("@code/rust"),
        "md" => GrammarId::new("@code/markdown"),
        "mirror" => GrammarId::new("@mirror"),
        "toml" => GrammarId::new("@code/toml"),
        "nix" => GrammarId::new("@code/nix"),
        "yaml" | "yml" => GrammarId::new("@code/yaml"),
        "json" => GrammarId::new("@code/json"),
        "txt" => GrammarId::new("@nl"),
        "bash" | "sh" => GrammarId::new("@code/bash"),
        _ => GrammarId::new("@nl"),
    }
}

/// Split content into regions by grammar.
/// Detects embedded grammar transitions and recurses.
pub fn split_regions(content: &str, grammar: &GrammarId) -> Vec<Region> {
    match grammar.as_str() {
        "@code/rust" => split_rust(content),
        "@code/markdown" => split_markdown(content),
        "@mirror" => split_mirror(content),
        // Leaf grammars: no recursion, entire content is one region.
        _ => vec![Region {
            grammar: grammar.clone(),
            content: content.to_string(),
            span: Span {
                start: 0,
                end: content.len(),
            },
            children: vec![],
        }],
    }
}

// ---------------------------------------------------------------------------
// @code/rust splitter
// ---------------------------------------------------------------------------

/// Split Rust source into @code/rust and @nl regions.
///
/// Detection rules (line-based):
/// - Lines starting with `///` or `//!` -> @nl (doc comments, prefix stripped)
/// - Lines starting with `//` (but not `///` or `//!`) -> @nl (inline comments)
/// - Everything else -> @code/rust
///
/// Adjacent same-grammar lines merge into one region.
fn split_rust(content: &str) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    let mut current_grammar: Option<GrammarId> = None;
    let mut current_start: usize = 0;
    let mut current_content = String::new();
    let mut offset: usize = 0;

    for line in content.split_inclusive('\n') {
        let trimmed = line.trim_start();
        let line_grammar = if trimmed.starts_with("///") || trimmed.starts_with("//!") {
            GrammarId::new("@nl")
        } else if trimmed.starts_with("//") {
            GrammarId::new("@nl")
        } else {
            GrammarId::new("@code/rust")
        };

        match &current_grammar {
            Some(g) if *g == line_grammar => {
                // Same grammar, extend current region.
                current_content.push_str(line);
            }
            _ => {
                // Grammar transition. Flush previous region if any.
                if let Some(g) = current_grammar.take() {
                    if !current_content.is_empty() {
                        regions.push(Region {
                            grammar: g,
                            content: current_content.clone(),
                            span: Span {
                                start: current_start,
                                end: offset,
                            },
                            children: vec![],
                        });
                    }
                }
                current_grammar = Some(line_grammar);
                current_start = offset;
                current_content = line.to_string();
            }
        }

        offset += line.len();
    }

    // Flush the last region.
    if let Some(g) = current_grammar {
        if !current_content.is_empty() {
            regions.push(Region {
                grammar: g,
                content: current_content,
                span: Span {
                    start: current_start,
                    end: offset,
                },
                children: vec![],
            });
        }
    }

    regions
}

// ---------------------------------------------------------------------------
// @code/markdown splitter
// ---------------------------------------------------------------------------

/// Split markdown content into @nl and @code/* regions.
///
/// Detection rules (line-based):
/// - `---` at line 1 starts YAML frontmatter (@code/yaml) until next `---`
/// - ` ```lang ` starts a fenced code block (@code/<lang>) until ` ``` `
/// - ` ``` ` (no lang) starts @code/unknown until ` ``` `
/// - Everything else -> @nl
fn split_markdown(content: &str) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    let mut offset: usize = 0;
    let mut current_grammar = GrammarId::new("@nl");
    let mut current_start: usize = 0;
    let mut current_content = String::new();
    let mut in_frontmatter = false;
    let mut in_code_block = false;
    let mut line_number: usize = 0;

    for line in content.split_inclusive('\n') {
        let trimmed = line.trim();

        if line_number == 0 && trimmed == "---" {
            // YAML frontmatter start.
            // Flush any current region.
            if !current_content.is_empty() {
                regions.push(Region {
                    grammar: current_grammar.clone(),
                    content: current_content.clone(),
                    span: Span {
                        start: current_start,
                        end: offset,
                    },
                    children: vec![],
                });
            }
            in_frontmatter = true;
            current_grammar = GrammarId::new("@code/yaml");
            current_start = offset;
            current_content = line.to_string();
            offset += line.len();
            line_number += 1;
            continue;
        }

        if in_frontmatter {
            current_content.push_str(line);
            if trimmed == "---" {
                // End of frontmatter.
                offset += line.len();
                regions.push(Region {
                    grammar: current_grammar.clone(),
                    content: current_content.clone(),
                    span: Span {
                        start: current_start,
                        end: offset,
                    },
                    children: vec![],
                });
                in_frontmatter = false;
                current_grammar = GrammarId::new("@nl");
                current_start = offset;
                current_content = String::new();
                line_number += 1;
                continue;
            }
            offset += line.len();
            line_number += 1;
            continue;
        }

        if !in_code_block && trimmed.starts_with("```") {
            // Start of fenced code block.
            // Flush current region.
            if !current_content.is_empty() {
                regions.push(Region {
                    grammar: current_grammar.clone(),
                    content: current_content.clone(),
                    span: Span {
                        start: current_start,
                        end: offset,
                    },
                    children: vec![],
                });
            }

            // Determine the language.
            let lang_tag = trimmed.trim_start_matches('`').trim();
            let code_grammar = if lang_tag.is_empty() {
                GrammarId::new("@code/unknown")
            } else {
                match lang_tag.split_whitespace().next().unwrap_or("") {
                    "rust" => GrammarId::new("@code/rust"),
                    "nix" => GrammarId::new("@code/nix"),
                    "toml" => GrammarId::new("@code/toml"),
                    "yaml" | "yml" => GrammarId::new("@code/yaml"),
                    "json" => GrammarId::new("@code/json"),
                    "mirror" => GrammarId::new("@mirror"),
                    "bash" | "sh" | "shell" => GrammarId::new("@code/bash"),
                    other => GrammarId::new(&format!("@code/{}", other)),
                }
            };

            in_code_block = true;
            current_grammar = code_grammar;
            current_start = offset;
            current_content = line.to_string();
            offset += line.len();
            line_number += 1;
            continue;
        }

        if in_code_block && trimmed == "```" {
            // End of fenced code block.
            current_content.push_str(line);
            offset += line.len();
            regions.push(Region {
                grammar: current_grammar.clone(),
                content: current_content.clone(),
                span: Span {
                    start: current_start,
                    end: offset,
                },
                children: vec![],
            });
            in_code_block = false;
            current_grammar = GrammarId::new("@nl");
            current_start = offset;
            current_content = String::new();
            line_number += 1;
            continue;
        }

        // Normal line: accumulate into current region.
        current_content.push_str(line);
        offset += line.len();
        line_number += 1;
    }

    // Flush the last region.
    if !current_content.is_empty() {
        regions.push(Region {
            grammar: current_grammar,
            content: current_content,
            span: Span {
                start: current_start,
                end: offset,
            },
            children: vec![],
        });
    }

    regions
}

// ---------------------------------------------------------------------------
// @mirror splitter
// ---------------------------------------------------------------------------

/// Split mirror grammar content into @mirror, @nl, and @code/* regions.
///
/// Detection rules (line-based):
/// - Lines starting with `#` or `//` -> @nl (comments)
/// - `in @code/rust {` ... `}` -> @code/rust regions
/// - Everything else -> @mirror
fn split_mirror(content: &str) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    let mut current_grammar = GrammarId::new("@mirror");
    let mut current_start: usize = 0;
    let mut current_content = String::new();
    let mut offset: usize = 0;
    let mut in_embedded_block = false;
    let mut embedded_grammar = GrammarId::new("@mirror");
    let mut brace_depth: usize = 0;

    for line in content.split_inclusive('\n') {
        let trimmed = line.trim_start();

        if in_embedded_block {
            current_content.push_str(line);
            // Count braces to track nesting.
            for ch in line.chars() {
                match ch {
                    '{' => brace_depth += 1,
                    '}' => {
                        if brace_depth > 0 {
                            brace_depth -= 1;
                        }
                    }
                    _ => {}
                }
            }
            offset += line.len();

            if brace_depth == 0 {
                // End of embedded block.
                regions.push(Region {
                    grammar: embedded_grammar.clone(),
                    content: current_content.clone(),
                    span: Span {
                        start: current_start,
                        end: offset,
                    },
                    children: vec![],
                });
                in_embedded_block = false;
                current_grammar = GrammarId::new("@mirror");
                current_start = offset;
                current_content = String::new();
            }
            continue;
        }

        // Check for `in @code/rust {` pattern.
        if trimmed.contains("in @code/") && trimmed.contains('{') {
            // Flush current region.
            if !current_content.is_empty() {
                regions.push(Region {
                    grammar: current_grammar.clone(),
                    content: current_content.clone(),
                    span: Span {
                        start: current_start,
                        end: offset,
                    },
                    children: vec![],
                });
            }

            // Extract the grammar from `in @code/rust`.
            let code_grammar = if let Some(at_pos) = trimmed.find("@code/") {
                let rest = &trimmed[at_pos..];
                let end = rest.find(|c: char| c.is_whitespace() || c == '{').unwrap_or(rest.len());
                GrammarId::new(&rest[..end])
            } else {
                GrammarId::new("@code/unknown")
            };

            embedded_grammar = code_grammar;
            in_embedded_block = true;
            brace_depth = 0;
            // Count braces in this line.
            for ch in line.chars() {
                match ch {
                    '{' => brace_depth += 1,
                    '}' => {
                        if brace_depth > 0 {
                            brace_depth -= 1;
                        }
                    }
                    _ => {}
                }
            }
            current_start = offset;
            current_content = line.to_string();
            offset += line.len();

            // If the block closes on the same line:
            if brace_depth == 0 {
                regions.push(Region {
                    grammar: embedded_grammar.clone(),
                    content: current_content.clone(),
                    span: Span {
                        start: current_start,
                        end: offset,
                    },
                    children: vec![],
                });
                in_embedded_block = false;
                current_grammar = GrammarId::new("@mirror");
                current_start = offset;
                current_content = String::new();
            }
            continue;
        }

        // Detect comment lines.
        let line_grammar = if trimmed.starts_with('#') || trimmed.starts_with("//") {
            GrammarId::new("@nl")
        } else {
            GrammarId::new("@mirror")
        };

        if line_grammar == current_grammar {
            current_content.push_str(line);
        } else {
            // Grammar transition.
            if !current_content.is_empty() {
                regions.push(Region {
                    grammar: current_grammar.clone(),
                    content: current_content.clone(),
                    span: Span {
                        start: current_start,
                        end: offset,
                    },
                    children: vec![],
                });
            }
            current_grammar = line_grammar;
            current_start = offset;
            current_content = line.to_string();
        }

        offset += line.len();
    }

    // Flush the last region.
    if !current_content.is_empty() {
        regions.push(Region {
            grammar: current_grammar,
            content: current_content,
            span: Span {
                start: current_start,
                end: offset,
            },
            children: vec![],
        });
    }

    regions
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -- Primary grammar detection ------------------------------------------

    #[test]
    fn primary_grammar_rs() {
        assert_eq!(primary_grammar("src/dirac.rs"), GrammarId::new("@code/rust"));
    }

    #[test]
    fn primary_grammar_md() {
        assert_eq!(
            primary_grammar("docs/spec.md"),
            GrammarId::new("@code/markdown")
        );
    }

    #[test]
    fn primary_grammar_mirror() {
        assert_eq!(
            primary_grammar("garden/ai.mirror"),
            GrammarId::new("@mirror")
        );
    }

    #[test]
    fn primary_grammar_toml() {
        assert_eq!(
            primary_grammar("Cargo.toml"),
            GrammarId::new("@code/toml")
        );
    }

    #[test]
    fn primary_grammar_unknown_defaults_to_nl() {
        assert_eq!(primary_grammar("README"), GrammarId::new("@nl"));
    }

    // -- Rust splitter ------------------------------------------------------

    #[test]
    fn split_rust_doc_comments_become_nl() {
        let content = "\
//! Module doc line 1.
//! Module doc line 2.

use std::collections::BinaryHeap;
use std::cmp::Ordering;

// ---------------------------------------------------------------------------
// SparseMatrix
// ---------------------------------------------------------------------------

/// Sparse matrix in CSR format.
#[derive(Clone, Debug)]
pub struct SparseMatrix {
    pub nrows: usize,
}
";
        let grammar = GrammarId::new("@code/rust");
        let regions = split_regions(content, &grammar);

        // First region should be @nl (module doc comments).
        assert_eq!(regions[0].grammar, GrammarId::new("@nl"));
        assert!(regions[0].content.contains("Module doc line 1"));

        // Then code (use + blank line before).
        // The blank line between doc comment and use is @code/rust.
        let code_regions: Vec<_> = regions.iter().filter(|r| r.grammar == GrammarId::new("@code/rust")).collect();
        assert!(!code_regions.is_empty(), "should have @code/rust regions");

        // The section comment lines should be @nl.
        let nl_regions: Vec<_> = regions.iter().filter(|r| r.grammar == GrammarId::new("@nl")).collect();
        assert!(nl_regions.len() >= 2, "should have at least 2 @nl regions (module doc + section comment)");

        // The `/// Sparse matrix` doc comment should be @nl.
        let has_doc_comment = nl_regions.iter().any(|r| r.content.contains("Sparse matrix"));
        assert!(has_doc_comment, "doc comment should be an @nl region");
    }

    #[test]
    fn split_rust_regions_cover_entire_file() {
        let content = "\
//! Module doc.

use std::cmp::Ordering;

// A comment.

pub fn main() {}
";
        let grammar = GrammarId::new("@code/rust");
        let regions = split_regions(content, &grammar);

        // Verify no gaps: each region's end == next region's start.
        for i in 0..regions.len() - 1 {
            assert_eq!(
                regions[i].span.end, regions[i + 1].span.start,
                "gap between region {} and {}: end={} start={}",
                i, i + 1, regions[i].span.end, regions[i + 1].span.start
            );
        }

        // Verify coverage: first starts at 0, last ends at content length.
        assert_eq!(regions[0].span.start, 0);
        assert_eq!(regions.last().unwrap().span.end, content.len());
    }

    #[test]
    fn split_rust_regions_in_file_order() {
        let content = "\
//! Doc.
use std::io;
// Comment.
fn main() {}
";
        let grammar = GrammarId::new("@code/rust");
        let regions = split_regions(content, &grammar);

        // Regions should be in ascending order of span.start.
        for i in 0..regions.len() - 1 {
            assert!(
                regions[i].span.start < regions[i + 1].span.start,
                "region {} starts at {} but region {} starts at {}",
                i, regions[i].span.start, i + 1, regions[i + 1].span.start
            );
        }
    }

    #[test]
    fn split_rust_adjacent_same_grammar_lines_merge() {
        let content = "\
/// Doc line 1.
/// Doc line 2.
/// Doc line 3.
pub struct Foo {}
";
        let grammar = GrammarId::new("@code/rust");
        let regions = split_regions(content, &grammar);

        // The three doc lines should be merged into one @nl region.
        assert_eq!(regions[0].grammar, GrammarId::new("@nl"));
        assert!(regions[0].content.contains("Doc line 1"));
        assert!(regions[0].content.contains("Doc line 3"));

        // Total should be exactly 2 regions: @nl + @code/rust.
        assert_eq!(regions.len(), 2);
    }

    // -- Markdown splitter --------------------------------------------------

    #[test]
    fn split_markdown_code_blocks_become_code_regions() {
        let content = "\
# Heading

Some prose here.

```rust
fn main() {}
```

More prose.
";
        let grammar = GrammarId::new("@code/markdown");
        let regions = split_regions(content, &grammar);

        // Should have @nl, @code/rust, @nl regions.
        let grammars: Vec<&str> = regions.iter().map(|r| r.grammar.as_str()).collect();
        assert!(grammars.contains(&"@code/rust"), "should have @code/rust region");
        assert!(grammars.contains(&"@nl"), "should have @nl regions");

        // The code region should contain the function.
        let code_region = regions.iter().find(|r| r.grammar.as_str() == "@code/rust").unwrap();
        assert!(code_region.content.contains("fn main()"));
    }

    #[test]
    fn split_markdown_yaml_frontmatter() {
        let content = "\
---
title: Test
date: 2026-05-06
---

# Heading

Body text.
";
        let grammar = GrammarId::new("@code/markdown");
        let regions = split_regions(content, &grammar);

        // First region should be @code/yaml (frontmatter).
        assert_eq!(regions[0].grammar, GrammarId::new("@code/yaml"));
        assert!(regions[0].content.contains("title: Test"));
    }

    #[test]
    fn split_markdown_regions_cover_entire_file() {
        let content = "\
# Spec

Some text.

```rust
fn foo() {}
```

End.
";
        let grammar = GrammarId::new("@code/markdown");
        let regions = split_regions(content, &grammar);

        // No gaps.
        for i in 0..regions.len() - 1 {
            assert_eq!(
                regions[i].span.end, regions[i + 1].span.start,
                "gap between markdown region {} and {}",
                i, i + 1
            );
        }

        assert_eq!(regions[0].span.start, 0);
        assert_eq!(regions.last().unwrap().span.end, content.len());
    }

    #[test]
    fn split_markdown_untyped_code_block() {
        let content = "\
Some text.

```
raw code here
```

More text.
";
        let grammar = GrammarId::new("@code/markdown");
        let regions = split_regions(content, &grammar);

        let code_region = regions.iter().find(|r| r.grammar.as_str() == "@code/unknown").unwrap();
        assert!(code_region.content.contains("raw code here"));
    }

    // -- Mirror splitter ----------------------------------------------------

    #[test]
    fn split_mirror_comments_become_nl() {
        let content = "\
# This is a comment.
// Another comment.
grammar @test {
  type = foo | bar
}
";
        let grammar = GrammarId::new("@mirror");
        let regions = split_regions(content, &grammar);

        // First region(s) should be @nl.
        assert_eq!(regions[0].grammar, GrammarId::new("@nl"));
        assert!(regions[0].content.contains("This is a comment"));

        // Should have @mirror region for the grammar block.
        let mirror_regions: Vec<_> = regions.iter().filter(|r| r.grammar.as_str() == "@mirror").collect();
        assert!(!mirror_regions.is_empty(), "should have @mirror regions");
    }

    #[test]
    fn split_mirror_embedded_code_block() {
        let content = "\
grammar @test {
  type = foo
}

in @code/rust {
  fn example() -> u32 {
    42
  }
}
";
        let grammar = GrammarId::new("@mirror");
        let regions = split_regions(content, &grammar);

        // Should detect the embedded Rust code block.
        let rust_regions: Vec<_> = regions.iter().filter(|r| r.grammar.as_str() == "@code/rust").collect();
        assert_eq!(rust_regions.len(), 1, "should have exactly 1 @code/rust region");
        assert!(rust_regions[0].content.contains("fn example()"));
    }

    #[test]
    fn split_mirror_regions_cover_entire_file() {
        let content = "\
# Comment.
in @prism
grammar @test { type = x }
";
        let grammar = GrammarId::new("@mirror");
        let regions = split_regions(content, &grammar);

        for i in 0..regions.len() - 1 {
            assert_eq!(
                regions[i].span.end, regions[i + 1].span.start,
                "gap between mirror region {} and {}",
                i, i + 1
            );
        }

        assert_eq!(regions[0].span.start, 0);
        assert_eq!(regions.last().unwrap().span.end, content.len());
    }

    // -- Leaf grammars ------------------------------------------------------

    #[test]
    fn leaf_grammar_is_single_region() {
        let content = "[package]\nname = \"spectral\"\nversion = \"0.1.0\"\n";
        let grammar = GrammarId::new("@code/toml");
        let regions = split_regions(content, &grammar);

        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].grammar, GrammarId::new("@code/toml"));
        assert_eq!(regions[0].content, content);
        assert_eq!(regions[0].span.start, 0);
        assert_eq!(regions[0].span.end, content.len());
    }

    // -- Real file: dirac.rs content ----------------------------------------

    #[test]
    fn split_real_rust_file_dirac_prefix() {
        // First ~38 lines of dirac.rs.
        let content = "\
//! Dirac operator for spectral triples on finite graphs.
//!
//! The Dirac operator D unifies eigenvalues, distance, and action into a single matrix:
//!
//! - D is the block matrix [[0, B^T], [B, 0]] where B is the signed weighted incidence matrix
//! - D^2 restricted to 0-forms = graph Laplacian L_0
//! - D^2 restricted to 1-forms = edge Laplacian L_1
//! - D is self-adjoint (D = D^T for real matrices)
//! - Eigenvalues of D are symmetric about 0
//! - Connes distance = Dijkstra with edge lengths 1/sqrt(w)
//!
//! ## Architecture
//!
//! This module provides the bare math. No external dependencies beyond std.
//! The Jacobi eigenvalue solver is self-contained (same algorithm as eigentest.rs).

use std::collections::BinaryHeap;
use std::cmp::Ordering;

// ---------------------------------------------------------------------------
// SparseMatrix -- CSR format
// ---------------------------------------------------------------------------

/// Sparse matrix in Compressed Sparse Row (CSR) format.
#[derive(Clone, Debug)]
pub struct SparseMatrix {
    /// Number of rows.
    pub nrows: usize,
    /// Number of columns.
    pub ncols: usize,
    /// Row pointers.
    pub row_ptr: Vec<usize>,
    /// Column indices.
    pub col_idx: Vec<usize>,
    /// Values.
    pub values: Vec<f64>,
}
";
        let grammar = GrammarId::new("@code/rust");
        let regions = split_regions(content, &grammar);

        // Region 0: @nl -- module doc comment (//! lines).
        assert_eq!(regions[0].grammar, GrammarId::new("@nl"), "first region should be @nl");
        assert!(regions[0].content.starts_with("//! Dirac operator"));

        // After the module doc, there should be @code/rust (use statements).
        let code_regions: Vec<_> = regions
            .iter()
            .filter(|r| r.grammar == GrammarId::new("@code/rust"))
            .collect();
        assert!(!code_regions.is_empty());

        // The section comment (// ---) should produce @nl regions.
        let nl_regions: Vec<_> = regions
            .iter()
            .filter(|r| r.grammar == GrammarId::new("@nl"))
            .collect();
        // Should have: module doc, section comment, struct doc comments
        assert!(nl_regions.len() >= 3, "should have at least 3 @nl regions, got {}", nl_regions.len());

        // The `/// Sparse matrix...` doc comment should be @nl.
        let has_sparse_doc = nl_regions
            .iter()
            .any(|r| r.content.contains("Sparse matrix"));
        assert!(has_sparse_doc, "struct doc comment should be @nl");

        // No gaps, no overlaps.
        for i in 0..regions.len() - 1 {
            assert_eq!(
                regions[i].span.end,
                regions[i + 1].span.start,
                "gap/overlap between regions {} and {}",
                i, i + 1
            );
        }
        assert_eq!(regions[0].span.start, 0);
        assert_eq!(regions.last().unwrap().span.end, content.len());
    }
}
