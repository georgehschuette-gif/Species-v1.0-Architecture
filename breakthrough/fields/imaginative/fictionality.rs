// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// FictionalityScore: Degree to which content is fictional, with computed metrics.
#[derive(Debug, Clone, PartialEq)]
pub struct FictionalityScore {
    pub content_id: u64,
    pub fictionality: f64,
    pub world_distance: f64,
    pub suspension_of_disbelief: f64,
}

impl FictionalityScore {
    pub fn new(content_id: u64) -> Result<Self, ImaginativeError> {
        if content_id == 0 {
            return Err(ImaginativeError::InvalidContent { content_id });
        }
        Ok(Self { content_id, fictionality: 0.0, world_distance: 0.0, suspension_of_disbelief: 1.0 })
    }

    pub fn set_fictionality(&mut self, value: f64) -> Result<(), ImaginativeError> {
        if !(0.0..=1.0).contains(&value) {
            return Err(ImaginativeError::InvalidScore { score: value });
        }
        self.fictionality = value;
        Ok(())
    }

    pub fn is_purely_fictional(&self) -> bool {
        self.fictionality > 0.8 && self.world_distance > 0.7
    }

    pub fn is_realistic(&self) -> bool {
        self.fictionality < 0.2 && self.world_distance < 0.3
    }

    pub fn immersion_score(&self) -> f64 {
        (self.suspension_of_disbelief * (1.0 - self.fictionality * 0.5)).clamp(0.0, 1.0)
    }

    pub fn genre_clarity(&self) -> &'static str {
        if self.fictionality > 0.8 {
            "speculative"
        } else if self.fictionality > 0.4 {
            "blended"
        } else {
            "factual"
        }
    }
}
