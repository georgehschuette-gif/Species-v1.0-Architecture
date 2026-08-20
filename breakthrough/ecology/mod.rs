// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Ecology: The interaction dynamics between cognitive entities and their environment.
//!
//! This module models the ecosystem as an ecological system where concepts,
//! entities, and processes interact through competition, cooperation,
//! symbiosis, migration, adaptation, and succession.

pub mod species;
pub mod populations;
pub mod habitats;
pub mod migration;
pub mod adaptation;
pub mod extinction;
pub mod symbiosis;
pub mod competition;
pub mod cooperation;
pub mod succession;

pub use species::{Species, SpeciesRegistry, SpeciesError};
pub use populations::{Population, PopulationDynamics, PopulationError};
pub use habitats::{Habitat, HabitatMap, HabitatError};
pub use migration::{MigrationPattern, MigrationEngine, MigrationError};
pub use adaptation::{AdaptationMechanism, AdaptationResult, AdaptationError};
pub use extinction::{ExtinctionEvent, ExtinctionRisk, ExtinctionError};
pub use symbiosis::{SymbioticRelationship, SymbiosisType, SymbiosisError};
pub use competition::{CompetitionArena, CompetitionResult, CompetitionError};
pub use cooperation::{CooperativeGroup, CooperationProtocol, CooperationError};
pub use succession::{SuccessionStage, EcologicalSuccession, SuccessionError};
