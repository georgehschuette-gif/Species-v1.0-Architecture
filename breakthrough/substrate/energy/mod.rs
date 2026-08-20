// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Energy: The conserved quantity driving cognitive transformations.
//! Defines energy types, budgets, and transfer mechanisms.

pub mod budget;
pub mod transfer;
pub mod form;

pub use budget::{EnergyBudget, BudgetError};
pub use transfer::{EnergyTransfer, TransferError};
pub use form::EnergyForm;
