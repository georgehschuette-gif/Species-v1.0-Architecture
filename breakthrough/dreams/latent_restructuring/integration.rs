// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::DreamsError;
use crate::DreamsResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegrationMethod {
    PatternCompletion,
    SchemaAlignment,
    MemoryBinding,
    ContextualEmbedding,
    CrossModalBinding,
}

impl std::fmt::Display for IntegrationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntegrationMethod::PatternCompletion => write!(f, "PatternCompletion"),
            IntegrationMethod::SchemaAlignment => write!(f, "SchemaAlignment"),
            IntegrationMethod::MemoryBinding => write!(f, "MemoryBinding"),
            IntegrationMethod::ContextualEmbedding => write!(f, "ContextualEmbedding"),
            IntegrationMethod::CrossModalBinding => write!(f, "CrossModalBinding"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KnowledgeIntegration {
    pub integration_id: u64,
    pub method: IntegrationMethod,
    pub sources_integrated: u64,
    pub coherence_gain: f64,
}

impl KnowledgeIntegration {
    pub fn new(integration_id: u64, method: IntegrationMethod) -> Self {
        Self {
            integration_id,
            method,
            sources_integrated: 0,
            coherence_gain: 0.0,
        }
    }

    pub fn validate(&self) -> DreamsResult<()> {
        if self.coherence_gain < 0.0 {
            return Err(DreamsError::InvalidRestructuring(
                "Coherence gain cannot be negative".to_string(),
            ));
        }
        Ok(())
    }
}

impl std::fmt::Display for KnowledgeIntegration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KnowledgeIntegration(id={}, method={}, sources={}, gain={:.2})",
            self.integration_id,
            self.method,
            self.sources_integrated,
            self.coherence_gain
        )
    }
}