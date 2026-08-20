// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::ImaginationError;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SourceType {
    Episodic,
    Semantic,
    Procedural,
    Emotional,
    Imaginative,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemorySource {
    pub id: usize,
    pub source_type: SourceType,
    pub weight: f64,
    pub fidelity: f64,
}

pub struct MemoryFabrication {
    pub sources: Vec<MemorySource>,
    pub blended_content: Vec<String>,
    pub fabrication_confidence: f64,
    pub source_count: usize,
    pub interpolation_factor: f64,
}

impl MemoryFabrication {
    pub fn new(source_count: usize) -> Self {
        Self {
            sources: Vec::new(),
            blended_content: Vec::new(),
            fabrication_confidence: 0.0,
            source_count,
            interpolation_factor: 0.5,
        }
    }

    pub fn sources(&self) -> &[MemorySource] { &self.sources }
    pub fn blended_content(&self) -> &[String] { &self.blended_content }
    pub fn fabrication_confidence(&self) -> f64 { self.fabrication_confidence }
    pub fn source_count(&self) -> usize { self.source_count }
    pub fn interpolation_factor(&self) -> f64 { self.interpolation_factor }

    pub fn add_source(&mut self, source_type: SourceType, weight: f64, fidelity: f64) -> Result<usize, ImaginationError> {
        if !(0.0..=1.0).contains(&weight) {
            return Err(ImaginationError::OutOfRange { field: "weight".into(), value: weight, min: 0.0, max: 1.0 });
        }
        if !(0.0..=1.0).contains(&fidelity) {
            return Err(ImaginationError::OutOfRange { field: "fidelity".into(), value: fidelity, min: 0.0, max: 1.0 });
        }
        if self.sources.len() >= self.source_count {
            return Err(ImaginationError::CapacityExceeded { max: self.source_count, attempted: self.sources.len() + 1 });
        }
        let id = self.sources.len();
        self.sources.push(MemorySource { id, source_type, weight, fidelity });
        Ok(id)
    }

    pub fn set_interpolation_factor(&mut self, factor: f64) -> Result<(), ImaginationError> {
        if !(0.0..=1.0).contains(&factor) {
            return Err(ImaginationError::OutOfRange { field: "interpolation_factor".into(), value: factor, min: 0.0, max: 1.0 });
        }
        self.interpolation_factor = factor;
        Ok(())
    }

    pub fn fabricate(&mut self) -> Result<(), ImaginationError> {
        if self.sources.is_empty() {
            return Err(ImaginationError::MissingInput("no sources provided for fabrication".into()));
        }
        self.blended_content.clear();
        let total_weight: f64 = self.sources.iter().map(|s| s.weight).sum::<f64>();
        for source in &self.sources {
            let contribution = (source.weight / total_weight) * self.interpolation_factor;
            let fragment = format!("fragment_{}_{:.2}", source.id, contribution);
            self.blended_content.push(fragment);
        }
        let avg_fidelity: f64 = self.sources.iter().map(|s| s.fidelity * s.weight).sum::<f64>() / total_weight;
        self.fabrication_confidence = avg_fidelity.clamp(0.0, 1.0);
        Ok(())
    }

    pub fn dominant_source(&self) -> Option<&MemorySource> {
        self.sources.iter().max_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap())
    }

    pub fn confidence_by_type(&self, source_type: SourceType) -> f64 {
        let matching: Vec<&MemorySource> = self.sources.iter().filter(|s| s.source_type == source_type).collect();
        if matching.is_empty() { return 0.0; }
        let total_weight: f64 = matching.iter().map(|s| s.weight).sum::<f64>();
        matching.iter().map(|s| s.fidelity * s.weight).sum::<f64>() / total_weight
    }

    pub fn validate(&self) -> Result<(), ImaginationError> {
        if self.sources.is_empty() {
            return Err(ImaginationError::MissingInput("no sources defined".into()));
        }
        Ok(())
    }
}

impl fmt::Display for MemoryFabrication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MemoryFabrication(sources={}, confidence={:.2})", self.sources.len(), self.fabrication_confidence)
    }
}

impl fmt::Display for SourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Episodic => write!(f, "Episodic"),
            Self::Semantic => write!(f, "Semantic"),
            Self::Procedural => write!(f, "Procedural"),
            Self::Emotional => write!(f, "Emotional"),
            Self::Imaginative => write!(f, "Imaginative"),
        }
    }
}
