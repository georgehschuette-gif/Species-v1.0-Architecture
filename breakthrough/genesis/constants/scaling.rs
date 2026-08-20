// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::genesis::{GenesisError, GenesisResult};

/// ScalingConstants: Parameters that govern how the ecosystem scales
/// from micro to macro levels.
///
/// These constants define the capacity and granularity limits of the
/// cognitive ecosystem, ensuring that the system remains computationally
/// tractable and structurally coherent as it grows.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScalingConstants;

impl ScalingConstants {
    /// The maximum number of entities the ecosystem can contain.
    pub const MAX_ENTITIES: usize = 1_000_000;

    /// The maximum number of connections (edges) any single entity
    /// can maintain.
    pub const MAX_CONNECTIONS_PER_ENTITY: usize = 1000;

    /// The granularity of the cognitive field, determining the
    /// resolution at which field values are discretized.
    pub const FIELD_GRANULARITY: f64 = 0.001;

    /// Maximum allowed ratio of edges to entities before the graph
    /// is considered over-connected.
    pub const MAX_EDGE_RATIO: f64 = 10.0;

    /// Returns the maximum total number of edges allowed in the system
    /// based on the per-entity connection limit.
    pub fn max_total_edges() -> usize {
        Self::MAX_ENTITIES * Self::MAX_CONNECTIONS_PER_ENTITY
    }

    /// Checks whether the given entity count is within the allowed limit.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::CapacityExceeded`] if `entity_count`
    /// exceeds `MAX_ENTITIES`.
    pub fn validate_entity_count(entity_count: usize) -> GenesisResult<()> {
        if entity_count > Self::MAX_ENTITIES {
            return Err(GenesisError::CapacityExceeded {
                max: Self::MAX_ENTITIES,
                attempted: entity_count,
            });
        }
        Ok(())
    }

    /// Checks whether the given connection count for a specific entity
    /// is within the allowed per-entity limit.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::CapacityExceeded`] if `connection_count`
    /// exceeds `MAX_CONNECTIONS_PER_ENTITY`.
    pub fn validate_connection_count(
        connection_count: usize,
    ) -> GenesisResult<()> {
        if connection_count > Self::MAX_CONNECTIONS_PER_ENTITY {
            return Err(GenesisError::CapacityExceeded {
                max: Self::MAX_CONNECTIONS_PER_ENTITY,
                attempted: connection_count,
            });
        }
        Ok(())
    }

    /// Computes the edge-to-entity ratio for a given graph size,
    /// returning whether the graph is within acceptable density bounds.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if the ratio exceeds the
    /// maximum allowed edge ratio.
    pub fn validate_density(
        entity_count: usize,
        edge_count: usize,
    ) -> GenesisResult<()> {
        if entity_count == 0 {
            return Err(GenesisError::MissingInput(
                "entity count must be positive".to_string(),
            ));
        }
        let ratio = edge_count as f64 / entity_count as f64;
        if ratio > Self::MAX_EDGE_RATIO {
            return Err(GenesisError::OutOfRange {
                field: "edge_ratio".to_string(),
                value: ratio,
                min: 0.0,
                max: Self::MAX_EDGE_RATIO,
            });
        }
        Ok(())
    }

    /// Computes a scaled field resolution based on the number of
    /// entities, ensuring adequate resolution for large ecosystems.
    pub fn adaptive_resolution(entity_count: usize) -> f64 {
        let base = Self::FIELD_GRANULARITY;
        let scale = (entity_count as f64 / Self::MAX_ENTITIES as f64).sqrt();
        base * (1.0 + scale)
    }

    /// Validates all scaling constants are positive and non-zero.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::ValidationFailure`] if any constant
    /// is zero or negative.
    pub fn validate() -> GenesisResult<()> {
        if Self::MAX_ENTITIES == 0 {
            return Err(GenesisError::ValidationFailure(
                "MAX_ENTITIES must be positive".to_string(),
            ));
        }
        if Self::MAX_CONNECTIONS_PER_ENTITY == 0 {
            return Err(GenesisError::ValidationFailure(
                "MAX_CONNECTIONS_PER_ENTITY must be positive".to_string(),
            ));
        }
        if Self::FIELD_GRANULARITY <= 0.0 {
            return Err(GenesisError::ValidationFailure(
                "FIELD_GRANULARITY must be positive".to_string(),
            ));
        }
        Ok(())
    }
}

impl Default for ScalingConstants {
    fn default() -> Self {
        Self
    }
}