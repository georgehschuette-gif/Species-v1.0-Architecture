// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::TensionsError;

pub mod gradient;
pub mod interpretation;
pub mod resolution;

pub use gradient::AmbiguityGradient;
pub use interpretation::AmbiguityInterpretation;
pub use resolution::AmbiguityResolution;

pub const DEFAULT_AMBIGUITY_TOLERANCE: f64 = 0.3;
pub const MAX_AMBIGUITY_LEVEL: f64 = 1.0;

pub fn create_ambiguity_gradient(entropy: f64) -> Result<AmbiguityGradient, TensionsError> {
    AmbiguityGradient::new(entropy)
}

pub fn interpret_ambiguity(signal: f64) -> AmbiguityInterpretation {
    AmbiguityInterpretation::from_signal(signal)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ambiguity_gradient_creation() {
        let g = create_ambiguity_gradient(0.4).unwrap();
        assert!(g.entropy() >= 0.0);
    }
}
