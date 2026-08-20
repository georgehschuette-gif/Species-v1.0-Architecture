// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// ForecastingModel: The statistical model used for temporal prediction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForecastingModel {
    ExponentialSmoothing,
    LinearTrend,
    Seasonal,
    ARIMA,
}

impl ForecastingModel {
    pub fn as_str(&self) -> &'static str {
        match self {
            ForecastingModel::ExponentialSmoothing => "exponential_smoothing",
            ForecastingModel::LinearTrend => "linear_trend",
            ForecastingModel::Seasonal => "seasonal",
            ForecastingModel::ARIMA => "arima",
        }
    }

    pub fn supports_seasonality(&self) -> bool {
        matches!(self, ForecastingModel::Seasonal | ForecastingModel::ARIMA)
    }

    pub fn default_horizon(&self) -> f64 {
        match self {
            ForecastingModel::ExponentialSmoothing => 10.0,
            ForecastingModel::LinearTrend => 20.0,
            ForecastingModel::Seasonal => 30.0,
            ForecastingModel::ARIMA => 15.0,
        }
    }
}

/// TemporalForecasting: Predicted future cognitive states.
pub struct TemporalForecasting {
    pub horizon: f64,
    pub predicted_states: Vec<f64>,
    pub confidence_interval: (f64, f64),
    pub model: ForecastingModel,
}

impl TemporalForecasting {
    pub fn new(horizon: f64, model: ForecastingModel) -> Result<Self, TemporalError> {
        if !horizon.is_finite() || horizon <= 0.0 {
            return Err(TemporalError::InvalidHorizon { horizon });
        }
        Ok(Self { horizon, predicted_states: Vec::new(), confidence_interval: (0.0, 1.0), model })
    }

    pub fn add_prediction(&mut self, state: f64) {
        self.predicted_states.push(state);
    }

    pub fn latest_prediction(&self) -> Option<f64> {
        self.predicted_states.last().copied()
    }

    pub fn confidence_width(&self) -> f64 {
        self.confidence_interval.1 - self.confidence_interval.0
    }

    pub fn model_name(&self) -> &'static str {
        self.model.as_str()
    }

    pub fn is_ready(&self) -> bool {
        !self.predicted_states.is_empty() && self.confidence_width() > 0.0
    }
}
