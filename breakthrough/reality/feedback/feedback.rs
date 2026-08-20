// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::VecDeque;
use std::fmt;

use crate::{AdaptationRate, RealityError};

/// LearningSignal: A signal that drives learning and adaptation.
///
/// Learning signals encode the feedback from the environment that
/// guides model improvement. They can be positive (reward) or
/// negative (punishment/error).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LearningSignal {
    /// The signal value (positive = reward, negative = punishment).
    pub value: f64,
    /// Confidence in this signal.
    pub confidence: f64,
    /// Source of this signal.
    pub source: SignalSource,
    /// Timestamp of the signal.
    pub timestamp: f64,
}

impl LearningSignal {
    /// Minimum valid signal value.
    pub const MIN_VALUE: f64 = -1.0;
    /// Maximum valid signal value.
    pub const MAX_VALUE: f64 = 1.0;

    /// Creates a new learning signal.
    ///
    /// # Errors
    /// Returns `RealityError::OutOfRange` if value is outside [-1.0, 1.0].
    pub fn new(
        value: f64,
        confidence: f64,
        source: SignalSource,
        timestamp: f64,
    ) -> Result<Self, RealityError> {
        if value < Self::MIN_VALUE || value > Self::MAX_VALUE {
            return Err(RealityError::OutOfRange {
                field: "value".to_string(),
                value,
                min: Self::MIN_VALUE,
                max: Self::MAX_VALUE,
            });
        }
        if confidence < 0.0 || confidence > 1.0 {
            return Err(RealityError::OutOfRange {
                field: "confidence".to_string(),
                value: confidence,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            value,
            confidence,
            source,
            timestamp,
        })
    }

    /// Returns whether this is a positive (reward) signal.
    pub fn is_positive(&self) -> bool {
        self.value > 0.0
    }

    /// Returns whether this is a negative (punishment) signal.
    pub fn is_negative(&self) -> bool {
        self.value < 0.0
    }

    /// Returns the weighted signal value (value * confidence).
    pub fn weighted_value(&self) -> f64 {
        self.value * self.confidence
    }
}

/// SignalSource: The origin of a learning signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SignalSource {
    /// Signal from direct environmental feedback.
    Environment,
    /// Signal from internal model evaluation.
    Internal,
    /// Signal from another agent.
    Social,
    /// Signal from intrinsic motivation.
    Intrinsic,
}

impl SignalSource {
    /// Returns the string label of this source.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Environment => "environment",
            Self::Internal => "internal",
            Self::Social => "social",
            Self::Intrinsic => "intrinsic",
        }
    }

    /// Returns the base reliability of this source.
    pub fn base_reliability(&self) -> f64 {
        match self {
            Self::Environment => 0.9,
            Self::Internal => 0.7,
            Self::Social => 0.6,
            Self::Intrinsic => 0.5,
        }
    }
}

impl std::fmt::Display for SignalSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for SignalSource {
    fn default() -> Self {
        Self::Environment
    }
}

/// Feedback: Learning signals collected over time.
///
/// Feedback aggregates learning signals to drive adaptation
/// and model improvement.
#[derive(Debug, Clone, PartialEq)]
pub struct Feedback {
    /// Unique identifier for this feedback record.
    pub id: String,
    /// Recent learning signals.
    pub signals: VecDeque<LearningSignal>,
    /// Current adaptation rate.
    pub adaptation_rate: AdaptationRate,
    /// Total number of signals received.
    pub total_signals: usize,
    /// Whether feedback is currently being processed.
    pub processing: bool,
}

impl Feedback {
    /// Maximum number of signals to retain.
    pub const MAX_SIGNALS: usize = 1000;

    /// Creates a new feedback collector.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            signals: VecDeque::with_capacity(Self::MAX_SIGNALS),
            adaptation_rate: AdaptationRate::default(),
            total_signals: 0,
            processing: false,
        }
    }

    /// Adds a learning signal.
    ///
    /// # Errors
    /// Returns `RealityError::CapacityExceeded` if max signals exceeded.
    pub fn add_signal(&mut self, signal: LearningSignal) -> Result<(), RealityError> {
        if self.signals.len() >= Self::MAX_SIGNALS {
            return Err(RealityError::CapacityExceeded {
                max: Self::MAX_SIGNALS,
                attempted: self.signals.len() + 1,
            });
        }
        self.signals.push_back(signal);
        self.total_signals += 1;
        Ok(())
    }

    /// Returns the average signal value.
    pub fn average_signal(&self) -> f64 {
        if self.signals.is_empty() {
            return 0.0;
        }
        self.signals.iter().map(|s| s.value).sum::<f64>() / self.signals.len() as f64
    }

    /// Returns the net learning signal (weighted by confidence).
    pub fn net_signal(&self) -> f64 {
        if self.signals.is_empty() {
            return 0.0;
        }
        self.signals.iter().map(|s| s.weighted_value()).sum::<f64>() / self.signals.len() as f64
    }

    /// Updates the adaptation rate based on recent signals.
    pub fn update_adaptation_rate(&mut self) {
        let net = self.net_signal();
        let magnitude = net.abs();
        let new_rate = (self.adaptation_rate.0 + magnitude * 0.1).min(1.0);
        self.adaptation_rate = AdaptationRate(new_rate);
    }

    /// Clears old signals.
    pub fn prune_old_signals(&mut self, max_age: f64, now: f64) {
        self.signals.retain(|s| now - s.timestamp <= max_age);
    }
}

