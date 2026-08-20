// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::ImaginationError;

use super::*;

pub struct SimulationEngine {
    pub seed: u64,
    pub state: SimulationState,
    pub tick_count: usize,
    pub parameters: SimulationParameters,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SimulationState {
    Idle,
    Running,
    Paused,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimulationParameters {
    pub time_step: f64,
    pub max_iterations: usize,
    pub convergence_threshold: f64,
    pub stochasticity: f64,
}

impl SimulationEngine {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            state: SimulationState::Idle,
            tick_count: 0,
            parameters: SimulationParameters {
                time_step: 0.1,
                max_iterations: 1000,
                convergence_threshold: 1e-6,
                stochasticity: 0.01,
            },
        }
    }

    pub fn seed(&self) -> u64 { self.seed }
    pub fn state(&self) -> SimulationState { self.state }
    pub fn tick_count(&self) -> usize { self.tick_count }
    pub fn parameters(&self) -> SimulationParameters { self.parameters }

    pub fn configure(&mut self, params: SimulationParameters) {
        self.parameters = params;
    }

    pub fn start(&mut self) -> Result<(), ImaginationError> {
        if matches!(self.state, SimulationState::Running) {
            return Err(ImaginationError::InvalidInput("simulation already running".into()));
        }
        self.state = SimulationState::Running;
        self.tick_count = 0;
        Ok(())
    }

    pub fn step(&mut self) -> Result<SimulationTick, ImaginationError> {
        if !matches!(self.state, SimulationState::Running) {
            return Err(ImaginationError::InvalidInput("simulation not running".into()));
        }
        if self.tick_count >= self.parameters.max_iterations {
            self.state = SimulationState::Completed;
            return Ok(SimulationTick::terminal());
        }

        let energy = (self.tick_count as f64 * self.parameters.time_step).sin() * self.parameters.stochasticity;
        self.tick_count += 1;

        if energy.abs() < self.parameters.convergence_threshold && self.tick_count > 10 {
            self.state = SimulationState::Completed;
        }

        Ok(SimulationTick::new(self.tick_count, energy))
    }

    pub fn pause(&mut self) -> Result<(), ImaginationError> {
        if !matches!(self.state, SimulationState::Running) {
            return Err(ImaginationError::InvalidInput("cannot pause non-running simulation".into()));
        }
        self.state = SimulationState::Paused;
        Ok(())
    }

    pub fn reset(&mut self) {
        self.state = SimulationState::Idle;
        self.tick_count = 0;
    }

    pub fn validate(&self) -> Result<(), ImaginationError> {
        if self.parameters.max_iterations == 0 {
            return Err(ImaginationError::InvalidInput("max_iterations cannot be zero".into()));
        }
        if self.parameters.time_step <= 0.0 {
            return Err(ImaginationError::OutOfRange { field: "time_step".into(), value: self.parameters.time_step, min: 0.0, max: f64::MAX });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimulationTick {
    pub tick: usize,
    pub energy: f64,
    pub is_terminal: bool,
}

impl SimulationTick {
    pub fn new(tick: usize, energy: f64) -> Self {
        Self { tick, energy, is_terminal: false }
    }

    pub fn terminal() -> Self {
        Self { tick: 0, energy: 0.0, is_terminal: true }
    }

    pub fn magnitude(&self) -> f64 {
        self.energy.abs()
    }
}

impl fmt::Display for SimulationEngine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SimulationEngine(seed={}, ticks={}, state={:?})", self.seed, self.tick_count, self.state)
    }
}

impl Default for SimulationParameters {
    fn default() -> Self {
        Self {
            time_step: 0.1,
            max_iterations: 1000,
            convergence_threshold: 1e-6,
            stochasticity: 0.01,
        }
    }
}

impl fmt::Display for SimulationState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Idle => write!(f, "Idle"),
            Self::Running => write!(f, "Running"),
            Self::Paused => write!(f, "Paused"),
            Self::Completed => write!(f, "Completed"),
            Self::Failed => write!(f, "Failed"),
        }
    }
}
