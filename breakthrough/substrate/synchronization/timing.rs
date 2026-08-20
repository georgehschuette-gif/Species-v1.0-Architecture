// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.


/// CollectiveTiming: The synchronized temporal coordination of multiple entities.
///
/// Tracks a group of entities and measures the quality
/// of their synchronization. Higher quality indicates
/// better collective alignment.
#[derive(Debug, Clone, PartialEq)]
pub struct CollectiveTiming {
    pub entities: Vec<u64>,
    pub synchronization_quality: f64,
}

impl CollectiveTiming {
    /// The minimum valid quality.
    pub const MIN_QUALITY: f64 = 0.0;
    /// The maximum valid quality.
    pub const MAX_QUALITY: f64 = 1.0;

    /// Creates a new collective timing configuration.
    ///
    /// Quality is clamped to [0, 1].
    ///
    /// # Errors
    /// Returns `TimingError::InvalidQuality` if quality is NaN or outside [0, 1].
    /// Returns `TimingError::EmptyEntityList` if no entities are provided.
    pub fn new(entities: Vec<u64>, quality: f64) -> Result<Self, TimingError> {
        if entities.is_empty() {
            return Err(TimingError::EmptyEntityList);
        }
        if quality.is_nan() || quality < Self::MIN_QUALITY || quality > Self::MAX_QUALITY {
            return Err(TimingError::InvalidQuality { quality });
        }
        Ok(Self {
            entities,
            synchronization_quality: quality.clamp(Self::MIN_QUALITY, Self::MAX_QUALITY),
        })
    }

    /// Returns the number of synced entities.
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// Returns whether the timing group is valid (has entities).
    pub fn is_valid(&self) -> bool {
        !self.entities.is_empty()
    }

    /// Adds an entity to the timing group.
    ///
    /// Does nothing if the entity is already present.
    pub fn add_entity(&mut self, entity: u64) {
        if !self.entities.contains(&entity) {
            self.entities.push(entity);
        }
    }

    /// Removes an entity from the timing group.
    ///
    /// Returns whether the entity was found and removed.
    pub fn remove_entity(&mut self, entity: u64) -> bool {
        if let Some(index) = self.entities.iter().position(|&e| e == entity) {
            self.entities.remove(index);
            true
        } else {
            false
        }
    }

    /// Checks whether the given entity is part of this timing group.
    pub fn has_entity(&self, entity: u64) -> bool {
        self.entities.contains(&entity)
    }

    /// Updates the synchronization quality, clamped to [0, 1].
    ///
    /// # Errors
    /// Returns `TimingError::InvalidQuality` if quality is NaN.
    pub fn set_quality(&mut self, quality: f64) -> Result<(), TimingError> {
        if quality.is_nan() {
            return Err(TimingError::InvalidQuality { quality });
        }
        self.synchronization_quality = quality.clamp(Self::MIN_QUALITY, Self::MAX_QUALITY);
        Ok(())
    }

    /// Returns the average quality per entity.
    pub fn quality_per_entity(&self) -> f64 {
        if self.entities.is_empty() {
            0.0
        } else {
            self.synchronization_quality / self.entities.len() as f64
        }
    }

    /// Checks whether synchronization quality exceeds the given threshold.
    pub fn is_synchronized(&self, threshold: f64) -> bool {
        self.synchronization_quality >= threshold
    }

    /// Computes a weighted quality where more entities improve the effective quality.
    ///
    /// The formula is `quality * log2(1 + n)` where `n` is the entity count.
    pub fn effective_quality(&self) -> f64 {
        let n = self.entities.len() as f64;
        if n == 0.0 {
            0.0
        } else {
            self.synchronization_quality * (1.0 + n).log2()
        }
    }

    /// Merges two collective timing groups.
    pub fn merge(&mut self, other: &CollectiveTiming) -> Result<(), TimingError> {
        for &entity in &other.entities {
            self.add_entity(entity);
        }
        let combined_quality =
            (self.synchronization_quality + other.synchronization_quality) * 0.5;
        self.set_quality(combined_quality)
    }
}

impl Default for CollectiveTiming {
    fn default() -> Self {
        Self {
            entities: Vec::new(),
            synchronization_quality: 0.0,
        }
    }
}

/// Error type for collective timing failures.
#[derive(Debug, Clone, PartialEq)]
pub enum TimingError {
    /// The quality is NaN or outside [0, 1].
    InvalidQuality { quality: f64 },
    /// No entities in the group.
    EmptyEntityList,
}

impl std::fmt::Display for TimingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimingError::InvalidQuality { quality } => {
                write!(f, "Invalid quality {}: must be in [0.0, 1.0]", quality)
            }
            TimingError::EmptyEntityList => {
                write!(f, "Cannot create collective timing with no entities")
            }
        }
    }
}

impl std::error::Error for TimingError {}