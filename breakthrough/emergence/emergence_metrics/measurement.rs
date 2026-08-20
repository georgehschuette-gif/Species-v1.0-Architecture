// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::VecDeque;
use std::fmt;

use super::metric::Metric;

#[derive(Debug, Clone, PartialEq)]
pub struct Measurement {
    metrics: Vec<Metric>,
    window_size: usize,
}

impl Measurement {
    pub fn new(metrics: Vec<Metric>, window_size: usize) -> crate::Result<Self> {
        if window_size == 0 {
            return Err(crate::EmergenceError::MeasurementError(
                "window_size must be positive".to_string(),
            ));
        }
        Ok(Measurement {
            metrics,
            window_size,
        })
    }

    pub fn metrics(&self) -> &[Metric] {
        &self.metrics
    }

    pub fn window_size(&self) -> usize {
        self.window_size
    }

    pub fn aggregate(&self) -> crate::Result<Aggregate> {
        if self.metrics.is_empty() {
            return Err(crate::EmergenceError::MeasurementError(
                "cannot aggregate empty measurement".to_string(),
            ));
        }
        let mut sum = 0.0;
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for m in &self.metrics {
            let v = m.value();
            sum += v;
            if v < min {
                min = v;
            }
            if v > max {
                max = v;
            }
        }
        let mean = sum / self.metrics.len() as f64;
        Ok(Aggregate {
            count: self.metrics.len(),
            mean,
            min,
            max,
        })
    }

    pub fn validate(&self) -> crate::Result<()> {
        if self.window_size == 0 {
            return Err(crate::EmergenceError::MeasurementError(
                "window_size is zero".to_string(),
            ));
        }
        for m in &self.metrics {
            m.validate()?;
        }
        Ok(())
    }
}

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Measurement(metrics={}, window={})",
            self.metrics.len(),
            self.window_size
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Aggregate {
    count: usize,
    mean: f64,
    min: f64,
    max: f64,
}

impl Aggregate {
    pub fn count(&self) -> usize {
        self.count
    }

    pub fn mean(&self) -> f64 {
        self.mean
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }
}

impl fmt::Display for Aggregate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Aggregate(count={}, mean={:.3}, min={:.3}, max={:.3})",
            self.count, self.mean, self.min, self.max
        )
    }
}
