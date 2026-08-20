// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Measurements: Quantified attributes of reality with precision and accuracy.
//!
//! Measurements convert raw observations into numerical values with
//! known uncertainties. They are the quantitative backbone of the
//! reality loop, enabling comparison, prediction, and verification.

use std::collections::HashMap;
use std::fmt;

use crate::RealityError;

/// MeasurementType: The kind of quantity being measured.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MeasurementType {
    /// A scalar quantity (single numerical value).
    Scalar,
    /// A vector quantity (multiple related scalar values).
    Vector,
    /// A categorical quantity (discrete labels).
    Categorical,
    /// A temporal quantity (time-based measurement).
    Temporal,
    /// A spatial quantity (position, distance, area).
    Spatial,
    /// A probabilistic quantity (probability distribution).
    Probabilistic,
}

impl MeasurementType {
    /// Returns the string label of this measurement type.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Scalar => "scalar",
            Self::Vector => "vector",
            Self::Categorical => "categorical",
            Self::Temporal => "temporal",
            Self::Spatial => "spatial",
            Self::Probabilistic => "probabilistic",
        }
    }

    /// Returns whether this type is numerical.
    pub fn is_numerical(&self) -> bool {
        matches!(self, Self::Scalar | Self::Vector | Self::Temporal | Self::Spatial)
    }

    /// Returns whether this type represents a distribution.
    pub fn is_distributional(&self) -> bool {
        matches!(self, Self::Probabilistic)
    }
}

impl std::fmt::Display for MeasurementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for MeasurementType {
    fn default() -> Self {
        Self::Scalar
    }
}

/// Unit: The unit of measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Unit {
    /// No specific unit (dimensionless).
    None,
    /// Meters (length).
    Meter,
    /// Seconds (time).
    Second,
    /// Kilograms (mass).
    Kilogram,
    /// Kelvin (temperature).
    Kelvin,
    /// Amperes (electric current).
    Ampere,
    /// Moles (amount of substance).
    Mole,
    /// Candela (luminous intensity).
    Candela,
    /// Custom unit with a label.
    Custom(&'static str),
}

impl Unit {
    /// Returns the string label of this unit.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Meter => "m",
            Self::Second => "s",
            Self::Kilogram => "kg",
            Self::Kelvin => "K",
            Self::Ampere => "A",
            Self::Mole => "mol",
            Self::Candela => "cd",
            Self::Custom(label) => label,
        }
    }

    /// Returns whether this is a base SI unit.
    pub fn is_si_base(&self) -> bool {
        matches!(
            self,
            Self::Meter | Self::Second | Self::Kilogram | Self::Kelvin | Self::Ampere | Self::Mole | Self::Candela
        )
    }

    /// Returns the dimension tag for this unit.
    pub fn dimension(&self) -> Dimension {
        match self {
            Self::Meter => Dimension::Length,
            Self::Second => Dimension::Time,
            Self::Kilogram => Dimension::Mass,
            Self::Kelvin => Dimension::Temperature,
            Self::Ampere => Dimension::Current,
            Self::Mole => Dimension::Amount,
            Self::Candela => Dimension::Intensity,
            _ => Dimension::Dimensionless,
        }
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for Unit {
    fn default() -> Self {
        Self::None
    }
}

/// Dimension: Physical dimension of a measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dimension {
    Dimensionless,
    Length,
    Time,
    Mass,
    Temperature,
    Current,
    Amount,
    Intensity,
}

/// Scale: A measurement scale type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scale {
    /// Nominal scale (names/categories only).
    Nominal,
    /// Ordinal scale (ordered categories).
    Ordinal,
    /// Interval scale (ordered with equal intervals, no true zero).
    Interval,
    /// Ratio scale (ordered with equal intervals and a true zero).
    Ratio,
}

impl Scale {
    /// Returns the string label of this scale.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Nominal => "nominal",
            Self::Ordinal => "ordinal",
            Self::Interval => "interval",
            Self::Ratio => "ratio",
        }
    }

    /// Returns whether this scale supports meaningful ratios.
    pub fn supports_ratios(&self) -> bool {
        matches!(self, Self::Ratio)
    }

    /// Returns whether this scale supports meaningful differences.
    pub fn supports_differences(&self) -> bool {
        matches!(self, Self::Interval | Self::Ratio)
    }

    /// Returns whether this scale supports ordering.
    pub fn supports_ordering(&self) -> bool {
        matches!(self, Self::Ordinal | Self::Interval | Self::Ratio)
    }
}

impl std::fmt::Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for Scale {
    fn default() -> Self {
        Self::Ratio
    }
}

