// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Sleeping Attractor
//!
//! A [`SleepingAttractor`] is a deeply dormant dynamical state that
//! persists for extended periods before spontaneously or externally
//! triggered activation. Sleeping attractors model biological hibernation,
//! dormant seeds, deep fault lines, and quiescent solar phenomena.
//!
//! ## Dormancy Depth
//!
//! Dormancy depth quantifies how suppressed the attractor is. Deeply
//! sleeping attractors require large perturbations or long waiting times
//! to activate.
//!
//! ## Wake Probability
//!
//! Wake-up follows stochastic or threshold dynamics. Wake probability
//! may increase with external stress, temperature, or elapsed time.
//!
//! ## Cyclical Dormancy
//!
//! Many systems exhibit periodic dormancy cycles (circadian rhythms,
//! seasonal hibernation) governed by internal clocks or environmental
//! cues.

use std::cmp::PartialEq;
use std::fmt::{self, Display, Formatter};

/// Sleep depth classification for a dormant attractor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SleepDepth {
    /// Light dormancy; easy to wake.
    Light,
    /// Moderate dormancy; requires significant perturbation.
    Moderate,
    /// Deep dormancy; high activation barrier.
    Deep,
    /// Hibernation-like extreme dormancy.
    Hibernation,
    /// Comatose state; activation practically impossible.
    Comatose,
}

impl Display for SleepDepth {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Light => "light",
            Self::Moderate => "moderate",
            Self::Deep => "deep",
            Self::Hibernation => "hibernation",
            Self::Comatose => "comatose",
        })
    }
}

/// A sleeping attractor with probabilistic wake-up dynamics.
///
/// # Fields
///
/// * `dormancy_depth` - Qualitative depth of dormancy.
/// * `wake_threshold` - Minimum perturbation magnitude to trigger
///   activation.
/// * `cycle_length` - Expected duration of a full dormancy cycle.
/// * `cycle_phase` - Current position within the cycle [0, 1].
/// * `wake_probability` - Probability of spontaneous wake-up per unit
///   time.
/// * `activation_energy` - Energy barrier for spontaneous activation.
/// * `external_stress` - Accumulated external perturbation magnitude.
/// * `is_hibernating` - Whether the system is in extended hibernation.
/// * `last_activated` - Time since last activation (arbitrary units).
#[derive(Debug, Clone, PartialEq)]
pub struct SleepingAttractor {
    pub dormancy_depth: SleepDepth,
    pub wake_threshold: f64,
    pub cycle_length: f64,
    pub cycle_phase: f64,
    pub wake_probability: f64,
    pub activation_energy: f64,
    pub external_stress: f64,
    pub is_hibernating: bool,
    pub last_activated: f64,
}

impl SleepingAttractor {
    /// Creates a new sleeping attractor with dormancy parameters.
    ///
    /// # Arguments
    ///
    /// * `dormancy_depth` - Sleep depth classification.
    /// * `wake_threshold` - Minimum perturbation to wake.
    /// * `cycle_length` - Full dormancy cycle duration.
    /// * `activation_energy` - Energy barrier for spontaneous wake.
    ///
    /// # Panics
    ///
    /// Panics if `wake_threshold` is negative or `cycle_length` is
    /// non-positive.
    pub fn new(
        dormancy_depth: SleepDepth,
        wake_threshold: f64,
        cycle_length: f64,
        activation_energy: f64,
    ) -> Self {
        assert!(wake_threshold >= 0.0, "wake_threshold must be non-negative");
        assert!(cycle_length > 0.0, "cycle_length must be positive");
        let wake_probability = Self::spontaneous_wake_rate(activation_energy, 1.0);
        Self {
            dormancy_depth,
            wake_threshold,
            cycle_length,
            cycle_phase: 0.0,
            wake_probability,
            activation_energy,
            external_stress: 0.0,
            is_hibernating: false,
            last_activated: 0.0,
        }
    }

