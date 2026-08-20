// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// PrimarySuccession: The colonization of a previously empty habitat.
pub struct PrimarySuccession {
    pub pioneer_species: Vec<u64>,
    pub soil_development: f64,
    pub succession_stage: SuccessionStage,
    pub time_elapsed: f64,
}

impl PrimarySuccession {
    /// Constructs a new PrimarySuccession with validated parameters.
    ///
    /// # Errors
    /// Returns `NoPioneerSpecies` if pioneer_species is empty.
    /// Returns `InvalidProgress` if soil_development is outside [0.0, 1.0].
    /// Returns `InvalidTime` if time_elapsed is negative, NaN, or infinite.
    /// Returns `InvalidParticipant` if any species ID is zero.
    pub fn new(
        pioneer_species: Vec<u64>,
        soil_development: f64,
        succession_stage: SuccessionStage,
        time_elapsed: f64,
    ) -> Result<Self, SuccessionError> {
        if pioneer_species.is_empty() {
            return Err(SuccessionError::NoPioneerSpecies(
                "At least one pioneer species is required".to_string(),
            ));
        }
        if soil_development.is_nan()
            || soil_development.is_infinite()
            || !(0.0..=1.0).contains(&soil_development)
        {
            return Err(SuccessionError::InvalidProgress(
                "Soil development must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        if time_elapsed.is_nan() || time_elapsed.is_infinite() || time_elapsed < 0.0 {
            return Err(SuccessionError::InvalidTime(
                "Time elapsed must be a non-negative finite number".to_string(),
            ));
        }
        for (i, &id) in pioneer_species.iter().enumerate() {
            if id == 0 {
                return Err(SuccessionError::InvalidParticipant(
                    format!("Pioneer species at index {} has zero ID", i),
                ));
            }
        }
        Ok(Self {
            pioneer_species,
            soil_development,
            succession_stage,
            time_elapsed,
        })
    }

    /// Validates the internal consistency of this primary succession.
    pub fn validate(&self) -> Result<(), SuccessionError> {
        if self.pioneer_species.is_empty() {
            return Err(SuccessionError::NoPioneerSpecies(
                "At least one pioneer species is required".to_string(),
            ));
        }
        if self.soil_development.is_nan()
            || self.soil_development.is_infinite()
            || !(0.0..=1.0).contains(&self.soil_development)
        {
            return Err(SuccessionError::InvalidProgress(
                "Soil development must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        if self.time_elapsed.is_nan()
            || self.time_elapsed.is_infinite()
            || self.time_elapsed < 0.0
        {
            return Err(SuccessionError::InvalidTime(
                "Time elapsed must be a non-negative finite number".to_string(),
            ));
        }
        for &id in &self.pioneer_species {
            if id == 0 {
                return Err(SuccessionError::InvalidParticipant(
                    "Pioneer species ID must be non-zero".to_string(),
                ));
            }
        }
        Ok(())
    }

    /// Advances the succession by the given time delta.
    ///
    /// Soil development increases proportionally to the time delta,
    /// clamped to [0.0, 1.0]. Returns an error if delta is negative.
    pub fn advance(&mut self, delta: f64) -> Result<(), SuccessionError> {
        if delta.is_nan() || delta.is_infinite() || delta < 0.0 {
            return Err(SuccessionError::InvalidTime(
                "Time delta must be a non-negative finite number".to_string(),
            ));
        }
        self.soil_development = (self.soil_development + delta * 0.1).min(1.0);
        self.time_elapsed += delta;
        Ok(())
    }

    /// Returns true if primary succession is complete (soil development >= 1.0).
    pub fn is_complete(&self) -> bool {
        self.soil_development >= 1.0
    }

    /// Returns the soil maturity as a percentage.
    pub fn soil_maturity(&self) -> f64 {
        self.soil_development * 100.0
    }

    /// Returns the number of pioneer species.
    pub fn pioneer_count(&self) -> usize {
        self.pioneer_species.len()
    }

    /// Returns the elapsed time.
    pub fn elapsed_time(&self) -> f64 {
        self.time_elapsed
    }
}

impl std::fmt::Display for PrimarySuccession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PrimarySuccession(pioneers={}, soil_development={:.4}, time_elapsed={:.4})",
            self.pioneer_species.len(),
            self.soil_development,
            self.time_elapsed
        )
    }
}