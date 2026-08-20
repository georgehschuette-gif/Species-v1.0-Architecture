// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Echoes: Ultra-short-term perceptual reverberations in cognitive state space.
//!
//! An echo is a transient copy of a sensory or perceptual event that persists
//! briefly after the original stimulus has ceased. Echoes are characterized by
//! rapid exponential decay and the ability to superpose when multiple stimuli
//! arrive within a short temporal window.
//!
//! # Phases
//!
//! 1. **Emission** — A stimulus with sufficient intensity generates an echo.
//! 2. **Fading** — The echo decays over time according to a configurable fade rate.
//! 3. **Superposition** — Coincident echoes combine their intensities, capped at
//!    the maximum representable strength.
//! 4. **Extinction** — Once intensity drops below the perception threshold,
//!    the echo is removed from the active set.
//!
//! # Examples
//!
//! ```
//! use breakthrough::memory::echoes::{EchoEvent, EchoReverberation};
//!
//! let mut echo = EchoEvent::new(0.8, 0.1).expect("valid echo");
//! assert!(echo.is_active());
//! echo.fade(0.5);
//! assert!(echo.intensity < 0.8);
//!
//! let mut chain = EchoReverberation::new(0.2).expect("valid chain");
//! chain.emit(0.9).expect("emit");
//! chain.echo(0.6).expect("echo");
//! ```

use std::fmt;

use crate::MemoryError;

pub mod echo;
pub mod reverberation;

pub use echo::EchoEvent;
pub use reverberation::EchoReverberation;

/// Default echo fade rate per time unit.
pub const DEFAULT_ECHO_FADE_RATE: f64 = 0.08;
/// Minimum intensity for an echo to remain perceptible.
pub const PERCEPTION_THRESHOLD: f64 = 0.01;
/// Maximum superposition intensity cap.
pub const MAX_SUPERPOSED_INTENSITY: f64 = 1.0;

/// Creates a fresh echo event with the given intensity and fade rate.
///
/// # Errors
///
/// Returns [`MemoryError::OutOfRange`] if intensity or fade rate is outside [0.0, 1.0].
pub fn create_echo(intensity: f64, fade_rate: f64) -> Result<EchoEvent, MemoryError> {
    EchoEvent::new(intensity, fade_rate)
}

/// Creates an echo reverberation chain with the given fade rate.
///
/// # Errors
///
/// Returns [`MemoryError::OutOfRange`]] if fade rate is outside [0.0, 1.0].
pub fn create_reverberation(fade_rate: f64) -> Result<EchoReverberation, MemoryError> {
    EchoReverberation::new(fade_rate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn echo_creation_succeeds() {
        let e = create_echo(0.8, DEFAULT_ECHO_FADE_RATE).unwrap();
        assert!(e.is_active());
        assert!((e.intensity - 0.8).abs() < f64::EPSILON);
    }

    #[test]
    fn echo_fades_over_time() {
        let mut e = create_echo(1.0, 0.1).unwrap();
        e.fade(1.0);
        assert!(e.intensity < 1.0);
    }

    #[test]
    fn echo_event_label_settable() {
        let mut e = create_echo(0.5, 0.05).unwrap();
        e.set_label("stimulus_a");
        assert_eq!(e.label(), "stimulus_a");
    }

    #[test]
    fn reverberation_chain_emits_and_echoes() {
        let mut chain = create_reverberation(0.05).unwrap();
        chain.emit(0.9).unwrap();
        chain.echo(0.7).unwrap();
        assert_eq!(chain.active_count(), 2);
    }

    #[test]
    fn active_counts_match() {
        let mut chain = create_reverberation(0.1).unwrap();
        for i in 1..6 {
            chain.emit(i as f64 * 0.15).unwrap();
        }
        assert_eq!(chain.active_count(), 5);
    }

    #[test]
    fn propagate_ages_all_echoes() {
        let mut chain = create_reverberation(0.05).unwrap();
        chain.emit(0.8).unwrap();
        chain.propagate(1.0);
        assert!(chain.active_count() <= 1);
    }

    #[test]
    fn default_constants_valid() {
        assert!(DEFAULT_ECHO_FADE_RATE > 0.0 && DEFAULT_ECHO_FADE_RATE <= 1.0);
        assert!(PERCEPTION_THRESHOLD >= 0.0);
        assert!(MAX_SUPERPOSED_INTENSITY > 0.0);
    }
}
