// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::ImaginationError;

pub mod divergence;
pub mod branch;

pub use divergence::WorldDivergence;
pub use branch::AlternateWorld;

pub const DEFAULT_DIVERGENCE_POINT: f64 = 0.5;
pub const MAX_WORLD_DEPTH: usize = 32;

pub fn create_world_divergence(point: f64) -> Result<WorldDivergence, ImaginationError> {
    WorldDivergence::new(point)
}

pub fn create_alternate_world(id: u64, divergence: f64) -> AlternateWorld {
    AlternateWorld::new(id, divergence)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn world_divergence_creation() {
        let d = create_world_divergence(0.7).unwrap();
        assert!(d.point() > 0.0);
    }
}
