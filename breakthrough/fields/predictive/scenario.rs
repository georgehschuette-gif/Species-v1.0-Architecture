// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// Scenario: A possible future state with an associated probability and outcome.
#[derive(Debug, Clone, PartialEq)]
pub struct Scenario {
    pub name: String,
    pub probability: f64,
    pub outcome_value: f64,
    pub description: String,
}

impl Scenario {
    pub fn new(name: String, probability: f64, outcome_value: f64, description: String) -> Result<Self, PredictiveError> {
        if !(0.0..=1.0).contains(&probability) {
            return Err(PredictiveError::InvalidProbability {
                probability,
                reason: "probability must be between 0 and 1".to_string(),
            });
        }
        if !probability.is_finite() {
            return Err(PredictiveError::InvalidProbability {
                probability,
                reason: "probability must be finite".to_string(),
            });
        }
        Ok(Self { name, probability, outcome_value, description })
    }

    pub fn is_high_probability(&self) -> bool {
        self.probability > 0.7
    }

    pub fn is_low_probability(&self) -> bool {
        self.probability < 0.1
    }

    pub fn risk_exposure(&self) -> f64 {
        self.probability * self.outcome_value.abs()
    }
}

/// ScenarioLikelihood: Probability distribution over possible futures.
pub struct ScenarioLikelihood {
    pub scenarios: Vec<Scenario>,
    pub entropy: f64,
    pub surprisal: f64,
}

impl ScenarioLikelihood {
    pub fn new(scenarios: Vec<Scenario>) -> Result<Self, PredictiveError> {
        let total_prob: f64 = scenarios.iter().map(|s| s.probability).sum();
        if (total_prob - 1.0).abs() > 1e-6 {
            return Err(PredictiveError::InvalidProbability {
                probability: total_prob,
                reason: "probabilities must sum to 1.0".to_string(),
            });
        }
        let entropy = scenarios.iter().filter(|s| s.probability > 0.0).map(|s| -s.probability * s.probability.ln()).sum();
        let surprisal = scenarios.iter().map(|s| s.probability * s.outcome_value).sum();
        Ok(Self { scenarios, entropy, surprisal })
    }

    pub fn most_likely(&self) -> Option<&Scenario> {
        self.scenarios.iter().max_by(|a, b| a.probability.partial_cmp(&b.probability).unwrap_or(std::cmp::Ordering::Equal))
    }

    pub fn expected_value(&self) -> f64 {
        self.scenarios.iter().map(|s| s.probability * s.outcome_value).sum()
    }

    pub fn scenario_count(&self) -> usize {
        self.scenarios.len()
    }

    pub fn total_entropy(&self) -> f64 {
        self.entropy
    }
}
