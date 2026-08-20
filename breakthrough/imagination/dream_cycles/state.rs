// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::ImaginationError;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DreamPhase {
    Awake,
    Drowsy,
    REM,
    LightSleep,
    DeepSleep,
    Lucid,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DreamSymbol {
    Water,
    Flight,
    Falling,
    Chase,
    Transformation,
    Mirror,
    Door,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolicElement {
    pub symbol: DreamSymbol,
    pub intensity: f64,
    pub emotional_valence: f64,
}

pub struct DreamState {
    pub phase: DreamPhase,
    pub vividness: f64,
    pub emotional_intensity: f64,
    pub symbolic_elements: Vec<SymbolicElement>,
    pub lucidity: f64,
    pub duration_cycles: usize,
}

impl DreamState {
    pub fn new(phase: DreamPhase) -> Self {
        Self {
            phase,
            vividness: 0.5,
            emotional_intensity: 0.3,
            symbolic_elements: Vec::new(),
            lucidity: 0.0,
            duration_cycles: 1,
        }
    }

    pub fn phase(&self) -> DreamPhase { self.phase }
    pub fn vividness(&self) -> f64 { self.vividness }
    pub fn emotional_intensity(&self) -> f64 { self.emotional_intensity }
    pub fn symbolic_elements(&self) -> &[SymbolicElement] { &self.symbolic_elements }
    pub fn lucidity(&self) -> f64 { self.lucidity }
    pub fn duration_cycles(&self) -> usize { self.duration_cycles }

    pub fn transition(&mut self, new_phase: DreamPhase) {
        self.phase = new_phase;
        match new_phase {
            DreamPhase::REM => { self.vividness = (self.vividness + 0.3).clamp(0.0, 1.0); self.lucidity = 0.1; }
            DreamPhase::Lucid => { self.lucidity = 1.0; self.vividness = 1.0; }
            DreamPhase::DeepSleep => { self.vividness = (self.vividness * 0.3).max(0.0); self.emotional_intensity *= 0.5; }
            DreamPhase::Awake => { self.vividness = 0.0; self.lucidity = 0.0; }
            _ => {}
        }
        self.duration_cycles += 1;
    }

    pub fn add_symbol(&mut self, symbol: DreamSymbol, intensity: f64) -> Result<(), ImaginationError> {
        if !(0.0..=1.0).contains(&intensity) {
            return Err(ImaginationError::OutOfRange { field: "intensity".into(), value: intensity, min: 0.0, max: 1.0 });
        }
        let emotional_valence = match symbol {
            DreamSymbol::Flight | DreamSymbol::Mirror => 0.8,
            DreamSymbol::Falling | DreamSymbol::Chase => -0.6,
            DreamSymbol::Transformation | DreamSymbol::Door => 0.4,
            DreamSymbol::Water => 0.2,
            DreamSymbol::Unknown => 0.0,
        };
        self.symbolic_elements.push(SymbolicElement { symbol, intensity, emotional_valence });
        self.emotional_intensity = (self.emotional_intensity + emotional_valence * intensity * 0.1).clamp(-1.0, 1.0);
        self.vividness = (self.vividness + intensity * 0.05).clamp(0.0, 1.0);
        Ok(())
    }

    pub fn induce_lucidity(&mut self, effort: f64) -> Result<(), ImaginationError> {
        if !(0.0..=1.0).contains(&effort) {
            return Err(ImaginationError::OutOfRange { field: "effort".into(), value: effort, min: 0.0, max: 1.0 });
        }
        self.lucidity = (self.lucidity + effort * 0.5).clamp(0.0, 1.0);
        if self.lucidity > 0.8 {
            self.phase = DreamPhase::Lucid;
            self.vividness = 1.0;
        }
        Ok(())
    }

    pub fn dominant_emotion(&self) -> f64 { self.emotional_intensity }
    pub fn is_lucid(&self) -> bool { matches!(self.phase, DreamPhase::Lucid) || self.lucidity > 0.7 }
    pub fn validate(&self) -> Result<(), ImaginationError> {
        if !(0.0..=1.0).contains(&self.vividness) {
            return Err(ImaginationError::OutOfRange { field: "vividness".into(), value: self.vividness, min: 0.0, max: 1.0 });
        }
        if !(-1.0..=1.0).contains(&self.emotional_intensity) {
            return Err(ImaginationError::OutOfRange { field: "emotional_intensity".into(), value: self.emotional_intensity, min: -1.0, max: 1.0 });
        }
        Ok(())
    }
}

impl fmt::Display for DreamState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DreamState(phase={:?}, vividness={:.2}, emotion={:.2}, lucidity={:.2})", self.phase, self.vividness, self.emotional_intensity, self.lucidity)
    }
}

impl fmt::Display for DreamPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Awake => write!(f, "Awake"),
            Self::Drowsy => write!(f, "Drowsy"),
            Self::REM => write!(f, "REM"),
            Self::LightSleep => write!(f, "LightSleep"),
            Self::DeepSleep => write!(f, "DeepSleep"),
            Self::Lucid => write!(f, "Lucid"),
        }
    }
}
