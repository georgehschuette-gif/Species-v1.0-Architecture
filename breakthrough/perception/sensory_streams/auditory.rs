// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// AuditoryStream: Sound-based sensory input channel.
///
/// Processes raw audio sample buffers into band-energy features.
pub struct AuditoryStream {
    pub sample_rate: f64,
    pub channels: usize,
    pub is_active: bool,
    pub buffer_window: Vec<f64>,
    pub frame_count: u64,
}

impl AuditoryStream {
    /// Construct a new AuditoryStream.
    pub fn new(sample_rate: f64, channels: usize) -> PerceptionResult<Self> {
        if sample_rate <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "sample rate must be positive".into(),
            ));
        }
        if channels == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "channel count must be positive".into(),
            ));
        }
        Ok(Self {
            sample_rate,
            channels,
            is_active: true,
            buffer_window: Vec::new(),
            frame_count: 0,
        })
    }

    /// Compute energy in a sub-band of the signal.
    pub fn band_energy(&self, signal: &[f64], start: usize, end: usize) -> PerceptionResult<f64> {
        if end > signal.len() {
            return Err(PerceptionError::OutOfBounds(format!(
                "band end {} exceeds signal length {}",
                end,
                signal.len()
            )));
        }
        if start >= end {
            return Err(PerceptionError::InvalidConfiguration(
                "band start must be less than end".into(),
            ));
        }
        let sum_sq: f64 = signal[start..end].iter().map(|v| v * v).sum();
        Ok(sum_sq / (end - start) as f64)
    }

    /// Compute zero-crossing rate.
    pub fn zero_crossing_rate(&self, signal: &[f64]) -> PerceptionResult<f64> {
        if signal.len() < 2 {
            return Err(PerceptionError::InsufficientData);
        }
        let mut crossings = 0;
        for i in 1..signal.len() {
            if (signal[i] >= 0.0) != (signal[i - 1] >= 0.0) {
                crossings += 1;
            }
        }
        Ok(crossings as f64 / signal.len() as f64)
    }
}

impl SensoryStream for AuditoryStream {
    fn configure(&mut self, config: StreamConfig) -> PerceptionResult<()> {
        config.validate()?;
        if config.channel != SensoryChannel::Auditory {
            return Err(PerceptionError::InvalidConfiguration(
                "stream channel mismatch for auditory".into(),
            ));
        }
        self.sample_rate = config.sample_rate;
        self.channels = 1.max(config.resolution);
        self.is_active = config.enable_filtering;
        Ok(())
    }

    fn ingest(&mut self, raw_signal: &[f64]) -> PerceptionResult<Vec<f64>> {
        if !self.is_active {
            return Err(PerceptionError::ComputationError(
                "auditory stream is inactive".into(),
            ));
        }
        if raw_signal.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        let samples_per_channel = raw_signal.len() / self.channels;
        if samples_per_channel == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "signal length too small for channel count".into(),
            ));
        }
        let mut features = Vec::new();
        for ch in 0..self.channels {
            let start = ch * samples_per_channel;
            let end = start + samples_per_channel.min(raw_signal.len());
            let channel_signal = &raw_signal[start..end];
            let band1 = self.band_energy(channel_signal, 0, channel_signal.len() / 4)?;
            let band2 = self.band_energy(channel_signal, channel_signal.len() / 4, channel_signal.len() / 2)?;
            let band3 = self.band_energy(channel_signal, channel_signal.len() / 2, 3 * channel_signal.len() / 4)?;
            let band4 = self.band_energy(channel_signal, 3 * channel_signal.len() / 4, channel_signal.len())?;
            let zcr = self.zero_crossing_rate(channel_signal)?;
            features.extend_from_slice(&[band1, band2, band3, band4, zcr]);
        }
        let rms: f64 = raw_signal.iter().map(|v| v * v).sum::<f64>().sqrt();
        features.insert(0, rms);
        features.insert(1, self.frame_count as f64);
        self.frame_count += 1;
        Ok(features)
    }

    fn validate(&self) -> PerceptionResult<()> {
        if self.sample_rate <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "auditory sample rate must be positive".into(),
            ));
        }
        if self.channels == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "auditory channel count must be positive".into(),
            ));
        }
        Ok(())
    }

    fn block_size(&self) -> usize {
        self.channels * 64
    }

    fn active(&self) -> bool {
        self.is_active
    }

    fn reset(&mut self) {
        self.frame_count = 0;
        self.buffer_window.clear();
    }
}
