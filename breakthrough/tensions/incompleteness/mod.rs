// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::TensionsError;

pub mod gap;
pub mod completion;
pub mod tolerance;

pub use gap::IncompletenessGap;
pub use completion::IncompletenessCompletion;
pub use tolerance::IncompletenessTolerance;

pub const DEFAULT_GAP_TOLERANCE: f64 = 0.2;
pub const MAX_INCOMPLETENESS: f64 = 1.0;

pub fn measure_gap(known: f64, total: f64) -> f64 {
    if total <= 0.0 { return 0.0; }
    (1.0 - known / total).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gap_measurement() {
        let g = measure_gap(3.0, 10.0);
        assert!((g - 0.7).abs() < 1e-6);
    }
}
