// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Reflection {
    pub id: String,
    pub subject: String,
    pub timestamp_ms: u64,
    pub observations: Vec<Observation>,
    pub insights: Vec<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Observation {
    pub phenomenon: String,
    pub details: String,
    pub severity: Severity,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl Reflection {
    pub fn new<S: Into<String>>(id: S, subject: S) -> Self {
        Self {
            id: id.into(),
            subject: subject.into(),
            timestamp_ms: 0,
            observations: Vec::new(),
            insights: Vec::new(),
            confidence: 0.0,
        }
    }

    pub fn with_timestamp(mut self, timestamp_ms: u64) -> Self {
        self.timestamp_ms = timestamp_ms;
        self
    }

    pub fn add_observation(&mut self, observation: Observation) {
        self.observations.push(observation);
    }

    pub fn add_insight<S: Into<String>>(&mut self, insight: S) {
        self.insights.push(insight.into());
    }

    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    pub fn observation_count(&self) -> usize {
        self.observations.len()
    }

    pub fn insight_count(&self) -> usize {
        self.insights.len()
    }
}

impl Observation {
    pub fn new<S: Into<String>>(phenomenon: S, details: S, severity: Severity) -> Self {
        Self {
            phenomenon: phenomenon.into(),
            details: details.into(),
            severity,
            timestamp_ms: 0,
        }
    }

    pub fn with_timestamp(mut self, timestamp_ms: u64) -> Self {
        self.timestamp_ms = timestamp_ms;
        self
    }
}

impl fmt::Display for Reflection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Reflection {} on {} ({} observations)", self.id, self.subject, self.observation_count())
    }
}

impl fmt::Display for Observation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} ({:?})", self.phenomenon, self.details, self.severity)
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Severity::Low => write!(f, "low"),
            Severity::Medium => write!(f, "medium"),
            Severity::High => write!(f, "high"),
            Severity::Critical => write!(f, "critical"),
        }
    }
}

