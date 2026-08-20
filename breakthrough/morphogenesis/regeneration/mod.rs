// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::MorphogenesisError;

pub mod cellular;
pub mod tissue;
pub mod systemic;

pub use cellular::CellularRegeneration;
pub use tissue::TissueRegeneration;
pub use systemic::SystemicRegeneration;

pub const DEFAULT_REGENERATION_RATE: f64 = 0.2;
pub const MAX_REGENERATION_CAPACITY: f64 = 1.0;

pub fn create_cellular_regeneration(capacity: f64) -> Result<CellularRegeneration, MorphogenesisError> {
    CellularRegeneration::new(capacity)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cellular_regeneration_creation() {
        let r = create_cellular_regeneration(0.8).unwrap();
        assert!(r.capacity() > 0.0);
    }
}
