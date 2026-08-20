// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// ConceptIntegration: Connecting a concept to broader knowledge structures.
///
/// Integration anchors a concept within a larger web of related concepts,
/// increasing its robustness and accessibility. Each connection strengthens
/// the concept's integration with the broader knowledge network.
///
/// # Fields
/// - `integration_strength`: Base strength of each integration link, in [0.0, 1.0].
/// - `connection_count`: Number of connections to the broader knowledge network.
///
/// # Example
/// ```
/// use breakthrough::cognition::concept_growth::ConceptIntegration;
///
/// let mut integrator = ConceptIntegration::new(0.5, 3).expect("valid parameters");
/// integrator.add_connection();
/// assert_eq!(integrator.connections(), 4);
/// ```
pub struct ConceptIntegration {
    /// Base strength of each integration link, in [0.0, 1.0].
    pub integration_strength: f64,
    /// Number of connections to the broader knowledge network.
    pub connection_count: usize,
    /// Maximum allowed connections.
    max_connections: usize,
}

impl ConceptIntegration {
    /// Creates a new `ConceptIntegration` with the given base strength and connection count.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `integration_strength` is outside [0.0, 1.0]
    /// or `max_connections` is zero.
    pub fn new(integration_strength: f64, connection_count: usize) -> Result<Self, CognitionError> {
        if !(0.0..=1.0).contains(&integration_strength) {
            return Err(CognitionError::OutOfRange {
                field: "integration_strength".to_string(),
                value: integration_strength,
                min: 0.0,
                max: 1.0,
            });
        }
        if connection_count > 10_000 {
            return Err(CognitionError::OutOfRange {
                field: "connection_count".to_string(),
                value: connection_count as f64,
                min: 0.0,
                max: 10_000.0,
            });
        }
        Ok(Self {
            integration_strength,
            connection_count,
            max_connections: 10_000,
        })
    }

    /// Adds a new connection to the knowledge network, up to the maximum.
    ///
    /// # Errors
    /// Returns [`CognitionError::CapacityExceeded`] if the maximum number of connections is reached.
    pub fn add_connection(&mut self) -> Result<(), CognitionError> {
        if self.connection_count >= self.max_connections {
            return Err(CognitionError::CapacityExceeded {
                max: self.max_connections,
                attempted: self.connection_count + 1,
            });
        }
        self.connection_count += 1;
        Ok(())
    }

    /// Adds multiple connections at once.
    ///
    /// # Errors
    /// Returns [`CognitionError::CapacityExceeded`] if adding `count` connections would exceed the limit.
    pub fn add_connections(&mut self, count: usize) -> Result<(), CognitionError> {
        let new_count = self.connection_count + count;
        if new_count > self.max_connections {
            return Err(CognitionError::CapacityExceeded {
                max: self.max_connections,
                attempted: new_count,
            });
        }
        self.connection_count = new_count;
        Ok(())
    }

    /// Removes a connection if any exist.
    ///
    /// Returns `true` if a connection was removed, `false` if there were none.
    pub fn remove_connection(&mut self) -> bool {
        if self.connection_count > 0 {
            self.connection_count -= 1;
            true
        } else {
            false
        }
    }

    /// Returns the total effective integration strength, computed as
    /// `integration_strength * connection_count`.
    pub fn total_strength(&self) -> f64 {
        self.integration_strength * self.connection_count as f64
    }

    /// Returns the current number of connections.
    pub fn connections(&self) -> usize {
        self.connection_count
    }

    /// Validates all field values are within valid ranges.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] for invalid strength or connection count.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&self.integration_strength) {
            return Err(CognitionError::OutOfRange {
                field: "integration_strength".to_string(),
                value: self.integration_strength,
                min: 0.0,
                max: 1.0,
            });
        }
        if self.connection_count > self.max_connections {
            return Err(CognitionError::OutOfRange {
                field: "connection_count".to_string(),
                value: self.connection_count as f64,
                min: 0.0,
                max: self.max_connections as f64,
            });
        }
        Ok(())
    }
}

impl fmt::Debug for ConceptIntegration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConceptIntegration")
            .field("integration_strength", &self.integration_strength)
            .field("connection_count", &self.connection_count)
            .finish()
    }
}