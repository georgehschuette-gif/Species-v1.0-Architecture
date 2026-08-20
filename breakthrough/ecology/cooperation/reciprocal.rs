// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// ReciprocalAltruism: Cooperation based on future reciprocation expectations.
pub struct ReciprocalAltruism {
    pub actor: u64,
    pub recipient: u64,
    pub cost: f64,
    pub benefit: f64,
    pub repayment_probability: f64,
}

impl ReciprocalAltruism {
    /// Constructs a new ReciprocalAltruism with validated parameters.
    ///
    /// # Errors
    /// Returns `InvalidCost` if cost is negative, NaN, or infinite.
    /// Returns `InvalidBenefit` if benefit is negative, NaN, or infinite.
    /// Returns `InvalidTrust` if repayment_probability is outside [0.0, 1.0].
    /// Returns `SpeciesNotFound` if actor or recipient ID is zero.
    pub fn new(
        actor: u64,
        recipient: u64,
        cost: f64,
        benefit: f64,
        repayment_probability: f64,
    ) -> Result<Self, CooperationError> {
        if actor == 0 {
            return Err(CooperationError::InvalidTrust(
                "Actor ID must be non-zero".to_string(),
            ));
        }
        if recipient == 0 {
            return Err(CooperationError::InvalidTrust(
                "Recipient ID must be non-zero".to_string(),
            ));
        }
        if cost.is_nan() || cost.is_infinite() || cost < 0.0 {
            return Err(CooperationError::InvalidCost(
                "Cost must be a non-negative finite number".to_string(),
            ));
        }
        if benefit.is_nan() || benefit.is_infinite() || benefit < 0.0 {
            return Err(CooperationError::InvalidBenefit(
                "Benefit must be a non-negative finite number".to_string(),
            ));
        }
        if repayment_probability.is_nan()
            || repayment_probability.is_infinite()
            || !(0.0..=1.0).contains(&repayment_probability)
        {
            return Err(CooperationError::InvalidTrust(
                "Repayment probability must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        Ok(Self {
            actor,
            recipient,
            cost,
            benefit,
            repayment_probability,
        })
    }

    /// Validates the internal consistency of this reciprocal altruism.
    pub fn validate(&self) -> Result<(), CooperationError> {
        if self.actor == 0 {
            return Err(CooperationError::InvalidTrust(
                "Actor ID must be non-zero".to_string(),
            ));
        }
        if self.recipient == 0 {
            return Err(CooperationError::InvalidTrust(
                "Recipient ID must be non-zero".to_string(),
            ));
        }
        if self.cost.is_nan() || self.cost.is_infinite() || self.cost < 0.0 {
            return Err(CooperationError::InvalidCost(
                "Cost must be a non-negative finite number".to_string(),
            ));
        }
        if self.benefit.is_nan() || self.benefit.is_infinite() || self.benefit < 0.0 {
            return Err(CooperationError::InvalidBenefit(
                "Benefit must be a non-negative finite number".to_string(),
            ));
        }
        if self.repayment_probability.is_nan()
            || self.repayment_probability.is_infinite()
            || !(0.0..=1.0).contains(&self.repayment_probability)
        {
            return Err(CooperationError::InvalidTrust(
                "Repayment probability must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        Ok(())
    }

    /// Returns the net benefit (benefit minus cost).
    pub fn net_benefit(&self) -> f64 {
        self.benefit - self.cost
    }

    /// Returns true if the net benefit is positive (favorable for the actor).
    pub fn is_favorable(&self) -> bool {
        self.net_benefit() > 0.0
    }

    /// Returns true if reciprocation is expected (repayment probability > 0.5).
    pub fn reciprocation_expected(&self) -> bool {
        self.repayment_probability > 0.5
    }

    /// Returns the expected return on investment.
    pub fn expected_roi(&self) -> f64 {
        if self.cost == 0.0 {
            f64::INFINITY
        } else {
            self.benefit * self.repayment_probability / self.cost
        }
    }

    /// Returns the actor species ID.
    pub fn actor_id(&self) -> u64 {
        self.actor
    }

    /// Returns the recipient species ID.
    pub fn recipient_id(&self) -> u64 {
        self.recipient
    }
}

impl std::fmt::Display for ReciprocalAltruism {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ReciprocalAltruism(actor={}, recipient={}, cost={:.4}, benefit={:.4}, repayment_prob={:.4})",
            self.actor, self.recipient, self.cost, self.benefit, self.repayment_probability
        )
    }
}