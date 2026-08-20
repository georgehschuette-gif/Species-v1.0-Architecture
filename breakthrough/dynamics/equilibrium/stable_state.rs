// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct StableState {
    pub position: f64,
    pub potential_energy: f64,
    pub stability_margin: f64,
}

impl StableState {
    pub fn new(position: f64, potential_energy: f64, stability_margin: f64) -> Self {
        Self { position, potential_energy, stability_margin }
    }

    pub fn is_stable(&self, perturbation: f64) -> bool {
        perturbation.abs() < self.stability_margin
    }

    pub fn energy_gap(&self, other_position: f64) -> f64 {
        (other_position - self.position).abs() * self.potential_energy
    }
}

impl fmt::Display for StableState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StableState(pos={:.2}, energy={:.2}, margin={:.2})", self.position, self.potential_energy, self.stability_margin)
    }
}

