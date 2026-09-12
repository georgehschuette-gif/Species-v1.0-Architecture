# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
"""Brier score for probabilistic predictions.

Measures the mean squared difference between predicted probability
and actual outcome (one-hot encoded). Lower is better.
Range: [0, 2] (0 = perfect, 2 = worst possible for binary).
"""
from typing import Sequence


def brier_score(
    predictions: Sequence[Sequence[float]],
    actuals: Sequence[int],
) -> float:
    """Compute the Brier score.

    Args:
        predictions: List of probability distributions.
        actuals: List of actual action indices.

    Returns:
        float in [0, 2]. Lower is better.

    Raises:
        ValueError: If lengths mismatch.
    """
    if len(predictions) != len(actuals):
        raise ValueError("predictions and actuals must have equal length")
    if len(predictions) == 0:
        return 0.0

    total = 0.0
    for pred, actual in zip(predictions, actuals):
        for i, p in enumerate(pred):
            target = 1.0 if i == actual else 0.0
            total += (p - target) ** 2
    return total / len(predictions)


def brier_score_multi(
    predictions: Sequence[Sequence[float]],
    actuals: Sequence[int],
) -> float:
    """Multi-class Brier score normalized to [0, 1].

    Divides by 2.0 to normalize the theoretical max of 2.0 to 1.0.
    """
    return brier_score(predictions, actuals) / 2.0


__all__ = ["brier_score", "brier_score_multi"]
