// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

pub mod nucleation;
pub mod growth;
pub mod lattice;

pub use nucleation::Nucleation;
pub use growth::CrystalGrowth;
pub use lattice::CrystalLattice;

pub const DEFAULT_NUCLEATION_RATE: f64 = 0.1;
pub const MAX_CRYSTAL_SIZE: usize = 10000;

pub fn create_nucleation(seed_size: usize) -> Nucleation {
    Nucleation::new(seed_size)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nucleation_creation() {
        let n = create_nucleation(5);
        assert_eq!(n.seed_size(), 5);
    }
}
