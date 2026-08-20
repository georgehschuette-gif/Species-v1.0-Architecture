// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MemoryError;

use super::*;

/// EchoReverberation: A chain of echo events that propagate and resonate.
///
/// This models the sustained chain reaction of perceptual echoes that occurs
/// when stimuli repeatedly interact with a receptive cognitive surface. The
/// chain automatically prunes inactive echoes after each propagation step.
#[derive(Debug, Clone, PartialEq)]
pub struct EchoReverberation {
    /// Active echo events in the chain.
    pub echoes: Vec<EchoEvent>,
    /// Maximum number of echoes the chain can sustain.
    pub capacity: usize,
    /// Fade rate applied to all echoes during propagation.
    pub fade_rate: f64,
    /// Maximum intensity any single echo may reach via resonance.
    pub max_intensity: f64,
}

impl EchoReverberation {
    /// Minimum capacity for a reverberation chain.
    pub const MIN_CAPACITY: usize = 1;
    /// Maximum capacity for a reverberation chain.
    pub const MAX_CAPACITY: usize = 10_000;

    /// Creates a new EchoReverberation with the given fade rate and capacity.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::OutOfRange`] if fade rate is outside [0.0, 1.0].
    /// Returns [`MemoryError::CapacityExceeded`] if capacity is outside
    /// [`MIN_CAPACITY`..`MAX_CAPACITY`].
    pub fn new(fade_rate: f64) -> Result<Self, MemoryError> {
        Self::with_capacity(fade_rate, 100)
    }

    /// Creates a new EchoReverberation with explicit capacity.
    pub fn with_capacity(fade_rate: f64, capacity: usize) -> Result<Self, MemoryError> {
        if !(0.0..=1.0).contains(&fade_rate) {
            return Err(MemoryError::OutOfRange {
                field: "fade_rate".into(),
                value: fade_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(Self::MIN_CAPACITY..=Self::MAX_CAPACITY).contains(&capacity) {
            return Err(MemoryError::CapacityExceeded {
                max: Self::MAX_CAPACITY,
                attempted: capacity,
            });
        }
        Ok(Self {
            echoes: Vec::with_capacity(capacity),
            capacity,
            fade_rate,
            max_intensity: MAX_SUPERPOSED_INTENSITY,
        })
    }

    /// Emits a new echo into the chain with the given intensity.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::CapacityExceeded`] if the chain is at capacity.
    /// Returns [`MemoryError::OutOfRange`] if intensity is outside [0.0, 1.0].
    pub fn emit(&mut self, intensity: f64) -> Result<(), MemoryError> {
        if self.echoes.len() >= self.capacity {
            return Err(MemoryError::CapacityExceeded {
                max: self.capacity,
                attempted: self.echoes.len() + 1,
            });
        }
        let mut echo = EchoEvent::new(intensity, self.fade_rate)?;
        if self.echoes.is_empty() {
            echo.set_label(format!("echo_{}", self.echoes.len()));
        } else {
            let base_label = self.echoes[0].label().to_string();
            echo.set_label(base_label);
        }
        self.echoes.push(echo);
        Ok(())
    }

    /// Adds a secondary echo that resonates with the chain.
    ///
    /// Unlike `emit`, a resonant echo may amplify existing echoes if their
    /// labels match.
    pub fn echo(&mut self, intensity: f64) -> Result<(), MemoryError> {
        if self.echoes.is_empty() {
            return self.emit(intensity);
        }
        let primary_label = self.echoes[0].label().to_string();
        let mut echo = EchoEvent::new(intensity, self.fade_rate)?;
        echo.set_label(primary_label.clone());
        for existing in &mut self.echoes {
            if existing.label() == &primary_label {
                existing.amplify(0.05).ok();
            }
        }
        self.echoes.push(echo);
        Ok(())
    }

    /// Advances the chain by delta time, fading all echoes and pruning inactive ones.
    pub fn propagate(&mut self, delta: f64) {
        if delta <= 0.0 {
            return;
        }
        for echo in &mut self.echoes {
            echo.fade(delta);
        }
        self.echoes.retain(EchoEvent::is_active);
    }

    /// Induces resonance across all echoes sharing the same label, amplifying them.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::MissingInput`] if no echoes exist.
    pub fn resonate(&mut self) -> Result<(), MemoryError> {
        if self.echoes.is_empty() {
            return Err(MemoryError::MissingInput("no echoes to resonate".into()));
        }
        let primary = self.echoes[0].label().to_string();
        for echo in &mut self.echoes {
            if echo.label() == primary {
                echo.amplify(0.1).ok();
            }
        }
        Ok(())
    }

    /// Returns the intensity of the strongest echo in the chain.
    pub fn strongest_echo(&self) -> f64 {
        self.echoes.iter().map(|e| e.intensity).fold(0.0, f64::max)
    }

    /// Returns the number of currently active (perceptible) echoes.
    pub fn active_count(&self) -> usize {
        self.echoes.iter().filter(|e| e.is_active()).count()
    }

    /// Returns whether the reverberation chain has any active echoes.
    pub fn is_active(&self) -> bool {
        self.echoes.iter().any(EchoEvent::is_active)
    }

    /// Validates the reverberation chain state.
    pub fn validate(&self) -> Result<(), MemoryError> {
        if self.echoes.len() > self.capacity {
            return Err(MemoryError::CapacityExceeded {
                max: self.capacity,
                attempted: self.echoes.len(),
            });
        }
        for echo in &self.echoes {
            echo.validate()?;
        }
        Ok(())
    }
}

