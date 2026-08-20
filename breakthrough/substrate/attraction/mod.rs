// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Attraction: The force drawing entities together based on affinity.
//! Governs gravitational-like pulling, bonding, and convergence.

pub mod affinity;
pub mod force;
pub mod bond;

pub use affinity::{AffinityField, AffinityError};
pub use force::{AttractiveForce, ForceError};
pub use bond::{CognitiveBond, BondError};
