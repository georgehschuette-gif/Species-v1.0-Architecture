// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::ImaginationError;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BranchType {
    Optimistic,
    Pessimistic,
    Baseline,
    Wildcard,
    Convergent,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioNode {
    pub year: u64,
    pub description: String,
    pub probability: f64,
    pub children: Vec<usize>,
}

pub struct FutureHistory {
    pub start_year: u64,
    pub branches: Vec<ScenarioNode>,
    pub active_branch: usize,
    pub branch_type: BranchType,
    pub scenario_count: usize,
    pub temporal_coherence: f64,
}

impl FutureHistory {
    pub fn new(start_year: u64) -> Self {
        Self {
            start_year,
            branches: vec![ScenarioNode { year: start_year, description: "Origin".into(), probability: 1.0, children: Vec::new() }],
            active_branch: 0,
            branch_type: BranchType::Baseline,
            scenario_count: 1,
            temporal_coherence: 1.0,
        }
    }

    pub fn start_year(&self) -> u64 { self.start_year }
    pub fn branches(&self) -> &[ScenarioNode] { &self.branches }
    pub fn active_branch(&self) -> usize { self.active_branch }
    pub fn branch_type(&self) -> BranchType { self.branch_type }
    pub fn scenario_count(&self) -> usize { self.scenario_count }
    pub fn temporal_coherence(&self) -> f64 { self.temporal_coherence }

    pub fn spawn_branch(&mut self, parent: usize, year: u64, description: impl Into<String>, probability: f64) -> Result<usize, ImaginationError> {
        if !(0.0..=1.0).contains(&probability) {
            return Err(ImaginationError::OutOfRange { field: "probability".into(), value: probability, min: 0.0, max: 1.0 });
        }
        if parent >= self.branches.len() {
            return Err(ImaginationError::DimensionMismatch { expected: self.branches.len(), actual: parent });
        }
        if self.branches.len() >= MAX_FUTURE_BRANCHES {
            return Err(ImaginationError::CapacityExceeded { max: MAX_FUTURE_BRANCHES, attempted: self.branches.len() + 1 });
        }
        let id = self.branches.len();
        self.branches.push(ScenarioNode { year, description: description.into(), probability, children: Vec::new() });
        self.branches[parent].children.push(id);
        self.scenario_count = self.branches.len();
        self.recalculate_coherence();
        Ok(id)
    }

    pub fn set_branch_type(&mut self, branch_type: BranchType) {
        self.branch_type = branch_type;
    }

    pub fn switch_branch(&mut self, id: usize) -> Result<(), ImaginationError> {
        if id >= self.branches.len() {
            return Err(ImaginationError::DimensionMismatch { expected: self.branches.len(), actual: id });
        }
        self.active_branch = id;
        Ok(())
    }

    pub fn prune_unlikely(&mut self, threshold: f64) -> Result<(), ImaginationError> {
        if !(0.0..=1.0).contains(&threshold) {
            return Err(ImaginationError::OutOfRange { field: "threshold".into(), value: threshold, min: 0.0, max: 1.0 });
        }
        let mut to_remove = Vec::new();
        for (i, branch) in self.branches.iter().enumerate() {
            if i == 0 { continue; }
            if branch.probability < threshold {
                to_remove.push(i);
            }
        }
        for &idx in to_remove.iter().rev() {
            self.branches.remove(idx);
        }
        self.scenario_count = self.branches.len();
        if self.active_branch >= self.branches.len() {
            self.active_branch = 0;
        }
        self.recalculate_coherence();
        Ok(())
    }

    pub fn path_to(&self, target: usize) -> Option<Vec<usize>> {
        if target >= self.branches.len() { return None; }
        let mut path = Vec::new();
        let mut current = target;
        while current != 0 {
            path.push(current);
            let parent = self.find_parent(current)?;
            current = parent;
        }
        path.push(0);
        path.reverse();
        Some(path)
    }

    pub fn validate(&self) -> Result<(), ImaginationError> {
        if self.branches.is_empty() {
            return Err(ImaginationError::InvalidInput("future history has no branches".into()));
        }
        Ok(())
    }

    fn find_parent(&self, target: usize) -> Option<usize> {
        for (i, branch) in self.branches.iter().enumerate() {
            if branch.children.contains(&target) {
                return Some(i);
            }
        }
        None
    }

    fn recalculate_coherence(&mut self) {
        if self.branches.is_empty() {
            self.temporal_coherence = 0.0;
            return;
        }
        let avg_prob: f64 = self.branches.iter().map(|b| b.probability).sum::<f64>() / self.branches.len() as f64;
        self.temporal_coherence = avg_prob.clamp(0.0, 1.0);
    }
}

impl fmt::Display for FutureHistory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FutureHistory(start={}, branches={}, type={:?}, coherence={:.2})", self.start_year, self.scenario_count, self.branch_type, self.temporal_coherence)
    }
}

impl fmt::Display for BranchType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Optimistic => write!(f, "Optimistic"),
            Self::Pessimistic => write!(f, "Pessimistic"),
            Self::Baseline => write!(f, "Baseline"),
            Self::Wildcard => write!(f, "Wildcard"),
            Self::Convergent => write!(f, "Convergent"),
        }
    }
}
