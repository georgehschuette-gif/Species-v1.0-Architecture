// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::MetabolismError;
use crate::activation::ActivationThreshold;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Activation {
    Dormant,
    Active,
    Hyperactive,
}

impl Activation {
    pub fn promote(&mut self, threshold: &ActivationThreshold) -> Result<(), MetabolismError> {
        match self {
            Activation::Dormant => {
                if threshold.min_energy <= threshold.max_energy {
                    *self = Activation::Active;
                    Ok(())
                } else {
                    Err(MetabolismError::InvalidThreshold)
                }
            }
            Activation::Active => {
                *self = Activation::Hyperactive;
                Ok(())
            }
            Activation::Hyperactive => Err(MetabolismError::Configuration(
                "Already hyperactive".into(),
            )),
        }
    }

    pub fn suppress(&mut self) {
        *self = Activation::Dormant;
    }
}

