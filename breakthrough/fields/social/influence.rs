// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// SocialInfluence: Capacity to affect others' cognitive states.
pub struct SocialInfluence {
    pub agent_id: u64,
    pub reach: usize,
    pub persuasion_strength: f64,
    pub reciprocity_factor: f64,
}

impl SocialInfluence {
    pub fn new(agent_id: u64, persuasion_strength: f64) -> Result<Self, SocialError> {
        if agent_id == 0 {
            return Err(SocialError::InvalidAgent { agent_id });
        }
        if !(0.0..=1.0).contains(&persuasion_strength) {
            return Err(SocialError::InvalidInfluence { influence: persuasion_strength });
        }
        Ok(Self { agent_id, reach: 0, persuasion_strength, reciprocity_factor: 0.5 })
    }

    pub fn effective_influence(&self) -> f64 {
        self.persuasion_strength * (1.0 + self.reciprocity_factor * (self.reach as f64 / 100.0))
    }

    pub fn extend_reach(&mut self, new_contacts: usize) {
        self.reach += new_contacts;
    }
}
