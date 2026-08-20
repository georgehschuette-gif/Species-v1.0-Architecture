// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// Contextualizer: Enriches extracted meaning with situational context.
///
/// Maintains a sliding window of recent symbols and relations,
/// computes contextual activation, and propagates activation
/// across related symbols.
pub struct Contextualizer {
    pub context_window: usize,
    pub decay_rate: f64,
    pub window: ContextWindow,
    pub propagation_steps: usize,
}

impl Contextualizer {
    /// Create a new contextualizer.
    pub fn new(context_window: usize, decay_rate: f64) -> PerceptionResult<Self> {
        if context_window == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "context window must be positive".into(),
            ));
        }
        if decay_rate <= 0.0 || decay_rate >= 1.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "decay rate must be in (0, 1)".into(),
            ));
        }
        Ok(Self {
            context_window,
            decay_rate,
            window: ContextWindow::new(context_window)?,
            propagation_steps: 1,
        })
    }

    /// Inject a symbol into the context window.
    pub fn inject_symbol(&mut self, symbol: Symbol) -> PerceptionResult<()> {
        self.window.push_symbol(symbol)?;
        Ok(())
    }

    /// Propagate activation from a focal symbol to neighbors within the window.
    pub fn propagate(&mut self, focal_id: usize) -> PerceptionResult<Vec<f64>> {
        let mut activations: Vec<f64> = vec![0.0; self.window.symbols.len()];
        let mut focal_idx = None;
        for (i, sym) in self.window.symbols.iter().enumerate() {
            if sym.id == focal_id {
                focal_idx = Some(i);
                activations[i] = 1.0;
            }
        }
        let focal_idx = focal_idx.ok_or(ExtractionError::EmptyContext)?;
        for _ in 0..self.propagation_steps {
            let current = activations.clone();
            for (i, sym) in self.window.symbols.iter().enumerate() {
                if i == focal_idx {
                    continue;
                }
                let mut incoming = 0.0;
                for rel in &self.window.relations {
                    if rel.target == sym.id && current.get(rel.source).copied().unwrap_or(0.0) > 0.0 {
                        incoming += current[rel.source] * rel.strength;
                    }
                    if rel.source == sym.id && current.get(rel.target).copied().unwrap_or(0.0) > 0.0 {
                        incoming += current[rel.target] * rel.strength;
                    }
                }
                activations[i] = (current[i] + incoming) * self.decay_rate;
            }
            for a in &mut activations {
                *a = a.clamp(0.0, 1.0);
            }
        }
        Ok(activations)
    }

    /// Compute the entropy of the current context distribution.
    pub fn entropy(&self) -> PerceptionResult<f64> {
        if self.window.symbols.is_empty() {
            return Ok(0.0);
        }
        let mut total = 0.0;
        for sym in &self.window.symbols {
            let p = sym.activation;
            if p > 0.0 {
                total -= p * p.ln();
            }
        }
        Ok(total / self.window.symbols.len() as f64)
    }

    fn contextualize(&self, symbol: &Symbol, window: &ContextWindow) -> PerceptionResult<Vec<f64>> {
        if window.symbols.is_empty() {
            return Err(ExtractionError::EmptyContext.into());
        }
        let mut context_vec = Vec::new();
        context_vec.push(self.decay_rate);
        context_vec.push(window.symbols.len() as f64 / self.context_window as f64);
        let dissimilarity_sum: f64 = window.symbols.iter().map(|s| s.euclidean_distance(symbol).unwrap_or(0.0)).sum();
        context_vec.push(dissimilarity_sum / window.symbols.len() as f64);
        let num_relations = window.relations.len();
        context_vec.push(num_relations as f64 / (window.symbols.len().max(1) as f64).powi(2));
        context_vec.extend_from_slice(&symbol.feature_vector[..4.min(symbol.feature_vector.len())]);
        Ok(context_vec)
    }
}
