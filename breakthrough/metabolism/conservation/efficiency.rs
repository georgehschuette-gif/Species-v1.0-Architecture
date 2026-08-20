// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::MetabolismError;
use crate::conservation::ConservationPolicy;
use crate::decay::DecayCurve;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EfficiencyMetric {
    pub policy: ConservationPolicy,
    pub ratio: u8,
}

impl EfficiencyMetric {
    pub fn new(policy: ConservationPolicy, ratio: u8) -> Self {
        Self { policy, ratio }
    }

    pub fn is_compliant(&self) -> Result<bool, MetabolismError> {
        if self.ratio < self.policy.min_efficiency {
            return Err(MetabolismError::ConservationViolation);
        }
        Ok(true)
    }

    pub fn adjusted_efficiency(&self, decay_curve: &DecayCurve) -> f64 {
        let decay_factor = decay_curve.current_factor();
        (self.ratio as f64) * decay_factor / 100.0
    }
}

