// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// BeliefMixture: Composite distribution from multiple sources with blending methods.
pub struct BeliefMixture {
    pub components: Vec<BayesianBelief>,
    pub weights: Vec<f64>,
    pub mixture_posterior: f64,
}

impl BeliefMixture {
    pub fn new(components: Vec<BayesianBelief>, weights: Vec<f64>) -> Result<Self, ProbabilisticError> {
        if components.len() != weights.len() {
            return Err(ProbabilisticError::InvalidDistribution);
        }
        if components.is_empty() {
            return Err(ProbabilisticError::InsufficientData);
        }
        let weight_sum: f64 = weights.iter().sum();
        if (weight_sum - 1.0).abs() > 1e-6 {
            return Err(ProbabilisticError::InvalidProbability { probability: weight_sum });
        }
        let mixture_posterior = components.iter().zip(weights.iter()).map(|(b, w)| b.posterior * w).sum();
        Ok(Self { components, weights, mixture_posterior })
    }

    pub fn add_component(&mut self, belief: BayesianBelief, weight: f64) -> Result<(), ProbabilisticError> {
        if !(0.0..=1.0).contains(&weight) {
            return Err(ProbabilisticError::InvalidProbability { probability: weight });
        }
        self.components.push(belief);
        self.weights.push(weight);
        self.recompute();
        Ok(())
    }

    pub fn recompute(&mut self) {
        let weight_sum: f64 = self.weights.iter().sum();
        if weight_sum > 0.0 {
            self.mixture_posterior = self.components.iter().zip(&self.weights).map(|(b, w)| b.posterior * w / weight_sum).sum();
        }
    }

    pub fn is_biased(&self) -> bool {
        self.mixture_posterior > 0.7 || self.mixture_posterior < 0.3
    }

    pub fn mixture_confidence(&self) -> f64 {
        (self.mixture_posterior - 0.5).abs() * 2.0
    }

    pub fn component_count(&self) -> usize {
        self.components.len()
    }
}
