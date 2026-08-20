// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExplorationError {
    InvalidBearing(String),
    InvalidDepth(String),
    MapOverflow(String),
}

impl fmt::Display for ExplorationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExplorationError::InvalidBearing(msg) => write!(f, "Invalid bearing: {}", msg),
            ExplorationError::InvalidDepth(msg) => write!(f, "Invalid depth: {}", msg),
            ExplorationError::MapOverflow(msg) => write!(f, "Map overflow: {}", msg),
        }
    }
}

impl std::error::Error for ExplorationError {}

pub struct Exploration {
    pub exploration_id: u64,
    pub bearing: f64,
    pub depth: u64,
    pub discovered_count: u64,
}

impl Exploration {
    pub fn new(exploration_id: u64, bearing: f64, depth: u64) -> Result<Self, ExplorationError> {
        if !(0.0..=360.0).contains(&bearing) {
            return Err(ExplorationError::InvalidBearing(format!(
                "Bearing {} out of range",
                bearing
            )));
        }
        if depth == 0 {
            return Err(ExplorationError::InvalidDepth(
                "Depth must be positive".to_string(),
            ));
        }
        Ok(Self {
            exploration_id,
            bearing,
            depth,
            discovered_count: 0,
        })
    }

    pub fn probe(&mut self) -> Result<u64, ExplorationError> {
        if self.discovered_count >= self.depth {
            return Err(ExplorationError::MapOverflow(
                "Maximum depth reached".to_string(),
            ));
        }
        self.discovered_count += 1;
        Ok(self.discovered_count)
    }

    pub fn discovered(&self) -> u64 {
        self.discovered_count
    }

    pub fn remaining(&self) -> u64 {
        self.depth - self.discovered_count
    }
}

impl fmt::Display for Exploration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Exploration(id={}, bearing={:.1}, depth={}, discovered={})",
            self.exploration_id, self.bearing, self.depth, self.discovered_count
        )
    }
}
