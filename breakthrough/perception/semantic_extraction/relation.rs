// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// RelationExtractor: Identifies relationships between extracted symbols.
///
/// Uses co-occurrence statistics and feature-space distance to
/// infer directed and undirected relations.
pub struct RelationExtractor {
    pub relation_types: usize,
    pub cooccurrence_window: usize,
    pub min_cooccurrence: usize,
    pub relation_count: usize,
    pub relation_table: Vec<Relation>,
}

impl RelationExtractor {
    /// Create a new relation extractor.
    pub fn new(relation_types: usize, cooccurrence_window: usize, min_cooccurrence: usize) -> PerceptionResult<Self> {
        if relation_types == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "relation_types must be positive".into(),
            ));
        }
        if cooccurrence_window == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "cooccurrence window must be positive".into(),
            ));
        }
        Ok(Self {
            relation_types,
            cooccurrence_window,
            min_cooccurrence,
            relation_count: 0,
            relation_table: Vec::new(),
        })
    }

    /// Determine the strongest relation type given two symbol feature vectors.
    pub fn classify_relation(
        &self,
        source: &Symbol,
        target: &Symbol,
    ) -> PerceptionResult<RelationType> {
        if source.feature_vector.len() != target.feature_vector.len() {
            return Err(PerceptionError::InvalidConfiguration(
                "feature dimension mismatch".into(),
            ));
        }
        let sim = source.similarity(target)?;
        let dist = source.euclidean_distance(target)?;
        if sim > 0.8 {
            Ok(RelationType::Similarity)
        } else if dist > 2.0 {
            Ok(RelationType::Opposition)
        } else if source.feature_vector[0] < target.feature_vector[0] {
            Ok(RelationType::Sequence)
        } else if sim > 0.3 {
            Ok(RelationType::Coordinate)
        } else {
            Ok(RelationType::Causality)
        }
    }

    /// Compute relation strength from similarity and context.
    pub fn strength(&self, source: &Symbol, target: &Symbol) -> PerceptionResult<f64> {
        let sim = source.similarity(target)?;
        let raw = (sim + 0.5).min(1.0).max(0.0);
        Ok(raw)
    }

    fn extract_relations(&mut self, symbols: &[Symbol]) -> PerceptionResult<Vec<Relation>> {
        if symbols.len() < 2 {
            return Ok(Vec::new());
        }
        let mut relations = Vec::new();
        for i in 0..symbols.len() {
            for j in (i + 1)..symbols.len() {
                let rel_type = self.classify_relation(&symbols[i], &symbols[j])?;
                let strength = self.strength(&symbols[i], &symbols[j])?;
                if strength >= self.min_cooccurrence as f64 / self.cooccurrence_window as f64 {
                    relations.push(Relation::new(symbols[i].id, symbols[j].id, rel_type, strength)?);
                }
            }
        }
        self.relation_table.extend(relations.clone());
        self.relation_count = self.relation_table.len();
        Ok(relations)
    }
}
