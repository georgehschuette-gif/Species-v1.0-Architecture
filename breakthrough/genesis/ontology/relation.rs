// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::genesis::{GenesisError, GenesisResult};

/// Relation: A directed connection between two entities.
///
/// Relations carry a weight that indicates the strength of the
/// connection and a type that categorizes the nature of the
/// relationship between the source and target entities.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Relation {
    /// The source entity of this relation.
    pub source: u64,
    /// The target entity of this relation.
    pub target: u64,
    /// The strength of this connection, normalized to [0.0, 1.0].
    pub weight: f64,
    /// The type of relationship between source and target.
    pub relation_type: RelationType,
}

/// The category of relationship between two entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationType {
    /// A causal relationship where the source influences the target.
    Causal,
    /// A resonant relationship where entities amplify each other.
    Resonant,
    /// An inhibitory relationship where the source suppresses the target.
    Inhibitory,
    /// A facilitative relationship where the source enables the target.
    Facilitative,
}

impl Relation {
    /// The minimum valid weight for a relation.
    pub const MIN_WEIGHT: f64 = 0.0;

    /// The maximum valid weight for a relation.
    pub const MAX_WEIGHT: f64 = 1.0;

    /// Creates a new relation with normalized weight.
    ///
    /// The weight is clamped to the valid range [0.0, 1.0].
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if either `source` or `target`
    /// is zero.
    pub fn new(source: u64, target: u64, weight: f64, relation_type: RelationType) -> GenesisResult<Self> {
        if source == 0 {
            return Err(GenesisError::OutOfRange {
                field: "source".to_string(),
                value: source as f64,
                min: 1.0,
                max: u64::MAX as f64,
            });
        }
        if target == 0 {
            return Err(GenesisError::OutOfRange {
                field: "target".to_string(),
                value: target as f64,
                min: 1.0,
                max: u64::MAX as f64,
            });
        }
        if weight.is_nan() {
            return Err(GenesisError::ComputationError(
                "relation weight cannot be NaN".to_string(),
            ));
        }
        let clamped_weight = weight.clamp(Self::MIN_WEIGHT, Self::MAX_WEIGHT);
        Ok(Self {
            source,
            target,
            weight: clamped_weight,
            relation_type,
        })
    }

    /// Returns whether this relation is considered strong
    /// (weight above the resonance threshold).
    pub fn is_strong(&self) -> bool {
        self.weight > crate::genesis::constants::ThresholdConstants::RESONANCE_THRESHOLD
    }

    /// Returns whether this relation is considered weak
    /// (weight below the activation threshold).
    pub fn is_weak(&self) -> bool {
        self.weight < crate::genesis::constants::ThresholdConstants::ACTIVATION_THRESHOLD
    }

    /// Normalizes the weight of this relation to the valid range [0.0, 1.0].
    /// This is a no-op if the weight is already within range.
    pub fn weight_normalize(&mut self) {
        self.weight = self.weight.clamp(Self::MIN_WEIGHT, Self::MAX_WEIGHT);
    }

    /// Returns the inverse of this relation (source and target swapped).
    pub fn inverse(&self) -> Self {
        Self {
            source: self.target,
            target: self.source,
            weight: self.weight,
            relation_type: self.relation_type,
        }
    }

    /// Validates the relation, checking that source and target are non-zero
    /// and that the weight is within valid bounds.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if source or target is zero,
    /// or if weight is out of [0.0, 1.0].
    pub fn validate(&self) -> GenesisResult<()> {
        if self.source == 0 {
            return Err(GenesisError::OutOfRange {
                field: "source".to_string(),
                value: self.source as f64,
                min: 1.0,
                max: u64::MAX as f64,
            });
        }
        if self.target == 0 {
            return Err(GenesisError::OutOfRange {
                field: "target".to_string(),
                value: self.target as f64,
                min: 1.0,
                max: u64::MAX as f64,
            });
        }
        if self.weight < Self::MIN_WEIGHT || self.weight > Self::MAX_WEIGHT {
            return Err(GenesisError::OutOfRange {
                field: "weight".to_string(),
                value: self.weight,
                min: Self::MIN_WEIGHT,
                max: Self::MAX_WEIGHT,
            });
        }
        Ok(())
    }

    /// Returns a string representation of the relation type.
    pub fn type_label(&self) -> &'static str {
        match self.relation_type {
            RelationType::Causal => "causal",
            RelationType::Resonant => "resonant",
            RelationType::Inhibitory => "inhibitory",
            RelationType::Facilitative => "facilitative",
        }
    }
}

impl Default for Relation {
    fn default() -> Self {
        Self {
            source: 0,
            target: 0,
            weight: 0.0,
            relation_type: RelationType::Causal,
        }
    }
}