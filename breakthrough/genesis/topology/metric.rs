// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::genesis::{GenesisError, GenesisResult};

/// TopologicalMetric: Measures of cognitive space structure.
///
/// The topological metric captures geometric properties
/// of the cognitive manifold: curvature, connectedness,
/// and dimensionality. These measurements inform how
/// entities navigate and interact within the cognitive space.
#[derive(Debug, Clone, PartialEq)]
pub struct TopologicalMetric {
    /// The Gaussian curvature of the space at this point.
    /// Positive curvature indicates spherical geometry;
    /// negative curvature indicates hyperbolic geometry;
    /// zero indicates Euclidean geometry.
    pub curvature: f64,
    /// A measure of how well-connected the space is.
    /// Ranges from 0.0 (disconnected) to 1.0 (fully
    /// connected).
    pub connectedness: f64,
    /// The topological dimension of the space.
    pub dimension: f64,
}

impl TopologicalMetric {
    /// The minimum valid curvature value.
    pub const MIN_CURVATURE: f64 = -1e6;

    /// The maximum valid curvature value.
    pub const MAX_CURVATURE: f64 = 1e6;

    /// The minimum valid connectedness value.
    pub const MIN_CONNECTEDNESS: f64 = 0.0;

    /// The maximum valid connectedness value.
    pub const MAX_CONNECTEDNESS: f64 = 1.0;

    /// The minimum valid dimension value.
    pub const MIN_DIMENSION: f64 = 1.0;

    /// The maximum valid dimension value.
    pub const MAX_DIMENSION: f64 = 128.0;

    /// Creates a new topological metric with the specified
    /// parameters.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if any parameter
    /// is outside its valid range or non-finite.
    pub fn new(
        curvature: f64,
        connectedness: f64,
        dimension: f64,
    ) -> GenesisResult<Self> {
        if curvature.is_nan() || curvature.is_infinite() {
            return Err(GenesisError::OutOfRange {
                field: "curvature".to_string(),
                value: curvature,
                min: Self::MIN_CURVATURE,
                max: Self::MAX_CURVATURE,
            });
        }
        if connectedness.is_nan() || connectedness.is_infinite() {
            return Err(GenesisError::OutOfRange {
                field: "connectedness".to_string(),
                value: connectedness,
                min: Self::MIN_CONNECTEDNESS,
                max: Self::MAX_CONNECTEDNESS,
            });
        }
        if dimension.is_nan() || dimension.is_infinite() {
            return Err(GenesisError::OutOfRange {
                field: "dimension".to_string(),
                value: dimension,
                min: Self::MIN_DIMENSION,
                max: Self::MAX_DIMENSION,
            });
        }
        if curvature < Self::MIN_CURVATURE || curvature > Self::MAX_CURVATURE {
            return Err(GenesisError::OutOfRange {
                field: "curvature".to_string(),
                value: curvature,
                min: Self::MIN_CURVATURE,
                max: Self::MAX_CURVATURE,
            });
        }
        if connectedness < Self::MIN_CONNECTEDNESS
            || connectedness > Self::MAX_CONNECTEDNESS
        {
            return Err(GenesisError::OutOfRange {
                field: "connectedness".to_string(),
                value: connectedness,
                min: Self::MIN_CONNECTEDNESS,
                max: Self::MAX_CONNECTEDNESS,
            });
        }
        if dimension < Self::MIN_DIMENSION
            || dimension > Self::MAX_DIMENSION
        {
            return Err(GenesisError::OutOfRange {
                field: "dimension".to_string(),
                value: dimension,
                min: Self::MIN_DIMENSION,
                max: Self::MAX_DIMENSION,
            });
        }
        Ok(Self {
            curvature,
            connectedness,
            dimension,
        })
    }

    /// Returns whether the space is locally Euclidean
    /// (flat, zero curvature).
    pub fn is_flat(&self) -> bool {
        self.curvature.abs() < 1e-10
    }

    /// Returns whether the space is positively curved
    /// (spherical geometry).
    pub fn is_spherical(&self) -> bool {
        self.curvature > 1e-10
    }

    /// Returns whether the space is negatively curved
    /// (hyperbolic geometry).
    pub fn is_hyperbolic(&self) -> bool {
        self.curvature < -1e-10
    }

    /// Computes the geodesic distance between two points
    /// using a simplified metric based on curvature.
    ///
    /// For flat space, this is the Euclidean distance.
    /// For curved space, this applies a curvature correction.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::ComputationError`] if the
    /// computation produces NaN or infinity.
    pub fn geodesic_distance(
        &self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    ) -> GenesisResult<f64> {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let euclidean = (dx * dx + dy * dy).sqrt();
        if euclidean.is_nan() || euclidean.is_infinite() {
            return Err(GenesisError::ComputationError(
                "geodesic distance computation produced NaN or infinity".to_string(),
            ));
        }
        if self.is_flat() {
            return Ok(euclidean);
        }
        // Apply curvature correction factor
        let correction = 1.0 + self.curvature * euclidean.powi(2) / 6.0;
        let corrected = euclidean * correction;
        if corrected.is_nan() || corrected.is_infinite() {
            return Err(GenesisError::ComputationError(
                "corrected geodesic distance is NaN or infinity".to_string(),
            ));
        }
        Ok(corrected)
    }

    /// Computes the topology score, a composite measure
    /// combining curvature and connectedness.
    ///
    /// Higher scores indicate more structured, well-connected
    /// spaces.
    pub fn topology_score(&self) -> f64 {
        self.connectedness
            * (1.0 - self.curvature.abs() / Self::MAX_CURVATURE)
            * self.dimension
    }

    /// Validates the topological metric, ensuring all values
    /// are within valid ranges and the metric is self-consistent.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if any value is
    /// outside its valid range.
    pub fn validate(&self) -> GenesisResult<()> {
        if self.curvature.is_nan() || self.curvature.is_infinite() {
            return Err(GenesisError::OutOfRange {
                field: "curvature".to_string(),
                value: self.curvature,
                min: Self::MIN_CURVATURE,
                max: Self::MAX_CURVATURE,
            });
        }
        if self.connectedness.is_nan()
            || self.connectedness.is_infinite()
            || self.connectedness < Self::MIN_CONNECTEDNESS
            || self.connectedness > Self::MAX_CONNECTEDNESS
        {
            return Err(GenesisError::OutOfRange {
                field: "connectedness".to_string(),
                value: self.connectedness,
                min: Self::MIN_CONNECTEDNESS,
                max: Self::MAX_CONNECTEDNESS,
            });
        }
        if self.dimension.is_nan()
            || self.dimension.is_infinite()
            || self.dimension < Self::MIN_DIMENSION
            || self.dimension > Self::MAX_DIMENSION
        {
            return Err(GenesisError::OutOfRange {
                field: "dimension".to_string(),
                value: self.dimension,
                min: Self::MIN_DIMENSION,
                max: Self::MAX_DIMENSION,
            });
        }
        Ok(())
    }
}

impl Default for TopologicalMetric {
    fn default() -> Self {
        Self {
            curvature: 0.0,
            connectedness: 1.0,
            dimension: 2.0,
        }
    }
}