// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

pub struct UnknownRegions {
    pub region_count: u64,
    pub next_id: u64,
    pub mapped_cells: u64,
}

impl UnknownRegions {
    pub fn new() -> Self {
        Self {
            region_count: 0,
            next_id: 1,
            mapped_cells: 0,
        }
    }

    pub fn register(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.region_count += 1;
        id
    }

    pub fn map_cells(&mut self, count: u64) -> u64 {
        self.mapped_cells += count;
        self.mapped_cells
    }

    pub fn unregistered_regions(&self) -> u64 {
        self.region_count
    }

    pub fn map_density(&self) -> f64 {
        if self.mapped_cells == 0 {
            0.0
        } else {
            self.region_count as f64 / self.mapped_cells as f64
        }
    }
}

impl Default for UnknownRegions {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for UnknownRegions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UnknownRegions(regions={}, mapped_cells={})",
            self.region_count, self.mapped_cells
        )
    }
}
