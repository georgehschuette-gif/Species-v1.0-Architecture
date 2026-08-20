// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Signal Fusion: Combining multiple sensory inputs into unified percepts.
//! Integrates cross-modal data into coherent representations.

pub mod kalman;
pub mod bayesian;
pub mod attention;

pub use kalman::KalmanFusion;
pub use bayesian::BayesianFusion;
pub use attention::AttentionMechanism;

pub use super::PerceptionResult;
pub use super::PerceptionError;
