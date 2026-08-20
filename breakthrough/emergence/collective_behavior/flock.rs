// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FlockId(u64);

impl FlockId {
    pub fn new(id: u64) -> Self {
        FlockId(id)
    }

    pub fn inner(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for FlockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FlockId({})", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Flock {
    id: FlockId,
    members: Vec<String>,
    alignment_weight: f64,
    cohesion_weight: f64,
    separation_weight: f64,
}

impl Flock {
    pub fn new(
        id: FlockId,
        members: Vec<String>,
        alignment_weight: f64,
        cohesion_weight: f64,
        separation_weight: f64,
    ) -> crate::Result<Self> {
        if members.len() < 2 {
            return Err(crate::EmergenceError::InvalidPattern(
                "flock requires at least two members".to_string(),
            ));
        }
        let sum = alignment_weight + cohesion_weight + separation_weight;
        if (sum - 1.0).abs() > 1e-6 {
            return Err(crate::EmergenceError::InvalidPattern(
                "alignment, cohesion, and separation weights must sum to 1.0".to_string(),
            ));
        }
        Ok(Flock {
            id,
            members,
            alignment_weight,
            cohesion_weight,
            separation_weight,
        })
    }

    pub fn id(&self) -> FlockId {
        self.id
    }

    pub fn members(&self) -> &[String] {
        &self.members
    }

    pub fn alignment_weight(&self) -> f64 {
        self.alignment_weight
    }

    pub fn cohesion_weight(&self) -> f64 {
        self.cohesion_weight
    }

    pub fn separation_weight(&self) -> f64 {
        self.separation_weight
    }

    pub fn flock_step(&mut self) -> crate::Result<()> {
        let sum = self.alignment_weight + self.cohesion_weight + self.separation_weight;
        if (sum - 1.0).abs() > 1e-6 {
            return Err(crate::EmergenceError::InvalidPattern(
                "flock weights are not normalized".to_string(),
            ));
        }
        Ok(())
    }

    pub fn validate(&self) -> crate::Result<()> {
        if self.members.len() < 2 {
            return Err(crate::EmergenceError::InvalidPattern(
                "flock has fewer than two members".to_string(),
            ));
        }
        let sum = self.alignment_weight + self.cohesion_weight + self.separation_weight;
        if (sum - 1.0).abs() > 1e-6 {
            return Err(crate::EmergenceError::InvalidPattern(
                format!("weights do not sum to 1.0: {}", sum),
            ));
        }
        Ok(())
    }
}

impl fmt::Display for Flock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Flock(id={}, members={}, alignment={:.3}, cohesion={:.3}, separation={:.3})",
            self.id,
            self.members.len(),
            self.alignment_weight,
            self.cohesion_weight,
            self.separation_weight
        )
    }
}
