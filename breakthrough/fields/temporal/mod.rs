// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Temporal Field: Time-indexed cognitive state distributions.
//! Encodes recency effects, forecasting models, temporal distance metrics,
//! and event ordering across cognitive time.
//!
//! The temporal field module provides tools for modeling how cognitive states
//! change over time, including recency-weighted relevance, future predictions,
//! and distance measures between temporal events.

pub mod recency;
pub mod forecasting;
pub mod distance;

pub use recency::TemporalRecency;
pub use forecasting::{TemporalForecasting, ForecastingModel};
pub use distance::{TemporalDistance, TemporalMetric};

use std::fmt;

/// TemporalField: A time-indexed container for temporal cognitive data.
#[derive(Debug, Clone, PartialEq)]
pub struct TemporalField {
    pub name: String,
    pub timestamps: Vec<f64>,
    pub values: Vec<f64>,
}

impl TemporalField {
    pub fn new(name: String) -> Self {
        Self { name, timestamps: Vec::new(), values: Vec::new() }
    }

    pub fn add_observation(&mut self, timestamp: f64, value: f64) {
        self.timestamps.push(timestamp);
        self.values.push(value);
    }

    pub fn latest_value(&self) -> Option<f64> {
        self.values.last().copied()
    }

    pub fn recency_at(&self, current_time: f64) -> Vec<f64> {
        self.timestamps.iter().map(|&t| {
            let age = current_time - t;
            if age <= 0.0 { 1.0 } else { 0.5f64.powf(age / 10.0) }
        }).collect()
    }

    pub fn observation_count(&self) -> usize {
        self.timestamps.len()
    }

    pub fn time_span(&self) -> f64 {
        if self.timestamps.len() < 2 {
            return 0.0;
        }
        self.timestamps.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
            - self.timestamps.iter().cloned().fold(f64::INFINITY, f64::min)
    }
}

impl Default for TemporalField {
    fn default() -> Self {
        Self::new(String::from("unnamed"))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TemporalError {
    InvalidTimestamp { timestamp: f64 },
    InvalidHalfLife { half_life: f64 },
    InvalidHorizon { horizon: f64 },
    InvalidVariance { variance: f64 },
    InvalidMetric,
    InsufficientData,
    ComputationError(String),
}

impl fmt::Display for TemporalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemporalError::InvalidTimestamp { timestamp } => write!(f, "invalid timestamp: {}", timestamp),
            TemporalError::InvalidHalfLife { half_life } => write!(f, "invalid half-life: {}", half_life),
            TemporalError::InvalidHorizon { horizon } => write!(f, "invalid horizon: {}", horizon),
            TemporalError::InvalidVariance { variance } => write!(f, "invalid variance: {}", variance),
            TemporalError::InvalidMetric => write!(f, "invalid temporal metric"),
            TemporalError::InsufficientData => write!(f, "insufficient data for operation"),
            TemporalError::ComputationError(msg) => write!(f, "computation error: {}", msg),
        }
    }
}

impl std::error::Error for TemporalError {}

pub type TemporalResult<T> = Result<T, TemporalError>;
