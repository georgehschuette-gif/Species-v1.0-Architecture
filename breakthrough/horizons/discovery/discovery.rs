// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscoveryError {
    InvalidDiscoveryId(String),
    InvalidConfidence(String),
    InvalidDistance(String),
    DuplicateDiscovery(String),
}

impl fmt::Display for DiscoveryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiscoveryError::InvalidDiscoveryId(msg) => write!(f, "Invalid discovery ID: {}", msg),
            DiscoveryError::InvalidConfidence(msg) => write!(f, "Invalid confidence: {}", msg),
            DiscoveryError::InvalidDistance(msg) => write!(f, "Invalid distance: {}", msg),
            DiscoveryError::DuplicateDiscovery(msg) => write!(f, "Duplicate discovery: {}", msg),
        }
    }
}

impl std::error::Error for DiscoveryError {}

pub struct Discovery {
    pub discovery_id: u64,
    pub confidence: f64,
    pub distance: f64,
    pub verified: bool,
}

impl Discovery {
    pub fn new(discovery_id: u64, confidence: f64, distance: f64) -> Result<Self, DiscoveryError> {
        if !(0.0..=1.0).contains(&confidence) {
            return Err(DiscoveryError::InvalidConfidence(format!(
                "Confidence {} out of range",
                confidence
            )));
        }
        if distance < 0.0 {
            return Err(DiscoveryError::InvalidDistance(format!(
                "Distance {} must be non-negative",
                distance
            )));
        }
        Ok(Self {
            discovery_id,
            confidence,
            distance,
            verified: false,
        })
    }

    pub fn verify(&mut self) -> Result<(), DiscoveryError> {
        if self.verified {
            return Err(DiscoveryError::DuplicateDiscovery(
                "Already verified".to_string(),
            ));
        }
        self.verified = true;
        Ok(())
    }

    pub fn confidence(&self) -> f64 {
        self.confidence
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }

    pub fn is_verified(&self) -> bool {
        self.verified
    }

    pub fn is_significant(&self) -> bool {
        self.confidence > 0.8 && !self.verified
    }
}

impl fmt::Display for Discovery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Discovery(id={}, confidence={:.2}, distance={:.2}, verified={})",
            self.discovery_id, self.confidence, self.distance, self.verified
        )
    }
}
