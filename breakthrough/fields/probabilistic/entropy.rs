// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// ProbabilisticEntropy: Uncertainty measure in a belief distribution with computation methods.
pub struct ProbabilisticEntropy {
    pub distribution: Vec<f64>,
    pub entropy: f64,
    pub max_entropy: f64,
}

impl ProbabilisticEntropy {
    pub fn new(distribution: Vec<f64>) -> Result<Self, ProbabilisticError> {
        if distribution.is_empty() {
            return Err(ProbabilisticError::InsufficientData);
        }
        let total: f64 = distribution.iter().sum();
        if (total - 1.0).abs() > 1e-6 {
            return Err(ProbabilisticError::InvalidProbability { probability: total });
        }
        for &p in &distribution {
            if p < 0.0 {
                return Err(ProbabilisticError::InvalidProbability { probability: p });
            }
        }
        let entropy = distribution.iter().filter(|&&p| p > 0.0).map(|&p| -p * p.ln()).sum();
        let n = distribution.len() as f64;
        let max_entropy = n.ln();
        Ok(Self { distribution, entropy, max_entropy })
    }

    pub fn normalized_entropy(&self) -> f64 {
        if self.max_entropy > 0.0 {
            self.entropy / self.max_entropy
        } else {
            0.0
        }
    }

    pub fn is_uniform(&self) -> bool {
        self.normalized_entropy() > 0.95
    }

    pub fn is_deterministic(&self) -> bool {
        self.normalized_entropy() < 0.05
    }

    pub fn information_gain(&self, other: &ProbabilisticEntropy) -> f64 {
        self.normalized_entropy() - other.normalized_entropy()
    }

    pub fn uncertainty(&self) -> f64 {
        1.0 - self.normalized_entropy()
    }
}
