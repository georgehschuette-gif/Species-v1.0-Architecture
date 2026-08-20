// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct MetastableState {
    pub position: f64,
    pub activation_energy: f64,
    pub lifetime: f64,
}

impl MetastableState {
    pub fn new(position: f64, activation_energy: f64, lifetime: f64) -> Self {
        Self { position, activation_energy, lifetime }
    }

    pub fn decay_probability(&self, dt: f64) -> f64 {
        1.0 - (-dt / self.lifetime).exp()
    }

    pub fn can_transition(&self, energy_input: f64) -> bool {
        energy_input >= self.activation_energy
    }
}

impl fmt::Display for MetastableState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MetastableState(pos={:.2}, activation={:.2}, lifetime={:.2})", self.position, self.activation_energy, self.lifetime)
    }
}

