// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Species: The classification and definition of cognitive entity types.
//! Defines how different kinds of cognitive entities are categorized,
//! differentiated, and evolved.

use std::collections::HashMap;
use std::fmt;
use std::error::Error;

pub mod classification;
pub mod taxonomy;
pub mod variation;

/// SpeciesError: Error types for species operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpeciesError {
    DuplicateSpecies(SpeciesId),
    SpeciesNotFound(SpeciesId),
    InvalidClassification(String),
    InvalidTaxonomy(String),
    InvalidVariation(String),
    InvalidState(String),
}

impl fmt::Display for SpeciesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpeciesError::DuplicateSpecies(id) => {
                write!(f, "Duplicate species registration: id={}", id.0)
            }
            SpeciesError::SpeciesNotFound(id) => {
                write!(f, "Species not found: id={}", id.0)
            }
            SpeciesError::InvalidClassification(msg) => {
                write!(f, "Invalid classification: {}", msg)
            }
            SpeciesError::InvalidTaxonomy(msg) => {
                write!(f, "Invalid taxonomy: {}", msg)
            }
            SpeciesError::InvalidVariation(msg) => {
                write!(f, "Invalid variation: {}", msg)
            }
            SpeciesError::InvalidState(msg) => {
                write!(f, "Invalid state: {}", msg)
            }
        }
    }
}

impl Error for SpeciesError {}

/// Species: A cognitive entity type with classification, taxonomy, and variation data.
pub struct Species {
    pub id: SpeciesId,
    pub name: String,
    pub classifier: SpeciesClassifier,
    pub taxonomy: CognitiveTaxonomy,
    pub variation: SpeciesVariation,
}

impl Species {
    /// Constructs a new Species with the given identity, classifier, taxonomy, and variation.
    ///
    /// Validates that the species name is non-empty and all sub-components are internally consistent.
    pub fn new(
        id: SpeciesId,
        name: String,
        classifier: SpeciesClassifier,
        taxonomy: CognitiveTaxonomy,
        variation: SpeciesVariation,
    ) -> Result<Self, SpeciesError> {
        if name.trim().is_empty() {
            return Err(SpeciesError::InvalidClassification(
                "Species name must not be empty".to_string(),
            ));
        }
        classifier.validate()?;
        taxonomy.validate()?;
        variation.validate()?;
        Ok(Self {
            id,
            name,
            classifier,
            taxonomy,
            variation,
        })
    }

    /// Returns the species identifier.
    pub fn id(&self) -> SpeciesId {
        self.id
    }

    /// Returns the species name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a reference to the species classifier.
    pub fn classifier(&self) -> &SpeciesClassifier {
        &self.classifier
    }

    /// Returns a reference to the species taxonomy.
    pub fn taxonomy(&self) -> &CognitiveTaxonomy {
        &self.taxonomy
    }

    /// Returns a reference to the species variation data.
    pub fn variation(&self) -> &SpeciesVariation {
        &self.variation
    }

    /// Returns the taxonomic rank of this species if present in the taxonomy.
    pub fn taxonomic_rank(&self) -> Option<TaxonomicRank> {
        self.taxonomy.find_species(self.id).map(|(rank, _)| rank)
    }

    /// Computes a composite stability score based on variation stability and classification confidence.
    pub fn stability(&self) -> f64 {
        self.variation.stability() * (self.classifier.classification_confidence() as f64)
    }
}

/// SpeciesRegistry: A registry of species with classification and lookup capabilities.
pub struct SpeciesRegistry {
    entries: HashMap<SpeciesId, Species>,
}

impl SpeciesRegistry {
    /// Constructs an empty SpeciesRegistry.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Registers a species in the registry.
    ///
    /// Returns an error if a species with the same ID already exists.
    pub fn register(&mut self, species: Species) -> Result<(), SpeciesError> {
        if self.entries.contains_key(&species.id) {
            return Err(SpeciesError::DuplicateSpecies(species.id));
        }
        self.entries.insert(species.id, species);
        Ok(())
    }

    /// Retrieves a species by its identifier, if present.
    pub fn get(&self, id: SpeciesId) -> Option<&Species> {
        self.entries.get(&id)
    }

    /// Retrieves a mutable reference to a species by its identifier, if present.
    pub fn get_mut(&mut self, id: SpeciesId) -> Option<&mut Species> {
        self.entries.get_mut(&id)
    }

    /// Removes a species from the registry by its identifier.
    ///
    /// Returns an error if the species does not exist.
    pub fn remove(&mut self, id: SpeciesId) -> Result<Species, SpeciesError> {
        self.entries
            .remove(&id)
            .ok_or(SpeciesError::SpeciesNotFound(id))
    }

    /// Returns an iterator over all registered species.
    pub fn list(&self) -> Vec<&Species> {
        self.entries.values().collect()
    }

    /// Attempts to classify a set of feature vectors and returns the matched SpeciesId.
    ///
    /// Uses the internal classifier to find the best-matching species.
    pub fn classify(&self, features: &[f64]) -> Result<SpeciesId, SpeciesError> {
        if features.is_empty() {
            return Err(SpeciesError::InvalidClassification(
                "Feature vector must not be empty".to_string(),
            ));
        }
        let mut best_id = None;
        let mut best_score = f64::NEG_INFINITY;
        for species in self.entries.values() {
            let score = species.classifier.compute_similarity(features);
            if score > best_score {
                best_score = score;
                best_id = Some(species.id);
            }
        }
        best_id.ok_or_else(|| {
            SpeciesError::InvalidClassification("No species registered".to_string())
        })
    }

    /// Returns the number of species in the registry.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if the registry contains no species.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Finds a species by name. Returns the first match.
    pub fn find_by_name(&self, name: &str) -> Option<&Species> {
        self.entries.values().find(|s| s.name == name)
    }
}

impl Default for SpeciesRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub use classification::SpeciesClassifier;
pub use taxonomy::CognitiveTaxonomy;
pub use taxonomy::TaxonomicRank;
pub use variation::SpeciesVariation;

/// SpeciesId: A unique identifier for a cognitive species.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpeciesId(pub u64);

impl SpeciesId {
    /// Constructs a new SpeciesId from a raw u64 value.
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Returns the raw u64 value.
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    /// Checks if this ID matches another.
    pub fn is_same(&self, other: SpeciesId) -> bool {
        self.0 == other.0
    }
}