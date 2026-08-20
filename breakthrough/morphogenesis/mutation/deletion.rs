// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct DeletionMutation {
    pub start: usize,
    pub end: usize,
    pub deleted_length: usize,
}

impl DeletionMutation {
    pub fn new(start: usize, end: usize) -> Result<Self, MorphogenesisError> {
        if end < start { return Err(MorphogenesisError::OutOfRange { field: "end".into(), value: end as f64, min: start as f64, max: f64::INFINITY }); }
        Ok(Self { start, end, deleted_length: end - start })
    }

    pub fn range(&self) -> (usize, usize) { (self.start, self.end) }
    pub fn length(&self) -> usize { self.deleted_length }
    pub fn is_frameshift(&self) -> bool { self.deleted_length % 3 != 0 }
    pub fn contains(&self, position: usize) -> bool { position >= self.start && position < self.end }
    pub fn extend(&mut self, additional: usize) { self.end += additional; self.deleted_length = self.end - self.start; }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if self.end < self.start { return Err(MorphogenesisError::OutOfRange { field: "end".into(), value: self.end as f64, min: self.start as f64, max: f64::INFINITY }); }
        Ok(())
    }
}

impl fmt::Debug for DeletionMutation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeletionMutation").field("start", &self.start).field("end", &self.end).field("length", &self.deleted_length).finish()
    }
}
