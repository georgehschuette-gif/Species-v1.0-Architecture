// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct SiteSpecificRecombination {
    pub recognition_site: Vec<u8>,
    pub recombinase: RecombinaseType,
    pub product: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecombinaseType {
    Tyrosine,
    Serine,
    Integrase,
}

impl SiteSpecificRecombination {
    pub fn new(recognition_site: Vec<u8>, recombinase: RecombinaseType) -> Self {
        Self { recognition_site, recombinase, product: Vec::new() }
    }

    pub fn catalyze(&mut self, substrate: &[u8]) -> Result<Vec<u8>, MorphogenesisError> {
        if substrate.len() < self.recognition_site.len() { return Err(MorphogenesisError::OutOfRange { field: "substrate".into(), value: substrate.len() as f64, min: self.recognition_site.len() as f64, max: f64::INFINITY }); }
        let pos = substrate.windows(self.recognition_site.len()).position(|w| w == self.recognition_site.as_slice()).ok_or_else(|| MorphogenesisError::MissingInput("recognition site not found".into()))?;
        let mut product = substrate.to_vec();
        match self.recombinase {
            RecombinaseType::Tyrosine => { product.drain(pos..pos + self.recognition_site.len()); }
            RecombinaseType::Serine => { product[pos..pos + self.recognition_site.len()].copy_from_slice(&self.recognition_site); }
            RecombinaseType::Integrase => { product.rotate_right(self.recognition_site.len()); }
        }
        self.product = product.clone();
        Ok(product)
    }

    pub fn product(&self) -> &[u8] { &self.product }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if self.recognition_site.is_empty() { return Err(MorphogenesisError::MissingInput("recognition_site must not be empty".into())); }
        Ok(())
    }
}

