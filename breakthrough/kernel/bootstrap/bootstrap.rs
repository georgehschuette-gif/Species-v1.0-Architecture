// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::{TopologyEngine, KernelError, KernelResult};

#[derive(Debug, Clone)]
pub struct Bootstrap {
    topology: TopologyEngine,
    initialized: bool,
}

impl Bootstrap {
    pub fn new(topology: TopologyEngine) -> Self {
        Self {
            topology,
            initialized: false,
        }
    }

    pub fn initialize(&mut self) -> KernelResult<()> {
        if self.initialized {
            return Err(KernelError::InvalidState(
                "bootstrap already initialized".to_string(),
            ));
        }
        self.topology.advance_tick();
        self.initialized = true;
        Ok(())
    }

    pub fn is_ready(&self) -> bool {
        self.initialized && self.topology.node_count() > 0
    }
}

impl std::fmt::Display for Bootstrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Bootstrap(initialized={}, nodes={})",
            self.initialized,
            self.topology.node_count()
        )
    }
}
