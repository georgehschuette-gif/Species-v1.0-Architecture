// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

use crate::TensionsError;

pub struct IncompletenessGap {
    pub known_fraction: f64,
    pub total_domain: f64,
    pub gap_size: f64,
}

impl IncompletenessGap {
    pub fn new(known: f64, total: f64) -> Result<Self, TensionsError> {
        if total < 0.0 { return Err(TensionsError::OutOfRange { field: "total".into(), value: total, min: 0.0, max: f64::INFINITY }); }
        let gap = if total > 0.0 { (1.0 - known / total).clamp(0.0, 1.0) } else { 0.0 };
        Ok(Self { known_fraction: known, total_domain: total, gap_size: gap })
    }

    pub fn size(&self) -> f64 { self.gap_size }
    pub fn known(&self) -> f64 { self.known_fraction }
    pub fn total(&self) -> f64 { self.total_domain }
    pub fn update_known(&mut self, delta: f64) -> Result<(), TensionsError> {
        self.known_fraction = (self.known_fraction + delta).clamp(0.0, self.total_domain);
        self.gap_size = if self.total_domain > 0.0 { (1.0 - self.known_fraction / self.total_domain).clamp(0.0, 1.0) } else { 0.0 };
        Ok(())
    }
    pub fn is_critical(&self, threshold: f64) -> bool { self.gap_size > threshold }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if self.total_domain < 0.0 { return Err(TensionsError::OutOfRange { field: "total_domain".into(), value: self.total_domain, min: 0.0, max: f64::INFINITY }); }
        Ok(())
    }
}

impl fmt::Debug for IncompletenessGap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IncompletenessGap").field("gap", &self.gap_size).field("known", &self.known_fraction).field("total", &self.total_domain).finish()
    }
}
