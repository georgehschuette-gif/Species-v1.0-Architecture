// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrontierError {
    InvalidThreshold(String),
    InvalidResolution(String),
    InvalidVolume(String),
}

impl fmt::Display for FrontierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FrontierError::InvalidThreshold(msg) => write!(f, "Invalid threshold: {}", msg),
            FrontierError::InvalidResolution(msg) => write!(f, "Invalid resolution: {}", msg),
            FrontierError::InvalidVolume(msg) => write!(f, "Invalid volume: {}", msg),
        }
    }
}

impl std::error::Error for FrontierError {}

pub struct FrontierDetection {
    pub detection_id: u64,
    pub threshold: f64,
    pub resolution: u64,
    pub frontier_cells: Vec<(u64, u64, u64)>,
}

impl FrontierDetection {
    pub fn new(detection_id: u64, threshold: f64, resolution: u64) -> Result<Self, FrontierError> {
        if !(0.0..=1.0).contains(&threshold) {
            return Err(FrontierError::InvalidThreshold(format!(
                "Threshold {} out of range",
                threshold
            )));
        }
        if resolution == 0 {
            return Err(FrontierError::InvalidResolution(
                "Resolution must be positive".to_string(),
            ));
        }
        Ok(Self {
            detection_id,
            threshold,
            resolution,
            frontier_cells: Vec::new(),
        })
    }

    pub fn scan(&mut self, field: &[f64]) -> Result<(), FrontierError> {
        if field.len() != self.resolution as usize {
            return Err(FrontierError::InvalidVolume(
                "Field length does not match resolution".to_string(),
            ));
        }
        self.frontier_cells.clear();
        for (idx, &val) in field.iter().enumerate() {
            if val > self.threshold {
                self.frontier_cells.push((idx as u64, 0, 0));
            }
        }
        Ok(())
    }

    pub fn frontier_cells(&self) -> &[(u64, u64, u64)] {
        &self.frontier_cells
    }

    pub fn count(&self) -> u64 {
        self.frontier_cells.len() as u64
    }
}

impl fmt::Display for FrontierDetection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FrontierDetection(id={}, threshold={:.2}, resolution={}, cells={})",
            self.detection_id,
            self.threshold,
            self.resolution,
            self.frontier_cells.len()
        )
    }
}
