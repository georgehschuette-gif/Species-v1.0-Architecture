// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

pub mod sensor;
pub mod scene;
pub mod categorization;

pub use sensor::{SensoryModality, BandwidthClass};
pub use scene::{ObjectDetection, Region, Scene};
pub use categorization::{Observation, ObservationConfidence, ObservationSource};
