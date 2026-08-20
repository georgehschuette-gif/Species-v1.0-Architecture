// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Feedback Loop Attractor
//!
//! A [`FeedbackLoop`] models a closed signal or control loop where the
//! output of a system is fed back as input, creating recursive dynamics.
//! Feedback loops are fundamental to control theory, neural networks,
//! ecological systems, and economic models.
//!
//! ## Stability
//!
//! Feedback loop stability is analyzed via the Nyquist criterion, root
//! locus, and Bode plots. Positive feedback tends to destabilize (runaway
//! growth), while negative feedback can stabilize or induce oscillations
//! depending on phase lag.
//!
//! ## Transfer Functions
//!
//! The loop transfer function `L(s) = G(s)H(s)` determines the
//! open-loop gain and phase margin, which predict closed-loop behavior.

use std::cmp::PartialEq;
use std::fmt::{self, Display, Formatter};

/// Feedback polarity classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FeedbackPolarity {
    /// Output reinforces input (destabilizing).
    Positive,
    /// Output opposes input (stabilizing).
    Negative,
    /// Polarity switches depending on operating point.
    Mixed,
}

impl Display for FeedbackPolarity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Positive => "positive",
            Self::Negative => "negative",
            Self::Mixed => "mixed",
        })
    }
}

/// A feedback loop attractor with gain, delay, and transfer function.
///
/// # Fields
///
/// * `gain` - Open-loop gain magnitude.
/// * `delay` - Time delay in the feedback path.
/// * `phase_lag` - Phase shift introduced by the loop (radians).
/// * `polarity` - Feedback polarity.
/// * `natural_frequency` - Undamped natural frequency of the loop.
/// * `damping_ratio` - Loop damping ratio.
/// * `transfer_coeffs` - Numerator coefficients of the transfer function.
/// * `bandwidth` - -3 dB bandwidth of the loop.
#[derive(Debug, Clone, PartialEq)]
pub struct FeedbackLoop {
    pub gain: f64,
    pub delay: f64,
    pub phase_lag: f64,
    pub polarity: FeedbackPolarity,
    pub natural_frequency: f64,
    pub damping_ratio: f64,
    pub transfer_coeffs: Vec<f64>,
    pub bandwidth: f64,
}

impl FeedbackLoop {
    /// Creates a new feedback loop with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `gain` - Open-loop gain (must be non-negative).
    /// * `delay` - Feedback path delay.
    /// * `phase_lag` - Phase shift in radians.
    /// * `polarity` - Feedback polarity.
    /// * `natural_frequency` - Natural frequency ωₙ > 0.
    /// * `transfer_coeffs` - Transfer function numerator coefficients.
    ///
    /// # Panics
    ///
    /// Panics if `gain` is negative or `natural_frequency` is
    /// non-positive.
    pub fn new(
        gain: f64,
        delay: f64,
        phase_lag: f64,
        polarity: FeedbackPolarity,
        natural_frequency: f64,
        transfer_coeffs: Vec<f64>,
    ) -> Self {
        assert!(gain >= 0.0, "gain must be non-negative");
        assert!(natural_frequency > 0.0, "natural_frequency must be positive");
        assert!(!transfer_coeffs.is_empty(), "transfer_coeffs must be non-empty");
        let bandwidth = if gain > 1e-12 { natural_frequency * gain.sqrt() } else { natural_frequency };
        Self {
            gain,
            delay,
            phase_lag,
            polarity,
            natural_frequency,
            damping_ratio: 0.0,
            transfer_coeffs,
            bandwidth,
        }
    }

    /// Sets the damping ratio and updates derived quantities.
    pub fn set_damping_ratio(&mut self, damping_ratio: f64) {
        assert!(damping_ratio >= 0.0, "damping_ratio must be non-negative");
        self.damping_ratio = damping_ratio;
    }

    /// Returns the closed-loop damping ratio.
    pub fn damping_ratio(&self) -> f64 {
        self.damping_ratio
    }

