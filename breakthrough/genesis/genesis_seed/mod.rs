// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Genesis Seed: The initial conditions from which the ecosystem bootstraps.
//! Contains the primordial state and bootstrap sequence.

pub mod initial_state;
pub mod bootstrap;
pub mod primordial;

pub use initial_state::InitialState;
pub use bootstrap::BootstrapSequence;
pub use bootstrap::BootstrapStep;
pub use primordial::PrimordialField;