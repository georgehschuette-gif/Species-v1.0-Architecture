// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JustificationStrength {
    Weak,
    Moderate,
    Strong,
    Definitive,
}

impl fmt::Display for JustificationStrength {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JustificationStrength::Weak => write!(f, "weak"),
            JustificationStrength::Moderate => write!(f, "moderate"),
            JustificationStrength::Strong => write!(f, "strong"),
            JustificationStrength::Definitive => write!(f, "definitive"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Justification {
    pub id: String,
    pub claim: String,
    pub reasoning: String,
    pub strength: JustificationStrength,
    pub premises: Vec<String>,
    pub conclusion: String,
}

impl Justification {
    pub fn new<S: Into<String>>(id: S, claim: S) -> Self {
        Self {
            id: id.into(),
            claim: claim.into(),
            reasoning: String::new(),
            strength: JustificationStrength::Moderate,
            premises: Vec::new(),
            conclusion: String::new(),
        }
    }

    pub fn with_reasoning<S: Into<String>>(mut self, reasoning: S) -> Self {
        self.reasoning = reasoning.into();
        self
    }

    pub fn with_strength(mut self, strength: JustificationStrength) -> Self {
        self.strength = strength;
        self
    }

    pub fn add_premise<S: Into<String>>(&mut self, premise: S) {
        self.premises.push(premise.into());
    }

    pub fn with_conclusion<S: Into<String>>(mut self, conclusion: S) -> Self {
        self.conclusion = conclusion.into();
        self
    }

    pub fn is_sound(&self) -> bool {
        !self.premises.is_empty() && !self.conclusion.is_empty()
    }
}

impl fmt::Display for Justification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Justification {}: {} ({})", self.id, self.claim, self.strength)
    }
}

