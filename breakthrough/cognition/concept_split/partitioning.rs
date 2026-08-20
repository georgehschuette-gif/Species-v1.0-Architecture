// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// ConceptPartitioning: Dividing a concept into disjoint sub-concepts.
///
/// Partitioning decomposes a broad concept into mutually exclusive
/// sub-concepts, each covering a distinct region of the concept's
/// feature space. The balance factor ensures approximately equal
/// distribution of sub-concept coverage.
///
/// # Fields
/// - `partition_count`: Number of sub-concepts to create, in [1, usize::MAX].
/// - `balance_factor`: Desired evenness of partition sizes, in [0.0, 1.0].
///
/// # Example
/// ```
/// use breakthrough::cognition::concept_split::ConceptPartitioning;
///
/// let partitioner = ConceptPartitioning::new(4, 0.8).expect("valid parameters");
/// assert_eq!(partitioner.partitions(), 4);
/// ```
pub struct ConceptPartitioning {
    /// Number of sub-concepts to create, in [1, 1000].
    pub partition_count: usize,
    /// Desired evenness of partition sizes, in [0.0, 1.0].
    pub balance_factor: f64,
}

impl ConceptPartitioning {
    /// Creates a new `ConceptPartitioning` with the given count and balance.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `partition_count` is zero
    /// or exceeds 1000, or if `balance_factor` is outside [0.0, 1.0].
    pub fn new(partition_count: usize, balance_factor: f64) -> Result<Self, CognitionError> {
        if partition_count == 0 || partition_count > 1000 {
            return Err(CognitionError::OutOfRange {
                field: "partition_count".to_string(),
                value: partition_count as f64,
                min: 1.0,
                max: 1000.0,
            });
        }
        if !(0.0..=1.0).contains(&balance_factor) {
            return Err(CognitionError::OutOfRange {
                field: "balance_factor".to_string(),
                value: balance_factor,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            partition_count,
            balance_factor,
        })
    }

    /// Computes the size of a partition at the given index,
    /// distributing the total `item_count` across `partition_count` bins.
    ///
    /// Returns the number of items assigned to the partition at `index`.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `index` is >= partition_count,
    /// or `item_count` is negative (represented as a very large f64).
    pub fn partition_size(
        &self,
        index: usize,
        item_count: usize,
    ) -> Result<usize, CognitionError> {
        if index >= self.partition_count {
            return Err(CognitionError::OutOfRange {
                field: "partition_index".to_string(),
                value: index as f64,
                min: 0.0,
                max: self.partition_count as f64 - 1.0,
            });
        }
        let base_size = item_count / self.partition_count;
        let remainder = item_count % self.partition_count;
        let size = if index < remainder {
            base_size + 1
        } else {
            base_size
        };
        let imbalance = if self.partition_count > 1 {
            let max_size = if remainder > 0 {
                base_size + 1
            } else {
                base_size
            };
            let min_size = base_size;
            (max_size as f64 - min_size as f64) / (max_size as f64 + f64::EPSILON)
        } else {
            0.0
        };
        if imbalance > 1.0 - self.balance_factor && item_count > self.partition_count {
            // Redistribution needed for better balance
            let adjusted = (item_count as f64 / self.partition_count as f64).ceil() as usize;
            return Ok(adjusted.min(item_count));
        }
        Ok(size)
    }

    /// Returns the number of partitions.
    pub fn partitions(&self) -> usize {
        self.partition_count
    }

    /// Computes the expected coverage ratio for partition at `index`.
    pub fn coverage_ratio(&self, index: usize, item_count: usize) -> Result<f64, CognitionError> {
        let size = self.partition_size(index, item_count)?;
        if item_count == 0 {
            return Ok(0.0);
        }
        Ok(size as f64 / item_count as f64)
    }

    /// Adjusts the partition count.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if the new count is zero or exceeds 1000.
    pub fn set_partition_count(&mut self, new_count: usize) -> Result<(), CognitionError> {
        if new_count == 0 || new_count > 1000 {
            return Err(CognitionError::OutOfRange {
                field: "partition_count".to_string(),
                value: new_count as f64,
                min: 1.0,
                max: 1000.0,
            });
        }
        self.partition_count = new_count;
        Ok(())
    }

    /// Validates all field values are within valid ranges.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if self.partition_count == 0 || self.partition_count > 1000 {
            return Err(CognitionError::OutOfRange {
                field: "partition_count".to_string(),
                value: self.partition_count as f64,
                min: 1.0,
                max: 1000.0,
            });
        }
        if !(0.0..=1.0).contains(&self.balance_factor) {
            return Err(CognitionError::OutOfRange {
                field: "balance_factor".to_string(),
                value: self.balance_factor,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}

impl fmt::Debug for ConceptPartitioning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConceptPartitioning")
            .field("partition_count", &self.partition_count)
            .field("balance_factor", &self.balance_factor)
            .finish()
    }
}