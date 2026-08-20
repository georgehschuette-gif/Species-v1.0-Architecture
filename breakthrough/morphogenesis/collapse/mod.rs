// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::MorphogenesisError;

pub mod structural_collapse;
pub mod energy_collapse;
pub mod information_collapse;

pub use structural_collapse::StructuralCollapse;
pub use energy_collapse::EnergyCollapse;
pub use information_collapse::InformationCollapse;

pub const DEFAULT_COLLAPSE_THRESHOLD: f64 = 0.8;
pub const MAX_COLLAPSE_MAGNITUDE: f64 = 1.0;

pub fn trigger_structural_collapse(strain: f64) -> Result<StructuralCollapse, MorphogenesisError> {
    StructuralCollapse::new(strain)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn structural_collapse_creation() {
        let c = trigger_structural_collapse(0.9).unwrap();
        assert!(c.strain() > 0.0);
    }
}
