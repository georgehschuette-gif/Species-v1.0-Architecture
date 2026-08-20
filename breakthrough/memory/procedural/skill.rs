// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MemoryError;

use super::*;

/// ProceduralSkill: A named skill composed of ordered execution steps.
///
/// Skills model the "how" of cognition — the procedural knowledge that governs
/// skilled behavior. Mastery grows with rehearsal, automaticity increases
/// until the skill becomes unconscious, and error rates drop following a
/// learning curve.
#[derive(Debug, Clone, PartialEq)]
pub struct ProceduralSkill {
    /// Name identifying this skill.
    pub name: String,
    /// Number of discrete steps in this skill.
    pub steps: usize,
    /// Current mastery level in [0.0, 1.0].
    pub mastery_level: f64,
    /// Degree of automaticity in [0.0, 1.0].
    pub automaticity: f64,
    /// Current error rate during execution in [0.0, 1.0].
    pub error_rate: f64,
    /// Total number of rehearsals performed.
    pub rehearsal_count: usize,
    /// Base error rate before any learning.
    pub base_error_rate: f64,
}

impl ProceduralSkill {
    /// Creates a new ProceduralSkill with the given name and step count.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::MissingInput`] if the name is empty.
    pub fn new(name: impl Into<String>, steps: usize) -> Result<Self, MemoryError> {
        let n = name.into();
        if n.is_empty() {
            return Err(MemoryError::MissingInput("skill name must not be empty".into()));
        }
        if steps == 0 {
            return Err(MemoryError::DimensionMismatch {
                expected: 1,
                actual: 0,
            });
        }
        Ok(Self {
            name: n,
            steps,
            mastery_level: 0.0,
            automaticity: 0.0,
            error_rate: DEFAULT_BASE_ERROR_RATE,
            rehearsal_count: 0,
            base_error_rate: DEFAULT_BASE_ERROR_RATE,
        })
    }

    /// Rehearses the skill, increasing mastery and automaticity.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::OutOfRange`] if success is outside [0.0, 1.0].
    pub fn rehearse(&mut self, success: f64) -> Result<(), MemoryError> {
        if !(0.0..=1.0).contains(&success) {
            return Err(MemoryError::OutOfRange {
                field: "success".into(),
                value: success,
                min: 0.0,
                max: 1.0,
            });
        }
        let mastery_gain = DEFAULT_MASTERY_GAIN * success * (1.0 - self.mastery_level);
        self.mastery_level = (self.mastery_level + mastery_gain).min(MAX_MASTERY);
        let auto_gain = DEFAULT_AUTOMATICITY_GAIN * success * (1.0 - self.automaticity);
        self.automaticity = (self.automaticity + auto_gain).min(1.0);
        self.error_rate = (self.error_rate * (1.0 - mastery_gain)).max(MIN_ERROR_RATE);
        self.rehearsal_count += 1;
        Ok(())
    }

    /// Degrades the skill, simulating forgetting from disuse.
    pub fn degrade(&mut self, amount: f64) {
        self.mastery_level = (self.mastery_level - amount).max(0.0);
        self.automaticity = (self.automaticity - amount * 0.5).max(0.0);
        self.error_rate = (self.error_rate + amount * self.base_error_rate).min(1.0);
    }

    /// Simulates executing the skill, returning whether execution succeeded.
    pub fn execute(&self) -> bool {
        rand::random::<f64>() > self.error_rate
    }

    /// Computes the current success rate based on mastery and automaticity.
    pub fn success_rate(&self) -> f64 {
        let skill_factor = self.mastery_level * 0.7 + self.automaticity * 0.3;
        (skill_factor * (1.0 - self.error_rate)).clamp(0.0, 1.0)
    }

    /// Computes the learning progress rate (rehearsals per mastery point).
    pub fn progress_rate(&self) -> f64 {
        if self.mastery_level >= MAX_MASTERY {
            return 0.0;
        }
        self.rehearsal_count as f64 / (MAX_MASTERY - self.mastery_level)
    }

    /// Validates the skill state.
    pub fn validate(&self) -> Result<(), MemoryError> {
        Self::new(&self.name, self.steps)?;
        if !(0.0..=1.0).contains(&self.mastery_level) {
            return Err(MemoryError::OutOfRange {
                field: "mastery_level".into(),
                value: self.mastery_level,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&self.error_rate) {
            return Err(MemoryError::OutOfRange {
                field: "error_rate".into(),
                value: self.error_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&self.automaticity) {
            return Err(MemoryError::OutOfRange {
                field: "automaticity".into(),
                value: self.automaticity,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}
