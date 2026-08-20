// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// ProbabilisticMap: Represents perceptual hypotheses with probabilities.
pub struct ProbabilisticMap {
    pub hypotheses: Vec<(String, f64)>,
}

impl ProbabilisticMap {
    /// Create a newProbabilisticMap with a given capacity.
    pub fn new(capacity: usize) -> PerceptionResult<Self> {
        if capacity == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "capacity must be positive".into(),
            ));
        }
        Ok(Self {
            hypotheses: Vec::with_capacity(capacity),
        })
    }

    /// Insert a hypothesis with its probability.
    pub fn insert(&mut self, label: String, probability: f64) -> PerceptionResult<()> {
        if probability < 0.0 || probability > 1.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "probability must be in [0, 1]".into(),
            ));
        }
        if self.hypotheses.len() >= self.capacity() {
            return Err(PerceptionError::InvalidConfiguration(
                "hypothesis capacity exceeded".into(),
            ));
        }
        self.hypotheses.push((label, probability));
        self.normalize();
        Ok(())
    }

    /// Remove a hypothesis by label.
    pub fn remove(&mut self, label: &str) -> PerceptionResult<()> {
        let initial_len = self.hypotheses.len();
        self.hypotheses.retain(|(l, _)| l != label);
        if self.hypotheses.len() == initial_len {
            return Err(PerceptionError::InvalidConfiguration(
                format!("hypothesis '{}' not found", label),
            ));
        }
        self.normalize();
        Ok(())
    }

    /// Retrieve the probability of a specific hypothesis.
    pub fn probability(&self, label: &str) -> PerceptionResult<f64> {
        self.hypotheses
            .iter()
            .find(|(l, _)| l == label)
            .map(|(_, p)| *p)
            .ok_or_else(|| {
                PerceptionError::InvalidConfiguration(
                    format!("hypothesis '{}' not found", label),
                )
            })
    }

    /// Return the most probable hypothesis.
    pub fn most_likely(&self) -> PerceptionResult<&str> {
        self.hypotheses
            .iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(l, _)| l.as_str())
            .ok_or_else(|| {
                PerceptionError::InsufficientData
            })
    }

    /// Normalize all probabilities so they sum to 1.0.
    pub fn normalize(&mut self) {
        let total: f64 = self.hypotheses.iter().map(|(_, p)| p).sum();
        if total > 0.0 {
            for (_, p) in &mut self.hypotheses {
                *p /= total;
            }
        }
    }

    /// Compute the entropy of the probability distribution.
    pub fn entropy(&self) -> f64 {
        if self.hypotheses.is_empty() {
            return 0.0;
        }
        let total: f64 = self.hypotheses.iter().map(|(_, p)| p).sum();
        if total <= 0.0 {
            return 0.0;
        }
        let mut ent = 0.0;
        for (_, p) in &self.hypotheses {
            if *p > 0.0 {
                ent -= p * p.log2();
            }
        }
        ent
    }

    /// Compute the Kullback-Leibler divergence from another distribution.
    pub fn kl_divergence(&self, other: &ProbabilisticMap) -> PerceptionResult<f64> {
        if self.hypotheses.is_empty() || other.hypotheses.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        let mut divergence = 0.0;
        for (label, p) in &self.hypotheses {
            if *p <= 0.0 {
                continue;
            }
            let q = other.probability(label)?;
            if q <= 0.0 {
                return Err(PerceptionError::InvalidConfiguration(
                    format!("KL divergence undefined for zero-probability hypothesis '{}'", label),
                ));
            }
            divergence += p * (p / q).ln();
        }
        Ok(divergence)
    }

    /// Return the number of hypotheses in the map.
    pub fn len(&self) -> usize {
        self.hypotheses.len()
    }

    /// Return the hypothesis capacity.
    pub fn capacity(&self) -> usize {
        self.hypotheses.capacity()
    }

    /// Whether the map contains no hypotheses.
    pub fn is_empty(&self) -> bool {
        self.hypotheses.is_empty()
    }

    /// Clear all hypotheses from the map.
    pub fn clear(&mut self) {
        self.hypotheses.clear();
    }

    /// Return the most probable hypothesis as an owned String.
    pub fn most_likely_owned(&self) -> PerceptionResult<String> {
        self.most_likely().map(|s| s.to_string())
    }
}