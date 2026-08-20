// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// SemanticCoherence: Topical consistency within a semantic field.
pub struct SemanticCoherence {
    pub local_density: f64,
    pub global_alignment: f64,
    pub noise_estimate: f64,
}

impl SemanticCoherence {
    pub fn new(local_density: f64, global_alignment: f64, noise_estimate: f64) -> Result<Self, SemanticError> {
        if !(0.0..=1.0).contains(&local_density) {
            return Err(SemanticError::InvalidDensity { density: local_density });
        }
        if !(0.0..=1.0).contains(&global_alignment) {
            return Err(SemanticError::InvalidAlignment { alignment: global_alignment });
        }
        if !(0.0..=1.0).contains(&noise_estimate) {
            return Err(SemanticError::InvalidNoise { noise: noise_estimate });
        }
        Ok(Self { local_density, global_alignment, noise_estimate })
    }

    pub fn overall_coherence(&self) -> f64 {
        (self.local_density * 0.4 + self.global_alignment * 0.4 + (1.0 - self.noise_estimate) * 0.2).clamp(0.0, 1.0)
    }

    pub fn is_coherent(&self) -> bool {
        self.overall_coherence() > 0.5
    }

    pub fn signal_to_noise(&self) -> f64 {
        if self.noise_estimate < f64::EPSILON { f64::INFINITY } else { (1.0 - self.noise_estimate) / self.noise_estimate }
    }
}
