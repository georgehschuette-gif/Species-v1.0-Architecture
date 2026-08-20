// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.


/// EnergyForm: The type or manifestation of cognitive energy.
///
/// Each variant represents a distinct mode of energy within the cognitive
/// ecosystem:
/// - `Activation`: Energy associated with entity activation and firing
/// - `Connection`: Energy stored in and transmitted through connections
/// - `Computation`: Energy consumed during cognitive processing
/// - `Storage`: Energy held in memory or reserve states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EnergyForm {
    /// Energy associated with entity activation and firing.
    Activation,
    /// Energy stored in and transmitted through connections.
    Connection,
    /// Energy consumed during cognitive processing.
    Computation,
    /// Energy held in memory or reserve states.
    Storage,
}

impl EnergyForm {
    /// Returns the string representation of this energy form.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Activation => "activation",
            Self::Connection => "connection",
            Self::Computation => "computation",
            Self::Storage => "storage",
        }
    }

    /// Returns an iterator over all energy forms.
    pub fn all() -> impl Iterator<Item = Self> {
        [Self::Activation, Self::Connection, Self::Computation, Self::Storage].into_iter()
    }

    /// Attempts to parse an energy form from a string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "activation" => Some(Self::Activation),
            "connection" => Some(Self::Connection),
            "computation" => Some(Self::Computation),
            "storage" => Some(Self::Storage),
            _ => None,
        }
    }

    /// Returns whether this form is a transient energy type
    /// (activation or computation) as opposed to a persistent type.
    pub fn is_transient(&self) -> bool {
        matches!(self, Self::Activation | Self::Computation)
    }

    /// Returns whether this form is a persistent energy type
    /// (connection or storage).
    pub fn is_persistent(&self) -> bool {
        matches!(self, Self::Connection | Self::Storage)
    }

    /// Returns the priority ordering of this form.
    /// Lower values indicate higher priority.
    pub fn priority(&self) -> u8 {
        match self {
            Self::Activation => 0,
            Self::Connection => 1,
            Self::Computation => 2,
            Self::Storage => 3,
        }
    }

    /// Returns the default conversion efficiency to another form.
    pub fn conversion_efficiency_to(&self, target: &EnergyForm) -> f64 {
        if self == target {
            1.0
        } else {
            match (self, target) {
                (Self::Activation, Self::Connection) => 0.8,
                (Self::Activation, Self::Computation) => 0.9,
                (Self::Activation, Self::Storage) => 0.6,
                (Self::Connection, Self::Activation) => 0.7,
                (Self::Connection, Self::Computation) => 0.5,
                (Self::Connection, Self::Storage) => 0.95,
                (Self::Computation, Self::Activation) => 0.85,
                (Self::Computation, Self::Connection) => 0.45,
                (Self::Computation, Self::Storage) => 0.7,
                (Self::Storage, Self::Activation) => 0.55,
                (Self::Storage, Self::Connection) => 0.8,
                (Self::Storage, Self::Computation) => 0.6,
                _ => 0.5,
            }
        }
    }
}

impl std::fmt::Display for EnergyForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for EnergyForm {
    fn default() -> Self {
        Self::Activation
    }
}