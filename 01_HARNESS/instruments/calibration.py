# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
"""Calibration analysis: reliability diagrams and ECE.

Calibration measures whether predicted probabilities match empirical
frequencies. A well-calibrated predictor outputs P(action) = 0.9
exactly 90% of the time.
"""
from typing import Sequence, List, Tuple, Dict, Any
import math


class CalibrationResult:
    """Result of calibration analysis."""

    def __init__(self):
        self.bins: List[Dict[str, float]] = []
        self.ece: float = 0.0
        self.slope: float = 0.0
        self.intercept: float = 0.0
        self.rmse: float = 0.0
        self.n_predictions: int = 0


def calibrate(
    predictions: Sequence[Sequence[float]],
    actuals: Sequence[int],
    n_bins: int = 10,
) -> CalibrationResult:
    """Compute calibration metrics from prediction/actual pairs.

    Args:
        predictions: List of probability distributions.
        actuals: List of actual action indices.
        n_bins: Number of bins for the reliability diagram.

    Returns:
        CalibrationResult with ECE, slope, intercept, RMSE.
    """
    if not predictions or not actuals:
        return CalibrationResult()

    result = CalibrationResult()
    result.n_predictions = len(predictions)

    # Collect (predicted_prob, was_correct) pairs for the top predicted action
    prob_correct: List[Tuple[float, int]] = []
    for pred, actual in zip(predictions, actuals):
        max_idx = max(range(len(pred)), key=lambda i: pred[i])
        max_prob = pred[max_idx]
        correct = 1 if max_idx == actual else 0
        prob_correct.append((max_prob, correct))

    # Bin by predicted probability
    bin_counts: List[int] = [0] * n_bins
    bin_correct: List[int] = [0] * n_bins
    bin_prob_sum: List[float] = [0.0] * n_bins

    for prob, correct in prob_correct:
        bin_idx = min(int(prob * n_bins), n_bins - 1)
        bin_counts[bin_idx] += 1
        bin_correct[bin_idx] += correct
        bin_prob_sum[bin_idx] += prob

    total_prob_diff = 0.0
    total_count = 0

    for i in range(n_bins):
        if bin_counts[i] > 0:
            bin_acc = bin_correct[i] / bin_counts[i]
            bin_avg_prob = bin_prob_sum[i] / bin_counts[i]
            bin_prob_diff = abs(bin_acc - bin_avg_prob)
            total_prob_diff += bin_prob_diff * bin_counts[i]
            total_count += bin_counts[i]
            result.bins.append({
                "bin": i,
                "count": bin_counts[i],
                "avg_prob": bin_avg_prob,
                "accuracy": bin_acc,
                "prob_diff": bin_prob_diff,
            })
        else:
            result.bins.append({
                "bin": i,
                "count": 0,
                "avg_prob": float(i + 0.5) / n_bins,
                "accuracy": 0.0,
                "prob_diff": 0.0,
            })

    result.ece = total_prob_diff / total_count if total_count > 0 else 0.0

    # Linear regression for slope/intercept (predicted vs. actual)
    xs = [b["avg_prob"] for b in result.bins if b["count"] > 0]
    ys = [b["accuracy"] for b in result.bins if b["count"] > 0]

    if len(xs) >= 2:
        n = len(xs)
        mean_x = sum(xs) / n
        mean_y = sum(ys) / n
        ss_xy = sum((x - mean_x) * (y - mean_y) for x, y in zip(xs, ys))
        ss_xx = sum((x - mean_x) ** 2 for x in xs)
        if ss_xx > 0:
            result.slope = ss_xy / ss_xx
            result.intercept = mean_y - result.slope * mean_x
        result.rmse = math.sqrt(sum(
            (y_ - (result.slope * x_ + result.intercept)) ** 2
            for x_, y_ in zip(xs, ys)
        ) / n) if n > 0 else 0.0
    else:
        result.slope = 0.0
        result.intercept = 0.0
        result.rmse = 0.0

    return result


def is_calibrated(result: CalibrationResult,
                  slope_min: float = 0.9,
                  intercept_max: float = 0.1) -> bool:
    """Check if calibration meets thresholds.

    Args:
        result: CalibrationResult from calibrate().
        slope_min: Minimum acceptable slope.
        intercept_max: Maximum acceptable |intercept|.

    Returns:
        True if calibration thresholds are met.
    """
    return result.slope >= slope_min and abs(result.intercept) <= intercept_max


__all__ = ["CalibrationResult", "calibrate", "is_calibrated"]
