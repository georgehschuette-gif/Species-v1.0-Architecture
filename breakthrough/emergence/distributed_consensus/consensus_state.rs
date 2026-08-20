// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsensusState {
    Proposing,
    Voting,
    Committed,
    Failed,
}

impl fmt::Display for ConsensusState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConsensusState::Proposing => write!(f, "Proposing"),
            ConsensusState::Voting => write!(f, "Voting"),
            ConsensusState::Committed => write!(f, "Committed"),
            ConsensusState::Failed => write!(f, "Failed"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConsensusRound {
    round_id: u64,
    state: ConsensusState,
    votes: BTreeMap<String, bool>,
    threshold: usize,
}

impl ConsensusRound {
    pub fn new(round_id: u64, threshold: usize) -> Self {
        ConsensusRound {
            round_id,
            state: ConsensusState::Proposing,
            votes: BTreeMap::new(),
            threshold,
        }
    }

    pub fn round_id(&self) -> u64 {
        self.round_id
    }

    pub fn state(&self) -> ConsensusState {
        self.state
    }

    pub fn threshold(&self) -> usize {
        self.threshold
    }

    pub fn votes(&self) -> &BTreeMap<String, bool> {
        &self.votes
    }

    pub fn transition(&mut self, new_state: ConsensusState) -> crate::Result<()> {
        let allowed = match self.state {
            ConsensusState::Proposing => {
                vec![ConsensusState::Voting, ConsensusState::Failed]
            }
            ConsensusState::Voting => {
                vec![ConsensusState::Committed, ConsensusState::Failed]
            }
            ConsensusState::Committed => {
                vec![ConsensusState::Proposing]
            }
            ConsensusState::Failed => {
                vec![ConsensusState::Proposing]
            }
        };
        if !allowed.contains(&new_state) {
            return Err(crate::EmergenceError::ConsensusFailure(format!(
                "cannot transition from {} to {}",
                self.state, new_state
            )));
        }
        self.state = new_state;
        Ok(())
    }

    pub fn add_vote(&mut self, voter: String, approve: bool) -> crate::Result<()> {
        if self.state != ConsensusState::Voting {
            return Err(crate::EmergenceError::ConsensusFailure(
                "cannot vote outside of Voting state".to_string(),
            ));
        }
        self.votes.insert(voter, approve);
        let approval_count = self.votes.values().filter(|&&v| v).count();
        if approval_count >= self.threshold {
            self.state = ConsensusState::Committed;
        }
        Ok(())
    }

    pub fn validate(&self) -> crate::Result<()> {
        if self.votes.len() > self.threshold && self.state == ConsensusState::Proposing {
            return Err(crate::EmergenceError::ConsensusFailure(
                "round has votes but is still in Proposing state".to_string(),
            ));
        }
        Ok(())
    }
}

impl fmt::Display for ConsensusRound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ConsensusRound(id={}, state={}, votes={}, threshold={})",
            self.round_id,
            self.state,
            self.votes.len(),
            self.threshold
        )
    }
}
