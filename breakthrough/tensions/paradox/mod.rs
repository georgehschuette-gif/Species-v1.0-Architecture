// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::TensionsError;

pub mod containment;
pub mod suspension;
pub mod resolution;

pub use containment::ParadoxContainment;
pub use suspension::ParadoxSuspension;
pub use resolution::ParadoxResolution;

pub const DEFAULT_PARADOX_TOLERANCE: f64 = 0.4;
pub const MAX_PARADOX_INTENSITY: f64 = 1.0;

pub fn contain_paradox(intensity: f64) -> Result<ParadoxContainment, TensionsError> {
    ParadoxContainment::new(intensity)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn paradox_containment_creation() {
        let c = contain_paradox(0.6).unwrap();
        assert!(c.intensity() > 0.0);
    }
}
