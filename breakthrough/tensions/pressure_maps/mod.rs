// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::TensionsError;

pub mod field;
pub mod gradient;
pub mod relief;

pub use field::PressureField;
pub use gradient::PressureGradient;
pub use relief::PressureRelief;

pub const DEFAULT_PRESSURE_THRESHOLD: f64 = 0.7;
pub const MAX_PRESSURE: f64 = 1.0;

pub fn create_pressure_field(dimensions: usize) -> PressureField {
    PressureField::new(dimensions)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pressure_field_creation() {
        let f = create_pressure_field(3);
        assert_eq!(f.dimensions(), 3);
    }
}
