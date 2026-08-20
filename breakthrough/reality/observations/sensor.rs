// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::HashMap;
use std::fmt;

use crate::RealityError;

/// SensoryModality: The type of sensory input channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SensoryModality {
    /// Visual input (light, color, shape, motion).
    Visual,
    /// Auditory input (sound, speech, frequency).
    Auditory,
    /// Tactile input (pressure, texture, temperature).
    Tactile,
    /// Olfactory input (smell, chemical signals).
    Olfactory,
    /// Gustatory input (taste).
    Gustatory,
    /// Proprioceptive input (body position, movement).
    Proprioceptive,
    /// Vestibular input (balance, spatial orientation).
    Vestibular,
    /// Abstract/derived modality (conceptual, symbolic).
    Abstract,
}

impl SensoryModality {
    /// Returns the string label of this sensory modality.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Visual => "visual",
            Self::Auditory => "auditory",
            Self::Tactile => "tactile",
            Self::Olfactory => "olfactory",
            Self::Gustatory => "gustatory",
            Self::Proprioceptive => "proprioceptive",
            Self::Vestibular => "vestibular",
            Self::Abstract => "abstract",
        }
    }

    /// Returns the bandwidth class of this modality (rough estimate).
    pub fn bandwidth_class(&self) -> BandwidthClass {
        match self {
            Self::Visual => BandwidthClass::High,
            Self::Auditory => BandwidthClass::Medium,
            Self::Tactile => BandwidthClass::Medium,
            Self::Olfactory => BandwidthClass::Low,
            Self::Gustatory => BandwidthClass::Low,
            Self::Proprioceptive => BandwidthClass::Medium,
            Self::Vestibular => BandwidthClass::Low,
            Self::Abstract => BandwidthClass::Variable,
        }
    }

    /// Returns whether this modality is typically high-bandwidth.
    pub fn is_high_bandwidth(&self) -> bool {
        matches!(self.bandwidth_class(), BandwidthClass::High)
    }

    /// Returns the typical latency range for this modality in milliseconds.
    pub fn typical_latency_ms(&self) -> (u64, u64) {
        match self {
            Self::Visual => (20, 50),
            Self::Auditory => (5, 20),
            Self::Tactile => (10, 40),
            Self::Olfactory => (100, 500),
            Self::Gustatory => (100, 500),
            Self::Proprioceptive => (5, 30),
            Self::Vestibular => (5, 20),
            Self::Abstract => (1, 100),
        }
    }
}

impl std::fmt::Display for SensoryModality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for SensoryModality {
    fn default() -> Self {
        Self::Visual
    }
}

/// BandwidthClass: Categorizes sensory modalities by information throughput.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BandwidthClass {
    /// Low information throughput (olfactory, gustatory, vestibular).
    Low,
    /// Medium information throughput (auditory, tactile, proprioceptive).
    Medium,
    /// High information throughput (visual).
    High,
    /// Variable bandwidth depending on context (abstract).
    Variable,
}

