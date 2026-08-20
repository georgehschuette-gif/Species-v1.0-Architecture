// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::HashMap;
use std::fmt;

use crate::MemoryError;

use super::*;

/// LatentPatternDiscovery: Statistical engine for discovering patterns in data streams.
///
/// Uses a sliding observation window and histogram-based frequency analysis
/// to detect recurring patterns that exceed chance expectation. Discovered
/// patterns are scored by significance and normalized for comparison.
#[derive(Debug, Clone, PartialEq)]
pub struct LatentPatternDiscovery {
    /// Observed frequency histogram: pattern_key -> count.
    pub observed_histogram: HashMap<String, usize>,
    /// Total number of observations processed.
    pub observation_count: usize,
    /// Size of the sliding observation window.
    pub window_size: usize,
    /// Significance threshold for pattern detection.
    pub significance_threshold: f64,
    /// Mean observation frequency used as null baseline.
    pub baseline_frequency: f64,
    /// Number of unique patterns discovered so far.
    pub discovery_count: usize,
}

impl LatentPatternDiscovery {
    /// Minimum valid window size.
    pub const MIN_WINDOW: usize = 1;
    /// Maximum valid window size.
    pub const MAX_WINDOW: usize = 1_000_000;

    /// Creates a new LatentPatternDiscovery with the given window size.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::DimensionMismatch`] if window size exceeds
    /// [`MAX_WINDOW`].
    pub fn new(window_size: usize) -> Result<Self, MemoryError> {
        if window_size == 0 || window_size > Self::MAX_WINDOW {
            return Err(MemoryError::DimensionMismatch {
                expected: Self::MIN_WINDOW,
                actual: window_size,
            });
        }
        Ok(Self {
            observed_histogram: HashMap::new(),
            observation_count: 0,
            window_size,
            significance_threshold: DEFAULT_SIGNIFICANCE_THRESHOLD,
            baseline_frequency: 0.0,
            discovery_count: 0,
        })
    }

    /// Observes a batch of data points, updating the histogram.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::MissingInput`] if the data slice is empty.
    pub fn observe(&mut self, data: &[f64]) -> Result<(), MemoryError> {
        if data.is_empty() {
            return Err(MemoryError::MissingInput("observation data must not be empty".into()));
        }
        for value in data {
            let key = format!("{:.4}", value);
            *self.observed_histogram.entry(key).or_insert(0) += 1;
            self.observation_count += 1;
            if self.observation_count > self.window_size {
                let overflow = self.observation_count - self.window_size;
                if let Some(old_key) = data.get(overflow % data.len()) {
                    let old_str = format!("{:.4}", old_key);
                    if let Some(count) = self.observed_histogram.get_mut(&old_str) {
                        *count = count.saturating_sub(1);
                        if *count == 0 {
                            self.observed_histogram.remove(&old_str);
                        }
                    }
                }
            }
        }
        self.recompute_baseline();
        Ok(())
    }

    /// Discovers statistically significant patterns in the current histogram.
    ///
    /// Returns a list of pattern keys whose frequency exceeds the significance threshold.
    pub fn discover_patterns(&mut self) -> Vec<String> {
        let mut discovered = Vec::new();
        if self.baseline_frequency <= 0.0 {
            return discovered;
        }
        for (key, &count) in &self.observed_histogram {
            if count as f64 > self.baseline_frequency * (1.0 + 1.0 / self.significance_threshold) {
                discovered.push(key.clone());
            }
        }
        self.discovery_count = discovered.len();
        discovered
    }

    /// Computes the significance score of a specific pattern key.
    pub fn pattern_score(&self, pattern_key: &str) -> f64 {
        let key = format!("{:.4}", pattern_key);
        let count = self.observed_histogram.get(&key).copied().unwrap_or(0) as f64;
        if self.baseline_frequency <= 0.0 {
            return 0.0;
        }
        (count / self.baseline_frequency).clamp(0.0, 1.0)
    }

    /// Normalizes the histogram scores to [0.0, 1.0].
    pub fn normalize(&mut self) {
        let max_count = self.observed_histogram.values().max().copied().unwrap_or(0);
        if max_count == 0 {
            return;
        }
        for count in self.observed_histogram.values_mut() {
            *count = (*count * 1000 / max_count).max(1);
        }
    }

    /// Clears the discovery state, resetting for a new batch.
    pub fn reset(&mut self) {
        self.observed_histogram.clear();
        self.observation_count = 0;
        self.discovery_count = 0;
        self.baseline_frequency = 0.0;
    }

    /// Recomputes the baseline frequency from histogram data.
    fn recompute_baseline(&mut self) {
        let total: usize = self.observed_histogram.values().sum();
        let distinct = self.observed_histogram.len();
        self.baseline_frequency = if distinct > 0 {
            total as f64 / distinct as f64
        } else {
            0.0
        };
    }

    /// Validates the discovery state.
    pub fn validate(&self) -> Result<(), MemoryError> {
        Self::new(self.window_size)?;
        if !(0.0..=1.0).contains(&self.significance_threshold) {
            return Err(MemoryError::OutOfRange {
                field: "significance_threshold".into(),
                value: self.significance_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}
