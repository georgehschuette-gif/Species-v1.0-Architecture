// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// ThresholdDetector: Flags anomalies exceeding fixed thresholds.
pub struct ThresholdDetector {
    pub threshold: f64,
    pub sensitivity: f64,
    pub anomaly_count: usize,
    pub history: Vec<f64>,
}

impl ThresholdDetector {
    /// Create a new threshold detector.
    pub fn new(threshold: f64, sensitivity: f64) -> PerceptionResult<Self> {
        if threshold <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "threshold must be positive".into(),
            ));
        }
        if !(0.0..=1.0).contains(&sensitivity) {
            return Err(PerceptionError::InvalidConfiguration(
                "sensitivity must be in [0, 1]".into(),
            ));
        }
        Ok(Self {
            threshold,
            sensitivity,
            anomaly_count: 0,
            history: Vec::new(),
        })
    }

    /// Check a measurement against the threshold.
    pub fn detect(&mut self, measurement: f64) -> PerceptionResult<bool> {
        let adjusted_threshold = self.threshold * (1.0 - self.sensitivity);
        let is_anomaly = measurement > adjusted_threshold;
        self.history.push(measurement);
        if self.history.len() > 10_000 {
            self.history.remove(0);
        }
        if is_anomaly {
            self.anomaly_count += 1;
        }
        Ok(is_anomaly)
    }

    /// Check a full feature vector against the threshold using its magnitude.
    pub fn detect_vector(&mut self, features: &[f64]) -> PerceptionResult<bool> {
        if features.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        let magnitude: f64 = features.iter().map(|v| v.powi(2)).sum::<f64>().sqrt();
        self.detect(magnitude)
    }

    /// Return the current threshold.
    pub fn threshold(&self) -> f64 {
        self.threshold
    }

    /// Return the current sensitivity.
    pub fn sensitivity(&self) -> f64 {
        self.sensitivity
    }

    /// Return the total number of anomalies detected.
    pub fn anomaly_count(&self) -> usize {
        self.anomaly_count
    }

    /// Adjust the threshold upward or downward.
    pub fn adjust_threshold(&mut self, delta: f64) -> PerceptionResult<()> {
        let new_threshold = self.threshold + delta;
        if new_threshold <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "threshold must remain positive after adjustment".into(),
            ));
        }
        self.threshold = new_threshold;
        Ok(())
    }

    /// Adjust the sensitivity.
    pub fn set_sensitivity(&mut self, sensitivity: f64) -> PerceptionResult<()> {
        if !(0.0..=1.0).contains(&sensitivity) {
            return Err(PerceptionError::InvalidConfiguration(
                "sensitivity must be in [0, 1]".into(),
            ));
        }
        self.sensitivity = sensitivity;
        Ok(())
    }

    /// Reset the anomaly counter and history.
    pub fn reset(&mut self) {
        self.anomaly_count = 0;
        self.history.clear();
    }

    /// Validate all parameters are within valid ranges.
    pub fn validate(&self) -> PerceptionResult<()> {
        if self.threshold <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "threshold must be positive".into(),
            ));
        }
        if !(0.0..=1.0).contains(&self.sensitivity) {
            return Err(PerceptionError::InvalidConfiguration(
                "sensitivity must be in [0, 1]".into(),
            ));
        }
        Ok(())
    }
}