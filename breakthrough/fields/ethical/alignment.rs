// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// EthicalValue: A fundamental ethical value used in alignment assessment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EthicalValue {
    Beneficence,
    NonMaleficence,
    Autonomy,
    Justice,
}

impl EthicalValue {
    pub fn as_str(&self) -> &'static str {
        match self {
            EthicalValue::Beneficence => "beneficence",
            EthicalValue::NonMaleficence => "non_maleficence",
            EthicalValue::Autonomy => "autonomy",
            EthicalValue::Justice => "justice",
        }
    }

    pub fn is_beneficence(&self) -> bool {
        matches!(self, EthicalValue::Beneficence)
    }

    pub fn is_justice(&self) -> bool {
        matches!(self, EthicalValue::Justice)
    }
}

/// ValueAlignment: Compatibility between values and actions.
pub struct ValueAlignment {
    pub value_set: Vec<EthicalValue>,
    pub action_vector: Vec<f64>,
    pub alignment_score: f64,
}

impl ValueAlignment {
    pub fn new(value_set: Vec<EthicalValue>, action_vector: Vec<f64>) -> Result<Self, EthicalError> {
        if value_set.len() != action_vector.len() {
            return Err(EthicalError::DimensionMismatch {
                values: value_set.len(),
                actions: action_vector.len(),
            });
        }
        Ok(Self { value_set, action_vector, alignment_score: 0.0 })
    }

    pub fn compute_alignment(&mut self) -> Result<f64, EthicalError> {
        if self.value_set.is_empty() {
            return Err(EthicalError::InsufficientData);
        }
        let magnitude = self.action_vector.iter().map(|v| v * v).sum::<f64>().sqrt();
        if magnitude == 0.0 {
            self.alignment_score = 0.0;
            return Ok(0.0);
        }
        self.alignment_score = self.action_vector.iter().sum::<f64>() / magnitude;
        Ok(self.alignment_score)
    }

    pub fn is_aligned(&self) -> bool {
        self.alignment_score > 0.7
    }

    pub fn value_count(&self) -> usize {
        self.value_set.len()
    }
}
