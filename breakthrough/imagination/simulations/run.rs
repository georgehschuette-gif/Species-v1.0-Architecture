// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::ImaginationError;

use super::engine::{SimulationEngine, SimulationTick};

use super::*;

pub struct SimulationRun {
    pub steps: usize,
    pub current_step: usize,
    pub history: Vec<SimulationTick>,
    pub metrics: RunMetrics,
    pub status: RunStatus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RunStatus {
    Queued,
    Active,
    Finished,
    Aborted,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RunMetrics {
    pub total_energy: f64,
    pub peak_energy: f64,
    pub mean_energy: f64,
    pub convergence_step: Option<usize>,
}

impl SimulationRun {
    pub fn new(steps: usize) -> Result<Self, ImaginationError> {
        if steps == 0 {
            return Err(ImaginationError::InvalidInput("steps must be non-zero".into()));
        }
        if steps > MAX_SIMULATION_DEPTH * 10 {
            return Err(ImaginationError::CapacityExceeded { max: MAX_SIMULATION_DEPTH * 10, attempted: steps });
        }
        Ok(Self {
            steps,
            current_step: 0,
            history: Vec::with_capacity(steps.min(MAX_SIMULATION_DEPTH * 10)),
            metrics: RunMetrics {
                total_energy: 0.0,
                peak_energy: 0.0,
                mean_energy: 0.0,
                convergence_step: None,
            },
            status: RunStatus::Queued,
        })
    }

    pub fn steps(&self) -> usize { self.steps }
    pub fn current_step(&self) -> usize { self.current_step }
    pub fn history(&self) -> &[SimulationTick] { &self.history }
    pub fn metrics(&self) -> RunMetrics { self.metrics.clone() }
    pub fn status(&self) -> RunStatus { self.status }

    pub fn execute(&mut self, engine: &mut SimulationEngine) -> Result<(), ImaginationError> {
        engine.start()?;
        self.status = RunStatus::Active;
        self.current_step = 0;
        self.history.clear();
        self.metrics = RunMetrics {
            total_energy: 0.0,
            peak_energy: 0.0,
            mean_energy: 0.0,
            convergence_step: None,
        };

        for _ in 0..self.steps.min(engine.parameters().max_iterations) {
            let tick = engine.step()?;
            self.history.push(tick);
            self.metrics.total_energy += tick.energy;
            if tick.energy.abs() > self.metrics.peak_energy {
                self.metrics.peak_energy = tick.energy.abs();
            }
            if tick.is_terminal {
                self.metrics.convergence_step = Some(self.current_step);
                break;
            }
            self.current_step += 1;
        }

        if !self.history.is_empty() {
            self.metrics.mean_energy = self.metrics.total_energy / self.history.len() as f64;
        }

        self.status = RunStatus::Finished;
        Ok(())
    }

    pub fn abort(&mut self) {
        self.status = RunStatus::Aborted;
    }

    pub fn energy_at(&self, step: usize) -> Option<f64> {
        self.history.get(step).map(|t| t.energy)
    }

    pub fn validate(&self) -> Result<(), ImaginationError> {
        if self.steps == 0 {
            return Err(ImaginationError::InvalidInput("steps cannot be zero".into()));
        }
        if self.current_step > self.steps {
            return Err(ImaginationError::OutOfRange { field: "current_step".into(), value: self.current_step as f64, min: 0.0, max: self.steps as f64 });
        }
        Ok(())
    }
}

impl fmt::Display for SimulationRun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SimulationRun(steps={}, current={}, status={:?})", self.steps, self.current_step, self.status)
    }
}

impl fmt::Display for RunMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RunMetrics(total={:.4}, peak={:.4}, mean={:.4})", self.total_energy, self.peak_energy, self.mean_energy)
    }
}
