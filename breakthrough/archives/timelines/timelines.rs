// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TimelineError {
    EmptyName,
    InvertedRange,
    OverflowDuration,
}

impl fmt::Display for TimelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "timeline name must not be empty"),
            Self::InvertedRange => write!(f, "start year must precede end year"),
            Self::OverflowDuration => write!(f, "timeline duration overflows u64"),
        }
    }
}

impl std::error::Error for TimelineError {}

pub struct Timeline {
    pub id: u64,
    pub name: String,
    pub events: Vec<String>,
    pub start_year: i32,
    pub end_year: i32,
}

impl Timeline {
    pub fn new(
        id: u64,
        name: String,
        events: Vec<String>,
        start_year: i32,
        end_year: i32,
    ) -> Result<Self, TimelineError> {
        if name.trim().is_empty() {
            return Err(TimelineError::EmptyName);
        }
        if start_year >= end_year {
            return Err(TimelineError::InvertedRange);
        }
        Ok(Self {
            id,
            name,
            events,
            start_year,
            end_year,
        })
    }

    pub fn duration(&self) -> u64 {
        (self.end_year - self.start_year) as u64
    }

    pub fn add_event(&mut self, event: String) {
        self.events.push(event);
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn is_ancient(&self) -> bool {
        self.start_year < -5000
    }
}

impl fmt::Display for Timeline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Timeline(id={}, name={}, years={}-{}, events={})",
            self.id,
            self.name,
            self.start_year,
            self.end_year,
            self.event_count()
        )
    }
}
