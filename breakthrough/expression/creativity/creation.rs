// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreationMedium {
    Text,
    Code,
    Visual,
    Audio,
    Mixed(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Creation {
    pub id: String,
    pub title: String,
    pub medium: CreationMedium,
    pub content: String,
    pub tags: Vec<String>,
    pub complexity: u8,
}

impl Creation {
    pub fn new<S: Into<String>>(id: S, title: S, medium: CreationMedium) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            medium,
            content: String::new(),
            tags: Vec::new(),
            complexity: 5,
        }
    }

    pub fn with_content<S: Into<String>>(mut self, content: S) -> Self {
        self.content = content.into();
        self
    }

    pub fn add_tag<S: Into<String>>(&mut self, tag: S) {
        self.tags.push(tag.into());
    }

    pub fn with_complexity(mut self, complexity: u8) -> Self {
        self.complexity = complexity.min(10);
        self
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t.eq_ignore_ascii_case(tag))
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("Creation title cannot be empty".to_string());
        }
        Ok(())
    }
}

impl fmt::Display for Creation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Creation {}: {}", self.id, self.title)
    }
}

impl fmt::Display for CreationMedium {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CreationMedium::Text => write!(f, "text"),
            CreationMedium::Code => write!(f, "code"),
            CreationMedium::Visual => write!(f, "visual"),
            CreationMedium::Audio => write!(f, "audio"),
            CreationMedium::Mixed(media) => write!(f, "mixed({})", media.join("+")),
        }
    }
}

