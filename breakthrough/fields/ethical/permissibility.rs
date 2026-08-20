// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// EthicalNorm: A normative principle governing moral permissibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EthicalNorm {
    Deontological,
    Consequentialist,
    VirtueBased,
    Contractual,
}

impl EthicalNorm {
    pub fn as_str(&self) -> &'static str {
        match self {
            EthicalNorm::Deontological => "deontological",
            EthicalNorm::Consequentialist => "consequentialist",
            EthicalNorm::VirtueBased => "virtue_based",
            EthicalNorm::Contractual => "contractual",
        }
    }

    pub fn is_deontological(&self) -> bool {
        matches!(self, EthicalNorm::Deontological)
    }

    pub fn is_consequentialist(&self) -> bool {
        matches!(self, EthicalNorm::Consequentialist)
    }
}

/// EthicalPermissibility: Moral permissibility of actions.
pub struct EthicalPermissibility {
    pub action_id: u64,
    pub permissibility_score: f64,
    pub justification_strength: f64,
    pub norm_conflicts: Vec<EthicalNorm>,
}

impl EthicalPermissibility {
    pub fn new(action_id: u64) -> Result<Self, EthicalError> {
        if action_id == 0 {
            return Err(EthicalError::InvalidAction { action_id });
        }
        Ok(Self { action_id, permissibility_score: 0.5, justification_strength: 0.0, norm_conflicts: Vec::new() })
    }

    pub fn add_conflict(&mut self, norm: EthicalNorm) {
        self.norm_conflicts.push(norm);
    }

    pub fn conflict_count(&self) -> usize {
        self.norm_conflicts.len()
    }

    pub fn is_permissible(&self) -> bool {
        self.permissibility_score > 0.5 && self.conflict_count() == 0
    }

    pub fn set_score(&mut self, score: f64) -> Result<(), EthicalError> {
        if !(0.0..=1.0).contains(&score) {
            return Err(EthicalError::InvalidScore { score });
        }
        self.permissibility_score = score;
        Ok(())
    }

    pub fn has_conflict(&self, norm: &EthicalNorm) -> bool {
        self.norm_conflicts.contains(norm)
    }

    pub fn justification_strength_label(&self) -> &'static str {
        if self.justification_strength > 0.8 {
            "strong"
        } else if self.justification_strength > 0.4 {
            "moderate"
        } else {
            "weak"
        }
    }
}
