// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::genesis::{GenesisError, GenesisResult};

/// Causality Axiom: Effects follow causes in a directed acyclic relationship.
/// No effect may precede its cause in the cognitive timeline.
///
/// This axiom enforces temporal and logical ordering within the cognitive
/// ecosystem. All causal relationships must form a directed acyclic graph
/// (DAG), ensuring that no entity can be both the cause and the effect
/// of itself, directly or transitively.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CausalityAxiom {
    /// The maximum depth allowed for a causal chain before it must be
    /// considered a new causal root.
    pub max_chain_depth: u32,
}

impl CausalityAxiom {
    /// Default maximum depth for a causal chain.
    pub const DEFAULT_MAX_DEPTH: u32 = 64;

    /// Minimum allowed maximum depth.
    pub const MIN_MAX_DEPTH: u32 = 1;

    /// Creates a new Causality Axiom with the specified maximum chain depth.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if `max_chain_depth` is zero.
    pub fn new(max_chain_depth: u32) -> GenesisResult<Self> {
        if max_chain_depth < Self::MIN_MAX_DEPTH {
            return Err(GenesisError::OutOfRange {
                field: "max_chain_depth".to_string(),
                value: max_chain_depth as f64,
                min: Self::MIN_MAX_DEPTH as f64,
                max: u32::MAX as f64,
            });
        }
        Ok(Self { max_chain_depth })
    }

    /// Returns the default causality axiom with standard depth limit.
    pub fn default() -> Self {
        Self {
            max_chain_depth: Self::DEFAULT_MAX_DEPTH,
        }
    }

    /// Verifies that a cause-effect pair respects temporal ordering.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::ValidationFailure`] if `effect_time` is before
    /// `cause_time`.
    pub fn verify_temporal_order(
        &self,
        cause_time: f64,
        effect_time: f64,
    ) -> GenesisResult<()> {
        if cause_time > effect_time {
            return Err(GenesisError::ValidationFailure(format!(
                "effect time {} precedes cause time {}",
                effect_time, cause_time
            )));
        }
        if cause_time.is_nan() || effect_time.is_nan() {
            return Err(GenesisError::ComputationError(
                "temporal values cannot be NaN".to_string(),
            ));
        }
        Ok(())
    }

    /// Verifies that a causal chain of timestamps is properly ordered.
    ///
    /// Each element must be less than or equal to the next element.
    /// The chain length must not exceed the maximum allowed depth.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if the chain is empty or exceeds
    /// maximum depth.
    /// Returns [`GenesisError::ValidationFailure`] if the chain is not properly
    /// ordered.
    pub fn verify_chain(
        &self,
        timestamps: &[f64],
    ) -> GenesisResult<()> {
        if timestamps.is_empty() {
            return Err(GenesisError::MissingInput(
                "causal chain cannot be empty".to_string(),
            ));
        }
        if timestamps.len() > self.max_chain_depth as usize {
            return Err(GenesisError::OutOfRange {
                field: "chain_length".to_string(),
                value: timestamps.len() as f64,
                min: 0.0,
                max: self.max_chain_depth as f64,
            });
        }
        for i in 1..timestamps.len() {
            if timestamps[i - 1] > timestamps[i] {
                return Err(GenesisError::ValidationFailure(format!(
                    "causal chain not ordered at index {}: {} > {}",
                    i - 1,
                    timestamps[i - 1],
                    timestamps[i]
                )));
            }
        }
        Ok(())
    }

    /// Verifies that a proposed edge `(from, to)` in a causal graph does not
    /// create a cycle with the existing edges.
    ///
    /// The existing edges are given as a list of `(source, target)` pairs.
    /// The new edge `(from, to)` is checked to ensure that adding it would
    /// not introduce a cycle.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::CycleDetected`] if adding the edge would create
    /// a cycle.
    /// Returns [`GenesisError::ComputationError`] if the node count is zero.
    pub fn verify_acyclic(
        &self,
        existing_edges: &[(u64, u64)],
        from: u64,
        to: u64,
    ) -> GenesisResult<()> {
        if from == to {
            return Err(GenesisError::CycleDetected(format!(
                "self-loop detected on node {}",
                from
            )));
        }

        // BFS/DFS to check if `to` can reach `from` through existing edges.
        // If so, adding (from -> to) would create a cycle.
        let mut visited = std::collections::HashSet::new();
        let mut stack = vec![to];

        while let Some(current) = stack.pop() {
            if current == from {
                return Err(GenesisError::CycleDetected(format!(
                    "adding edge ({}, {}) creates cycle: {} is reachable from {}",
                    from, to, from, to
                )));
            }
            if !visited.insert(current) {
                continue;
            }
            for &(src, tgt) in existing_edges {
                if src == current && !visited.contains(&tgt) {
                    stack.push(tgt);
                }
            }
        }

        Ok(())
    }
}

impl Default for CausalityAxiom {
    fn default() -> Self {
        Self {
            max_chain_depth: Self::DEFAULT_MAX_DEPTH,
        }
    }
}