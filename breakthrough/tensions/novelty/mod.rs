// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::TensionsError;

pub mod detection;
pub mod weighting;
pub mod decay;

pub use detection::NoveltyDetection;
pub use weighting::NoveltyWeighting;
pub use decay::NoveltyDecay;

pub const DEFAULT_NOVELTY_THRESHOLD: f64 = 0.6;
pub const MAX_NOVELTY: f64 = 1.0;

pub fn create_novelty_detection(sensitivity: f64) -> Result<NoveltyDetection, TensionsError> {
    NoveltyDetection::new(sensitivity)
}

pub fn compute_novelty_weight(familiarity: f64) -> f64 {
    1.0 - familiarity
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn novelty_weight_inverse_familiarity() {
        assert!((compute_novelty_weight(0.3) - 0.7).abs() < 1e-6);
    }
}
