// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::HashMap;
use std::fmt;

use super::unit::{MeasurementType, Unit, Scale};
use super::precision::Precision;
use super::accuracy::Accuracy;
use crate::RealityError;

/// Measurement: A quantified attribute of reality.
///
/// Measurements convert observations into numerical values with
/// known precision and accuracy, enabling comparison, prediction,
/// and verification.
#[derive(Debug, Clone, PartialEq)]
pub struct Measurement {
    /// Unique identifier for this measurement.
    pub id: String,
    /// The measured value(s).
    pub value: MeasurementValue,
    /// Type of measurement (scalar, vector, etc.).
    pub measurement_type: MeasurementType,
    /// Unit of measurement.
    pub unit: Unit,
    /// Scale of measurement.
    pub scale: Scale,
    /// Precision of this measurement.
    pub precision: Precision,
    /// Accuracy of this measurement.
    pub accuracy: Accuracy,
    /// Timestamp when the measurement was taken.
    pub timestamp: f64,
    /// Contextual metadata.
    pub metadata: HashMap<String, String>,
}

/// MeasurementValue: The actual measured value(s).
#[derive(Debug, Clone, PartialEq)]
pub enum MeasurementValue {
    /// A single scalar value.
    Scalar(f64),
    /// Multiple scalar values forming a vector.
    Vector(Vec<f64>),
    /// Categorical label.
    Categorical(String),
    /// Temporal duration.
    Temporal(f64),
    /// Spatial position.
    Spatial { x: f64, y: f64, z: f64 },
}

impl Measurement {
    /// Minimum valid timestamp (Unix epoch).
    pub const MIN_TIMESTAMP: f64 = 0.0;
    /// Maximum valid timestamp (year 3000 in seconds).
    pub const MAX_TIMESTAMP: f64 = 32503680000.0;

    /// Creates a new scalar measurement.
    ///
    /// # Errors
    /// Returns `RealityError::OutOfRange` if timestamp is invalid or precision/accuracy are invalid.
    pub fn new_scalar(
        id: impl Into<String>,
        value: f64,
        unit: Unit,
        precision: Precision,
        accuracy: Accuracy,
        timestamp: f64,
    ) -> Result<Self, RealityError> {
        Self::validate_timestamp(timestamp)?;
        Ok(Self {
            id: id.into(),
            value: MeasurementValue::Scalar(value),
            measurement_type: MeasurementType::Scalar,
            unit,
            scale: Scale::Ratio,
            precision,
            accuracy,
            timestamp,
            metadata: HashMap::new(),
        })
    }

    /// Creates a new vector measurement.
    pub fn new_vector(
        id: impl Into<String>,
        values: Vec<f64>,
        unit: Unit,
        precision: Precision,
        accuracy: Accuracy,
        timestamp: f64,
    ) -> Result<Self, RealityError> {
        Self::validate_timestamp(timestamp)?;
        Ok(Self {
            id: id.into(),
            value: MeasurementValue::Vector(values),
            measurement_type: MeasurementType::Vector,
            unit,
            scale: Scale::Ratio,
            precision,
            accuracy,
            timestamp,
            metadata: HashMap::new(),
        })
    }

    /// Adds metadata to the measurement.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Sets the scale of measurement.
    pub fn with_scale(mut self, scale: Scale) -> Self {
        self.scale = scale;
        self
    }

    /// Sets the measurement type.
    pub fn with_type(mut self, measurement_type: MeasurementType) -> Self {
        self.measurement_type = measurement_type;
        self
    }

    /// Returns the scalar value if this is a scalar measurement.
    pub fn as_scalar(&self) -> Option<f64> {
        match &self.value {
            MeasurementValue::Scalar(v) => Some(*v),
            _ => None,
        }
    }

    /// Returns the vector values if this is a vector measurement.
    pub fn as_vector(&self) -> Option<&Vec<f64>> {
        match &self.value {
            MeasurementValue::Vector(v) => Some(v),
            _ => None,
        }
    }

