// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Recursive Attractors
//!
//! Recursive attractors encode self-reference or nested self-application
//! within their dynamics. These structures appear in formal systems,
//! reflexively adaptive control, and fractal geometry. This submodule
//! provides types for self-referential maps, feedback loops, and nested
//! attractor hierarchies.
//!
//! ## Types
//!
//! - [`SelfReferential`]: A map that transforms its own representation,
//!   requiring a fixed-point condition for consistency. Implements bounded
//!   recursion with configurable depth limits.
//! - [`FeedbackLoop`]: A closed control or signal loop with gain, delay,
//!   and transfer-function characterization. Supports Nyquist analysis,
//!   step response, and phase margin computation.
//! - [`NestedAttractor`]: A hierarchical attractor containing inner and
//!   outer attractors with scale-ratio embeddings. Supports self-similarity
//!   detection and scale-consistency validation.
//!
//! ## Recursive Depth
//!
//! Each type tracks recursion depth to prevent infinite descent and to
//! support termination proofs or bounded approximation. Maximum depth is
//! configurable at construction time.
//!
//! ## Fixed Points and Consistency
//!
//! Recursive structures must satisfy consistency conditions: applying the
//! transformation must yield an equivalent representation. Failure to
//! converge indicates logical inconsistency or insufficient depth.
//!
//! ## Applications
//!
//! Recursive attractors model reflexively adaptive agents, self-modifying
//! code, fractal dimension estimation, renormalization group flows, and
//! meta-learning systems where the learner modifies its own learning
//! algorithm.

pub mod self_referential;
pub mod feedback_loop;
pub mod nested;

pub use self_referential::SelfReferential;
pub use feedback_loop::FeedbackLoop;
pub use nested::NestedAttractor;

use crate::Attractor;

/// Returns the number of recursive attractor submodules.
pub fn recursive_submodule_count() -> usize {
    3
}

/// Describes the recursive attractors package scope.
pub fn describe_recursive_package() -> &'static str {
    "Recursive attractors: self-referential maps, feedback loops, and nested hierarchies."
}
