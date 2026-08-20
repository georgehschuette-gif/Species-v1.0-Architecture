// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

pub mod pitchfork;
pub mod hopf;
pub mod period_doubling;

pub use pitchfork::PitchforkBifurcation;
pub use hopf::HopfBifurcation;
pub use period_doubling::PeriodDoubling;

pub const DEFAULT_BIFURCATION_PARAMETER: f64 = 0.5;
pub const MAX_BRANCHES: usize = 16;

pub fn create_pitchfork(parameter: f64) -> PitchforkBifurcation {
    PitchforkBifurcation::new(parameter)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pitchfork_creation() {
        let b = create_pitchfork(0.5);
        assert!(b.parameter() == 0.5);
    }
}
