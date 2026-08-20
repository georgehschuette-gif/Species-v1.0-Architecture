// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// CommensalRelationship: A relationship where one species benefits and the other is unaffected.
pub struct CommensalRelationship {
    pub beneficiary: u64,
    pub host: u64,
    pub benefit_magnitude: f64,
    pub host_impact: f64,
}

impl CommensalRelationship {
    /// Constructs a new CommensalRelationship with validated parameters.
    ///
    /// # Errors
    /// Returns `InvalidBenefit` if benefit_magnitude is negative, NaN, or infinite.
    /// Returns `InvalidRelationship` if host_impact is not zero (commensalism requires no host impact).
    /// Returns `SpeciesNotFound` if either species ID is zero.
    pub fn new(
        beneficiary: u64,
        host: u64,
        benefit_magnitude: f64,
        host_impact: f64,
    ) -> Result<Self, SymbiosisError> {
        if beneficiary == 0 {
            return Err(SymbiosisError::SpeciesNotFound(
                "Beneficiary ID must be non-zero".to_string(),
            ));
        }
        if host == 0 {
            return Err(SymbiosisError::SpeciesNotFound(
                "Host ID must be non-zero".to_string(),
            ));
        }
        if benefit_magnitude.is_nan() || benefit_magnitude.is_infinite() || benefit_magnitude < 0.0 {
            return Err(SymbiosisError::InvalidBenefit(
                "Benefit magnitude must be a non-negative finite number".to_string(),
            ));
        }
        if host_impact != 0.0 {
            return Err(SymbiosisError::InvalidRelationship(
                "Host impact must be zero for commensalism".to_string(),
            ));
        }
        Ok(Self {
            beneficiary,
            host,
            benefit_magnitude,
            host_impact,
        })
    }

    /// Validates the internal consistency of this commensal relationship.
    pub fn validate(&self) -> Result<(), SymbiosisError> {
        if self.beneficiary == 0 {
            return Err(SymbiosisError::SpeciesNotFound(
                "Beneficiary ID must be non-zero".to_string(),
            ));
        }
        if self.host == 0 {
            return Err(SymbiosisError::SpeciesNotFound(
                "Host ID must be non-zero".to_string(),
            ));
        }
        if self.benefit_magnitude.is_nan()
            || self.benefit_magnitude.is_infinite()
            || self.benefit_magnitude < 0.0
        {
            return Err(SymbiosisError::InvalidBenefit(
                "Benefit magnitude must be a non-negative finite number".to_string(),
            ));
        }
        if self.host_impact != 0.0 {
            return Err(SymbiosisError::InvalidRelationship(
                "Host impact must be zero for commensalism".to_string(),
            ));
        }
        Ok(())
    }

    /// Returns the net effect on the ecosystem (benefit to beneficiary plus host impact).
    pub fn net_effect(&self) -> f64 {
        self.benefit_magnitude + self.host_impact
    }

    /// Returns true if the relationship is purely commensal (beneficiary gains, host is unaffected).
    pub fn is_beneficial(&self) -> bool {
        self.benefit_magnitude > 0.0 && self.host_impact == 0.0
    }

    /// Returns the symbiosis type for this relationship.
    pub fn interaction_type(&self) -> SymbiosisType {
        SymbiosisType::Commensalism
    }

    /// Returns the benefit-to-host-impact ratio.
    ///
    /// For commensalism, host_impact is always zero, so this returns `f64::INFINITY`.
    pub fn impact_ratio(&self) -> f64 {
        if self.host_impact == 0.0 {
            f64::INFINITY
        } else {
            self.benefit_magnitude / self.host_impact
        }
    }

    /// Returns the beneficiary species ID.
    pub fn beneficiary_id(&self) -> u64 {
        self.beneficiary
    }

    /// Returns the host species ID.
    pub fn host_id(&self) -> u64 {
        self.host
    }
}

impl std::fmt::Display for CommensalRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CommensalRelationship(beneficiary={}, host={}, benefit_magnitude={:.4}, host_impact={:.4})",
            self.beneficiary, self.host, self.benefit_magnitude, self.host_impact
        )
    }
}