// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::genesis::{GenesisError, GenesisResult};
use crate::genesis::ontology::{Entity, CognitiveField};

/// InitialState: The starting configuration of the ecosystem at t=0.
///
/// The initial state defines the entities, the cognitive field,
/// and the time at which the ecosystem begins its evolution.
/// It serves as the input to the bootstrap process.
#[derive(Debug, Clone, PartialEq)]
pub struct InitialState {
    /// The entities present at the start of the ecosystem.
    pub entities: Vec<Entity>,
    /// The cognitive field at the start of the ecosystem.
    pub field: CognitiveField,
    /// The initial time, typically 0.0.
    pub time: f64,
}

impl InitialState {
    /// Creates a new initial state with the given entities and field,
    /// setting time to 0.0.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if the field is
    /// invalid.
    /// Returns [`GenesisError::MissingInput`] if the entities
    /// list is empty and the field has no values.
    pub fn new(
        entities: Vec<Entity>,
        field: CognitiveField,
    ) -> GenesisResult<Self> {
        field.validate()?;
        if entities.is_empty() && field.size() == 0 {
            return Err(GenesisError::MissingInput(
                "initial state must have at least one entity or field values".to_string(),
            ));
        }
        for entity in &entities {
            entity.validate()?;
        }
        Ok(Self {
            entities,
            field,
            time: 0.0,
        })
    }

    /// Advances the state by one time step, applying decay
    /// to all entity energies.
    ///
    /// Returns a new state with damped energies.
    pub fn advance_time(&self) -> GenesisResult<Self> {
        let new_entities: Vec<Entity> = self
            .entities
            .iter()
            .map(|e| {
                let new_energy =
                    e.energy * (1.0 - crate::genesis::constants::ThresholdConstants::DECAY_RATE);
                let mut new_e = e.clone();
                new_e.energy = new_energy.max(0.0);
                if new_e.is_dissolved() {
                    Ok(new_e)
                } else {
                    new_e.validate().map(|_| new_e)
                }
            })
            .collect::<GenesisResult<Vec<_>>>()?;
        Ok(Self {
            entities: new_entities,
            field: self.field.clone(),
            time: self.time + 1.0,
        })
    }

    /// Returns the total energy of all entities in the state.
    pub fn total_energy(&self) -> f64 {
        self.entities
            .iter()
            .map(|e| e.energy_contribution())
            .sum()
    }

    /// Validates the initial state, ensuring all entities
    /// are valid, the field is consistent, and time is
    /// non-negative.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if time is negative
    /// or any entity fails validation.
    /// Returns [`GenesisError::InvalidState`] if the field
    /// is inconsistent.
    pub fn validate(&self) -> GenesisResult<()> {
        if self.time < 0.0 {
            return Err(GenesisError::OutOfRange {
                field: "time".to_string(),
                value: self.time,
                min: 0.0,
                max: f64::MAX,
            });
        }
        for entity in &self.entities {
            entity.validate()?;
        }
        self.field.validate()?;
        Ok(())
    }

    /// Checks whether the initial state is ready for
    /// bootstrapping. A ready state has at least one entity
    /// and a valid field.
    pub fn is_ready(&self) -> bool {
        !self.entities.is_empty() && self.field.validate().is_ok()
    }
}

impl Default for InitialState {
    fn default() -> Self {
        Self {
            entities: Vec::new(),
            field: CognitiveField::default(),
            time: 0.0,
        }
    }
}