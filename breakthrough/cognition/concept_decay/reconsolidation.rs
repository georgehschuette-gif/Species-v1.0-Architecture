// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// ConceptReconsolidation: Strengthening concepts during reactivation.
///
/// Reconsolidation models the process whereby a previously activated
/// and then weakened concept is stabilized anew when it is brought
/// back into active use. The consolidation factor determines how
/// much strength is recovered during reactivation.
///
/// # Fields
/// - `consolidation_factor`: Fraction of lost strength recovered per reactivation, in (0.0, 1.0].
/// - `stability_window`: Time window during which reactivation confers extra stability, in > 0.0.
pub struct ConceptReconsolidation {
    /// Fraction of lost strength recovered per reactivation, in (0.0, 1.0].
    pub consolidation_factor: f64,
    /// Time window during which reactivation confers extra stability, in > 0.0.
    pub stability_window: f64,
}

impl ConceptReconsolidation {
    /// Creates a new `ConceptReconsolidation` with the given factor and window.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `consolidation_factor` is outside (0.0, 1.0]
    /// or `stability_window` is not positive.
    pub fn new(consolidation_factor: f64, stability_window: f64) -> Result<Self, CognitionError> {
        if consolidation_factor <= 0.0 || consolidation_factor > 1.0 {
            return Err(CognitionError::OutOfRange {
                field: "consolidation_factor".to_string(),
                value: consolidation_factor,
                min: 0.0,
                max: 1.0,
            });
        }
        if stability_window <= 0.0 {
            return Err(CognitionError::OutOfRange {
                field: "stability_window".to_string(),
                value: stability_window,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        Ok(Self {
            consolidation_factor,
            stability_window,
        })
    }

    /// Recovers strength for a concept being reconsolidated.
    ///
    /// The recovered strength is computed as:
    /// `current_strength + consolidation_factor * (1.0 - current_strength)`.
    /// If the elapsed time since last activation is within the stability window,
    /// an additional bonus is applied.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `current_strength` is outside [0.0, 1.0]
    /// or `elapsed_time` is negative.
    pub fn reconsolidate(
        &self,
        current_strength: f64,
        elapsed_time: f64,
    ) -> Result<f64, CognitionError> {
        if !(0.0..=1.0).contains(&current_strength) {
            return Err(CognitionError::OutOfRange {
                field: "current_strength".to_string(),
                value: current_strength,
                min: 0.0,
                max: 1.0,
            });
        }
        if elapsed_time < 0.0 {
            return Err(CognitionError::OutOfRange {
                field: "elapsed_time".to_string(),
                value: elapsed_time,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        let recovered = current_strength
            + self.consolidation_factor * (1.0 - current_strength);
        let in_window = elapsed_time <= self.stability_window;
        let bonus = if in_window {
            self.consolidation_factor * 0.5
        } else {
            0.0
        };
        Ok((recovered + bonus).min(1.0))
    }

    /// Returns `true` if the elapsed time falls within the stability window,
    /// granting bonus consolidation benefits.
    pub fn in_stability_window(&self, elapsed_time: f64) -> bool {
        elapsed_time >= 0.0 && elapsed_time <= self.stability_window
    }

    /// Computes the effective consolidation rate adjusted for elapsed time.
    /// The rate decays linearly as elapsed time exceeds the stability window.
    pub fn effective_factor(&self, elapsed_time: f64) -> f64 {
        if elapsed_time <= self.stability_window {
            self.consolidation_factor
        } else {
            let overshoot = elapsed_time - self.stability_window;
            let decay = (overshoot / self.stability_window).min(1.0);
            self.consolidation_factor * (1.0 - decay)
        }
    }

    /// Validates all field values are within valid ranges.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] for invalid parameters.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if self.consolidation_factor <= 0.0 || self.consolidation_factor > 1.0 {
            return Err(CognitionError::OutOfRange {
                field: "consolidation_factor".to_string(),
                value: self.consolidation_factor,
                min: 0.0,
                max: 1.0,
            });
        }
        if self.stability_window <= 0.0 {
            return Err(CognitionError::OutOfRange {
                field: "stability_window".to_string(),
                value: self.stability_window,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        Ok(())
    }
}

impl fmt::Debug for ConceptReconsolidation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConceptReconsolidation")
            .field("consolidation_factor", &self.consolidation_factor)
            .field("stability_window", &self.stability_window)
            .finish()
    }
}