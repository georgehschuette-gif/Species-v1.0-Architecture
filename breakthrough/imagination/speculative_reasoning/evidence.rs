// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::ImaginationError;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EvidenceType {
    Direct,
    Circumstantial,
    Testimonial,
    Analytical,
    Empirical,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Evidence {
    pub evidence_type: EvidenceType,
    pub weight: f64,
    pub source_reliability: f64,
    pub description: String,
    pub supporting: bool,
}

impl Evidence {
    pub const MIN_WEIGHT: f64 = 0.0;
    pub const MAX_WEIGHT: f64 = 1.0;

    pub fn new(
        evidence_type: EvidenceType,
        weight: f64,
        source_reliability: f64,
        description: impl Into<String>,
    ) -> Result<Self, ImaginationError> {
        if !(Self::MIN_WEIGHT..=Self::MAX_WEIGHT).contains(&weight) {
            return Err(ImaginationError::OutOfRange {
                field: "weight".into(),
                value: weight,
                min: Self::MIN_WEIGHT,
                max: Self::MAX_WEIGHT,
            });
        }
        if !(0.0..=1.0).contains(&source_reliability) {
            return Err(ImaginationError::OutOfRange {
                field: "source_reliability".into(),
                value: source_reliability,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            evidence_type,
            weight,
            source_reliability,
            description: description.into(),
            supporting: true,
        })
    }

    pub fn effective_weight(&self) -> f64 {
        self.weight * self.source_reliability
    }

    pub fn set_supporting(&mut self, supporting: bool) {
        self.supporting = supporting;
    }

    pub fn validate(&self) -> Result<(), ImaginationError> {
        Self::new(
            self.evidence_type,
            self.weight,
            self.source_reliability,
            &self.description,
        )?;
        Ok(())
    }
}

impl fmt::Display for Evidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Evidence(type={:?}, weight={:.2}, reliability={:.2})",
            self.evidence_type, self.weight, self.source_reliability
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EvidenceWeight {
    Strong,
    Moderate,
    Weak,
    Negligible,
}

impl EvidenceWeight {
    pub fn from_score(score: f64) -> Self {
        if score >= 0.8 {
            Self::Strong
        } else if score >= 0.5 {
            Self::Moderate
        } else if score >= 0.2 {
            Self::Weak
        } else {
            Self::Negligible
        }
    }

    pub fn as_factor(&self) -> f64 {
        match self {
            Self::Strong => 1.0,
            Self::Moderate => 0.7,
            Self::Weak => 0.4,
            Self::Negligible => 0.1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceSource {
    Observation,
    Experiment,
    Authority,
    Deduction,
    Analogy,
}

impl fmt::Display for EvidenceSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Observation => write!(f, "Observation"),
            Self::Experiment => write!(f, "Experiment"),
            Self::Authority => write!(f, "Authority"),
            Self::Deduction => write!(f, "Deduction"),
            Self::Analogy => write!(f, "Analogy"),
        }
    }
}