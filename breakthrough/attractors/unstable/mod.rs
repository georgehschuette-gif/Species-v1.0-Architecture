// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Unstable Attractors
//!
//! Unstable attractors repel nearby trajectories and are associated with
//! divergence, sensitivity to initial conditions, and explosive growth.
//! This submodule models repellors, saddle points, and explosive attractors
//! that play central roles in catastrophe theory, turbulence, and
//! non-linear instability analysis.
//!
//! ## Types
//!
//! - [`Repellor`]: An attractor that drives trajectories outward from its
//!   neighborhood. Characterized by positive Lyapunov exponents and
//!   divergence rates.
//! - [`SaddlePoint`]: A hybrid structure with both stable and unstable
//!   manifolds. Governs separatrix topology and transition state theory.
//! - [`ExplosiveAttractor`]: A singularity or blow-up structure with
//!   finite-time divergence. Models super-exponential growth and
//!   regularization challenges.
//!
//! ## Instability Metrics
//!
//! Instability is quantified through Lyapunov exponents, divergence rates,
//! and manifold separation velocities. Positive Lyapunov exponents indicate
//! sensitive dependence on initial conditions.
//!
//! ## Manifold Geometry
//!
//! Unstable attractors organize phase space through their stable and
//! unstable manifolds. Saddle points, in particular, define the boundaries
//! of basins of attraction and mediate transitions between attractors.
//!
//! ## Finite-Time Singularities
//!
//! Explosive attractors require specialized numerical treatment due to
//! stiffness, overflow risk, and regularization needs. Standard integrators
//! fail near blow-up; adaptive methods with cutoffs are necessary.

pub mod repellor;
pub mod saddle;
pub mod explosive;

pub use repellor::Repellor;
pub use saddle::SaddlePoint;
pub use explosive::ExplosiveAttractor;

use crate::Attractor;

/// Returns the number of unstable attractor submodules.
pub fn unstable_submodule_count() -> usize {
    3
}

/// Describes the unstable attractors package scope.
pub fn describe_unstable_package() -> &'static str {
    "Unstable attractors: repellors, saddles, and explosive blow-up structures."
}
