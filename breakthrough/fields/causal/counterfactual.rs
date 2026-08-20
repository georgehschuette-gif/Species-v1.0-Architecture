// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// CounterfactualGradient: Sensitivity of outcomes to hypothetical interventions.
pub struct CounterfactualGradient {
    pub node_id: u64,
    pub gradient_vector: Vec<f64>,
    pub confidence: f64,
    pub baseline: f64,
}

impl CounterfactualGradient {
    pub fn new(node_id: u64, gradient_vector: Vec<f64>, baseline: f64) -> Result<Self, CausalError> {
        if node_id == 0 {
            return Err(CausalError::InvalidNode { node_id });
        }
        if gradient_vector.is_empty() {
            return Err(CausalError::InsufficientData);
        }
        Ok(Self { node_id, gradient_vector, confidence: 1.0, baseline })
    }

    pub fn intervene(&self, intervention: f64) -> f64 {
        self.baseline + intervention * self.gradient_vector.first().copied().unwrap_or(0.0)
    }

    pub fn gradient_magnitude(&self) -> f64 {
        self.gradient_vector.iter().map(|v| v * v).sum::<f64>().sqrt()
    }
}
