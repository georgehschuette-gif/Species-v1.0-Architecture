// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Equilibrium Attractor
//!
//! An [`Equilibrium`] is a fixed point of a dynamical system where the
//! state vector does not change under time evolution. Equilibria are
//! classified by the local geometry of the flow, which is captured by the
//! eigenvalues of the Jacobian matrix.
//!
//! ## Classification
//!
//! - **Node**: Real eigenvalues with the same sign.
//! - **Spiral**: Complex conjugate eigenvalues.
//! - **Center**: Purely imaginary eigenvalues (neutrally stable).
//! - **Saddle**: Eigenvalues of mixed sign (unstable).
//!
//! ## Linearization
//!
//! Near an equilibrium, the dynamics are approximated by the linear system
//! `dx/dt = J·(x - x₀)`, where `J` is the Jacobian. The eigenvectors
//! determine the stable and unstable manifolds.

use std::cmp::PartialEq;
use std::fmt::{self, Display, Formatter};

/// Classification of an equilibrium by local flow geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquilibriumClass {
    /// Real eigenvalues with same sign; trajectories approach along
    /// straight lines.
    Node,
    /// Complex conjugate eigenvalues; trajectories spiral inward or
    /// outward.
    Spiral,
    /// Purely imaginary eigenvalues; trajectories are closed orbits
    /// (neutrally stable).
    Center,
    /// Eigenvalues of mixed sign; stable and unstable directions
    /// coexist.
    Saddle,
    /// Degenerate case requiring normal-form analysis.
    Degenerate,
    /// Classification could not be determined from spectral data.
    Unknown,
}

impl Display for EquilibriumClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Node => "node",
            Self::Spiral => "spiral",
            Self::Center => "center",
            Self::Saddle => "saddle",
            Self::Degenerate => "degenerate",
            Self::Unknown => "unknown",
        })
    }
}

/// A fixed-point equilibrium attractor with local linearization data.
///
/// # Fields
///
/// * `position` - Coordinates of the equilibrium in phase space.
/// * `jacobian` - Flattened Jacobian matrix at the equilibrium.
/// * `eigenvalues` - Complex eigenvalues of the Jacobian.
/// * `class` - Geometric classification of the equilibrium.
#[derive(Debug, Clone, PartialEq)]
pub struct Equilibrium {
    pub position: Vec<f64>,
    pub jacobian: Vec<f64>,
    pub eigenvalues: Vec<f64>,
    pub class: EquilibriumClass,
}

impl Equilibrium {
    /// Creates a new equilibrium with spectral and geometric data.
    ///
    /// # Arguments
    ///
    /// * `position` - Fixed-point coordinates.
    /// * `jacobian` - Jacobian matrix stored in row-major order.
    /// * `eigenvalues` - Real parts of the Jacobian eigenvalues.
    /// * `class` - Geometric classification.
    ///
    /// # Panics
    ///
    /// Panics if `jacobian` length is not a perfect square, or if dimension
    /// checks fail.
    pub fn new(
        position: Vec<f64>,
        jacobian: Vec<f64>,
        eigenvalues: Vec<f64>,
        class: EquilibriumClass,
    ) -> Self {
        let dim = position.len();
        assert!(dim > 0, "dimension must be positive");
        assert_eq!(jacobian.len() % dim, 0, "jacobian must be square");
        assert_eq!(jacobian.len() / dim, dim, "jacobian must be square");
        assert_eq!(eigenvalues.len(), dim, "eigenvalue count must match dimension");
        Self { position, jacobian, eigenvalues, class }
    }

    /// Returns the spatial dimension of the equilibrium.
    pub fn dimension(&self) -> usize {
        self.position.len()
    }

    /// Returns the Jacobian as a square matrix.
    ///
    /// The returned matrix is in row-major order with `dim × dim` elements.
    pub fn jacobian_matrix(&self) -> &[f64] {
        &self.jacobian
    }

    /// Computes the determinant of the Jacobian at the equilibrium.
    pub fn determinant(&self) -> f64 {
        let dim = self.dimension();
        match dim {
            1 => self.jacobian[0],
            2 => {
                let j = &self.jacobian;
                j[0] * j[3] - j[1] * j[2]
            }
            3 => {
                let j = &self.jacobian;
                j[0] * (j[4] * j[8] - j[5] * j[7])
                    - j[1] * (j[3] * j[8] - j[5] * j[6])
                    + j[2] * (j[3] * j[7] - j[4] * j[6])
            }
            _ => {
                let mut mat = self.jacobian.clone();
                det_lu(&mut mat, dim)
            }
        }
    }

