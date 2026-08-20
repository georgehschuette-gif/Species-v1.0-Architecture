// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// ClockSynchronizer: Aligns temporal clocks across sensory channels.
///
/// Tracks per-source clock offsets and drift, then converts
/// arbitrary timestamps into a common reference frame.
pub struct ClockSynchronizer {
    pub precision: f64,
    pub max_drift: f64,
    pub clock_offsets: Vec<(usize, i64)>,
    pub drift_estimates: Vec<(usize, f64)>,
    pub reference: MonotonicTimestamp,
    pub sample_count: usize,
}

impl ClockSynchronizer {
    /// Create a new synchronizer with given precision (nanoseconds).
    pub fn new(precision_ns: f64, max_drift_ns: f64) -> PerceptionResult<Self> {
        if precision_ns <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "precision must be positive".into(),
            ));
        }
        if max_drift_ns <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "max drift must be positive".into(),
            ));
        }
        Ok(Self {
            precision: precision_ns,
            max_drift: max_drift_ns,
            clock_offsets: Vec::new(),
            drift_estimates: Vec::new(),
            reference: 0,
            sample_count: 0,
        })
    }

    /// Register a remote clock with an estimated offset from the reference.
    pub fn register_clock(&mut self, clock_id: usize, offset_ns: i64) -> PerceptionResult<()> {
        if clock_id > 16 {
            return Err(PerceptionError::InvalidConfiguration(
                "too many clocks registered".into(),
            ));
        }
        if let Some(pos) = self.clock_offsets.iter().position(|(id, _)| *id == clock_id) {
            self.clock_offsets[pos].1 = offset_ns;
        } else {
            self.clock_offsets.push((clock_id, offset_ns));
        }
        Ok(())
    }

    /// Convert a timestamp from a remote clock to the reference frame.
    pub fn synchronize(&self, timestamp: MonotonicTimestamp, source: usize) -> PerceptionResult<MonotonicTimestamp> {
        let offset = self.clock_offsets.iter().find(|(id, _)| *id == source).map(|(_, off)| *off).ok_or_else(|| {
            PerceptionError::ComputationError(format!("unregistered clock source: {}", source))
        })?;
        let synced = if offset >= 0 {
            timestamp + offset as u64
        } else if timestamp > (-offset) as u64 {
            timestamp - (-offset) as u64
        } else {
            0
        };
        Ok(synced)
    }

    /// Update drift estimate and clamp to tolerance.
    pub fn update_drift(&mut self, source: usize, observed_drift: f64) -> PerceptionResult<bool> {
        if observed_drift.abs() > self.max_drift {
            return Ok(false);
        }
        if let Some(pos) = self.drift_estimates.iter().position(|(id, _)| *id == source) {
            let old = self.drift_estimates[pos].1;
            self.drift_estimates[pos].1 = old * 0.9 + observed_drift * 0.1;
        } else {
            self.drift_estimates.push((source, observed_drift));
        }
        Ok(true)
    }

    /// Estimate clock drift for a registered clock.
    pub fn drift_estimate(&self, source: usize) -> PerceptionResult<f64> {
        self.drift_estimates
            .iter()
            .find(|(id, _)| *id == source)
            .map(|(_, drift)| *drift)
            .ok_or_else(|| PerceptionError::ComputationError(format!("no drift estimate for clock {}", source)))
    }
}
