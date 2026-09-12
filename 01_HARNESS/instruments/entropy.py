# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
"""Entropy calculations: Shannon, behavioral, and environmental.

Shannon entropy measures uncertainty in a probability distribution.
Behavioral entropy measures diversity of actions taken.
Environmental entropy measures diversity of observations received.
"""
import math
from collections import Counter
from typing import Sequence


def shannon_entropy(freqs: Sequence[float]) -> float:
    """Compute Shannon entropy (in bits) from a frequency/count sequence.

    Args:
        freqs: Non-negative counts or frequencies.

    Returns:
        Entropy in bits. 0 if all mass is on one item.
    """
    total = sum(freqs)
    if total <= 0:
        return 0.0

    entropy = 0.0
    for f in freqs:
        if f > 0:
            p = f / total
            entropy -= p * math.log2(p)
    return entropy


def distribution_entropy(probs: Sequence[float]) -> float:
    """Compute Shannon entropy from a probability distribution.

    Args:
        probs: A probability distribution (should sum to ~1.0).

    Returns:
        Entropy in bits.
    """
    entropy = 0.0
    for p in probs:
        if p > 0:
            entropy -= p * math.log2(p)
    return entropy


def behavioral_entropy(actions: Sequence[int]) -> float:
    """Compute behavioral entropy from a sequence of actions.

    Args:
        actions: Sequence of action indices.

    Returns:
        Shannon entropy (bits) of the action frequency distribution.
    """
    if not actions:
        return 0.0
    counts = Counter(actions)
    return shannon_entropy(list(counts.values()))


def environmental_entropy(observations: Sequence[Sequence[float]]) -> float:
    """Compute environmental entropy from observation vectors.

    Discretizes continuous observations by rounding to 6 decimal places
    and computing Shannon entropy over the resulting symbols.

    Args:
        observations: Sequence of observation feature vectors.

    Returns:
        Shannon entropy (bits) of the observation distribution.
    """
    if not observations:
        return 0.0

    # Discretize: convert each observation to a hashable tuple
    symbols = []
    for obs in observations:
        rounded = tuple(round(v, 6) for v in obs)
        symbols.append(rounded)

    counts = Counter(symbols)
    return shannon_entropy(list(counts.values()))


def low_information_filter(
    actions: Sequence[int],
    min_entropy: float = 0.5,
) -> bool:
    """Detect whether a behavior sequence is too low-entropy to be meaningful.

    Used to filter out degenerate policies (e.g., "always do nothing")
    that achieve high SPC trivially.

    Args:
        actions: Sequence of action indices.
        min_entropy: Minimum behavioral entropy in bits.

    Returns:
        True if the behavior is too low-entropy (should be flagged), False otherwise.
    """
    return behavioral_entropy(actions) < min_entropy


__all__ = [
    "shannon_entropy",
    "distribution_entropy",
    "behavioral_entropy",
    "environmental_entropy",
    "low_information_filter",
]
