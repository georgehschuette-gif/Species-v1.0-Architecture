// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Explanation {
    pub id: String,
    pub subject: String,
    pub content: String,
    pub format: ExplanationFormat,
    pub evidence_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExplanationFormat {
    Narrative,
    Structured,
    BulletPoints,
    Mathematical,
    Procedural,
}

impl Explanation {
    pub fn new<S: Into<String>>(id: S, subject: S, content: S) -> Self {
        Self {
            id: id.into(),
            subject: subject.into(),
            content: content.into(),
            format: ExplanationFormat::Narrative,
            evidence_ids: Vec::new(),
        }
    }

    pub fn with_format(mut self, format: ExplanationFormat) -> Self {
        self.format = format;
        self
    }

    pub fn add_evidence<S: Into<String>>(&mut self, evidence_id: S) {
        self.evidence_ids.push(evidence_id.into());
    }

    pub fn word_count(&self) -> usize {
        self.content.split_whitespace().count()
    }

    pub fn has_evidence(&self) -> bool {
        !self.evidence_ids.is_empty()
    }
}

impl fmt::Display for Explanation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Explanation {}: {}", self.id, self.subject)
    }
}

impl fmt::Display for ExplanationFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExplanationFormat::Narrative => write!(f, "narrative"),
            ExplanationFormat::Structured => write!(f, "structured"),
            ExplanationFormat::BulletPoints => write!(f, "bullet points"),
            ExplanationFormat::Mathematical => write!(f, "mathematical"),
            ExplanationFormat::Procedural => write!(f, "procedural"),
        }
    }
}

