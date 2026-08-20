// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Consequences: Outcomes resulting from interventions.
//!
//! Consequences are the observable results of actions taken by the
//! cognitive system. They provide the raw material for learning,
//! calibration, and model improvement.

pub mod consequence;
pub mod causality;
pub mod chain;
pub mod magnitude;

pub use consequence::Consequence;
pub use causality::CausalLink;
pub use chain::{CausalChain, Outcome};
pub use magnitude::EffectMagnitude;
