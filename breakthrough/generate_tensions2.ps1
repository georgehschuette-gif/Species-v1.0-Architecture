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
# TENSIONS - remaining submodules
# ============================================================================

# --- tensions/novelty ---
New-RustFile -Path "$root\tensions\novelty\mod.rs" -Value @'
pub mod detection;
pub mod weighting;
pub mod decay;

pub use detection::NoveltyDetection;
pub use weighting::NoveltyWeighting;
pub use decay::NoveltyDecay;

pub const DEFAULT_NOVELTY_THRESHOLD: f64 = 0.6;
pub const MAX_NOVELTY: f64 = 1.0;

pub fn create_novelty_detection(sensitivity: f64) -> Result<NoveltyDetection, TensionsError> {
    NoveltyDetection::new(sensitivity)
}

pub fn compute_novelty_weight(familiarity: f64) -> f64 {
    1.0 - familiarity
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn novelty_weight_inverse_familiarity() {
        assert!((compute_novelty_weight(0.3) - 0.7).abs() < 1e-6);
    }
}
'@

New-RustFile -Path "$root\tensions\novelty\detection.rs" -Value @'
use std::fmt;
use super::*;

pub struct NoveltyDetection {
    pub sensitivity: f64,
    pub baseline: f64,
    pub history: Vec<f64>,
}

