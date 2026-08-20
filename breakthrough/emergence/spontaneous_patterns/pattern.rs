// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PatternId(u64);

impl PatternId {
    pub fn new(id: u64) -> Self {
        PatternId(id)
    }

    pub fn inner(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for PatternId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PatternId({})", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    id: PatternId,
    strength: f64,
    elements: Vec<String>,
}

impl Pattern {
    pub fn new(id: PatternId, strength: f64, elements: Vec<String>) -> crate::Result<Self> {
        if strength < 0.0 || strength > 1.0 {
            return Err(crate::EmergenceError::InvalidPattern(
                "strength must be between 0.0 and 1.0".to_string(),
            ));
        }
        if elements.is_empty() {
            return Err(crate::EmergenceError::InvalidPattern(
                "pattern must contain at least one element".to_string(),
            ));
        }
        Ok(Pattern {
            id,
            strength,
            elements,
        })
    }

    pub fn id(&self) -> PatternId {
        self.id
    }

    pub fn strength(&self) -> f64 {
        self.strength
    }

    pub fn elements(&self) -> &[String] {
        &self.elements
    }

    pub fn validate(&self) -> crate::Result<()> {
        if self.strength < 0.0 || self.strength > 1.0 {
            return Err(crate::EmergenceError::InvalidPattern(
                format!("invalid strength: {}", self.strength),
            ));
        }
        if self.elements.is_empty() {
            return Err(crate::EmergenceError::InvalidPattern(
                "pattern has no elements".to_string(),
            ));
        }
        Ok(())
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Pattern(id={}, strength={:.3}, elements={})",
            self.id,
            self.strength,
            self.elements.len()
        )
    }
}
