// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

pub mod selection;
pub mod drift;
pub mod adaptation;

pub use selection::EvolutionarySelection;
pub use drift::EvolutionaryDrift;
pub use adaptation::EvolutionaryAdaptation;

pub const DEFAULT_SELECTION_PRESSURE: f64 = 0.5;
pub const MAX_FITNESS: f64 = 1.0;

pub fn create_evolutionary_selection(population_size: usize) -> EvolutionarySelection {
    EvolutionarySelection::new(population_size)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn evolutionary_selection_creation() {
        let s = create_evolutionary_selection(100);
        assert_eq!(s.population_size(), 100);
    }
}
