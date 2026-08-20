// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// ExtinctionEvent: A widespread extinction event affecting multiple species.
pub struct ExtinctionEvent {
    pub id: u64,
    pub severity: f64,
    pub affected_species: Vec<u64>,
    pub recovery_time: f64,
    pub cause: ExtinctionCause,
    pub active: bool,
}

impl ExtinctionEvent {
    /// Create a new extinction event. Severity and recovery_time must be
    /// non-negative. The event starts active.
    pub fn new(
        id: u64,
        severity: f64,
        recovery_time: f64,
        cause: ExtinctionCause,
    ) -> Result<Self, ExtinctionError> {
        if severity < 0.0 {
            return Err(ExtinctionError::InvalidProbability(severity));
        }
        if recovery_time < 0.0 {
            return Err(ExtinctionError::InvalidProbability(recovery_time));
        }
        Ok(Self {
            id,
            severity,
            affected_species: Vec::new(),
            recovery_time,
            cause,
            active: true,
        })
    }

    /// Add a species identifier to the event's affected list.
    /// Returns an error if the event has already ended.
    pub fn add_affected_species(&mut self, species_id: u64) -> Result<(), ExtinctionError> {
        if !self.active {
            return Err(ExtinctionError::UnfeasibleRecovery);
        }
        self.affected_species.push(species_id);
        Ok(())
    }

    /// Remove a species identifier from the event's affected list.
    /// Returns an error if the species is not present or the event ended.
    pub fn remove_affected_species(&mut self, species_id: u64) -> Result<(), ExtinctionError> {
        if !self.active {
            return Err(ExtinctionError::UnfeasibleRecovery);
        }
        let len_before = self.affected_species.len();
        self.affected_species.retain(|&id| id != species_id);
        if self.affected_species.len() == len_before {
            return Err(ExtinctionError::UnfeasibleRecovery);
        }
        Ok(())
    }

    /// End the extinction event, marking it inactive.
    pub fn conclude(&mut self) -> Result<(), ExtinctionError> {
        if !self.active {
            return Err(ExtinctionError::UnfeasibleRecovery);
        }
        self.active = false;
        Ok(())
    }

    /// Reactivate a concluded event, allowing further species registration.
    pub fn reopen(&mut self) -> Result<(), ExtinctionError> {
        if self.active {
            return Err(ExtinctionError::UnfeasibleRecovery);
        }
        if self.affected_species.is_empty() {
            return Err(ExtinctionError::EmptyPopulation);
        }
        self.active = true;
        Ok(())
    }

    /// Number of species affected by this event.
    pub fn affected_count(&self) -> usize {
        self.affected_species.len()
    }

    /// Compute an impact score combining severity and species count.
    pub fn impact_score(&self) -> f64 {
        self.severity * (self.affected_species.len() as f64).ln().max(1.0)
    }

    /// Return true if this event is currently active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Advance recovery time by a given delta. Clamped at 0.0.
    pub fn advance_recovery(&mut self, years: f64) {
        let delta = years.max(0.0);
        self.recovery_time = (self.recovery_time - delta).max(0.0);
        if self.recovery_time == 0.0 && self.active {
            self.active = false;
        }
    }

    /// Set severity, validated to be non-negative.
    pub fn set_severity(&mut self, severity: f64) -> Result<(), ExtinctionError> {
        if severity < 0.0 {
            return Err(ExtinctionError::InvalidProbability(severity));
        }
        self.severity = severity;
        Ok(())
    }
}

impl PartialEq for ExtinctionEvent {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ExtinctionEvent {}

impl std::fmt::Display for ExtinctionEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.active { "active" } else { "concluded" };
        write!(
            f,
            "ExtinctionEvent(id={}, severity={:.2}, species={}, cause={:?}, status={})",
            self.id,
            self.severity,
            self.affected_species.len(),
            self.cause,
            status
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtinctionCause {
    Environmental,
    Competitive,
    Catastrophic,
    Anthropogenic,
}

impl std::fmt::Display for ExtinctionCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Environmental => write!(f, "environmental"),
            Self::Competitive => write!(f, "competitive"),
            Self::Catastrophic => write!(f, "catastrophic"),
            Self::Anthropogenic => write!(f, "anthropogenic"),
        }
    }
}