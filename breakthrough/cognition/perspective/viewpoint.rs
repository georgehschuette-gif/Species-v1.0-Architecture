// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// CognitiveViewpoint: The current perspective from which cognition operates.
///
/// The viewpoint defines the focal point of cognitive operations,
/// determining which aspects of the environment are attended to
/// and how information is prioritized. The focus area represents
/// the salient dimensions of the current perspective.
///
/// # Fields
/// - `viewpoint_id`: Unique identifier for this viewpoint.
/// - `focus_area`: Normalized coordinates indicating the focus region, values in [0.0, 1.0].
///
/// # Example
/// ```
/// use breakthrough::cognition::perspective::CognitiveViewpoint;
///
/// let mut vp = CognitiveViewpoint::new(1, vec![0.5, 0.3]).expect("valid parameters");
/// vp.shift_focus(0.7, 0.2).expect("shift succeeded");
/// assert_eq!(vp.focus_area()[0], 0.7);
/// ```
pub struct CognitiveViewpoint {
    /// Unique identifier for this viewpoint.
    pub viewpoint_id: u64,
    /// Normalized coordinates indicating the focus region, values in [0.0, 1.0].
    pub focus_area: Vec<f64>,
}

impl CognitiveViewpoint {
    /// Creates a new `CognitiveViewpoint` with the given ID and focus area.
    ///
    /// # Errors
    /// Returns [`CognitionError::MissingInput`] if the focus area is empty.
    /// Returns [`CognitionError::OutOfRange`] if any focus coordinate is outside [0.0, 1.0].
    pub fn new(viewpoint_id: u64, focus_area: Vec<f64>) -> Result<Self, CognitionError> {
        if focus_area.is_empty() {
            return Err(CognitionError::MissingInput(
                "focus_area must not be empty".to_string(),
            ));
        }
        for (i, &coord) in focus_area.iter().enumerate() {
            if !(0.0..=1.0).contains(&coord) {
                return Err(CognitionError::OutOfRange {
                    field: format!("focus_area[{}]", i),
                    value: coord,
                    min: 0.0,
                    max: 1.0,
                });
            }
        }
        Ok(Self {
            viewpoint_id,
            focus_area,
        })
    }

    /// Shifts the focus area to new coordinates.
    ///
    /// The new focus area must have the same dimensionality as the current one.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if dimensions mismatch or values are invalid.
    pub fn shift_focus(&mut self, x: f64, y: f64) -> Result<(), CognitionError> {
        if self.focus_area.len() != 2 {
            return Err(CognitionError::IncompatibleConcepts {
                reason: format!(
                    "new focus has {} dimensions, expected 2",
                    self.focus_area.len()
                ),
            });
        }
        if !(0.0..=1.0).contains(&x) {
            return Err(CognitionError::OutOfRange {
                field: "focus_area[0]".to_string(),
                value: x,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&y) {
            return Err(CognitionError::OutOfRange {
                field: "focus_area[1]".to_string(),
                value: y,
                min: 0.0,
                max: 1.0,
            });
        }
        self.focus_area[0] = x;
        self.focus_area[1] = y;
        Ok(())
    }

    /// Shifts focus by a delta vector (additive).
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if dimensions mismatch or resulting values are out of bounds.
    pub fn shift_focus_delta(&mut self, delta: &[f64]) -> Result<(), CognitionError> {
        if delta.len() != self.focus_area.len() {
            return Err(CognitionError::IncompatibleConcepts {
                reason: format!(
                    "delta has {} dimensions, expected {}",
                    delta.len(),
                    self.focus_area.len()
                ),
            });
        }
        let new_focus: Vec<f64> = self
            .focus_area
            .iter()
            .zip(delta.iter())
            .map(|(&current, &d)| current + d)
            .collect();
        for (i, &coord) in new_focus.iter().enumerate() {
            if !(0.0..=1.0).contains(&coord) {
                return Err(CognitionError::OutOfRange {
                    field: format!("focus_area[{}] after delta", i),
                    value: coord,
                    min: 0.0,
                    max: 1.0,
                });
            }
        }
        self.focus_area = new_focus;
        Ok(())
    }

    /// Returns the dimensionality of the focus area.
    pub fn focus_dimensionality(&self) -> usize {
        self.focus_area.len()
    }

    /// Computes the Euclidean distance from the current focus to a target point.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if the target has wrong dimensionality.
    pub fn distance_to(&self, target: &[f64]) -> Result<f64, CognitionError> {
        if target.len() != self.focus_area.len() {
            return Err(CognitionError::IncompatibleConcepts {
                reason: format!(
                    "target has {} dimensions, expected {}",
                    target.len(),
                    self.focus_area.len()
                ),
            });
        }
        let dist: f64 = self
            .focus_area
            .iter()
            .zip(target.iter())
            .map(|(&a, &b)| {
                let d = a - b;
                d * d
            })
            .sum();
        Ok(dist.sqrt())
    }

    /// Returns the current focus area as a slice.
    pub fn focus_area(&self) -> &[f64] {
        &self.focus_area
    }

    /// Updates the viewpoint ID.
    pub fn set_id(&mut self, new_id: u64) {
        self.viewpoint_id = new_id;
    }

    /// Validates all field values are within valid ranges.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if self.focus_area.is_empty() {
            return Err(CognitionError::MissingInput(
                "focus_area must not be empty".to_string(),
            ));
        }
        for (i, &coord) in self.focus_area.iter().enumerate() {
            if !(0.0..=1.0).contains(&coord) {
                return Err(CognitionError::OutOfRange {
                    field: format!("focus_area[{}]", i),
                    value: coord,
                    min: 0.0,
                    max: 1.0,
                });
            }
        }
        Ok(())
    }
}

impl fmt::Debug for CognitiveViewpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CognitiveViewpoint")
            .field("viewpoint_id", &self.viewpoint_id)
            .field("focus_area", &self.focus_area)
            .finish()
    }
}