// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// PredictiveConfidence: Reliability of a prediction.
pub struct PredictiveConfidence {
    pub prediction_id: u64,
    pub confidence_score: f64,
    pub calibration_error: f64,
    pub coverage: f64,
}

impl PredictiveConfidence {
    pub fn new(prediction_id: u64, confidence_score: f64) -> Result<Self, PredictiveError> {
        if prediction_id == 0 {
            return Err(PredictiveError::InvalidPrediction { prediction_id });
        }
        if !(0.0..=1.0).contains(&confidence_score) {
            return Err(PredictiveError::InvalidConfidence { confidence: confidence_score });
        }
        Ok(Self { prediction_id, confidence_score, calibration_error: 0.0, coverage: 0.0 })
    }

    pub fn update_calibration(&mut self, outcome_occurred: bool) {
        let error = if (self.confidence_score > 0.5) == outcome_occurred { 0.0 } else { 1.0 };
        self.calibration_error = (self.calibration_error * 0.9 + error as f64 * 0.1).clamp(0.0, 1.0);
    }

    pub fn is_well_calibrated(&self) -> bool {
        self.calibration_error < 0.2
    }
}
