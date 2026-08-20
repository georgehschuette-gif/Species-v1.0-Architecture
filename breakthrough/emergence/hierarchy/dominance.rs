// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::HashMap;
use std::fmt;

use super::level::{LevelId, Level};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DominanceRelation {
    Dominates,
    SubordinateTo,
    Independent,
}

impl fmt::Display for DominanceRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DominanceRelation::Dominates => write!(f, "Dominates"),
            DominanceRelation::SubordinateTo => write!(f, "SubordinateTo"),
            DominanceRelation::Independent => write!(f, "Independent"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DominanceHierarchy {
    levels: HashMap<LevelId, Level>,
    relations: Vec<(LevelId, LevelId, DominanceRelation)>,
}

impl DominanceHierarchy {
    pub fn new() -> Self {
        DominanceHierarchy {
            levels: HashMap::new(),
            relations: Vec::new(),
        }
    }

    pub fn add_level(&mut self, level: Level) -> crate::Result<()> {
        let id = level.id();
        if self.levels.contains_key(&id) {
            return Err(crate::EmergenceError::HierarchyViolation(
                format!("level {} already exists", id),
            ));
        }
        level.validate()?;
        self.levels.insert(id, level);
        Ok(())
    }

    pub fn add_relation(
        &mut self,
        from: LevelId,
        to: LevelId,
        relation: DominanceRelation,
    ) -> crate::Result<()> {
        if !self.levels.contains_key(&from) || !self.levels.contains_key(&to) {
            return Err(crate::EmergenceError::HierarchyViolation(
                "both levels must exist before adding a relation".to_string(),
            ));
        }
        self.relations.push((from, to, relation));
        Ok(())
    }

    pub fn levels(&self) -> &HashMap<LevelId, Level> {
        &self.levels
    }

    pub fn relations(&self) -> &[(LevelId, LevelId, DominanceRelation)] {
        &self.relations
    }

    pub fn validate(&self) -> crate::Result<()> {
        for (from, to, _) in &self.relations {
            if !self.levels.contains_key(from) || !self.levels.contains_key(to) {
                return Err(crate::EmergenceError::HierarchyViolation(
                    "relation references non-existent level".to_string(),
                ));
            }
        }
        for level in self.levels.values() {
            level.validate()?;
        }
        Ok(())
    }
}

impl Default for DominanceHierarchy {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DominanceHierarchy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DominanceHierarchy(levels={}, relations={})",
            self.levels.len(),
            self.relations.len()
        )
    }
}
