// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Ordered,
    Critical,
    Disordered,
    Transitioning,
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Phase::Ordered => write!(f, "Ordered"),
            Phase::Critical => write!(f, "Critical"),
            Phase::Disordered => write!(f, "Disordered"),
            Phase::Transitioning => write!(f, "Transitioning"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroState {
    phase: Phase,
    macroscopic_vars: Vec<f64>,
    stability: f64,
}

impl MacroState {
    pub fn new(phase: Phase, macroscopic_vars: Vec<f64>, stability: f64) -> crate::Result<Self> {
        if macroscopic_vars.is_empty() {
            return Err(crate::EmergenceError::MeasurementError(
                "macroscopic variables cannot be empty".to_string(),
            ));
        }
        if !(0.0..=1.0).contains(&stability) {
            return Err(crate::EmergenceError::MeasurementError(
                "stability must be between 0.0 and 1.0".to_string(),
            ));
        }
        Ok(MacroState {
            phase,
            macroscopic_vars,
            stability,
        })
    }

    pub fn phase(&self) -> Phase {
        self.phase
    }

    pub fn macroscopic_vars(&self) -> &[f64] {
        &self.macroscopic_vars
    }

    pub fn stability(&self) -> f64 {
        self.stability
    }

    pub fn evolve(&mut self, new_phase: Phase, delta_vars: Vec<f64>) -> crate::Result<()> {
        if delta_vars.len() != self.macroscopic_vars.len() {
            return Err(crate::EmergenceError::MeasurementError(
                "delta_vars dimension mismatch".to_string(),
            ));
        }
        for (i, delta) in delta_vars.iter().enumerate() {
            self.macroscopic_vars[i] += delta;
        }
        self.phase = new_phase;
        Ok(())
    }

    pub fn validate(&self) -> crate::Result<()> {
        if self.macroscopic_vars.is_empty() {
            return Err(crate::EmergenceError::MeasurementError(
                "macrostate has no macroscopic variables".to_string(),
            ));
        }
        if !(0.0..=1.0).contains(&self.stability) {
            return Err(crate::EmergenceError::MeasurementError(
                format!("invalid stability: {}", self.stability),
            ));
        }
        Ok(())
    }
}

impl fmt::Display for MacroState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MacroState(phase={}, vars={}, stability={:.3})",
            self.phase,
            self.macroscopic_vars.len(),
            self.stability
        )
    }
}
