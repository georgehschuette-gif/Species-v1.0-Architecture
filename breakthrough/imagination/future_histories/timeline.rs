// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::ImaginationError;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimelineState {
    Deterministic,
    Probabilistic,
    Branching,
    Collapsed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimelineEvent {
    pub year: u64,
    pub description: String,
    pub probability: f64,
    pub impact: f64,
}

pub struct Timeline {
    pub name: String,
    pub events: Vec<TimelineEvent>,
    pub state: TimelineState,
    pub horizon: usize,
    pub current_year: u64,
    pub confidence: f64,
}

impl Timeline {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            events: Vec::new(),
            state: TimelineState::Deterministic,
            horizon: DEFAULT_TIMELINE_HORIZON,
            current_year: 0,
            confidence: 1.0,
        }
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn events(&self) -> &[TimelineEvent] { &self.events }
    pub fn state(&self) -> TimelineState { self.state }
    pub fn horizon(&self) -> usize { self.horizon }
    pub fn current_year(&self) -> u64 { self.current_year }
    pub fn confidence(&self) -> f64 { self.confidence }

    pub fn set_horizon(&mut self, horizon: usize) -> Result<(), ImaginationError> {
        if horizon == 0 {
            return Err(ImaginationError::InvalidInput("horizon must be non-zero".into()));
        }
        self.horizon = horizon;
        Ok(())
    }

    pub fn add_event(&mut self, year: u64, description: impl Into<String>, probability: f64, impact: f64) -> Result<(), ImaginationError> {
        if !(0.0..=1.0).contains(&probability) {
            return Err(ImaginationError::OutOfRange { field: "probability".into(), value: probability, min: 0.0, max: 1.0 });
        }
        if !(0.0..=1.0).contains(&impact) {
            return Err(ImaginationError::OutOfRange { field: "impact".into(), value: impact, min: 0.0, max: 1.0 });
        }
        if year < self.current_year {
            return Err(ImaginationError::InvalidInput("event year cannot be before current year".into()));
        }
        self.events.push(TimelineEvent { year, description: description.into(), probability, impact });
        self.events.sort_by_key(|e| e.year);
        self.recalculate_coherence();
        Ok(())
    }

    pub fn advance_year(&mut self, years: u64) -> Result<(), ImaginationError> {
        if years == 0 {
            return Err(ImaginationError::InvalidInput("years must be non-zero".into()));
        }
        self.current_year += years;
        self.events.retain(|e| e.year >= self.current_year);
        self.state = if self.events.len() > 1 { TimelineState::Branching } else { TimelineState::Deterministic };
        Ok(())
    }

    pub fn next_event(&self) -> Option<&TimelineEvent> {
        self.events.iter().find(|e| e.year >= self.current_year)
    }

    pub fn events_in_range(&self, start: u64, end: u64) -> Vec<&TimelineEvent> {
        self.events.iter().filter(|e| e.year >= start && e.year <= end).collect()
    }

    pub fn collapse_to_single(&mut self) -> Result<(), ImaginationError> {
        if self.events.is_empty() {
            return Err(ImaginationError::MissingInput("no events to collapse".into()));
        }
        self.events.truncate(1);
        self.state = TimelineState::Collapsed;
        self.confidence = 1.0;
        Ok(())
    }

    fn recalculate_coherence(&mut self) {
        if self.events.is_empty() {
            self.confidence = 1.0;
            return;
        }
        let avg_probability: f64 = self.events.iter().map(|e| e.probability).sum::<f64>() / self.events.len() as f64;
        self.confidence = avg_probability.clamp(0.0, 1.0);
        if self.confidence < 0.5 {
            self.state = TimelineState::Probabilistic;
        }
    }

    pub fn validate(&self) -> Result<(), ImaginationError> {
        if self.horizon == 0 {
            return Err(ImaginationError::InvalidInput("horizon must be non-zero".into()));
        }
        Ok(())
    }
}

impl fmt::Display for Timeline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Timeline(name='{}', events={}, state={:?}, year={})", self.name, self.events.len(), self.state, self.current_year)
    }
}

impl fmt::Display for TimelineState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Deterministic => write!(f, "Deterministic"),
            Self::Probabilistic => write!(f, "Probabilistic"),
            Self::Branching => write!(f, "Branching"),
            Self::Collapsed => write!(f, "Collapsed"),
        }
    }
}
