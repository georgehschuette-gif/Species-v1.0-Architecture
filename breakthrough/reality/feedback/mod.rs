// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Feedback: Learning signals that close the perception-action loop.
//!
//! Feedback aggregates the outcomes of interventions and verifications
//! into learning signals that drive model adaptation and improvement.
//! It is the mechanism by which the cognitive system learns from experience.

pub mod feedback;
pub mod learning;
pub mod adaptation;
pub mod correction;

pub use feedback::{Feedback, LearningSignal, SignalSource};
pub use learning::LearningMode;
pub use adaptation::AdaptationRate;
pub use correction::ErrorCorrection;
