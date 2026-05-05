use crate::kernel::{ContentAddressed, Oid};
use fragmentation::encoding::Encode;

/// The kind of a token in the NL tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// A stemmed word leaf: "eigenvalu", "jacobi", "comput"
    Word,
    /// A compound node with children: "lambda_2", "approx_lambda_2"
    Compound,
}

/// Data carried by each node in the NL token tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    /// The stemmed/normalized text.
    pub text: String,
    /// Word or Compound.
    pub kind: TokenKind,
}

impl Token {
    pub fn word(text: impl Into<String>) -> Self {
        Token {
            text: text.into(),
            kind: TokenKind::Word,
        }
    }

    pub fn compound(text: impl Into<String>) -> Self {
        Token {
            text: text.into(),
            kind: TokenKind::Compound,
        }
    }

    /// Content address label: "token:{text}"
    fn label(&self) -> String {
        format!("token:{}", self.text)
    }
}

impl Encode for Token {
    fn encode(&self) -> Vec<u8> {
        self.label().into_bytes()
    }
}

impl ContentAddressed for Token {
    type Oid = Oid;
    fn content_oid(&self) -> Oid {
        Oid::hash(self.label().as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_word_same_oid() {
        let a = Token::word("eigenvalu");
        let b = Token::word("eigenvalu");
        assert_eq!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn different_words_different_oids() {
        let a = Token::word("eigenvalu");
        let b = Token::word("jacobi");
        assert_ne!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn word_and_compound_same_oid_same_text() {
        let w = Token::word("lambda_2");
        let c = Token::compound("lambda_2");
        assert_eq!(w.content_oid(), c.content_oid());
    }
}
