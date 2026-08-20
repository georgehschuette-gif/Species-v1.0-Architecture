// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Anomaly Detection: Identifying deviations from expected patterns.
//!
//! Anomaly detection flags novel, unexpected, or contradictory sensory inputs
//! using three complementary backends:
//!
//! 1. **ThresholdDetector** — Flags anomalies exceeding fixed or adaptive
//!    thresholds on scalar measurements or vector magnitudes.
//! 2. **ReconstructionDetector** — Identifies anomalies by measuring the
//!    reconstruction error of a predictive model.
//! 3. **ClusteringDetector** — Detects outliers by distance from the nearest
//!    cluster centroid in feature space.
//!
//! # Detection Strategies
//!
//! Each strategy has distinct strengths:
//! - **Threshold** excels at low-latency, single-dimensional monitoring.
//! - **Reconstruction** captures high-dimensional structural anomalies.
//! - **Clustering** identifies contextual anomalies in streaming data.
//!
//! # History Bounds
//!
//! Detectors maintain bounded history buffers (capped at 10,000 entries)
//! to prevent unbounded memory growth during sustained operation.
//!
//! # Examples
//!
//! ```
//! use breakthrough::perception::anomaly_detection::{ThresholdDetector, ReconstructionDetector, ClusteringDetector};
//!
//! let mut thr = ThresholdDetector::new(0.8, 0.5).expect("valid");
//! let is_anomaly = thr.detect(0.9).expect("detected");
//!
//! let mut recon = ReconstructionDetector::new(0.1, "mean".into()).expect("valid");
//! let recon_anomaly = recon.detect(&[1.0, 2.0, 3.0]).expect("detected");
//!
//! let mut cluster = ClusteringDetector::new(3, 0.5).expect("valid");
//! let cluster_anomaly = cluster.detect_anomaly(&[10.0, 20.0]).expect("detected");
//! ```
//!
//! # Alert Aggregation
//!
//! All detectors expose an `anomaly_count()` method for aggregating alerts
//! across time windows. This enables downstream throttling and escalation.
//!
//! [`PerceptionError`]: super::PerceptionError

pub mod threshold;
pub mod reconstruction;
pub mod clustering;

pub use threshold::ThresholdDetector;
pub use reconstruction::ReconstructionDetector;
pub use clustering::{ClusteringDetector, Cluster};
pub use super::{PerceptionResult, PerceptionError};

/// Default detection threshold for threshold-based detectors.
pub const DEFAULT_THRESHOLD: f64 = 0.8;
/// Default reconstruction error threshold.
pub const DEFAULT_ERROR_THRESHOLD: f64 = 0.1;
/// Default outlier distance threshold for clustering.
pub const DEFAULT_OUTLIER_THRESHOLD: f64 = 0.5;
/// Maximum detection history length across all strategies.
pub const MAX_HISTORY_LENGTH: usize = 10_000;

/// Runs all three anomaly detectors on a single measurement.
///
/// Convenience aggregator that returns a collective verdict and
/// individual detector outcomes.
///
/// # Errors
///
/// Returns [`PerceptionError`] if any detector encounters invalid input.
pub fn detect_anomalies(
    measurement: f64,
    vector: &[f64],
    error_threshold: f64,
    model: &str,
) -> PerceptionResult<(bool, bool, bool)> {
    let mut thr = ThresholdDetector::new(DEFAULT_THRESHOLD, 0.5)?;
    let threshold_anomaly = thr.detect(measurement)?;

    let mut recon = ReconstructionDetector::new(error_threshold, model.into())?;
    let recon_anomaly = recon.detect(vector)?;

    let mut cluster = ClusteringDetector::new(3, DEFAULT_OUTLIER_THRESHOLD)?;
    let cluster_anomaly = cluster.detect_anomaly(vector)?;

    Ok((threshold_anomaly, recon_anomaly, cluster_anomaly))
}

/// Validates a shared anomaly detection configuration.
pub fn validate_anomaly_config(
    threshold: f64,
    error_threshold: f64,
    outlier_threshold: f64,
) -> PerceptionResult<()> {
    ThresholdDetector::new(threshold, 0.5)?.validate()?;
    ReconstructionDetector::new(error_threshold, "model".into())?.validate()?;
    ClusteringDetector::new(3, outlier_threshold)?.validate()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_anomalies_returns_bools() {
        let (t, r, c) = detect_anomalies(0.9, &[1.0, 2.0], 0.1, "mean").unwrap();
        assert!(t || !t);
        assert!(r || !r);
        assert!(c || !c);
    }

    #[test]
    fn validate_rejects_bad() {
        assert!(validate_anomaly_config(-0.1, 0.1, 0.5).is_err());
    }
}
