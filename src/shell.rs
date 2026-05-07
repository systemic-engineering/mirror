//! Shell evaluation — REPL and single-shot eval over Filesystem domain.

use std::io::{BufRead, Write};

use crate::domain::filesystem::{Filesystem, Folder};
use crate::resolve::Conversation;
use crate::Vector;

/// Evaluate a .conv source against a filesystem path.
///
/// Parses `source` into `Conversation<Filesystem>`, reads `input_path` into
/// `Tree<Folder>`, traces to `serde_json::Value`, writes pretty JSON to `output`.
pub fn eval(source: &str, input_path: &str, output: &mut dyn Write) -> Result<(), String> {
    let resolved: Conversation<Filesystem> =
        Conversation::from_source(source).map_err(|e| format!("conversation: {}", e))?;
    let tree = Folder::read_tree(input_path);
    let value: serde_json::Value = resolved.trace(tree).into_result().unwrap();
    let json = serde_json::to_string_pretty(&value).unwrap();
    let _ = output.write_all(json.as_bytes());
    let _ = output.write_all(b"\n");
    Ok(())
}

/// Run the conversation REPL.
///
/// Reads lines from `input`, evaluates each as a .conv expression against
/// `Tree<Folder>` at `path`, writes JSON to `output`, errors to `err`.
pub fn repl(path: &str, input: &mut dyn BufRead, output: &mut dyn Write, err: &mut dyn Write) {
    for line in input.lines() {
        let _ = write!(output, "conversation> ");
        let _ = output.flush();

        let line = match line {
            Ok(l) => l,
            Err(e) => {
                let _ = writeln!(err, "conversation: read error: {}", e);
                break;
            }
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let source = format!("out {}\n", trimmed);

        let resolved: Conversation<Filesystem> = match Conversation::from_source(&source) {
            Ok(conv) => conv,
            Err(e) => {
                let _ = writeln!(err, "  error: {}", e);
                continue;
            }
        };

        let tree = Folder::read_tree(path);
        let value: serde_json::Value = resolved.trace(tree).into_result().unwrap();
        let json = serde_json::to_string_pretty(&value).unwrap();
        let _ = writeln!(output, "{}", json);
    }

    let _ = writeln!(output);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Read};

    #[test]
    fn repl_evaluates_expression() {
        let mut input = Cursor::new(b"@json\n".to_vec());
        let mut output: Vec<u8> = Vec::new();
        let mut err: Vec<u8> = Vec::new();
        repl(".", &mut input, &mut output, &mut err);
        let out = String::from_utf8(output).unwrap();
        assert!(out.contains("\"@json\""));
    }

    #[test]
    fn repl_skips_empty_lines() {
        let mut input = Cursor::new(b"\n@json\n".to_vec());
        let mut output: Vec<u8> = Vec::new();
        let mut err: Vec<u8> = Vec::new();
        repl(".", &mut input, &mut output, &mut err);
        let out = String::from_utf8(output).unwrap();
        assert!(out.contains("\"@json\""));
    }

    #[test]
    fn repl_reports_errors_and_continues() {
        let mut input = Cursor::new(b"{ unclosed\n@json\n".to_vec());
        let mut output: Vec<u8> = Vec::new();
        let mut err: Vec<u8> = Vec::new();
        repl(".", &mut input, &mut output, &mut err);
        let err_str = String::from_utf8(err).unwrap();
        assert!(err_str.contains("error"));
        let out = String::from_utf8(output).unwrap();
        assert!(out.contains("\"@json\""));
    }

    #[test]
    fn repl_breaks_on_read_error() {
        struct FailReader;
        impl std::io::Read for FailReader {
            fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "test read error",
                ))
            }
        }
        impl BufRead for FailReader {
            fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "test read error",
                ))
            }
            fn consume(&mut self, _amt: usize) {}
        }

        let mut reader = FailReader;
        let mut output: Vec<u8> = Vec::new();
        let mut err: Vec<u8> = Vec::new();
        repl(".", &mut reader, &mut output, &mut err);
        let err_str = String::from_utf8(err).unwrap();
        assert!(err_str.contains("read error"));

        // Exercise Read + consume impls for coverage (Lines iterator uses fill_buf only).
        let mut buf = [0u8; 1];
        assert!(reader.read(&mut buf).is_err());
        reader.consume(0);
    }

    #[test]
    fn eval_produces_json() {
        let mut output: Vec<u8> = Vec::new();
        let result = eval("out @json\n", ".", &mut output);
        assert!(result.is_ok());
        let out = String::from_utf8(output).unwrap();
        let _: serde_json::Value = serde_json::from_str(&out).expect("valid JSON");
    }

    #[test]
    fn eval_returns_error_on_bad_source() {
        let mut output: Vec<u8> = Vec::new();
        let result = eval("{ unclosed", ".", &mut output);
        assert!(result.is_err());
    }
}
