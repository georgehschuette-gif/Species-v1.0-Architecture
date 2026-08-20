// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Invariants: Properties that must never be violated.
//! These are the guardrails of the cognitive ecosystem.

pub mod conservation;
pub mod coherence;
pub mod causality;

pub use conservation::ConservationInvariant;
pub use coherence::CoherenceInvariant;
pub use causality::CausalityInvariant;
