// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// ClimaxCommunity: The stable, final stage of ecological succession.
pub struct ClimaxCommunity {
    pub dominant_species: Vec<u64>,
    pub diversity_index: f64,
    pub stability_score: f64,
    pub resilience: f64,
}

impl ClimaxCommunity {
    /// Constructs a new ClimaxCommunity with validated parameters.
    ///
    /// # Errors
    /// Returns `NoPioneerSpecies` if dominant_species is empty.
    /// Returns `InvalidProgress` if diversity_index is outside [0.0, 1.0].
    /// Returns `InvalidStability` if stability_score is outside [0.0, 1.0].
    /// Returns `InvalidTrust` if resilience is outside [0.0, 1.0].
    /// Returns `InvalidParticipant` if any species ID is zero.
    pub fn new(
        dominant_species: Vec<u64>,
        diversity_index: f64,
        stability_score: f64,
        resilience: f64,
    ) -> Result<Self, SuccessionError> {
        if dominant_species.is_empty() {
            return Err(SuccessionError::NoPioneerSpecies(
                "At least one dominant species is required".to_string(),
            ));
        }
        if diversity_index.is_nan()
            || diversity_index.is_infinite()
            || !(0.0..=1.0).contains(&diversity_index)
        {
            return Err(SuccessionError::InvalidProgress(
                "Diversity index must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        if stability_score.is_nan()
            || stability_score.is_infinite()
            || !(0.0..=1.0).contains(&stability_score)
        {
            return Err(SuccessionError::InvalidStability(
                "Stability score must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        if resilience.is_nan()
            || resilience.is_infinite()
            || !(0.0..=1.0).contains(&resilience)
        {
            return Err(SuccessionError::InvalidTrust(
                "Resilience must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        for (i, &id) in dominant_species.iter().enumerate() {
            if id == 0 {
                return Err(SuccessionError::InvalidParticipant(
                    format!("Dominant species at index {} has zero ID", i),
                ));
            }
        }
        Ok(Self {
            dominant_species,
            diversity_index,
            stability_score,
            resilience,
        })
    }

    /// Validates the internal consistency of this climax community.
    pub fn validate(&self) -> Result<(), SuccessionError> {
        if self.dominant_species.is_empty() {
            return Err(SuccessionError::NoPioneerSpecies(
                "At least one dominant species is required".to_string(),
            ));
        }
        if self.diversity_index.is_nan()
            || self.diversity_index.is_infinite()
            || !(0.0..=1.0).contains(&self.diversity_index)
        {
            return Err(SuccessionError::InvalidProgress(
                "Diversity index must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        if self.stability_score.is_nan()
            || self.stability_score.is_infinite()
            || !(0.0..=1.0).contains(&self.stability_score)
        {
            return Err(SuccessionError::InvalidStability(
                "Stability score must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        if self.resilience.is_nan()
            || self.resilience.is_infinite()
            || !(0.0..=1.0).contains(&self.resilience)
        {
            return Err(SuccessionError::InvalidTrust(
                "Resilience must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        for &id in &self.dominant_species {
            if id == 0 {
                return Err(SuccessionError::InvalidParticipant(
                    "Dominant species ID must be non-zero".to_string(),
                ));
            }
        }
        Ok(())
    }

    /// Returns true if the community is stable (stability_score > 0.7).
    pub fn is_stable(&self) -> bool {
        self.stability_score > 0.7
    }

    /// Returns the resilience score adjusted by diversity.
    pub fn resilience_score(&self) -> f64 {
        self.resilience * self.diversity_index
    }

    /// Returns the species richness (number of dominant species).
    pub fn species_richness(&self) -> usize {
        self.dominant_species.len()
    }

    /// Returns the diversity index.
    pub fn diversity(&self) -> f64 {
        self.diversity_index
    }

    /// Returns the stability score.
    pub fn stability(&self) -> f64 {
        self.stability_score
    }

    /// Returns the resilience value.
    pub fn resilience_value(&self) -> f64 {
        self.resilience
    }
}

impl std::fmt::Display for ClimaxCommunity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ClimaxCommunity(species={}, diversity={:.4}, stability={:.4}, resilience={:.4})",
            self.dominant_species.len(),
            self.diversity_index,
            self.stability_score,
            self.resilience
        )
    }
}