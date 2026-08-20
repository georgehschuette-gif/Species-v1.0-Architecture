# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

$ErrorActionPreference = 'Stop'
$root = 'C:\Users\Administrator\AppData\Local\Programs\Windsurf\breakthrough'

function New-RustFile {
    param([string]$Path, [string]$Content)
    New-Item -ItemType File -Path $Path -Force | Out-Null
    Set-Content -Path $Path -Value $Content -NoNewline
}

# ============================================================================
# MORPHOGENESIS - continued
# ============================================================================

# --- morphogenesis/bifurcation/period_doubling.rs ---
New-RustFile -Path "$root\morphogenesis\bifurcation\period_doubling.rs" -Value @'
use std::fmt;
use super::*;

pub struct PeriodDoubling {
    pub parameter: f64,
    pub period: usize,
    pub bifurcation_points: Vec<f64>,
}

impl PeriodDoubling {
    pub fn new(parameter: f64) -> Self {
        Self { parameter, period: 1, bifurcation_points: vec![parameter] }
    }

    pub fn parameter(&self) -> f64 { self.parameter }
    pub fn period(&self) -> usize { self.period }
    pub fn double(&mut self, new_parameter: f64) -> Result<(), MorphogenesisError> {
        if self.period >= 16 { return Err(MorphogenesisError::CapacityExceeded { max: 16, attempted: self.period + 1 }); }
        self.period *= 2;
        self.parameter = new_parameter;
        self.bifurcation_points.push(new_parameter);
        Ok(())
    }
    pub fn bifurcation_count(&self) -> usize { self.bifurcation_points.len() }
    pub fn is_chaotic(&self) -> bool { self.period >= 8 }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if self.period == 0 { return Err(MorphogenesisError::OutOfRange { field: "period".into(), value: 0.0, min: 1.0, max: 16.0 }); }
        Ok(())
    }
}

impl fmt::Debug for PeriodDoubling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PeriodDoubling").field("period", &self.period).field("bifurcations", &self.bifurcation_points.len()).finish()
    }
}
'@

# --- morphogenesis/collapse ---
New-RustFile -Path "$root\morphogenesis\collapse\mod.rs" -Value @'
pub mod structural_collapse;
pub mod energy_collapse;
pub mod information_collapse;

pub use structural_collapse::StructuralCollapse;
pub use energy_collapse::EnergyCollapse;
pub use information_collapse::InformationCollapse;

pub const DEFAULT_COLLAPSE_THRESHOLD: f64 = 0.8;
pub const MAX_COLLAPSE_MAGNITUDE: f64 = 1.0;

pub fn trigger_structural_collapse(strain: f64) -> Result<StructuralCollapse, MorphogenesisError> {
    StructuralCollapse::new(strain)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn structural_collapse_creation() {
        let c = trigger_structural_collapse(0.9).unwrap();
        assert!(c.strain() > 0.0);
    }
}
'@

New-RustFile -Path "$root\morphogenesis\collapse\structural_collapse.rs" -Value @'
use std::fmt;
use super::*;

pub struct StructuralCollapse {
    pub strain: f64,
    pub integrity: f64,
    pub collapse_type: CollapseType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CollapseType {
    Brittle,
    Ductile,
    Catastrophic,
    Partial,
}

impl StructuralCollapse {
    pub fn new(strain: f64) -> Result<Self, MorphogenesisError> {
        if !(0.0..=1.0).contains(&strain) { return Err(MorphogenesisError::OutOfRange { field: "strain".into(), value: strain, min: 0.0, max: 1.0 }); }
        let collapse_type = match strain {
            s if s > 0.9 => CollapseType::Catastrophic,
            s if s > 0.7 => CollapseType::Brittle,
            s if s > 0.4 => CollapseType::Ductile,
            _ => CollapseType::Partial,
        };
        Ok(Self { strain, integrity: 1.0 - strain, collapse_type })
    }

