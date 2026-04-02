//! Analysis core for the LSP server.
//!
//! Bridges the conversation parser/resolver to LSP diagnostic and hover types.

use lsp_types::{Diagnostic, DiagnosticSeverity, Position};

use crate::ast::Span;
use crate::lsp::position::LineIndex;
use crate::model::Domain;
use crate::parse::Parse;
use crate::resolve::Namespace;
use crate::Vector;

/// Result of analyzing a `.conv` source file.
#[derive(Clone, Debug)]
pub struct AnalysisResult {
    pub diagnostics: Vec<Diagnostic>,
    pub domains: Vec<Domain>,
}

/// Parse source, extract grammars, collect errors as LSP diagnostics.
pub fn analyze(source: &str, namespace: &Namespace) -> AnalysisResult {
    let line_index = LineIndex::new(source);
    let mut diagnostics = Vec::new();
    let mut domains = Vec::new();

    match Parse.trace(source.to_string()).into_result() {
        Ok(ast) => {
            // Extract grammars from the AST.
            for child in ast.children() {
                if child.data().is_decl("grammar") {
                    match Domain::from_grammar(child) {
                        Ok(domain) => domains.push(domain),
                        Err(msg) => {
                            let span = child.data().span;
                            diagnostics.push(span_diagnostic(
                                &line_index,
                                span,
                                DiagnosticSeverity::ERROR,
                                msg,
                            ));
                        }
                    }
                }
            }

            // Validate domain references against namespace + local domains.
            for child in ast.children() {
                if child.data().is_ref("domain") {
                    let name = &child.data().value;
                    let known_locally = domains.iter().any(|d| d.domain_name() == name);
                    if !known_locally && !namespace.has_grammar(name) {
                        let span = child.data().span;
                        diagnostics.push(span_diagnostic(
                            &line_index,
                            span,
                            DiagnosticSeverity::WARNING,
                            format!("unknown domain: @{}", name),
                        ));
                    }
                }
            }
        }
        Err(e) => {
            let range = if let Some(span) = e.span {
                line_index.range(span)
            } else {
                lsp_types::Range::new(Position::new(0, 0), Position::new(0, 0))
            };
            diagnostics.push(Diagnostic {
                range,
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("conversation".into()),
                message: e.message,
                ..Default::default()
            });
        }
    }

    AnalysisResult {
        diagnostics,
        domains,
    }
}

/// Find the word at a cursor position and return hover markdown.
///
/// Looks up domains and types in the namespace and locally-defined domains.
pub fn hover_at(
    source: &str,
    position: Position,
    namespace: &Namespace,
    local_domains: &[Domain],
) -> Option<String> {
    let line_index = LineIndex::new(source);
    let offset = line_index.offset(position) as usize;
    let word = word_at_offset(source, offset)?;

    // Strip leading '@' for domain lookups.
    let bare = word.strip_prefix('@').unwrap_or(&word);

    // Check namespace domains.
    if let Some(domain) = namespace.domain(bare) {
        return Some(domain_hover(domain));
    }

    // Check locally-defined domains.
    for domain in local_domains {
        if domain.domain_name() == bare {
            return Some(domain_hover(domain));
        }
    }

    // Check types across all known domains.
    for domain in local_domains
        .iter()
        .chain(namespace.grammar_domains().iter().filter_map(|n| namespace.domain(n)))
    {
        if domain.has_type(&word) {
            let variants = domain
                .variants(&word)
                .unwrap_or_default()
                .join(" | ");
            return Some(format!(
                "**type** `{}` in `@{}`\n\n{}",
                word,
                domain.domain_name(),
                variants
            ));
        }
    }

    None
}

/// Extract the word (identifier or @-prefixed domain ref) at a byte offset.
pub fn word_at_offset(source: &str, offset: usize) -> Option<String> {
    if offset > source.len() {
        return None;
    }
    let bytes = source.as_bytes();

    // Find the start of the word.
    let mut start = offset;
    while start > 0 && is_word_byte(bytes[start - 1]) {
        start -= 1;
    }
    // Include a leading '@' if present.
    if start > 0 && bytes[start - 1] == b'@' {
        start -= 1;
    }

    // Find the end of the word.
    let mut end = offset;
    while end < bytes.len() && is_word_byte(bytes[end]) {
        end += 1;
    }

    if start == end {
        return None;
    }

    Some(source[start..end].to_string())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn span_diagnostic(
    line_index: &LineIndex,
    span: Span,
    severity: DiagnosticSeverity,
    message: String,
) -> Diagnostic {
    Diagnostic {
        range: line_index.range(span),
        severity: Some(severity),
        source: Some("conversation".into()),
        message,
        ..Default::default()
    }
}

fn domain_hover(domain: &Domain) -> String {
    let mut parts = vec![format!("**domain** `@{}`", domain.domain_name())];

    let types = domain.type_names();
    if !types.is_empty() {
        parts.push(format!("\n**types:** {}", types.join(", ")));
    }

    let actions = domain.act_names();
    if !actions.is_empty() {
        parts.push(format!("\n**actions:** {}", actions.join(", ")));
    }

    parts.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analyze_valid_grammar() {
        let source = "grammar @test {\n  type status = active | inactive\n}\n";
        let ns = Namespace::new();
        let result = analyze(source, &ns);
        assert!(result.diagnostics.is_empty(), "expected no diagnostics, got: {:?}", result.diagnostics);
        assert_eq!(result.domains.len(), 1);
        assert_eq!(result.domains[0].domain_name(), "test");
    }

    #[test]
    fn analyze_parse_error() {
        let source = "grammar @broken {\n";
        let ns = Namespace::new();
        let result = analyze(source, &ns);
        assert!(!result.diagnostics.is_empty(), "expected diagnostics for parse error");
        assert_eq!(result.diagnostics[0].severity, Some(DiagnosticSeverity::ERROR));
    }

    #[test]
    fn word_at_offset_domain() {
        let source = "in @filesystem";
        // Cursor on 'f' of filesystem (offset 4)
        let word = word_at_offset(source, 4).unwrap();
        assert_eq!(word, "@filesystem");
    }

    #[test]
    fn word_at_offset_type() {
        let source = "type status = active | inactive";
        // Cursor on 's' of status (offset 5)
        let word = word_at_offset(source, 5).unwrap();
        assert_eq!(word, "status");
    }

    #[test]
    fn hover_domain() {
        let source = "grammar @mydom {\n  type kind = a | b\n}\n";
        let ns = Namespace::new();

        // Parse and build a domain from the source.
        let ast = Parse.trace(source.to_string()).into_result().unwrap();
        let mut domains = Vec::new();
        for child in ast.children() {
            if child.data().is_decl("grammar") {
                domains.push(Domain::from_grammar(child).unwrap());
            }
        }

        // Hover over @mydom — position at 'in @mydom' would be line 0, col 10 (the 'm')
        let result = hover_at(source, Position::new(0, 10), &ns, &domains);
        assert!(result.is_some(), "expected hover result");
        let text = result.unwrap();
        assert!(text.contains("@mydom"), "hover should mention domain name");
        assert!(text.contains("kind"), "hover should list types");
    }
}
