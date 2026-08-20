// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Utterance {
    pub content: String,
    pub language: String,
    pub tokens: Vec<String>,
}

impl Utterance {
    pub fn new<S: Into<String>>(content: S, language: S) -> Result<Self, String> {
        let content = content.into();
        if content.trim().is_empty() {
            return Err("Utterance content cannot be empty".to_string());
        }
        let language = language.into();
        if language.trim().is_empty() {
            return Err("Language identifier cannot be empty".to_string());
        }
        let tokens: Vec<String> = content.split_whitespace().map(|s| s.to_string()).collect();
        Ok(Self {
            content,
            language,
            tokens,
        })
    }

    pub fn token_count(&self) -> usize {
        self.tokens.len()
    }

    pub fn is_question(&self) -> bool {
        self.content.trim().ends_with('?')
    }

    pub fn contains_word(&self, word: &str) -> bool {
        self.tokens.iter().any(|t| t.eq_ignore_ascii_case(word))
    }
}

impl fmt::Display for Utterance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.language, self.content)
    }
}

