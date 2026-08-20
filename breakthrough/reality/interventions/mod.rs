// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Interventions: Actions taken to modify the environment or system state.
//!
//! Interventions close the perception-action loop by translating
//! intentions into concrete actions that change reality.

pub mod intervention;
pub mod action;
pub mod execution;
pub mod effect;

pub use intervention::{Intervention, ExecutionPlan};
pub use action::InterventionType;
pub use execution::{Action, ActionResult};
pub use effect::{Effect, EffectDirection};
