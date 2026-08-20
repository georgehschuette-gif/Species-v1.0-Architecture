// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// ConceptEvolution: Long-term structural changes in concepts.
///
/// Evolution drives the gradual transformation of concept structures
/// over extended time horizons. Selection pressure determines which
/// structural variants survive, while evolution speed controls the
/// rate of structural change.
///
/// # Fields
/// - `evolution_speed`: Rate of structural change per epoch, in [0.0, 1.0].
/// - `selection_pressure`: Strength of selection favoring fit variants, in [0.0, 1.0].
pub struct ConceptEvolution {
    /// Rate of structural change per epoch, in [0.0, 1.0].
    pub evolution_speed: f64,
    /// Strength of selection favoring fit variants, in [0.0, 1.0].
    pub selection_pressure: f64,
    /// Current fitness score of the concept structure.
    fitness: f64,
}

impl ConceptEvolution {
    /// Creates a new `ConceptEvolution` with the given speed and pressure.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if either parameter is outside [0.0, 1.0].
    pub fn new(
        evolution_speed: f64,
        selection_pressure: f64,
    ) -> Result<Self, CognitionError> {
        if !(0.0..=1.0).contains(&evolution_speed) {
            return Err(CognitionError::OutOfRange {
                field: "evolution_speed".to_string(),
                value: evolution_speed,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&selection_pressure) {
            return Err(CognitionError::OutOfRange {
                field: "selection_pressure".to_string(),
                value: selection_pressure,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            evolution_speed,
            selection_pressure,
            fitness: 0.5,
        })
    }

    /// Advances evolution by one epoch, modifying fitness and structure.
    ///
    /// Fitness changes by a random perturbation scaled by evolution_speed,
    /// then selection pressure is applied to favor higher fitness.
    pub fn evolve(&mut self) -> f64 {
        // Simulate structural variation
        let perturbation = (self.evolution_speed - 0.5).abs() * 0.1;
        let new_fitness = self.fitness + perturbation;
        // Apply selection pressure: fitter variants are retained
        self.fitness = new_fitness * self.selection_pressure + self.fitness * (1.0 - self.selection_pressure);
        self.fitness.min(1.0)
    }

    /// Evolves for multiple epochs.
    ///
    /// # Errors
    /// Returns [`CognitionError::InvalidState`] if `epochs` is zero.
    pub fn evolve_epochs(&mut self, epochs: usize) -> Result<f64, CognitionError> {
        if epochs == 0 {
            return Err(CognitionError::InvalidState(
                "epochs must be greater than zero".to_string(),
            ));
        }
        let mut final_fitness = self.fitness;
        for _ in 0..epochs {
            final_fitness = self.evolve();
        }
        Ok(final_fitness)
    }

    /// Returns the current fitness score.
    pub fn fitness(&self) -> f64 {
        self.fitness
    }

    /// Determines whether the concept has evolved sufficiently to
    /// be considered a new structural variant.
    pub fn has_diverged(&self, baseline_fitness: f64) -> bool {
        (self.fitness - baseline_fitness).abs() > self.selection_pressure
    }

    /// Updates the selection pressure.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if pressure is outside [0.0, 1.0].
    pub fn set_selection_pressure(&mut self, pressure: f64) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&pressure) {
            return Err(CognitionError::OutOfRange {
                field: "selection_pressure".to_string(),
                value: pressure,
                min: 0.0,
                max: 1.0,
            });
        }
        self.selection_pressure = pressure;
        Ok(())
    }

    /// Validates all field values are within valid ranges.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&self.evolution_speed) {
            return Err(CognitionError::OutOfRange {
                field: "evolution_speed".to_string(),
                value: self.evolution_speed,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&self.selection_pressure) {
            return Err(CognitionError::OutOfRange {
                field: "selection_pressure".to_string(),
                value: self.selection_pressure,
                min: 0.0,
                max: 1.0,
            });
        }
        if self.fitness < 0.0 || self.fitness > 1.0 {
            return Err(CognitionError::InvalidState(format!(
                "fitness {} out of range [0.0, 1.0]",
                self.fitness
            )));
        }
        Ok(())
    }
}

impl fmt::Debug for ConceptEvolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConceptEvolution")
            .field("evolution_speed", &self.evolution_speed)
            .field("selection_pressure", &self.selection_pressure)
            .field("fitness", &self.fitness)
            .finish()
    }
}