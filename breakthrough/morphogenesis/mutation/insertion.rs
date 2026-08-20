// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct InsertionMutation {
    pub position: usize,
    pub inserted_sequence: Vec<u8>,
    pub length: usize,
}

impl InsertionMutation {
    pub fn new(position: usize, inserted_sequence: Vec<u8>) -> Self {
        let length = inserted_sequence.len();
        Self { position, inserted_sequence, length }
    }

    pub fn position(&self) -> usize { self.position }
    pub fn sequence(&self) -> &[u8] { &self.inserted_sequence }
    pub fn length(&self) -> usize { self.length }
    pub fn is_frameshift(&self) -> bool { self.length % 3 != 0 }
    pub fn reverse(&mut self) { self.inserted_sequence.reverse(); }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if self.inserted_sequence.is_empty() { return Err(MorphogenesisError::MissingInput("inserted_sequence must not be empty".into())); }
        Ok(())
    }
}

impl fmt::Debug for InsertionMutation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InsertionMutation").field("position", &self.position).field("length", &self.length).finish()
    }
}
