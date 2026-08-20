// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// SelfReference: The system's ability to refer to its own state.
///
/// Self-reference enables the cognitive system to examine
/// and reason about its own internal processes, creating
/// a meta-cognitive loop. The reference depth controls how
/// many levels of self-examination are possible, while
/// coherence measures the consistency of those self-references.
///
/// # Fields
/// - `reference_depth`: Number of self-reference levels, in [0, usize::MAX].
/// - `coherence`: Consistency of self-referential data, in [0.0, 1.0].
///
/// # Example
/// ```
/// use breakthrough::cognition::identity::SelfReference;
///
/// let mut sref = SelfReference::new(3, 0.9).expect("valid parameters");
/// sref.deepen();
/// assert_eq!(sref.depth(), 4);
/// ```
pub struct SelfReference {
    /// Number of self-reference levels, in [0, 100].
    pub reference_depth: usize,
    /// Consistency of self-referential data, in [0.0, 1.0].
    pub coherence: f64,
}

impl SelfReference {
    /// Creates a new `SelfReference` with the given depth and coherence.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `reference_depth` exceeds 100
    /// or `coherence` is outside [0.0, 1.0].
    pub fn new(reference_depth: usize, coherence: f64) -> Result<Self, CognitionError> {
        if reference_depth > 100 {
            return Err(CognitionError::OutOfRange {
                field: "reference_depth".to_string(),
                value: reference_depth as f64,
                min: 0.0,
                max: 100.0,
            });
        }
        if !(0.0..=1.0).contains(&coherence) {
            return Err(CognitionError::OutOfRange {
                field: "coherence".to_string(),
                value: coherence,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            reference_depth,
            coherence,
        })
    }

    /// Deepens the self-reference by one level.
    ///
    /// # Errors
    /// Returns [`CognitionError::CapacityExceeded`] if depth exceeds 100.
    pub fn deepen(&mut self) -> Result<(), CognitionError> {
        if self.reference_depth >= 100 {
            return Err(CognitionError::CapacityExceeded {
                max: 100,
                attempted: self.reference_depth + 1,
            });
        }
        self.reference_depth += 1;
        Ok(())
    }

    /// Reduces the self-reference depth by one level (ascent).
    ///
    /// Returns `true` if depth was reduced, `false` if already at zero.
    pub fn ascend(&mut self) -> bool {
        if self.reference_depth > 0 {
            self.reference_depth -= 1;
            true
        } else {
            false
        }
    }

    /// Returns the current reference depth.
    pub fn depth(&self) -> usize {
        self.reference_depth
    }

    /// Adjusts coherence based on a consistency check result.
    ///
    /// If `consistent` is true, coherence increases; otherwise, it decreases.
    pub fn update_coherence(&mut self, consistent: bool) {
        if consistent {
            self.coherence = (self.coherence + 0.05).min(1.0);
        } else {
            self.coherence = (self.coherence - 0.1).max(0.0);
        }
    }

    /// Returns `true` if the self-reference is considered coherent
    /// (coherence >= 0.5).
    pub fn is_coherent(&self) -> bool {
        self.coherence >= 0.5
    }

    /// Validates all field values are within valid ranges.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if self.reference_depth > 100 {
            return Err(CognitionError::OutOfRange {
                field: "reference_depth".to_string(),
                value: self.reference_depth as f64,
                min: 0.0,
                max: 100.0,
            });
        }
        if !(0.0..=1.0).contains(&self.coherence) {
            return Err(CognitionError::OutOfRange {
                field: "coherence".to_string(),
                value: self.coherence,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}

impl fmt::Debug for SelfReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SelfReference")
            .field("reference_depth", &self.reference_depth)
            .field("coherence", &self.coherence)
            .finish()
    }
}