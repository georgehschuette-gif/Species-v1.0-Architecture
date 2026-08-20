// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct MultiCrossover {
    pub points: Vec<usize>,
    pub offspring_a: Vec<u8>,
    pub offspring_b: Vec<u8>,
}

impl MultiCrossover {
    pub fn new(points: Vec<usize>) -> Result<Self, MorphogenesisError> {
        if points.len() > 100 { return Err(MorphogenesisError::CapacityExceeded { max: 100, attempted: points.len() }); }
        if !points.windows(2).all(|w| w[0] < w[1]) { return Err(MorphogenesisError::InvalidInput("crossover points must be sorted".into())); }
        Ok(Self { points, offspring_a: Vec::new(), offspring_b: Vec::new() })
    }

    pub fn recombine(&mut self, parent_a: &[u8], parent_b: &[u8]) -> Result<(), MorphogenesisError> {
        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut last = 0;
        let mut swap = false;
        for &point in &self.points {
            let (src_a, src_b) = if swap { (parent_b, parent_a) } else { (parent_a, parent_b) };
            a.extend_from_slice(&src_a[last..point]);
            b.extend_from_slice(&src_b[last..point]);
            last = point;
            swap = !swap;
        }
        let (src_a, src_b) = if swap { (parent_b, parent_a) } else { (parent_a, parent_b) };
        a.extend_from_slice(&src_a[last..]);
        b.extend_from_slice(&src_b[last..]);
        self.offspring_a = a;
        self.offspring_b = b;
        Ok(())
    }

    pub fn point_count(&self) -> usize { self.points.len() }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if self.offspring_a.len() != self.offspring_b.len() { return Err(MorphogenesisError::DimensionMismatch { expected: self.offspring_b.len(), actual: self.offspring_a.len() }); }
        Ok(())
    }
}

impl fmt::Debug for MultiCrossover {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MultiCrossover").field("point_count", &self.points.len()).finish()
    }
}
