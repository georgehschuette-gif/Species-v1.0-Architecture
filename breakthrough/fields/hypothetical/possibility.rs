// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// HypotheticalPossibility: Degree to which an alternative is viable.
#[derive(Debug, Clone, PartialEq)]
pub struct HypotheticalPossibility {
    pub alternative_id: u64,
    pub possibility_score: f64,
    pub consistency_with_facts: f64,
    pub coherence_with_beliefs: f64,
}

impl HypotheticalPossibility {
    pub fn new(alternative_id: u64, possibility_score: f64) -> Result<Self, HypotheticalError> {
        if alternative_id == 0 {
            return Err(HypotheticalError::InvalidAlternative { alternative_id });
        }
        if !(0.0..=1.0).contains(&possibility_score) {
            return Err(HypotheticalError::InvalidScore { score: possibility_score });
        }
        Ok(Self { alternative_id, possibility_score, consistency_with_facts: 0.5, coherence_with_beliefs: 0.5 })
    }

    pub fn overall_viability(&self) -> f64 {
        (self.possibility_score * 0.4 + self.consistency_with_facts * 0.3 + self.coherence_with_beliefs * 0.3).clamp(0.0, 1.0)
    }

    pub fn is_viable(&self) -> bool {
        self.overall_viability() > 0.6
    }
}