    pub fn strain(&self) -> f64 { self.strain }
    pub fn propagate(&mut self, additional_strain: f64) -> Result<(), MorphogenesisError> {
        self.strain = (self.strain + additional_strain).min(1.0);
        self.integrity = 1.0 - self.strain;
        Ok(())
    }
    pub fn is_collapsed(&self) -> bool { self.strain >= 0.8 }
    pub fn collapse_type(&self) -> CollapseType { self.collapse_type }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&self.strain) { return Err(MorphogenesisError::OutOfRange { field: "strain".into(), value: self.strain, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for StructuralCollapse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StructuralCollapse").field("strain", &self.strain).field("integrity", &self.integrity).field("type", &self.collapse_type).finish()
    }
}
'@

New-RustFile -Path "$root\morphogenesis\collapse\energy_collapse.rs" -Value @'
use std::fmt;
use super::*;

pub struct EnergyCollapse {
    pub energy_level: f64,
    pub dissipation_rate: f64,
    pub critical_threshold: f64,
}

impl EnergyCollapse {
    pub fn new(energy_level: f64) -> Result<Self, MorphogenesisError> {
        if !(0.0..=1.0).contains(&energy_level) { return Err(MorphogenesisError::OutOfRange { field: "energy_level".into(), value: energy_level, min: 0.0, max: 1.0 }); }
        Ok(Self { energy_level, dissipation_rate: 0.1, critical_threshold: 0.2 })
    }

    pub fn dissipate(&mut self, dt: f64) -> Result<(), MorphogenesisError> {
        self.energy_level = (self.energy_level - self.dissipation_rate * dt).max(0.0);
        Ok(())
    }

    pub fn is_depleted(&self) -> bool { self.energy_level <= self.critical_threshold }
    pub fn inject(&mut self, amount: f64) -> Result<(), MorphogenesisError> {
        self.energy_level = (self.energy_level + amount).min(1.0);
        Ok(())
    }
    pub fn set_dissipation_rate(&mut self, rate: f64) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&rate) { return Err(MorphogenesisError::OutOfRange { field: "dissipation_rate".into(), value: rate, min: 0.0, max: 1.0 }); }
        self.dissipation_rate = rate;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&self.energy_level) { return Err(MorphogenesisError::OutOfRange { field: "energy_level".into(), value: self.energy_level, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for EnergyCollapse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EnergyCollapse").field("level", &self.energy_level).field("rate", &self.dissipation_rate).finish()
    }
}
'@

New-RustFile -Path "$root\morphogenesis\collapse\information_collapse.rs" -Value @'
use std::fmt;
use super::*;

pub struct InformationCollapse {
    pub information_content: f64,
    pub entropy: f64,
    pub redundancy: f64,
}

impl InformationCollapse {
    pub fn new(information_content: f64) -> Result<Self, MorphogenesisError> {
        if !(0.0..=1.0).contains(&information_content) { return Err(MorphogenesisError::OutOfRange { field: "information_content".into(), value: information_content, min: 0.0, max: 1.0 }); }
        Ok(Self { information_content, entropy: 1.0 - information_content, redundancy: 0.0 })
    }

    pub fn degrade(&mut self, noise: f64) -> Result<(), MorphogenesisError> {
        self.information_content = (self.information_content - noise * 0.1).max(0.0);
        self.entropy = 1.0 - self.information_content;
        Ok(())
    }

    pub fn is_corrupted(&self, threshold: f64) -> bool { self.information_content < threshold }
    pub fn recover(&mut self, source_information: f64) -> Result<(), MorphogenesisError> {
        self.information_content = (self.information_content + source_information * 0.2).min(1.0);
        self.entropy = 1.0 - self.information_content;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&self.information_content) { return Err(MorphogenesisError::OutOfRange { field: "information_content".into(), value: self.information_content, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for InformationCollapse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InformationCollapse").field("content", &self.information_content).field("entropy", &self.entropy).finish()
    }
}
'@

# --- morphogenesis/regeneration ---
New-RustFile -Path "$root\morphogenesis\regeneration\mod.rs" -Value @'
pub mod cellular;
pub mod tissue;
pub mod systemic;

pub use cellular::CellularRegeneration;
pub use tissue::TissueRegeneration;
pub use systemic::SystemicRegeneration;

pub const DEFAULT_REGENERATION_RATE: f64 = 0.2;
pub const MAX_REGENERATION_CAPACITY: f64 = 1.0;

pub fn create_cellular_regeneration(capacity: f64) -> Result<CellularRegeneration, MorphogenesisError> {
    CellularRegeneration::new(capacity)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cellular_regeneration_creation() {
        let r = create_cellular_regeneration(0.8).unwrap();
        assert!(r.capacity() > 0.0);
    }
}
'@

New-RustFile -Path "$root\morphogenesis\regeneration\cellular.rs" -Value @'
use std::fmt;
use super::*;

pub struct CellularRegeneration {
    pub capacity: f64,
    pub proliferation_rate: f64,
    pub differentiation: f64,
}

impl CellularRegeneration {
    pub fn new(capacity: f64) -> Result<Self, MorphogenesisError> {
        if !(0.0..=1.0).contains(&capacity) { return Err(MorphogenesisError::OutOfRange { field: "capacity".into(), value: capacity, min: 0.0, max: 1.0 }); }
        Ok(Self { capacity, proliferation_rate: 0.1, differentiation: 0.0 })
    }

    pub fn capacity(&self) -> f64 { self.capacity }
    pub fn proliferate(&mut self, dt: f64) -> Result<(), MorphogenesisError> {
        self.differentiation = (self.differentiation + self.proliferation_rate * dt).min(1.0);
        Ok(())
    }
    pub fn differentiate(&mut self, lineage: CellLineage) {
        match lineage {
            CellLineage::Stem => self.differentiation *= 0.5,
            CellLineage::Progenitor => self.differentiation *= 0.8,
            CellLineage::Terminal => self.differentiation = 1.0,
        }
    }
    pub fn is_differentiated(&self) -> bool { self.differentiation >= 0.9 }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&self.capacity) { return Err(MorphogenesisError::OutOfRange { field: "capacity".into(), value: self.capacity, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellLineage {
    Stem,
    Progenitor,
    Terminal,
}

impl fmt::Debug for CellularRegeneration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CellularRegeneration").field("capacity", &self.capacity).field("differentiation", &self.differentiation).finish()
    }
}
'@

New-RustFile -Path "$root\morphogenesis\regeneration\tissue.rs" -Value @'
use std::fmt;
use super::*;

pub struct TissueRegeneration {
    pub tissue_type: TissueType,
    pub integrity: f64,
    pub scar_tissue: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TissueType {
    Epithelial,
    Connective,
    Muscle,
    Nervous,
}

impl TissueRegeneration {
    pub fn new(tissue_type: TissueType, integrity: f64) -> Result<Self, MorphogenesisError> {
        if !(0.0..=1.0).contains(&integrity) { return Err(MorphogenesisError::OutOfRange { field: "integrity".into(), value: integrity, min: 0.0, max: 1.0 }); }
        Ok(Self { tissue_type, integrity, scar_tissue: 0.0 })
    }

    pub fn heal(&mut self, rate: f64) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&rate) { return Err(MorphogenesisError::OutOfRange { field: "rate".into(), value: rate, min: 0.0, max: 1.0 }); }
        self.integrity = (self.integrity + rate).min(1.0);
        self.scar_tissue = (self.scar_tissue + rate * 0.1).min(1.0);
        Ok(())
    }

    pub fn is_healed(&self) -> bool { self.integrity >= 0.95 }
    pub fn scar_level(&self) -> f64 { self.scar_tissue }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&self.integrity) { return Err(MorphogenesisError::OutOfRange { field: "integrity".into(), value: self.integrity, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for TissueRegeneration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TissueRegeneration").field("type", &self.tissue_type).field("integrity", &self.integrity).field("scar", &self.scar_tissue).finish()
    }
}
'@

New-RustFile -Path "$root\morphogenesis\regeneration\systemic.rs" -Value @'
use std::fmt;
use super::*;

pub struct SystemicRegeneration {
    pub resources: f64,
    pub mobilization_rate: f64,
    pub recovery_index: f64,
}

impl SystemicRegeneration {
    pub fn new(resources: f64) -> Result<Self, MorphogenesisError> {
        if !(0.0..=1.0).contains(&resources) { return Err(MorphogenesisError::OutOfRange { field: "resources".into(), value: resources, min: 0.0, max: 1.0 }); }
        Ok(Self { resources, mobilization_rate: 0.1, recovery_index: 0.0 })
    }

    pub fn mobilize(&mut self, amount: f64) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&amount) { return Err(MorphogenesisError::OutOfRange { field: "amount".into(), value: amount, min: 0.0, max: 1.0 }); }
        self.resources = (self.resources - amount).max(0.0);
        self.recovery_index = (self.recovery_index + amount * self.mobilization_rate).min(1.0);
        Ok(())
    }

    pub fn recover(&mut self, dt: f64) -> Result<(), MorphogenesisError> {
        self.resources = (self.resources + dt * 0.05).min(1.0);
        Ok(())
    }
    pub fn recovery(&self) -> f64 { self.recovery_index }
    pub fn is_recovered(&self, threshold: f64) -> bool { self.recovery_index >= threshold }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&self.resources) { return Err(MorphogenesisError::OutOfRange { field: "resources".into(), value: self.resources, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for SystemicRegeneration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SystemicRegeneration").field("resources", &self.resources).field("recovery", &self.recovery_index).finish()
    }
}
'@

# --- morphogenesis/evolutionary_cycles ---
New-RustFile -Path "$root\morphogenesis\evolutionary_cycles\mod.rs" -Value @'
pub mod selection;
pub mod drift;
pub mod adaptation;

pub use selection::EvolutionarySelection;
pub use drift::EvolutionaryDrift;
pub use adaptation::EvolutionaryAdaptation;

pub const DEFAULT_SELECTION_PRESSURE: f64 = 0.5;
pub const MAX_FITNESS: f64 = 1.0;

pub fn create_evolutionary_selection(population_size: usize) -> EvolutionarySelection {
    EvolutionarySelection::new(population_size)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn evolutionary_selection_creation() {
        let s = create_evolutionary_selection(100);
        assert_eq!(s.population_size(), 100);
    }
}
'@

New-RustFile -Path "$root\morphogenesis\evolutionary_cycles\selection.rs" -Value @'
use std::fmt;
use super::*;

pub struct EvolutionarySelection {
    pub population_size: usize,
    pub selection_pressure: f64,
    pub fitness_distribution: Vec<f64>,
}

impl EvolutionarySelection {
    pub fn new(population_size: usize) -> Self {
        Self { population_size, selection_pressure: 0.5, fitness_distribution: vec![0.5; population_size] }
    }

    pub fn population_size(&self) -> usize { self.population_size }
    pub fn select(&self, count: usize) -> Result<Vec<usize>, MorphogenesisError> {
        if count > self.population_size { return Err(MorphogenesisError::OutOfRange { field: "count".into(), value: count as f64, min: 0.0, max: self.population_size as f64 }); }
        let mut indices: Vec<usize> = (0..self.population_size).collect();
        indices.sort_by(|&a, &b| self.fitness_distribution[b].partial_cmp(&self.fitness_distribution[a]).unwrap());
        Ok(indices.into_iter().take(count).collect())
    }
    pub fn set_fitness(&mut self, individual: usize, fitness: f64) -> Result<(), MorphogenesisError> {
        if individual >= self.population_size { return Err(MorphogenesisError::OutOfRange { field: "individual".into(), value: individual as f64, min: 0.0, max: (self.population_size - 1) as f64 }); }
        if !(0.0..=1.0).contains(&fitness) { return Err(MorphogenesisError::OutOfRange { field: "fitness".into(), value: fitness, min: 0.0, max: 1.0 }); }
        self.fitness_distribution[individual] = fitness;
        Ok(())
    }
    pub fn mean_fitness(&self) -> f64 {
        if self.fitness_distribution.is_empty() { return 0.0; }
        self.fitness_distribution.iter().sum::<f64>() / self.fitness_distribution.len() as f64
    }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if self.population_size == 0 { return Err(MorphogenesisError::MissingInput("population_size must be > 0".into())); }
        Ok(())
    }
}

impl fmt::Debug for EvolutionarySelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EvolutionarySelection").field("size", &self.population_size).field("mean_fitness", &self.mean_fitness()).finish()
    }
}
'@

New-RustFile -Path "$root\morphogenesis\evolutionary_cycles\drift.rs" -Value @'
use std::fmt;
use super::*;

pub struct EvolutionaryDrift {
    pub allele_frequency: f64,
    pub drift_rate: f64,
    pub population_size: usize,
    pub generation: usize,
}

impl EvolutionaryDrift {
    pub fn new(allele_frequency: f64, population_size: usize) -> Result<Self, MorphogenesisError> {
        if !(0.0..=1.0).contains(&allele_frequency) { return Err(MorphogenesisError::OutOfRange { field: "allele_frequency".into(), value: allele_frequency, min: 0.0, max: 1.0 }); }
        if population_size == 0 { return Err(MorphogenesisError::MissingInput("population_size must be > 0".into())); }
        Ok(Self { allele_frequency, drift_rate: 1.0 / (2.0 * population_size as f64), population_size, generation: 0 })
    }

