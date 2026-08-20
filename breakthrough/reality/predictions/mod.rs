// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Predictions: Forecasts of future states derived from models.
//!
//! Predictions extend the reality loop into the future, allowing the
//! cognitive system to plan, prepare, and evaluate alternative courses
//! of action. Each prediction carries uncertainty quantification.

pub mod prediction;
pub mod forecast;
pub mod scenario;
pub mod uncertainty;
pub mod model;

pub use prediction::Prediction;
pub use uncertainty::{TrendDirection, Uncertainty};
pub use forecast::Forecast;
pub use scenario::Scenario;
pub use model::PredictionModel;
