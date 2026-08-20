// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Attractors Module
//!
//! This module provides a comprehensive taxonomy and analysis framework for
//! dynamical systems attractors. It classifies attractors into six fundamental
//! categories based on their long-term behavioral characteristics:
//!
//! - **Stable**: Attractors that converge asymptotically to a fixed point or
//!   periodic orbit from an open neighborhood of initial conditions.
//! - **Unstable**: Repellors, saddle points, and explosive divergences that
//!   push trajectories away from their neighborhood.
//! - **Transient**: Non-persistent structures including pulses, damped waves,
//!   and travelling waves that decay or propagate without settling.
//! - **Recursive**: Self-referential structures, feedback loops, and nested
//!   hierarchies where the system encodes its own transformation rules.
//! - **Emergent**: Spontaneously ordered structures, phase transitions, and
//!   critical points arising from collective interactions.
//! - **Dormant**: Latent attractors, potential wells, and sleeping states that
//!   exist below observable thresholds awaiting activation.
//!
//! Each submodule exposes concrete types, stability classification methods,
//! numerical integration helpers, and phase-space analysis utilities.

pub mod stable;
pub mod unstable;
pub mod transient;
pub mod recursive;
pub mod emergent;
pub mod dormant;

pub use stable::{AttractorPoint, Equilibrium, LimitCycle, Stability};
pub use unstable::{Repellor, SaddlePoint, ExplosiveAttractor};
pub use transient::{TransientPulse, DampedWave, TravellingWave};
pub use recursive::{SelfReferential, FeedbackLoop, NestedAttractor};
pub use emergent::{SpontaneousAttractor, PhaseTransition, CriticalPoint};
pub use dormant::{LatentAttractor, PotentialWell, SleepingAttractor};

use std::fmt::{self, Display, Formatter};

/// Top-level classification of an attractor's dynamical role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttractorType {
    /// Asymptotically stable fixed point or limit cycle.
    Stable,
    /// Divergent or repelling structure.
    Unstable,
    /// Non-persistent, time-limited structure.
    Transient,
    /// Self-referential or recursively defined structure.
    Recursive,
    /// Spontaneously arising collective order.
    Emergent,
    /// Below-threshold, awaiting activation.
    Dormant,
}

impl Display for AttractorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Stable => "stable",
            Self::Unstable => "unstable",
            Self::Transient => "transient",
            Self::Recursive => "recursive",
            Self::Emergent => "emergent",
            Self::Dormant => "dormant",
        })
    }
}

/// Runtime state metadata for any attractor instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttractorState {
    /// Attractor is active and influencing trajectories.
    Active,
    /// Attractor exists but is not currently dominant.
    Latent,
    /// Attractor is decaying or transitioning.
    Decaying,
    /// Attractor has collapsed or lost stability.
    Collapsed,
    /// Attractor is forming but not yet stable.
    Forming,
}

impl Display for AttractorState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Active => "active",
            Self::Latent => "latent",
            Self::Decaying => "decaying",
            Self::Collapsed => "collapsed",
            Self::Forming => "forming",
        })
    }
}

/// Common interface for all attractor types in the module.
pub trait Attractor {
    fn attractor_type(&self) -> AttractorType;
    fn state(&self) -> AttractorState;
    fn is_stable(&self) -> bool;
    fn dimension(&self) -> usize;
    fn describe(&self) -> &'static str;
}
