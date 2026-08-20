// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

use crate::TensionsError;

pub struct NoveltyDetection {
    pub sensitivity: f64,
    pub baseline: f64,
    pub history: Vec<f64>,
}

impl NoveltyDetection {
    pub fn new(sensitivity: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&sensitivity) { return Err(TensionsError::OutOfRange { field: "sensitivity".into(), value: sensitivity, min: 0.0, max: 1.0 }); }
        Ok(Self { sensitivity, baseline: 0.0, history: Vec::new() })
    }

    pub fn observe(&mut self, signal: f64) -> Result<f64, TensionsError> {
        self.history.push(signal);
        if self.history.len() > 1000 { self.history.remove(0); }
        let mean = self.history.iter().sum::<f64>() / self.history.len() as f64;
        let novelty = (signal - mean).abs() * self.sensitivity;
        Ok(novelty.clamp(0.0, 1.0))
    }

    pub fn update_baseline(&mut self) { self.baseline = self.history.iter().sum::<f64>() / self.history.len() as f64; }
    pub fn is_novel(&self, signal: f64, threshold: f64) -> bool { (signal - self.baseline).abs() > threshold }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.sensitivity) { return Err(TensionsError::OutOfRange { field: "sensitivity".into(), value: self.sensitivity, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for NoveltyDetection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NoveltyDetection").field("sensitivity", &self.sensitivity).field("baseline", &self.baseline).finish()
    }
}
