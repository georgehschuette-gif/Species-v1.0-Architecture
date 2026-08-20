// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrganizerId(u64);

impl OrganizerId {
    pub fn new(id: u64) -> Self {
        OrganizerId(id)
    }

    pub fn inner(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for OrganizerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OrganizerId({})", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Organizer {
    id: OrganizerId,
    name: String,
    sensitivity: f64,
}

impl Organizer {
    pub fn new(id: OrganizerId, name: String, sensitivity: f64) -> crate::Result<Self> {
        if name.trim().is_empty() {
            return Err(crate::EmergenceError::HierarchyViolation(
                "organizer name cannot be empty".to_string(),
            ));
        }
        if sensitivity <= 0.0 {
            return Err(crate::EmergenceError::HierarchyViolation(
                "sensitivity must be positive".to_string(),
            ));
        }
        Ok(Organizer {
            id,
            name,
            sensitivity,
        })
    }

    pub fn id(&self) -> OrganizerId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn sensitivity(&self) -> f64 {
        self.sensitivity
    }

    pub fn catalyze(&self, pattern_strength: f64) -> f64 {
        pattern_strength * self.sensitivity
    }

    pub fn validate(&self) -> crate::Result<()> {
        if self.name.trim().is_empty() {
            return Err(crate::EmergenceError::HierarchyViolation(
                "organizer name is empty".to_string(),
            ));
        }
        if self.sensitivity <= 0.0 {
            return Err(crate::EmergenceError::HierarchyViolation(
                format!("invalid sensitivity: {}", self.sensitivity),
            ));
        }
        Ok(())
    }
}

impl fmt::Display for Organizer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Organizer(id={}, name={}, sensitivity={:.3})",
            self.id, self.name, self.sensitivity
        )
    }
}
