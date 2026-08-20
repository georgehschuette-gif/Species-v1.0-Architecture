// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::HashMap;
use std::fmt;

use super::test::TestResult;
use super::falsification::FalsificationCriterion;
use crate::RealityError;

/// Verification: The process of checking predictions against observations.
///
/// Verification is the critical step that closes the reality loop
/// by comparing what was predicted with what was actually observed.
/// It drives calibration and learning.
#[derive(Debug, Clone, PartialEq)]
pub struct Verification {
    /// Unique identifier for this verification.
    pub id: String,
    /// The prediction being verified.
    pub prediction_id: String,
    /// The observed value.
    pub observed: f64,
    /// The predicted value.
    pub predicted: f64,
    /// The error (observed - predicted).
    pub error: f64,
    /// Test result.
    pub result: TestResult,
    /// Falsification criteria applied.
    pub criteria: Vec<FalsificationCriterion>,
    /// Timestamp of verification.
    pub timestamp: f64,
    /// Contextual metadata.
    pub metadata: HashMap<String, String>,
}

impl Verification {
    /// Minimum error threshold for considering a verification meaningful.
    pub const MIN_ERROR_THRESHOLD: f64 = 1e-10;

    /// Creates a new verification.
    pub fn new(
        id: impl Into<String>,
        prediction_id: impl Into<String>,
        observed: f64,
        predicted: f64,
        timestamp: f64,
    ) -> Self {
        let error = observed - predicted;
        let result = if error.abs() < Self::MIN_ERROR_THRESHOLD {
            TestResult::Pass
        } else {
            TestResult::Fail
        };
        Self {
            id: id.into(),
            prediction_id: prediction_id.into(),
            observed,
            predicted,
            error,
            result,
            criteria: Vec::new(),
            timestamp,
            metadata: HashMap::new(),
        }
    }

    /// Adds a falsification criterion to this verification.
    pub fn add_criterion(&mut self, criterion: FalsificationCriterion) {
        if !criterion.passes(self.error.abs()) {
            self.result = TestResult::Fail;
        }
        self.criteria.push(criterion);
    }

    /// Returns whether the verification passed all criteria.
    pub fn passed(&self) -> bool {
        self.result == TestResult::Pass
    }

    /// Returns the absolute error.
    pub fn absolute_error(&self) -> f64 {
        self.error.abs()
    }

    /// Returns the relative error.
    pub fn relative_error(&self) -> f64 {
        if self.predicted.abs() < f64::EPSILON {
            f64::INFINITY
        } else {
            (self.error / self.predicted).abs()
        }
    }

    /// Adds metadata to the verification.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

