// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// SpeciesVariation: Tracks phenotypic and genotypic variation within a species.
pub struct SpeciesVariation {
    pub genetic_diversity: f64,
    pub phenotypic_range: (f64, f64),
    pub mutation_rate: f64,
}

impl SpeciesVariation {
    /// Constructs a new SpeciesVariation after validating all fields.
    ///
    /// `genetic_diversity` and `mutation_rate` must be in [0.0, 1.0].
    /// `phenotypic_range` must have the minimum value less than or equal to the maximum.
    pub fn new(
        genetic_diversity: f64,
        phenotypic_range: (f64, f64),
        mutation_rate: f64,
    ) -> Result<Self, SpeciesError> {
        let variation = Self {
            genetic_diversity,
            phenotypic_range,
            mutation_rate,
        };
        variation.validate()?;
        Ok(variation)
    }

    /// Validates the variation data.
    ///
    /// Returns an error if genetic_diversity or mutation_rate is outside [0.0, 1.0],
    /// contains NaN or Inf, or if phenotypic_range has min > max.
    pub fn validate(&self) -> Result<(), SpeciesError> {
        if self.genetic_diversity.is_nan() {
            return Err(SpeciesError::InvalidVariation(
                "genetic_diversity cannot be NaN".to_string(),
            ));
        }
        if self.genetic_diversity.is_infinite() {
            return Err(SpeciesError::InvalidVariation(
                "genetic_diversity cannot be infinite".to_string(),
            ));
        }
        if !(0.0..=1.0).contains(&self.genetic_diversity) {
            return Err(SpeciesError::InvalidVariation(format!(
                "genetic_diversity must be in [0.0, 1.0], got {}",
                self.genetic_diversity
            )));
        }
        if self.mutation_rate.is_nan() {
            return Err(SpeciesError::InvalidVariation(
                "mutation_rate cannot be NaN".to_string(),
            ));
        }
        if self.mutation_rate.is_infinite() {
            return Err(SpeciesError::InvalidVariation(
                "mutation_rate cannot be infinite".to_string(),
            ));
        }
        if !(0.0..=1.0).contains(&self.mutation_rate) {
            return Err(SpeciesError::InvalidVariation(format!(
                "mutation_rate must be in [0.0, 1.0], got {}",
                self.mutation_rate
            )));
        }
        let (min, max) = self.phenotypic_range;
        if min.is_nan() || max.is_nan() {
            return Err(SpeciesError::InvalidVariation(
                "Phenotypic range bounds cannot be NaN".to_string(),
            ));
        }
        if min.is_infinite() || max.is_infinite() {
            return Err(SpeciesError::InvalidVariation(
                "Phenotypic range bounds cannot be infinite".to_string(),
            ));
        }
        if min > max {
            return Err(SpeciesError::InvalidVariation(format!(
                "Phenotypic range min ({}) must not exceed max ({})",
                min, max
            )));
        }
        Ok(())
    }

    /// Returns the width of the phenotypic range.
    pub fn phenotypic_width(&self) -> f64 {
        let (min, max) = self.phenotypic_range;
        max - min
    }

    /// Computes the expected number of mutations over the given number of generations.
    ///
    /// Returns 0 if generations is 0 or if mutation_rate is 0.0.
    pub fn expected_mutations(&self, generations: u64) -> u64 {
        if generations == 0 || self.mutation_rate == 0.0 {
            return 0;
        }
        let expected = generations as f64 * self.mutation_rate;
        expected.round() as u64
    }

    /// Returns a stability score in [0.0, 1.0].
    ///
    /// Higher stability means less genetic diversity and lower mutation rate,
    /// indicating a more stable phenotype.
    pub fn stability(&self) -> f64 {
        1.0 - (self.genetic_diversity * 0.6 + self.mutation_rate * 0.4)
    }

    /// Merges this variation with another, producing a new variation that
    /// represents the combined state.
    ///
    /// The merged genetic_diversity and mutation_rate are the arithmetic means.
    /// The merged phenotypic_range spans from the global minimum to the global maximum.
    ///
    /// Returns an error if the merge produces invalid values.
    pub fn merge(&self, other: &Self) -> Result<Self, SpeciesError> {
        let merged_diversity = (self.genetic_diversity + other.genetic_diversity) / 2.0;
        let merged_mutation = (self.mutation_rate + other.mutation_rate) / 2.0;
        let min = self.phenotypic_range.0.min(other.phenotypic_range.0);
        let max = self.phenotypic_range.1.max(other.phenotypic_range.1);
        let merged = Self::new(merged_diversity, (min, max), merged_mutation)?;
        Ok(merged)
    }

    /// Clamps all numeric fields to their valid ranges in place.
    pub fn normalize(&mut self) {
        self.genetic_diversity = self.genetic_diversity.clamp(0.0, 1.0);
        self.mutation_rate = self.mutation_rate.clamp(0.0, 1.0);
        let (min, max) = self.phenotypic_range;
        if min > max {
            self.phenotypic_range = (max, min);
        }
        if self.phenotypic_range.0.is_nan() {
            self.phenotypic_range.0 = 0.0;
        }
        if self.phenotypic_range.1.is_nan() {
            self.phenotypic_range.1 = 0.0;
        }
    }

    /// Returns a divergence score between this variation and another.
    ///
    /// The score is a weighted combination of genetic diversity difference,
    /// mutation rate difference, and phenotypic width difference.
    /// Values near 0.0 indicate high similarity; values near 1.0 indicate high divergence.
    pub fn divergence(&self, other: &Self) -> f64 {
        let diversity_diff = (self.genetic_diversity - other.genetic_diversity).abs();
        let mutation_diff = (self.mutation_rate - other.mutation_rate).abs();
        let width_self = self.phenotypic_width();
        let width_other = other.phenotypic_width();
        let width_diff = ((width_self - width_other) / (width_self + width_other + f64::EPSILON)).abs();
        (diversity_diff * 0.4 + mutation_diff * 0.3 + width_diff * 0.3).min(1.0)
    }
}