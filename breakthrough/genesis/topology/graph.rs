// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::genesis::{GenesisError, GenesisResult};
use crate::genesis::ontology::{Entity, Relation};

/// CognitiveGraph: The network structure of entity relationships.
///
/// The cognitive graph is a directed multigraph where nodes
/// represent entities and edges represent causal, resonant,
/// inhibitory, or facilitative relationships between them.
/// The graph supports traversal, cycle detection, and
/// topological analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct CognitiveGraph {
    /// The entities (nodes) in the graph.
    pub nodes: Vec<Entity>,
    /// The relations (edges) in the graph.
    pub edges: Vec<Relation>,
}

impl CognitiveGraph {
    /// Creates an empty cognitive graph.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Creates a cognitive graph with the specified capacity
    /// pre-allocated for nodes and edges.
    pub fn with_capacity(
        node_capacity: usize,
        edge_capacity: usize,
    ) -> Self {
        Self {
            nodes: Vec::with_capacity(node_capacity),
            edges: Vec::with_capacity(edge_capacity),
        }
    }

    /// Adds an entity to the graph.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::CapacityExceeded`] if the
    /// number of entities exceeds the scaling limit.
    pub fn add_entity(
        &mut self,
        entity: Entity,
    ) -> GenesisResult<()> {
        crate::genesis::constants::ScalingConstants::validate_entity_count(self.nodes.len() + 1)?;
        if self.nodes.iter().any(|e| e.id == entity.id) {
            return Err(GenesisError::InvalidState(format!(
                "entity with id {} already exists",
                entity.id
            )));
        }
        self.nodes.push(entity);
        Ok(())
    }

    /// Adds a relation to the graph, validating both
    /// endpoints exist.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::IncompatibleEntities`] if
    /// either the source or target entity does not exist.
    /// Returns [`GenesisError::CapacityExceeded`] if the
    /// number of edges exceeds capacity.
    pub fn add_relation(
        &mut self,
        relation: Relation,
    ) -> GenesisResult<()> {
        relation.validate()?;
        let source_exists = self.nodes.iter().any(|e| e.id == relation.source);
        let target_exists = self.nodes.iter().any(|e| e.id == relation.target);
        if !source_exists {
            return Err(GenesisError::IncompatibleEntities {
                reason: format!(
                    "source entity {} does not exist in the graph",
                    relation.source
                ),
            });
        }
        if !target_exists {
            return Err(GenesisError::IncompatibleEntities {
                reason: format!(
                    "target entity {} does not exist in the graph",
                    relation.target
                ),
            });
        }
        let max_edges = crate::genesis::constants::ScalingConstants::max_total_edges();
        if self.edges.len() >= max_edges {
            return Err(GenesisError::CapacityExceeded {
                max: max_edges,
                attempted: self.edges.len() + 1,
            });
        }
        self.edges.push(relation);
        Ok(())
    }

    /// Returns the entity with the given ID, or `None` if not found.
    pub fn get_entity(&self, id: u64) -> Option<&Entity> {
        self.nodes.iter().find(|e| e.id == id)
    }

    /// Returns all relations originating from the given entity.
    pub fn outgoing_relations(&self, source_id: u64) -> Vec<&Relation> {
        self.edges
            .iter()
            .filter(|r| r.source == source_id)
            .collect()
    }

    /// Returns all relations pointing to the given entity.
    pub fn incoming_relations(&self, target_id: u64) -> Vec<&Relation> {
        self.edges
            .iter()
            .filter(|r| r.target == target_id)
            .collect()
    }

    /// Detects whether the graph contains any cycles.
    ///
    /// Uses DFS-based cycle detection. Returns `true`
    /// if any cycle exists.
    pub fn has_cycles(&self) -> bool {
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();

        for node in &self.nodes {
            if self.dfs_cycle_check(
                node.id,
                &mut visited,
                &mut rec_stack,
            ) {
                return true;
            }
        }
        false
    }

    /// Performs DFS-based cycle detection starting from
    /// the given node.
    fn dfs_cycle_check(
        &self,
        node_id: u64,
        visited: &mut std::collections::HashSet<u64>,
        rec_stack: &mut std::collections::HashSet<u64>,
    ) -> bool {
        if !visited.insert(node_id) {
            return false;
        }
        rec_stack.insert(node_id);

        for relation in &self.edges {
            if relation.source != node_id {
                continue;
            }
            if rec_stack.contains(&relation.target) {
                return true;
            }
            if !visited.contains(&relation.target) {
                if self.dfs_cycle_check(
                    relation.target,
                    visited,
                    rec_stack,
                ) {
                    return true;
                }
            }
        }

        rec_stack.remove(&node_id);
        false
    }

    /// Returns the number of entities in the graph.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of relations in the graph.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Validates the graph, ensuring all nodes have valid
    /// states and all edges reference existing nodes.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::InvalidState`] if a node fails
    /// validation.
    /// Returns [`GenesisError::IncompatibleEntities`] if an
    /// edge references a non-existent node.
    /// Returns [`GenesisError::CycleDetected`] if the graph
    /// contains cycles.
    pub fn validate(&self) -> GenesisResult<()> {
        for node in &self.nodes {
            node.validate()?;
        }
        for relation in &self.edges {
            if !self.nodes.iter().any(|e| e.id == relation.source) {
                return Err(GenesisError::IncompatibleEntities {
                    reason: format!(
                        "edge source {} not found in graph",
                        relation.source
                    ),
                });
            }
            if !self.nodes.iter().any(|e| e.id == relation.target) {
                return Err(GenesisError::IncompatibleEntities {
                    reason: format!(
                        "edge target {} not found in graph",
                        relation.target
                    ),
                });
            }
            relation.validate()?;
        }
        if self.has_cycles() {
            return Err(GenesisError::CycleDetected(
                "graph contains cycles".to_string(),
            ));
        }
        Ok(())
    }
}

impl Default for CognitiveGraph {
    fn default() -> Self {
        Self::new()
    }
}