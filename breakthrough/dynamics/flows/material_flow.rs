// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MaterialType {
    Fluid,
    Granular,
    Powder,
    Solid,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlowRegime {
    Laminar,
    Turbulent,
    Transitional,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MaterialFlow {
    pub material: MaterialType,
    pub flow_rate: f64,
    pub regime: FlowRegime,
    pub density: f64,
}

impl MaterialFlow {
    pub fn new(material: MaterialType, flow_rate: f64, density: f64) -> Result<Self, crate::DynamicsError> {
        if flow_rate < 0.0 {
            return Err(crate::DynamicsError::InvalidFlowRate(flow_rate));
        }
        let regime = if flow_rate < 0.1 {
            FlowRegime::Laminar
        } else if flow_rate < 1.0 {
            FlowRegime::Transitional
        } else {
            FlowRegime::Turbulent
        };
        Ok(Self { material, flow_rate, regime, density })
    }

    pub fn reynolds_number(&self, characteristic_length: f64, viscosity: f64) -> f64 {
        (self.flow_rate * characteristic_length) / viscosity
    }
}

impl fmt::Display for MaterialFlow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MaterialFlow({:?}, rate={:.2}, density={:.2})", self.material, self.flow_rate, self.density)
    }
}

