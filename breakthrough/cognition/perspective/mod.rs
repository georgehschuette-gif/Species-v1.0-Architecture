// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Perspective: The viewpoint-dependent filtering of cognitive content.
//!
//! Perspective shapes *what* the system notices and *how* it interprets
//! percepts. Three mechanisms collaborate to produce viewpoint effects:
//!
//! 1. **Viewpoint** — Defines the focal point in perceptual space,
//!    determining salience allocation.
//! 2. **Framing** — Applies an interpretive lens (analytic, holistic,
//!    pragmatic, critical) that modulates strength and rigidity.
//! 3. **Bias** — Introduces systematic directional distortions that
//!    amplify or suppress perceptual dimensions.
//!
//! # Viewpoint Dynamics
//!
//! The viewpoint can be shifted incrementally or absolutely. Shifts compute
//! distance to target to ensure transitions are smooth and bounded.
//!
//! # Frame Influence
//!
//! Each frame type carries an inherent rigidity multiplier:
//! - Analytic: 0.8 (relatively flexible)
//! - Pragmatic: 0.9 (firm but adaptable)
//! - Critical: 0.7 (self-questioning)
//! - Holistic: 0.6 (most fluid)
//!
//! # Bias Vectors
//!
//! Bias vectors operate in the same dimension as perceptual inputs.
//! Components range from -1.0 (pure suppression) to +1.0 (pure amplification).
//!
//! # Examples
//!
//! ```
//! use breakthrough::cognition::perspective::{CognitiveViewpoint, CognitiveFraming, PerspectiveBias, FrameType};
//!
//! let mut vp = CognitiveViewpoint::new(1, vec![0.5, 0.3]).expect("valid");
//! vp.shift_focus(0.7, 0.2).expect("shifted");
//!
//! let mut frame = CognitiveFraming::new(FrameType::Analytic, 0.7).expect("valid");
//! frame.strengthen(0.1).expect("strengthened");
//!
//! let bias = PerspectiveBias::new(vec![0.5, -0.3, 0.1], 0.4).expect("valid");
//! assert!(!bias.is_neutral());
//! ```
//!
//! # Neutral Perspective Detection
//!
//! A perspective is considered neutral when magnitude < 0.05 and all
//! bias components are near zero. This is useful for bias-removal pipelines.
//!
//! [`CognitionError`]: super::CognitionError

pub mod viewpoint;
pub mod framing;
pub mod bias;

pub use super::CognitionError;
pub use viewpoint::CognitiveViewpoint;
pub use framing::{CognitiveFraming, FrameType};
pub use bias::PerspectiveBias;

/// Default viewpoint focus coordinates for neutral initialization.
pub const DEFAULT_VIEWPOINT_FOCUS: &[f64] = &[0.5, 0.5];
/// Default frame strength for balanced interpretation.
pub const DEFAULT_FRAME_STRENGTH: f64 = 0.7;
/// Default bias magnitude for low-impact perspective.
pub const DEFAULT_BIAS_MAGNITUDE: f64 = 0.1;
/// Maximum viewpoint shift delta per operation.
pub const MAX_VIEWPOINT_SHIFT: f64 = 1.0;

/// Creates a balanced perspective bundle with neutral bias.
///
/// Convenience constructor for initializing a default perspective state.
///
/// # Errors
///
/// Returns [`CognitionError::OutOfRange`] if focus coordinates are invalid.
pub fn create_neutral_perspective(
    dimensions: usize,
) -> Result<(CognitiveViewpoint, CognitiveFraming, PerspectiveBias), CognitionError> {
    let focus = vec![0.5; dimensions];
    let vp = CognitiveViewpoint::new(0, focus)?;
    let frame = CognitiveFraming::new(FrameType::Holistic, DEFAULT_FRAME_STRENGTH)?;
    let bias_vector = vec![0.0; dimensions];
    let bias = PerspectiveBias::new(bias_vector, DEFAULT_BIAS_MAGNITUDE)?;
    Ok((vp, frame, bias))
}

/// Validates a perspective configuration.
pub fn validate_perspective(
    viewpoint: &CognitiveViewpoint,
    frame: &CognitiveFraming,
    bias: &PerspectiveBias,
) -> Result<(), CognitionError> {
    viewpoint.validate()?;
    frame.validate()?;
    bias.validate()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neutral_perspective_succeeds() {
        let (vp, frame, bias) = create_neutral_perspective(3).unwrap();
        assert!(bias.is_neutral());
        assert_eq!(vp.focus_area().len(), 3);
    }

    #[test]
    fn validate_rejects_invalid() {
        let vp = CognitiveViewpoint::new(0, vec![]).unwrap_err();
        assert!(matches!(vp, CognitionError::MissingInput(_)));
    }
}