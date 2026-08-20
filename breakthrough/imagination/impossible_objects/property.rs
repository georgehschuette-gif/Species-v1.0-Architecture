// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::ImaginationError;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParadoxType {
    Geometrical,
    Temporal,
    Logical,
    Semantic,
    Physical,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParadoxResolution {
    Accepted,
    Suspended,
    Transformed,
    Rejected,
}

pub struct ParadoxicalProperty {
    pub description: String,
    pub intensity: f64,
    pub paradox_type: ParadoxType,
    pub resolution: ParadoxResolution,
    pub entangled_properties: Vec<String>,
}

impl ParadoxicalProperty {
    pub fn new(description: impl Into<String>, intensity: f64) -> Result<Self, ImaginationError> {
        if !(0.0..=1.0).contains(&intensity) {
            return Err(ImaginationError::OutOfRange { field: "intensity".into(), value: intensity, min: 0.0, max: 1.0 });
        }
        let desc_str = description.into();
        let paradox_type = Self::infer_type(&desc_str);
        Ok(Self {
            description: desc_str,
            intensity,
            paradox_type,
            resolution: ParadoxResolution::Suspended,
            entangled_properties: Vec::new(),
        })
    }

    pub fn description(&self) -> &str { &self.description }
    pub fn intensity(&self) -> f64 { self.intensity }
    pub fn paradox_type(&self) -> ParadoxType { self.paradox_type }
    pub fn resolution(&self) -> ParadoxResolution { self.resolution }
    pub fn entangled_properties(&self) -> &[String] { &self.entangled_properties }

    pub fn entangle(&mut self, property: impl Into<String>) {
        self.entangled_properties.push(property.into());
    }

    pub fn resolve(&mut self, resolution: ParadoxResolution) -> Result<(), ImaginationError> {
        if matches!(resolution, ParadoxResolution::Accepted) && self.intensity > 0.8 {
            return Err(ImaginationError::ParadoxViolation { description: "cannot accept high-intensity paradox".into() });
        }
        self.resolution = resolution;
        Ok(())
    }

    pub fn attenuate(&mut self, amount: f64) -> Result<(), ImaginationError> {
        if !(0.0..=1.0).contains(&amount) {
            return Err(ImaginationError::OutOfRange { field: "amount".into(), value: amount, min: 0.0, max: 1.0 });
        }
        self.intensity = (self.intensity - amount).max(0.0);
        if self.intensity < 0.2 && matches!(self.resolution, ParadoxResolution::Suspended) {
            self.resolution = ParadoxResolution::Transformed;
        }
        Ok(())
    }

    pub fn is_resolvable(&self) -> bool {
        self.intensity < 0.7 || !self.entangled_properties.is_empty()
    }

    pub fn validate(&self) -> Result<(), ImaginationError> {
        if !(0.0..=1.0).contains(&self.intensity) {
            return Err(ImaginationError::OutOfRange { field: "intensity".into(), value: self.intensity, min: 0.0, max: 1.0 });
        }
        Ok(())
    }

    fn infer_type(description: &str) -> ParadoxType {
        let lower = description.to_lowercase();
        if lower.contains("triangle") || lower.contains("cube") || lower.contains("sphere") {
            ParadoxType::Geometrical
        } else if lower.contains("time") || lower.contains("future") || lower.contains("past") {
            ParadoxType::Temporal
        } else if lower.contains("true") || lower.contains("false") || lower.contains("lie") {
            ParadoxType::Logical
        } else if lower.contains("word") || lower.contains("meaning") || lower.contains("name") {
            ParadoxType::Semantic
        } else {
            ParadoxType::Physical
        }
    }
}

impl fmt::Display for ParadoxicalProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParadoxicalProperty({}, intensity={:.2}, type={:?})", self.description, self.intensity, self.paradox_type)
    }
}

impl fmt::Display for ParadoxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Geometrical => write!(f, "Geometrical"),
            Self::Temporal => write!(f, "Temporal"),
            Self::Logical => write!(f, "Logical"),
            Self::Semantic => write!(f, "Semantic"),
            Self::Physical => write!(f, "Physical"),
        }
    }
}
