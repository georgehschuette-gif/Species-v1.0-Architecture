// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::VecDeque;
use std::fmt;

use super::magnitude::EffectMagnitude;
use crate::RealityError;

/// CausalChain: A sequence of causally linked events or states.
///
/// Causal chains trace how one event leads to another through
/// a series of cause-effect relationships.
#[derive(Debug, Clone, PartialEq)]
pub struct CausalChain {
    /// Links in the causal chain.
    pub links: VecDeque<CausalLink>,
    /// Overall confidence in the causal chain.
    pub confidence: f64,
    /// Whether this chain has been verified.
    pub verified: bool,
}

impl CausalChain {
    /// Maximum number of links in a chain.
    pub const MAX_LINKS: usize = 1000;

    /// Creates a new empty causal chain.
    pub fn new() -> Self {
        Self {
            links: VecDeque::new(),
            confidence: 1.0,
            verified: false,
        }
    }

    /// Adds a causal link to the chain.
    ///
    /// # Errors
    /// Returns `RealityError::CapacityExceeded` if the chain exceeds max links.
    pub fn add_link(&mut self, link: CausalLink) -> Result<(), RealityError> {
        if self.links.len() >= Self::MAX_LINKS {
            return Err(RealityError::CapacityExceeded {
                max: Self::MAX_LINKS,
                attempted: self.links.len() + 1,
            });
        }
        self.links.push_back(link);
        self.update_confidence();
        Ok(())
    }

    /// Returns the number of links in the chain.
    pub fn len(&self) -> usize {
        self.links.len()
    }

    /// Returns whether the chain is empty.
    pub fn is_empty(&self) -> bool {
        self.links.is_empty()
    }

    /// Returns the first link in the chain.
    pub fn first(&self) -> Option<&CausalLink> {
        self.links.front()
    }

    /// Returns the last link in the chain.
    pub fn last(&self) -> Option<&CausalLink> {
        self.links.back()
    }

    /// Updates the overall confidence based on link confidences.
    pub fn update_confidence(&mut self) {
        if self.links.is_empty() {
            self.confidence = 1.0;
            return;
        }
        let product: f64 = self.links.iter().map(|l| l.confidence).product();
        self.confidence = product.powf(1.0 / self.links.len() as f64);
    }

    /// Reverses the chain (if causal relationships are reversible).
    pub fn reverse(&self) -> Self {
        let mut reversed = Self::new();
        for link in self.links.iter().rev() {
            reversed.links.push_back(CausalLink {
                cause: link.effect.clone(),
                effect: link.cause.clone(),
                confidence: link.confidence,
                delay: link.delay,
            });
        }
        reversed.confidence = self.confidence;
        reversed.verified = self.verified;
        reversed
    }

    /// Marks the chain as verified.
    pub fn mark_verified(&mut self) {
        self.verified = true;
    }
}

/// CausalLink: A single cause-effect relationship in a chain.
#[derive(Debug, Clone, PartialEq)]
pub struct CausalLink {
    /// The cause (antecedent).
    pub cause: String,
    /// The effect (consequent).
    pub effect: String,
    /// Confidence in this causal link.
    pub confidence: f64,
    /// Time delay between cause and effect.
    pub delay: f64,
}

impl CausalLink {
    /// Creates a new causal link.
    pub fn new(
        cause: impl Into<String>,
        effect: impl Into<String>,
        confidence: f64,
        delay: f64,
    ) -> Self {
        Self {
            cause: cause.into(),
            effect: effect.into(),
            confidence: confidence.clamp(0.0, 1.0),
            delay: delay.max(0.0),
        }
    }

    /// Returns whether this link is strong (high confidence).
    pub fn is_strong(&self, threshold: f64) -> bool {
        self.confidence >= threshold
    }

    /// Returns the causal strength.
    pub fn strength(&self) -> f64 {
        self.confidence
    }
}

/// Outcome: The result of a consequence.
///
/// Outcomes capture the end state after all causal effects have
/// propagated through the system.
#[derive(Debug, Clone, PartialEq)]
pub struct Outcome {
    /// Unique identifier for this outcome.
    pub id: String,
    /// Description of the outcome.
    pub description: String,
    /// Magnitude of the outcome.
    pub magnitude: EffectMagnitude,
    /// Whether the outcome was expected.
    pub expected: bool,
    /// Timestamp when the outcome was observed.
    pub observed_at: f64,
    /// Associated causal chain.
    pub causal_chain: Option<CausalChain>,
}

impl Outcome {
    /// Creates a new outcome.
    pub fn new(
        id: impl Into<String>,
        description: impl Into<String>,
        magnitude: EffectMagnitude,
        expected: bool,
        observed_at: f64,
    ) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            magnitude,
            expected,
            observed_at,
            causal_chain: None,
        }
    }

    /// Sets the causal chain for this outcome.
    pub fn with_causal_chain(mut self, chain: CausalChain) -> Self {
        self.causal_chain = Some(chain);
        self
    }

    /// Returns whether this outcome was a surprise (unexpected).
    pub fn is_surprise(&self) -> bool {
        !self.expected
    }

    /// Returns the deviation from expected magnitude (placeholder).
    pub fn deviation(&self, expected_magnitude: EffectMagnitude) -> f64 {
        (self.magnitude.0 - expected_magnitude.0).abs()
    }
}

