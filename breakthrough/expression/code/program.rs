// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CodeSource {
    Generated,
    Transcribed,
    Modified,
    External,
}

impl fmt::Display for CodeSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CodeSource::Generated => write!(f, "generated"),
            CodeSource::Transcribed => write!(f, "transcribed"),
            CodeSource::Modified => write!(f, "modified"),
            CodeSource::External => write!(f, "external"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Program {
    pub source_code: String,
    pub language: String,
    pub entry_point: Option<String>,
    pub source: CodeSource,
    pub metadata: Vec<(String, String)>,
}

impl Program {
    pub fn new<S: Into<String>>(source_code: S, language: S) -> Self {
        Self {
            source_code: source_code.into(),
            language: language.into(),
            entry_point: None,
            source: CodeSource::Generated,
            metadata: Vec::new(),
        }
    }

    pub fn with_source(mut self, source: CodeSource) -> Self {
        self.source = source;
        self
    }

    pub fn with_entry_point<S: Into<String>>(mut self, entry: S) -> Self {
        self.entry_point = Some(entry.into());
        self
    }

    pub fn with_metadata<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.metadata.push((key.into(), value.into()));
        self
    }

    pub fn line_count(&self) -> usize {
        self.source_code.lines().count()
    }

    pub fn is_empty(&self) -> bool {
        self.source_code.trim().is_empty()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.source_code.trim().is_empty() {
            return Err("Program source code is empty".to_string());
        }
        if self.language.trim().is_empty() {
            return Err("Program language is empty".to_string());
        }
        Ok(())
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} program ({} lines)", self.language, self.line_count())
    }
}

