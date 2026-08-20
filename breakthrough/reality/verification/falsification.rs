// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::RealityError;

/// FalsificationCriterion: A criterion that can falsify a prediction.
///
/// Falsification criteria define the conditions under which a
/// prediction is considered false, following Popper's principle
/// of falsifiability.
#[derive(Debug, Clone, PartialEq)]
pub struct FalsificationCriterion {
    /// Name of this criterion.
    pub name: String,
    /// The predicted value that would falsify the prediction.
    pub threshold: f64,
    /// Whether values below or above the threshold falsify.
    pub direction: FalsificationDirection,
    /// The tolerance allowed around the threshold.
    pub tolerance: f64,
}

impl FalsificationCriterion {
    /// Creates a new falsification criterion.
    pub fn new(
        name: impl Into<String>,
        threshold: f64,
        direction: FalsificationDirection,
        tolerance: f64,
    ) -> Self {
        Self {
            name: name.into(),
            threshold,
            direction,
            tolerance: tolerance.max(0.0),
        }
    }

    /// Evaluates whether an observed value falsifies the prediction.
    pub fn evaluates(&self, observed: f64) -> bool {
        let lower = self.threshold - self.tolerance;
        let upper = self.threshold + self.tolerance;
        match self.direction {
            FalsificationDirection::Above => observed > upper,
            FalsificationDirection::Below => observed < lower,
            FalsificationDirection::Outside => observed < lower || observed > upper,
            FalsificationDirection::Deviation => (observed - self.threshold).abs() > self.tolerance,
        }
    }

    /// Returns whether the observed value passes the criterion.
    pub fn passes(&self, observed: f64) -> bool {
        !self.evaluates(observed)
    }
}

/// FalsificationDirection: The direction in which a value falsifies a prediction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FalsificationDirection {
    /// Falsified if observed value is above threshold.
    Above,
    /// Falsified if observed value is below threshold.
    Below,
    /// Falsified if observed value is outside [threshold - tol, threshold + tol].
    Outside,
    /// Falsified if observed deviates from threshold by more than tolerance.
    Deviation,
}

impl std::fmt::Display for FalsificationDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Above => write!(f, "above"),
            Self::Below => write!(f, "below"),
            Self::Outside => write!(f, "outside"),
            Self::Deviation => write!(f, "deviation"),
        }
    }
}

