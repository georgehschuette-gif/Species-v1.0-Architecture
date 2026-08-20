// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Verification: Validating predictions against observations.
//!
//! Verification compares predicted outcomes with actual observations
//! to assess model accuracy and identify areas for improvement.

pub mod verification;
pub mod test;
pub mod falsification;
pub mod validation;

pub use verification::Verification;
pub use test::TestResult;
pub use falsification::{FalsificationCriterion, FalsificationDirection};
pub use validation::{ValidationStatus, ValidationCriterion};
