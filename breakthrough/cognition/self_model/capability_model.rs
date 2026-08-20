// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// CapabilityModel: The system's understanding of its own abilities.
pub struct CapabilityModel {
    pub capabilities: Vec<Capability>,
    pub proficiency: Vec<f64>,
}

/// A single capability with an identifier and capacity score.
#[derive(Debug, Clone, Copy)]
pub struct Capability {
    pub id: u64,
    pub capacity: f64,
}

impl CapabilityModel {
    /// Create a new capability model with a given number of capability slots.
    pub fn new(capacity: usize) -> Result<Self, CognitionError> {
        if capacity == 0 {
            return Err(CognitionError::InvalidState(
                "capacity must be positive".into(),
            ));
        }
        if capacity > 10_000 {
            return Err(CognitionError::CapacityExceeded {
                max: 10_000,
                attempted: capacity,
            });
        }
        Ok(Self {
            capabilities: Vec::with_capacity(capacity),
            proficiency: Vec::with_capacity(capacity),
        })
    }

    /// Register a new capability with a given capacity score.
    pub fn register_capability(&mut self, id: u64, capacity: f64) -> Result<usize, CognitionError> {
        if self.capabilities.len() >= self.capabilities.capacity() {
            return Err(CognitionError::CapacityExceeded {
                max: self.capabilities.capacity(),
                attempted: self.capabilities.len() + 1,
            });
        }
        if !(0.0..=1.0).contains(&capacity) {
            return Err(CognitionError::OutOfRange {
                field: "capacity".to_string(),
                value: capacity,
                min: 0.0,
                max: 1.0,
            });
        }
        let idx = self.capabilities.len();
        self.capabilities.push(Capability { id, capacity });
        self.proficiency.push(0.0);
        Ok(idx)
    }

    /// Record a proficiency demonstration for a capability.
    pub fn demonstrate(&mut self, id: u64, proficiency_delta: f64) -> Result<(), CognitionError> {
        let idx = self.find_capability(id)?;
        if proficiency_delta < 0.0 {
            return Err(CognitionError::OutOfRange {
                field: "proficiency_delta".to_string(),
                value: proficiency_delta,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        self.proficiency[idx] = (self.proficiency[idx] + proficiency_delta).min(1.0);
        Ok(())
    }

    /// Return the proficiency level for a given capability.
    pub fn get_proficiency(&self, id: u64) -> Result<f64, CognitionError> {
        let idx = self.find_capability(id)?;
        Ok(self.proficiency[idx])
    }

    /// Return the capacity of a given capability.
    pub fn get_capacity(&self, id: u64) -> Result<f64, CognitionError> {
        let idx = self.find_capability(id)?;
        Ok(self.capabilities[idx].capacity)
    }

    /// Remove a capability from the model.
    pub fn remove_capability(&mut self, id: u64) -> Result<(), CognitionError> {
        let idx = self.find_capability(id)?;
        self.capabilities.remove(idx);
        self.proficiency.remove(idx);
        Ok(())
    }

    /// Compute the overall capability score as the weighted average.
    pub fn overall_score(&self) -> f64 {
        if self.capabilities.is_empty() {
            return 0.0;
        }
        let total: f64 = self
            .capabilities
            .iter()
            .zip(self.proficiency.iter())
            .map(|(c, p)| c.capacity * p)
            .sum();
        let total_capacity: f64 = self.capabilities.iter().map(|c| c.capacity).sum();
        if total_capacity == 0.0 {
            0.0
        } else {
            total / total_capacity
        }
    }

    /// Find the index of a capability by its ID.
    fn find_capability(&self, id: u64) -> Result<usize, CognitionError> {
        self.capabilities
            .iter()
            .position(|c| c.id == id)
            .ok_or_else(|| {
                CognitionError::InvalidState(format!(
                    "capability {} not found",
                    id
                ))
            })
    }

    /// Return the number of registered capabilities.
    pub fn capability_count(&self) -> usize {
        self.capabilities.len()
    }

    /// Reset all proficiency scores to zero.
    pub fn reset_proficiencies(&mut self) {
        for p in &mut self.proficiency {
            *p = 0.0;
        }
    }

    /// Validate the capability model integrity.
    pub fn validate(&self) -> Result<(), CognitionError> {
        for (i, c) in self.capabilities.iter().enumerate() {
            if !(0.0..=1.0).contains(&c.capacity) {
                return Err(CognitionError::OutOfRange {
                    field: format!("capability[{}].capacity", i),
                    value: c.capacity,
                    min: 0.0,
                    max: 1.0,
                });
            }
        }
        for (i, p) in self.proficiency.iter().enumerate() {
            if !(0.0..=1.0).contains(p) {
                return Err(CognitionError::OutOfRange {
                    field: format!("proficiency[{}]", i),
                    value: *p,
                    min: 0.0,
                    max: 1.0,
                });
            }
        }
        if self.capabilities.len() != self.proficiency.len() {
            return Err(CognitionError::ConsistencyError(
                "capabilities and proficiency vectors have mismatched lengths".into(),
            ));
        }
        Ok(())
    }
}

