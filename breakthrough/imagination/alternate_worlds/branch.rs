// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::ImaginationError;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WorldConsistency {
    Consistent,
    Tenuous,
    Collapsed,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WorldType {
    Divergent,
    Convergent,
    Parallel,
    Nested,
}

pub struct AlternateWorld {
    pub id: u64,
    pub divergence: f64,
    pub properties: Vec<WorldProperty>,
    pub consistency: WorldConsistency,
    pub world_type: WorldType,
    pub coherence: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorldProperty {
    pub name: String,
    pub value: f64,
    pub certainty: f64,
}

impl AlternateWorld {
    pub fn new(id: u64, divergence: f64) -> Self {
        Self {
            id,
            divergence,
            properties: Vec::new(),
            consistency: WorldConsistency::Consistent,
            world_type: WorldType::Divergent,
            coherence: 1.0,
        }
    }

    pub fn id(&self) -> u64 { self.id }
    pub fn divergence(&self) -> f64 { self.divergence }
    pub fn properties(&self) -> &[WorldProperty] { &self.properties }
    pub fn consistency(&self) -> WorldConsistency { self.consistency }
    pub fn world_type(&self) -> WorldType { self.world_type }
    pub fn coherence(&self) -> f64 { self.coherence }

    pub fn add_property(&mut self, name: impl Into<String>, value: f64, certainty: f64) -> Result<(), ImaginationError> {
        if !(0.0..=1.0).contains(&certainty) {
            return Err(ImaginationError::OutOfRange { field: "certainty".into(), value: certainty, min: 0.0, max: 1.0 });
        }
        self.properties.push(WorldProperty { name: name.into(), value, certainty });
        self.recalculate_coherence();
        Ok(())
    }

    pub fn set_world_type(&mut self, world_type: WorldType) {
        self.world_type = world_type;
    }

    pub fn diverge(&mut self, amount: f64) -> Result<(), ImaginationError> {
        self.divergence = (self.divergence + amount).clamp(0.0, 1.0);
        if self.divergence > 0.8 {
            self.consistency = WorldConsistency::Tenuous;
            self.coherence *= 0.8;
        }
        if self.divergence > 0.95 {
            self.consistency = WorldConsistency::Collapsed;
            self.coherence = 0.0;
        }
        Ok(())
    }

    pub fn stabilize(&mut self) {
        self.divergence = (self.divergence * 0.5).max(0.0);
        self.consistency = WorldConsistency::Consistent;
        self.recalculate_coherence();
    }

    pub fn merge_with(&mut self, other: &AlternateWorld) -> Result<(), ImaginationError> {
        if self.id == other.id {
            return Err(ImaginationError::Inconsistency { detail: "cannot merge world with itself".into() });
        }
        self.divergence = (self.divergence + other.divergence) / 2.0;
        self.coherence = (self.coherence + other.coherence) / 2.0;
        for prop in &other.properties {
            if !self.properties.iter().any(|p| p.name == prop.name) {
                self.properties.push(prop.clone());
            }
        }
        self.recalculate_coherence();
        Ok(())
    }

    fn recalculate_coherence(&mut self) {
        if self.properties.is_empty() {
            self.coherence = 1.0;
            return;
        }
        let total_certainty: f64 = self.properties.iter().map(|p| p.certainty).sum();
        self.coherence = (total_certainty / self.properties.len() as f64).clamp(0.0, 1.0);
        if self.coherence < 0.3 {
            self.consistency = WorldConsistency::Tenuous;
        }
    }

    pub fn validate(&self) -> Result<(), ImaginationError> {
        if !(0.0..=1.0).contains(&self.divergence) {
            return Err(ImaginationError::OutOfRange { field: "divergence".into(), value: self.divergence, min: 0.0, max: 1.0 });
        }
        if !(0.0..=1.0).contains(&self.coherence) {
            return Err(ImaginationError::OutOfRange { field: "coherence".into(), value: self.coherence, min: 0.0, max: 1.0 });
        }
        Ok(())
    }
}

impl fmt::Display for AlternateWorld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AlternateWorld(id={}, divergence={:.2}, coherence={:.2}, type={:?})", self.id, self.divergence, self.coherence, self.world_type)
    }
}

impl fmt::Display for WorldConsistency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Consistent => write!(f, "Consistent"),
            Self::Tenuous => write!(f, "Tenuous"),
            Self::Collapsed => write!(f, "Collapsed"),
        }
    }
}
