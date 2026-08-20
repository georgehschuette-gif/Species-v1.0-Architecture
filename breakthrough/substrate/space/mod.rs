// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Space: The geometric substrate in which cognitive entities exist.
//! Defines positions, distances, and spatial relationships.

pub mod position;
pub mod distance;
pub mod manifold;

pub use position::{Position, PositionError};
pub use distance::{Distance, DistanceError};
pub use manifold::{SpatialManifold, ManifoldError};
