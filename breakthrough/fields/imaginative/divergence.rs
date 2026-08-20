// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// CreativeDivergence: Breadth of imaginative exploration with computed metrics.
pub struct CreativeDivergence {
    pub seed_concept: u64,
    pub generated_variants: usize,
    pub semantic_spread: f64,
    pub utility_score: f64,
}

impl CreativeDivergence {
    pub fn new(seed_concept: u64) -> Result<Self, ImaginativeError> {
        if seed_concept == 0 {
            return Err(ImaginativeError::InvalidContent { content_id: seed_concept });
        }
        Ok(Self { seed_concept, generated_variants: 0, semantic_spread: 0.0, utility_score: 0.0 })
    }

    pub fn add_variant(&mut self, semantic_distance: f64, usefulness: f64) -> Result<(), ImaginativeError> {
        if !(0.0..=1.0).contains(&semantic_distance) {
            return Err(ImaginativeError::InvalidDistance { distance: semantic_distance });
        }
        if !(0.0..=1.0).contains(&usefulness) {
            return Err(ImaginativeError::InvalidScore { score: usefulness });
        }
        self.generated_variants += 1;
        self.semantic_spread = (self.semantic_spread * (self.generated_variants - 1) as f64 + semantic_distance) / self.generated_variants as f64;
        self.utility_score = (self.utility_score * (self.generated_variants - 1) as f64 + usefulness) / self.generated_variants as f64;
        Ok(())
    }

    pub fn is_divergent(&self) -> bool {
        self.semantic_spread > 0.6 && self.generated_variants >= 3
    }

    pub fn is_utilitarian(&self) -> bool {
        self.utility_score > 0.7
    }

    pub fn diversity_score(&self) -> f64 {
        (self.semantic_spread * 0.6 + (self.generated_variants as f64 / 10.0).min(1.0) * 0.4).clamp(0.0, 1.0)
    }
}