impl NoveltyDetection {
    pub fn new(sensitivity: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&sensitivity) { return Err(TensionsError::OutOfRange { field: "sensitivity".into(), value: sensitivity, min: 0.0, max: 1.0 }); }
        Ok(Self { sensitivity, baseline: 0.0, history: Vec::new() })
    }

    pub fn observe(&mut self, signal: f64) -> Result<f64, TensionsError> {
        self.history.push(signal);
        if self.history.len() > 1000 { self.history.remove(0); }
        let mean = self.history.iter().sum::<f64>() / self.history.len() as f64;
        let novelty = (signal - mean).abs() * self.sensitivity;
        Ok(novelty.clamp(0.0, 1.0))
    }

    pub fn update_baseline(&mut self) { self.baseline = self.history.iter().sum::<f64>() / self.history.len() as f64; }
    pub fn is_novel(&self, signal: f64, threshold: f64) -> bool { (signal - self.baseline).abs() > threshold }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.sensitivity) { return Err(TensionsError::OutOfRange { field: "sensitivity".into(), value: self.sensitivity, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for NoveltyDetection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NoveltyDetection").field("sensitivity", &self.sensitivity).field("baseline", &self.baseline).finish()
    }
}
'@

New-RustFile -Path "$root\tensions\novelty\weighting.rs" -Value @'
use std::fmt;
use super::*;

pub struct NoveltyWeighting {
    pub weight: f64,
    pub familiarity: f64,
    pub recency: f64,
}

impl NoveltyWeighting {
    pub fn new(weight: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&weight) { return Err(TensionsError::OutOfRange { field: "weight".into(), value: weight, min: 0.0, max: 1.0 }); }
        Ok(Self { weight, familiarity: 1.0 - weight, recency: 1.0 })
    }

    pub fn compute(&self) -> f64 { self.weight * self.recency }
    pub fn update_familiarity(&mut self, delta: f64) -> Result<(), TensionsError> {
        self.familiarity = (self.familiarity + delta).clamp(0.0, 1.0);
        self.weight = 1.0 - self.familiarity;
        Ok(())
    }
    pub fn decay_recency(&mut self, rate: f64) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&rate) { return Err(TensionsError::OutOfRange { field: "rate".into(), value: rate, min: 0.0, max: 1.0 }); }
        self.recency = (self.recency * (1.0 - rate)).max(0.0);
        Ok(())
    }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.weight) { return Err(TensionsError::OutOfRange { field: "weight".into(), value: self.weight, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for NoveltyWeighting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NoveltyWeighting").field("weight", &self.weight).field("familiarity", &self.familiarity).field("recency", &self.recency).finish()
    }
}
'@

New-RustFile -Path "$root\tensions\novelty\decay.rs" -Value @'
use std::fmt;
use super::*;

pub struct NoveltyDecay {
    pub decay_constant: f64,
    pub current_novelty: f64,
}

impl NoveltyDecay {
    pub fn new(decay_constant: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&decay_constant) { return Err(TensionsError::OutOfRange { field: "decay_constant".into(), value: decay_constant, min: 0.0, max: 1.0 }); }
        Ok(Self { decay_constant, current_novelty: 1.0 })
    }

    pub fn step(&mut self) -> f64 {
        self.current_novelty *= (1.0 - self.decay_constant);
        self.current_novelty.max(0.0)
    }

    pub fn inject(&mut self, amount: f64) -> Result<(), TensionsError> {
        self.current_novelty = (self.current_novelty + amount).min(1.0);
        Ok(())
    }
    pub fn level(&self) -> f64 { self.current_novelty }
    pub fn is_familiar(&self, threshold: f64) -> bool { self.current_novelty < threshold }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.current_novelty) { return Err(TensionsError::OutOfRange { field: "current_novelty".into(), value: self.current_novelty, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for NoveltyDecay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NoveltyDecay").field("decay_constant", &self.decay_constant).field("current_novelty", &self.current_novelty).finish()
    }
}
'@

# --- tensions/curiosity ---
New-RustFile -Path "$root\tensions\curiosity\mod.rs" -Value @'
pub mod drive;
pub mod saturation;
pub mod reward;

pub use drive::CuriosityDrive;
pub use saturation::CuriositySaturation;
pub use reward::CuriosityReward;

pub const DEFAULT_CURIOUSITY_DRIVE: f64 = 0.7;
pub const MAX_CURIOUSITY: f64 = 1.0;

pub fn create_curiosity_drive(intensity: f64) -> Result<CuriosityDrive, TensionsError> {
    CuriosityDrive::new(intensity)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn curiosity_drive_creation() {
        let d = create_curiosity_drive(0.8).unwrap();
        assert!(d.intensity() > 0.0);
    }
}
'@

New-RustFile -Path "$root\tensions\curiosity\drive.rs" -Value @'
use std::fmt;
use super::*;

pub struct CuriosityDrive {
    pub intensity: f64,
    pub direction: f64,
    pub persistence: f64,
}

impl CuriosityDrive {
    pub fn new(intensity: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&intensity) { return Err(TensionsError::OutOfRange { field: "intensity".into(), value: intensity, min: 0.0, max: 1.0 }); }
        Ok(Self { intensity, direction: 0.0, persistence: 1.0 })
    }

    pub fn intensity(&self) -> f64 { self.intensity }
    pub fn orient(&mut self, direction: f64) -> Result<(), TensionsError> {
        if !(-1.0..=1.0).contains(&direction) { return Err(TensionsError::OutOfRange { field: "direction".into(), value: direction, min: -1.0, max: 1.0 }); }
        self.direction = direction;
        Ok(())
    }
    pub fn fatigue(&mut self, amount: f64) -> Result<(), TensionsError> {
        self.persistence = (self.persistence - amount).max(0.0);
        self.intensity = (self.intensity * self.persistence).clamp(0.0, 1.0);
        Ok(())
    }
    pub fn restore(&mut self, amount: f64) -> Result<(), TensionsError> {
        self.persistence = (self.persistence + amount).min(1.0);
        Ok(())
    }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.intensity) { return Err(TensionsError::OutOfRange { field: "intensity".into(), value: self.intensity, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for CuriosityDrive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CuriosityDrive").field("intensity", &self.intensity).field("direction", &self.direction).field("persistence", &self.persistence).finish()
    }
}
'@

New-RustFile -Path "$root\tensions\curiosity\saturation.rs" -Value @'
use std::fmt;
use super::*;

pub struct CuriositySaturation {
    pub current_level: f64,
    pub threshold: f64,
    pub saturation_curve: f64,
}

impl CuriositySaturation {
    pub fn new(threshold: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&threshold) { return Err(TensionsError::OutOfRange { field: "threshold".into(), value: threshold, min: 0.0, max: 1.0 }); }
        Ok(Self { current_level: 0.0, threshold, saturation_curve: 2.0 })
    }

    pub fn increase(&mut self, amount: f64) -> Result<bool, TensionsError> {
        self.current_level = (self.current_level + amount).min(1.0);
        Ok(self.current_level >= self.threshold)
    }

    pub fn is_saturated(&self) -> bool { self.current_level >= self.threshold }
    pub fn relief(&mut self, amount: f64) -> Result<(), TensionsError> {
        self.current_level = (self.current_level - amount).max(0.0);
        Ok(())
    }
    pub fn set_curve(&mut self, curve: f64) -> Result<(), TensionsError> {
        if curve <= 0.0 { return Err(TensionsError::OutOfRange { field: "saturation_curve".into(), value: curve, min: 0.0, max: f64::INFINITY }); }
        self.saturation_curve = curve;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.current_level) { return Err(TensionsError::OutOfRange { field: "current_level".into(), value: self.current_level, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for CuriositySaturation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CuriositySaturation").field("level", &self.current_level).field("threshold", &self.threshold).finish()
    }
}
'@

New-RustFile -Path "$root\tensions\curiosity\reward.rs" -Value @'
use std::fmt;
use super::*;

pub struct CuriosityReward {
    pub predicted_value: f64,
    pub actual_value: f64,
    pub learning_signal: f64,
}

impl CuriosityReward {
    pub fn new(predicted: f64, actual: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&predicted) || !(0.0..=1.0).contains(&actual) {
            return Err(TensionsError::OutOfRange { field: "values".into(), value: predicted.max(actual), min: 0.0, max: 1.0 });
        }
        let diff = (actual - predicted).abs();
        Ok(Self { predicted_value: predicted, actual_value: actual, learning_signal: diff })
    }

    pub fn signal(&self) -> f64 { self.learning_signal }
    pub fn error(&self) -> f64 { (self.actual_value - self.predicted_value).abs() }
    pub fn update(&mut self, actual: f64) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&actual) { return Err(TensionsError::OutOfRange { field: "actual".into(), value: actual, min: 0.0, max: 1.0 }); }
        self.actual_value = actual;
        self.learning_signal = (actual - self.predicted_value).abs();
        Ok(())
    }
    pub fn reinforce(&mut self, amount: f64) -> Result<(), TensionsError> {
        self.predicted_value = (self.predicted_value + amount).clamp(0.0, 1.0);
        Ok(())
    }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.learning_signal) { return Err(TensionsError::OutOfRange { field: "learning_signal".into(), value: self.learning_signal, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for CuriosityReward {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CuriosityReward").field("predicted", &self.predicted_value).field("actual", &self.actual_value).field("signal", &self.learning_signal).finish()
    }
}
'@

# --- tensions/paradox ---
New-RustFile -Path "$root\tensions\paradox\mod.rs" -Value @'
pub mod containment;
pub mod suspension;
pub mod resolution;

pub use containment::ParadoxContainment;
pub use suspension::ParadoxSuspension;
pub use resolution::ParadoxResolution;

pub const DEFAULT_PARADOX_TOLERANCE: f64 = 0.4;
pub const MAX_PARADOX_INTENSITY: f64 = 1.0;

pub fn contain_paradox(intensity: f64) -> Result<ParadoxContainment, TensionsError> {
    ParadoxContainment::new(intensity)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn paradox_containment_creation() {
        let c = contain_paradox(0.6).unwrap();
        assert!(c.intensity() > 0.0);
    }
}
'@

New-RustFile -Path "$root\tensions\paradox\containment.rs" -Value @'
use std::fmt;
use super::*;

pub struct ParadoxContainment {
    pub intensity: f64,
    pub boundary_strength: f64,
    pub containment_zone: f64,
}

impl ParadoxContainment {
    pub fn new(intensity: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&intensity) { return Err(TensionsError::OutOfRange { field: "intensity".into(), value: intensity, min: 0.0, max: 1.0 }); }
        Ok(Self { intensity, boundary_strength: 0.5, containment_zone: 0.0 })
    }

    pub fn intensity(&self) -> f64 { self.intensity }
    pub fn strengthen_boundary(&mut self, amount: f64) -> Result<(), TensionsError> {
        self.boundary_strength = (self.boundary_strength + amount).clamp(0.0, 1.0);
        self.containment_zone = self.intensity * (1.0 - self.boundary_strength);
        Ok(())
    }
    pub fn is_contained(&self, threshold: f64) -> bool { self.containment_zone < threshold }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.intensity) { return Err(TensionsError::OutOfRange { field: "intensity".into(), value: self.intensity, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for ParadoxContainment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParadoxContainment").field("intensity", &self.intensity).field("boundary", &self.boundary_strength).finish()
    }
}
'@

New-RustFile -Path "$root\tensions\paradox\suspension.rs" -Value @'
use std::fmt;
use super::*;

pub struct ParadoxSuspension {
    pub suspended: bool,
    pub suspension_strength: f64,
    pub time_remaining: f64,
}

impl ParadoxSuspension {
    pub fn new() -> Self {
        Self { suspended: false, suspension_strength: 0.0, time_remaining: 0.0 }
    }

    pub fn suspend(&mut self, strength: f64, duration: f64) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&strength) { return Err(TensionsError::OutOfRange { field: "strength".into(), value: strength, min: 0.0, max: 1.0 }); }
        if duration <= 0.0 { return Err(TensionsError::OutOfRange { field: "duration".into(), value: duration, min: 0.0, max: f64::INFINITY }); }
        self.suspended = true;
        self.suspension_strength = strength;
        self.time_remaining = duration;
        Ok(())
    }

    pub fn tick(&mut self, dt: f64) -> Result<bool, TensionsError> {
        if !self.suspended { return Ok(false); }
        self.time_remaining -= dt;
        if self.time_remaining <= 0.0 {
            self.suspended = false;
            self.suspension_strength = 0.0;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn is_suspended(&self) -> bool { self.suspended }
    pub fn release(&mut self) { self.suspended = false; self.suspension_strength = 0.0; self.time_remaining = 0.0; }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if self.suspended && !(0.0..=1.0).contains(&self.suspension_strength) { return Err(TensionsError::OutOfRange { field: "suspension_strength".into(), value: self.suspension_strength, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for ParadoxSuspension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParadoxSuspension").field("suspended", &self.suspended).field("strength", &self.suspension_strength).field("time", &self.time_remaining).finish()
    }
}
'@

New-RustFile -Path "$root\tensions\paradox\resolution.rs" -Value @'
use std::fmt;
use super::*;

pub struct ParadoxResolution {
    pub resolution_type: ParadoxResolutionType,
    pub coherence_gain: f64,
    pub energy_cost: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParadoxResolutionType {
    Reframe,
    Accept,
    Transcend,
    Dissolve,
}

impl ParadoxResolution {
    pub fn new(resolution_type: ParadoxResolutionType) -> Self {
        Self { resolution_type, coherence_gain: 0.0, energy_cost: 0.0 }
    }

    pub fn apply(&mut self, paradox: &ParadoxContainment) -> Result<f64, TensionsError> {
        self.coherence_gain = match self.resolution_type {
            ParadoxResolutionType::Reframe => 0.4,
            ParadoxResolutionType::Accept => 0.6,
            ParadoxResolutionType::Transcend => 0.9,
            ParadoxResolutionType::Dissolve => 1.0,
        };
        self.energy_cost = paradox.intensity * 0.5;
        Ok(self.coherence_gain)
    }

    pub fn coherence(&self) -> f64 { self.coherence_gain }
    pub fn cost(&self) -> f64 { self.energy_cost }
    pub fn set_type(&mut self, resolution_type: ParadoxResolutionType) { self.resolution_type = resolution_type; }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.coherence_gain) { return Err(TensionsError::OutOfRange { field: "coherence_gain".into(), value: self.coherence_gain, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for ParadoxResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParadoxResolution").field("type", &self.resolution_type).field("coherence", &self.coherence_gain).finish()
    }
}
'@

# --- tensions/incompleteness ---
New-RustFile -Path "$root\tensions\incompleteness\mod.rs" -Value @'
pub mod gap;
pub mod completion;
pub mod tolerance;

pub use gap::IncompletenessGap;
pub use completion::IncompletenessCompletion;
pub use tolerance::IncompletenessTolerance;

pub const DEFAULT_GAP_TOLERANCE: f64 = 0.2;
pub const MAX_INCOMPLETENESS: f64 = 1.0;

pub fn measure_gap(known: f64, total: f64) -> f64 {
    if total <= 0.0 { return 0.0; }
    (1.0 - known / total).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gap_measurement() {
        let g = measure_gap(3.0, 10.0);
        assert!((g - 0.7).abs() < 1e-6);
    }
}
'@

New-RustFile -Path "$root\tensions\incompleteness\gap.rs" -Value @'
use std::fmt;
use super::*;

pub struct IncompletenessGap {
    pub known_fraction: f64,
    pub total_domain: f64,
    pub gap_size: f64,
}

impl IncompletenessGap {
    pub fn new(known: f64, total: f64) -> Result<Self, TensionsError> {
        if total < 0.0 { return Err(TensionsError::OutOfRange { field: "total".into(), value: total, min: 0.0, max: f64::INFINITY }); }
        let gap = if total > 0.0 { (1.0 - known / total).clamp(0.0, 1.0) } else { 0.0 };
        Ok(Self { known_fraction: known, total_domain: total, gap_size: gap })
    }

    pub fn size(&self) -> f64 { self.gap_size }
    pub fn known(&self) -> f64 { self.known_fraction }
    pub fn total(&self) -> f64 { self.total_domain }
    pub fn update_known(&mut self, delta: f64) -> Result<(), TensionsError> {
        self.known_fraction = (self.known_fraction + delta).clamp(0.0, self.total_domain);
        self.gap_size = if self.total_domain > 0.0 { (1.0 - self.known_fraction / self.total_domain).clamp(0.0, 1.0) } else { 0.0 };
        Ok(())
    }
    pub fn is_critical(&self, threshold: f64) -> bool { self.gap_size > threshold }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if self.total_domain < 0.0 { return Err(TensionsError::OutOfRange { field: "total_domain".into(), value: self.total_domain, min: 0.0, max: f64::INFINITY }); }
        Ok(())
    }
}

impl fmt::Debug for IncompletenessGap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IncompletenessGap").field("gap", &self.gap_size).field("known", &self.known_fraction).field("total", &self.total_domain).finish()
    }
}
'@

New-RustFile -Path "$root\tensions\incompleteness\completion.rs" -Value @'
use std::fmt;
use super::*;

pub struct IncompletenessCompletion {
    pub progress: f64,
    pub estimated_remaining: f64,
    pub steps: usize,
}

impl IncompletenessCompletion {
    pub fn new() -> Self {
        Self { progress: 0.0, estimated_remaining: 1.0, steps: 0 }
    }

    pub fn advance(&mut self, amount: f64) -> Result<(), TensionsError> {
        self.progress = (self.progress + amount).min(1.0);
        self.estimated_remaining = 1.0 - self.progress;
        self.steps += 1;
        Ok(())
    }

    pub fn progress(&self) -> f64 { self.progress }
    pub fn is_complete(&self) -> bool { self.progress >= 1.0 }
    pub fn estimate_remaining(&self) -> f64 { self.estimated_remaining }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.progress) { return Err(TensionsError::OutOfRange { field: "progress".into(), value: self.progress, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for IncompletenessCompletion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IncompletenessCompletion").field("progress", &self.progress).field("remaining", &self.estimated_remaining).finish()
    }
}
'@

New-RustFile -Path "$root\tensions\incompleteness\tolerance.rs" -Value @'
use std::fmt;
use super::*;

pub struct IncompletenessTolerance {
    pub tolerance: f64,
    pub acceptable_gap: f64,
    pub anxiety: f64,
}

impl IncompletenessTolerance {
    pub fn new(tolerance: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&tolerance) { return Err(TensionsError::OutOfRange { field: "tolerance".into(), value: tolerance, min: 0.0, max: 1.0 }); }
        Ok(Self { tolerance, acceptable_gap: tolerance, anxiety: 0.0 })
    }

    pub fn assess(&mut self, gap: f64) -> Result<(), TensionsError> {
        if gap > self.acceptable_gap {
            self.anxiety = (self.anxiety + (gap - self.acceptable_gap)).min(1.0);
        } else {
            self.anxiety = (self.anxiety * 0.9).max(0.0);
        }
        Ok(())
    }

    pub fn anxiety_level(&self) -> f64 { self.anxiety }
    pub fn is_tolerable(&self, gap: f64) -> bool { gap <= self.acceptable_gap }
    pub fn set_tolerance(&mut self, tolerance: f64) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&tolerance) { return Err(TensionsError::OutOfRange { field: "tolerance".into(), value: tolerance, min: 0.0, max: 1.0 }); }
        self.tolerance = tolerance;
        self.acceptable_gap = tolerance;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.tolerance) { return Err(TensionsError::OutOfRange { field: "tolerance".into(), value: self.tolerance, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for IncompletenessTolerance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IncompletenessTolerance").field("tolerance", &self.tolerance).field("anxiety", &self.anxiety).finish()
    }
}
'@

# --- tensions/pressure_maps ---
New-RustFile -Path "$root\tensions\pressure_maps\mod.rs" -Value @'
pub mod field;
pub mod gradient;
pub mod relief;

pub use field::PressureField;
pub use gradient::PressureGradient;
pub use relief::PressureRelief;

pub const DEFAULT_PRESSURE_THRESHOLD: f64 = 0.7;
pub const MAX_PRESSURE: f64 = 1.0;

pub fn create_pressure_field(dimensions: usize) -> PressureField {
    PressureField::new(dimensions)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pressure_field_creation() {
        let f = create_pressure_field(3);
        assert_eq!(f.dimensions(), 3);
    }
}
'@

New-RustFile -Path "$root\tensions\pressure_maps\field.rs" -Value @'
use std::fmt;
use super::*;

pub struct PressureField {
    pub dimensions: usize,
    pub values: Vec<f64>,
    pub max_pressure: f64,
}

impl PressureField {
    pub fn new(dimensions: usize) -> Self {
        let values = vec![0.0; dimensions];
        Self { dimensions, values, max_pressure: 0.0 }
    }

    pub fn dimensions(&self) -> usize { self.dimensions }
    pub fn set(&mut self, index: usize, value: f64) -> Result<(), TensionsError> {
        if index >= self.dimensions { return Err(TensionsError::OutOfRange { field: "index".into(), value: index as f64, min: 0.0, max: (self.dimensions - 1) as f64 }); }
        if !(0.0..=1.0).contains(&value) { return Err(TensionsError::OutOfRange { field: "value".into(), value, min: 0.0, max: 1.0 }); }
        self.values[index] = value;
        self.max_pressure = self.values.iter().cloned().fold(0.0, f64::max);
        Ok(())
    }

    pub fn get(&self, index: usize) -> Result<f64, TensionsError> {
        if index >= self.dimensions { return Err(TensionsError::OutOfRange { field: "index".into(), value: index as f64, min: 0.0, max: (self.dimensions - 1) as f64 }); }
        Ok(self.values[index])
    }

    pub fn is_critical(&self, threshold: f64) -> bool { self.max_pressure > threshold }
    pub fn normalize(&mut self) { let sum: f64 = self.values.iter().sum(); if sum > 0.0 { for v in &mut self.values { *v /= sum; } } }
    pub fn validate(&self) -> Result<(), TensionsError> {
        for (i, v) in self.values.iter().enumerate() { if !(0.0..=1.0).contains(v) { return Err(TensionsError::OutOfRange { field: format!("values[{}]", i), value: *v, min: 0.0, max: 1.0 }); } }
        Ok(())
    }
}

impl fmt::Debug for PressureField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PressureField").field("dimensions", &self.dimensions).field("max", &self.max_pressure).finish()
    }
}
'@

New-RustFile -Path "$root\tensions\pressure_maps\gradient.rs" -Value @'
use std::fmt;
use super::*;

pub struct PressureGradient {
    pub vector: Vec<f64>,
    pub magnitude: f64,
}

impl PressureGradient {
    pub fn new(vector: Vec<f64>) -> Result<Self, TensionsError> {
        let mag = vector.iter().map(|v| v * v).sum::<f64>().sqrt();
        if !(0.0..=f64::INFINITY).contains(&mag) { return Err(TensionsError::OutOfRange { field: "magnitude".into(), value: mag, min: 0.0, max: f64::INFINITY }); }
        Ok(Self { vector, magnitude: mag })
    }

    pub fn direction(&self) -> Vec<f64> { self.vector.clone() }
    pub fn strength(&self) -> f64 { self.magnitude }
    pub fn step(&mut self, field: &mut PressureField) -> Result<(), TensionsError> {
        if field.dimensions() != self.vector.len() { return Err(TensionsError::DimensionMismatch { expected: field.dimensions(), actual: self.vector.len() }); }
        for (i, g) in self.vector.iter().enumerate() {
            field.values[i] = (field.values[i] + g * 0.1).clamp(0.0, 1.0);
        }
        Ok(())
    }
    pub fn normalize(&mut self) { let mag = self.magnitude.max(1e-12); for v in &mut self.vector { *v /= mag; } self.magnitude = 1.0; }
    pub fn validate(&self) -> Result<(), TensionsError> {
        for (i, v) in self.vector.iter().enumerate() { if !v.is_finite() { return Err(TensionsError::OutOfRange { field: format!("vector[{}]", i), value: *v, min: -f64::INFINITY, max: f64::INFINITY }); } }
        Ok(())
    }
}

impl fmt::Debug for PressureGradient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PressureGradient").field("magnitude", &self.magnitude).finish()
    }
}
'@

