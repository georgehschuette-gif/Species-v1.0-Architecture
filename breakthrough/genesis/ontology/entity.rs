// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::genesis::{GenesisError, GenesisResult};

/// Entity: A discrete cognitive unit within the ecosystem.
///
/// Each entity possesses a unique identifier, an energy level,
/// and a mutable state that governs its behavior and interactions
/// within the cognitive topology.
#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    /// Unique identifier for this entity.
    pub id: u64,
    /// Current energy level of this entity.
    pub energy: f64,
    /// Current state of this entity.
    pub state: EntityState,
}

/// The possible states an entity can occupy within the ecosystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityState {
    /// Entity exists but is not actively participating.
    Dormant,
    /// Entity is actively processing and contributing.
    Active,
    /// Entity is resonating, amplifying its influence.
    Resonant,
    /// Entity has been dissolved and is no longer active.
    Dissolved,
}

impl Entity {
    /// The default energy assigned to a newly created entity.
    pub const DEFAULT_ENERGY: f64 = 1.0;

    /// Creates a new entity with the given identifier, default energy,
    /// and dormant state.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if `id` is zero.
    pub fn new(id: u64) -> GenesisResult<Self> {
        if id == 0 {
            return Err(GenesisError::OutOfRange {
                field: "id".to_string(),
                value: 0.0,
                min: 1.0,
                max: u64::MAX as f64,
            });
        }
        Ok(Self {
            id,
            energy: Self::DEFAULT_ENERGY,
            state: EntityState::Dormant,
        })
    }

    /// Creates a new entity with specified energy and state.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if `id` is zero or `energy`
    /// is negative.
    pub fn with_energy(id: u64, energy: f64) -> GenesisResult<Self> {
        if id == 0 {
            return Err(GenesisError::OutOfRange {
                field: "id".to_string(),
                value: 0.0,
                min: 1.0,
                max: u64::MAX as f64,
            });
        }
        if energy < 0.0 {
            return Err(GenesisError::OutOfRange {
                field: "energy".to_string(),
                value: energy,
                min: 0.0,
                max: f64::MAX,
            });
        }
        Ok(Self {
            id,
            energy,
            state: EntityState::Dormant,
        })
    }

    /// Activates the entity, transitioning from Dormant to Active.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::InvalidState`] if the entity is not
    /// in Dormant state.
    pub fn activate(&mut self) -> GenesisResult<()> {
        match self.state {
            EntityState::Dormant => {
                self.state = EntityState::Active;
                Ok(())
            }
            EntityState::Dissolved => Err(GenesisError::InvalidState(
                "dissolved entities cannot be activated".to_string(),
            )),
            _ => Err(GenesisError::InvalidState(format!(
                "entity {} is already active or resonant",
                self.id
            ))),
        }
    }

    /// Transitions the entity to Resonant state, amplifying its influence.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::InvalidState`] if the entity is not Active.
    pub fn resonate(&mut self) -> GenesisResult<()> {
        match self.state {
            EntityState::Active => {
                self.state = EntityState::Resonant;
                Ok(())
            }
            _ => Err(GenesisError::InvalidState(format!(
                "entity {} must be Active to become Resonant (current: {:?})",
                self.id, self.state
            ))),
        }
    }

    /// Dissolves the entity, marking it as no longer active.
    pub fn dissolve(&mut self) {
        self.state = EntityState::Dissolved;
        self.energy = 0.0;
    }

    /// Returns whether the entity is currently Active or Resonant.
    pub fn is_active(&self) -> bool {
        matches!(
            self.state,
            EntityState::Active | EntityState::Resonant
        )
    }

    /// Returns whether the entity is in the Dissolved state.
    pub fn is_dissolved(&self) -> bool {
        self.state == EntityState::Dissolved
    }

    /// Returns the energy contribution of this entity to the ecosystem.
    ///
    /// Dissolved entities contribute zero energy.
    pub fn energy_contribution(&self) -> f64 {
        match self.state {
            EntityState::Dissolved => 0.0,
            _ => self.energy,
        }
    }

    /// Increases the entity's energy by the given amount.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if the resulting energy
    /// would be negative or the increment itself is invalid.
    pub fn add_energy(&mut self, delta: f64) -> GenesisResult<()> {
        if delta.is_nan() || delta.is_infinite() {
            return Err(GenesisError::ComputationError(
                "energy delta cannot be NaN or infinite".to_string(),
            ));
        }
        let new_energy = self.energy + delta;
        if new_energy < 0.0 {
            return Err(GenesisError::OutOfRange {
                field: "energy".to_string(),
                value: new_energy,
                min: 0.0,
                max: f64::MAX,
            });
        }
        self.energy = new_energy;
        Ok(())
    }

    /// Validates the entity, ensuring it has a valid id, non-negative
    /// energy (if not dissolved), and a consistent state.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if the id is zero.
    /// Returns [`GenesisError::OutOfRange`] if energy is negative.
    /// Returns [`GenesisError::InvalidState`] if a dissolved entity has
    /// non-zero energy.
    pub fn validate(&self) -> GenesisResult<()> {
        if self.id == 0 {
            return Err(GenesisError::OutOfRange {
                field: "id".to_string(),
                value: 0.0,
                min: 1.0,
                max: u64::MAX as f64,
            });
        }
        if self.energy < 0.0 {
            return Err(GenesisError::OutOfRange {
                field: "energy".to_string(),
                value: self.energy,
                min: 0.0,
                max: f64::MAX,
            });
        }
        if self.state == EntityState::Dissolved && self.energy > 0.0 {
            return Err(GenesisError::InvalidState(format!(
                "dissolved entity {} should have zero energy (has {})",
                self.id, self.energy
            )));
        }
        Ok(())
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            id: 0,
            energy: 0.0,
            state: EntityState::Dormant,
        }
    }
}

impl Default for EntityState {
    fn default() -> Self {
        EntityState::Dormant
    }
}