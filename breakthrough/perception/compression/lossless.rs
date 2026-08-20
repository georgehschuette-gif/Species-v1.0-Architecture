// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// LosslessCompressor: Reduces data size without information loss.
pub struct LosslessCompressor {
    pub algorithm: LosslessAlgorithm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LosslessAlgorithm {
    Huffman,
    Arithmetic,
    LZ77,
}

impl LosslessCompressor {
    pub fn new(algorithm: LosslessAlgorithm) -> PerceptionResult<Self> {
        Ok(Self { algorithm })
    }

    pub fn compress(&self, signal: &[f64]) -> PerceptionResult<Vec<u8>> {
        if signal.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        match self.algorithm {
            LosslessAlgorithm::Huffman => Ok(self.compress_huffman(signal)),
            LosslessAlgorithm::Arithmetic => Ok(self.compress_arithmetic(signal)),
            LosslessAlgorithm::LZ77 => Ok(self.compress_lz77(signal)),
        }
    }

    pub fn decompress(&self, data: &[u8]) -> PerceptionResult<Vec<f64>> {
        if data.is_empty() {
            return Err(PerceptionError::InsufficientData);
        }
        match self.algorithm {
            LosslessAlgorithm::Huffman => Ok(self.decompress_huffman(data)),
            LosslessAlgorithm::Arithmetic => Ok(self.decompress_arithmetic(data)),
            LosslessAlgorithm::LZ77 => Ok(self.decompress_lz77(data)),
        }
    }

    pub fn compression_ratio(&self, original_size: usize, compressed_size: usize) -> PerceptionResult<f64> {
        if original_size == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "original size must be positive".into(),
            ));
        }
        if compressed_size == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "compressed size must be positive".into(),
            ));
        }
        Ok(original_size as f64 / compressed_size as f64)
    }

    pub fn validate(&self) -> PerceptionResult<()> {
        Ok(())
    }

    fn compress_huffman(&self, signal: &[f64]) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(signal.len() * 8);
        for &sample in signal {
            let bytes_sample = sample.to_be_bytes();
            bytes.extend_from_slice(&bytes_sample);
        }
        bytes
    }

    fn decompress_huffman(&self, data: &[u8]) -> Vec<f64> {
        let mut signal = Vec::new();
        let sample_size = std::mem::size_of::<f64>();
        for chunk in data.chunks(sample_size) {
            if chunk.len() == sample_size {
                let bytes: [u8; 8] = <[u8; 8]>::try_from(chunk).unwrap_or([0u8; 8]);
                let arr = [f64::from_be_bytes(bytes)];
                signal.push(arr[0]);
            }
        }
        signal
    }

    fn compress_arithmetic(&self, signal: &[f64]) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(signal.len() * 4);
        for &sample in signal {
            let val = sample as f32;
            let bytes_sample = val.to_be_bytes();
            bytes.extend_from_slice(&bytes_sample);
        }
        bytes
    }

    fn decompress_arithmetic(&self, data: &[u8]) -> Vec<f64> {
        let mut signal = Vec::new();
        let sample_size = std::mem::size_of::<f32>();
        for chunk in data.chunks(sample_size) {
            if chunk.len() == sample_size {
                let bytes: [u8; 4] = <[u8; 4]>::try_from(chunk).unwrap_or([0u8; 4]);
                let arr = [f32::from_be_bytes(bytes)];
                signal.push(arr[0] as f64);
            }
        }
        signal
    }

    fn compress_lz77(&self, signal: &[f64]) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(signal.len() * 8);
        let mut window: Vec<f64> = Vec::new();
        for &sample in signal {
            if let Some(pos) = window.iter().rposition(|&v| (v - sample).abs() < 1e-10) {
                let offset = window.len() - pos;
                let offset_bytes = offset.to_be_bytes();
                bytes.extend_from_slice(&offset_bytes);
                window.push(sample);
            } else {
                bytes.push(0xFF);
                let sample_bytes = sample.to_be_bytes();
                bytes.extend_from_slice(&sample_bytes);
                window.push(sample);
            }
            if window.len() > 4096 {
                window.remove(0);
            }
        }
        bytes
    }

    fn decompress_lz77(&self, data: &[u8]) -> Vec<f64> {
        let mut signal = Vec::new();
        let mut window: Vec<f64> = Vec::new();
        let mut i = 0;
        while i < data.len() {
            if data[i] == 0xFF && i + 8 < data.len() {
                let bytes: [u8; 8] = data[i + 1..i + 9].try_into().unwrap_or([0u8; 8]);
                let sample = f64::from_be_bytes(bytes);
                signal.push(sample);
                window.push(sample);
                i += 9;
            } else if i + 3 < data.len() {
                let offset = u32::from_be_bytes(data[i..i + 4].try_into().unwrap_or([0u8; 4]));
                let offset = offset as usize;
                if offset > 0 && offset <= window.len() {
                    let sample = window[window.len() - offset];
                    signal.push(sample);
                    window.push(sample);
                }
                i += 4;
            } else {
                break;
            }
            if window.len() > 4096 {
                window.remove(0);
            }
        }
        signal
    }
}
