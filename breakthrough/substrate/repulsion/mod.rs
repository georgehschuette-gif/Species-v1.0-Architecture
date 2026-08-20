// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Repulsion: The force pushing entities apart based on incompatibility.
//! Governs defensive separation, exclusion, and boundary maintenance.

pub mod incompatibility;
pub mod force;
pub mod boundary;

pub use incompatibility::{IncompatibilityField, IncompatibilityError};
pub use force::{RepulsiveForce, ForceError};
pub use boundary::{CognitiveBoundary, BoundaryError};
