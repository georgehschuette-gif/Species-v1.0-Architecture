// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Semantic Extraction: Deriving meaning from raw sensory signals.
//! Converts patterns into cognitive symbols and relations.

pub mod symbol;
pub mod relation;
pub mod context;

pub use symbol::SymbolExtractor;
pub use relation::RelationExtractor;
pub use context::Contextualizer;

pub use super::PerceptionResult;
pub use super::PerceptionError;

/// A discrete cognitive symbol extracted from a sensory pattern.
#[derive(Debug, Clone)]
pub struct Symbol {
    pub id: usize,
    pub feature_vector: Vec<f64>,
    pub activation: f64,
    pub timestamp: u64,
}

impl Symbol {
    pub fn new(id: usize, features: Vec<f64>) -> PerceptionResult<Self> {
        if features.is_empty() {
            return Err(PerceptionError::InvalidConfiguration(
                "symbol feature vector must not be empty".into(),
            ));
        }
        Ok(Self {
            id,
            feature_vector: features,
            activation: 0.0,
            timestamp: 0,
        })
    }

    /// Cosine similarity between two symbols.
    pub fn similarity(&self, other: &Symbol) -> PerceptionResult<f64> {
        if self.feature_vector.len() != other.feature_vector.len() {
            return Err(PerceptionError::InvalidConfiguration(
                "feature dimension mismatch".into(),
            ));
        }
        let dot: f64 = self.feature_vector.iter().zip(other.feature_vector.iter()).map(|(a, b)| a * b).sum();
        let norm_a: f64 = self.feature_vector.iter().map(|v| v * v).sum::<f64>().sqrt();
        let norm_b: f64 = other.feature_vector.iter().map(|v| v * v).sum::<f64>().sqrt();
        if norm_a == 0.0 || norm_b == 0.0 {
            return Ok(0.0);
        }
        Ok(dot / (norm_a * norm_b))
    }

    /// Euclidean distance between two symbols.
    pub fn euclidean_distance(&self, other: &Symbol) -> PerceptionResult<f64> {
        if self.feature_vector.len() != other.feature_vector.len() {
            return Err(PerceptionError::InvalidConfiguration(
                "feature dimension mismatch".into(),
            ));
        }
        let dist: f64 = self.feature_vector.iter().zip(other.feature_vector.iter()).map(|(a, b)| (a - b).powi(2)).sum::<f64>().sqrt();
        Ok(dist)
    }
}

/// Type of relation between two symbols.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationType {
    Similarity,
    Causality,
    Containment,
    Sequence,
    Opposition,
    Coordinate,
}

impl std::fmt::Display for RelationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Similarity => write!(f, "similarity"),
            Self::Causality => write!(f, "causality"),
            Self::Containment => write!(f, "containment"),
            Self::Sequence => write!(f, "sequence"),
            Self::Opposition => write!(f, "opposition"),
            Self::Coordinate => write!(f, "coordinate"),
        }
    }
}

/// A relation between two symbols.
#[derive(Debug, Clone)]
pub struct Relation {
    pub source: usize,
    pub target: usize,
    pub relation_type: RelationType,
    pub strength: f64,
}

impl Relation {
    pub fn new(source: usize, target: usize, relation_type: RelationType, strength: f64) -> PerceptionResult<Self> {
        if strength < 0.0 || strength > 1.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "relation strength must be in [0, 1]".into(),
            ));
        }
        if source == target {
            return Err(PerceptionError::InvalidConfiguration(
                "relation source and target cannot be identical".into(),
            ));
        }
        Ok(Self {
            source,
            target,
            relation_type,
            strength,
        })
    }
}

/// A temporal context window wrapping a set of symbols and relations.
#[derive(Debug, Clone)]
pub struct ContextWindow {
    pub symbols: Vec<Symbol>,
    pub relations: Vec<Relation>,
    pub window_size: usize,
    pub timestamp: u64,
}

impl ContextWindow {
    pub fn new(window_size: usize) -> PerceptionResult<Self> {
        if window_size == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "context window size must be positive".into(),
            ));
        }
        Ok(Self {
            symbols: Vec::new(),
            relations: Vec::new(),
            window_size,
            timestamp: 0,
        })
    }

    /// Add a symbol to the window, evicting the oldest if full.
    pub fn push_symbol(&mut self, symbol: Symbol) -> PerceptionResult<()> {
        if self.symbols.len() >= self.window_size {
            self.symbols.remove(0);
        }
        self.symbols.push(symbol);
        Ok(())
    }

    /// Compute average pair-wise similarity among symbols in the window.
    pub fn coherence(&self) -> PerceptionResult<f64> {
        if self.symbols.len() < 2 {
            return Ok(0.0);
        }
        let mut total = 0.0;
        let mut count = 0;
        for i in 0..self.symbols.len() {
            for j in (i + 1)..self.symbols.len() {
                let sim = self.symbols[i].similarity(&self.symbols[j])?;
                total += sim;
                count += 1;
            }
        }
        Ok(total / count as f64)
    }
}

/// Failure modes during semantic extraction.
#[derive(Debug)]
pub enum ExtractionError {
    InsufficientSignal,
    VocabularyOverflow,
    UnsupportedRelation,
    EmptyContext,
}

impl std::fmt::Display for ExtractionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InsufficientSignal => write!(f, "insufficient signal for extraction"),
            Self::VocabularyOverflow => write!(f, "vocabulary capacity exceeded"),
            Self::UnsupportedRelation => write!(f, "relation type unsupported"),
            Self::EmptyContext => write!(f, "context window is empty"),
        }
    }
}

impl std::error::Error for ExtractionError {}

impl From<ExtractionError> for PerceptionError {
    fn from(err: ExtractionError) -> Self {
        PerceptionError::ComputationError(err.to_string())
    }
}
