// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MetricId(u64);

impl MetricId {
    pub fn new(id: u64) -> Self {
        MetricId(id)
    }

    pub fn inner(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for MetricId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MetricId({})", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Metric {
    id: MetricId,
    name: String,
    value: f64,
    timestamp: u64,
}

impl Metric {
    pub fn new(id: MetricId, name: String, value: f64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Metric {
            id,
            name,
            value,
            timestamp,
        }
    }

    pub fn id(&self) -> MetricId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn update(&mut self, new_value: f64) {
        self.value = new_value;
        self.timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
    }

    pub fn validate(&self) -> crate::Result<()> {
        if self.name.trim().is_empty() {
            return Err(crate::EmergenceError::MeasurementError(
                "metric name cannot be empty".to_string(),
            ));
        }
        if self.value.is_nan() || self.value.is_infinite() {
            return Err(crate::EmergenceError::MeasurementError(
                format!("invalid metric value: {}", self.value),
            ));
        }
        Ok(())
    }
}

impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Metric(id={}, name={}, value={:.3}, ts={})",
            self.id, self.name, self.value, self.timestamp
        )
    }
}
