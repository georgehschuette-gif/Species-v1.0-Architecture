// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// BayesianFusion: Combines sensory streams using Bayesian inference.
pub struct BayesianFusion {
    pub priors: Vec<f64>,
    pub likelihoods: Vec<Vec<f64>>,
}

impl BayesianFusion {
    /// Create a new BayesianFusion with the given number of hypotheses.
    pub fn new(num_hypotheses: usize) -> PerceptionResult<Self> {
        if num_hypotheses == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "num_hypotheses must be positive".into(),
            ));
        }
        let prior = 1.0 / num_hypotheses as f64;
        Ok(Self {
            priors: vec![prior; num_hypotheses],
            likelihoods: Vec::new(),
        })
    }

    /// Add a likelihood observation for each hypothesis.
    pub fn observe(&mut self, likelihoods: Vec<f64>) -> PerceptionResult<()> {
        if likelihoods.len() != self.priors.len() {
            return Err(PerceptionError::InvalidConfiguration(format!(
                "expected {} likelihoods, got {}",
                self.priors.len(),
                likelihoods.len()
            )));
        }
        for (i, &l) in likelihoods.iter().enumerate() {
            if l < 0.0 {
                return Err(PerceptionError::InvalidConfiguration(format!(
                    "likelihood[{}] must be non-negative",
                    i
                )));
            }
        }
        self.likelihoods.push(likelihoods);
        self.update_posteriors();
        Ok(())
    }

    /// Update the posterior probabilities from priors and accumulated likelihoods.
    fn update_posteriors(&mut self) {
        if self.likelihoods.is_empty() {
            return;
        }
        let num_h = self.priors.len();
        let mut unnormalized = vec![0.0; num_h];
        for h in 0..num_h {
            let mut product = self.priors[h];
            for likelihood_row in &self.likelihoods {
                product *= likelihood_row[h];
            }
            unnormalized[h] = product;
        }
        let total: f64 = unnormalized.iter().sum();
        if total > 0.0 {
            for h in 0..num_h {
                self.priors[h] = unnormalized[h] / total;
            }
        }
    }

    /// Return the posterior probability of a hypothesis by index.
    pub fn posterior(&self, hypothesis: usize) -> PerceptionResult<f64> {
        if hypothesis >= self.priors.len() {
            return Err(PerceptionError::OutOfBounds(format!(
                "hypothesis index {} out of range {}",
                hypothesis,
                self.priors.len()
            )));
        }
        Ok(self.priors[hypothesis])
    }

    /// Return the hypothesis with the highest posterior probability.
    pub fn most_likely(&self) -> PerceptionResult<usize> {
        if self.priors.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        let mut best_idx = 0;
        let mut best_prob = self.priors[0];
        for (i, &p) in self.priors.iter().enumerate() {
            if p > best_prob {
                best_prob = p;
                best_idx = i;
            }
        }
        Ok(best_idx)
    }

    /// Compute the entropy of the current posterior distribution.
    pub fn entropy(&self) -> f64 {
        let mut ent = 0.0;
        for &p in &self.priors {
            if p > 0.0 {
                ent -= p * p.log2();
            }
        }
        ent
    }

    /// Reset the prior distribution to uniform.
    pub fn reset_priors(&mut self) {
        let n = self.priors.len();
        if n == 0 {
            return;
        }
        let uniform = 1.0 / n as f64;
        for p in &mut self.priors {
            *p = uniform;
        }
        self.likelihoods.clear();
    }

    /// Set the prior for a specific hypothesis.
    pub fn set_prior(&mut self, hypothesis: usize, prior: f64) -> PerceptionResult<()> {
        if hypothesis >= self.priors.len() {
            return Err(PerceptionError::OutOfBounds(format!(
                "hypothesis index {} out of range {}",
                hypothesis,
                self.priors.len()
            )));
        }
        if !(0.0..=1.0).contains(&prior) {
            return Err(PerceptionError::InvalidConfiguration(
                "prior must be in [0, 1]".into(),
            ));
        }
        self.priors[hypothesis] = prior;
        self.normalize_priors();
        Ok(())
    }

    /// Normalize priors so they sum to 1.0.
    fn normalize_priors(&mut self) {
        let total: f64 = self.priors.iter().sum();
        if total > 0.0 {
            for p in &mut self.priors {
                *p /= total;
            }
        }
    }

    /// Return the number of hypotheses.
    pub fn num_hypotheses(&self) -> usize {
        self.priors.len()
    }

    /// Reset likelihood observations while keeping priors.
    pub fn clear_observations(&mut self) {
        self.likelihoods.clear();
    }
}