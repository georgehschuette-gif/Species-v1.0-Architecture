// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::TensionsError;
use crate::contradictions::tension_field::ContradictionState;
use crate::contradictions::resolution::ResolutionOutcome;

pub mod tension_field;
pub mod resolution;
pub mod balance;

pub use tension_field::ContradictionField;
pub use resolution::ContradictionResolution;
pub use balance::ContradictionBalance;

/// Default contradiction tension threshold.
pub const DEFAULT_CONTRADICTION_THRESHOLD: f64 = 0.5;
/// Maximum contradiction intensity before forced resolution.
pub const MAX_CONTRADICTION_INTENSITY: f64 = 1.0;

/// Creates a new contradiction field with default parameters.
pub fn create_contradiction_field(strength: f64) -> Result<ContradictionField, TensionsError> {
    ContradictionField::new(strength)
}

/// Resolves a contradiction given its current state.
pub fn resolve_contradiction(state: ContradictionState) -> Result<ResolutionOutcome, TensionsError> {
    ContradictionResolution::resolve(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn contradiction_field_creation() {
        let field = create_contradiction_field(0.7).unwrap();
        assert!(field.strength() > 0.0);
    }
}
