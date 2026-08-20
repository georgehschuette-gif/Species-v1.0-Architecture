// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LevelId(u32);

impl LevelId {
    pub fn new(id: u32) -> Self {
        LevelId(id)
    }

    pub fn inner(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for LevelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LevelId({})", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Level {
    id: LevelId,
    depth: usize,
    agents: Vec<String>,
}

impl Level {
    pub fn new(id: LevelId, depth: usize, agents: Vec<String>) -> crate::Result<Self> {
        if agents.is_empty() {
            return Err(crate::EmergenceError::HierarchyViolation(
                "level must contain at least one agent".to_string(),
            ));
        }
        Ok(Level {
            id,
            depth,
            agents,
        })
    }

    pub fn id(&self) -> LevelId {
        self.id
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn agents(&self) -> &[String] {
        &self.agents
    }

    pub fn subdivide(&mut self, subgroups: Vec<Vec<String>>) -> crate::Result<()> {
        if subgroups.is_empty() {
            return Err(crate::EmergenceError::HierarchyViolation(
                "subdivision requires at least one subgroup".to_string(),
            ));
        }
        let total: usize = subgroups.iter().map(|g| g.len()).sum();
        if total != self.agents.len() {
            return Err(crate::EmergenceError::HierarchyViolation(
                "subgroups must cover all agents without overlap".to_string(),
            ));
        }
        self.depth += 1;
        Ok(())
    }

    pub fn validate(&self) -> crate::Result<()> {
        if self.agents.is_empty() {
            return Err(crate::EmergenceError::HierarchyViolation(
                "level has no agents".to_string(),
            ));
        }
        Ok(())
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Level(id={}, depth={}, agents={})",
            self.id,
            self.depth,
            self.agents.len()
        )
    }
}
