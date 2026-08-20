// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Temporal Alignment: Synchronizing signals across time.
//! Ensures that events are ordered and time-stamped correctly.

pub mod clock_sync;
pub mod event_ordering;
pub mod windowing;

pub use clock_sync::ClockSynchronizer;
pub use event_ordering::EventOrdering;
pub use windowing::TemporalWindow;

pub use super::PerceptionResult;
pub use super::PerceptionError;

/// Monotonic timestamp in nanoseconds.
pub type MonotonicTimestamp = u64;

/// A temporally-tagged event from any sensory channel.
#[derive(Debug, Clone)]
pub struct TemporalEvent {
    pub timestamp: MonotonicTimestamp,
    pub channel: usize,
    pub payload: Vec<u64>,
    pub priority: f64,
}

/// A lightweight event reference used in ordering and windowing APIs.
#[derive(Debug, Clone)]
pub struct Event {
    pub id: usize,
    pub timestamp: u64,
    pub payload: Vec<f64>,
    pub source_channel: usize,
}

impl TemporalEvent {
    pub fn new(timestamp: MonotonicTimestamp, channel: usize, payload: Vec<u64>) -> PerceptionResult<Self> {
        Ok(Self {
            timestamp,
            channel,
            payload,
            priority: 0.0,
        })
    }

    /// Lexicographic ordering by timestamp, then channel, then payload.
    pub fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp
            .cmp(&other.timestamp)
            .then_with(|| self.channel.cmp(&other.channel))
            .then_with(|| self.payload.cmp(&other.payload))
    }
}

impl PartialOrd for TemporalEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TemporalEvent {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

/// Error during temporal alignment.
#[derive(Debug)]
pub enum AlignmentError {
    ClockDriftExceeded,
    BufferOverflow,
    EmptyBuffer,
    OutOfOrder,
    InvalidWindow,
}

impl std::fmt::Display for AlignmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ClockDriftExceeded => write!(f, "clock drift exceeded tolerance"),
            Self::BufferOverflow => write!(f, "temporal buffer overflow"),
            Self::EmptyBuffer => write!(f, "temporal buffer is empty"),
            Self::OutOfOrder => write!(f, "events arrived out of order"),
            Self::InvalidWindow => write!(f, "temporal window configuration invalid"),
        }
    }
}

impl std::error::Error for AlignmentError {}

impl From<AlignmentError> for PerceptionError {
    fn from(err: AlignmentError) -> Self {
        PerceptionError::SynchronizationTimeout
    }
}
