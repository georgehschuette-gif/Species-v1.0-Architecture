// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// ConfidenceMap: Tracks reliability of perceptual interpretations.
///
/// Maintains a per-signal confidence vector updated by quality
/// measurements and temporal decay.
pub struct ConfidenceMap {
    pub values: Vec<f64>,
    pub smoothing: f64,
    pub min_confidence: f64,
    pub update_count: usize,
}

impl ConfidenceMap {
    /// Create a confidence map with a given signal length.
    pub fn new(length: usize, initial_confidence: f64) -> PerceptionResult<Self> {
        if length == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "confidence map length must be positive".into(),
            ));
        }
        if !(0.0..=1.0).contains(&initial_confidence) {
            return Err(PerceptionError::InvalidConfiguration(
                "initial confidence must be in [0, 1]".into(),
            ));
        }
        Ok(Self {
            values: vec![initial_confidence; length],
            smoothing: 0.9,
            min_confidence: 0.01,
            update_count: 0,
        })
    }

    /// Exponential moving average update with quality feedback.
    pub fn update(&mut self, signal_idx: usize, quality: f64) -> PerceptionResult<()> {
        if signal_idx >= self.values.len() {
            return Err(PerceptionError::OutOfBounds(format!(
                "signal index {} out of range {}",
                signal_idx,
                self.values.len()
            )));
        }
        let q = quality.clamp(0.0, 1.0);
        self.values[signal_idx] = self.smoothing * self.values[signal_idx] + (1.0 - self.smoothing) * q;
        self.values[signal_idx] = self.values[signal_idx].clamp(self.min_confidence, 1.0);
        self.update_count += 1;
        Ok(())
    }

    /// Apply global temporal decay across all confidence values.
    pub fn decay(&mut self, factor: f64) -> PerceptionResult<()> {
        if !(0.0..1.0).contains(&factor) {
            return Err(PerceptionError::InvalidConfiguration(
                "decay factor must be in (0, 1)".into(),
            ));
        }
        for v in &mut self.values {
            *v *= factor;
            *v = (*v).clamp(self.min_confidence, 1.0);
        }
        Ok(())
    }

    /// Average confidence across all signals.
    pub fn mean(&self) -> f64 {
        if self.values.is_empty() {
            0.0
        } else {
            self.values.iter().sum::<f64>() / self.values.len() as f64
        }
    }

    /// Index of the signal with the highest confidence.
    pub fn argmax(&self) -> Option<usize> {
        self.values.iter().enumerate().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).map(|(i, _)| i)
    }

    /// Retrieve the confidence value for a specific signal index.
    pub fn get(&self, signal_idx: usize) -> Option<f64> {
        self.values.get(signal_idx).cloned()
    }
}
