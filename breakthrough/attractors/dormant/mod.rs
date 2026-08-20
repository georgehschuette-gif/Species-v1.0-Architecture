// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Dormant Attractors
//!
//! Dormant attractors exist in a suppressed or metastable state, requiring
//! perturbation, threshold crossing, or sufficient time to become active.
//! They are relevant to metastability, quantum tunneling, biological
//! hibernation, and latent failure modes in engineered systems.
//!
//! ## Types
//!
//! - [`LatentAttractor`]: An attractor that is mathematically present but
//!   not yet dominant, separated by an activation barrier. Supports
//!   Arrhenius and Kramers rate theory, WKB tunneling probabilities, and
//!   thermal activation models.
//! - [`PotentialWell`]: A localized energy minimum with bound states and
//!   tunneling probabilities. Models harmonic, double-well, square,
//!   Lennard-Jones, and Morse potentials.
//! - [`SleepingAttractor`]: A deeply dormant state with probabilistic
//!   wake-up dynamics and cyclical dormancy patterns. Models light, deep,
//!   hibernation, and comatose sleep depths.
//!
//! ## Activation Criteria
//!
//! Dormant attractors are characterized by activation energy, barrier
//! heights, and tunneling probabilities. Transition rates often follow
//! Arrhenius or Kramers laws depending on damping regime.
//!
//! ## Quantum and Stochastic Effects
//!
//! At low temperatures or high barriers, quantum tunneling dominates
//! over thermal activation. Tunneling probabilities are estimated via the
//! WKB approximation, which depends on barrier width and particle mass.
//!
//! ## Biological Relevance
//!
//! Dormant attractors model seed banks, spore formation, viral latency,
//! circadian sleep cycles, and predator-prey refuges. Wake-up probabilities
//! depend on environmental cues and internal biological clocks.

pub mod latent;
pub mod potential;
pub mod sleeping;

pub use latent::LatentAttractor;
pub use potential::PotentialWell;
pub use sleeping::SleepingAttractor;

use crate::Attractor;

/// Returns the number of dormant attractor submodules.
pub fn dormant_submodule_count() -> usize {
    3
}

/// Describes the dormant attractors package scope.
pub fn describe_dormant_package() -> &'static str {
    "Dormant attractors: latent structures, potential wells, and sleeping states."
}
