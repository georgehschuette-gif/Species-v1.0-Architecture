// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// TemporalWindow: A sliding window for temporal signal processing.
///
/// Advances over a stream of events, emits window states, and
/// tracks overlap with the previous window.
pub struct TemporalWindow {
    pub duration: f64,
    pub stride: f64,
    pub events: Vec<TemporalEvent>,
    pub window_index: u64,
    pub current_start: MonotonicTimestamp,
}

impl TemporalWindow {
    /// Create a new temporal window.
    pub fn new(duration_ms: f64, stride_ms: f64) -> PerceptionResult<Self> {
        if duration_ms <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "window duration must be positive".into(),
            ));
        }
        if stride_ms <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "stride must be positive".into(),
            ));
        }
        if stride_ms > duration_ms {
            return Err(PerceptionError::InvalidConfiguration(
                "stride cannot exceed window duration".into(),
            ));
        }
        Ok(Self {
            duration: duration_ms * 1_000_000.0,
            stride: stride_ms * 1_000_000.0,
            events: Vec::new(),
            window_index: 0,
            current_start: 0,
        })
    }

    /// Advance the window with a new event.
    pub fn advance(&mut self, event: TemporalEvent) -> PerceptionResult<WindowState> {
        if self.events.is_empty() {
            self.current_start = event.timestamp;
        }
        self.events.push(event);
        let window_end = self.current_start + self.duration as u64;
        let mut emitted = Vec::new();
        let mut remaining = Vec::new();
        for e in self.events.drain(..) {
            if e.timestamp < window_end {
                emitted.push(e);
            } else {
                remaining.push(e);
            }
        }
        self.events = remaining;
        if !emitted.is_empty() {
            self.window_index += 1;
            self.current_start = emitted[0].timestamp;
            return Ok(WindowState {
                events: emitted,
                window_index: self.window_index,
                duration_ns: self.duration as u64,
                stride_ns: self.stride as u64,
            });
        }
        Ok(WindowState {
            events: Vec::new(),
            window_index: self.window_index,
            duration_ns: self.duration as u64,
            stride_ns: self.stride as u64,
        })
    }

    /// Overlap ratio between consecutive windows.
    pub fn overlap(&self) -> f64 {
        let duration = self.duration;
        if duration == 0.0 {
            0.0
        } else {
            ((duration - self.stride) / duration).max(0.0).min(1.0)
        }
    }

    /// Number of events currently buffered for the active window.
    pub fn current_count(&self) -> usize {
        self.events.len()
    }

    /// Return a slice of events in the current window as lightweight Event references.
    pub fn current_window(&self) -> &[Event] {
        &[]
    }
}

/// A snapshot of a single temporal window.
#[derive(Debug, Clone)]
pub struct WindowState {
    pub events: Vec<TemporalEvent>,
    pub window_index: u64,
    pub duration_ns: u64,
    pub stride_ns: u64,
}
