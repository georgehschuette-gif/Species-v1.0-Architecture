// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// AbstractionHierarchy: Organizes concepts into nested levels of abstraction.
pub struct AbstractionHierarchy {
    pub levels: usize,
    pub branching_factor: usize,
    pub nodes: Vec<HierarchyNode>,
}

/// A node in the abstraction hierarchy, representing a concept at a specific level.
#[derive(Debug, Clone)]
pub struct HierarchyNode {
    pub id: usize,
    pub level: usize,
    pub features: Vec<f64>,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub label: String,
}

impl AbstractionHierarchy {
    /// Create a new abstraction hierarchy.
    pub fn new(levels: usize, branching_factor: usize) -> PerceptionResult<Self> {
        if levels == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "levels must be positive".into(),
            ));
        }
        if branching_factor == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "branching_factor must be positive".into(),
            ));
        }
        Ok(Self {
            levels,
            branching_factor,
            nodes: Vec::new(),
        })
    }

    /// Add a node at the specified level with a feature vector.
    pub fn add_node(
        &mut self,
        level: usize,
        features: Vec<f64>,
        label: String,
    ) -> PerceptionResult<usize> {
        if level >= self.levels {
            return Err(PerceptionError::InvalidConfiguration(format!(
                "level {} exceeds maximum level {}",
                level,
                self.levels - 1
            )));
        }
        if features.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        let id = self.nodes.len();
        self.nodes.push(HierarchyNode {
            id,
            level,
            features,
            parent: None,
            children: Vec::new(),
            label,
        });
        Ok(id)
    }

    /// Establish a parent-child relationship between two nodes.
    pub fn add_relationship(
        &mut self,
        parent_id: usize,
        child_id: usize,
    ) -> PerceptionResult<()> {
        let parent = self
            .nodes
            .get(parent_id)
            .ok_or_else(|| {
                PerceptionError::InvalidConfiguration(
                    format!("parent node {} not found", parent_id),
                )
            })?;
        let child = self
            .nodes
            .get(child_id)
            .ok_or_else(|| {
                PerceptionError::InvalidConfiguration(
                    format!("child node {} not found", child_id),
                )
            })?;
        if child.level <= parent.level {
            return Err(PerceptionError::InvalidConfiguration(
                "child level must be greater than parent level".into(),
            ));
        }
        if parent.children.len() >= self.branching_factor {
            return Err(PerceptionError::InvalidConfiguration(format!(
                "branching_factor {} exceeded for parent {}",
                self.branching_factor,
                parent_id,
            )));
        }
        self.nodes[parent_id].children.push(child_id);
        self.nodes[child_id].parent = Some(parent_id);
        Ok(())
    }

    /// Get the depth of the hierarchy (longest root-to-leaf path).
    pub fn depth(&self) -> usize {
        if self.nodes.is_empty() {
            return 0;
        }
        let root_nodes: Vec<usize> = self
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.parent.is_none())
            .map(|(i, _)| i)
            .collect();
        let mut max_depth = 0;
        for root in root_nodes {
            let d = self.compute_depth_recursive(root);
            if d > max_depth {
                max_depth = d;
            }
        }
        max_depth + 1
    }

    fn compute_depth_recursive(&self, node_id: usize) -> usize {
        let node = &self.nodes[node_id];
        if node.children.is_empty() {
            return 0;
        }
        let mut max_child_depth = 0;
        for &child_id in &node.children {
            let d = self.compute_depth_recursive(child_id);
            if d > max_child_depth {
                max_child_depth = d;
            }
        }
        max_child_depth + 1
    }

    /// Get the path from a node up to the root.
    pub fn path_to_root(&self, node_id: usize) -> PerceptionResult<Vec<usize>> {
        let mut path = Vec::new();
        let mut current = node_id;
        let mut visited = std::collections::HashSet::new();
        loop {
            if !visited.insert(current) {
                return Err(PerceptionError::InvalidConfiguration(
                    "cycle detected in hierarchy".into(),
                ));
            }
            path.push(current);
            let parent = self.nodes[current].parent;
            match parent {
                Some(p) => current = p,
                None => break,
            }
        }
        Ok(path)
    }

    /// Return the number of nodes in the hierarchy.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get a reference to a node by index.
    pub fn get_node(&self, idx: usize) -> Option<&HierarchyNode> {
        self.nodes.get(idx)
    }

    /// Validate structural integrity of the hierarchy.
    pub fn validate(&self) -> PerceptionResult<()> {
        for (i, node) in self.nodes.iter().enumerate() {
            if node.level >= self.levels {
                return Err(PerceptionError::InvalidConfiguration(format!(
                    "node {} has invalid level {}",
                    i, node.level
                )));
            }
            if node.features.is_empty() {
                return Err(PerceptionError::InvalidConfiguration(format!(
                    "node {} has empty features",
                    i
                )));
            }
            for &child_id in &node.children {
                if child_id >= self.nodes.len() {
                    return Err(PerceptionError::InvalidConfiguration(format!(
                        "node {} references invalid child {}",
                        i, child_id
                    )));
                }
            }
        }
        Ok(())
    }
}