    /// Returns the spatial dimension.
    pub fn dimension(&self) -> usize {
        1
    }

    /// Checks whether the attractor is currently asleep.
    ///
    /// Asleep means external stress is below the wake threshold and no
    /// recent activation has occurred.
    pub fn is_asleep(&self) -> bool {
        self.external_stress < self.wake_threshold && self.cycle_phase < 0.9
    }

    /// Computes the instantaneous wake probability given accumulated
    /// stress.
    ///
    /// The probability follows a sigmoidal dependence on stress relative
    /// to threshold.
    ///
    /// # Arguments
    ///
    /// * `stress` - External perturbation magnitude.
    pub fn wake_probability(&self, stress: f64) -> f64 {
        let ratio = stress / self.wake_threshold.max(1e-12);
        1.0 / (1.0 + (-10.0 * (ratio - 1.0)).exp())
    }

    /// Estimates the cycle length in arbitrary time units.
    ///
    /// Cycle length may be modulated by external stress and temperature.
    pub fn cycle_length(&self) -> f64 {
        let stress_factor = 1.0 + self.external_stress * 0.1;
        self.cycle_length * stress_factor
    }

    /// Checks whether the system is in extended hibernation.
    ///
    /// Hibernation is a deep dormancy state with severely suppressed
    /// wake probability.
    pub fn is_hibernating(&self) -> bool {
        self.is_hibernating || matches!(self.dormancy_depth, SleepDepth::Hibernation | SleepDepth::Comatose)
    }

    /// Accumulates external stress and updates internal state.
    ///
    /// # Arguments
    ///
    /// * `stress_increment` - Additional stress applied.
    /// * `dt` - Time elapsed since last update.
    pub fn accumulate_stress(&mut self, stress_increment: f64, dt: f64) {
        self.external_stress += stress_increment;
        self.cycle_phase = (self.cycle_phase + dt / self.cycle_length) % 1.0;
        if self.external_stress >= self.wake_threshold {
            self.last_activated = 0.0;
        }
    }

    /// Computes the spontaneous wake rate using an Arrhenius-like law.
    fn spontaneous_wake_rate(activation_energy: f64, temperature: f64) -> f64 {
        if activation_energy <= 0.0 { return 1.0; }
        (-activation_energy / temperature.max(1e-12)).exp()
    }

    /// Returns the remaining time until the next expected cycle peak.
    pub fn time_to_next_cycle_peak(&self) -> f64 {
        (1.0 - self.cycle_phase) * self.cycle_length
    }

    /// Resets the external stress and dormancy phase.
    pub fn reset_stress(&mut self) {
        self.external_stress = 0.0;
        self.cycle_phase = 0.0;
    }

    /// Returns a summary string.
    pub fn summary(&self) -> String {
        format!(
            "SleepingAttractor(depth={}, wake_threshold={:.4}, cycle={:.4}, asleep={})",
            self.dormancy_depth,
            self.wake_threshold,
            self.cycle_length,
            self.is_asleep()
        )
    }
}

impl Display for SleepingAttractor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SleepingAttractor(depth={}, wake_threshold={:.6}, cycle_length={:.6})",
            self.dormancy_depth, self.wake_threshold, self.cycle_length
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_asleep() {
        let sa = SleepingAttractor::new(SleepDepth::Deep, 1.0, 10.0, 2.0);
        assert!(sa.is_asleep());
    }

    #[test]
    fn test_wake_probability_sigmoid() {
        let sa = SleepingAttractor::new(SleepDepth::Moderate, 1.0, 10.0, 1.0);
        let p_low = sa.wake_probability(0.5);
        let p_high = sa.wake_probability(2.0);
        assert!(p_high > p_low);
    }

    #[test]
    fn test_cycle_length_modulation() {
        let mut sa = SleepingAttractor::new(SleepDepth::Light, 1.0, 10.0, 0.5);
        sa.accumulate_stress(1.0, 1.0);
        assert!(sa.cycle_length() >= 10.0);
    }
}
