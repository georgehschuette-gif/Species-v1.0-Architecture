// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// StrategicAlliance: A formal cooperative arrangement between entities.
pub struct StrategicAlliance {
    pub members: Vec<u64>,
    pub purpose: AlliancePurpose,
    pub commitment_level: f64,
    pub duration: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlliancePurpose {
    Defense,
    ResourceAcquisition,
    KnowledgeSharing,
    RiskMitigation,
}

impl StrategicAlliance {
    /// Constructs a new StrategicAlliance with validated parameters.
    ///
    /// # Errors
    /// Returns `InsufficientMembers` if there are fewer than 2 members.
    /// Returns `InvalidCommitment` if commitment_level is outside [0.0, 1.0].
    /// Returns `InvalidCost` if duration is negative, NaN, or infinite.
    /// Returns `InvalidParticipant` if any member ID is zero.
    pub fn new(
        members: Vec<u64>,
        purpose: AlliancePurpose,
        commitment_level: f64,
        duration: f64,
    ) -> Result<Self, CooperationError> {
        if members.len() < 2 {
            return Err(CooperationError::InsufficientMembers(
                "An alliance requires at least 2 members".to_string(),
            ));
        }
        if commitment_level.is_nan()
            || commitment_level.is_infinite()
            || !(0.0..=1.0).contains(&commitment_level)
        {
            return Err(CooperationError::InvalidCommitment(
                "Commitment level must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        if duration.is_nan() || duration.is_infinite() || duration < 0.0 {
            return Err(CooperationError::InvalidCost(
                "Duration must be a non-negative finite number".to_string(),
            ));
        }
        for (i, &id) in members.iter().enumerate() {
            if id == 0 {
                return Err(CooperationError::InvalidCommitment(
                    format!("Member at index {} has zero ID", i),
                ));
            }
        }
        Ok(Self {
            members,
            purpose,
            commitment_level,
            duration,
        })
    }

    /// Validates the internal consistency of this strategic alliance.
    pub fn validate(&self) -> Result<(), CooperationError> {
        if self.members.len() < 2 {
            return Err(CooperationError::InsufficientMembers(
                "An alliance requires at least 2 members".to_string(),
            ));
        }
        if self.commitment_level.is_nan()
            || self.commitment_level.is_infinite()
            || !(0.0..=1.0).contains(&self.commitment_level)
        {
            return Err(CooperationError::InvalidCommitment(
                "Commitment level must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        if self.duration.is_nan() || self.duration.is_infinite() || self.duration < 0.0 {
            return Err(CooperationError::InvalidCost(
                "Duration must be a non-negative finite number".to_string(),
            ));
        }
        for &id in &self.members {
            if id == 0 {
                return Err(CooperationError::InvalidCommitment(
                    "Member ID must be non-zero".to_string(),
                ));
            }
        }
        Ok(())
    }

    /// Returns true if the alliance is considered strong (commitment > 0.7).
    pub fn is_strong(&self) -> bool {
        self.commitment_level > 0.7
    }

    /// Returns the effectiveness score based on commitment and member count.
    pub fn effectiveness(&self) -> f64 {
        self.commitment_level * (self.members.len() as f64).sqrt()
    }

    /// Returns the remaining duration, clamped to non-negative.
    pub fn remaining_duration(&self) -> f64 {
        self.duration.max(0.0)
    }

    /// Returns the number of members in the alliance.
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// Returns the purpose of the alliance as a string.
    pub fn purpose_str(&self) -> &'static str {
        match self.purpose {
            AlliancePurpose::Defense => "defense",
            AlliancePurpose::ResourceAcquisition => "resource_acquisition",
            AlliancePurpose::KnowledgeSharing => "knowledge_sharing",
            AlliancePurpose::RiskMitigation => "risk_mitigation",
        }
    }
}

impl std::fmt::Display for StrategicAlliance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StrategicAlliance(members={}, purpose={}, commitment={:.4}, duration={:.4})",
            self.members.len(),
            self.purpose_str(),
            self.commitment_level,
            self.duration
        )
    }
}