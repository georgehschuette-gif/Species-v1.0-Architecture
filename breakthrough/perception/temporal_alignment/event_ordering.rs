// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// EventOrdering: Establishes causal and temporal order of perceived events.
///
/// Maintains a bounded ring of recent events, sorts by timestamp,
/// and flushes ordered batches when full.
pub struct EventOrdering {
    pub buffer_size: usize,
    pub events: Vec<TemporalEvent>,
    pub flush_count: usize,
}

impl EventOrdering {
    /// Create a new ordering buffer.
    pub fn new(buffer_size: usize) -> PerceptionResult<Self> {
        if buffer_size == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "buffer size must be positive".into(),
            ));
        }
        Ok(Self {
            buffer_size,
            events: Vec::with_capacity(buffer_size),
            flush_count: 0,
        })
    }

    /// Insert an event and return a sorted batch if the buffer is full.
    pub fn insert(&mut self, event: TemporalEvent) -> PerceptionResult<Vec<TemporalEvent>> {
        self.events.push(event);
        if self.events.len() >= self.buffer_size {
            let mut batch = self.events.clone();
            batch.sort_by(|a, b| a.cmp(b));
            self.flush_count += 1;
            self.events.clear();
            Ok(batch)
        } else {
            Ok(Vec::new())
        }
    }

    /// Peek at the current ordered set without flushing.
    pub fn ordered_peek(&self) -> PerceptionResult<Vec<TemporalEvent>> {
        if self.events.is_empty() {
            return Err(AlignmentError::EmptyBuffer.into());
        }
        let mut sorted = self.events.clone();
        sorted.sort_by(|a, b| a.cmp(b));
        Ok(sorted)
    }

    /// Number of events currently buffered.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Whether the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Clear all buffered events.
    pub fn clear(&mut self) {
        self.events.clear();
    }

    /// Return a sorted batch of events as lightweight Event references.
    pub fn ordered_batch(&self) -> Vec<Event> {
        self.ordered_peek()
            .unwrap_or_default()
            .into_iter()
            .map(|e| Event {
                id: 0,
                timestamp: e.timestamp,
                payload: e.payload.into_iter().map(|v| v as f64).collect(),
                source_channel: e.channel,
            })
            .collect()
    }
}
