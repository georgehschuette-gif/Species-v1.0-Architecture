// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Synchronization: The alignment of oscillatory cognitive processes.
//! Governs phase locking, rhythm formation, and collective timing.

pub mod phase_lock;
pub mod rhythm;
pub mod timing;

pub use phase_lock::{PhaseLock, PhaseLockError};
pub use rhythm::{CognitiveRhythm, RhythmError};
pub use timing::{CollectiveTiming, TimingError};
