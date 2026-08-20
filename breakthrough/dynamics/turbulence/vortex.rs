// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Vortex {
    pub center: [f64; 2],
    pub intensity: f64,
    pub radius: f64,
    pub is_rotating: bool,
}

impl Vortex {
    pub fn new(center: [f64; 2], intensity: f64, radius: f64) -> Self {
        Self { center, intensity, radius, is_rotating: true }
    }

    pub fn velocity_at(&self, point: [f64; 2]) -> f64 {
        let dx = point[0] - self.center[0];
        let dy = point[1] - self.center[1];
        let r = (dx.powi(2) + dy.powi(2)).sqrt();
        if r < self.radius && r > 1e-6 {
            self.intensity * (1.0 - r / self.radius)
        } else {
            0.0
        }
    }

    pub fn stop(&mut self) {
        self.is_rotating = false;
    }
}

impl fmt::Display for Vortex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vortex(center=[{:.2}, {:.2}], intensity={:.2}, radius={:.2})", self.center[0], self.center[1], self.intensity, self.radius)
    }
}

