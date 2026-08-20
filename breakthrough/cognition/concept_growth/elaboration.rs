// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// ConceptElaboration: Adding detail and nuance to existing concepts.
///
/// Elaboration enriches a concept by increasing its depth and
/// branching factor, creating a richer internal structure that
/// captures finer distinctions and more nuanced associations.
///
/// # Fields
/// - `elaboration_depth`: Number of detail layers applied, in [0, usize::MAX].
/// - `branching_factor`: Number of sub-branches per elaboration level, in [0.0, 10.0].
///
/// # Example
/// ```
/// use breakthrough::cognition::concept_growth::ConceptElaboration;
///
/// let mut elab = ConceptElaboration::new(2, 1.5).expect("valid parameters");
/// elab.deepen();
/// assert_eq!(elab.depth(), 3);
/// ```
pub struct ConceptElaboration {
    /// Number of detail layers applied, in [0, usize::MAX].
    pub elaboration_depth: usize,
    /// Number of sub-branches per elaboration level, in [0.0, 10.0].
    pub branching_factor: f64,
}

impl ConceptElaboration {
    /// Creates a new `ConceptElaboration` with the given depth and branching factor.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `branching_factor` is outside [0.0, 10.0]
    /// or if `elaboration_depth` exceeds a reasonable maximum of 100.
    pub fn new(elaboration_depth: usize, branching_factor: f64) -> Result<Self, CognitionError> {
        if elaboration_depth > 100 {
            return Err(CognitionError::OutOfRange {
                field: "elaboration_depth".to_string(),
                value: elaboration_depth as f64,
                min: 0.0,
                max: 100.0,
            });
        }
        if !(0.0..=10.0).contains(&branching_factor) {
            return Err(CognitionError::OutOfRange {
                field: "branching_factor".to_string(),
                value: branching_factor,
                min: 0.0,
                max: 10.0,
            });
        }
        Ok(Self {
            elaboration_depth,
            branching_factor,
        })
    }

    /// Deepens the elaboration by one layer, multiplying the effective
    /// complexity by the branching factor.
    ///
    /// # Errors
    /// Returns [`CognitionError::CapacityExceeded`] if depth exceeds 100.
    pub fn deepen(&mut self) -> Result<(), CognitionError> {
        if self.elaboration_depth >= 100 {
            return Err(CognitionError::CapacityExceeded {
                max: 100,
                attempted: self.elaboration_depth + 1,
            });
        }
        self.elaboration_depth += 1;
        Ok(())
    }

    /// Increases elaboration depth by the given number of layers.
    ///
    /// # Errors
    /// Returns [`CognitionError::CapacityExceeded`] if the resulting depth exceeds 100.
    pub fn deepen_by(&mut self, layers: usize) -> Result<(), CognitionError> {
        let new_depth = self.elaboration_depth + layers;
        if new_depth > 100 {
            return Err(CognitionError::CapacityExceeded {
                max: 100,
                attempted: new_depth,
            });
        }
        self.elaboration_depth = new_depth;
        Ok(())
    }

    /// Returns the current elaboration depth.
    pub fn depth(&self) -> usize {
        self.elaboration_depth
    }

    /// Computes the effective complexity as `depth * branching_factor`.
    pub fn complexity(&self) -> f64 {
        self.elaboration_depth as f64 * self.branching_factor
    }

    /// Validates all field values are within valid ranges.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] for invalid values.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if self.elaboration_depth > 100 {
            return Err(CognitionError::OutOfRange {
                field: "elaboration_depth".to_string(),
                value: self.elaboration_depth as f64,
                min: 0.0,
                max: 100.0,
            });
        }
        if !(0.0..=10.0).contains(&self.branching_factor) {
            return Err(CognitionError::OutOfRange {
                field: "branching_factor".to_string(),
                value: self.branching_factor,
                min: 0.0,
                max: 10.0,
            });
        }
        Ok(())
    }
}

impl fmt::Debug for ConceptElaboration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConceptElaboration")
            .field("elaboration_depth", &self.elaboration_depth)
            .field("branching_factor", &self.branching_factor)
            .finish()
    }
}