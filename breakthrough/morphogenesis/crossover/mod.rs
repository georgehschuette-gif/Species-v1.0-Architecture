// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

pub mod single;
pub mod multi;
pub mod uniform;

pub use single::SingleCrossover;
pub use multi::MultiCrossover;
pub use uniform::UniformCrossover;

pub const DEFAULT_CROSSOVER_RATE: f64 = 0.7;
pub const MAX_CROSSOVER_POINTS: usize = 100;

pub fn create_single_crossover(point: usize) -> SingleCrossover {
    SingleCrossover::new(point)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn single_crossover_creation() {
        let c = create_single_crossover(5);
        assert_eq!(c.point(), 5);
    }
}
