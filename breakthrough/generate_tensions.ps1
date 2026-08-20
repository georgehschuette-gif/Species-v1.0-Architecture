# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

$ErrorActionPreference = 'Stop'
$root = 'C:\Users\Administrator\AppData\Local\Programs\Windsurf\breakthrough'

function New-RustFile {
    param(
        [string]$Path,
        [string]$Content
    )
    New-Item -ItemType File -Path $Path -Force | Out-Null
    Set-Content -Path $Path -Value $Content -NoNewline
}

# ============================================================================
# TENSIONS
# ============================================================================

# --- tensions/contradictions ---
New-RustFile -Path "$root\tensions\contradictions\mod.rs" -Value @'
pub mod tension_field;
pub mod resolution;
pub mod balance;

pub use tension_field::ContradictionField;
pub use resolution::ContradictionResolution;
pub use balance::ContradictionBalance;

/// Default contradiction tension threshold.
pub const DEFAULT_CONTRADICTION_THRESHOLD: f64 = 0.5;
/// Maximum contradiction intensity before forced resolution.
pub const MAX_CONTRADICTION_INTENSITY: f64 = 1.0;

/// Creates a new contradiction field with default parameters.
pub fn create_contradiction_field(strength: f64) -> Result<ContradictionField, TensionsError> {
    ContradictionField::new(strength)
}

