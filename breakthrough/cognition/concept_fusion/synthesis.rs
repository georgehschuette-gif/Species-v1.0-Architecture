// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// ConceptSynthesis: Building higher-order concepts from lower ones.
///
/// Synthesis constructs abstract concepts from concrete ones
/// by iteratively combining and abstracting concepts across
/// multiple depth levels. The synthesis depth controls how
/// many levels of abstraction are achieved.
///
/// # Fields
/// - `synthesis_depth`: Number of abstraction levels to perform, in [0, usize::MAX].
/// - `abstraction_level`: Current level of abstraction achieved, in [0.0, 1.0].
///
/// # Example
/// ```
/// use breakthrough::cognition::concept_fusion::ConceptSynthesis;
///
/// let mut synth = ConceptSynthesis::new(0, 0.0).expect("valid parameters");
/// synth.abstract_up().unwrap();
/// assert_eq!(synth.depth(), 1);
/// ```
pub struct ConceptSynthesis {
    /// Number of abstraction levels to perform, in [0, 100].
    pub synthesis_depth: usize,
    /// Current level of abstraction achieved, in [0.0, 1.0].
    pub abstraction_level: f64,
    /// Maximum allowed synthesis depth.
    max_depth: usize,
}

impl ConceptSynthesis {
    /// Creates a new `ConceptSynthesis` with the given depth and abstraction level.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `synthesis_depth` exceeds 100
    /// or `abstraction_level` is outside [0.0, 1.0].
    pub fn new(synthesis_depth: usize, abstraction_level: f64) -> Result<Self, CognitionError> {
        if synthesis_depth > 100 {
            return Err(CognitionError::OutOfRange {
                field: "synthesis_depth".to_string(),
                value: synthesis_depth as f64,
                min: 0.0,
                max: 100.0,
            });
        }
        if !(0.0..=1.0).contains(&abstraction_level) {
            return Err(CognitionError::OutOfRange {
                field: "abstraction_level".to_string(),
                value: abstraction_level,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            synthesis_depth,
            abstraction_level,
            max_depth: 100,
        })
    }

    /// Advances the synthesis by one level of abstraction.
    ///
    /// Each level increases the abstraction level proportionally
    /// to the remaining headroom. Returns an error if the maximum
    /// depth has been reached.
    ///
    /// # Errors
    /// Returns [`CognitionError::CapacityExceeded`] if the depth limit is reached.
    pub fn abstract_up(&mut self) -> Result<(), CognitionError> {
        if self.synthesis_depth >= self.max_depth {
            return Err(CognitionError::CapacityExceeded {
                max: self.max_depth,
                attempted: self.synthesis_depth + 1,
            });
        }
        let headroom = 1.0 - self.abstraction_level;
        let increment = self.abstraction_level.max(0.1) * headroom;
        self.abstraction_level = (self.abstraction_level + increment).min(1.0);
        self.synthesis_depth += 1;
        Ok(())
    }

    /// Advances synthesis by multiple levels at once.
    ///
    /// # Errors
    /// Returns [`CognitionError::CapacityExceeded`] if levels would exceed the maximum.
    pub fn abstract_up_by(&mut self, levels: usize) -> Result<(), CognitionError> {
        for _ in 0..levels {
            self.abstract_up()?;
        }
        Ok(())
    }

    /// Returns the current synthesis depth.
    pub fn depth(&self) -> usize {
        self.synthesis_depth
    }

    /// Returns the current abstraction level.
    pub fn level(&self) -> f64 {
        self.abstraction_level
    }

    /// Computes the conceptual complexity as `depth * abstraction_level`.
    pub fn complexity(&self) -> f64 {
        self.synthesis_depth as f64 * self.abstraction_level
    }

    /// Resets the synthesis to its initial state.
    pub fn reset(&mut self) {
        self.synthesis_depth = 0;
        self.abstraction_level = 0.0;
    }

    /// Validates all field values are within valid ranges.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] for invalid values.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if self.synthesis_depth > self.max_depth {
            return Err(CognitionError::OutOfRange {
                field: "synthesis_depth".to_string(),
                value: self.synthesis_depth as f64,
                min: 0.0,
                max: self.max_depth as f64,
            });
        }
        if !(0.0..=1.0).contains(&self.abstraction_level) {
            return Err(CognitionError::OutOfRange {
                field: "abstraction_level".to_string(),
                value: self.abstraction_level,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}

impl fmt::Debug for ConceptSynthesis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConceptSynthesis")
            .field("synthesis_depth", &self.synthesis_depth)
            .field("abstraction_level", &self.abstraction_level)
            .finish()
    }
}