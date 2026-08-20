// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum HistoricalStateError {
    InvalidEntropy(f64),
    EmptyLabel,
    NegativeTimestamp,
}

impl fmt::Display for HistoricalStateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEntropy(value) => write!(f, "invalid entropy value: {}", value),
            Self::EmptyLabel => write!(f, "state label must not be empty"),
            Self::NegativeTimestamp => write!(f, "timestamp must not be negative"),
        }
    }
}

impl std::error::Error for HistoricalStateError {}

pub struct HistoricalState {
    pub id: u64,
    pub label: String,
    pub timestamp: u64,
    pub entropy: f64,
    pub preserved_data: Vec<u8>,
}

impl HistoricalState {
    pub fn new(
        id: u64,
        label: String,
        timestamp: u64,
        entropy: f64,
        preserved_data: Vec<u8>,
    ) -> Result<Self, HistoricalStateError> {
        if !(0.0..=1.0).contains(&entropy) {
            return Err(HistoricalStateError::InvalidEntropy(entropy));
        }
        if label.trim().is_empty() {
            return Err(HistoricalStateError::EmptyLabel);
        }
        Ok(Self {
            id,
            label,
            timestamp,
            entropy,
            preserved_data,
        })
    }

    pub fn integrity(&self) -> f64 {
        1.0 - self.entropy
    }

    pub fn data_size(&self) -> usize {
        self.preserved_data.len()
    }

    pub fn is_well_preserved(&self) -> bool {
        self.entropy < 0.1
    }
}

impl fmt::Display for HistoricalState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HistoricalState(id={}, label={}, timestamp={}, integrity={:.2})",
            self.id,
            self.label,
            self.timestamp,
            self.integrity()
        )
    }
}
