//! LSP integration — generate `@code/<language>` grammars from tree-sitter and LSP servers.
//!
//! ## Pipeline
//!
//! 1. Read `node-types.json` from a tree-sitter grammar
//! 2. (Optional) Probe an LSP server for capabilities
//! 3. Generate a `.mirror` grammar file combining types + actions
//!
//! ## Usage
//!
//! ```text
//! mirror lsp learn @code/python src/**/*.py
//! ```

pub mod generate;
pub mod language;
pub mod node_types;
pub mod server;
