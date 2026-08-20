// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MemoryError;

use super::*;

/// SemanticFact: A grounded assertion in the knowledge store.
///
/// Facts encode subject-predicate-object triples with confidence and source
/// reliability metadata. They can be reinforced through repeated verification
/// or discounted in light of contradictory evidence.
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticFact {
    /// Subject entity of the fact.
    pub subject: String,
    /// Predicate relation of the fact.
    pub predicate: String,
    /// Object entity of the fact.
    pub object: String,
    /// Confidence in the truth of this fact in [0.0, 1.0].
    pub confidence: f64,
    /// Reliability of the fact's source in [0.0, 1.0].
    pub source_reliability: f64,
    /// Abstract time since last verification.
    pub last_verified: f64,
    /// Number of supporting observations.
    pub support_count: usize,
    /// Optional tags for categorization.
    pub tags: Vec<String>,
}

impl SemanticFact {
    /// Minimum valid confidence.
    pub const MIN_CONFIDENCE: f64 = 0.0;
    /// Maximum valid confidence.
    pub const MAX_CONFIDENCE: f64 = 1.0;

    /// Creates a new SemanticFact from a subject-predicate-object triple.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::MissingInput`] if any part of the triple is empty.
    /// Returns [`MemoryError::OutOfRange`] if confidence or reliability is outside [0.0, 1.0].
    pub fn new(
        subject: impl Into<String>,
        predicate: impl Into<String>,
        object: impl Into<String>,
        confidence: f64,
        source_reliability: f64,
    ) -> Result<Self, MemoryError> {
        let s = subject.into();
        let p = predicate.into();
        let o = object.into();
        if s.is_empty() || p.is_empty() || o.is_empty() {
            return Err(MemoryError::MissingInput(
                "fact triple must have non-empty subject, predicate, and object".into(),
            ));
        }
        if !(Self::MIN_CONFIDENCE..=Self::MAX_CONFIDENCE).contains(&confidence) {
            return Err(MemoryError::OutOfRange {
                field: "confidence".into(),
                value: confidence,
                min: Self::MIN_CONFIDENCE,
                max: Self::MAX_CONFIDENCE,
            });
        }
        if !(0.0..=1.0).contains(&source_reliability) {
            return Err(MemoryError::OutOfRange {
                field: "source_reliability".into(),
                value: source_reliability,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            subject: s,
            predicate: p,
            object: o,
            confidence,
            source_reliability,
            last_verified: 0.0,
            support_count: 1,
            tags: Vec::new(),
        })
    }

    /// Reinforces the fact, increasing confidence and support count.
    pub fn reinforce(&mut self, amount: f64) {
        self.confidence = (self.confidence + amount * self.source_reliability).min(Self::MAX_CONFIDENCE);
        self.support_count += 1;
    }

    /// Discounts the fact, reducing confidence due to contradictory evidence.
    pub fn discount(&mut self, amount: f64) {
        self.confidence = (self.confidence - amount).max(Self::MIN_CONFIDENCE);
    }

    /// Verifies the fact against a source, updating reliability.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::OutOfRange`] if source reliability is outside [0.0, 1.0].
    pub fn verify(&mut self, source_reliability: f64) -> Result<(), MemoryError> {
        if !(0.0..=1.0).contains(&source_reliability) {
            return Err(MemoryError::OutOfRange {
                field: "source_reliability".into(),
                value: source_reliability,
                min: 0.0,
                max: 1.0,
            });
        }
        self.source_reliability = (self.source_reliability + source_reliability) / 2.0;
        self.last_verified = 0.0;
        self.reinforce(0.05);
        Ok(())
    }

    /// Computes the effective confidence factoring in source reliability.
    pub fn confidence_score(&self) -> f64 {
        self.confidence * self.source_reliability
    }

    /// Returns whether this fact is considered core knowledge.
    pub fn is_core(&self) -> bool {
        self.confidence >= DEFAULT_CONFIDENCE_FLOOR && self.support_count >= 3
    }

    /// Adds a tag to this fact.
    pub fn add_tag(&mut self, tag: impl Into<String>) {
        let t = tag.into();
        if !self.tags.contains(&t) {
            self.tags.push(t);
        }
    }

    /// Returns the triple as a dot-notation string.
    pub fn triple(&self) -> String {
        format!("{}.{}.{}", self.subject, self.predicate, self.object)
    }

    /// Validates the fact state.
    pub fn validate(&self) -> Result<(), MemoryError> {
        Self::new(
            &self.subject,
            &self.predicate,
            &self.object,
            self.confidence,
            self.source_reliability,
        )?;
        if self.last_verified < 0.0 {
            return Err(MemoryError::OutOfRange {
                field: "last_verified".into(),
                value: self.last_verified,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        Ok(())
    }
}
