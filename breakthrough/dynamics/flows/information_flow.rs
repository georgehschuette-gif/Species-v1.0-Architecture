// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct InformationFlow {
    pub source_id: u64,
    pub target_id: u64,
    pub data_rate: f64,
    pub entropy: f64,
}

impl InformationFlow {
    pub fn new(source_id: u64, target_id: u64, data_rate: f64, entropy: f64) -> Self {
        Self { source_id, target_id, data_rate, entropy }
    }

    pub fn capacity(&self) -> f64 {
        self.data_rate * self.entropy
    }
}

impl fmt::Display for InformationFlow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InformationFlow(source={}, target={}, rate={:.2}, entropy={:.2})", self.source_id, self.target_id, self.data_rate, self.entropy)
    }
}

