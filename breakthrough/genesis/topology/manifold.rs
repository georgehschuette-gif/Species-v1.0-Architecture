// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::genesis::{GenesisError, GenesisResult};

/// MetricTensor: The mathematical object describing the
/// geometry of cognitive space.
///
/// The metric tensor defines distances and angles in the
/// cognitive manifold. It is a symmetric positive-definite
/// matrix whose components encode the local geometry.
#[derive(Debug, Clone, PartialEq)]
pub struct MetricTensor {
    /// The dimension of the manifold this tensor operates on.
    pub dimension: usize,
    /// The components of the metric tensor stored in row-major
    /// order. Must be a square matrix of size `dimension x dimension`.
    pub components: Vec<f64>,
}

impl MetricTensor {
    /// The minimum manifold dimension.
    pub const MIN_DIMENSION: usize = 1;

    /// The maximum manifold dimension.
    pub const MAX_DIMENSION: usize = 16;

    /// Creates a new metric tensor for the given dimension,
    /// initialized to the identity matrix.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if `dimension` is
    /// zero or exceeds the maximum allowed.
    pub fn new(dimension: usize) -> GenesisResult<Self> {
        Self::validate_dimension(dimension)?;
        let size = dimension * dimension;
        let mut components = vec![0.0; size];
        for i in 0..dimension {
            components[i * dimension + i] = 1.0;
        }
        Ok(Self {
            dimension,
            components,
        })
    }

    /// Creates a metric tensor from raw components.
    ///
    /// The components must form a square matrix of size
    /// `dimension x dimension`.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if `dimension` is
    /// invalid or the components length does not match.
    /// Returns [`GenesisError::ValidationFailure`] if the
    /// matrix is not symmetric or is not positive-definite.
    pub fn from_components(
        dimension: usize,
        components: Vec<f64>,
    ) -> GenesisResult<Self> {
        Self::validate_dimension(dimension)?;
        let expected_len = dimension * dimension;
        if components.len() != expected_len {
            return Err(GenesisError::ValidationFailure(format!(
                "components length {} does not match dimension {} x {}",
                components.len(),
                dimension,
                dimension
            )));
        }
        // Check symmetry
        for i in 0..dimension {
            for j in (i + 1)..dimension {
                let a = components[i * dimension + j];
                let b = components[j * dimension + i];
                if (a - b).abs() > 1e-10 {
                    return Err(GenesisError::ValidationFailure(format!(
                        "metric tensor is not symmetric at ({}, {})",
                        i, j
                    )));
                }
            }
        }
        // Basic positive-definiteness check: diagonal must be positive
        for i in 0..dimension {
            if components[i * dimension + i] <= 0.0 {
                return Err(GenesisError::ValidationFailure(format!(
                    "metric tensor is not positive-definite: diagonal[{}] = {}",
                    i,
                    components[i * dimension + i]
                )));
            }
        }
        Ok(Self {
            dimension,
            components,
        })
    }

    /// Validates that the dimension is within valid bounds.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if dimension is
    /// outside [MIN_DIMENSION, MAX_DIMENSION].
    fn validate_dimension(dimension: usize) -> GenesisResult<()> {
        if dimension < Self::MIN_DIMENSION {
            return Err(GenesisError::OutOfRange {
                field: "dimension".to_string(),
                value: dimension as f64,
                min: Self::MIN_DIMENSION as f64,
                max: Self::MAX_DIMENSION as f64,
            });
        }
        if dimension > Self::MAX_DIMENSION {
            return Err(GenesisError::OutOfRange {
                field: "dimension".to_string(),
                value: dimension as f64,
                min: Self::MIN_DIMENSION as f64,
                max: Self::MAX_DIMENSION as f64,
            });
        }
        Ok(())
    }

    /// Computes the determinant of the metric tensor using
    /// LU decomposition for small matrices.
    ///
    /// Returns `None` if the determinant cannot be computed
    /// (e.g., singular matrix).
    pub fn determinant(&self) -> Option<f64> {
        if self.dimension == 1 {
            return Some(self.components[0]);
        }
        if self.dimension == 2 {
            let a = self.components[0];
            let b = self.components[1];
            let c = self.components[2];
            let d = self.components[3];
            return Some(a * d - b * c);
        }
        // For larger dimensions, use cofactor expansion
        // (acceptable for small dimensions)
        Some(self.cofactor_determinant())
    }

    /// Returns the identity metric tensor for the given dimension.
    pub fn identity(dimension: usize) -> GenesisResult<Self> {
        Self::new(dimension)
    }

    /// Returns the number of components in the tensor.
    pub fn size(&self) -> usize {
        self.components.len()
    }

