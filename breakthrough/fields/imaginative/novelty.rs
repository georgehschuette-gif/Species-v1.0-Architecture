// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// ImaginativeNovelty: Degree of newness in cognitive content with computed metrics.
#[derive(Debug, Clone, PartialEq)]
pub struct ImaginativeNovelty {
    pub content_id: u64,
    pub novelty_score: f64,
    pub familiarity_distance: f64,
    pub recombination_factor: f64,
}

impl ImaginativeNovelty {
    pub fn new(content_id: u64) -> Result<Self, ImaginativeError> {
        if content_id == 0 {
            return Err(ImaginativeError::InvalidContent { content_id });
        }
        Ok(Self { content_id, novelty_score: 0.0, familiarity_distance: 0.0, recombination_factor: 0.0 })
    }

    pub fn set_novelty(&mut self, novelty_score: f64, familiarity_distance: f64, recombination_factor: f64) -> Result<(), ImaginativeError> {
        if !(0.0..=1.0).contains(&novelty_score) {
            return Err(ImaginativeError::InvalidScore { score: novelty_score });
        }
        if !(0.0..=1.0).contains(&familiarity_distance) {
            return Err(ImaginativeError::InvalidDistance { distance: familiarity_distance });
        }
        if !(0.0..=1.0).contains(&recombination_factor) {
            return Err(ImaginativeError::InvalidScore { score: recombination_factor });
        }
        self.novelty_score = novelty_score;
        self.familiarity_distance = familiarity_distance;
        self.recombination_factor = recombination_factor;
        Ok(())
    }

    pub fn is_novel(&self) -> bool {
        self.novelty_score > 0.7 && self.familiarity_distance > 0.5
    }

    pub fn is_derivative(&self) -> bool {
        self.novelty_score < 0.3 && self.familiarity_distance < 0.3
    }

    pub fn novelty_index(&self) -> f64 {
        (self.novelty_score * 0.5 + self.familiarity_distance * 0.3 + self.recombination_factor * 0.2).clamp(0.0, 1.0)
    }

    pub fn content_id(&self) -> u64 {
        self.content_id
    }
}
