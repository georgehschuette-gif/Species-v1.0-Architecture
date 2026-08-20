// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// SocialTrust: Reliability attributed to social sources.
pub struct SocialTrust {
    pub source_id: u64,
    pub trust_score: f64,
    pub history_length: usize,
    pub decay_rate: f64,
}

impl SocialTrust {
    pub fn new(source_id: u64, initial_trust: f64) -> Result<Self, SocialError> {
        if source_id == 0 {
            return Err(SocialError::InvalidAgent { agent_id: source_id });
        }
        if !(0.0..=1.0).contains(&initial_trust) {
            return Err(SocialError::InvalidTrust { trust: initial_trust });
        }
        Ok(Self { source_id, trust_score: initial_trust, history_length: 0, decay_rate: 0.01 })
    }

    pub fn update(&mut self, outcome: bool) {
        self.history_length += 1;
        let adjustment = if outcome { 0.05 } else { -0.1 };
        self.trust_score = (self.trust_score + adjustment).clamp(0.0, 1.0);
    }

    pub fn decay(&mut self) {
        self.trust_score = (self.trust_score * (1.0 - self.decay_rate)).max(0.0);
    }

    pub fn is_trustworthy(&self) -> bool {
        self.trust_score > 0.6 && self.history_length >= 5
    }
}
