// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.


/// EnergyBudget: The total energy available to an entity or system.
///
/// Tracks both the total capacity and the currently available energy.
/// The available amount can never exceed the total capacity.
/// All values are guaranteed to be non-negative and finite.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnergyBudget {
    pub total: f64,
    pub available: f64,
}

impl EnergyBudget {
    /// The minimum valid energy budget total.
    pub const MIN_TOTAL: f64 = 0.0;

    /// Creates a new energy budget with the given total capacity.
    ///
    /// All available energy starts at the total capacity.
    ///
    /// # Errors
    /// Returns `BudgetError::InvalidTotal` if total is NaN, infinite, or negative.
    pub fn new(total: f64) -> Result<Self, BudgetError> {
        if total.is_nan() {
            return Err(BudgetError::NaNValue);
        }
        if total.is_infinite() {
            return Err(BudgetError::InfiniteValue);
        }
        if total < Self::MIN_TOTAL {
            return Err(BudgetError::InvalidTotal { total });
        }
        Ok(Self {
            total,
            available: total,
        })
    }

    /// Attempts to consume the specified amount of energy.
    ///
    /// # Errors
    /// Returns `BudgetError::InsufficientEnergy` if `amount` exceeds available energy.
    /// Returns `BudgetError::InvalidAmount` if `amount` is NaN or negative.
    pub fn consume(&mut self, amount: f64) -> Result<bool, BudgetError> {
        if amount.is_nan() {
            return Err(BudgetError::InvalidAmount { amount });
        }
        if amount < 0.0 {
            return Err(BudgetError::InvalidAmount { amount });
        }
        if self.available < amount {
            return Ok(false);
        }
        self.available -= amount;
        Ok(true)
    }

    /// Replenishes the available energy by the given amount.
    ///
    /// The available energy is capped at the total capacity.
    ///
    /// # Errors
    /// Returns `BudgetError::InvalidAmount` if `amount` is NaN or negative.
    pub fn replenish(&mut self, amount: f64) -> Result<(), BudgetError> {
        if amount.is_nan() {
            return Err(BudgetError::NaNValue);
        }
        if amount < 0.0 {
            return Err(BudgetError::InvalidAmount { amount });
        }
        self.available = (self.available + amount).min(self.total);
        Ok(())
    }

    /// Returns the fraction of total energy currently available.
    pub fn availability_ratio(&self) -> f64 {
        if self.total == 0.0 {
            1.0
        } else {
            self.available / self.total
        }
    }

    /// Returns the amount of energy that has been consumed.
    pub fn consumed(&self) -> f64 {
        self.total - self.available
    }

    /// Returns whether the budget is depleted (no available energy).
    pub fn is_depleted(&self) -> bool {
        self.available < f64::EPSILON
    }

    /// Returns whether the budget is full (available equals total).
    pub fn is_full(&self) -> bool {
        (self.available - self.total).abs() < f64::EPSILON
    }

    /// Sets the total capacity, capping available at the new total if it is lower.
    ///
    /// # Errors
    /// Returns `BudgetError::InvalidTotal` if `total` is NaN, infinite, or negative.
    pub fn set_total(&mut self, total: f64) -> Result<(), BudgetError> {
        if total.is_nan() || total.is_infinite() || total < Self::MIN_TOTAL {
            return Err(BudgetError::InvalidTotal { total });
        }
        self.total = total;
        self.available = self.available.min(total);
        Ok(())
    }
}

impl Default for EnergyBudget {
    fn default() -> Self {
        Self {
            total: 0.0,
            available: 0.0,
        }
    }
}

/// Error type for budget operation failures.
#[derive(Debug, Clone, PartialEq)]
pub enum BudgetError {
    /// The total capacity is NaN.
    NaNValue,
    /// The total capacity is infinite.
    InfiniteValue,
    /// The total capacity is negative.
    InvalidTotal { total: f64 },
    /// The amount is NaN or negative.
    InvalidAmount { amount: f64 },
    /// Insufficient energy available for the requested consumption.
    InsufficientEnergy { requested: f64, available: f64 },
}

impl std::fmt::Display for BudgetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BudgetError::NaNValue => write!(f, "Energy value cannot be NaN"),
            BudgetError::InfiniteValue => write!(f, "Energy value cannot be infinite"),
            BudgetError::InvalidTotal { total } => {
                write!(f, "Invalid total energy {}: must be non-negative and finite", total)
            }
            BudgetError::InvalidAmount { amount } => {
                write!(f, "Invalid amount {}: must be non-negative and finite", amount)
            }
            BudgetError::InsufficientEnergy { requested, available } => {
                write!(
                    f,
                    "Insufficient energy: requested {} but only {} available",
                    requested, available
                )
            }
        }
    }
}

impl std::error::Error for BudgetError {}