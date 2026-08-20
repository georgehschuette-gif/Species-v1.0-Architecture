// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::MetabolismError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivationThreshold {
    pub min_energy: u64,
    pub max_energy: u64,
    pub hysteresis: u64,
}

impl ActivationThreshold {
    pub fn new(min_energy: u64, max_energy: u64, hysteresis: u64) -> Result<Self, MetabolismError> {
        if min_energy > max_energy {
            return Err(MetabolismError::InvalidThreshold);
        }
        if hysteresis > max_energy - min_energy {
            return Err(MetabolismError::Configuration(
                "Hysteresis exceeds threshold band".into(),
            ));
        }
        Ok(Self {
            min_energy,
            max_energy,
            hysteresis,
        })
    }

    pub fn is_crossed(&self, energy: u64) -> bool {
        energy >= self.min_energy
    }
}

