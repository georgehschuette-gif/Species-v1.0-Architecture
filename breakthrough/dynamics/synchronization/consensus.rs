// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Consensus {
    pub nodes: Vec<String>,
    pub agreed_value: Option<f64>,
    pub threshold: f64,
}

impl Consensus {
    pub fn new(nodes: Vec<String>, threshold: f64) -> Self {
        Self { nodes, agreed_value: None, threshold }
    }

    pub fn propose(&mut self, value: f64) -> Result<bool, crate::DynamicsError> {
        if self.nodes.is_empty() {
            return Err(crate::DynamicsError::SyncError("No nodes".to_string()));
        }
        let required_support = (self.nodes.len() as f64 * self.threshold).ceil() as usize;
        let support = (value - self.agreed_value.unwrap_or(value)).abs() < 1e-6;
        if support && required_support <= self.nodes.len() {
            self.agreed_value = Some(value);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn reached(&self) -> bool {
        self.agreed_value.is_some()
    }
}

impl fmt::Display for Consensus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.agreed_value {
            Some(v) => write!(f, "Consensus(value={:.2}, nodes={})", v, self.nodes.len()),
            None => write!(f, "Consensus(pending, nodes={})", self.nodes.len()),
        }
    }
}