    pub fn step(&mut self) -> Result<(), MorphogenesisError> {
        let noise = (rand::random::<f64>() - 0.5) * 2.0 * self.drift_rate.sqrt();
        self.allele_frequency = (self.allele_frequency + noise).clamp(0.0, 1.0);
        self.generation += 1;
        Ok(())
    }

    pub fn frequency(&self) -> f64 { self.allele_frequency }
    pub fn generation(&self) -> usize { self.generation }
    pub fn is_fixed(&self) -> bool { self.allele_frequency == 0.0 || self.allele_frequency == 1.0 }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&self.allele_frequency) { return Err(MorphogenesisError::OutOfRange { field: "allele_frequency".into(), value: self.allele_frequency, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for EvolutionaryDrift {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EvolutionaryDrift").field("frequency", &self.allele_frequency).field("generation", &self.generation).finish()
    }
}
'@

New-RustFile -Path "$root\morphogenesis\evolutionary_cycles\adaptation.rs" -Value @'
use std::fmt;
use super::*;

pub struct EvolutionaryAdaptation {
    pub fitness: f64,
    pub adaptation_rate: f64,
    pub environmental_pressure: f64,
    pub generations: usize,
}

impl EvolutionaryAdaptation {
    pub fn new(fitness: f64, adaptation_rate: f64) -> Result<Self, MorphogenesisError> {
        if !(0.0..=1.0).contains(&fitness) { return Err(MorphogenesisError::OutOfRange { field: "fitness".into(), value: fitness, min: 0.0, max: 1.0 }); }
        if !(0.0..=1.0).contains(&adaptation_rate) { return Err(MorphogenesisError::OutOfRange { field: "adaptation_rate".into(), value: adaptation_rate, min: 0.0, max: 1.0 }); }
        Ok(Self { fitness, adaptation_rate, environmental_pressure: 0.5, generations: 0 })
    }

    pub fn adapt(&mut self) -> Result<(), MorphogenesisError> {
        self.fitness = (self.fitness + self.adaptation_rate * self.environmental_pressure * 0.1).min(1.0);
        self.generations += 1;
        Ok(())
    }

    pub fn set_pressure(&mut self, pressure: f64) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&pressure) { return Err(MorphogenesisError::OutOfRange { field: "pressure".into(), value: pressure, min: 0.0, max: 1.0 }); }
        self.environmental_pressure = pressure;
        Ok(())
    }
    pub fn is_adapted(&self, threshold: f64) -> bool { self.fitness >= threshold }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&self.fitness) { return Err(MorphogenesisError::OutOfRange { field: "fitness".into(), value: self.fitness, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for EvolutionaryAdaptation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EvolutionaryAdaptation").field("fitness", &self.fitness).field("generations", &self.generations).finish()
    }
}
'@

Write-Host 'Morphogenesis complete'
