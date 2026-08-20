// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::ImaginationError;

pub mod fabrication;
pub mod recollection;

pub use fabrication::MemoryFabrication;
pub use recollection::SyntheticMemory;

pub const DEFAULT_MEMORY_VIVIDNESS: f64 = 0.5;
pub const MAX_MEMORY_AGE: usize = 1000;

pub fn create_memory_fabrication(source_count: usize) -> MemoryFabrication {
    MemoryFabrication::new(source_count)
}

pub fn create_synthetic_memory(content: impl Into<String>) -> SyntheticMemory {
    SyntheticMemory::new(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn memory_fabrication_creation() {
        let f = create_memory_fabrication(3);
        assert_eq!(f.source_count(), 3);
    }
}
