// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::pattern::Pattern;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FormationId(u64);

impl FormationId {
    pub fn new(id: u64) -> Self {
        FormationId(id)
    }

    pub fn inner(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for FormationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FormationId({})", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Formation {
    id: FormationId,
    pattern: Pattern,
    agents: Vec<String>,
    stability: f64,
}

impl Formation {
    pub fn new(
        id: FormationId,
        pattern: Pattern,
        agents: Vec<String>,
        stability: f64,
    ) -> crate::Result<Self> {
        if agents.is_empty() {
            return Err(crate::EmergenceError::InvalidPattern(
                "formation must contain at least one agent".to_string(),
            ));
        }
        if stability < 0.0 || stability > 1.0 {
            return Err(crate::EmergenceError::InvalidPattern(
                "stability must be between 0.0 and 1.0".to_string(),
            ));
        }
        Ok(Formation {
            id,
            pattern,
            agents,
            stability,
        })
    }

    pub fn id(&self) -> FormationId {
        self.id
    }

    pub fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    pub fn agents(&self) -> &[String] {
        &self.agents
    }

    pub fn stability(&self) -> f64 {
        self.stability
    }

    pub fn validate(&self) -> crate::Result<()> {
        if self.agents.is_empty() {
            return Err(crate::EmergenceError::InvalidPattern(
                "formation has no agents".to_string(),
            ));
        }
        if self.stability < 0.0 || self.stability > 1.0 {
            return Err(crate::EmergenceError::InvalidPattern(
                format!("invalid stability: {}", self.stability),
            ));
        }
        Ok(())
    }
}

impl fmt::Display for Formation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Formation(id={}, pattern={}, agents={}, stability={:.3})",
            self.id,
            self.pattern,
            self.agents.len(),
            self.stability
        )
    }
}
