// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::ImaginationError;

pub mod timeline;
pub mod branching;

pub use timeline::Timeline;
pub use branching::FutureHistory;

pub const DEFAULT_TIMELINE_HORIZON: usize = 50;
pub const MAX_FUTURE_BRANCHES: usize = 16;

pub fn create_timeline(name: impl Into<String>) -> Timeline {
    Timeline::new(name)
}

pub fn create_future_history(start_year: u64) -> FutureHistory {
    FutureHistory::new(start_year)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn timeline_creation() {
        let t = create_timeline("Main");
        assert_eq!(t.name(), "Main");
    }
}
