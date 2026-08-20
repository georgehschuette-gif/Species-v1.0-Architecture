// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Migration: The movement of entities between habitats and populations.
//! Defines how cognitive entities relocate, spread, and colonize new spaces.

pub mod dispersal;
pub mod colonization;
pub mod gene_flow;

pub use dispersal::DispersalMechanism;
pub use colonization::ColonizationEvent;
pub use gene_flow::GeneFlow;

use std::fmt;

/// MigrationError: Errors that can occur during migration operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigrationError {
    /// The dispersal distance is invalid (negative or NaN).
    InvalidDispersalDistance,
    /// The colonization probability is outside valid range [0.0, 1.0].
    InvalidColonizationProbability,
    /// The gene flow rate is invalid (negative or NaN).
    InvalidGeneFlowRate,
    /// The migrant count exceeds the source population size.
    MigrantCountExceedsPopulation,
    /// The founding population is too small to sustain colonization.
    FoundingPopulationTooSmall,
    /// The target habitat does not exist or is invalid.
    InvalidTargetHabitat,
    /// The source population does not exist or is invalid.
    InvalidSourcePopulation,
    /// A required habitat or population reference is zero (uninitialized).
    UninitializedReference,
    /// The migration pattern has contradictory parameters.
    ContradictoryParameters,
    /// The migration operation would cause a population to go extinct.
    ExtinctionRiskTooHigh,
}

impl fmt::Display for MigrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MigrationError::InvalidDispersalDistance => {
                write!(f, "Dispersal distance must be a non-negative, finite number")
            }
            MigrationError::InvalidColonizationProbability => {
                write!(f, "Colonization probability must be in the range [0.0, 1.0]")
            }
            MigrationError::InvalidGeneFlowRate => {
                write!(f, "Gene flow rate must be a non-negative, finite number")
            }
            MigrationError::MigrantCountExceedsPopulation => {
                write!(f, "Migrant count cannot exceed the source population size")
            }
            MigrationError::FoundingPopulationTooSmall => {
                write!(f, "Founding population is too small to sustain colonization")
            }
            MigrationError::InvalidTargetHabitat => {
                write!(f, "Target habitat ID is invalid or does not exist")
            }
            MigrationError::InvalidSourcePopulation => {
                write!(f, "Source population ID is invalid or does not exist")
            }
            MigrationError::UninitializedReference => {
                write!(f, "A required reference (habitat or population) is uninitialized (zero)")
            }
            MigrationError::ContradictoryParameters => {
                write!(f, "Migration parameters contain contradictory values")
            }
            MigrationError::ExtinctionRiskTooHigh => {
                write!(f, "Migration operation would exceed acceptable extinction risk")
            }
        }
    }
}

impl std::error::Error for MigrationError {}

/// MigrationPattern: A defined pattern governing how entities migrate between habitats.
///
/// A migration pattern encapsulates the type of migration (random, directed,
/// active, or passive), the mean and variance of dispersal distances, and the
/// rate at which migration events occur. It serves as a template for
/// [DispersalMechanism] instances and for the [MigrationEngine] to evaluate
/// migration outcomes.
#[derive(Debug, Clone)]
pub struct MigrationPattern {
    /// The type of dispersal mechanism governing this pattern.
    pub mechanism_type: dispersal::DispersalType,
    /// The mean dispersal distance per migration event.
    pub distance_mean: f64,
    /// The variance of dispersal distance per migration event.
    pub distance_variance: f64,
    /// The rate of migration events per time step (must be in [0.0, 1.0]).
    pub migration_rate: f64,
}

impl MigrationPattern {
    /// Construct a new [MigrationPattern] with the given parameters.
    ///
    /// # Errors
    /// Returns [MigrationError::InvalidDispersalDistance] if `distance_mean`
    /// or `distance_variance` is negative or NaN.
    /// Returns [MigrationError::InvalidGeneFlowRate] if `migration_rate`
    /// is outside [0.0, 1.0].
    pub fn new(
        mechanism_type: dispersal::DispersalType,
        distance_mean: f64,
        distance_variance: f64,
        migration_rate: f64,
    ) -> Result<Self, MigrationError> {
        if distance_mean.is_nan() || distance_mean < 0.0 {
            return Err(MigrationError::InvalidDispersalDistance);
        }
        if distance_variance.is_nan() || distance_variance < 0.0 {
            return Err(MigrationError::InvalidDispersalDistance);
        }
        if migration_rate.is_nan() || migration_rate < 0.0 || migration_rate > 1.0 {
            return Err(MigrationError::InvalidGeneFlowRate);
        }
        if distance_variance > distance_mean * distance_mean && distance_mean > 0.0 {
            return Err(MigrationError::ContradictoryParameters);
        }
        Ok(Self {
            mechanism_type,
            distance_mean,
            distance_variance,
            migration_rate,
        })
    }

    /// Validate that all parameters in this pattern are internally consistent.
    pub fn validate(&self) -> Result<(), MigrationError> {
        if self.distance_mean.is_nan() || self.distance_mean < 0.0 {
            return Err(MigrationError::InvalidDispersalDistance);
        }
        if self.distance_variance.is_nan() || self.distance_variance < 0.0 {
            return Err(MigrationError::InvalidDispersalDistance);
        }
        if self.migration_rate.is_nan()
            || self.migration_rate < 0.0
            || self.migration_rate > 1.0
        {
            return Err(MigrationError::InvalidGeneFlowRate);
        }
        if self.distance_variance > self.distance_mean * self.distance_mean
            && self.distance_mean > 0.0
        {
            return Err(MigrationError::ContradictoryParameters);
        }
        Ok(())
    }

