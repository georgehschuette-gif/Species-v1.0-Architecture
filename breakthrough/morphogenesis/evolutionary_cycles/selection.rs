// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

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
