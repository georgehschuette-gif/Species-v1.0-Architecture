// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

pub mod homologous;
pub mod non_homologous;
pub mod site_specific;

pub use homologous::HomologousRecombination;
pub use non_homologous::NonHomologousRecombination;
pub use site_specific::SiteSpecificRecombination;

pub const DEFAULT_RECOMBINATION_RATE: f64 = 0.5;
pub const MAX_RECOMBINATION_EVENTS: usize = 1000;

pub fn create_homologous_recombination(homology_length: usize) -> HomologousRecombination {
    HomologousRecombination::new(homology_length)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn homologous_recombination_creation() {
        let r = create_homologous_recombination(10);
        assert_eq!(r.homology_length(), 10);
    }
}
