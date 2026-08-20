// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// ResourceContest: A discrete competition event over a specific resource.
pub struct ResourceContest {
    pub resource_id: u64,
    pub contestants: Vec<u64>,
    pub resource_value: f64,
    pub winner: Option<u64>,
}

impl ResourceContest {
    /// Constructs a new ResourceContest with validated parameters.
    ///
    /// # Errors
    /// Returns `NoContestants` if the contestants list is empty.
    /// Returns `InvalidConsumption` if resource_value is negative, NaN, or infinite.
    /// Returns `InvalidParticipant` if any contestant ID is zero.
    pub fn new(
        resource_id: u64,
        contestants: Vec<u64>,
        resource_value: f64,
    ) -> Result<Self, CompetitionError> {
        if resource_id == 0 {
            return Err(CompetitionError::InvalidParticipant(
                "Resource ID must be non-zero".to_string(),
            ));
        }
        if contestants.is_empty() {
            return Err(CompetitionError::NoContestants);
        }
        if resource_value.is_nan() || resource_value.is_infinite() || resource_value < 0.0 {
            return Err(CompetitionError::InvalidConsumption(
                "Resource value must be a non-negative finite number".to_string(),
            ));
        }
        for (i, &id) in contestants.iter().enumerate() {
            if id == 0 {
                return Err(CompetitionError::InvalidParticipant(
                    format!("Contestant at index {} has zero ID", i),
                ));
            }
        }
        Ok(Self {
            resource_id,
            contestants,
            resource_value,
            winner: None,
        })
    }

    /// Validates the internal consistency of this resource contest.
    pub fn validate(&self) -> Result<(), CompetitionError> {
        if self.resource_id == 0 {
            return Err(CompetitionError::InvalidParticipant(
                "Resource ID must be non-zero".to_string(),
            ));
        }
        if self.contestants.is_empty() {
            return Err(CompetitionError::NoContestants);
        }
        if self.resource_value.is_nan()
            || self.resource_value.is_infinite()
            || self.resource_value < 0.0
        {
            return Err(CompetitionError::InvalidConsumption(
                "Resource value must be a non-negative finite number".to_string(),
            ));
        }
        for (i, &id) in self.contestants.iter().enumerate() {
            if id == 0 {
                return Err(CompetitionError::InvalidParticipant(
                    format!("Contestant at index {} has zero ID", i),
                ));
            }
        }
        Ok(())
    }

    /// Determines the winner based on contestant ID ordering.
    ///
    /// The contestant with the lowest ID wins. Sets `self.winner` accordingly.
    /// Returns an error if there are no contestants.
    pub fn determine_winner(&mut self) -> Result<(), CompetitionError> {
        if self.contestants.is_empty() {
            return Err(CompetitionError::NoContestants);
        }
        let winner_id = *self
            .contestants
            .iter()
            .min()
            .ok_or(CompetitionError::NoContestants)?;
        self.winner = Some(winner_id);
        Ok(())
    }

    /// Returns the contest intensity as a function of resource value and contestant count.
    pub fn contest_intensity(&self) -> f64 {
        if self.contestants.is_empty() {
            return 0.0;
        }
        self.resource_value / self.contestants.len() as f64
    }

    /// Returns true if the contest has been resolved with a winner.
    pub fn is_resolved(&self) -> bool {
        self.winner.is_some()
    }

    /// Returns the number of contestants.
    pub fn contestant_count(&self) -> usize {
        self.contestants.len()
    }

    /// Returns the resource value at stake.
    pub fn value(&self) -> f64 {
        self.resource_value
    }
}

impl std::fmt::Display for ResourceContest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ResourceContest(resource_id={}, contestants={}, resource_value={:.4}, winner={:?})",
            self.resource_id,
            self.contestants.len(),
            self.resource_value,
            self.winner
        )
    }
}