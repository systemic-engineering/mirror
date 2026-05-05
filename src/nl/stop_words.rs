//! Static English stop word list for NL tokenization.
//!
//! Function words (articles, prepositions, conjunctions, auxiliaries,
//! pronouns) that would create edges between everything if tokenized.

/// Returns true if the word is a stop word.
pub fn is_stop_word(word: &str) -> bool {
    STOP_WORDS.contains(&word)
}

const STOP_WORDS: &[&str] = &[
    // Articles
    "a",
    "an",
    "the",
    // Prepositions
    "at",
    "by",
    "for",
    "from",
    "in",
    "into",
    "of",
    "on",
    "to",
    "with",
    "about",
    "above",
    "after",
    "against",
    "along",
    "among",
    "around",
    "before",
    "behind",
    "below",
    "beneath",
    "beside",
    "between",
    "beyond",
    "down",
    "during",
    "except",
    "inside",
    "near",
    "off",
    "onto",
    "out",
    "outside",
    "over",
    "past",
    "since",
    "through",
    "throughout",
    "toward",
    "under",
    "until",
    "up",
    "upon",
    "within",
    "without",
    // Conjunctions
    "and",
    "but",
    "or",
    "nor",
    "yet",
    "so",
    "both",
    "either",
    "neither",
    "not",
    "only",
    "than",
    "whether",
    "while",
    // Auxiliary verbs
    "am",
    "are",
    "be",
    "been",
    "being",
    "can",
    "could",
    "did",
    "do",
    "does",
    "had",
    "has",
    "have",
    "is",
    "may",
    "might",
    "must",
    "shall",
    "should",
    "was",
    "were",
    "will",
    "would",
    // Pronouns
    "he",
    "her",
    "him",
    "his",
    "it",
    "its",
    "me",
    "my",
    "our",
    "she",
    "their",
    "them",
    "they",
    "us",
    "we",
    "you",
    "your",
    // Demonstratives / determiners
    "that",
    "these",
    "this",
    "those",
    // Other function words
    "all",
    "also",
    "any",
    "as",
    "each",
    "else",
    "even",
    "every",
    "here",
    "how",
    "if",
    "just",
    "more",
    "most",
    "much",
    "no",
    "now",
    "once",
    "other",
    "own",
    "same",
    "some",
    "still",
    "such",
    "then",
    "there",
    "too",
    "very",
    "what",
    "when",
    "where",
    "which",
    "who",
    "whom",
    "whose",
    "why",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_stop_words_filtered() {
        assert!(is_stop_word("the"));
        assert!(is_stop_word("is"));
        assert!(is_stop_word("and"));
        assert!(is_stop_word("of"));
    }

    #[test]
    fn content_words_pass() {
        assert!(!is_stop_word("eigenvalue"));
        assert!(!is_stop_word("spectral"));
        assert!(!is_stop_word("fiedler"));
        assert!(!is_stop_word("graph"));
    }

    #[test]
    fn short_words_not_all_stopped() {
        // "go", "ok" are not stop words
        assert!(!is_stop_word("go"));
        assert!(!is_stop_word("ok"));
    }
}
