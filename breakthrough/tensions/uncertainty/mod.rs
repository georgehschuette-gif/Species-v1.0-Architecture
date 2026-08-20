// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::TensionsError;

pub mod measure;
pub mod propagation;
pub mod reduction;

pub use measure::UncertaintyMeasure;
pub use propagation::UncertaintyPropagation;
pub use reduction::UncertaintyReduction;

pub const DEFAULT_UNCERTAINTY_DECAY: f64 = 0.1;
pub const MAX_UNCERTAINTY: f64 = 1.0;

pub fn create_uncertainty_measure(initial: f64) -> Result<UncertaintyMeasure, TensionsError> {
    UncertaintyMeasure::new(initial)
}

pub fn propagate_uncertainty(source: &UncertaintyMeasure) -> UncertaintyPropagation {
    UncertaintyPropagation::from_source(source)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn uncertainty_measure_creation() {
        let m = create_uncertainty_measure(0.3).unwrap();
        assert!(m.value() > 0.0);
    }
}
