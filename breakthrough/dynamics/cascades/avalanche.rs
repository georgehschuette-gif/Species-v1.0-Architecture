// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Avalanche {
    pub trigger_point: f64,
    pub release_size: f64,
    pub speed: f64,
    pub is_active: bool,
}

impl Avalanche {
    pub fn new(trigger_point: f64, release_size: f64, speed: f64) -> Self {
        Self { trigger_point, release_size, speed, is_active: true }
    }

    pub fn propagate(&mut self, distance: f64) {
        self.release_size += distance * self.speed;
    }

    pub fn stop(&mut self) {
        self.is_active = false;
    }
}

impl fmt::Display for Avalanche {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Avalanche(trigger={:.2}, size={:.2}, speed={:.2})", self.trigger_point, self.release_size, self.speed)
    }
}

