// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.


/// EnergyTransfer: The movement of energy between entities.
///
/// Represents a single directed transfer from a source entity
/// to a target entity, with an amount and an efficiency factor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnergyTransfer {
    pub source: u64,
    pub target: u64,
    pub amount: f64,
    pub efficiency: f64,
}

impl EnergyTransfer {
    /// Creates a new energy transfer.
    ///
    /// # Errors
    /// Returns `TransferError::InvalidAmount` if `amount` is NaN or negative.
    /// Returns `TransferError::InvalidEfficiency` if `efficiency` is outside [0.0, 1.0].
    pub fn new(
        source: u64,
        target: u64,
        amount: f64,
        efficiency: f64,
    ) -> Result<Self, TransferError> {
        if amount.is_nan() || amount < 0.0 {
            return Err(TransferError::InvalidAmount { amount });
        }
        if efficiency.is_nan() || efficiency < 0.0 || efficiency > 1.0 {
            return Err(TransferError::InvalidEfficiency { efficiency });
        }
        Ok(Self {
            source,
            target,
            amount,
            efficiency,
        })
    }

    /// Returns the effective amount after applying the efficiency factor.
    pub fn effective_amount(&self) -> f64 {
        self.amount * self.efficiency
    }

    /// Returns the energy lost during transfer (waste due to inefficiency).
    pub fn lost_amount(&self) -> f64 {
        self.amount * (1.0 - self.efficiency)
    }

    /// Checks whether this transfer can be satisfied given the source budget.
    pub fn can_be_satisfied(&self, source_budget: f64) -> bool {
        source_budget.is_finite() && source_budget >= self.amount
    }

    /// Returns a new transfer with the same source and target but a scaled amount.
    ///
    /// # Errors
    /// Returns `TransferError::InvalidAmount` if the scaled amount is invalid.
    /// Returns `TransferError::InvalidEfficiency` if the efficiency has changed.
    pub fn scale_amount(&self, factor: f64) -> Result<Self, TransferError> {
        Self::new(
            self.source,
            self.target,
            self.amount * factor,
            self.efficiency,
        )
    }

    /// Checks whether source and target are distinct entities.
    pub fn is_cross_entity(&self) -> bool {
        self.source != self.target
    }

    /// Returns the transfer as a tuple of (source, target, effective_amount).
    pub fn as_tuple(&self) -> (u64, u64, f64) {
        (self.source, self.target, self.effective_amount())
    }
}

impl Default for EnergyTransfer {
    fn default() -> Self {
        Self {
            source: 0,
            target: 0,
            amount: 0.0,
            efficiency: 1.0,
        }
    }
}

/// Error type for transfer validation failures.
#[derive(Debug, Clone, PartialEq)]
pub enum TransferError {
    /// The amount is NaN or negative.
    InvalidAmount { amount: f64 },
    /// The efficiency is outside the valid range [0.0, 1.0].
    InvalidEfficiency { efficiency: f64 },
}

impl std::fmt::Display for TransferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransferError::InvalidAmount { amount } => {
                write!(f, "Invalid transfer amount {}: must be non-negative and finite", amount)
            }
            TransferError::InvalidEfficiency { efficiency } => {
                write!(f, "Invalid efficiency {}: must be in [0.0, 1.0]", efficiency)
            }
        }
    }
}

impl std::error::Error for TransferError {}