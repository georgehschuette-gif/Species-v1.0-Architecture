// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::ImaginationError;

pub mod condition;
pub mod outcome;

pub use condition::CounterfactualCondition;
pub use outcome::CounterfactualOutcome;

pub const DEFAULT_COUNTERFACTUAL_PROBABILITY: f64 = 0.5;
pub const MAX_PLAUSIBILITY: f64 = 1.0;

pub fn create_counterfactual_condition(antecedent: f64) -> Result<CounterfactualCondition, ImaginationError> {
    CounterfactualCondition::new(antecedent)
}

pub fn evaluate_outcome(plausibility: f64) -> CounterfactualOutcome {
    CounterfactualOutcome::from_plausibility(plausibility)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn counterfactual_condition_creation() {
        let c = create_counterfactual_condition(0.8).unwrap();
        assert!(c.antecedent() > 0.0);
    }
}
