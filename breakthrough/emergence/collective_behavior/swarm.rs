// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SwarmId(u64);

impl SwarmId {
    pub fn new(id: u64) -> Self {
        SwarmId(id)
    }

    pub fn inner(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for SwarmId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SwarmId({})", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Swarm {
    id: SwarmId,
    agents: Vec<String>,
    cohesion: f64,
    alignment: f64,
}

impl Swarm {
    pub fn new(id: SwarmId, agents: Vec<String>, cohesion: f64, alignment: f64) -> crate::Result<Self> {
        if agents.len() < 2 {
            return Err(crate::EmergenceError::InvalidPattern(
                "swarm requires at least two agents".to_string(),
            ));
        }
        if !(0.0..=1.0).contains(&cohesion) {
            return Err(crate::EmergenceError::InvalidPattern(
                "cohesion must be between 0.0 and 1.0".to_string(),
            ));
        }
        if !(0.0..=1.0).contains(&alignment) {
            return Err(crate::EmergenceError::InvalidPattern(
                "alignment must be between 0.0 and 1.0".to_string(),
            ));
        }
        Ok(Swarm {
            id,
            agents,
            cohesion,
            alignment,
        })
    }

    pub fn id(&self) -> SwarmId {
        self.id
    }

    pub fn agents(&self) -> &[String] {
        &self.agents
    }

    pub fn cohesion(&self) -> f64 {
        self.cohesion
    }

    pub fn alignment(&self) -> f64 {
        self.alignment
    }

    pub fn update(&mut self, new_cohesion: f64, new_alignment: f64) -> crate::Result<()> {
        if !(0.0..=1.0).contains(&new_cohesion) {
            return Err(crate::EmergenceError::InvalidPattern(
                "cohesion must be between 0.0 and 1.0".to_string(),
            ));
        }
        if !(0.0..=1.0).contains(&new_alignment) {
            return Err(crate::EmergenceError::InvalidPattern(
                "alignment must be between 0.0 and 1.0".to_string(),
            ));
        }
        self.cohesion = new_cohesion;
        self.alignment = new_alignment;
        Ok(())
    }

    pub fn validate(&self) -> crate::Result<()> {
        if self.agents.len() < 2 {
            return Err(crate::EmergenceError::InvalidPattern(
                "swarm has fewer than two agents".to_string(),
            ));
        }
        if !(0.0..=1.0).contains(&self.cohesion) {
            return Err(crate::EmergenceError::InvalidPattern(
                format!("invalid cohesion: {}", self.cohesion),
            ));
        }
        if !(0.0..=1.0).contains(&self.alignment) {
            return Err(crate::EmergenceError::InvalidPattern(
                format!("invalid alignment: {}", self.alignment),
            ));
        }
        Ok(())
    }
}

impl fmt::Display for Swarm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Swarm(id={}, agents={}, cohesion={:.3}, alignment={:.3})",
            self.id,
            self.agents.len(),
            self.cohesion,
            self.alignment
        )
    }
}
