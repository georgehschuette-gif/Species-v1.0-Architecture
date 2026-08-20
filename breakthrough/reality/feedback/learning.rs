// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::RealityError;

/// LearningMode: The mode of learning being applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LearningMode {
    /// Supervised learning from labeled examples.
    Supervised,
    /// Reinforcement learning from rewards/punishments.
    Reinforcement,
    /// Unsupervised learning from patterns.
    Unsupervised,
    /// Semi-supervised learning.
    SemiSupervised,
    /// Online/incremental learning.
    Online,
    /// Transfer learning from another domain.
    Transfer,
}

impl LearningMode {
    /// Returns the string label of this learning mode.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Supervised => "supervised",
            Self::Reinforcement => "reinforcement",
            Self::Unsupervised => "unsupervised",
            Self::SemiSupervised => "semi_supervised",
            Self::Online => "online",
            Self::Transfer => "transfer",
        }
    }

    /// Returns whether this mode requires external labels.
    pub fn requires_labels(&self) -> bool {
        matches!(self, Self::Supervised | Self::SemiSupervised)
    }

    /// Returns whether this mode supports incremental updates.
    pub fn is_incremental(&self) -> bool {
        matches!(self, Self::Online | Self::Reinforcement)
    }
}

impl std::fmt::Display for LearningMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for LearningMode {
    fn default() -> Self {
        Self::Online
    }
}

