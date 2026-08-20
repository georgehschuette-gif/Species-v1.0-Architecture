// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::ImaginationError;

pub mod property;
pub mod construction;

pub use property::ParadoxicalProperty;
pub use construction::ImpossibleObject;

pub const MAX_PARADOX_INTENSITY: f64 = 1.0;
pub const DEFAULT_OBJECT_COHERENCE: f64 = 0.0;

pub fn create_impossible_object(name: impl Into<String>) -> ImpossibleObject {
    ImpossibleObject::new(name)
}

pub fn create_paradoxical_property(description: impl Into<String>, intensity: f64) -> Result<ParadoxicalProperty, ImaginationError> {
    ParadoxicalProperty::new(description, intensity)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn impossible_object_creation() {
        let obj = create_impossible_object("Penrose Triangle");
        assert_eq!(obj.name, "Penrose Triangle");
    }
}
