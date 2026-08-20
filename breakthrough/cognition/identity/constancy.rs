// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// IdentityConstancy: Resisting change to core identity features.
///
/// Constancy protects the fundamental aspects of a concept's identity
/// from modification by external influences. Core features are
/// designated as immutable or semi-immutable, ensuring that
/// identity persists through perturbation.
///
/// # Fields
/// - `constancy_threshold`: Minimum protection level for core features, in [0.0, 1.0].
/// - `core_features`: Set of feature IDs designated as core to identity.
pub struct IdentityConstancy {
    /// Minimum protection level for core features, in [0.0, 1.0].
    pub constancy_threshold: f64,
    /// Set of feature IDs designated as core to identity.
    pub core_features: Vec<u64>,
}

impl IdentityConstancy {
    /// Creates a new `IdentityConstancy` with the given threshold and core features.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `constancy_threshold` is outside [0.0, 1.0].
    pub fn new(
        constancy_threshold: f64,
        core_features: Vec<u64>,
    ) -> Result<Self, CognitionError> {
        if !(0.0..=1.0).contains(&constancy_threshold) {
            return Err(CognitionError::OutOfRange {
                field: "constancy_threshold".to_string(),
                value: constancy_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        if core_features.is_empty() {
            return Err(CognitionError::MissingInput(
                "at least one core feature is required".to_string(),
            ));
        }
        Ok(Self {
            constancy_threshold,
            core_features,
        })
    }

    /// Determines whether a given feature is protected as a core identity feature.
    pub fn is_core(&self, feature_id: u64) -> bool {
        self.core_features.contains(&feature_id)
    }

    /// Attempts to modify a core feature value. If the feature is core and
    /// the modification exceeds the constancy threshold, it is rejected.
    ///
    /// Returns `true` if the modification was allowed, `false` if rejected.
    pub fn try_modify_core(
        &self,
        feature_id: u64,
        proposed_change: f64,
    ) -> Result<bool, CognitionError> {
        if !(0.0..=1.0).contains(&proposed_change) {
            return Err(CognitionError::OutOfRange {
                field: "proposed_change".to_string(),
                value: proposed_change,
                min: 0.0,
                max: 1.0,
            });
        }
        if self.is_core(feature_id) {
            if proposed_change > self.constancy_threshold {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Returns the number of protected core features.
    pub fn core_count(&self) -> usize {
        self.core_features.len()
    }

    /// Adds a feature to the core identity set.
    ///
    /// # Errors
    /// Returns [`CognitionError::CapacityExceeded`] if more than 1000 core features.
    pub fn add_core_feature(&mut self, feature_id: u64) -> Result<(), CognitionError> {
        if self.core_features.len() >= 1000 {
            return Err(CognitionError::CapacityExceeded {
                max: 1000,
                attempted: self.core_features.len() + 1,
            });
        }
        if !self.core_features.contains(&feature_id) {
            self.core_features.push(feature_id);
        }
        Ok(())
    }

    /// Removes a feature from the core identity set.
    ///
    /// Returns `true` if the feature was present and removed, `false` otherwise.
    pub fn remove_core_feature(&mut self, feature_id: u64) -> bool {
        if let Some(pos) = self.core_features.iter().position(|&f| f == feature_id) {
            self.core_features.remove(pos);
            true
        } else {
            false
        }
    }

    /// Computes the identity stability score as the product of
    /// the constancy threshold and the count of core features,
    /// normalized by the maximum possible.
    pub fn stability(&self) -> f64 {
        self.constancy_threshold * self.core_features.len() as f64 / 1000.0
    }

    /// Updates the constancy threshold.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if the new threshold is outside [0.0, 1.0].
    pub fn set_threshold(&mut self, new_threshold: f64) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&new_threshold) {
            return Err(CognitionError::OutOfRange {
                field: "constancy_threshold".to_string(),
                value: new_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        self.constancy_threshold = new_threshold;
        Ok(())
    }

    /// Validates all field values are within valid ranges.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&self.constancy_threshold) {
            return Err(CognitionError::OutOfRange {
                field: "constancy_threshold".to_string(),
                value: self.constancy_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        if self.core_features.is_empty() {
            return Err(CognitionError::MissingInput(
                "core_features must not be empty".to_string(),
            ));
        }
        if self.core_features.len() > 1000 {
            return Err(CognitionError::CapacityExceeded {
                max: 1000,
                attempted: self.core_features.len(),
            });
        }
        Ok(())
    }
}

impl fmt::Debug for IdentityConstancy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IdentityConstancy")
            .field("constancy_threshold", &self.constancy_threshold)
            .field("core_features", &self.core_features)
            .finish()
    }
}