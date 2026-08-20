// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Novelty {
    pub creation_id: String,
    pub novelty_score: f64,
    pub dimensions: Vec<NoveltyDimension>,
    pub references: Vec<String>,
    pub is_original: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NoveltyDimension {
    pub name: String,
    pub score: f64,
    pub comparison_set: String,
}

impl Novelty {
    pub fn new<S: Into<String>>(creation_id: S) -> Self {
        Self {
            creation_id: creation_id.into(),
            novelty_score: 0.0,
            dimensions: Vec::new(),
            references: Vec::new(),
            is_original: false,
        }
    }

    pub fn with_score(mut self, score: f64) -> Self {
        self.novelty_score = score.clamp(0.0, 1.0);
        self
    }

    pub fn add_dimension(&mut self, dimension: NoveltyDimension) {
        self.dimensions.push(dimension);
    }

    pub fn add_reference<S: Into<String>>(&mut self, reference: S) {
        self.references.push(reference.into());
    }

    pub fn mark_original(mut self) -> Self {
        self.is_original = true;
        self
    }

    pub fn dimension_count(&self) -> usize {
        self.dimensions.len()
    }
}

impl NoveltyDimension {
    pub fn new<S: Into<String>>(name: S, score: f64, comparison_set: S) -> Self {
        Self {
            name: name.into(),
            score: score.clamp(0.0, 1.0),
            comparison_set: comparison_set.into(),
        }
    }
}

impl fmt::Display for Novelty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Novelty {} (score: {:.2})", self.creation_id, self.novelty_score)
    }
}

impl fmt::Display for NoveltyDimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:.2} (vs {})", self.name, self.score, self.comparison_set)
    }
}

