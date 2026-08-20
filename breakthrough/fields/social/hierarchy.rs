// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// HierarchyLevel: A single level in a social hierarchy with rank and authority.
#[derive(Debug, Clone, PartialEq)]
pub struct HierarchyLevel {
    pub rank: usize,
    pub label: String,
    pub authority: f64,
    pub scope: String,
}

impl HierarchyLevel {
    pub fn new(rank: usize, label: String, authority: f64, scope: String) -> Result<Self, SocialError> {
        if !(0.0..=1.0).contains(&authority) {
            return Err(SocialError::InvalidScore { score: authority });
        }
        Ok(Self { rank, label, authority, scope })
    }

    pub fn is_high_authority(&self) -> bool {
        self.authority > 0.7
    }

    pub fn is_low_authority(&self) -> bool {
        self.authority < 0.3
    }

    pub const MAX_RANK: usize = 100;
}

/// SocialHierarchy: Rank and authority distributions.
pub struct SocialHierarchy {
    pub levels: Vec<HierarchyLevel>,
    pub dominance_score: f64,
    pub mobility: f64,
}

impl SocialHierarchy {
    pub fn new() -> Self {
        Self { levels: Vec::new(), dominance_score: 0.0, mobility: 0.0 }
    }

    pub fn add_level(&mut self, level: HierarchyLevel) {
        self.levels.push(level);
        self.levels.sort_by_key(|l| l.rank);
        self.recalculate();
    }

    pub fn authority_at_rank(&self, rank: usize) -> Option<f64> {
        self.levels.iter().find(|l| l.rank == rank).map(|l| l.authority)
    }

    pub fn total_authority(&self) -> f64 {
        self.levels.iter().map(|l| l.authority).sum()
    }

    pub fn recalculate(&mut self) {
        self.dominance_score = self.total_authority() / self.levels.len().max(1) as f64;
    }

    pub fn is_strict(&self) -> bool {
        self.levels.len() >= 2 && self.mobility < 0.1
    }

    pub fn highest_level(&self) -> Option<&HierarchyLevel> {
        self.levels.iter().max_by_key(|l| l.rank)
    }

    pub fn lowest_level(&self) -> Option<&HierarchyLevel> {
        self.levels.iter().min_by_key(|l| l.rank)
    }
}
