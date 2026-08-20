// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// EmotionalArousal: Intensity and activation level of affect.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EmotionalArousal(pub f64);

impl PartialOrd for EmotionalArousal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for EmotionalArousal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl Eq for EmotionalArousal {}

impl EmotionalArousal {
    pub const MIN: f64 = 0.0;
    pub const MAX: f64 = 1.0;

    pub fn new(value: f64) -> Result<Self, EmotionalError> {
        if !value.is_finite() {
            return Err(EmotionalError::InvalidValue { value });
        }
        Ok(Self(value.clamp(Self::MIN, Self::MAX)))
    }

    pub fn is_calm(&self) -> bool { self.0 < 0.3 }
    pub fn is_activated(&self) -> bool { self.0 > 0.7 }
    pub fn intensity_level(&self) -> &'static str {
        if self.0 < 0.3 { "calm" } else if self.0 < 0.7 { "moderate" } else { "high" }
    }
}
