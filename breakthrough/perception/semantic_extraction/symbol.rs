// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// SymbolExtractor: Converts sensory patterns into discrete symbols.
///
/// Uses an incremental vocabulary with activation-threshold gating
/// to map continuous feature vectors to a sparse symbol set.
pub struct SymbolExtractor {
    pub vocabulary_size: usize,
    pub activation_threshold: f64,
    pub next_id: usize,
    pub active_symbols: usize,
    pub symbol_table: Vec<Symbol>,
}

impl SymbolExtractor {
    /// Create a new extractor with bounded vocabulary.
    pub fn new(vocabulary_size: usize, activation_threshold: f64) -> PerceptionResult<Self> {
        if vocabulary_size == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "vocabulary size must be positive".into(),
            ));
        }
        if activation_threshold <= 0.0 || activation_threshold > 1.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "activation threshold must be in (0, 1]".into(),
            ));
        }
        Ok(Self {
            vocabulary_size,
            activation_threshold,
            next_id: 0,
            active_symbols: 0,
            symbol_table: Vec::with_capacity(vocabulary_size),
        })
    }

    /// Hash-like feature quantization used for symbol lookup.
    pub fn quantize(features: &[f64]) -> u64 {
        let mut hash: u64 = 0;
        for (i, v) in features.iter().enumerate() {
            let bits = v.to_bits();
            hash = hash.wrapping_add((bits.wrapping_mul(i as u64 + 1)).rotate_left(7));
        }
        hash
    }

    /// Locate or create a symbol for the given feature vector.
    pub fn lookup_or_create(&mut self, features: Vec<f64>) -> PerceptionResult<usize> {
        if features.is_empty() {
            return Err(ExtractionError::InsufficientSignal.into());
        }
        let hash = Self::quantize(&features);
        for sym in &mut self.symbol_table {
            if Self::quantize(&sym.feature_vector) == hash {
                sym.activation = (sym.activation + 0.1).min(1.0);
                return Ok(sym.id);
            }
        }
        if self.symbol_table.len() >= self.vocabulary_size {
            return Err(ExtractionError::VocabularyOverflow.into());
        }
        let id = self.next_id;
        self.next_id += 1;
        let mut symbol = Symbol::new(id, features)?;
        symbol.activation = self.activation_threshold;
        symbol.timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        self.symbol_table.push(symbol);
        self.active_symbols = self.symbol_table.len();
        Ok(id)
    }

    fn extract(&mut self, signal: &[f64]) -> PerceptionResult<Symbol> {
        if signal.is_empty() {
            return Err(ExtractionError::InsufficientSignal.into());
        }
        let id = self.lookup_or_create(signal.to_vec())?;
        let symbol = self.symbol_table.iter_mut().find(|s| s.id == id).ok_or_else(|| {
            PerceptionError::ComputationError("symbol not found after creation".into())
        })?;
        Ok(symbol.clone())
    }

    fn vocabulary_capacity(&self) -> usize {
        self.vocabulary_size
    }
}
