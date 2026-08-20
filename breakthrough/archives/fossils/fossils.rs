// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum FossilError {
    InvalidQuality(f64),
    NegativeAge,
    EmptyOrganism,
}

impl fmt::Display for FossilError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQuality(value) => write!(f, "invalid preservation quality: {}", value),
            Self::NegativeAge => write!(f, "fossil age must not be negative"),
            Self::EmptyOrganism => write!(f, "organism name must not be empty"),
        }
    }
}

impl std::error::Error for FossilError {}

pub struct FossilRecord {
    pub id: u64,
    pub organism: String,
    pub geological_period: String,
    pub age_mya: f64,
    pub preservation_quality: f64,
}

impl FossilRecord {
    pub fn new(
        id: u64,
        organism: String,
        geological_period: String,
        age_mya: f64,
        preservation_quality: f64,
    ) -> Result<Self, FossilError> {
        if age_mya < 0.0 {
            return Err(FossilError::NegativeAge);
        }
        if organism.trim().is_empty() {
            return Err(FossilError::EmptyOrganism);
        }
        if !(0.0..=1.0).contains(&preservation_quality) {
            return Err(FossilError::InvalidQuality(preservation_quality));
        }
        Ok(Self {
            id,
            organism,
            geological_period,
            age_mya,
            preservation_quality,
        })
    }

    pub fn age_classification(&self) -> &'static str {
        if self.age_mya < 1.0 {
            "modern"
        } else if self.age_mya < 10.0 {
            "pleistocene"
        } else if self.age_mya < 65.0 {
            "tertiary"
        } else if self.age_mya < 250.0 {
            "mesozoic"
        } else if self.age_mya < 540.0 {
            "paleozoic"
        } else {
            "precambrian"
        }
    }

    pub fn quality_score(&self) -> f64 {
        self.preservation_quality * (1.0 / (1.0 + self.age_mya * 0.01))
    }

    pub fn is_exhibit_worthy(&self) -> bool {
        self.preservation_quality > 0.7 && self.age_mya > 100.0
    }
}

impl fmt::Display for FossilRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FossilRecord(id={}, organism={}, period={}, age={:.1}Mya, quality={:.2})",
            self.id,
            self.organism,
            self.geological_period,
            self.age_mya,
            self.preservation_quality
        )
    }
}
