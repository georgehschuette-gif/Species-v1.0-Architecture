// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// InterventionMechanism: The causal pathway through which an intervention operates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterventionMechanism {
    Direct,
    Indirect,
    Mediated,
    Confounded,
}

impl InterventionMechanism {
    pub fn as_str(&self) -> &'static str {
        match self {
            InterventionMechanism::Direct => "direct",
            InterventionMechanism::Indirect => "indirect",
            InterventionMechanism::Mediated => "mediated",
            InterventionMechanism::Confounded => "confounded",
        }
    }

    pub fn is_direct(&self) -> bool {
        matches!(self, InterventionMechanism::Direct)
    }

    pub fn requires_mediator(&self) -> bool {
        matches!(self, InterventionMechanism::Mediated)
    }
}

/// CausalIntervention: Strength of causal influence between nodes.
#[derive(Debug, Clone, PartialEq)]
pub struct CausalIntervention {
    pub source: u64,
    pub target: u64,
    pub strength: f64,
    pub mechanism: InterventionMechanism,
    pub confidence: f64,
}

impl CausalIntervention {
    pub fn new(source: u64, target: u64, strength: f64, mechanism: InterventionMechanism) -> Result<Self, CausalError> {
        if source == 0 || target == 0 {
            return Err(CausalError::InvalidNode { node_id: source.min(target) });
        }
        if !(0.0..=1.0).contains(&strength) {
            return Err(CausalError::InvalidStrength { strength });
        }
        Ok(Self { source, target, strength, mechanism, confidence: 1.0 })
    }

    pub fn apply(&self, target_state: f64) -> f64 {
        target_state + self.strength * (1.0 - target_state)
    }

    pub fn is_significant(&self) -> bool {
        self.strength > 0.5 && self.confidence > 0.7
    }

    pub fn mechanism_str(&self) -> &'static str {
        self.mechanism.as_str()
    }

    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }
}