    /// Compute the standard deviation of the dispersal distance.
    pub fn distance_std_dev(&self) -> f64 {
        self.distance_variance.sqrt()
    }

    /// Return whether this pattern represents a directed migration.
    pub fn is_directed(&self) -> bool {
        matches!(self.mechanism_type, dispersal::DispersalType::Directed)
    }

    /// Return whether this pattern represents a random migration.
    pub fn is_random(&self) -> bool {
        matches!(self.mechanism_type, dispersal::DispersalType::Random)
    }
}

/// MigrationEngine: Orchestrates migration operations across habitats and populations.
///
/// The engine manages active [MigrationPattern]s, evaluates colonization events,
/// and tracks gene flow between populations. It is the central coordinator for
/// all migration-related dynamics within the cognitive ecosystem.
#[derive(Debug)]
pub struct MigrationEngine {
    /// The active migration patterns governing dispersal behavior.
    pub patterns: Vec<MigrationPattern>,
    /// The current step index within the simulation.
    pub step: usize,
    /// The maximum number of migration events allowed per step.
    pub max_events_per_step: usize,
    /// The extinction risk threshold above which migration is suppressed.
    pub extinction_threshold: f64,
}

impl MigrationEngine {
    /// Construct a new [MigrationEngine] with default capacity settings.
    pub fn new(max_events_per_step: usize, extinction_threshold: f64) -> Result<Self, MigrationError> {
        if max_events_per_step == 0 {
            return Err(MigrationError::ContradictoryParameters);
        }
        if extinction_threshold.is_nan() || extinction_threshold < 0.0 || extinction_threshold > 1.0 {
            return Err(MigrationError::InvalidColonizationProbability);
        }
        Ok(Self {
            patterns: Vec::new(),
            step: 0,
            max_events_per_step,
            extinction_threshold,
        })
    }

    /// Register a new [MigrationPattern] with the engine.
    ///
    /// The pattern is validated before being added. If validation fails,
    /// the error is returned and the pattern is not registered.
    pub fn register_pattern(&mut self, pattern: MigrationPattern) -> Result<(), MigrationError> {
        pattern.validate()?;
        self.patterns.push(pattern);
        Ok(())
    }

    /// Remove all patterns matching the given dispersal type.
    pub fn clear_patterns_by_type(&mut self, mechanism_type: dispersal::DispersalType) {
        self.patterns.retain(|p| p.mechanism_type != mechanism_type);
    }

    /// Run a single migration step across all registered patterns.
    ///
    /// For each pattern, if the pattern's migration rate permits an event,
    /// a dispersal is simulated. Returns the count of migration events
    /// executed this step.
    pub fn step(&mut self) -> usize {
        if self.patterns.is_empty() {
            return 0;
        }
        let mut events = 0;
        for pattern in &self.patterns {
            if events >= self.max_events_per_step {
                break;
            }
            if pattern.migration_rate > 0.0 && pattern.migration_rate >= 1.0 {
                events += 1;
            } else if pattern.migration_rate > 0.0 {
                let rand_val = Self::pseudo_random(self.step, events);
                if rand_val < pattern.migration_rate {
                    events += 1;
                }
            }
        }
        self.step += 1;
        events
    }

    /// Evaluate whether a colonization event should proceed based on
    /// the extinction risk threshold.
    pub fn assess_colonization_risk(&self, success_probability: f64) -> Result<bool, MigrationError> {
        if success_probability.is_nan() || success_probability < 0.0 || success_probability > 1.0 {
            return Err(MigrationError::InvalidColonizationProbability);
        }
        let failure_probability = 1.0 - success_probability;
        Ok(failure_probability <= self.extinction_threshold)
    }

    /// Compute the total effective dispersal distance across all patterns,
    /// weighted by migration rate.
    pub fn total_effective_distance(&self) -> f64 {
        self.patterns
            .iter()
            .map(|p| {
                let weight = p.migration_rate;
                weight * p.distance_mean
            })
            .sum()
    }

    /// Return a summary of the engine's current state as a human-readable string.
    pub fn summary(&self) -> String {
        format!(
            "MigrationEngine(step={}, patterns={}, max_events={}, extinction_threshold={:.4})",
            self.step,
            self.patterns.len(),
            self.max_events_per_step,
            self.extinction_threshold,
        )
    }

    /// Simple deterministic pseudo-random number in [0.0, 1.0) based on step and event indices.
    fn pseudo_random(step: usize, event: usize) -> f64 {
        let mut x = (step as u32).wrapping_mul(2654435761u32).wrapping_add(event as u32);
        x ^= x >> 16;
        x = x.wrapping_mul(0x85ebca6b);
        x ^= x >> 13;
        x = x.wrapping_mul(0xc2b2ae35);
        x ^= x >> 16;
        (x as f64) / u32::MAX as f64
    }
}

impl Default for MigrationPattern {
    fn default() -> Self {
        Self {
            mechanism_type: dispersal::DispersalType::Random,
            distance_mean: 1.0,
            distance_variance: 0.0,
            migration_rate: 0.5,
        }
    }
}

impl Default for MigrationEngine {
    fn default() -> Self {
        Self {
            patterns: Vec::new(),
            step: 0,
            max_events_per_step: 100,
            extinction_threshold: 0.95,
        }
    }
}