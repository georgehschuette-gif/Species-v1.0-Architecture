// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// VisualStream: Light-based sensory input channel.
///
/// Processes raw pixel intensities or feature maps into normalized
/// visual descriptors suitable for downstream semantic extraction.
pub struct VisualStream {
    pub resolution: (usize, usize),
    pub frame_rate: f64,
    pub is_active: bool,
    pub filter_kernel: Vec<f64>,
    pub frame_count: u64,
}

impl VisualStream {
    /// Construct a new VisualStream with default configuration.
    pub fn new(resolution: (usize, usize), frame_rate: f64) -> PerceptionResult<Self> {
        if resolution.0 == 0 || resolution.1 == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "resolution dimensions must be positive".into(),
            ));
        }
        if frame_rate <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "frame rate must be positive".into(),
            ));
        }
        Ok(Self {
            resolution,
            frame_rate,
            is_active: true,
            filter_kernel: vec![0.0; 9],
            frame_count: 0,
        })
    }

    /// Apply a Gaussian-like smoothing kernel to the signal.
    pub fn apply_filter(&self, signal: &[f64]) -> PerceptionResult<Vec<f64>> {
        if signal.len() != 9 {
            return Err(PerceptionError::InvalidConfiguration(
                "visual filter expects 3x3 kernel (9 elements)".into(),
            ));
        }
        let mut output = vec![0.0; 9];
        let kernel = &[0.0625, 0.125, 0.0625, 0.125, 0.25, 0.125, 0.0625, 0.125, 0.0625];
        for i in 0..3 {
            for j in 0..3 {
                let mut sum = 0.0;
                for ki in 0..3 {
                    for kj in 0..3 {
                        let si = i + ki - 1;
                        let sj = j + kj - 1;
                        let idx = si * 3 + sj;
                        if si < 3 && sj < 3 && si >= 0 && sj >= 0 {
                            sum += signal[idx] * kernel[ki * 3 + kj];
                        }
                    }
                }
                output[i * 3 + j] = sum;
            }
        }
        Ok(output)
    }

    /// Compute total intensity across the window.
    pub fn total_intensity(&self, signal: &[f64]) -> PerceptionResult<f64> {
        if signal.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        Ok(signal.iter().sum())
    }
}

impl SensoryStream for VisualStream {
    fn configure(&mut self, config: StreamConfig) -> PerceptionResult<()> {
        config.validate()?;
        if config.channel != SensoryChannel::Visual {
            return Err(PerceptionError::InvalidConfiguration(
                "stream channel mismatch for visual".into(),
            ));
        }
        self.resolution = (config.resolution, config.resolution);
        self.frame_rate = config.sample_rate;
        self.is_active = config.enable_filtering;
        Ok(())
    }

    fn ingest(&mut self, raw_signal: &[f64]) -> PerceptionResult<Vec<f64>> {
        if !self.is_active {
            return Err(PerceptionError::ComputationError(
                "visual stream is inactive".into(),
            ));
        }
        if raw_signal.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        if raw_signal.len() < 9 {
            return Err(PerceptionError::InvalidConfiguration(
                "visual ingest requires at least 9 samples for 3x3 processing".into(),
            ));
        }
        let features = self.apply_filter(&raw_signal[..9])?;
        let intensity = self.total_intensity(&features)?;
        let mut normalized = features.clone();
        if intensity > 0.0 {
            normalized = features.iter().map(|v| v / intensity).collect();
        }
        normalized.insert(0, self.frame_count as f64);
        self.frame_count += 1;
        Ok(normalized)
    }

    fn validate(&self) -> PerceptionResult<()> {
        if self.resolution.0 == 0 || self.resolution.1 == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "resolution must be positive".into(),
            ));
        }
        if self.frame_rate <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "frame rate must be positive".into(),
            ));
        }
        Ok(())
    }

    fn block_size(&self) -> usize {
        self.resolution.0 * self.resolution.1
    }

    fn active(&self) -> bool {
        self.is_active
    }

    fn reset(&mut self) {
        self.frame_count = 0;
        self.filter_kernel = vec![0.0; 9];
    }
}