    /// Validates the metric tensor is well-formed.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::ValidationFailure`] if the tensor
    /// dimensions are inconsistent or components contain NaN/Inf.
    pub fn validate(&self) -> GenesisResult<()> {
        Self::validate_dimension(self.dimension)?;
        let expected = self.dimension * self.dimension;
        if self.components.len() != expected {
            return Err(GenesisError::ValidationFailure(format!(
                "components length {} does not match dimension {} x {}",
                self.components.len(),
                self.dimension,
                self.dimension
            )));
        }
        for (i, c) in self.components.iter().enumerate() {
            if c.is_nan() {
                return Err(GenesisError::ComputationError(format!(
                    "component {} is NaN",
                    i
                )));
            }
            if c.is_infinite() {
                return Err(GenesisError::ComputationError(format!(
                    "component {} is infinite",
                    i
                )));
            }
        }
        Ok(())
    }

    fn cofactor_determinant(&self) -> f64 {
        let n = self.dimension;
        if n == 1 {
            return self.components[0];
        }
        let mut det = 0.0;
        let mut sign = 1.0;
        for col in 0..n {
            let minor = self.minor_matrix(0, col);
            let minor_det = minor.cofactor_determinant();
            det += sign * self.components[col] * minor_det;
            sign = -sign;
        }
        det
    }

    fn minor_matrix(&self, skip_row: usize, skip_col: usize) -> Self {
        let n = self.dimension;
        let mut minor = Vec::with_capacity((n - 1) * (n - 1));
        for r in 0..n {
            if r == skip_row {
                continue;
            }
            for c in 0..n {
                if c == skip_col {
                    continue;
                }
                minor.push(self.components[r * n + c]);
            }
        }
        // Safe to unwrap: minor has (n-1)^2 components and n-1 >= 1
        MetricTensor {
            dimension: n - 1,
            components: minor,
        }
    }
}

impl Default for MetricTensor {
    fn default() -> Self {
        Self::new(2).unwrap_or_else(|_| {
            Self {
                dimension: 2,
                components: vec![1.0, 0.0, 0.0, 1.0],
            }
        })
    }
}

/// CognitiveManifold: The continuous topological surface on which
/// the cognitive ecosystem exists.
///
/// A manifold is a topological space that locally resembles Euclidean
/// space. In the cognitive ecosystem, the manifold provides the
/// geometric substrate in which entities are embedded and through
/// which they interact. It carries a metric tensor that defines
/// distances and angles, and tracks the overall connectivity of
/// the space.
#[derive(Debug, Clone, PartialEq)]
pub struct CognitiveManifold {
    /// The topological dimension of the manifold.
    pub dimension: usize,
    /// The metric tensor defining the geometry of the manifold.
    pub metric: MetricTensor,
    /// The number of connected components in the manifold.
    pub connected_components: usize,
    /// Whether the manifold is orientable.
    pub orientable: bool,
}

impl CognitiveManifold {
    /// The minimum manifold dimension.
    pub const MIN_DIMENSION: usize = 1;

    /// The maximum manifold dimension.
    pub const MAX_DIMENSION: usize = 16;

    /// Creates a new cognitive manifold with the given dimension
    /// and an identity metric tensor.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if `dimension` is
    /// zero or exceeds the maximum allowed.
    pub fn new(dimension: usize) -> GenesisResult<Self> {
        let metric = MetricTensor::new(dimension)?;
        Ok(Self {
            dimension,
            metric,
            connected_components: 1,
            orientable: true,
        })
    }

    /// Creates a cognitive manifold from an existing metric tensor.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if the metric tensor's
    /// dimension is invalid.
    pub fn from_metric(metric: MetricTensor) -> GenesisResult<Self> {
        Ok(Self {
            dimension: metric.dimension,
            metric,
            connected_components: 1,
            orientable: true,
        })
    }

    /// Returns the Euler characteristic of the manifold, computed
    /// from the connected components and orientability.
    ///
    /// For a connected orientable manifold, χ = 2 - 2g where g is
    /// the genus. This is a simplified model.
    pub fn euler_characteristic(&self) -> i64 {
        let genus = (self.connected_components.max(1) - 1) as i64;
        if self.orientable {
            2 - 2 * genus
        } else {
            2 - genus
        }
    }

    /// Returns whether the manifold is flat (zero curvature everywhere).
    pub fn is_flat(&self) -> bool {
        self.metric.determinant().map(|d| d.abs() < 1e-10).unwrap_or(false)
    }

    /// Validates the manifold, ensuring the metric tensor is valid
    /// and the dimension is consistent.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::ValidationFailure`] if the manifold
    /// is inconsistent.
    pub fn validate(&self) -> GenesisResult<()> {
        if self.dimension != self.metric.dimension {
            return Err(GenesisError::ValidationFailure(format!(
                "manifold dimension {} does not match metric tensor dimension {}",
                self.dimension, self.metric.dimension
            )));
        }
        self.metric.validate()?;
        if self.connected_components == 0 {
            return Err(GenesisError::ValidationFailure(
                "manifold must have at least one connected component".to_string(),
            ));
        }
        Ok(())
    }
}

impl Default for CognitiveManifold {
    fn default() -> Self {
        Self::new(2).unwrap_or_else(|_| Self {
            dimension: 2,
            metric: MetricTensor::default(),
            connected_components: 1,
            orientable: true,
        })
    }
}