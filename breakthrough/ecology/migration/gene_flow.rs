// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// GeneFlow: The transfer of genetic or structural information between populations.
pub struct GeneFlow {
    pub source_population: u64,
    pub target_population: u64,
    pub flow_rate: f64,
    pub migrants: usize,
}

impl GeneFlow {
    /// Construct a new [GeneFlow] instance.
    ///
    /// # Errors
    /// Returns [MigrationError::UninitializedReference] if either
    /// `source_population` or `target_population` is zero.
    /// Returns [MigrationError::InvalidGeneFlowRate] if `flow_rate`
    /// is negative or NaN.
    /// Returns [MigrationError::MigrantCountExceedsPopulation] if
    /// `migrants` exceeds a reasonable bound (flow_rate * 1000).
    pub fn new(
        source_population: u64,
        target_population: u64,
        flow_rate: f64,
        migrants: usize,
    ) -> Result<Self, MigrationError> {
        if source_population == 0 {
            return Err(MigrationError::InvalidSourcePopulation);
        }
        if target_population == 0 {
            return Err(MigrationError::InvalidTargetHabitat);
        }
        if flow_rate.is_nan() || flow_rate < 0.0 {
            return Err(MigrationError::InvalidGeneFlowRate);
        }
        let max_migrants = (flow_rate * 1000.0).ceil() as usize;
        if migrants > max_migrants.max(1) {
            return Err(MigrationError::MigrantCountExceedsPopulation);
        }
        if source_population == target_population {
            return Err(MigrationError::ContradictoryParameters);
        }
        Ok(Self {
            source_population,
            target_population,
            flow_rate,
            migrants,
        })
    }

    /// Validate the gene flow parameters for internal consistency.
    pub fn validate(&self) -> Result<(), MigrationError> {
        if self.source_population == 0 {
            return Err(MigrationError::InvalidSourcePopulation);
        }
        if self.target_population == 0 {
            return Err(MigrationError::InvalidTargetHabitat);
        }
        if self.flow_rate.is_nan() || self.flow_rate < 0.0 {
            return Err(MigrationError::InvalidGeneFlowRate);
        }
        if self.source_population == self.target_population {
            return Err(MigrationError::ContradictoryParameters);
        }
        let max_migrants = (self.flow_rate * 1000.0).ceil() as usize;
        if self.migrants > max_migrants.max(1) {
            return Err(MigrationError::MigrantCountExceedsPopulation);
        }
        Ok(())
    }

    /// Compute the effective number of migrants per generation,
    /// scaled by the flow rate.
    pub fn effective_migrants(&self) -> f64 {
        (self.migrants as f64) * self.flow_rate
    }

    /// Calculate the expected allele frequency shift in the target
    /// population due to migration, given the source frequency.
    ///
    /// The shift is computed as: `flow_rate * (source_freq - target_freq)`.
    /// Returns the shift magnitude, clamped to [0.0, 1.0].
    pub fn allele_shift(&self, source_freq: f64, target_freq: f64) -> Result<f64, MigrationError> {
        if source_freq.is_nan() || source_freq < 0.0 || source_freq > 1.0 {
            return Err(MigrationError::InvalidGeneFlowRate);
        }
        if target_freq.is_nan() || target_freq < 0.0 || target_freq > 1.0 {
            return Err(MigrationError::InvalidGeneFlowRate);
        }
        let shift = self.flow_rate * (source_freq - target_freq);
        Ok(shift.abs().min(1.0))
    }

    /// Compute the equilibrium allele frequency in the target population
    /// under constant gene flow from the source.
    ///
    /// At equilibrium: `p_eq = (flow_rate * p_source) / (flow_rate + 1)`
    /// when considering a simple island model with symmetric migration.
    pub fn equilibrium_frequency(&self, source_freq: f64) -> Result<f64, MigrationError> {
        if source_freq.is_nan() || source_freq < 0.0 || source_freq > 1.0 {
            return Err(MigrationError::InvalidGeneFlowRate);
        }
        let denominator = self.flow_rate + 1.0;
        let p_eq = (self.flow_rate * source_freq) / denominator;
        Ok(p_eq)
    }

    /// Assess whether gene flow from source to target is sufficient
    /// to prevent genetic divergence (migration-drift balance).
    ///
    /// A common heuristic: Nm > 1 (more than one migrant per generation)
    /// is sufficient to prevent substantial divergence.
    pub fn prevents_divergence(&self) -> bool {
        self.effective_migrants() > 1.0
    }

    /// Compute the migration load: the reduction in mean fitness of the
    /// target population due to gene flow from a maladapted source.
    ///
    /// Migration load = `flow_rate * (1.0 - source_fitness)`.
    /// Assumes the target is locally adapted (fitness = 1.0) and the
    /// source has fitness `source_fitness` relative to the target.
    pub fn migration_load(&self, source_fitness: f64) -> Result<f64, MigrationError> {
        if source_fitness.is_nan() || source_fitness < 0.0 || source_fitness > 1.0 {
            return Err(MigrationError::InvalidGeneFlowRate);
        }
        Ok(self.flow_rate * (1.0 - source_fitness))
    }

    /// Simulate one round of gene flow, returning the number of
    /// migrants that successfully transferred and the resulting
    /// allele frequency shift given a source frequency.
    ///
    /// Returns zero migrants if the flow rate is zero.
    pub fn simulate_transfer(&self, source_freq: f64) -> Result<(usize, f64), MigrationError> {
        let actual_migrants = if self.flow_rate >= 1.0 {
            self.migrants
        } else {
            let draw = Self::pseudo_random_draw(self.source_population, self.target_population);
            if draw < self.flow_rate {
                self.migrants
            } else {
                0
            }
        };
        let shift = self.allele_shift(source_freq, 1.0)?;
        Ok((actual_migrants, shift))
    }

    /// Return the total genetic contribution, defined as migrants * flow_rate.
    pub fn total_genetic_contribution(&self) -> f64 {
        (self.migrants as f64) * self.flow_rate
    }

    /// Return a summary of the gene flow event.
    pub fn summary(&self) -> String {
        format!(
            "GeneFlow(source={}, target={}, rate={:.4}, migrants={}, effective={:.2})",
            self.source_population,
            self.target_population,
            self.flow_rate,
            self.migrants,
            self.effective_migrants(),
        )
    }

    fn pseudo_random_draw(source: u64, target: u64) -> f64 {
        let mut x = (source as u32).wrapping_mul(0x6C62272Eu32);
        x ^= (target as u32).wrapping_add(0x27D4EB2Fu32);
        x ^= x >> 16;
        x = x.wrapping_mul(0x45D9F3Bu32);
        x ^= x >> 16;
        (x as f64) / u32::MAX as f64
    }
}

impl Default for GeneFlow {
    fn default() -> Self {
        Self {
            source_population: 1,
            target_population: 2,
            flow_rate: 0.05,
            migrants: 5,
        }
    }
}