/// Resolves a contradiction given its current state.
pub fn resolve_contradiction(state: ContradictionState) -> Result<ResolutionOutcome, TensionsError> {
    ContradictionResolution::resolve(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn contradiction_field_creation() {
        let field = create_contradiction_field(0.7).unwrap();
        assert!(field.strength() > 0.0);
    }
}
'@

New-RustFile -Path "$root\tensions\contradictions\tension_field.rs" -Value @'
use std::fmt;
use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContradictionState {
    Latent,
    Active,
    Escalating,
    Resolved,
}

pub struct ContradictionField {
    pub strength: f64,
    pub polarity: f64,
    pub state: ContradictionState,
}

impl ContradictionField {
    pub fn new(strength: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&strength) {
            return Err(TensionsError::OutOfRange { field: "strength".into(), value: strength, min: 0.0, max: 1.0 });
        }
        Ok(Self { strength, polarity: 0.0, state: ContradictionState::Latent })
    }

    pub fn strength(&self) -> f64 { self.strength }
    pub fn activate(&mut self) { self.state = ContradictionState::Active; }
    pub fn escalate(&mut self, amount: f64) -> Result<(), TensionsError> {
        self.strength = (self.strength + amount).min(1.0);
        self.state = if self.strength > 0.8 { ContradictionState::Escalating } else { ContradictionState::Active };
        Ok(())
    }
    pub fn resolve(&mut self) { self.state = ContradictionState::Resolved; self.strength = 0.0; }
    pub fn is_active(&self) -> bool { matches!(self.state, ContradictionState::Active | ContradictionState::Escalating) }
    pub fn tension(&self) -> f64 { self.strength * self.polarity.abs() }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.strength) { return Err(TensionsError::OutOfRange { field: "strength".into(), value: self.strength, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Display for ContradictionField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Contradiction(strength={:.2}, state={:?})", self.strength, self.state)
    }
}
'@

New-RustFile -Path "$root\tensions\contradictions\resolution.rs" -Value @'
use std::fmt;
use super::*;

pub struct ContradictionResolution {
    pub strategy: ResolutionStrategy,
    pub completeness: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResolutionStrategy {
    Synthesis,
    Supression,
    Integration,
    Transcendence,
}

impl ContradictionResolution {
    pub fn new(strategy: ResolutionStrategy) -> Self {
        Self { strategy, completeness: 0.0 }
    }

    pub fn resolve(state: ContradictionState) -> Result<ResolutionOutcome, TensionsError> {
        match state {
            ContradictionState::Latent => Ok(ResolutionOutcome::NoOp),
            ContradictionState::Active => Ok(ResolutionOutcome::Partial),
            ContradictionState::Escalating => Ok(ResolutionOutcome::Full),
            ContradictionState::Resolved => Ok(ResolutionOutcome::NoOp),
        }
    }

    pub fn apply(&mut self, field: &mut ContradictionField) -> Result<f64, TensionsError> {
        self.completeness = match self.strategy {
            ResolutionStrategy::Synthesis => 0.9,
            ResolutionStrategy::Supression => 0.6,
            ResolutionStrategy::Integration => 0.8,
            ResolutionStrategy::Transcendence => 1.0,
        };
        field.resolve();
        Ok(self.completeness)
    }

    pub fn completeness(&self) -> f64 { self.completeness }
    pub fn set_strategy(&mut self, strategy: ResolutionStrategy) { self.strategy = strategy; }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.completeness) { return Err(TensionsError::OutOfRange { field: "completeness".into(), value: self.completeness, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResolutionOutcome {
    NoOp,
    Partial,
    Full,
}

impl fmt::Display for ResolutionOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { Self::NoOp => write!(f, "NoOp"), Self::Partial => write!(f, "Partial"), Self::Full => write!(f, "Full") }
    }
}
'@

New-RustFile -Path "$root\tensions\contradictions\balance.rs" -Value @'
use std::fmt;
use super::*;

pub struct ContradictionBalance {
    pub positive_tension: f64,
    pub negative_tension: f64,
    pub equilibrium: f64,
}

impl ContradictionBalance {
    pub fn new(positive: f64, negative: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&positive) || !(0.0..=1.0).contains(&negative) {
            return Err(TensionsError::OutOfRange { field: "tensions".into(), value: positive.max(negative), min: 0.0, max: 1.0 });
        }
        Ok(Self { positive_tension: positive, negative_tension: negative, equilibrium: 0.5 })
    }

    pub fn shift(&mut self, delta: f64) -> Result<(), TensionsError> {
        self.positive_tension = (self.positive_tension + delta).clamp(0.0, 1.0);
        self.negative_tension = (self.negative_tension - delta).clamp(0.0, 1.0);
        self.equilibrium = (self.positive_tension + self.negative_tension) / 2.0;
        Ok(())
    }

    pub fn is_balanced(&self, tolerance: f64) -> bool {
        (self.positive_tension - self.negative_tension).abs() <= tolerance
    }

    pub fn net_tension(&self) -> f64 { (self.positive_tension - self.negative_tension).abs() }
    pub fn set_equilibrium(&mut self, eq: f64) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&eq) { return Err(TensionsError::OutOfRange { field: "equilibrium".into(), value: eq, min: 0.0, max: 1.0 }); }
        self.equilibrium = eq;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.positive_tension) { return Err(TensionsError::OutOfRange { field: "positive_tension".into(), value: self.positive_tension, min: 0.0, max: 1.0 }); }
        if !(0.0..=1.0).contains(&self.negative_tension) { return Err(TensionsError::OutOfRange { field: "negative_tension".into(), value: self.negative_tension, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for ContradictionBalance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContradictionBalance").field("positive", &self.positive_tension).field("negative", &self.negative_tension).field("equilibrium", &self.equilibrium).finish()
    }
}
'@

# --- tensions/ambiguity ---
New-RustFile -Path "$root\tensions\ambiguity\mod.rs" -Value @'
pub mod gradient;
pub mod interpretation;
pub mod resolution;

pub use gradient::AmbiguityGradient;
pub use interpretation::AmbiguityInterpretation;
pub use resolution::AmbiguityResolution;

pub const DEFAULT_AMBIGUITY_TOLERANCE: f64 = 0.3;
pub const MAX_AMBIGUITY_LEVEL: f64 = 1.0;

pub fn create_ambiguity_gradient(entropy: f64) -> Result<AmbiguityGradient, TensionsError> {
    AmbiguityGradient::new(entropy)
}

pub fn interpret_ambiguity(signal: f64) -> AmbiguityInterpretation {
    AmbiguityInterpretation::from_signal(signal)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ambiguity_gradient_creation() {
        let g = create_ambiguity_gradient(0.4).unwrap();
        assert!(g.entropy() >= 0.0);
    }
}
'@

New-RustFile -Path "$root\tensions\ambiguity\gradient.rs" -Value @'
use std::fmt;
use super::*;

pub struct AmbiguityGradient {
    pub entropy: f64,
    pub resolution_potential: f64,
}

impl AmbiguityGradient {
    pub fn new(entropy: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&entropy) { return Err(TensionsError::OutOfRange { field: "entropy".into(), value: entropy, min: 0.0, max: 1.0 }); }
        Ok(Self { entropy, resolution_potential: 1.0 - entropy })
    }

    pub fn entropy(&self) -> f64 { self.entropy }
    pub fn steepness(&self) -> f64 { self.resolution_potential }
    pub fn resolve(&mut self, amount: f64) -> Result<(), TensionsError> {
        self.entropy = (self.entropy - amount).max(0.0);
        self.resolution_potential = 1.0 - self.entropy;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.entropy) { return Err(TensionsError::OutOfRange { field: "entropy".into(), value: self.entropy, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for AmbiguityGradient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AmbiguityGradient").field("entropy", &self.entropy).field("resolution_potential", &self.resolution_potential).finish()
    }
}
'@

New-RustFile -Path "$root\tensions\ambiguity\interpretation.rs" -Value @'
use std::fmt;
use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterpretationConfidence {
    Low,
    Medium,
    High,
    Certain,
}

pub struct AmbiguityInterpretation {
    pub signal: f64,
    pub confidence: InterpretationConfidence,
    pub context_weight: f64,
}

impl AmbiguityInterpretation {
    pub fn from_signal(signal: f64) -> Self {
        let confidence = match signal {
            s if s < 0.25 => InterpretationConfidence::Low,
            s if s < 0.5 => InterpretationConfidence::Medium,
            s if s < 0.75 => InterpretationConfidence::High,
            _ => InterpretationConfidence::Certain,
        };
        Self { signal, confidence, context_weight: 1.0 }
    }

    pub fn adjust_confidence(&mut self, evidence: f64) {
        self.signal = (self.signal + evidence * 0.1).clamp(0.0, 1.0);
        self.confidence = match self.signal {
            s if s < 0.25 => InterpretationConfidence::Low,
            s if s < 0.5 => InterpretationConfidence::Medium,
            s if s < 0.75 => InterpretationConfidence::High,
            _ => InterpretationConfidence::Certain,
        };
    }

    pub fn confidence_level(&self) -> InterpretationConfidence { self.confidence }
    pub fn set_context_weight(&mut self, weight: f64) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&weight) { return Err(TensionsError::OutOfRange { field: "context_weight".into(), value: weight, min: 0.0, max: 1.0 }); }
        self.context_weight = weight;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.signal) { return Err(TensionsError::OutOfRange { field: "signal".into(), value: self.signal, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Display for InterpretationConfidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { Self::Low => write!(f, "Low"), Self::Medium => write!(f, "Medium"), Self::High => write!(f, "High"), Self::Certain => write!(f, "Certain") }
    }
}
'@

New-RustFile -Path "$root\tensions\ambiguity\resolution.rs" -Value @'
use std::fmt;
use super::*;

pub struct AmbiguityResolution {
    pub clarity: f64,
    pub residual_ambiguity: f64,
    pub iterations: usize,
}

impl AmbiguityResolution {
    pub fn new() -> Self {
        Self { clarity: 0.0, residual_ambiguity: 1.0, iterations: 0 }
    }

    pub fn iterate(&mut self, gradient: &AmbiguityGradient) -> Result<bool, TensionsError> {
        self.clarity = (self.clarity + gradient.steepness() * 0.1).min(1.0);
        self.residual_ambiguity = 1.0 - self.clarity;
        self.iterations += 1;
        Ok(self.residual_ambiguity < 0.1)
    }

    pub fn clarity(&self) -> f64 { self.clarity }
    pub fn is_resolved(&self) -> bool { self.residual_ambiguity < 0.1 }
    pub fn reset(&mut self) { self.clarity = 0.0; self.residual_ambiguity = 1.0; self.iterations = 0; }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.clarity) { return Err(TensionsError::OutOfRange { field: "clarity".into(), value: self.clarity, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for AmbiguityResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AmbiguityResolution").field("clarity", &self.clarity).field("residual", &self.residual_ambiguity).field("iterations", &self.iterations).finish()
    }
}
'@

# --- tensions/uncertainty ---
New-RustFile -Path "$root\tensions\uncertainty\mod.rs" -Value @'
pub mod measure;
pub mod propagation;
pub mod reduction;

pub use measure::UncertaintyMeasure;
pub use propagation::UncertaintyPropagation;
pub use reduction::UncertaintyReduction;

pub const DEFAULT_UNCERTAINTY_DECAY: f64 = 0.1;
pub const MAX_UNCERTAINTY: f64 = 1.0;

pub fn create_uncertainty_measure(initial: f64) -> Result<UncertaintyMeasure, TensionsError> {
    UncertaintyMeasure::new(initial)
}

pub fn propagate_uncertainty(source: &UncertaintyMeasure) -> UncertaintyPropagation {
    UncertaintyPropagation::from_source(source)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn uncertainty_measure_creation() {
        let m = create_uncertainty_measure(0.3).unwrap();
        assert!(m.value() > 0.0);
    }
}
'@

New-RustFile -Path "$root\tensions\uncertainty\measure.rs" -Value @'
use std::fmt;
use super::*;

pub struct UncertaintyMeasure {
    pub value: f64,
    pub variance: f64,
}

impl UncertaintyMeasure {
    pub fn new(value: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&value) { return Err(TensionsError::OutOfRange { field: "value".into(), value, min: 0.0, max: 1.0 }); }
        Ok(Self { value, variance: value * (1.0 - value) })
    }

    pub fn value(&self) -> f64 { self.value }
    pub fn variance(&self) -> f64 { self.variance }
    pub fn confidence(&self) -> f64 { 1.0 - self.value }
    pub fn update(&mut self, observation: f64, weight: f64) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&weight) { return Err(TensionsError::OutOfRange { field: "weight".into(), value: weight, min: 0.0, max: 1.0 }); }
        self.value = (self.value * (1.0 - weight) + observation * weight).clamp(0.0, 1.0);
        self.variance = self.value * (1.0 - self.value);
        Ok(())
    }
    pub fn decay(&mut self, rate: f64) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&rate) { return Err(TensionsError::OutOfRange { field: "rate".into(), value: rate, min: 0.0, max: 1.0 }); }
        self.value = (self.value * (1.0 - rate)).max(0.0);
        self.variance = self.value * (1.0 - self.value);
        Ok(())
    }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.value) { return Err(TensionsError::OutOfRange { field: "value".into(), value: self.value, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for UncertaintyMeasure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UncertaintyMeasure").field("value", &self.value).field("variance", &self.variance).finish()
    }
}
'@

New-RustFile -Path "$root\tensions\uncertainty\propagation.rs" -Value @'
use std::fmt;
use super::*;

pub struct UncertaintyPropagation {
    pub propagated_value: f64,
    pub accumulated_variance: f64,
    pub depth: usize,
}

impl UncertaintyPropagation {
    pub fn from_source(source: &UncertaintyMeasure) -> Self {
        Self { propagated_value: source.value, accumulated_variance: source.variance, depth: 1 }
    }

    pub fn through(&mut self, noise: f64) -> Result<(), TensionsError> {
        self.accumulated_variance += noise;
        self.propagated_value = (self.propagated_value + noise * 0.1).clamp(0.0, 1.0);
        self.depth += 1;
        Ok(())
    }

    pub fn propagated(&self) -> f64 { self.propagated_value }
    pub fn total_variance(&self) -> f64 { self.accumulated_variance }
    pub fn depth_level(&self) -> usize { self.depth }
    pub fn is_bounded(&self, threshold: f64) -> bool { self.accumulated_variance < threshold }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.propagated_value) { return Err(TensionsError::OutOfRange { field: "propagated_value".into(), value: self.propagated_value, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for UncertaintyPropagation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UncertaintyPropagation").field("value", &self.propagated_value).field("variance", &self.accumulated_variance).field("depth", &self.depth).finish()
    }
}
'@

New-RustFile -Path "$root\tensions\uncertainty\reduction.rs" -Value @'
use std::fmt;
use super::*;

pub struct UncertaintyReduction {
    pub reduction_rate: f64,
    pub current_uncertainty: f64,
    pub steps: usize,
}

impl UncertaintyReduction {
    pub fn new(rate: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&rate) { return Err(TensionsError::OutOfRange { field: "reduction_rate".into(), value: rate, min: 0.0, max: 1.0 }); }
        Ok(Self { reduction_rate: rate, current_uncertainty: 1.0, steps: 0 })
    }

    pub fn step(&mut self) -> Result<f64, TensionsError> {
        self.current_uncertainty = (self.current_uncertainty * (1.0 - self.reduction_rate)).max(0.0);
        self.steps += 1;
        Ok(self.current_uncertainty)
    }

    pub fn uncertainty(&self) -> f64 { self.current_uncertainty }
    pub fn is_certain(&self, threshold: f64) -> bool { self.current_uncertainty < threshold }
    pub fn reset(&mut self) { self.current_uncertainty = 1.0; self.steps = 0; }
    pub fn set_rate(&mut self, rate: f64) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&rate) { return Err(TensionsError::OutOfRange { field: "reduction_rate".into(), value: rate, min: 0.0, max: 1.0 }); }
        self.reduction_rate = rate;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.current_uncertainty) { return Err(TensionsError::OutOfRange { field: "current_uncertainty".into(), value: self.current_uncertainty, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for UncertaintyReduction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UncertaintyReduction").field("rate", &self.reduction_rate).field("uncertainty", &self.current_uncertainty).field("steps", &self.steps).finish()
    }
}
'@

Write-Host 'Tensions partial generation complete'
