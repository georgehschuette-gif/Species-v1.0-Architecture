// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// DeExtinction: The recovery and reintroduction of extinct species or concepts.
pub struct DeExtinction {
    pub extinct_species_id: u64,
    pub recovery_source: RecoverySource,
    pub success_probability: f64,
    pub progress: f64,
    pub started: bool,
    pub completed: bool,
}

impl DeExtinction {
    /// Create a new DeExtinction initiative.
    /// success_probability must be in [0.0, 1.0].
    pub fn new(
        extinct_species_id: u64,
        recovery_source: RecoverySource,
        success_probability: f64,
    ) -> Result<Self, ExtinctionError> {
        if !(0.0..=1.0).contains(&success_probability) {
            return Err(ExtinctionError::InvalidProbability(success_probability));
        }
        Ok(Self {
            extinct_species_id,
            recovery_source,
            success_probability,
            progress: 0.0,
            started: false,
            completed: false,
        })
    }

    /// Begin the de-extinction process.
    pub fn start(&mut self) -> Result<(), ExtinctionError> {
        if self.started {
            return Err(ExtinctionError::UnfeasibleRecovery);
        }
        if self.success_probability < 0.05 {
            return Err(ExtinctionError::UnfeasibleRecovery);
        }
        self.started = true;
        Ok(())
    }

    /// Advance progress toward completion. Progress is clamped to [0.0, 1.0].
    /// Returns an error if the initiative has not started or is already complete.
    pub fn advance(&mut self, delta: f64) -> Result<(), ExtinctionError> {
        if !self.started || self.completed {
            return Err(ExtinctionError::UnfeasibleRecovery);
        }
        self.progress = (self.progress + delta).clamp(0.0, 1.0);
        if self.progress >= 1.0 {
            self.progress = 1.0;
            self.completed = true;
        }
        Ok(())
    }

    /// Set the success probability, validated to [0.0, 1.0].
    pub fn set_success_probability(&mut self, probability: f64) -> Result<(), ExtinctionError> {
        if !(0.0..=1.0).contains(&probability) {
            return Err(ExtinctionError::InvalidProbability(probability));
        }
        self.success_probability = probability;
        Ok(())
    }

    /// Return true if the initiative is currently running (started and not completed).
    pub fn is_running(&self) -> bool {
        self.started && !self.completed
    }

    /// Compute feasibility score based on source availability and probability.
    pub fn feasibility(&self) -> f64 {
        let source_bonus = match self.recovery_source {
            RecoverySource::GeneticArchive => 0.3,
            RecoverySource::StructuralTemplate => 0.2,
            RecoverySource::HybridResurrection => 0.1,
        };
        (self.success_probability + source_bonus).clamp(0.0, 1.0)
    }

    /// Estimate remaining work as a percentage.
    pub fn remaining(&self) -> f64 {
        1.0 - self.progress
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoverySource {
    GeneticArchive,
    StructuralTemplate,
    HybridResurrection,
}

impl std::fmt::Display for RecoverySource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GeneticArchive => write!(f, "genetic_archive"),
            Self::StructuralTemplate => write!(f, "structural_template"),
            Self::HybridResurrection => write!(f, "hybrid_resurrection"),
        }
    }
}
