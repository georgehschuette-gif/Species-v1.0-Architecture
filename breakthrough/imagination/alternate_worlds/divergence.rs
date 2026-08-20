// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::ImaginationError;

use super::*;

pub struct WorldDivergence {
    pub point: f64,
    pub branches: Vec<WorldBranch>,
    pub stability: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WorldBranch {
    pub id: usize,
    pub probability: f64,
    pub depth: usize,
}

impl WorldDivergence {
    pub fn new(point: f64) -> Result<Self, ImaginationError> {
        if !(0.0..=1.0).contains(&point) {
            return Err(ImaginationError::OutOfRange { field: "point".into(), value: point, min: 0.0, max: 1.0 });
        }
        Ok(Self { point, branches: Vec::new(), stability: 1.0 })
    }

    pub fn point(&self) -> f64 { self.point }
    pub fn branches(&self) -> &[WorldBranch] { &self.branches }
    pub fn stability(&self) -> f64 { self.stability }

    pub fn add_branch(&mut self, probability: f64) -> Result<usize, ImaginationError> {
        if !(0.0..=1.0).contains(&probability) {
            return Err(ImaginationError::OutOfRange { field: "probability".into(), value: probability, min: 0.0, max: 1.0 });
        }
        let id = self.branches.len();
        let depth = self.branches.iter().map(|b| b.depth).max().unwrap_or(0) + 1;
        if depth > MAX_WORLD_DEPTH {
            return Err(ImaginationError::CapacityExceeded { max: MAX_WORLD_DEPTH, attempted: depth });
        }
        self.branches.push(WorldBranch { id, probability, depth });
        self.stability *= probability;
        Ok(id)
    }

    pub fn prune_branches_below(&mut self, threshold: f64) {
        self.branches.retain(|b| b.probability >= threshold);
    }

    pub fn most_likely_branch(&self) -> Option<&WorldBranch> {
        self.branches.iter().max_by(|a, b| a.probability.partial_cmp(&b.probability).unwrap())
    }

    pub fn entropy(&self) -> f64 {
        if self.branches.is_empty() { return 0.0; }
        let total: f64 = self.branches.iter().map(|b| b.probability).sum();
        self.branches.iter().filter(|b| b.probability > 0.0).map(|b| {
            let p = b.probability / total;
            -p * p.ln()
        }).sum()
    }

    pub fn validate(&self) -> Result<(), ImaginationError> {
        if !(0.0..=1.0).contains(&self.point) {
            return Err(ImaginationError::OutOfRange { field: "point".into(), value: self.point, min: 0.0, max: 1.0 });
        }
        Ok(())
    }
}

impl fmt::Display for WorldDivergence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WorldDivergence(point={:.2}, branches={}, stability={:.4})", self.point, self.branches.len(), self.stability)
    }
}

