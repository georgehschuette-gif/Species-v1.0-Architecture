// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Metacognition: The system's awareness of its own cognitive processes.
//!
//! Metacognition enables self-monitoring, self-regulation, and meta-level
//! reasoning about the cognitive stack's health and strategy selection.
//!
//! 1. **SelfMonitor** — Observes cognitive process quality, tracks anomalies,
//!    and issues alerts when quality degrades below threshold.
//! 2. **SelfRegulate** — Adjusts cognitive parameters based on monitoring data,
//!    strengthening or restraining processes dynamically.
//! 3. **MetaReason** — Selects reasoning strategies based on problem complexity,
//!    managing overhead across meta-levels.
//!
//! # Monitoring Loop
//!
//! The monitoring loop runs at a configurable depth. Deeper monitoring
//! provides finer-grained quality signals but incurs higher overhead.
//!
//! # Regulation Dynamics
//!
//! Regulation applies three regimes:
//! - Below 0.5 quality: upward push to recover performance.
//! - Between 0.5 and 0.8: no adjustment (stable zone).
//! - Above 0.8: gentle downward restraint to prevent overfitting.
//!
//! # Strategy Selection
//!
//! Meta-reasoning evaluates complexity and assigns strategies:
//! - Low complexity (< 0.2): greedy.
//! - Moderate (0.2–0.5): heuristic.
//! - High (0.5–0.8): probabilistic.
//! - Extreme (> 0.8): exhaustive.
//!
//! # Examples
//!
//! ```
//! use breakthrough::cognition::metacognition::{SelfMonitor, SelfRegulate, MetaReason, StrategySelection};
//!
//! let mut monitor = SelfMonitor::new(2, 0.3).expect("valid");
//! let alert = monitor.check_quality(0.2).expect("checked");
//!
//! let regulator = SelfRegulate::new(0.5, 0.2).expect("valid");
//! let adjustment = regulator.regulate(0.3, 0.3).expect("regulated");
//!
//! let meta = MetaReason::new(3, StrategySelection::Heuristic).expect("valid");
//! let strategy = meta.select_strategy(0.4).expect("selected");
//! ```
//!
//! # Overhead Budgets
//!
//! Metacognition is subject to a meta-overhead budget: monitoring depth
//! and strategy complexity must remain within system limits.
//!
//! [`CognitionError`]: super::CognitionError

pub mod self_monitor;
pub mod self_regulate;
pub mod meta_reason;

pub use super::CognitionError;
pub use self_monitor::SelfMonitor;
pub use self_regulate::SelfRegulate;
pub use meta_reason::{MetaReason, StrategySelection};

/// Default monitoring depth for baseline self-assessment.
pub const DEFAULT_MONITORING_DEPTH: usize = 2;
/// Default quality alert threshold.
pub const DEFAULT_ALERT_THRESHOLD: f64 = 0.3;
/// Maximum monitoring depth before overhead is unacceptable.
pub const MAX_MONITORING_DEPTH: usize = 10;
/// Maximum meta-reasoning depth.
pub const MAX_REASONING_DEPTH: usize = 50;
/// Default regulation strength.
pub const DEFAULT_REGULATION_STRENGTH: f64 = 0.5;

/// Runs a complete metacognitive assessment cycle.
///
/// Checks quality, optionally regulates a parameter, and selects a
/// strategy appropriate to the current complexity.
///
/// # Errors
///
/// Propagates [`CognitionError`] from quality checks and regulation.
pub fn metacognitive_cycle(
    quality_score: f64,
    target: f64,
    complexity: f64,
) -> Result<(bool, f64, StrategySelection), CognitionError> {
    let mut monitor = SelfMonitor::new(DEFAULT_MONITORING_DEPTH, DEFAULT_ALERT_THRESHOLD)?;
    let alert = monitor.check_quality(quality_score)?;

    let regulator = SelfRegulate::new(DEFAULT_REGULATION_STRENGTH, 0.2)?;
    let adjustment = regulator.regulate(target, quality_score)?;

    let meta = MetaReason::new(3, StrategySelection::Heuristic)?;
    let strategy = meta.select_strategy(complexity)?;

    Ok((alert, adjustment, strategy))
}

/// Validates all metacognitive parameters within safe bounds.
pub fn validate_metacognition_config(
    monitoring_depth: usize,
    alert_threshold: f64,
    regulation_strength: f64,
) -> Result<(), CognitionError> {
    SelfMonitor::new(monitoring_depth, alert_threshold)?.validate()?;
    SelfRegulate::new(regulation_strength, 0.2)?.validate()?;
    MetaReason::new(3, StrategySelection::Heuristic)?.validate()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metacognitive_cycle_runs() {
        let (alert, adj, strat) = metacognitive_cycle(0.2, 0.4, 0.3).unwrap();
        assert!(alert); // 0.2 < 0.3 threshold
        assert!(adj >= 0.0);
        assert_eq!(strat, StrategySelection::Heuristic);
    }

    #[test]
    fn constants_are_valid() {
        assert!(DEFAULT_ALERT_THRESHOLD >= 0.0 && DEFAULT_ALERT_THRESHOLD <= 1.0);
        assert!(MAX_MONITORING_DEPTH > 0);
    }
}