# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
"""Bootstrap confidence interval computation.

Resamples data with replacement to estimate the sampling distribution
of a statistic and compute confidence intervals.
"""
import random
from typing import Callable, Sequence, Tuple


def bootstrap_ci(
    data: Sequence[float],
    statistic: Callable[[Sequence[float]], float],
    n_resamples: int = 1000,
    confidence: float = 0.95,
    rng_seed: int | None = None,
) -> Tuple[float, float, float]:
    """Compute bootstrap confidence interval for a statistic.

    Args:
        data: Sample data.
        statistic: Function computing the statistic (e.g., mean, spc).
        n_resamples: Number of bootstrap resamples.
        confidence: Confidence level (e.g., 0.95 for 95% CI).
        rng_seed: Optional seed for reproducibility.

    Returns:
        Tuple of (statistic_value, ci_lower, ci_upper).
    """
    if not data:
        return (0.0, 0.0, 0.0)

    if rng_seed is not None:
        rng = random.Random(rng_seed)
    else:
        rng = random.Random()

    stat_value = statistic(list(data))
    resampled_stats = []

    n = len(data)
    for _ in range(n_resamples):
        sample = [rng.choice(data) for _ in range(n)]
        resampled_stats.append(statistic(sample))

    resampled_stats.sort()

    alpha = 1.0 - confidence
    lower_idx = int((alpha / 2.0) * n_resamples)
    upper_idx = int((1.0 - alpha / 2.0) * n_resamples)

    ci_lower = resampled_stats[lower_idx]
    ci_upper = resampled_stats[upper_idx]

    return (stat_value, ci_lower, ci_upper)


def bootstrap_pvalue(
    data: Sequence[float],
    statistic: Callable[[Sequence[float]], float],
    null_value: float = 0.0,
    n_resamples: int = 1000,
    rng_seed: int | None = None,
) -> float:
    """Compute a one-sided p-value via bootstrap.

    Tests H₀: statistic = null_value vs H₁: statistic > null_value.

    Args:
        data: Sample data.
        statistic: Function computing the statistic.
        null_value: The null hypothesis value.
        n_resamples: Number of bootstrap resamples.
        rng_seed: Optional seed for reproducibility.

    Returns:
        One-sided p-value in [0, 1].
    """
    if not data:
        return 1.0

    if rng_seed is not None:
        rng = random.Random(rng_seed)
    else:
        rng = random.Random()

    observed = statistic(list(data))
    count_extreme = 0
    n = len(data)

    for _ in range(n_resamples):
        sample = [rng.choice(data) - null_value for _ in range(n)]
        if statistic(sample) >= observed - null_value:
            count_extreme += 1

    return count_extreme / n_resamples


__all__ = ["bootstrap_ci", "bootstrap_pvalue"]
