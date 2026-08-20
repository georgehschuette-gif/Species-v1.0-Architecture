// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// KalmanFusion: Combines sensory streams using Kalman filtering.
pub struct KalmanFusion {
    pub state: Vec<f64>,
    pub covariance: Vec<Vec<f64>>,
    pub process_noise: f64,
    pub measurement_noise: f64,
}

impl KalmanFusion {
    /// Create a new KalmanFusion with the given state dimension and noise parameters.
    pub fn new(
        dimension: usize,
        process_noise: f64,
        measurement_noise: f64,
    ) -> PerceptionResult<Self> {
        if dimension == 0 {
            return Err(PerceptionError::InvalidConfiguration(
                "dimension must be positive".into(),
            ));
        }
        if process_noise <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "process_noise must be positive".into(),
            ));
        }
        if measurement_noise <= 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "measurement_noise must be positive".into(),
            ));
        }
        let state = vec![0.0; dimension];
        let covariance = vec![vec![1.0; dimension]; dimension];
        Ok(Self {
            state,
            covariance,
            process_noise,
            measurement_noise,
        })
    }

    /// Perform a prediction step using the state transition model.
    pub fn predict(&mut self, transition_matrix: &[Vec<f64>]) -> PerceptionResult<()> {
        let n = self.state.len();
        if transition_matrix.len() != n {
            return Err(PerceptionError::InvalidConfiguration(
                "transition matrix dimension mismatch".into(),
            ));
        }
        for row in transition_matrix {
            if row.len() != n {
                return Err(PerceptionError::InvalidConfiguration(
                    "transition matrix row length mismatch".into(),
                ));
            }
        }
        let mut new_state = vec![0.0; n];
        for i in 0..n {
            for j in 0..n {
                new_state[i] += transition_matrix[i][j] * self.state[j];
            }
        }
        self.state = new_state;
        let mut new_cov = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    new_cov[i][j] += transition_matrix[i][k] * self.covariance[k][j];
                }
            }
        }
        let mut process_cov = vec![vec![0.0; n]; n];
        for i in 0..n {
            process_cov[i][i] = self.process_noise;
        }
        for i in 0..n {
            for j in 0..n {
                self.covariance[i][j] = new_cov[i][j] + process_cov[i][j];
            }
        }
        Ok(())
    }

    /// Perform an update step with a new measurement.
    pub fn update(&mut self, measurement: &[f64], observation_matrix: &[Vec<f64>]) -> PerceptionResult<()> {
        let n = self.state.len();
        let m = measurement.len();
        if observation_matrix.len() != m {
            return Err(PerceptionError::InvalidConfiguration(
                "observation matrix row count mismatch".into(),
            ));
        }
        for row in observation_matrix {
            if row.len() != n {
                return Err(PerceptionError::InvalidConfiguration(
                    "observation matrix column count mismatch".into(),
                ));
            }
        }
        let mut innovation = vec![0.0; m];
        for i in 0..m {
            innovation[i] = measurement[i];
            for j in 0..n {
                innovation[i] -= observation_matrix[i][j] * self.state[j];
            }
        }
        let mut innovation_cov = vec![vec![0.0; m]; m];
        for i in 0..m {
            for j in 0..m {
                for k in 0..n {
                    innovation_cov[i][j] += observation_matrix[i][k] * self.covariance[k][j] * observation_matrix[j][k];
                }
                if i == j {
                    innovation_cov[i][j] += self.measurement_noise;
                }
            }
        }
        let kalman_gain = self.compute_kalman_gain(&innovation_cov, observation_matrix, n, m)?;
        for i in 0..n {
            let mut innovation_sum = 0.0;
            for j in 0..m {
                innovation_sum += kalman_gain[i][j] * innovation[j];
            }
            self.state[i] += innovation_sum;
        }
        let identity = identity_matrix(n);
        for i in 0..n {
            for j in 0..n {
                let mut sum = 0.0;
                for k in 0..m {
                    sum += kalman_gain[i][k] * observation_matrix[k][j];
                }
                self.covariance[i][j] = (identity[i][j] - sum) * self.covariance[i][j];
            }
        }
        Ok(())
    }

    /// Compute the Kalman gain matrix.
    fn compute_kalman_gain(
        &self,
        innovation_cov: &[Vec<f64>],
        observation_matrix: &[Vec<f64>],
        n: usize,
        m: usize,
    ) -> PerceptionResult<Vec<Vec<f64>>> {
        let mut h_times_p = vec![vec![0.0; n]; m];
        for i in 0..m {
            for j in 0..n {
                for k in 0..n {
                    h_times_p[i][j] += observation_matrix[i][k] * self.covariance[k][j];
                }
            }
        }
        let mut s = vec![vec![0.0; m]; m];
        for i in 0..m {
            for j in 0..m {
                for k in 0..n {
                    s[i][j] += h_times_p[i][k] * observation_matrix[j][k];
                }
                if i == j {
                    s[i][j] += self.measurement_noise;
                }
            }
        }
        let s_inv = invert_matrix(&s)?;
        let mut kalman_gain = vec![vec![0.0; m]; n];
        for i in 0..n {
            for j in 0..m {
                for k in 0..m {
                    kalman_gain[i][j] += h_times_p[i][k] * s_inv[k][j];
                }
            }
        }
        Ok(kalman_gain)
    }

    /// Return the current state estimate.
    pub fn state_estimate(&self) -> &[f64] {
        &self.state
    }

    /// Return the current uncertainty (diagonal of covariance matrix).
    pub fn uncertainty(&self) -> Vec<f64> {
        self.covariance
            .iter()
            .enumerate()
            .map(|(i, row)| row[i])
            .collect()
    }

    /// Reset the filter to initial conditions.
    pub fn reset(&mut self) {
        let n = self.state.len();
        self.state = vec![0.0; n];
        self.covariance = vec![vec![1.0; n]; n];
    }
}

fn invert_matrix(matrix: &[Vec<f64>]) -> PerceptionResult<Vec<Vec<f64>>> {
    let n = matrix.len();
    if n == 0 {
        return Err(PerceptionError::InvalidConfiguration(
            "cannot invert empty matrix".into(),
        ));
    }
    for row in matrix {
        if row.len() != n {
            return Err(PerceptionError::InvalidConfiguration(
                "matrix must be square".into(),
            ));
        }
    }
    let mut augmented = vec![vec![0.0; 2 * n]; n];
    for i in 0..n {
        for j in 0..n {
            augmented[i][j] = matrix[i][j];
        }
        augmented[i][n + i] = 1.0;
    }
    for col in 0..n {
        let mut max_row = col;
        for row in (col + 1)..n {
            if augmented[row][col].abs() > augmented[max_row][col].abs() {
                max_row = row;
            }
        }
        augmented.swap(col, max_row);
        let pivot = augmented[col][col];
        if pivot.abs() < 1e-12 {
            return Err(PerceptionError::ComputationError(
                "matrix is singular and cannot be inverted".into(),
            ));
        }
        for j in 0..(2 * n) {
            augmented[col][j] /= pivot;
        }
        for row in 0..n {
            if row != col {
                let factor = augmented[row][col];
                for j in 0..(2 * n) {
                    augmented[row][j] -= factor * augmented[col][j];
                }
            }
        }
    }
    let mut inverse = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            inverse[i][j] = augmented[i][n + j];
        }
    }
    Ok(inverse)
}

fn identity_matrix(n: usize) -> Vec<Vec<f64>> {
    let mut matrix = vec![vec![0.0; n]; n];
    for i in 0..n {
        matrix[i][i] = 1.0;
    }
    matrix
}