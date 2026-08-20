// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grammar {
    pub name: String,
    pub rules: Vec<GrammarRule>,
    pub start_symbol: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GrammarRule {
    pub left: String,
    pub right: Vec<String>,
}

impl Grammar {
    pub fn new<S: Into<String>>(name: S, start_symbol: S) -> Self {
        Self {
            name: name.into(),
            rules: Vec::new(),
            start_symbol: start_symbol.into(),
        }
    }

    pub fn add_rule<S: Into<String>>(&mut self, left: S, right: Vec<S>) {
        self.rules.push(GrammarRule {
            left: left.into(),
            right: right.into_iter().map(Into::into).collect(),
        });
    }

    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

impl GrammarRule {
    pub fn new<S: Into<String>>(left: S, right: Vec<S>) -> Self {
        Self {
            left: left.into(),
            right: right.into_iter().map(Into::into).collect(),
        }
    }
}

impl fmt::Display for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Grammar {} (start: {})", self.name, self.start_symbol)
    }
}

impl fmt::Display for GrammarRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.left, self.right.join(" "))
    }
}

