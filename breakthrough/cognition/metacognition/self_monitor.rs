// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// SelfMonitor: Observes and reports on cognitive process quality.
///
/// Self-monitoring tracks the health and quality of ongoing
/// cognitive operations. It detects anomalies, tracks
/// performance metrics, and issues alerts when quality
/// degrades below acceptable thresholds.
///
/// # Fields
/// - `monitoring_depth`: Number of nested monitoring levels, in [0, 10].
/// - `alert_threshold`: Quality score below which alerts fire, in [0.0, 1.0].
///
/// # Example
/// ```
/// use breakthrough::cognition::metacognition::SelfMonitor;
///
/// let mut monitor = SelfMonitor::new(2, 0.3).expect("valid parameters");
/// let healthy = monitor.check_quality(0.5).expect("check succeeded");
/// assert!(!healthy);
/// ```
pub struct SelfMonitor {
    /// Number of nested monitoring levels, in [0, 10].
    pub monitoring_depth: usize,
    /// Quality score below which alerts fire, in [0.0, 1.0].
    pub alert_threshold: f64,
    /// Internal count of alerts raised so far.
    alert_count: usize,
}

impl SelfMonitor {
    /// Creates a new `SelfMonitor` with the given depth and threshold.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `monitoring_depth` exceeds 10
    /// or `alert_threshold` is outside [0.0, 1.0].
    pub fn new(monitoring_depth: usize, alert_threshold: f64) -> Result<Self, CognitionError> {
        if monitoring_depth > 10 {
            return Err(CognitionError::OutOfRange {
                field: "monitoring_depth".to_string(),
                value: monitoring_depth as f64,
                min: 0.0,
                max: 10.0,
            });
        }
        if !(0.0..=1.0).contains(&alert_threshold) {
            return Err(CognitionError::OutOfRange {
                field: "alert_threshold".to_string(),
                value: alert_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            monitoring_depth,
            alert_threshold,
            alert_count: 0,
        })
    }

    /// Checks a quality score against the alert threshold.
    ///
    /// Returns `true` if quality is below threshold (alert condition).
    /// Increments the internal alert counter if quality is below threshold.
    pub fn check_quality(&mut self, quality_score: f64) -> Result<bool, CognitionError> {
        if !(0.0..=1.0).contains(&quality_score) {
            return Err(CognitionError::OutOfRange {
                field: "quality_score".to_string(),
                value: quality_score,
                min: 0.0,
                max: 1.0,
            });
        }
        let alert = quality_score < self.alert_threshold;
        if alert {
            self.alert_count += 1;
        }
        Ok(alert)
    }

    /// Returns the total number of alerts raised.
    pub fn alert_count(&self) -> usize {
        self.alert_count
    }

    /// Resets the alert counter to zero.
    pub fn reset_alerts(&mut self) {
        self.alert_count = 0;
    }

    /// Adjusts the alert threshold.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if the new threshold is outside [0.0, 1.0].
    pub fn set_threshold(&mut self, new_threshold: f64) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&new_threshold) {
            return Err(CognitionError::OutOfRange {
                field: "alert_threshold".to_string(),
                value: new_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        self.alert_threshold = new_threshold;
        Ok(())
    }

    /// Deepens the monitoring by one level.
    ///
    /// # Errors
    /// Returns [`CognitionError::CapacityExceeded`] if depth exceeds 10.
    pub fn deepen(&mut self) -> Result<(), CognitionError> {
        if self.monitoring_depth >= 10 {
            return Err(CognitionError::CapacityExceeded {
                max: 10,
                attempted: self.monitoring_depth + 1,
            });
        }
        self.monitoring_depth += 1;
        Ok(())
    }

    /// Returns the current monitoring depth.
    pub fn depth(&self) -> usize {
        self.monitoring_depth
    }

    /// Validates all field values are within valid ranges.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if self.monitoring_depth > 10 {
            return Err(CognitionError::OutOfRange {
                field: "monitoring_depth".to_string(),
                value: self.monitoring_depth as f64,
                min: 0.0,
                max: 10.0,
            });
        }
        if !(0.0..=1.0).contains(&self.alert_threshold) {
            return Err(CognitionError::OutOfRange {
                field: "alert_threshold".to_string(),
                value: self.alert_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}

impl fmt::Debug for SelfMonitor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SelfMonitor")
            .field("monitoring_depth", &self.monitoring_depth)
            .field("alert_threshold", &self.alert_threshold)
            .field("alert_count", &self.alert_count)
            .finish()
    }
}