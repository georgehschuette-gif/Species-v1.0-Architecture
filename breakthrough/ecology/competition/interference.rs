// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// InterferenceCompetition: Direct confrontation between competitors.
pub struct InterferenceCompetition {
    pub competitor_a: u64,
    pub competitor_b: u64,
    pub aggression: f64,
    pub territoriality: f64,
}

impl InterferenceCompetition {
    /// Constructs a new InterferenceCompetition with validated parameters.
    ///
    /// # Errors
    /// Returns `InvalidAggression` if aggression is outside [0.0, 1.0].
    /// Returns `InvalidTerritoriality` if territoriality is outside [0.0, 1.0].
    /// Returns `InvalidParticipant` if either competitor ID is zero.
    pub fn new(
        competitor_a: u64,
        competitor_b: u64,
        aggression: f64,
        territoriality: f64,
    ) -> Result<Self, CompetitionError> {
        if competitor_a == 0 {
            return Err(CompetitionError::InvalidParticipant(
                "Competitor A ID must be non-zero".to_string(),
            ));
        }
        if competitor_b == 0 {
            return Err(CompetitionError::InvalidParticipant(
                "Competitor B ID must be non-zero".to_string(),
            ));
        }
        if aggression.is_nan() || aggression.is_infinite() || !(0.0..=1.0).contains(&aggression) {
            return Err(CompetitionError::InvalidAggression(
                "Aggression must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        if territoriality.is_nan()
            || territoriality.is_infinite()
            || !(0.0..=1.0).contains(&territoriality)
        {
            return Err(CompetitionError::InvalidTerritoriality(
                "Territoriality must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        Ok(Self {
            competitor_a,
            competitor_b,
            aggression,
            territoriality,
        })
    }

    /// Validates the internal consistency of this interference competition.
    pub fn validate(&self) -> Result<(), CompetitionError> {
        if self.competitor_a == 0 {
            return Err(CompetitionError::InvalidParticipant(
                "Competitor A ID must be non-zero".to_string(),
            ));
        }
        if self.competitor_b == 0 {
            return Err(CompetitionError::InvalidParticipant(
                "Competitor B ID must be non-zero".to_string(),
            ));
        }
        if self.aggression.is_nan()
            || self.aggression.is_infinite()
            || !(0.0..=1.0).contains(&self.aggression)
        {
            return Err(CompetitionError::InvalidAggression(
                "Aggression must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        if self.territoriality.is_nan()
            || self.territoriality.is_infinite()
            || !(0.0..=1.0).contains(&self.territoriality)
        {
            return Err(CompetitionError::InvalidTerritoriality(
                "Territoriality must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        Ok(())
    }

    /// Determines the outcome of the interference competition.
    ///
    /// Competitor A is considered the initiator. Higher aggression favors A;
    /// higher territoriality favors B (defender advantage). If the weighted
    /// scores are equal, the result is a draw.
    pub fn outcome(&self) -> CompetitionResult {
        let score_a = self.aggression * (1.0 + self.territoriality);
        let score_b = (1.0 - self.aggression) * (1.0 + (1.0 - self.territoriality));
        if score_a > score_b {
            CompetitionResult::Win(self.competitor_a)
        } else if score_b > score_a {
            CompetitionResult::Win(self.competitor_b)
        } else {
            CompetitionResult::Draw(self.competitor_a, self.competitor_b)
        }
    }

    /// Returns the conflict intensity as a function of aggression and territoriality.
    pub fn conflict_intensity(&self) -> f64 {
        self.aggression * self.territoriality
    }

    /// Returns true if the competition is primarily territorial in nature.
    pub fn is_territorial(&self) -> bool {
        self.territoriality > 0.5
    }

    /// Returns the dominant competitor based on aggression level.
    pub fn dominant_competitor(&self) -> u64 {
        if self.aggression >= 0.5 {
            self.competitor_a
        } else {
            self.competitor_b
        }
    }
}

impl std::fmt::Display for InterferenceCompetition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "InterferenceCompetition(competitor_a={}, competitor_b={}, aggression={:.4}, territoriality={:.4})",
            self.competitor_a, self.competitor_b, self.aggression, self.territoriality
        )
    }
}