// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::DreamsError;
use crate::DreamsResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionCriterion {
    EmotionalSalience,
    FrequencyOfUse,
    Recency,
    PredictiveValue,
    SemanticDensity,
}

impl std::fmt::Display for SelectionCriterion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectionCriterion::EmotionalSalience => write!(f, "EmotionalSalience"),
            SelectionCriterion::FrequencyOfUse => write!(f, "FrequencyOfUse"),
            SelectionCriterion::Recency => write!(f, "Recency"),
            SelectionCriterion::PredictiveValue => write!(f, "PredictiveValue"),
            SelectionCriterion::SemanticDensity => write!(f, "SemanticDensity"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExperienceSelection {
    pub selection_id: u64,
    pub criteria: Vec<SelectionCriterion>,
    pub selected_experiences: Vec<u64>,
    pub rejection_rate: f64,
}

impl ExperienceSelection {
    pub fn new(selection_id: u64) -> Self {
        Self {
            selection_id,
            criteria: Vec::new(),
            selected_experiences: Vec::new(),
            rejection_rate: 0.0,
        }
    }

    pub fn add_criterion(&mut self, criterion: SelectionCriterion) {
        self.criteria.push(criterion);
    }

    pub fn validate(&self) -> DreamsResult<()> {
        if self.criteria.is_empty() {
            return Err(DreamsError::InvalidPruning(
                "Selection must have at least one criterion".to_string(),
            ));
        }
        if self.rejection_rate < 0.0 || self.rejection_rate > 1.0 {
            return Err(DreamsError::InvalidPruning(
                "Rejection rate must be between 0.0 and 1.0".to_string(),
            ));
        }
        Ok(())
    }
}

impl std::fmt::Display for ExperienceSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ExperienceSelection(id={}, criteria={}, selected={}, rejection={:.2})",
            self.selection_id,
            self.criteria.len(),
            self.selected_experiences.len(),
            self.rejection_rate
        )
    }
}