// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::MorphogenesisError;

pub mod energy;
pub mod structure;
pub mod equilibrium;

pub use energy::StabilizationEnergy;
pub use structure::StructuralStabilization;
pub use equilibrium::StabilizationEquilibrium;

pub const DEFAULT_ENERGY_BARRIER: f64 = 0.5;
pub const MAX_STABILITY: f64 = 1.0;

pub fn create_stabilization_energy(barrier: f64) -> Result<StabilizationEnergy, MorphogenesisError> {
    StabilizationEnergy::new(barrier)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stabilization_energy_creation() {
        let e = create_stabilization_energy(0.6).unwrap();
        assert!(e.barrier() > 0.0);
    }
}
