# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
"""Normalized log loss for probabilistic predictions.

Outputs a value in [0, 1] where 0 = no better than random, 1 = perfect.
Normalization: divide by log(|A|) so the score is independent of action space size.
"""
import math
from typing import Sequence


def normalized_log_loss(
    predictions: Sequence[Sequence[float]],
    actuals: Sequence[int],
    action_space_size: int,
) -> float:
    """Compute normalized log loss.

    Args:
        predictions: List of probability distributions (each sums to ~1.0).
        actuals: List of actual action indices taken.
        action_space_size: Total number of possible actions.

    Returns:
        float in [0, 1]. 0 = no better than uniform, 1 = perfect prediction.

    Raises:
        ValueError: If lengths mismatch or predictions are invalid.
    """
    if action_space_size < 2:
        raise ValueError("action_space_size must be >= 2")
    if len(predictions) != len(actuals):
        raise ValueError("predictions and actuals must have equal length")
    if len(predictions) == 0:
        return 0.0

    log_normalizer = math.log(action_space_size)
    total_nll = 0.0

    for pred, actual in zip(predictions, actuals):
        if actual < 0 or actual >= len(pred):
            raise ValueError(
                f"actual action {actual} out of range [0, {len(pred)})"
            )
        prob = pred[actual]
        if prob <= 0.0:
            prob = 1e-15
        elif prob > 1.0:
            raise ValueError(f"probability {prob} exceeds 1.0")
        total_nll -= math.log(prob)

    mean_nll = total_nll / len(predictions)
    normalized = mean_nll / log_normalizer
    return max(0.0, min(1.0, normalized))


def spc_score(
    predictions: Sequence[Sequence[float]],
    actuals: Sequence[int],
    action_space_size: int,
) -> float:
    """SPC = 1.0 - normalized_log_loss. Alias for the primary metric."""
    return 1.0 - normalized_log_loss(predictions, actuals, action_space_size)


__all__ = ["normalized_log_loss", "spc_score"]
