// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::HashMap;
use std::fmt;

use crate::RealityError;

/// TestResult: The result of a verification test.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestResult {
    /// The test passed.
    Pass,
    /// The test failed.
    Fail,
    /// The test was inconclusive.
    Inconclusive,
    /// The test was not run.
    NotRun,
}

impl TestResult {
    /// Returns the string label of this test result.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::Inconclusive => "inconclusive",
            Self::NotRun => "not_run",
        }
    }

    /// Returns whether this result is a pass.
    pub fn is_pass(&self) -> bool {
        matches!(self, Self::Pass)
    }

    /// Returns whether this result is a failure.
    pub fn is_failure(&self) -> bool {
        matches!(self, Self::Fail)
    }
}

impl std::fmt::Display for TestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for TestResult {
    fn default() -> Self {
        Self::NotRun
    }
}

