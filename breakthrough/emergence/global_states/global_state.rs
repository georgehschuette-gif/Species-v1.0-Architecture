// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlobalStateId(u64);

impl GlobalStateId {
    pub fn new(id: u64) -> Self {
        GlobalStateId(id)
    }

    pub fn inner(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for GlobalStateId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GlobalStateId({})", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalState {
    id: GlobalStateId,
    snapshot: Vec<f64>,
    entropy: f64,
}

impl GlobalState {
    pub fn new(id: GlobalStateId, snapshot: Vec<f64>, entropy: f64) -> crate::Result<Self> {
        if snapshot.is_empty() {
            return Err(crate::EmergenceError::MeasurementError(
                "global state snapshot cannot be empty".to_string(),
            ));
        }
        if entropy < 0.0 {
            return Err(crate::EmergenceError::MeasurementError(
                "entropy must be non-negative".to_string(),
            ));
        }
        Ok(GlobalState {
            id,
            snapshot,
            entropy,
        })
    }

    pub fn id(&self) -> GlobalStateId {
        self.id
    }

    pub fn snapshot(&self) -> &[f64] {
        &self.snapshot
    }

    pub fn entropy(&self) -> f64 {
        self.entropy
    }

    pub fn observe(&mut self, new_snapshot: Vec<f64>) -> crate::Result<()> {
        if new_snapshot.len() != self.snapshot.len() {
            return Err(crate::EmergenceError::MeasurementError(
                "snapshot dimension mismatch".to_string(),
            ));
        }
        self.snapshot = new_snapshot;
        Ok(())
    }

    pub fn validate(&self) -> crate::Result<()> {
        if self.snapshot.is_empty() {
            return Err(crate::EmergenceError::MeasurementError(
                "global state has empty snapshot".to_string(),
            ));
        }
        if self.entropy < 0.0 {
            return Err(crate::EmergenceError::MeasurementError(
                format!("invalid entropy: {}", self.entropy),
            ));
        }
        Ok(())
    }
}

impl fmt::Display for GlobalState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GlobalState(id={}, snapshot_len={}, entropy={:.3})",
            self.id,
            self.snapshot.len(),
            self.entropy
        )
    }
}
