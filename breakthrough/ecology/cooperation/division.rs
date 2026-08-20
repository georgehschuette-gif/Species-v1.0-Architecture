// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// LaborDivision: Specialization of tasks among cooperative members.
pub struct LaborDivision {
    pub members: Vec<u64>,
    pub roles: Vec<CognitiveRole>,
    pub efficiency_gain: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CognitiveRole {
    Explorer,
    Processor,
    Integrator,
    Validator,
}

impl LaborDivision {
    /// Constructs a new LaborDivision with validated parameters.
    ///
    /// # Errors
    /// Returns `InsufficientMembers` if there are fewer than 2 members.
    /// Returns `InvalidCost` if efficiency_gain is outside [0.0, 1.0].
    /// Returns `ProtocolMismatch` if roles length does not match members length.
    /// Returns `InvalidParticipant` if any member ID is zero.
    pub fn new(
        members: Vec<u64>,
        roles: Vec<CognitiveRole>,
        efficiency_gain: f64,
    ) -> Result<Self, CooperationError> {
        if members.len() < 2 {
            return Err(CooperationError::InsufficientMembers(
                "Labor division requires at least 2 members".to_string(),
            ));
        }
        if members.len() != roles.len() {
            return Err(CooperationError::ProtocolMismatch(
                "Roles length must match members length".to_string(),
            ));
        }
        if efficiency_gain.is_nan()
            || efficiency_gain.is_infinite()
            || !(0.0..=1.0).contains(&efficiency_gain)
        {
            return Err(CooperationError::InvalidCost(
                "Efficiency gain must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        for (i, &id) in members.iter().enumerate() {
            if id == 0 {
                return Err(CooperationError::InvalidParticipant(
                    format!("Member at index {} has zero ID", i),
                ));
            }
        }
        Ok(Self {
            members,
            roles,
            efficiency_gain,
        })
    }

    /// Validates the internal consistency of this labor division.
    pub fn validate(&self) -> Result<(), CooperationError> {
        if self.members.len() < 2 {
            return Err(CooperationError::InsufficientMembers(
                "Labor division requires at least 2 members".to_string(),
            ));
        }
        if self.members.len() != self.roles.len() {
            return Err(CooperationError::ProtocolMismatch(
                "Roles length must match members length".to_string(),
            ));
        }
        if self.efficiency_gain.is_nan()
            || self.efficiency_gain.is_infinite()
            || !(0.0..=1.0).contains(&self.efficiency_gain)
        {
            return Err(CooperationError::InvalidCost(
                "Efficiency gain must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        for &id in &self.members {
            if id == 0 {
                return Err(CooperationError::InvalidParticipant(
                    "Member ID must be non-zero".to_string(),
                ));
            }
        }
        Ok(())
    }

    /// Returns the number of roles assigned.
    pub fn role_assignment_count(&self) -> usize {
        self.roles.len()
    }

    /// Returns true if every member has a unique role (balanced division).
    pub fn is_balanced(&self) -> bool {
        let mut seen = std::collections::HashSet::new();
        for role in &self.roles {
            if !seen.insert(*role) {
                return false;
            }
        }
        true
    }

    /// Returns the count of members assigned to a specific role.
    pub fn role_count(&self, role: CognitiveRole) -> usize {
        self.roles.iter().filter(|r| **r == role).count()
    }

    /// Returns the efficiency gain as a percentage.
    pub fn efficiency_percentage(&self) -> f64 {
        self.efficiency_gain * 100.0
    }

    /// Returns the number of members in the division.
    pub fn member_count(&self) -> usize {
        self.members.len()
    }
}

impl std::fmt::Display for LaborDivision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LaborDivision(members={}, roles={}, efficiency_gain={:.4})",
            self.members.len(),
            self.roles.len(),
            self.efficiency_gain
        )
    }
}