    /// Validates the timestamp.
    fn validate_timestamp(timestamp: f64) -> Result<(), RealityError> {
        if timestamp < Self::MIN_TIMESTAMP || timestamp > Self::MAX_TIMESTAMP {
            return Err(RealityError::OutOfRange {
                field: "timestamp".to_string(),
                value: timestamp,
                min: Self::MIN_TIMESTAMP,
                max: Self::MAX_TIMESTAMP,
            });
        }
        Ok(())
    }

    /// Returns the absolute uncertainty of this measurement.
    pub fn absolute_uncertainty(&self) -> f64 {
        self.accuracy.confidence_interval + self.precision.standard_deviation
    }

    /// Returns the relative uncertainty (uncertainty / value).
    pub fn relative_uncertainty(&self) -> f64 {
        match &self.value {
            MeasurementValue::Scalar(v) => {
                if v.abs() < f64::EPSILON {
                    f64::INFINITY
                } else {
                    self.absolute_uncertainty() / v.abs()
                }
            }
            MeasurementValue::Vector(vals) => {
                if vals.is_empty() {
                    return f64::INFINITY;
                }
                let sum: f64 = vals.iter().map(|v| v.abs()).sum();
                if sum < f64::EPSILON {
                    f64::INFINITY
                } else {
                    self.absolute_uncertainty() / (sum / vals.len() as f64)
                }
            }
            _ => f64::INFINITY,
        }
    }

    /// Combines this measurement with another of the same unit.
    ///
    /// Uses weighted averaging based on inverse variance.
    pub fn combine(&self, other: &Measurement) -> Result<Self, RealityError> {
        if self.unit != other.unit {
            return Err(RealityError::IncompatibleObservations {
                reason: format!("units differ: {} vs {}", self.unit, other.unit),
            });
        }
        match (&self.value, &other.value) {
            (MeasurementValue::Scalar(a), MeasurementValue::Scalar(b)) => {
                let var_a = self.precision.variance();
                let var_b = other.precision.variance();
                let total_var = var_a + var_b;
                if total_var < f64::EPSILON {
                    return Ok(Measurement {
                        id: format!("{}+{}", self.id, other.id),
                        value: MeasurementValue::Scalar((*a + *b) / 2.0),
                        measurement_type: MeasurementType::Scalar,
                        unit: self.unit,
                        scale: self.scale,
                        precision: Precision::new(
                            (self.precision.standard_deviation + other.precision.standard_deviation) / 2.0,
                            self.precision.sample_size + other.precision.sample_size,
                        )?,
                        accuracy: Accuracy {
                            bias: (self.accuracy.bias + other.accuracy.bias) / 2.0,
                            confidence_interval: (self.accuracy.confidence_interval + other.accuracy.confidence_interval) / 2.0,
                            confidence_level: (self.accuracy.confidence_level + other.accuracy.confidence_level) / 2.0,
                        },
                        timestamp: self.timestamp.min(other.timestamp),
                        metadata: HashMap::new(),
                    });
                }
                let weight_a = var_b / total_var;
                let weight_b = var_a / total_var;
                let combined = a * weight_a + b * weight_b;
                let combined_precision = Precision::new(total_var.sqrt(), self.precision.sample_size + other.precision.sample_size)?;
                Ok(Measurement {
                    id: format!("{}+{}", self.id, other.id),
                    value: MeasurementValue::Scalar(combined),
                    measurement_type: MeasurementType::Scalar,
                    unit: self.unit,
                    scale: self.scale,
                    precision: combined_precision,
                    accuracy: Accuracy {
                        bias: (self.accuracy.bias + other.accuracy.bias) / 2.0,
                        confidence_interval: (self.accuracy.confidence_interval + other.accuracy.confidence_interval) / 2.0,
                        confidence_level: (self.accuracy.confidence_level + other.accuracy.confidence_level) / 2.0,
                    },
                    timestamp: self.timestamp.min(other.timestamp),
                    metadata: HashMap::new(),
                })
            }
            _ => Err(RealityError::IncompatibleObservations {
                reason: "cannot combine non-scalar measurements".to_string(),
            }),
        }
    }
}

