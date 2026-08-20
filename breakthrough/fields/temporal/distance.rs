// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// TemporalMetric: The metric used to measure temporal distance between events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemporalMetric {
    Absolute,
    Relative,
    LogScaled,
}

impl TemporalMetric {
    pub fn as_str(&self) -> &'static str {
        match self {
            TemporalMetric::Absolute => "absolute",
            TemporalMetric::Relative => "relative",
            TemporalMetric::LogScaled => "log_scaled",
        }
    }

    pub fn is_scaled(&self) -> bool {
        !matches!(self, TemporalMetric::Absolute)
    }
}

/// TemporalDistance: Separation between events in cognitive time.
pub struct TemporalDistance {
    pub event_a: f64,
    pub event_b: f64,
    pub metric: TemporalMetric,
}

impl TemporalDistance {
    pub fn new(event_a: f64, event_b: f64, metric: TemporalMetric) -> Result<Self, TemporalError> {
        if !event_a.is_finite() || !event_b.is_finite() {
            return Err(TemporalError::InvalidTimestamp { timestamp: event_a.min(event_b) });
        }
        Ok(Self { event_a, event_b, metric })
    }

    pub fn measure(&self) -> f64 {
        let diff = (self.event_a - self.event_b).abs();
        match self.metric {
            TemporalMetric::Absolute => diff,
            TemporalMetric::Relative => {
                let max_t = self.event_a.max(self.event_b).max(1.0);
                diff / max_t
            }
            TemporalMetric::LogScaled => {
                if diff <= 0.0 { 0.0 } else { diff.ln() }
            }
        }
    }

    pub fn is_close(&self, threshold: f64) -> bool {
        self.measure() < threshold
    }

    pub fn metric_name(&self) -> &'static str {
        self.metric.as_str()
    }
}
