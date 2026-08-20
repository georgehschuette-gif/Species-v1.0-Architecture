// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// ConditionalConstruct: If-then cognitive structure.
#[derive(Debug, Clone, PartialEq)]
pub struct ConditionalConstruct {
    pub antecedent_strength: f64,
    pub consequent_strength: f64,
    pub conditional_probability: f64,
    pub causal_necessity: f64,
}

impl ConditionalConstruct {
    pub fn new(antecedent_strength: f64, consequent_strength: f64, conditional_probability: f64) -> Result<Self, HypotheticalError> {
        if !(0.0..=1.0).contains(&antecedent_strength) {
            return Err(HypotheticalError::InvalidStrength { strength: antecedent_strength });
        }
        if !(0.0..=1.0).contains(&consequent_strength) {
            return Err(HypotheticalError::InvalidStrength { strength: consequent_strength });
        }
        if !(0.0..=1.0).contains(&conditional_probability) {
            return Err(HypotheticalError::InvalidProbability { probability: conditional_probability });
        }
        Ok(Self { antecedent_strength, consequent_strength, conditional_probability, causal_necessity: conditional_probability })
    }

    pub fn material_implication(&self) -> f64 {
        if self.antecedent_strength < 1e-9 { 1.0 } else { self.conditional_probability }
    }

    pub fn is_valid_modus_ponens(&self) -> bool {
        self.antecedent_strength > 0.8 && self.conditional_probability > 0.8
    }
}
