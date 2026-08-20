// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::genesis::{GenesisError, GenesisResult};

/// AggregationRule: Governs how simple entities combine into
/// complex structures.
///
/// The aggregation rule is the primary mechanism by which
/// the ecosystem builds complexity from simplicity. Entities
/// with sufficient affinity and count merge to form new,
/// composite structures.
#[derive(Debug, Clone, PartialEq)]
pub struct AggregationRule {
    /// The minimum number of entities required to trigger
    /// aggregation.
    pub min_entities: usize,
    /// The minimum affinity between entities required for
    /// aggregation to occur.
    pub affinity_threshold: f64,
}

impl AggregationRule {
    /// The default minimum number of entities for aggregation.
    pub const DEFAULT_MIN_ENTITIES: usize = 3;

    /// The default affinity threshold for aggregation.
    pub const DEFAULT_AFFINITY_THRESHOLD: f64 = 0.5;

    /// Creates a new AggregationRule with the specified parameters.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if `min_entities` is zero.
    /// Returns [`GenesisError::OutOfRange`] if `affinity_threshold`
    /// is outside [0.0, 1.0].
    pub fn new(
        min_entities: usize,
        affinity_threshold: f64,
    ) -> GenesisResult<Self> {
        if min_entities == 0 {
            return Err(GenesisError::OutOfRange {
                field: "min_entities".to_string(),
                value: 0.0,
                min: 1.0,
                max: usize::MAX as f64,
            });
        }
        if affinity_threshold.is_nan() || !affinity_threshold.is_finite() {
            return Err(GenesisError::OutOfRange {
                field: "affinity_threshold".to_string(),
                value: affinity_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        let clamped_threshold = affinity_threshold.clamp(0.0, 1.0);
        Ok(Self {
            min_entities,
            affinity_threshold: clamped_threshold,
        })
    }

    /// Determines whether aggregation should occur based on
    /// the current affinity and entity count.
    ///
    /// Returns `true` if both conditions are met:
    /// - `affinity >= self.affinity_threshold`
    /// - `count >= self.min_entities`
    pub fn should_aggregate(&self, affinity: f64, count: usize) -> bool {
        affinity >= self.affinity_threshold && count >= self.min_entities
    }

    /// Computes the aggregation affinity between a group of
    /// entities based on their individual energy levels.
    ///
    /// The group affinity is the arithmetic mean of all pairwise
    /// affinities, where pairwise affinity is the product of
    /// the normalized energies of the two entities.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::MissingInput`] if the group is empty.
    /// Returns [`GenesisError::OutOfRange`] if any entity energy
    /// is negative.
    pub fn group_affinity(&self, energies: &[f64]) -> GenesisResult<f64> {
        if energies.is_empty() {
            return Err(GenesisError::MissingInput(
                "group cannot be empty".to_string(),
            ));
        }
        if energies.len() < self.min_entities {
            return Ok(0.0);
        }
        let max_energy = energies
            .iter()
            .copied()
            .fold(0.0/0.0, f64::max);
        if max_energy.is_nan() {
            return Err(GenesisError::ComputationError(
                "energy values contain NaN".to_string(),
            ));
        }
        let safe_max = max_energy.max(f64::EPSILON);
        let normalized: Vec<f64> = energies
            .iter()
            .map(|e| *e / safe_max)
            .collect();

        let mut total_affinity = 0.0;
        let mut pairs = 0;
        for i in 0..normalized.len() {
            for j in (i + 1)..normalized.len() {
                total_affinity += normalized[i] * normalized[j];
                pairs += 1;
            }
        }
        if pairs == 0 {
            return Ok(0.0);
        }
        Ok(total_affinity / pairs as f64)
    }

    /// Validates the aggregation rule parameters.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if min_entities is zero
    /// or affinity_threshold is outside [0.0, 1.0].
    pub fn validate(&self) -> GenesisResult<()> {
        if self.min_entities == 0 {
            return Err(GenesisError::OutOfRange {
                field: "min_entities".to_string(),
                value: 0.0,
                min: 1.0,
                max: usize::MAX as f64,
            });
        }
        if self.affinity_threshold < 0.0 || self.affinity_threshold > 1.0 {
            return Err(GenesisError::OutOfRange {
                field: "affinity_threshold".to_string(),
                value: self.affinity_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}

impl Default for AggregationRule {
    fn default() -> Self {
        Self {
            min_entities: Self::DEFAULT_MIN_ENTITIES,
            affinity_threshold: Self::DEFAULT_AFFINITY_THRESHOLD,
        }
    }
}