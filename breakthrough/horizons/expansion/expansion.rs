// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpansionError {
    InvalidRadius(String),
    InvalidRate(String),
    ResourceExhaustion(String),
}

impl fmt::Display for ExpansionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpansionError::InvalidRadius(msg) => write!(f, "Invalid radius: {}", msg),
            ExpansionError::InvalidRate(msg) => write!(f, "Invalid rate: {}", msg),
            ExpansionError::ResourceExhaustion(msg) => write!(f, "Resource exhaustion: {}", msg),
        }
    }
}

impl std::error::Error for ExpansionError {}

pub struct Expansion {
    pub expansion_id: u64,
    pub radius: f64,
    pub rate: f64,
    pub occupied_cells: u64,
    pub resources_remaining: f64,
}

impl Expansion {
    pub fn new(expansion_id: u64, radius: f64, rate: f64) -> Result<Self, ExpansionError> {
        if radius <= 0.0 {
            return Err(ExpansionError::InvalidRadius(format!(
                "Radius {} must be positive",
                radius
            )));
        }
        if !(0.0..=1.0).contains(&rate) {
            return Err(ExpansionError::InvalidRate(format!(
                "Rate {} out of range",
                rate
            )));
        }
        Ok(Self {
            expansion_id,
            radius,
            rate,
            occupied_cells: 0,
            resources_remaining: 1.0,
        })
    }

    pub fn step(&mut self) -> Result<u64, ExpansionError> {
        if self.resources_remaining <= 0.0 {
            return Err(ExpansionError::ResourceExhaustion(
                "No resources remaining".to_string(),
            ));
        }
        let growth = (self.rate * self.resources_remaining).min(1.0);
        self.resources_remaining -= growth;
        self.occupied_cells += (growth * 100.0) as u64;
        self.radius += growth;
        Ok(self.occupied_cells)
    }

    pub fn occupied(&self) -> u64 {
        self.occupied_cells
    }

    pub fn is_depleted(&self) -> bool {
        self.resources_remaining <= 0.0
    }
}

impl fmt::Display for Expansion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Expansion(id={}, radius={:.2}, rate={:.2}, cells={}, resources={:.2})",
            self.expansion_id,
            self.radius,
            self.rate,
            self.occupied_cells,
            self.resources_remaining
        )
    }
}
