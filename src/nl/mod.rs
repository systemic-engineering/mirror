//! NL — natural language tokenizer for Mirror stdlib.
//!
//! Decomposes text into content-addressed token trees.
//! Each token is a `Prism<Token>` node. Compound tokens
//! (underscore-joined, CamelCase) become Fractal nodes whose
//! children are their decomposed parts.

pub mod compound;
pub mod stop_words;
pub mod token;

pub use compound::{CompoundNode, decompose};
pub use token::{Token, TokenKind};