    /// Computes the step response amplitude at time `t`.
    ///
    /// For underdamped second-order systems, this follows the standard
    /// damped sinusoid.
    ///
    /// # Arguments
    ///
    /// * `t` - Time.
    pub fn step_response(&self, t: f64) -> f64 {
        let zeta = self.damping_ratio;
        let omega_n = self.natural_frequency;
        if zeta < 1.0 {
            let omega_d = omega_n * (1.0 - zeta.powi(2)).sqrt();
            let decay = (-zeta * omega_n * t).exp();
            decay * (omega_d * t).sin() / omega_d
        } else if (zeta - 1.0).abs() < 1e-9 {
            let decay = (-omega_n * t).exp();
            decay * omega_n * t
        } else {
            let s1 = -omega_n * (zeta - (zeta.powi(2) - 1.0).sqrt());
            let s2 = -omega_n * (zeta + (zeta.powi(2) - 1.0).sqrt());
            (s1 * (s2 * t).exp() - s2 * (s1 * t).exp()) / (s1 - s2)
        }
    }

    /// Computes the magnitude of the frequency response at `omega`.
    ///
    /// # Arguments
    ///
    /// * `omega` - Angular frequency.
    pub fn frequency_response(&self, omega: f64) -> f64 {
        let zeta = self.damping_ratio;
        let omega_n = self.natural_frequency;
        let num = omega_n.powi(2);
        let den = ((omega_n.powi(2) - omega.powi(2)).powi(2) + (2.0 * zeta * omega_n * omega).powi(2)).sqrt();
        num / den
    }

    /// Checks whether the loop exhibits oscillatory behavior.
    ///
    /// Oscillations are expected when the damping ratio is below unity.
    pub fn is_oscillatory(&self) -> bool {
        self.damping_ratio < 1.0 - 1e-9
    }

    /// Computes the phase margin from the loop transfer function.
    ///
    /// Phase margin is the additional phase lag required to reach -180°
    /// at the gain crossover frequency.
    pub fn phase_margin(&self) -> f64 {
        let omega_gc = self.gain_crossover();
        let phase = self.loop_phase(omega_gc);
        (std::f64::consts::PI + phase).min(2.0 * std::f64::consts::PI)
    }

    /// Finds the approximate gain crossover frequency.
    pub fn gain_crossover(&self) -> f64 {
        let mut omega = self.natural_frequency;
        for _ in 0..64 {
            let mag = self.frequency_response(omega);
            if (mag - 1.0).abs() < 1e-9 {
                break;
            }
            omega *= if mag > 1.0 { 0.5 } else { 2.0 };
        }
        omega
    }

    /// Computes the loop phase at a given frequency.
    fn loop_phase(&self, omega: f64) -> f64 {
        let zeta = self.damping_ratio;
        let omega_n = self.natural_frequency;
        let num = -(2.0 * zeta * omega_n * omega).atan2(omega_n.powi(2) - omega.powi(2));
        num - self.phase_lag - self.delay * omega
    }

    /// Returns a summary string.
    pub fn summary(&self) -> String {
        format!(
            "FeedbackLoop(gain={:.4}, delay={:.4}, ζ={:.4}, polarity={})",
            self.gain, self.delay, self.damping_ratio, self.polarity
        )
    }
}

impl Display for FeedbackLoop {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FeedbackLoop(gain={:.6}, delay={:.6}, polarity={})",
            self.gain, self.delay, self.polarity
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oscillatory_detection() {
        let mut loop_ = FeedbackLoop::new(1.0, 0.0, 0.0, FeedbackPolarity::Negative, 1.0, vec![1.0]);
        loop_.set_damping_ratio(0.5);
        assert!(loop_.is_oscillatory());
    }

    #[test]
    fn test_step_response_shape() {
        let mut loop_ = FeedbackLoop::new(1.0, 0.0, 0.0, FeedbackPolarity::Negative, 1.0, vec![1.0]);
        loop_.set_damping_ratio(0.707);
        let val = loop_.step_response(0.1);
        assert!(val.is_finite());
    }

    #[test]
    fn test_bandwidth_scaling() {
        let loop_ = FeedbackLoop::new(4.0, 0.0, 0.0, FeedbackPolarity::Negative, 1.0, vec![1.0]);
        assert!((loop_.bandwidth - 2.0).abs() < 1e-10);
    }
}