New-RustFile -Path "$root\tensions\pressure_maps\relief.rs" -Value @'
use std::fmt;
use super::*;

pub struct PressureRelief {
    pub relief_capacity: f64,
    pub current_load: f64,
    pub efficiency: f64,
}

impl PressureRelief {
    pub fn new(capacity: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&capacity) { return Err(TensionsError::OutOfRange { field: "capacity".into(), value: capacity, min: 0.0, max: 1.0 }); }
        Ok(Self { relief_capacity: capacity, current_load: 0.0, efficiency: 1.0 })
    }

    pub fn load(&mut self, amount: f64) -> Result<(), TensionsError> {
        self.current_load = (self.current_load + amount).min(1.0);
        Ok(())
    }

    pub fn relieve(&mut self, amount: f64) -> Result<f64, TensionsError> {
        let relieved = (amount * self.efficiency).min(self.current_load);
        self.current_load -= relieved;
        Ok(relieved)
    }

    pub fn overloaded(&self, threshold: f64) -> bool { self.current_load > threshold }
    pub fn efficiency(&self) -> f64 { self.efficiency }
    pub fn set_efficiency(&mut self, eff: f64) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&eff) { return Err(TensionsError::OutOfRange { field: "efficiency".into(), value: eff, min: 0.0, max: 1.0 }); }
        self.efficiency = eff;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.current_load) { return Err(TensionsError::OutOfRange { field: "current_load".into(), value: self.current_load, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for PressureRelief {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PressureRelief").field("capacity", &self.relief_capacity).field("load", &self.current_load).field("efficiency", &self.efficiency).finish()
    }
}
'@

Write-Host 'Tensions complete'
