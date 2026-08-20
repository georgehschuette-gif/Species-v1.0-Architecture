// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Sensory Streams: Raw input channels from the environment.
//! Defines how external signals enter the cognitive ecosystem.
//!
//! Each stream ingests a raw signal, validates geometry, applies
//! optional filtering, and produces a structured output vector.

pub mod visual;
pub mod auditory;
pub mod tactile;
pub mod proprioceptive;

pub use visual::VisualStream;
pub use auditory::AuditoryStream;
pub use tactile::TactileStream;
pub use proprioceptive::ProprioceptiveStream;

pub use super::PerceptionResult;
pub use super::PerceptionError;

/// Source channel identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SensoryChannel {
    Visual = 0,
    Auditory = 1,
    Tactile = 2,
    Proprioceptive = 3,
}

impl TryFrom<usize> for SensoryChannel {
    type Error = PerceptionError;
    fn try_from(value: usize) -> PerceptionResult<Self> {
        match value {
            0 => Ok(Self::Visual),
            1 => Ok(Self::Auditory),
            2 => Ok(Self::Tactile),
            3 => Ok(Self::Proprioceptive),
            _ => Err(PerceptionError::InvalidConfiguration(format!(
                "unknown sensory channel index: {}",
                value
            ))),
        }
    }
}

/// Stream-level failure modes.
#[derive(Debug)]
pub enum StreamError {
    InvalidResolution,
    InvalidFrameRate,
    InvalidSampleRate,
    InvalidChannels,
    InvalidSensitivity,
    InvalidDimensions,
    InvalidUpdateRate,
    EmptySignal,
    OutOfRange(String),
    Unsupported,
}

impl std::fmt::Display for StreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidResolution => write!(f, "invalid resolution"),
            Self::InvalidFrameRate => write!(f, "invalid frame rate"),
            Self::InvalidSampleRate => write!(f, "invalid sample rate"),
            Self::InvalidChannels => write!(f, "invalid channel count"),
            Self::InvalidSensitivity => write!(f, "invalid sensitivity"),
            Self::InvalidDimensions => write!(f, "invalid dimensions"),
            Self::InvalidUpdateRate => write!(f, "invalid update rate"),
            Self::EmptySignal => write!(f, "empty signal"),
            Self::OutOfRange(msg) => write!(f, "out of range: {}", msg),
            Self::Unsupported => write!(f, "operation unsupported for this stream"),
        }
    }
}

impl std::error::Error for StreamError {}

impl From<StreamError> for PerceptionError {
    fn from(err: StreamError) -> Self {
        PerceptionError::InvalidConfiguration(err.to_string())
    }
}

/// Common configuration for sensory streams.
#[derive(Debug, Clone)]
pub struct StreamConfig {
    pub channel: SensoryChannel,
    pub resolution: usize,
    pub sample_rate: f64,
    pub buffer_size: usize,
    pub enable_filtering: bool,
}

impl StreamConfig {
    /// Construct a validated stream configuration.
    pub fn new(channel: SensoryChannel, sample_rate: f64, buffer_size: usize) -> PerceptionResult<Self> {
        if sample_rate <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "sample rate must be positive".into(),
            ));
        }
        if buffer_size == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "buffer size must be non-zero".into(),
            ));
        }
        Ok(Self {
            channel,
            resolution: 0,
            sample_rate,
            buffer_size,
            enable_filtering: true,
        })
    }

    /// Validate configuration parameters.
    pub fn validate(&self) -> PerceptionResult<()> {
        if self.sample_rate <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "sample rate must be positive".into(),
            ));
        }
        if self.buffer_size == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "buffer size must be non-zero".into(),
            ));
        }
        Ok(())
    }
}

/// Trait implemented by all sensory-stream processors.
pub trait SensoryStream {
    /// Apply configuration to the stream.
    fn configure(&mut self, config: StreamConfig) -> PerceptionResult<()>;

    /// Ingest a raw signal and return a processed feature vector.
    fn ingest(&mut self, raw_signal: &[f64]) -> PerceptionResult<Vec<f64>>;

    /// Validate that the stream is ready for ingestion.
    fn validate(&self) -> PerceptionResult<()>;

    /// Expected block size (number of samples) per ingestion call.
    fn block_size(&self) -> usize;

    /// Whether the stream is currently active.
    fn active(&self) -> bool;

    /// Reset internal state without destroying configuration.
    fn reset(&mut self);
}
