// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// SuppositionalStrength: Plausibility and evidential support for a supposition.
pub struct SuppositionalStrength {
    pub supposition_id: u64,
    pub strength: f64,
    pub supporting_evidence: f64,
    pub contradicting_evidence: f64,
}

impl SuppositionalStrength {
    pub fn new(supposition_id: u64, strength: f64, supporting_evidence: f64, contradicting_evidence: f64) -> Result<Self, HypotheticalError> {
        if supposition_id == 0 {
            return Err(HypotheticalError::InvalidAlternative { alternative_id: supposition_id });
        }
        if !(0.0..=1.0).contains(&strength) {
            return Err(HypotheticalError::InvalidStrength { strength });
        }
        if !(0.0..=1.0).contains(&supporting_evidence) {
            return Err(HypotheticalError::InvalidScore { score: supporting_evidence });
        }
        if !(0.0..=1.0).contains(&contradicting_evidence) {
            return Err(HypotheticalError::InvalidScore { score: contradicting_evidence });
        }
        Ok(Self { supposition_id, strength, supporting_evidence, contradicting_evidence })
    }

    pub fn net_support(&self) -> f64 {
        self.supporting_evidence - self.contradicting_evidence
    }

    pub fn evidential_ratio(&self) -> f64 {
        if self.contradicting_evidence < f64::EPSILON {
            f64::INFINITY
        } else {
            self.supporting_evidence / self.contradicting_evidence
        }
    }

    pub fn is_well_supported(&self) -> bool {
        self.supporting_evidence > 0.6 && self.contradicting_evidence < 0.3
    }

    pub fn is_controversial(&self) -> bool {
        self.supporting_evidence > 0.4 && self.contradicting_evidence > 0.4
    }

    pub fn strength_label(&self) -> &'static str {
        if self.strength > 0.8 {
            "strong"
        } else if self.strength > 0.4 {
            "moderate"
        } else {
            "weak"
        }
    }
}
