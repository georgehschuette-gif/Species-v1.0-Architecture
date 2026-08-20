// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Stable Attractors
//!
//! Stable attractors represent asymptotically persistent structures in phase
//! space. Trajectories initialized in a neighborhood converge toward these
//! objects under forward time evolution. This submodule covers equilibrium
//! points (fixed points) and limit cycles, which are the two classical stable
//! attractors in continuous-time dynamical systems.
//!
//! ## Types
//!
//! - [`AttractorPoint`]: A point attractor with stability metadata and
//!   Euclidean coordinates. Supports distance computation, perturbation
//!   analysis, and basin-of-attraction checks.
//! - [`Equilibrium`]: A fixed point classified by local linearization
//!   (node, spiral, center, saddle). Includes Jacobian, eigenvalue, and
//!   determinant computations.
//! - [`LimitCycle`]: A periodic orbit characterized by period, amplitude,
//!   phase, and transverse stability. Supports Floquet analysis and
//!   enclosed-area computation.
//!
//! ## Stability Theory
//!
//! Stability is determined via eigenvalue analysis of the Jacobian matrix
//! evaluated at the attractor. An attractor is stable when all eigenvalues
//! have negative real parts (continuous time) or modulus strictly less than
//! unity (discrete time).
//!
//! ## Basins of Attraction
//!
//! Each stable attractor defines a basin of attraction: the set of initial
//! conditions whose forward trajectories converge to the attractor. Basins
//! may have fractal boundaries in non-linear systems.
//!
//! ## Numerical Methods
//!
//! This submodule provides helper methods for linear evolution, Jacobian
//! determinant computation, and LU decomposition for higher-dimensional
//! systems. All methods operate on dense `Vec<f64>` representations for
//! simplicity and compatibility with numerical linear algebra libraries.

pub mod point;
pub mod equilibrium;
pub mod limit_cycle;

pub use point::{AttractorPoint, Stability};
pub use equilibrium::{Equilibrium, EquilibriumClass};
pub use limit_cycle::LimitCycle;

use crate::Attractor;

/// Returns the number of stable attractor submodules in this package.
pub fn stable_submodule_count() -> usize {
    3
}

/// Returns a summary description of the stable attractors package.
pub fn describe_stable_package() -> &'static str {
    "Stable attractors: asymptotically persistent fixed points and periodic orbits."
}
