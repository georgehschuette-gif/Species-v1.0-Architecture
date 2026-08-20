// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Principle {
    Autopoiesis,
    Redundancy,
    Modularity,
    NegativeFeedback,
    PositiveFeedback,
    Multistability,
}

impl Principle {
    pub fn validate_application(&self, context: &str) -> crate::Result<()> {
        match self {
            Principle::Autopoiesis => {
                if context.is_empty() {
                    return Err(crate::EmergenceError::HierarchyViolation(
                        "autopoiesis requires non-empty context".to_string(),
                    ));
                }
            }
            Principle::Redundancy => {
                if context.split_whitespace().count() < 2 {
                    return Err(crate::EmergenceError::HierarchyViolation(
                        "redundancy requires at least two components".to_string(),
                    ));
                }
            }
            Principle::Modularity => {
                if !context.contains('/') && !context.contains('.') {
                    return Err(crate::EmergenceError::HierarchyViolation(
                        "modularity requires identifiable modules".to_string(),
                    ));
                }
            }
            Principle::NegativeFeedback => {
                if context.parse::<f64>().is_err() {
                    return Err(crate::EmergenceError::HierarchyViolation(
                        "negative feedback requires numeric context".to_string(),
                    ));
                }
            }
            Principle::PositiveFeedback => {
                if context.parse::<f64>().is_err() {
                    return Err(crate::EmergenceError::HierarchyViolation(
                        "positive feedback requires numeric context".to_string(),
                    ));
                }
            }
            Principle::Multistability => {
                if context.len() < 3 {
                    return Err(crate::EmergenceError::HierarchyViolation(
                        "multistability requires richer context".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }
}

impl fmt::Display for Principle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Principle::Autopoiesis => write!(f, "Autopoiesis"),
            Principle::Redundancy => write!(f, "Redundancy"),
            Principle::Modularity => write!(f, "Modularity"),
            Principle::NegativeFeedback => write!(f, "NegativeFeedback"),
            Principle::PositiveFeedback => write!(f, "PositiveFeedback"),
            Principle::Multistability => write!(f, "Multistability"),
        }
    }
}
