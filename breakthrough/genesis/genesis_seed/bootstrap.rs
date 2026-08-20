// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::genesis::{GenesisError, GenesisResult};

/// BootstrapSequence: The ordered steps to initialize
/// the ecosystem.
///
/// The bootstrap sequence defines the stages through which
/// the cognitive ecosystem passes from an uninitialized
/// state to a fully operational one. Each step must be
/// completed successfully before the next begins.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootstrapStep {
    /// Initialize the cognitive field grid.
    InitializeField,
    /// Spawn the primordial entities that form
    /// the foundational nodes.
    SpawnPrimordialEntities,
    /// Apply the genesis rules (axioms and invariants)
    /// to establish the ecosystem's structural integrity.
    ApplyGenesisRules,
    /// Stabilize the ecosystem by damping residual
    /// oscillations and verifying invariants.
    Stabilize,
}

impl BootstrapStep {
    /// Returns a human-readable label for this step.
    pub fn label(&self) -> &'static str {
        match self {
            BootstrapStep::InitializeField => "InitializeField",
            BootstrapStep::SpawnPrimordialEntities => "SpawnPrimordialEntities",
            BootstrapStep::ApplyGenesisRules => "ApplyGenesisRules",
            BootstrapStep::Stabilize => "Stabilize",
        }
    }

    /// Returns the index of this step in the canonical
    /// bootstrap sequence.
    pub fn index(&self) -> usize {
        match self {
            BootstrapStep::InitializeField => 0,
            BootstrapStep::SpawnPrimordialEntities => 1,
            BootstrapStep::ApplyGenesisRules => 2,
            BootstrapStep::Stabilize => 3,
        }
    }

    /// Returns the next step in the sequence, or `None`
    /// if this is the last step.
    pub fn next(&self) -> Option<BootstrapStep> {
        match self {
            BootstrapStep::InitializeField => {
                Some(BootstrapStep::SpawnPrimordialEntities)
            }
            BootstrapStep::SpawnPrimordialEntities => {
                Some(BootstrapStep::ApplyGenesisRules)
            }
            BootstrapStep::ApplyGenesisRules => {
                Some(BootstrapStep::Stabilize)
            }
            BootstrapStep::Stabilize => None,
        }
    }
}

/// BootstrapSequence: The ordered steps to initialize
/// the ecosystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapSequence {
    /// The ordered steps in this bootstrap sequence.
    pub steps: Vec<BootstrapStep>,
}

impl BootstrapSequence {
    /// Creates a new bootstrap sequence with the
    /// standard canonical steps.
    pub fn new() -> Self {
        Self {
            steps: vec![
                BootstrapStep::InitializeField,
                BootstrapStep::SpawnPrimordialEntities,
                BootstrapStep::ApplyGenesisRules,
                BootstrapStep::Stabilize,
            ],
        }
    }

    /// Returns the next step to execute based on the
    /// completed steps.
    ///
    /// Returns `None` if all steps have been completed.
    pub fn next_step(
        &self,
        completed_steps: &[BootstrapStep],
    ) -> Option<BootstrapStep> {
        if completed_steps.len() >= self.steps.len() {
            return None;
        }
        self.steps.get(completed_steps.len()).copied()
    }

    /// Returns whether all steps in the sequence have
    /// been completed.
    pub fn is_complete(
        &self,
        completed_steps: &[BootstrapStep],
    ) -> bool {
        completed_steps.len() >= self.steps.len()
    }

    /// Returns the total number of steps in this sequence.
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Validates that the sequence contains all four
    /// canonical steps in the correct order.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::ValidationFailure`] if the
    /// sequence is missing steps or has them in the wrong
    /// order.
    pub fn validate(&self) -> GenesisResult<()> {
        if self.steps.len() != 4 {
            return Err(GenesisError::ValidationFailure(
                "bootstrap sequence must have exactly 4 steps".to_string(),
            ));
        }
        let expected = vec![
            BootstrapStep::InitializeField,
            BootstrapStep::SpawnPrimordialEntities,
            BootstrapStep::ApplyGenesisRules,
            BootstrapStep::Stabilize,
        ];
        for (i, step) in self.steps.iter().enumerate() {
            if *step != expected[i] {
                return Err(GenesisError::InvalidState(format!(
                    "step {} expected {:?}, got {:?}",
                    i, expected[i], step
                )));
            }
        }
        Ok(())
    }
}

impl Default for BootstrapSequence {
    fn default() -> Self {
        Self::new()
    }
}