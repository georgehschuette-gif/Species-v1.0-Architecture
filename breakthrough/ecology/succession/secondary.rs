// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// SecondarySuccession: Recovery of a habitat after disturbance.
pub struct SecondarySuccession {
    pub disturbance_event: DisturbanceEvent,
    pub recovery_rate: f64,
    pub residual_populations: Vec<u64>,
    pub expected_recovery_time: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisturbanceEvent {
    Fire,
    Flood,
    Collapse,
    Reset,
}

impl SecondarySuccession {
    /// Constructs a new SecondarySuccession with validated parameters.
    ///
    /// # Errors
    /// Returns `InvalidProgress` if recovery_rate is outside [0.0, 1.0].
    /// Returns `InvalidTime` if expected_recovery_time is negative, NaN, or infinite.
    /// Returns `NoPioneerSpecies` if residual_populations is empty.
    pub fn new(
        disturbance_event: DisturbanceEvent,
        recovery_rate: f64,
        residual_populations: Vec<u64>,
        expected_recovery_time: f64,
    ) -> Result<Self, SuccessionError> {
        if residual_populations.is_empty() {
            return Err(SuccessionError::NoPioneerSpecies(
                "At least one residual population is required".to_string(),
            ));
        }
        if recovery_rate.is_nan()
            || recovery_rate.is_infinite()
            || !(0.0..=1.0).contains(&recovery_rate)
        {
            return Err(SuccessionError::InvalidProgress(
                "Recovery rate must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        if expected_recovery_time.is_nan()
            || expected_recovery_time.is_infinite()
            || expected_recovery_time < 0.0
        {
            return Err(SuccessionError::InvalidTime(
                "Expected recovery time must be a non-negative finite number".to_string(),
            ));
        }
        for (i, &id) in residual_populations.iter().enumerate() {
            if id == 0 {
                return Err(SuccessionError::InvalidParticipant(
                    format!("Residual population at index {} has zero ID", i),
                ));
            }
        }
        Ok(Self {
            disturbance_event,
            recovery_rate,
            residual_populations,
            expected_recovery_time,
        })
    }

    /// Validates the internal consistency of this secondary succession.
    pub fn validate(&self) -> Result<(), SuccessionError> {
        if self.residual_populations.is_empty() {
            return Err(SuccessionError::NoPioneerSpecies(
                "At least one residual population is required".to_string(),
            ));
        }
        if self.recovery_rate.is_nan()
            || self.recovery_rate.is_infinite()
            || !(0.0..=1.0).contains(&self.recovery_rate)
        {
            return Err(SuccessionError::InvalidProgress(
                "Recovery rate must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        if self.expected_recovery_time.is_nan()
            || self.expected_recovery_time.is_infinite()
            || self.expected_recovery_time < 0.0
        {
            return Err(SuccessionError::InvalidTime(
                "Expected recovery time must be a non-negative finite number".to_string(),
            ));
        }
        for &id in &self.residual_populations {
            if id == 0 {
                return Err(SuccessionError::InvalidParticipant(
                    "Residual population ID must be non-zero".to_string(),
                ));
            }
        }
        Ok(())
    }

    /// Returns the recovery progress based on the recovery rate and time.
    ///
    /// Progress is computed as `1.0 - exp(-recovery_rate * time_factor)`.
    pub fn recovery_progress(&self) -> f64 {
        if self.expected_recovery_time == 0.0 {
            return 1.0;
        }
        let time_factor = self.recovery_rate * self.expected_recovery_time;
        1.0 - (-time_factor).exp()
    }

    /// Returns true if the habitat has fully recovered.
    pub fn is_recovered(&self) -> bool {
        self.recovery_progress() >= 0.95
    }

    /// Returns the remaining recovery time estimate.
    pub fn remaining_time(&self) -> f64 {
        if self.recovery_rate == 0.0 {
            f64::INFINITY
        } else {
            self.expected_recovery_time * (1.0 - self.recovery_progress())
        }
    }

    /// Returns the number of residual populations.
    pub fn residual_count(&self) -> usize {
        self.residual_populations.len()
    }

    /// Returns the disturbance event as a string.
    pub fn disturbance_str(&self) -> &'static str {
        match self.disturbance_event {
            DisturbanceEvent::Fire => "fire",
            DisturbanceEvent::Flood => "flood",
            DisturbanceEvent::Collapse => "collapse",
            DisturbanceEvent::Reset => "reset",
        }
    }
}

impl std::fmt::Display for SecondarySuccession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SecondarySuccession(disturbance={}, recovery_rate={:.4}, residuals={}, expected_time={:.4})",
            self.disturbance_str(),
            self.recovery_rate,
            self.residual_populations.len(),
            self.expected_recovery_time
        )
    }
}