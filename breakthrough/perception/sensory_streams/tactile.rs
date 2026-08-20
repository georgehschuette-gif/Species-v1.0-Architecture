// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// TactileStream: Touch-based sensory input channel.
///
/// Processes haptic sensor arrays into pressure-gradient features.
pub struct TactileStream {
    pub resolution: usize,
    pub sensitivity: f64,
    pub is_active: bool,
    pub last_reading: Vec<f64>,
    pub frame_count: u64,
}

impl TactileStream {
    /// Construct a new TactileStream.
    pub fn new(resolution: usize, sensitivity: f64) -> PerceptionResult<Self> {
        if resolution == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "tactile resolution must be positive".into(),
            ));
        }
        if sensitivity <= 0.0 || sensitivity > 1.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "sensitivity must be in (0, 1]".into(),
            ));
        }
        Ok(Self {
            resolution,
            sensitivity,
            is_active: true,
            last_reading: vec![0.0; resolution],
            frame_count: 0,
        })
    }

    /// Compute local pressure variance across the sensor array.
    pub fn pressure_variance(&self, signal: &[f64]) -> PerceptionResult<f64> {
        if signal.len() < 2 {
            return Err(PerceptionError::InsufficientData);
        }
        let mean: f64 = signal.iter().sum::<f64>() / signal.len() as f64;
        let var: f64 = signal.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / signal.len() as f64;
        Ok(var * self.sensitivity)
    }

    /// Detect the centroid of active pressure.
    pub fn pressure_centroid(&self, signal: &[f64]) -> PerceptionResult<f64> {
        if signal.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        let sum: f64 = signal.iter().sum();
        if sum == 0.0 {
            return Ok(0.0);
        }
        let weighted: f64 = signal.iter().enumerate().map(|(i, v)| i as f64 * v).sum();
        Ok(weighted / sum)
    }

    /// Scale raw sensor readings by sensitivity and clamp to physical limits.
    pub fn scale_reading(&self, raw: &[f64]) -> PerceptionResult<Vec<f64>> {
        if raw.len() != self.resolution {
            return Err(PerceptionError::InvalidConfiguration(format!(
                "expected {} tactile samples, got {}",
                self.resolution,
                raw.len()
            )));
        }
        Ok(raw.iter().map(|v| (v * self.sensitivity).clamp(0.0, 1.0)).collect())
    }
}

impl SensoryStream for TactileStream {
    fn configure(&mut self, config: StreamConfig) -> PerceptionResult<()> {
        config.validate()?;
        if config.channel != SensoryChannel::Tactile {
            return Err(PerceptionError::InvalidConfiguration(
                "stream channel mismatch for tactile".into(),
            ));
        }
        self.resolution = config.resolution;
        self.sensitivity = 0.5;
        self.is_active = config.enable_filtering;
        Ok(())
    }

    fn ingest(&mut self, raw_signal: &[f64]) -> PerceptionResult<Vec<f64>> {
        if !self.is_active {
            return Err(PerceptionError::ComputationError(
                "tactile stream is inactive".into(),
            ));
        }
        if raw_signal.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        let scaled = if raw_signal.len() >= self.resolution {
            raw_signal[..self.resolution].to_vec()
        } else {
            let mut padded = raw_signal.to_vec();
            padded.extend(std::iter::repeat(0.0).take(self.resolution - raw_signal.len()));
            padded
        };
        let scaled = self.scale_reading(&scaled)?;
        let var = self.pressure_variance(&scaled)?;
        let centroid = self.pressure_centroid(&scaled)?;
        let max_p = scaled.iter().cloned().fold(0.0 / 0.0, f64::max);
        let mut features = scaled.clone();
        features.push(var);
        features.push(centroid);
        features.push(max_p);
        features.insert(0, self.frame_count as f64);
        self.last_reading = scaled;
        self.frame_count += 1;
        Ok(features)
    }

    fn validate(&self) -> PerceptionResult<()> {
        if self.resolution == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "tactile resolution must be positive".into(),
            ));
        }
        if self.sensitivity <= 0.0 || self.sensitivity > 1.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "sensitivity must be in (0, 1]".into(),
            ));
        }
        Ok(())
    }

    fn block_size(&self) -> usize {
        self.resolution + 3
    }

    fn active(&self) -> bool {
        self.is_active
    }

    fn reset(&mut self) {
        self.frame_count = 0;
        self.last_reading = vec![0.0; self.resolution];
    }
}
