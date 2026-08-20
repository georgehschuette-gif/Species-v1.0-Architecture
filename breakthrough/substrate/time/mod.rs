// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Time: The temporal substrate governing cognitive process sequencing.
//! Defines clocks, intervals, and temporal ordering.

pub mod clock;
pub mod interval;
pub mod arrow;

pub use clock::{CognitiveClock, ClockError};
pub use interval::{TimeInterval, IntervalError};
pub use arrow::{TemporalArrow, ArrowError};
