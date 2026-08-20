// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::genesis::{GenesisError, GenesisResult};

/// Category: A class of entities sharing fundamental properties.
///
/// Categories organize entities along conceptual dimensions, enabling
/// structured classification and retrieval within the cognitive ecosystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Category {
    /// Unique identifier for this category.
    pub id: u64,
    /// The dimension along which this category operates.
    pub dimension: CategoryDimension,
}

/// The fundamental dimensions along which categories can be organized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CategoryDimension {
    /// Perceptual categories relate to sensory experience.
    Perceptual,
    /// Conceptual categories relate to abstract thought.
    Conceptual,
    /// Affective categories relate to emotional valence.
    Affective,
    /// Volitional categories relate to will and intention.
    Volitional,
}

impl Category {
    /// Creates a new category with the given id and dimension.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if `id` is zero.
    pub fn new(id: u64, dimension: CategoryDimension) -> GenesisResult<Self> {
        if id == 0 {
            return Err(GenesisError::OutOfRange {
                field: "id".to_string(),
                value: 0.0,
                min: 1.0,
                max: u64::MAX as f64,
            });
        }
        Ok(Self { id, dimension })
    }

    /// Returns a human-readable label for the category dimension.
    pub fn dimension_label(&self) -> &'static str {
        match self.dimension {
            CategoryDimension::Perceptual => "perceptual",
            CategoryDimension::Conceptual => "conceptual",
            CategoryDimension::Affective => "affective",
            CategoryDimension::Volitional => "volitional",
        }
    }

    /// Classifies an entity's energy level against a dimension-specific
    /// activation threshold.
    ///
    /// Returns `true` if the entity's energy meets or exceeds the
    /// activation threshold for this category's dimension.
    pub fn classify(&self, entity_energy: f64) -> bool {
        let threshold = match self.dimension {
            CategoryDimension::Perceptual => 0.3,
            CategoryDimension::Conceptual => 0.5,
            CategoryDimension::Affective => 0.4,
            CategoryDimension::Volitional => 0.6,
        };
        entity_energy >= threshold
    }

    /// Validates the category, ensuring the id is non-zero and the
    /// dimension is well-formed.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if the id is zero.
    pub fn validate(&self) -> GenesisResult<()> {
        if self.id == 0 {
            return Err(GenesisError::OutOfRange {
                field: "id".to_string(),
                value: 0.0,
                min: 1.0,
                max: u64::MAX as f64,
            });
        }
        Ok(())
    }
}

impl Default for Category {
    fn default() -> Self {
        Self {
            id: 0,
            dimension: CategoryDimension::Conceptual,
        }
    }
}

impl Default for CategoryDimension {
    fn default() -> Self {
        CategoryDimension::Conceptual
    }
}