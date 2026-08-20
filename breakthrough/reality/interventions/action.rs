// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::HashMap;
use std::fmt;

use crate::RealityError;

/// InterventionType: The category of intervention being performed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InterventionType {
    /// Modifying the environment directly.
    Environmental,
    /// Communicating with another agent.
    Communicative,
    /// Adjusting internal model parameters.
    Internal,
    /// Querying or requesting information.
    Query,
    /// Executing a predefined plan or procedure.
    Procedural,
    /// Exploring or gathering new data.
    Exploratory,
}

impl InterventionType {
    /// Returns the string label of this intervention type.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Environmental => "environmental",
            Self::Communicative => "communicative",
            Self::Internal => "internal",
            Self::Query => "query",
            Self::Procedural => "procedural",
            Self::Exploratory => "exploratory",
        }
    }

    /// Returns whether this intervention modifies the external environment.
    pub fn is_external(&self) -> bool {
        matches!(self, Self::Environmental | Self::Communicative | Self::Exploratory)
    }

    /// Returns whether this intervention requires physical capability.
    pub fn requires_physical(&self) -> bool {
        matches!(self, Self::Environmental | Self::Procedural)
    }

    /// Returns the expected latency class for this intervention type.
    pub fn latency_class(&self) -> LatencyClass {
        match self {
            Self::Environmental => LatencyClass::High,
            Self::Communicative => LatencyClass::Medium,
            Self::Internal => LatencyClass::Low,
            Self::Query => LatencyClass::Medium,
            Self::Procedural => LatencyClass::Medium,
            Self::Exploratory => LatencyClass::High,
        }
    }
}

impl std::fmt::Display for InterventionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for InterventionType {
    fn default() -> Self {
        Self::Internal
    }
}

/// LatencyClass: Expected latency for an intervention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LatencyClass {
    /// Low latency (internal computation).
    Low,
    /// Medium latency (communication, queries).
    Medium,
    /// High latency (environmental changes, exploration).
    High,
}

