// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

pub mod energy_budget;
pub mod activation;
pub mod cooling;
pub mod prioritization;
pub mod conservation;
pub mod recycling;
pub mod decay;

pub use energy_budget::{EnergyBudget, Allocation};
pub use activation::{Activation, ActivationThreshold};
pub use cooling::{Cooler, HeatDissipation, CoolingMode, ThermalState};
pub use prioritization::{RequestPriority, PrioritizedRequest, PriorityScheduler};
pub use conservation::{ConservationPolicy, EfficiencyMetric};
pub use recycling::{ResourceRecycler, ReusePolicy};
pub use decay::{DecayCurve, DegradationModel};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetabolismError {
    BudgetExhausted,
    InvalidThreshold,
    ThermalLimitExceeded,
    SchedulingConflict,
    ConservationViolation,
    RecyclingFailure,
    DegradationFatal,
    Configuration(String),
}

impl std::fmt::Display for MetabolismError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetabolismError::BudgetExhausted => write!(f, "Energy budget exhausted"),
            MetabolismError::InvalidThreshold => write!(f, "Invalid activation threshold"),
            MetabolismError::ThermalLimitExceeded => write!(f, "Thermal limit exceeded"),
            MetabolismError::SchedulingConflict => write!(f, "Scheduling conflict detected"),
            MetabolismError::ConservationViolation => write!(f, "Conservation policy violation"),
            MetabolismError::RecyclingFailure => write!(f, "Recycling process failed"),
            MetabolismError::DegradationFatal => write!(f, "Fatal degradation level reached"),
            MetabolismError::Configuration(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for MetabolismError {}

