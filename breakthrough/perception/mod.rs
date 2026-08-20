// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Perception: The processes by which the ecosystem acquires and interprets
//! information from its environment.
//!
//! This module transforms raw sensory data into meaningful cognitive
//! representations through extraction, alignment, fusion, and abstraction.
//!
//! # Architecture
//!
//! The perception stack is organized into six coordinated layers:
//!
//! 1. **Sensory Streams** — raw intake from visual, auditory, tactile, and proprioceptive channels.
//! 2. **Semantic Extraction** — conversion of raw signals into symbols, relations, and context.
//! 3. **Temporal Alignment** — clock synchronization, event ordering, and windowed processing.
//! 4. **Uncertainty Mapping** — confidence tracking, noise modeling, and probabilistic interpretation.
//! 5. **Signal Fusion** — Kalman, Bayesian, and attention-based integration of multi-modal signals.
//! 6. **Anomaly Detection** — threshold-based, reconstruction-based, and clustering-based novelty detection.
//! 7. **Compression** — lossless, lossy, and latent representation compression.
//! 8. **Abstraction** — category formation, prototype generation, and hierarchical concept lifting.

pub mod sensory_streams;
pub mod semantic_extraction;
pub mod temporal_alignment;
pub mod uncertainty_mapping;
pub mod signal_fusion;
pub mod anomaly_detection;
pub mod compression;
pub mod abstraction;

pub use sensory_streams::{VisualStream, AuditoryStream, TactileStream, ProprioceptiveStream};
pub use semantic_extraction::{SymbolExtractor, RelationExtractor, Contextualizer};
pub use temporal_alignment::{ClockSynchronizer, EventOrdering, TemporalWindow};
pub use uncertainty_mapping::{ConfidenceMap, NoiseModel, ProbabilisticMap};
pub use signal_fusion::{KalmanFusion, BayesianFusion, AttentionMechanism};
pub use anomaly_detection::{ThresholdDetector, ReconstructionDetector, ClusteringDetector};
pub use compression::{LosslessCompressor, LossyCompressor, LatentCompressor};
pub use abstraction::{CategoryFormation, PrototypeFormation, AbstractionHierarchy};

/// Unified result type for all perception operations.
pub type PerceptionResult<T> = Result<T, PerceptionError>;

/// Comprehensive error type for perception-pipeline failures.
#[derive(Debug)]
pub enum PerceptionError {
    /// Configuration or parameter validation failed.
    InvalidConfiguration(String),
    /// The operation received insufficient input data.
    InsufficientData,
    /// The requested operation is not supported by the current component.
    UnsupportedOperation,
    /// A numerical computation failed (e.g., singular matrix, overflow).
    ComputationError(String),
    /// A timeout or synchronization bound was exceeded.
    SynchronizationTimeout,
    /// An anomaly was detected outside acceptable bounds.
    OutOfBounds(String),
    /// A capacity limit was reached during an operation.
    CapacityExceeded { max: usize, attempted: usize },
}

impl std::fmt::Display for PerceptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidConfiguration(msg) => write!(f, "invalid configuration: {}", msg),
            Self::InsufficientData => write!(f, "insufficient data for operation"),
            Self::UnsupportedOperation => write!(f, "operation not supported"),
            Self::ComputationError(msg) => write!(f, "computation error: {}", msg),
            Self::SynchronizationTimeout => write!(f, "synchronization timeout exceeded"),
            Self::OutOfBounds(msg) => write!(f, "value out of bounds: {}", msg),
            Self::CapacityExceeded { max, attempted } => write!(f, "capacity exceeded: max {}, attempted {}", max, attempted),
        }
    }
}

impl std::error::Error for PerceptionError {}
