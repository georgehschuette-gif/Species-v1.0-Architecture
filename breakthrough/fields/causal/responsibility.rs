// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// CausalResponsibility: Attribution of causal contribution.
pub struct CausalResponsibility {
    pub agent_id: u64,
    pub outcome_id: u64,
    pub contribution: f64,
    pub necessity: f64,
    pub sufficiency: f64,
}

impl CausalResponsibility {
    pub fn new(agent_id: u64, outcome_id: u64, contribution: f64) -> Result<Self, CausalError> {
        if agent_id == 0 || outcome_id == 0 {
            return Err(CausalError::InvalidNode { node_id: agent_id.min(outcome_id) });
        }
        if !(0.0..=1.0).contains(&contribution) {
            return Err(CausalError::InvalidStrength { strength: contribution });
        }
        Ok(Self { agent_id, outcome_id, contribution, necessity: contribution, sufficiency: contribution })
    }

    pub fn combined_score(&self) -> f64 {
        (self.necessity + self.sufficiency + self.contribution) / 3.0
    }

    pub fn is_primary_cause(&self) -> bool {
        self.contribution > 0.7 && self.necessity > 0.7
    }
}
