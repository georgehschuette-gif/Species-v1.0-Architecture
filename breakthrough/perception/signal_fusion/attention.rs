// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// AttentionMechanism: Selects which sensory streams to prioritize.
pub struct AttentionMechanism {
    pub weights: Vec<f64>,
    pub focus: usize,
    pub attention_history: Vec<Vec<f64>>,
}

impl AttentionMechanism {
    /// Create a new attention mechanism with a given number of streams.
    pub fn new(num_streams: usize) -> PerceptionResult<Self> {
        if num_streams == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "num_streams must be positive".into(),
            ));
        }
        let weight = 1.0 / num_streams as f64;
        Ok(Self {
            weights: vec![weight; num_streams],
            focus: 0,
            attention_history: Vec::new(),
        })
    }

    /// Compute the weighted attention score for a stream.
    pub fn attention_score(&self, stream_index: usize) -> PerceptionResult<f64> {
        if stream_index >= self.weights.len() {
            return Err(PerceptionError::OutOfBounds(format!(
                "stream index {} out of range {}",
                stream_index,
                self.weights.len()
            )));
        }
        Ok(self.weights[stream_index])
    }

    /// Update weights based on a relevance vector, then normalize.
    pub fn update_weights(&mut self, relevance: &[f64]) -> PerceptionResult<()> {
        if relevance.len() != self.weights.len() {
            return Err(PerceptionError::InvalidConfiguration(format!(
                "expected {} relevance values, got {}",
                self.weights.len(),
                relevance.len()
            )));
        }
        for (i, &r) in relevance.iter().enumerate() {
            if r < 0.0 {
                return Err(PerceptionError::InvalidConfiguration(
                    "relevance values must be non-negative".into(),
                ));
            }
            self.weights[i] = r;
        }
        self.normalize_weights();
        Ok(())
    }

    /// Normalize weights so they sum to 1.0.
    fn normalize_weights(&mut self) {
        let total: f64 = self.weights.iter().sum();
        if total > 0.0 {
            for w in &mut self.weights {
                *w /= total;
            }
        }
    }

    /// Set the focus stream index.
    pub fn set_focus(&mut self, stream_index: usize) -> PerceptionResult<()> {
        if stream_index >= self.weights.len() {
            return Err(PerceptionError::OutOfBounds(format!(
                "stream index {} out of range {}",
                stream_index,
                self.weights.len()
            )));
        }
        self.focus = stream_index;
        Ok(())
    }

    /// Return the currently focused stream index.
    pub fn focus(&self) -> usize {
        self.focus
    }

    /// Compute the weighted fusion of multiple sensory vectors.
    pub fn fuse(&self, streams: &[Vec<f64>]) -> PerceptionResult<Vec<f64>> {
        if streams.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        let dim = streams[0].len();
        if streams.iter().any(|s| s.len() != dim) {
            return Err(PerceptionError::InvalidConfiguration(
                "all stream vectors must have the same dimension".into(),
            ));
        }
        if streams.len() != self.weights.len() {
            return Err(PerceptionError::InvalidConfiguration(format!(
                "expected {} streams, got {}",
                self.weights.len(),
                streams.len()
            )));
        }
        let mut result = vec![0.0; dim];
        for (i, stream) in streams.iter().enumerate() {
            let w = self.weights[i];
            for j in 0..dim {
                result[j] += w * stream[j];
            }
        }
        Ok(result)
    }

    /// Record the current weights to attention history for tracking.
    pub fn snapshot(&mut self) {
        self.attention_history.push(self.weights.clone());
        if self.attention_history.len() > 1000 {
            self.attention_history.remove(0);
        }
    }

    /// Return the number of recorded attention snapshots.
    pub fn history_len(&self) -> usize {
        self.attention_history.len()
    }

    /// Return the weight of the most attended stream.
    pub fn max_weight(&self) -> f64 {
        *self.weights.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or(&0.0)
    }

    /// Return the weight of the least attended stream.
    pub fn min_weight(&self) -> f64 {
        *self.weights.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or(&0.0)
    }

    /// Validate all parameters are within valid ranges.
    pub fn validate(&self) -> PerceptionResult<()> {
        let total: f64 = self.weights.iter().sum();
        if (total - 1.0).abs() > 0.01 && self.weights.len() > 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "weights do not sum to approximately 1.0".into(),
            ));
        }
        if self.focus >= self.weights.len() {
            return Err(PerceptionError::InvalidConfiguration(
                "focus index out of range".into(),
            ));
        }
        Ok(())
    }

    /// Reset attention to uniform distribution.
    pub fn reset(&mut self) {
        let n = self.weights.len();
        if n == 0 {
            return;
        }
        let uniform = 1.0 / n as f64;
        for w in &mut self.weights {
            *w = uniform;
        }
        self.focus = 0;
    }
}