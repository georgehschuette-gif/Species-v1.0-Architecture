// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MemoryError;

use super::*;

/// EchoEvent: A single perceptual echo emitted by a stimulus.
///
/// Echo events model the immediate reverberation of sensory input in the
/// cognitive system. They decay rapidly and can be superposed with other
/// events if their temporal windows overlap.
#[derive(Debug, Clone, PartialEq)]
pub struct EchoEvent {
    /// Current intensity of the echo in [0.0, 1.0].
    pub intensity: f64,
    /// Rate at which the echo fades per time unit.
    pub fade_rate: f64,
    /// Elapsed time since emission.
    pub age: f64,
    /// Optional label identifying the stimulus source.
    pub label: String,
}

impl EchoEvent {
    pub const MIN_INTENSITY: f64 = 0.0;
    pub const MAX_INTENSITY: f64 = 1.0;
    pub const MIN_FADE_RATE: f64 = 0.0;
    pub const MAX_FADE_RATE: f64 = 1.0;

    /// Creates a new EchoEvent with the given intensity and fade rate.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::OutOfRange`] if intensity or fade rate is outside
    /// their respective valid ranges.
    pub fn new(intensity: f64, fade_rate: f64) -> Result<Self, MemoryError> {
        if !(Self::MIN_INTENSITY..=Self::MAX_INTENSITY).contains(&intensity) {
            return Err(MemoryError::OutOfRange {
                field: "intensity".into(),
                value: intensity,
                min: Self::MIN_INTENSITY,
                max: Self::MAX_INTENSITY,
            });
        }
        if !(Self::MIN_FADE_RATE..=Self::MAX_FADE_RATE).contains(&fade_rate) {
            return Err(MemoryError::OutOfRange {
                field: "fade_rate".into(),
                value: fade_rate,
                min: Self::MIN_FADE_RATE,
                max: Self::MAX_FADE_RATE,
            });
        }
        Ok(Self {
            intensity,
            fade_rate,
            age: 0.0,
            label: String::new(),
        })
    }

    /// Advances the echo by the given delta, decaying its intensity.
    ///
    /// Intensity update: `new = intensity * (1 - fade_rate)^delta`
    pub fn fade(&mut self, delta: f64) {
        if delta < 0.0 {
            return;
        }
        self.age += delta;
        let decay_factor = (1.0 - self.fade_rate).powf(delta);
        self.intensity *= decay_factor;
        self.intensity = self.intensity.clamp(Self::MIN_INTENSITY, Self::MAX_INTENSITY);
    }

    /// Amplifies the echo by the given factor, capped at max intensity.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::OutOfRange`] if the factor is outside [0.0, 1.0].
    pub fn amplify(&mut self, factor: f64) -> Result<(), MemoryError> {
        if !(0.0..=1.0).contains(&factor) {
            return Err(MemoryError::OutOfRange {
                field: "factor".into(),
                value: factor,
                min: 0.0,
                max: 1.0,
            });
        }
        self.intensity = (self.intensity + factor).min(Self::MAX_INTENSITY);
        Ok(())
    }

    /// Superimposes another echo onto this one by adding intensities.
    pub fn superimpose(&mut self, other: &EchoEvent) {
        self.intensity = (self.intensity + other.intensity).min(MAX_SUPERPOSED_INTENSITY);
    }

    /// Returns whether the echo is still above the perception threshold.
    pub fn is_active(&self) -> bool {
        self.intensity >= PERCEPTION_THRESHOLD
    }

    /// Returns the current effective strength after accounting for age.
    pub fn strength(&self) -> f64 {
        (self.intensity * (1.0 - self.fade_rate).powf(self.age)).clamp(Self::MIN_INTENSITY, Self::MAX_INTENSITY)
    }

    /// Sets the label identifying the stimulus source.
    pub fn set_label(&mut self, label: impl Into<String>) {
        self.label = label.into();
    }

    /// Returns the current label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Validates the echo event state.
    pub fn validate(&self) -> Result<(), MemoryError> {
        Self::new(self.intensity, self.fade_rate)?;
        if self.age < 0.0 {
            return Err(MemoryError::OutOfRange {
                field: "age".into(),
                value: self.age,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        Ok(())
    }
}

