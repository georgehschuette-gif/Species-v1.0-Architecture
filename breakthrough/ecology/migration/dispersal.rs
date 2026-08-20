// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// DispersalMechanism: The process by which entities spread across habitats.
pub struct DispersalMechanism {
    pub mechanism_type: DispersalType,
    pub distance_mean: f64,
    pub distance_variance: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispersalType {
    Active,
    Passive,
    Directed,
    Random,
}

impl DispersalMechanism {
    /// Construct a new [DispersalMechanism] with the given parameters.
    ///
    /// # Errors
    /// Returns [MigrationError::InvalidDispersalDistance] if `distance_mean`
    /// or `distance_variance` is negative or NaN.
    pub fn new(
        mechanism_type: DispersalType,
        distance_mean: f64,
        distance_variance: f64,
    ) -> Result<Self, MigrationError> {
        if distance_mean.is_nan() || distance_mean < 0.0 {
            return Err(MigrationError::InvalidDispersalDistance);
        }
        if distance_variance.is_nan() || distance_variance < 0.0 {
            return Err(MigrationError::InvalidDispersalDistance);
        }
        if distance_variance > distance_mean * distance_mean && distance_mean > 0.0 {
            return Err(MigrationError::ContradictoryParameters);
        }
        Ok(Self {
            mechanism_type,
            distance_mean,
            distance_variance,
        })
    }

    /// Validate the dispersal mechanism's parameters.
    ///
    /// Checks that distance mean and variance are non-negative and finite,
    /// and that the variance does not exceed the square of the mean
    /// (which would indicate an inconsistent distribution).
    pub fn validate(&self) -> Result<(), MigrationError> {
        if self.distance_mean.is_nan() || self.distance_mean < 0.0 {
            return Err(MigrationError::InvalidDispersalDistance);
        }
        if self.distance_variance.is_nan() || self.distance_variance < 0.0 {
            return Err(MigrationError::InvalidDispersalDistance);
        }
        if self.distance_variance > self.distance_mean * self.distance_mean
            && self.distance_mean > 0.0
        {
            return Err(MigrationError::ContradictoryParameters);
        }
        Ok(())
    }

    /// Compute the standard deviation of the dispersal distance.
    pub fn distance_std_dev(&self) -> f64 {
        self.distance_variance.sqrt()
    }

    /// Sample a dispersal distance from the mechanism's distribution.
    ///
    /// Uses a deterministic approximation: for zero variance, returns the mean.
    /// Otherwise, samples from a half-normal distribution centered at the mean.
    pub fn sample_distance(&self) -> f64 {
        if self.distance_variance == 0.0 {
            return self.distance_mean;
        }
        let std_dev = self.distance_std_dev();
        let raw = Self::pseudo_random_sample();
        let half_normal = (raw * std_dev).abs();
        self.distance_mean + half_normal
    }

    /// Compute the dispersal probability for a given distance,
    /// assuming the mechanism's distribution parameters.
    pub fn probability_at_distance(&self, distance: f64) -> Result<f64, MigrationError> {
        if distance.is_nan() || distance < 0.0 {
            return Err(MigrationError::InvalidDispersalDistance);
        }
        if self.distance_mean == 0.0 && self.distance_variance == 0.0 {
            return if distance == 0.0 { Ok(1.0) } else { Ok(0.0) };
        }
        let std_dev = self.distance_std_dev();
        if std_dev == 0.0 {
            return Ok(if (distance - self.distance_mean).abs() < f64::EPSILON {
                1.0
            } else {
                0.0
            });
        }
        let z = (distance - self.distance_mean) / std_dev;
        let probability = (-0.5 * z * z).exp();
        Ok(probability)
    }

    /// Execute a dispersal event, returning the distance traveled and
    /// whether the entity remained within its home habitat.
    ///
    /// A dispersal is considered "home" if the sampled distance is below
    /// the mean distance.
    pub fn execute(&self) -> (f64, bool) {
        let distance = self.sample_distance();
        let at_home = distance < self.distance_mean;
        (distance, at_home)
    }

    /// Return the maximum likely dispersal distance (mean + 3 * std_dev).
    pub fn max_likely_distance(&self) -> f64 {
        let std_dev = self.distance_std_dev();
        self.distance_mean + 3.0 * std_dev
    }

    /// Return the minimum likely dispersal distance (max of 0 and mean - 3 * std_dev).
    pub fn min_likely_distance(&self) -> f64 {
        let std_dev = self.distance_std_dev();
        (self.distance_mean - 3.0 * std_dev).max(0.0)
    }

    fn pseudo_random_sample() -> f64 {
        let mut x = 0xDEADBEEFu32;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        x = x.wrapping_add(1);
        (x as f64) / u32::MAX as f64
    }
}

impl DispersalType {
    /// Return a human-readable label for this dispersal type.
    pub fn label(self) -> &'static str {
        match self {
            DispersalType::Active => "active",
            DispersalType::Passive => "passive",
            DispersalType::Directed => "directed",
            DispersalType::Random => "random",
        }
    }

    /// Return whether this dispersal type involves intentional movement.
    pub fn is_intentional(self) -> bool {
        matches!(self, DispersalType::Active | DispersalType::Directed)
    }

    /// Return whether this dispersal type is stochastic.
    pub fn is_stochastic(self) -> bool {
        matches!(self, DispersalType::Random | DispersalType::Passive)
    }
}

impl Default for DispersalMechanism {
    fn default() -> Self {
        Self {
            mechanism_type: DispersalType::Random,
            distance_mean: 1.0,
            distance_variance: 0.0,
        }
    }
}