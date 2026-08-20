// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum DynamicsError {
    InvalidFrequency(f64),
    InvalidAmplitude(f64),
    InvalidDamping(f64),
    UnstableState(String),
    InvalidFlowRate(f64),
    CascadeFailure(String),
    SyncError(String),
    TurbulenceOverflow,
    EquilibriumBreach,
    TransitionError(String),
    FeedbackInstability,
    StabilizationFailure(String),
}

impl fmt::Display for DynamicsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DynamicsError::InvalidFrequency(freq) => write!(f, "Invalid frequency: {}", freq),
            DynamicsError::InvalidAmplitude(amp) => write!(f, "Invalid amplitude: {}", amp),
            DynamicsError::InvalidDamping(damp) => write!(f, "Invalid damping: {}", damp),
            DynamicsError::UnstableState(msg) => write!(f, "Unstable state: {}", msg),
            DynamicsError::InvalidFlowRate(rate) => write!(f, "Invalid flow rate: {}", rate),
            DynamicsError::CascadeFailure(msg) => write!(f, "Cascade failure: {}", msg),
            DynamicsError::SyncError(msg) => write!(f, "Synchronization error: {}", msg),
            DynamicsError::TurbulenceOverflow => write!(f, "Turbulence overflow detected"),
            DynamicsError::EquilibriumBreach => write!(f, "Equilibrium breach"),
            DynamicsError::TransitionError(msg) => write!(f, "Phase transition error: {}", msg),
            DynamicsError::FeedbackInstability => write!(f, "Feedback instability detected"),
            DynamicsError::StabilizationFailure(msg) => write!(f, "Stabilization failure: {}", msg),
        }
    }
}

impl std::error::Error for DynamicsError {}

pub mod oscillations;
pub mod flows;
pub mod cascades;
pub mod synchronization;
pub mod turbulence;
pub mod equilibrium;
pub mod phase_transitions;
pub mod feedback_loops;
pub mod stabilization;

pub use oscillations::{Oscillator, DampedOscillator};
pub use flows::{MaterialFlow, InformationFlow};
pub use cascades::{Avalanche, ChainReaction};
pub use synchronization::{PhaseLocking, Consensus};
pub use turbulence::{ChaoticDynamics, Vortex};
pub use equilibrium::{StableState, MetastableState};
pub use phase_transitions::{Transition, OrderParameter};
pub use feedback_loops::{PositiveFeedback, NegativeFeedback};
pub use stabilization::{Stabilizer, Regulator};

