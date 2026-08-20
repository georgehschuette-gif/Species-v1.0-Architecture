// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// LatentCompressor: Encodes data into compact latent representations.
pub struct LatentCompressor {
    pub latent_dim: usize,
    pub encoder: String,
}

impl LatentCompressor {
    /// Create a new latent compressor.
    pub fn new(latent_dim: usize, encoder: String) -> PerceptionResult<Self> {
        if latent_dim == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "latent_dim must be positive".into(),
            ));
        }
        if encoder.is_empty() {
            return Err(PerceptionError::InvalidConfiguration(
                "encoder name must not be empty".into(),
            ));
        }
        Ok(Self {
            latent_dim,
            encoder,
        })
    }

    /// Encode a signal into a latent representation.
    pub fn encode(&self, signal: &[f64]) -> PerceptionResult<Vec<f64>> {
        if signal.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        let mut latent = vec![0.0; self.latent_dim];
        let chunk_size = (signal.len() as f64 / self.latent_dim as f64).max(1.0) as usize;
        for i in 0..self.latent_dim {
            let start = i * chunk_size;
            let end = (start + chunk_size).min(signal.len());
            if start < signal.len() {
                let chunk = &signal[start..end];
                latent[i] = chunk.iter().sum::<f64>() / chunk.len() as f64;
            }
        }
        Ok(latent)
    }

    /// Decode a latent representation back to the original space.
    pub fn decode(&self, latent: &[f64]) -> PerceptionResult<Vec<f64>> {
        if latent.len() != self.latent_dim {
            return Err(PerceptionError::InvalidConfiguration(format!(
                "expected latent dimension {}, got {}",
                self.latent_dim,
                latent.len()
            )));
        }
        let reconstructed_dim = self.latent_dim * 4;
        let mut reconstructed = vec![0.0; reconstructed_dim];
        for i in 0..self.latent_dim {
            let base = i * 4;
            reconstructed[base] = latent[i];
            reconstructed[base + 1] = latent[i] * 0.5;
            reconstructed[base + 2] = latent[i] * 0.25;
            reconstructed[base + 3] = latent[i] * 0.125;
        }
        Ok(reconstructed)
    }

    /// Compute the reconstruction error between original and decoded signals.
    pub fn reconstruction_error(&self, original: &[f64], decoded: &[f64]) -> PerceptionResult<f64> {
        if original.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        let len = original.len().min(decoded.len());
        let mse: f64 = original[..len]
            .iter()
            .zip(decoded[..len].iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            / len as f64;
        Ok(mse.sqrt())
    }

    /// Return the compression ratio as a function of the latent dimension vs original size.
    pub fn effective_ratio(&self, original_size: usize) -> PerceptionResult<f64> {
        if original_size == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "original_size must be positive".into(),
            ));
        }
        Ok(original_size as f64 / self.latent_dim as f64)
    }

    /// Update the latent dimension.
    pub fn set_latent_dim(&mut self, dim: usize) -> PerceptionResult<()> {
        if dim == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "latent_dim must be positive".into(),
            ));
        }
        self.latent_dim = dim;
        Ok(())
    }

    /// Update the encoder name.
    pub fn set_encoder(&mut self, encoder: String) -> PerceptionResult<()> {
        if encoder.is_empty() {
            return Err(PerceptionError::InvalidConfiguration(
                "encoder name must not be empty".into(),
            ));
        }
        self.encoder = encoder;
        Ok(())
    }

    /// Validate the compressor configuration.
    pub fn validate(&self) -> PerceptionResult<()> {
        if self.latent_dim == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "latent_dim must be positive".into(),
            ));
        }
        if self.encoder.is_empty() {
            return Err(PerceptionError::InvalidConfiguration(
                "encoder name must not be empty".into(),
            ));
        }
        Ok(())
    }
}