    /// Returns the trace of the Jacobian.
    pub fn trace(&self) -> f64 {
        let dim = self.dimension();
        (0..dim).map(|i| self.jacobian[i * dim + i]).sum()
    }

    /// Classifies the equilibrium from its eigenvalues.
    ///
    /// This is a convenience method that recomputes classification from
    /// the raw eigenvalues, independent of the stored `class` field.
    pub fn classify_from_eigenvalues(&self) -> EquilibriumClass {
        let reals: Vec<f64> = self.eigenvalues.iter().map(|e| e.abs()).collect();
        let has_complex = self.eigenvalues.iter().any(|e| e.fract() != 0.0);
        let all_negative = self.eigenvalues.iter().all(|e| *e < 0.0);
        let all_positive = self.eigenvalues.iter().all(|e| *e > 0.0);
        let has_negative = self.eigenvalues.iter().any(|e| *e < 0.0);
        let has_positive = self.eigenvalues.iter().any(|e| *e > 0.0);
        let all_zero = self.eigenvalues.iter().all(|e| e.abs() < 1e-12);

        if all_zero { EquilibriumClass::Center }
        else if has_complex && all_negative { EquilibriumClass::Spiral }
        else if has_complex && all_positive { EquilibriumClass::Spiral }
        else if !has_complex && all_negative { EquilibriumClass::Node }
        else if !has_complex && all_positive { EquilibriumClass::Node }
        else if has_negative && has_positive { EquilibriumClass::Saddle }
        else { EquilibriumClass::Degenerate }
    }

    /// Returns the stability index, defined as the number of eigenvalues
    /// with negative real part.
    pub fn stability_index(&self) -> usize {
        self.eigenvalues.iter().filter(|e| **e < 0.0).count()
    }

    /// Checks whether the equilibrium is hyperbolic (no eigenvalues with
    /// zero real part).
    pub fn is_hyperbolic(&self) -> bool {
        self.eigenvalues.iter().all(|e| e.abs() > 1e-12)
    }

    /// Returns the sum of real parts of all eigenvalues.
    pub fn divergence(&self) -> f64 {
        self.eigenvalues.iter().sum()
    }

    /// Returns a summary string for display or logging.
    pub fn summary(&self) -> String {
        format!(
            "Equilibrium(class={}, trace={:.4}, det={:.4}, dim={})",
            self.class,
            self.trace(),
            self.determinant(),
            self.dimension()
        )
    }
}

impl Display for Equilibrium {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Equilibrium(pos=[{}], class={}, eigenvalues=[{}])",
            self.position.iter().map(|c| format!("{:.6}", c)).collect::<Vec<_>>().join(", "),
            self.class,
            self.eigenvalues.iter().map(|e| format!("{:.6}", e)).collect::<Vec<_>>().join(", ")
        )
    }
}

/// Computes the determinant of a square matrix using LU decomposition.
///
/// This helper operates in-place on a row-major matrix slice.
fn det_lu(matrix: &mut [f64], dim: usize) -> f64 {
    let mut det = 1.0;
    for i in 0..dim {
        let mut max_row = i;
        for r in i + 1..dim {
            if matrix[r * dim + i].abs() > matrix[max_row * dim + i].abs() {
                max_row = r;
            }
        }
        if max_row != i {
            for c in 0..dim {
                matrix.swap(i * dim + c, max_row * dim + c);
            }
            det = -det;
        }
        let pivot = matrix[i * dim + i];
        if pivot.abs() < 1e-12 {
            return 0.0;
        }
        det *= pivot;
        for r in i + 1..dim {
            let factor = matrix[r * dim + i] / pivot;
            for c in i..dim {
                matrix[r * dim + c] -= factor * matrix[i * dim + c];
            }
        }
    }
    det
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2d_node_determinant() {
        let eq = Equilibrium::new(
            vec![0.0, 0.0],
            vec![-2.0, 0.0, 0.0, -3.0],
            vec![-2.0, -3.0],
            EquilibriumClass::Node,
        );
        assert!((eq.determinant() - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_hyperbolic_check() {
        let eq = Equilibrium::new(
            vec![1.0],
            vec![-1.0],
            vec![-1.0],
            EquilibriumClass::Node,
        );
        assert!(eq.is_hyperbolic());
    }

    #[test]
    fn test_stability_index() {
        let eq = Equilibrium::new(
            vec![0.0, 0.0, 0.0],
            vec![-1.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, -0.5],
            vec![-1.0, 2.0, -0.5],
            EquilibriumClass::Saddle,
        );
        assert_eq!(eq.stability_index(), 2);
    }
}
