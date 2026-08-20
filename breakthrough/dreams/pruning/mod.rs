// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

pub mod pruning;
pub mod selection;

pub use pruning::{SynapticPruning, PruningTarget};
pub use selection::{ExperienceSelection, SelectionCriterion};