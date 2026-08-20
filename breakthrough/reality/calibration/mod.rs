// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Calibration: Adjustment of model parameters to match reality.
//!
//! Calibration bridges predictions and observations by tuning model
//! parameters to minimize prediction error. It is the mechanism by
//! which models improve over time.

pub mod calibration;
pub mod parameter;
pub mod fitting;
pub mod residual;

pub use calibration::Calibration;
pub use parameter::{CalibrationMethod, FitMetric};
pub use fitting::ParameterAdjustment;
pub use residual::ResidualAnalysis;
