// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Cooperation: Collaborative interactions between entities.
//! Defines how entities work together for mutual benefit.

pub mod alliance;
pub mod division;
pub mod reciprocal;

pub use alliance::StrategicAlliance;
pub use division::LaborDivision;
pub use reciprocal::ReciprocalAltruism;

/// CooperationProtocol: The type of cooperative arrangement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CooperationProtocol {
    Alliance,
    Division,
    Reciprocal,
}

/// CooperativeGroup: A group of entities cooperating under a shared protocol.
pub struct CooperativeGroup {
    pub members: Vec<u64>,
    pub protocol: CooperationProtocol,
    pub trust_level: f64,
    pub coordination_cost: f64,
}

/// CooperationError: Error types for cooperation operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CooperationError {
    InvalidCommitment(String),
    InvalidTrust(String),
    InvalidCost(String),
    InvalidBenefit(String),
    InvalidParticipant(String),
    InsufficientMembers(String),
    ProtocolMismatch(String),
}

impl std::fmt::Display for CooperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CooperationError::InvalidCommitment(msg) => {
                write!(f, "Invalid commitment level: {}", msg)
            }
            CooperationError::InvalidTrust(msg) => {
                write!(f, "Invalid trust level: {}", msg)
            }
            CooperationError::InvalidCost(msg) => {
                write!(f, "Invalid cost value: {}", msg)
            }
            CooperationError::InvalidBenefit(msg) => {
                write!(f, "Invalid benefit value: {}", msg)
            }
            CooperationError::InvalidParticipant(msg) => {
                write!(f, "Invalid participant: {}", msg)
            }
            CooperationError::InsufficientMembers(msg) => {
                write!(f, "Insufficient members: {}", msg)
            }
            CooperationError::ProtocolMismatch(msg) => {
                write!(f, "Protocol mismatch: {}", msg)
            }
        }
    }
}

impl std::error::Error for CooperationError {}