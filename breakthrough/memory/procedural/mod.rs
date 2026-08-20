// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Procedural Memory: Skill and routine memory encoded as action sequences.
//!
//! Procedural memory stores the "how" of cognition: the step-by-step procedures
//! and action sequences that govern skilled behavior. Unlike declarative memory,
//! procedural memory operates largely outside conscious awareness and is
//! characterized by automaticity, error rates, and mastery progression.
//!
//! # Components
//!
//! - **ProceduralSkill** — A named skill with discrete steps, mastery level,
//!   automaticity, and error rate tracking.
//! - **ActionSequence** — An ordered chain of actions with transition probabilities,
//!   expected durations, and error recovery strategies.
//!
//! # Mastery
//!
//! Skills progress through stages of mastery: from initial clumsy execution
//! through deliberate practice to fully automatic performance. Each rehearsal
//! of a skill increases automaticity while reducing error rate, following a
//! power-law learning curve.

use std::fmt;

use crate::MemoryError;

pub mod skill;
pub mod sequence;

pub use skill::ProceduralSkill;
pub use sequence::ActionSequence;

/// Default mastery increment per successful rehearsal.
pub const DEFAULT_MASTERY_GAIN: f64 = 0.1;
/// Default automaticity growth per rehearsal.
pub const DEFAULT_AUTOMATICITY_GAIN: f64 = 0.05;
/// Default base error rate for unskilled performance.
pub const DEFAULT_BASE_ERROR_RATE: f64 = 0.3;
/// Maximum mastery level for any skill.
pub const MAX_MASTERY: f64 = 1.0;
/// Minimum error rate floor.
pub const MIN_ERROR_RATE: f64 = 0.001;

/// Creates a new procedural skill with the given name and step count.
///
/// # Errors
///
/// Returns [`MemoryError::MissingInput`] if the name is empty.
pub fn create_skill(name: impl Into<String>, steps: usize) -> Result<ProceduralSkill, MemoryError> {
    ProceduralSkill::new(name, steps)
}

/// Creates a new action sequence with the given action names.
///
/// # Errors
///
/// Returns [`MemoryError::MissingInput`] if any action name is empty.
pub fn create_sequence(actions: &[impl AsRef<str>]) -> Result<ActionSequence, MemoryError> {
    ActionSequence::new(actions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skill_creation_succeeds() {
        let s = create_skill("typing", 10).unwrap();
        assert_eq!(s.steps, 10);
        assert_eq!(s.mastery_level, 0.0);
    }

    #[test]
    fn skill_rehearsal_increases_mastery() {
        let mut s = create_skill("driving", 5).unwrap();
        s.rehearse(0.5).unwrap();
        assert!(s.mastery_level > 0.0);
    }

    #[test]
    fn skill_degrade_reduces_mastery() {
        let mut s = create_skill("dancing", 3).unwrap();
        s.rehearse(0.5).unwrap();
        s.degrade(0.1);
        assert!(s.mastery_level < 0.5);
    }

    #[test]
    fn sequence_prediction_returns_next_action() {
        let seq = create_sequence(&["start", "middle", "end"]).unwrap();
        assert!(seq.predict_next("start").is_some());
    }

    #[test]
    fn sequence_transition_probability_in_range() {
        let seq = create_sequence(&["a", "b", "c"]).unwrap();
        let p = seq.transition_probability("a", "b");
        assert!(p >= 0.0 && p <= 1.0);
    }

    #[test]
    fn default_constants_valid() {
        assert!(DEFAULT_MASTERY_GAIN > 0.0);
        assert!(MAX_MASTERY > 0.0);
    }
}
