// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// LossyCompressor: Reduces data size with controlled information loss.
pub struct LossyCompressor {
    pub compression_ratio: f64,
    pub quality: f64,
}

impl LossyCompressor {
    /// Create a new lossy compressor with the given ratio and quality.
    pub fn new(compression_ratio: f64, quality: f64) -> PerceptionResult<Self> {
        if !(0.0..=1.0).contains(&compression_ratio) {
            return Err(PerceptionError::InvalidConfiguration(
                "compression_ratio must be in [0, 1]".into(),
            ));
        }
        if !(0.0..=1.0).contains(&quality) {
            return Err(PerceptionError::InvalidConfiguration(
                "quality must be in [0, 1]".into(),
            ));
        }
        Ok(Self {
            compression_ratio,
            quality,
        })
    }

    /// Compress a signal using quantization-based lossy compression.
    pub fn compress(&self, signal: &[f64]) -> PerceptionResult<Vec<u8>> {
        if signal.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        let levels = (self.quality * 255.0).max(1.0).min(255.0) as usize;
        let mut min = f64::MAX;
        let mut max = f64::MIN;
        for &v in signal {
            if v < min { min = v; }
            if v > max { max = v; }
        }
        let range = max - min;
        let divisor = if range == 0.0 { 1.0 } else { range };
        let mut compressed = Vec::with_capacity((signal.len() as f64 * self.compression_ratio.max(0.1)) as usize);
        for &sample in signal {
            let normalized = (sample - min) / divisor;
            let quantized = (normalized * (levels as f64 - 1.0)).round() as u8;
            compressed.push(quantized);
        }
        let mut header = Vec::new();
        header.extend_from_slice(&min.to_be_bytes());
        header.extend_from_slice(&max.to_be_bytes());
        header.push(levels as u8);
        let mut result = header;
        result.extend(compressed);
        Ok(result)
    }

    /// Decompress a lossy-compressed signal.
    pub fn decompress(&self, data: &[u8]) -> PerceptionResult<Vec<f64>> {
        if data.len() < 17 {
            return Err(PerceptionError::InvalidConfiguration(
                "data too short for lossy decompression header".into(),
            ));
        }
        let min_bytes: [u8; 8] = data[..8].try_into().unwrap_or([0u8; 8]);
        let max_bytes: [u8; 8] = data[8..16].try_into().unwrap_or([0u8; 8]);
        let min = f64::from_be_bytes(min_bytes);
        let max = f64::from_be_bytes(max_bytes);
        let levels = data[16] as usize;
        let range = max - min;
        let divisor = if range == 0.0 { 1.0 } else { range };
        let mut signal = Vec::new();
        for &byte in &data[17..] {
            let normalized = byte as f64 / (levels as f64 - 1.0).max(1.0);
            let value = min + normalized * divisor;
            signal.push(value);
        }
        Ok(signal)
    }

    /// Compute the mean squared error between original and compressed signals.
    pub fn mse(&self, original: &[f64], compressed: &[f64]) -> PerceptionResult<f64> {
        if original.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        if original.len() != compressed.len() {
            return Err(PerceptionError::InvalidConfiguration(
                "signal length mismatch".into(),
            ));
        }
        let sum_sq_error: f64 = original
            .iter()
            .zip(compressed.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum();
        Ok(sum_sq_error / original.len() as f64)
    }

    /// Compute the peak signal-to-noise ratio.
    pub fn psnr(&self, original: &[f64], compressed: &[f64]) -> PerceptionResult<f64> {
        let mse = self.mse(original, compressed)?;
        if mse == 0.0 {
            return Ok(f64::INFINITY);
        }
        Ok(10.0 * ((1.0 / mse).log10()))
    }

    /// Update the compression ratio.
    pub fn set_compression_ratio(&mut self, ratio: f64) -> PerceptionResult<()> {
        if !(0.0..=1.0).contains(&ratio) {
            return Err(PerceptionError::InvalidConfiguration(
                "compression_ratio must be in [0, 1]".into(),
            ));
        }
        self.compression_ratio = ratio;
        Ok(())
    }

    /// Update the quality parameter.
    pub fn set_quality(&mut self, quality: f64) -> PerceptionResult<()> {
        if !(0.0..=1.0).contains(&quality) {
            return Err(PerceptionError::InvalidConfiguration(
                "quality must be in [0, 1]".into(),
            ));
        }
        self.quality = quality;
        Ok(())
    }

    /// Validate the compressor configuration.
    pub fn validate(&self) -> PerceptionResult<()> {
        if !(0.0..=1.0).contains(&self.compression_ratio) {
            return Err(PerceptionError::InvalidConfiguration(
                "compression_ratio must be in [0, 1]".into(),
            ));
        }
        if !(0.0..=1.0).contains(&self.quality) {
            return Err(PerceptionError::InvalidConfiguration(
                "quality must be in [0, 1]".into(),
            ));
        }
        Ok(())
    }
}