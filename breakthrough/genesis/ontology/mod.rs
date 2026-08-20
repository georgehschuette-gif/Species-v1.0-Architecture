// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Ontology: The fundamental types and category structures of the ecosystem.
//! Defines what exists and how entities relate to one another.

pub mod entity;
pub mod relation;
pub mod category;
pub mod field;

pub use entity::Entity;
pub use entity::EntityState;
pub use relation::Relation;
pub use relation::RelationType;
pub use category::Category;
pub use category::CategoryDimension;
pub use field::CognitiveField;