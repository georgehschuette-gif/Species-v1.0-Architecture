// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::ImaginationError;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConstructionPhase {
    Conceptualized,
    Structuralized,
    ParadoxEmbedded,
    Stabilized,
    Dissolved,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VisualizationMode {
    Perspective,
    Isometric,
    Orthographic,
    Projective,
    None,
}

pub struct ImpossibleObject {
    pub name: String,
    pub properties: Vec<ParadoxicalProperty>,
    pub phase: ConstructionPhase,
    pub visualization: VisualizationMode,
    pub coherence: f64,
    pub construction_effort: f64,
}

impl ImpossibleObject {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            properties: Vec::new(),
            phase: ConstructionPhase::Conceptualized,
            visualization: VisualizationMode::None,
            coherence: DEFAULT_OBJECT_COHERENCE,
            construction_effort: 0.0,
        }
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn properties(&self) -> &[ParadoxicalProperty] { &self.properties }
    pub fn phase(&self) -> ConstructionPhase { self.phase }
    pub fn visualization(&self) -> VisualizationMode { self.visualization }
    pub fn coherence(&self) -> f64 { self.coherence }
    pub fn construction_effort(&self) -> f64 { self.construction_effort }

    pub fn add_property(&mut self, property: ParadoxicalProperty) -> Result<(), ImaginationError> {
        property.validate()?;
        self.properties.push(property);
        self.recalculate_coherence();
        self.construction_effort += 0.1;
        Ok(())
    }

    pub fn set_visualization(&mut self, mode: VisualizationMode) {
        self.visualization = mode;
        self.construction_effort += 0.05;
    }

    pub fn advance_phase(&mut self) -> Result<(), ImaginationError> {
        self.phase = match self.phase {
            ConstructionPhase::Conceptualized => ConstructionPhase::Structuralized,
            ConstructionPhase::Structuralized => {
                if self.properties.is_empty() {
                    return Err(ImaginationError::MissingInput("no properties defined for structuralization".into()));
                }
                ConstructionPhase::ParadoxEmbedded
            }
            ConstructionPhase::ParadoxEmbedded => ConstructionPhase::Stabilized,
            ConstructionPhase::Stabilized => ConstructionPhase::Dissolved,
            ConstructionPhase::Dissolved => ConstructionPhase::Dissolved,
        };
        self.construction_effort += 0.2;
        Ok(())
    }

    pub fn dissolve(&mut self) {
        self.phase = ConstructionPhase::Dissolved;
        self.coherence = 0.0;
        self.properties.clear();
    }

    pub fn highest_intensity_property(&self) -> Option<&ParadoxicalProperty> {
        self.properties.iter().max_by(|a, b| a.intensity.partial_cmp(&b.intensity).unwrap())
    }

    pub fn can_stabilize(&self) -> bool {
        self.properties.iter().all(|p| p.is_resolvable())
    }

    pub fn recalculate_coherence(&mut self) {
        if self.properties.is_empty() {
            self.coherence = DEFAULT_OBJECT_COHERENCE;
            return;
        }
        let avg_intensity: f64 = self.properties.iter().map(|p| p.intensity).sum::<f64>() / self.properties.len() as f64;
        self.coherence = (1.0 - avg_intensity).clamp(0.0, 1.0);
        if self.coherence < 0.3 {
            self.phase = ConstructionPhase::ParadoxEmbedded;
        }
    }

    pub fn validate(&self) -> Result<(), ImaginationError> {
        if self.name.is_empty() {
            return Err(ImaginationError::InvalidInput("object name cannot be empty".into()));
        }
        for prop in &self.properties {
            prop.validate()?;
        }
        Ok(())
    }
}

impl fmt::Display for ImpossibleObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ImpossibleObject(name='{}', phase={:?}, coherence={:.2})", self.name, self.phase, self.coherence)
    }
}

impl fmt::Display for ConstructionPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Conceptualized => write!(f, "Conceptualized"),
            Self::Structuralized => write!(f, "Structuralized"),
            Self::ParadoxEmbedded => write!(f, "ParadoxEmbedded"),
            Self::Stabilized => write!(f, "Stabilized"),
            Self::Dissolved => write!(f, "Dissolved"),
        }
    }
}

impl fmt::Display for VisualizationMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Perspective => write!(f, "Perspective"),
            Self::Isometric => write!(f, "Isometric"),
            Self::Orthographic => write!(f, "Orthographic"),
            Self::Projective => write!(f, "Projective"),
            Self::None => write!(f, "None"),
        }
    }
}
