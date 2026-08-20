// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// ReconstructionDetector: Identifies anomalies via predictive reconstruction error.
pub struct ReconstructionDetector {
    pub error_threshold: f64,
    pub model: String,
    pub history: Vec<(Vec<f64>, f64)>,
    pub anomaly_count: usize,
}

impl ReconstructionDetector {
    /// Create a new reconstruction detector.
    pub fn new(error_threshold: f64, model: String) -> PerceptionResult<Self> {
        if error_threshold <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "error_threshold must be positive".into(),
            ));
        }
        if model.is_empty() {
            return Err(PerceptionError::InvalidConfiguration(
                "model name must not be empty".into(),
            ));
        }
        Ok(Self {
            error_threshold,
            model,
            history: Vec::new(),
            anomaly_count: 0,
        })
    }

    /// Compute the reconstruction error for an input.
    pub fn reconstruction_error(&self, input: &[f64]) -> PerceptionResult<f64> {
        if input.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        let mean = input.iter().sum::<f64>() / input.len() as f64;
        let reconstructed: Vec<f64> = vec![mean; input.len()];
        let error: f64 = input
            .iter()
            .zip(reconstructed.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt();
        Ok(error)
    }

    /// Detect an anomaly based on reconstruction error.
    pub fn detect(&mut self, input: &[f64]) -> PerceptionResult<bool> {
        let error = self.reconstruction_error(input)?;
        self.history.push((input.to_vec(), error));
        if self.history.len() > 10_000 {
            self.history.remove(0);
        }
        let is_anomaly = error > self.error_threshold;
        if is_anomaly {
            self.anomaly_count += 1;
        }
        Ok(is_anomaly)
    }

    /// Return the average reconstruction error across history.
    pub fn average_error(&self) -> PerceptionResult<f64> {
        if self.history.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        let total: f64 = self.history.iter().map(|(_, e)| e).sum();
        Ok(total / self.history.len() as f64)
    }

    /// Return the maximum reconstruction error observed.
    pub fn max_error(&self) -> PerceptionResult<f64> {
        if self.history.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        Ok(self
            .history
            .iter()
            .map(|(_, e)| e)
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max))
    }

    /// Return the number of anomalies detected so far.
    pub fn anomaly_count(&self) -> usize {
        self.anomaly_count
    }

    /// Update the error threshold.
    pub fn set_threshold(&mut self, threshold: f64) -> PerceptionResult<()> {
        if threshold <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "threshold must be positive".into(),
            ));
        }
        self.error_threshold = threshold;
        Ok(())
    }

    /// Update the model name.
    pub fn set_model(&mut self, model: String) -> PerceptionResult<()> {
        if model.is_empty() {
            return Err(PerceptionError::InvalidConfiguration(
                "model name must not be empty".into(),
            ));
        }
        self.model = model;
        Ok(())
    }

    /// Reset the detector history and anomaly counter.
    pub fn reset(&mut self) {
        self.history.clear();
        self.anomaly_count = 0;
    }

    /// Validate the detector configuration.
    pub fn validate(&self) -> PerceptionResult<()> {
        if self.error_threshold <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "error_threshold must be positive".into(),
            ));
        }
        if self.model.is_empty() {
            return Err(PerceptionError::InvalidConfiguration(
                "model name must not be empty".into(),
            ));
        }
        Ok(())
    }
}

fn identity_matrix(n: usize) -> Vec<Vec<f64>> {
    let mut matrix = vec![vec![0.0; n]; n];
    for i in 0..n {
        matrix[i][i] = 1.0;
    }
    matrix
}