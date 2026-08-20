// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

use crate::TensionsError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterpretationConfidence {
    Low,
    Medium,
    High,
    Certain,
}

pub struct AmbiguityInterpretation {
    pub signal: f64,
    pub confidence: InterpretationConfidence,
    pub context_weight: f64,
}

impl AmbiguityInterpretation {
    pub fn from_signal(signal: f64) -> Self {
        let confidence = match signal {
            s if s < 0.25 => InterpretationConfidence::Low,
            s if s < 0.5 => InterpretationConfidence::Medium,
            s if s < 0.75 => InterpretationConfidence::High,
            _ => InterpretationConfidence::Certain,
        };
        Self { signal, confidence, context_weight: 1.0 }
    }

    pub fn adjust_confidence(&mut self, evidence: f64) {
        self.signal = (self.signal + evidence * 0.1).clamp(0.0, 1.0);
        self.confidence = match self.signal {
            s if s < 0.25 => InterpretationConfidence::Low,
            s if s < 0.5 => InterpretationConfidence::Medium,
            s if s < 0.75 => InterpretationConfidence::High,
            _ => InterpretationConfidence::Certain,
        };
    }

    pub fn confidence_level(&self) -> InterpretationConfidence { self.confidence }
    pub fn set_context_weight(&mut self, weight: f64) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&weight) { return Err(TensionsError::OutOfRange { field: "context_weight".into(), value: weight, min: 0.0, max: 1.0 }); }
        self.context_weight = weight;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.signal) { return Err(TensionsError::OutOfRange { field: "signal".into(), value: self.signal, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Display for InterpretationConfidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { Self::Low => write!(f, "Low"), Self::Medium => write!(f, "Medium"), Self::High => write!(f, "High"), Self::Certain => write!(f, "Certain") }
    }
}
