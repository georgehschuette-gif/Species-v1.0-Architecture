// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// ProprioceptiveStream: Internal state sensory input channel.
///
/// Monitors joint angles, limb velocities, body configuration,
/// and self-motion cues.
pub struct ProprioceptiveStream {
    pub dimensions: usize,
    pub update_rate: f64,
    pub is_active: bool,
    pub history: Vec<Vec<f64>>,
    pub frame_count: u64,
}

impl ProprioceptiveStream {
    /// Construct a new ProprioceptiveStream.
    pub fn new(dimensions: usize, update_rate: f64) -> PerceptionResult<Self> {
        if dimensions == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "proprioceptive dimensions must be positive".into(),
            ));
        }
        if update_rate <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "update rate must be positive".into(),
            ));
        }
        Ok(Self {
            dimensions,
            update_rate,
            is_active: true,
            history: Vec::new(),
            frame_count: 0,
        })
    }

    /// Compute velocity from consecutive state vectors.
    pub fn estimate_velocity(&self, current: &[f64], previous: &[f64]) -> PerceptionResult<Vec<f64>> {
        if current.len() != previous.len() || current.len() != self.dimensions {
            return Err(PerceptionError::InvalidConfiguration(
                "state dimension mismatch".into(),
            ));
        }
        let dt = 1.0 / self.update_rate;
        Ok(current
            .iter()
            .zip(previous.iter())
            .map(|(c, p)| (c - p) / dt)
            .collect())
    }

    /// Compute kinetic energy of the current state.
    pub fn kinetic_energy(&self, state: &[f64], velocity: &[f64]) -> PerceptionResult<f64> {
        if state.len() != velocity.len() || state.len() != self.dimensions {
            return Err(PerceptionError::InvalidConfiguration(
                "state dimension mismatch".into(),
            ));
        }
        let ke: f64 = state
            .iter()
            .zip(velocity.iter())
            .map(|(s, v)| 0.5 * s * v * v)
            .sum();
        Ok(ke)
    }

    /// Detect if a joint angle exceeds its physical limit.
    pub fn is_within_limits(&self, angles: &[f64], limits: &[f64]) -> PerceptionResult<bool> {
        if angles.len() != limits.len() || angles.len() != self.dimensions {
            return Err(PerceptionError::InvalidConfiguration(
                "dimension mismatch between angles and limits".into(),
            ));
        }
        for (a, l) in angles.iter().zip(limits.iter()) {
            if a.abs() > *l {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

impl SensoryStream for ProprioceptiveStream {
    fn configure(&mut self, config: StreamConfig) -> PerceptionResult<()> {
        config.validate()?;
        if config.channel != SensoryChannel::Proprioceptive {
            return Err(PerceptionError::InvalidConfiguration(
                "stream channel mismatch for proprioceptive".into(),
            ));
        }
        self.dimensions = config.resolution.max(1);
        self.update_rate = config.sample_rate;
        self.is_active = config.enable_filtering;
        Ok(())
    }

    fn ingest(&mut self, raw_signal: &[f64]) -> PerceptionResult<Vec<f64>> {
        if !self.is_active {
            return Err(PerceptionError::ComputationError(
                "proprioceptive stream is inactive".into(),
            ));
        }
        if raw_signal.len() != self.dimensions {
            return Err(PerceptionError::InvalidConfiguration(format!(
                "expected {} proprioceptive samples, got {}",
                self.dimensions,
                raw_signal.len()
            )));
        }
        let previous = self.history.last().cloned().unwrap_or_else(|| vec![0.0; self.dimensions]);
        let velocity = self.estimate_velocity(raw_signal, &previous)?;
        let ke = self.kinetic_energy(raw_signal, &velocity)?;
        let limits: Vec<f64> = vec![1.0; self.dimensions];
        let within_limits = self.is_within_limits(raw_signal, &limits)?;
        let mut features = raw_signal.to_vec();
        features.extend(velocity);
        features.push(ke);
        features.push(if within_limits { 1.0 } else { 0.0 });
        features.push(self.frame_count as f64);
        self.history.push(raw_signal.to_vec());
        if self.history.len() > 128 {
            self.history.remove(0);
        }
        self.frame_count += 1;
        Ok(features)
    }

    fn validate(&self) -> PerceptionResult<()> {
        if self.dimensions == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "proprioceptive dimensions must be positive".into(),
            ));
        }
        if self.update_rate <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "proprioceptive update rate must be positive".into(),
            ));
        }
        Ok(())
    }

    fn block_size(&self) -> usize {
        self.dimensions * 2 + 3
    }

    fn active(&self) -> bool {
        self.is_active
    }

    fn reset(&mut self) {
        self.frame_count = 0;
        self.history.clear();
    }
}
