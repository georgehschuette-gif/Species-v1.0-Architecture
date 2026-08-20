// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Emergent Attractors
//!
//! Emergent attractors arise spontaneously from collective interactions
//! rather than being prescribed by individual components. They are central
//! to pattern formation, self-organization, and phase transition theory.
//! This submodule covers spontaneous symmetry breaking, continuous and
//! discontinuous phase transitions, and critical points with diverging
//! correlation lengths.
//!
//! ## Types
//!
//! - [`SpontaneousAttractor`]: An order parameter that emerges when a
//!   symmetry-breaking threshold is crossed. Supports Landau mean-field
//!   theory and susceptibility computation.
//! - [`PhaseTransition`]: A boundary between distinct macroscopic phases
//!   characterized by non-analytic free energy. Models first-order,
//!   second-order, tricritical, and infinite-order transitions.
//! - [`CriticalPoint`]: A singular point with diverging susceptibility and
//!   power-law scaling governed by universality classes. Includes critical
//!   exponent validation and scaling relations.
//!
//! ## Universality
//!
//! Critical exponents and scaling forms depend only on symmetry and
//! dimensionality, not microscopic details, enabling powerful predictive
//! frameworks across disparate physical systems.
//!
//! ## Scaling Laws
//!
//! Near criticality, physical quantities obey power laws with universal
//! exponents. The Rushbrooke, Widom, Fisher, and Josephson relations
//! connect these exponents, reducing the independent degrees of freedom.
//!
//! ## Applications
//!
//! Emergent attractors model ferromagnetic transitions, superfluid
//! helium, liquid-vapor critical points, percolation thresholds, and
//! synchronization transitions in complex networks.

pub mod spontaneous;
pub mod phase_transition;
pub mod critical;

pub use spontaneous::SpontaneousAttractor;
pub use phase_transition::PhaseTransition;
pub use critical::CriticalPoint;

use crate::Attractor;

/// Returns the number of emergent attractor submodules.
pub fn emergent_submodule_count() -> usize {
    3
}

/// Describes the emergent attractors package scope.
pub fn describe_emergent_package() -> &'static str {
    "Emergent attractors: spontaneous order, phase transitions, and critical points."
}
