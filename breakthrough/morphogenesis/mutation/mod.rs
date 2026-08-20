// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::MorphogenesisError;

pub mod point;
pub mod insertion;
pub mod deletion;

pub use point::PointMutation;
pub use insertion::InsertionMutation;
pub use deletion::DeletionMutation;

pub const DEFAULT_MUTATION_RATE: f64 = 0.01;
pub const MAX_MUTATION_MAGNITUDE: f64 = 1.0;

pub fn create_point_mutation(position: usize, magnitude: f64) -> Result<PointMutation, MorphogenesisError> {
    PointMutation::new(position, magnitude)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn point_mutation_creation() {
        let m = create_point_mutation(5, 0.2).unwrap();
        assert_eq!(m.position(), 5);
    }
}
