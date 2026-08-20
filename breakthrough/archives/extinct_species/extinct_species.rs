// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ExtinctSpeciesError {
    InvalidConfidence(f64),
    NegativeRecovery,
    UnidentifiedEra,
}

impl fmt::Display for ExtinctSpeciesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfidence(value) => write!(f, "invalid confidence value: {}", value),
            Self::NegativeRecovery => write!(f, "recovery potential must not be negative"),
            Self::UnidentifiedEra => write!(f, "extinction era is not recognized"),
        }
    }
}

impl std::error::Error for ExtinctSpeciesError {}

pub struct ExtinctSpecies {
    pub id: u64,
    pub name: String,
    pub extinction_era: String,
    pub recovery_potential: f64,
    pub confidence: f64,
}

impl ExtinctSpecies {
    pub fn new(
        id: u64,
        name: String,
        extinction_era: String,
        recovery_potential: f64,
        confidence: f64,
    ) -> Result<Self, ExtinctSpeciesError> {
        if !(0.0..=1.0).contains(&confidence) {
            return Err(ExtinctSpeciesError::InvalidConfidence(confidence));
        }
        if recovery_potential < 0.0 {
            return Err(ExtinctSpeciesError::NegativeRecovery);
        }
        if extinction_era.trim().is_empty() {
            return Err(ExtinctSpeciesError::UnidentifiedEra);
        }
        Ok(Self {
            id,
            name,
            extinction_era,
            recovery_potential,
            confidence,
        })
    }

    pub fn is_confirmed(&self) -> bool {
        self.confidence > 0.8
    }

    pub fn can_recover(&self) -> bool {
        self.recovery_potential > 0.0
    }

    pub fn recovery_score(&self) -> f64 {
        self.confidence * self.recovery_potential
    }
}

impl fmt::Display for ExtinctSpecies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ExtinctSpecies(id={}, name={}, era={}, confidence={:.2}, recoverable={})",
            self.id,
            self.name,
            self.extinction_era,
            self.confidence,
            self.can_recover()
        )
    }
}
