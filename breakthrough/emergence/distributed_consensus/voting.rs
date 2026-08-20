// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Vote {
    voter_id: String,
    choice: bool,
    weight: f64,
}

impl Vote {
    pub fn new(voter_id: String, choice: bool, weight: f64) -> crate::Result<Self> {
        if voter_id.trim().is_empty() {
            return Err(crate::EmergenceError::ConsensusFailure(
                "voter_id cannot be empty".to_string(),
            ));
        }
        if weight <= 0.0 {
            return Err(crate::EmergenceError::ConsensusFailure(
                "vote weight must be positive".to_string(),
            ));
        }
        Ok(Vote {
            voter_id,
            choice,
            weight,
        })
    }

    pub fn voter_id(&self) -> &str {
        &self.voter_id
    }

    pub fn choice(&self) -> bool {
        self.choice
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }

    pub fn validate(&self) -> crate::Result<()> {
        if self.voter_id.trim().is_empty() {
            return Err(crate::EmergenceError::ConsensusFailure(
                "vote has empty voter_id".to_string(),
            ));
        }
        if self.weight <= 0.0 {
            return Err(crate::EmergenceError::ConsensusFailure(
                format!("invalid weight: {}", self.weight),
            ));
        }
        Ok(())
    }
}

impl fmt::Display for Vote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Vote(voter={}, choice={}, weight={:.3})",
            self.voter_id, self.choice, self.weight
        )
    }
}

pub trait VotingProtocol {
    fn cast_vote(&mut self, vote: Vote) -> crate::Result<()>;
    fn tally(&self) -> (usize, usize);
    fn is_passed(&self, threshold: usize) -> bool;
    fn validate(&self) -> crate::Result<()>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct WeightedVoting {
    votes: BTreeMap<String, (bool, f64)>,
}

impl WeightedVoting {
    pub fn new() -> Self {
        WeightedVoting {
            votes: BTreeMap::new(),
        }
    }

    pub fn votes(&self) -> &BTreeMap<String, (bool, f64)> {
        &self.votes
    }
}

impl Default for WeightedVoting {
    fn default() -> Self {
        Self::new()
    }
}

impl VotingProtocol for WeightedVoting {
    fn cast_vote(&mut self, vote: Vote) -> crate::Result<()> {
        vote.validate()?;
        self.votes
            .insert(vote.voter_id().to_string(), (vote.choice(), vote.weight()));
        Ok(())
    }

    fn tally(&self) -> (usize, usize) {
        let mut approve = 0;
        let mut reject = 0;
        for (_, (choice, weight)) in &self.votes {
            if *choice {
                approve += 1;
            } else {
                reject += 1;
            }
        }
        (approve, reject)
    }

    fn is_passed(&self, threshold: usize) -> bool {
        let (approve, _) = self.tally();
        approve >= threshold
    }

    fn validate(&self) -> crate::Result<()> {
        for (voter_id, (_, weight)) in &self.votes {
            if voter_id.trim().is_empty() {
                return Err(crate::EmergenceError::ConsensusFailure(
                    "vote contains empty voter_id".to_string(),
                ));
            }
            if *weight <= 0.0 {
                return Err(crate::EmergenceError::ConsensusFailure(
                    format!("vote for {} has invalid weight", voter_id),
                ));
            }
        }
        Ok(())
    }
}
