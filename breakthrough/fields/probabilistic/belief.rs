// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// BayesianBelief: Probability distribution over hypotheses with update mechanics.
#[derive(Debug, Clone, PartialEq)]
pub struct BayesianBelief {
    pub hypothesis_id: u64,
    pub prior: f64,
    pub likelihood: f64,
    pub posterior: f64,
}

impl BayesianBelief {
    pub fn new(hypothesis_id: u64, prior: f64) -> Result<Self, ProbabilisticError> {
        if hypothesis_id == 0 {
            return Err(ProbabilisticError::InvalidProbability { probability: f64::from(hypothesis_id as u32) });
        }
        if !(0.0..=1.0).contains(&prior) {
            return Err(ProbabilisticError::InvalidProbability { probability: prior });
        }
        Ok(Self { hypothesis_id, prior, likelihood: 1.0, posterior: prior })
    }

    pub fn update(&mut self, evidence: bool) {
        let likelihood = if evidence { 0.9 } else { 0.1 };
        self.likelihood = likelihood;
        let numerator = likelihood * self.prior;
        let denominator = numerator + (1.0 - likelihood) * (1.0 - self.prior);
        if denominator > 0.0 {
            self.posterior = numerator / denominator;
        }
    }

    pub fn is_supported(&self) -> bool {
        self.posterior > 0.7
    }

    pub fn is_rejected(&self) -> bool {
        self.posterior < 0.3
    }

    pub fn confidence(&self) -> f64 {
        (self.posterior - 0.5).abs() * 2.0
    }

    pub fn hypothesis_id(&self) -> u64 {
        self.hypothesis_id
    }

    pub fn reset(&mut self) {
        self.posterior = self.prior;
        self.likelihood = 1.0;
    }
}
