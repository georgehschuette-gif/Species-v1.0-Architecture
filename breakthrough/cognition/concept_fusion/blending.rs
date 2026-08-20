// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// FeatureSelection: The method used to select features during blending.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureSelection {
    /// Only features present in all source concepts are retained.
    Intersection,
    /// All features from all source concepts are retained.
    Union,
    /// Features present in exactly one source concept are retained.
    SymmetricDifference,
}

/// ConceptBlending: Creating new concepts from feature combinations.
///
/// Blending generates novel concepts by cross-pollinating features
/// from existing concepts. The blend factor controls how much
/// each source contributes, and the feature selection strategy
/// determines which features survive the combination.
///
/// # Fields
/// - `blend_factor`: Weight given to each source concept, in [0.0, 1.0].
/// - `feature_selection`: The strategy for selecting surviving features.
///
/// # Example
/// ```
/// use breakthrough::cognition::concept_fusion::{ConceptBlending, FeatureSelection};
///
/// let mut blender = ConceptBlending::new(0.5, FeatureSelection::Intersection)
///     .expect("valid parameters");
/// let novel = blender.blend(&[0.8, 0.6]).expect("blend succeeded");
/// assert!(novel > 0.0);
/// ```
pub struct ConceptBlending {
    /// Weight given to each source concept, in [0.0, 1.0].
    pub blend_factor: f64,
    /// The strategy for selecting surviving features.
    pub feature_selection: FeatureSelection,
}

impl ConceptBlending {
    /// Creates a new `ConceptBlending` with the given factor and selection strategy.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `blend_factor` is outside [0.0, 1.0].
    pub fn new(blend_factor: f64, feature_selection: FeatureSelection) -> Result<Self, CognitionError> {
        if !(0.0..=1.0).contains(&blend_factor) {
            return Err(CognitionError::OutOfRange {
                field: "blend_factor".to_string(),
                value: blend_factor,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            blend_factor,
            feature_selection,
        })
    }

    /// Blends the given concept strengths into a novel concept.
    ///
    /// The result is computed as the weighted blend of all inputs,
    /// modulated by the blend factor and the feature selection strategy.
    ///
    /// # Errors
    /// Returns [`CognitionError::MissingInput`] if the strengths slice is empty.
    /// Returns [`CognitionError::OutOfRange`] if any strength is outside [0.0, 1.0].
    pub fn blend(&self, strengths: &[f64]) -> Result<f64, CognitionError> {
        if strengths.is_empty() {
            return Err(CognitionError::MissingInput(
                "cannot blend zero concepts".to_string(),
            ));
        }
        for &s in strengths {
            if !(0.0..=1.0).contains(&s) {
                return Err(CognitionError::OutOfRange {
                    field: "strength".to_string(),
                    value: s,
                    min: 0.0,
                    max: 1.0,
                });
            }
        }
        let raw_blend: f64 = strengths.iter().sum::<f64>() / strengths.len() as f64;
        let weighted = raw_blend * self.blend_factor;
        let adjusted = match self.feature_selection {
            FeatureSelection::Intersection => weighted * 1.2,
            FeatureSelection::Union => weighted * 0.9,
            FeatureSelection::SymmetricDifference => weighted * 1.05,
        };
        Ok(adjusted.min(1.0).max(0.0))
    }

    /// Selects a subset of features based on the chosen strategy.
    ///
    /// Returns the indices of features that survive selection.
    ///
    /// # Errors
    /// Returns [`CognitionError::MissingInput`] if the features slice is empty.
    pub fn select_features(
        &self,
        features: &[f64],
    ) -> Result<Vec<usize>, CognitionError> {
        if features.is_empty() {
            return Err(CognitionError::MissingInput(
                "features slice must not be empty".to_string(),
            ));
        }
        let selected: Vec<usize> = match self.feature_selection {
            FeatureSelection::Intersection => features
                .iter()
                .enumerate()
                .filter(|(_, &v)| v >= self.blend_factor)
                .map(|(i, _)| i)
                .collect(),
            FeatureSelection::Union => (0..features.len()).collect(),
            FeatureSelection::SymmetricDifference => features
                .iter()
                .enumerate()
                .filter(|(_, &v)| v.abs() >= self.blend_factor)
                .map(|(i, _)| i)
                .collect(),
        };
        Ok(selected)
    }

    /// Adjusts the blend factor toward a target value.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if the target blend factor is outside [0.0, 1.0].
    pub fn set_blend_factor(&mut self, target: f64) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&target) {
            return Err(CognitionError::OutOfRange {
                field: "blend_factor".to_string(),
                value: target,
                min: 0.0,
                max: 1.0,
            });
        }
        self.blend_factor = target;
        Ok(())
    }

    /// Validates all field values are within valid ranges.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&self.blend_factor) {
            return Err(CognitionError::OutOfRange {
                field: "blend_factor".to_string(),
                value: self.blend_factor,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}
