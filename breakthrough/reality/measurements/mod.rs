// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Measurements: Quantified attributes of reality with precision and accuracy.
//!
//! Measurements convert raw observations into numerical values with
//! known uncertainties. They are the quantitative backbone of the
//! reality loop, enabling comparison, prediction, and verification.

pub mod unit;
pub mod precision;
pub mod accuracy;
pub mod measurement;

pub use unit::{MeasurementType, Unit, Scale, Dimension};
pub use precision::Precision;
pub use accuracy::Accuracy;
pub use measurement::{Measurement, MeasurementValue};
