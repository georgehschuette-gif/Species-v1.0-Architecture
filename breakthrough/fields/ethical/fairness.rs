// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// EthicalFairness: Distributional justice across affected parties.
pub struct EthicalFairness {
    pub affected_parties: Vec<u64>,
    pub outcome_distribution: Vec<f64>,
    pub equity_index: f64,
    pub bias_score: f64,
}

impl EthicalFairness {
    pub fn new(affected_parties: Vec<u64>, outcome_distribution: Vec<f64>) -> Result<Self, EthicalError> {
        if affected_parties.len() != outcome_distribution.len() {
            return Err(EthicalError::DimensionMismatch {
                values: affected_parties.len(),
                actions: outcome_distribution.len(),
            });
        }
        Ok(Self { affected_parties, outcome_distribution, equity_index: 0.0, bias_score: 0.0 })
    }

    pub fn compute_equity(&mut self) -> Result<f64, EthicalError> {
        if self.outcome_distribution.is_empty() {
            return Err(EthicalError::InsufficientData);
        }
        let mean = self.outcome_distribution.iter().sum::<f64>() / self.outcome_distribution.len() as f64;
        let gini: f64 = {
            let n = self.outcome_distribution.len() as f64;
            let mut sum = 0.0;
            for i in 0..self.outcome_distribution.len() {
                for j in 0..self.outcome_distribution.len() {
                    sum += (self.outcome_distribution[i] - self.outcome_distribution[j]).abs();
                }
            }
            sum / (2.0 * n * n * mean.max(f64::EPSILON))
        };
        self.equity_index = 1.0 - gini;
        Ok(self.equity_index)
    }

    pub fn is_fair(&self) -> bool {
        self.equity_index > 0.7 && self.bias_score < 0.2
    }
}
