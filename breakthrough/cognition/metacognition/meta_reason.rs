// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// StrategySelection: The approach used for meta-level reasoning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrategySelection {
    /// Greedy: Always pick the locally optimal next step.
    Greedy,
    /// Exhaustive: Evaluate all possible moves before deciding.
    Exhaustive,
    /// Heuristic: Use rules of thumb to guide reasoning.
    Heuristic,
    /// Probabilistic: Reason under uncertainty with weighted choices.
    Probabilistic,
}

/// MetaReason: Reasoning about reasoning strategies themselves.
///
/// Meta-reasoning enables the system to evaluate and select
/// reasoning strategies based on the problem at hand.
/// The reasoning depth controls how many meta-levels are
/// considered, and the strategy selection method determines
/// how decisions are made.
///
/// # Fields
/// - `reasoning_depth`: Number of meta-reasoning levels, in [0, usize::MAX].
/// - `strategy_selection`: The method used to choose reasoning strategies.
///
/// # Example
/// ```
/// use breakthrough::cognition::metacognition::{MetaReason, StrategySelection};
///
/// let meta = MetaReason::new(3, StrategySelection::Heuristic)
///     .expect("valid parameters");
/// assert_eq!(meta.depth(), 3);
/// ```
pub struct MetaReason {
    /// Number of meta-reasoning levels, in [0, 50].
    pub reasoning_depth: usize,
    /// The method used to choose reasoning strategies.
    pub strategy_selection: StrategySelection,
}

impl MetaReason {
    /// Creates a new `MetaReason` with the given depth and strategy.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `reasoning_depth` exceeds 50.
    pub fn new(
        reasoning_depth: usize,
        strategy_selection: StrategySelection,
    ) -> Result<Self, CognitionError> {
        if reasoning_depth > 50 {
            return Err(CognitionError::OutOfRange {
                field: "reasoning_depth".to_string(),
                value: reasoning_depth as f64,
                min: 0.0,
                max: 50.0,
            });
        }
        Ok(Self {
            reasoning_depth,
            strategy_selection,
        })
    }

    /// Advances meta-reasoning by one level, incrementing depth.
    ///
    /// # Errors
    /// Returns [`CognitionError::CapacityExceeded`] if depth exceeds 50.
    pub fn deepen(&mut self) -> Result<(), CognitionError> {
        if self.reasoning_depth >= 50 {
            return Err(CognitionError::CapacityExceeded {
                max: 50,
                attempted: self.reasoning_depth + 1,
            });
        }
        self.reasoning_depth += 1;
        Ok(())
    }

    /// Retreats one meta-reasoning level.
    ///
    /// Returns `true` if depth was reduced, `false` if already at zero.
    pub fn shallow(&mut self) -> bool {
        if self.reasoning_depth > 0 {
            self.reasoning_depth -= 1;
            true
        } else {
            false
        }
    }

    /// Returns the current reasoning depth.
    pub fn depth(&self) -> usize {
        self.reasoning_depth
    }

    /// Selects the appropriate reasoning strategy for a problem
    /// of the given complexity.
    ///
    /// Higher complexity favors exhaustive strategies, while
    /// moderate complexity uses heuristic or probabilistic methods.
    pub fn select_strategy(&self, complexity: f64) -> Result<StrategySelection, CognitionError> {
        if !(0.0..=1.0).contains(&complexity) {
            return Err(CognitionError::OutOfRange {
                field: "complexity".to_string(),
                value: complexity,
                min: 0.0,
                max: 1.0,
            });
        }
        let strategy = if complexity < 0.2 {
            StrategySelection::Greedy
        } else if complexity < 0.5 {
            StrategySelection::Heuristic
        } else if complexity < 0.8 {
            StrategySelection::Probabilistic
        } else {
            StrategySelection::Exhaustive
        };
        Ok(strategy)
    }

    /// Updates the strategy selection method.
    pub fn set_strategy(&mut self, new_strategy: StrategySelection) {
        self.strategy_selection = new_strategy;
    }

    /// Computes the reasoning overhead as a function of depth
    /// and strategy type. Deeper reasoning with exhaustive
    /// strategies has higher overhead.
    pub fn overhead(&self) -> f64 {
        let base_cost = self.reasoning_depth as f64 * 0.1;
        let strategy_multiplier = match self.strategy_selection {
            StrategySelection::Greedy => 1.0,
            StrategySelection::Heuristic => 1.5,
            StrategySelection::Probabilistic => 2.0,
            StrategySelection::Exhaustive => 3.0,
        };
        base_cost * strategy_multiplier
    }

    /// Validates all field values are within valid ranges.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if self.reasoning_depth > 50 {
            return Err(CognitionError::OutOfRange {
                field: "reasoning_depth".to_string(),
                value: self.reasoning_depth as f64,
                min: 0.0,
                max: 50.0,
            });
        }
        Ok(())
    }
}

