# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
"""Adversarial generator designed to fool SPC metrics.

Attempts to achieve high SPC through metric gaming rather than
genuine self-prediction. Examples:
- Constant action (high SPC, zero behavioral entropy)
- Oscillating pattern (predictable but low information)
- Overfit to calibration bins

Ground truth: These should be REJECTED by the low-information filter.
"""
import random
import math
from typing import List, Tuple, Any

from instruments.log_loss import normalized_log_loss
from instruments.entropy import behavioral_entropy


def generate_constant(
    n_actions: int = 5,
    n_steps: int = 10000,
    fixed_action: int = 0,
    seed: int = 42,
) -> Tuple[List[List[float]], List[int], float, float, str]:
    """Generate a constant-action sequence (gaming attempt).

    The policy always predicts the fixed action with high confidence.
    SPC ≈ 1.0, but behavioral entropy = 0 (degenerate).

    Returns:
        Tuple of (predictions, actuals, true_spc, behavioral_entropy, description).
    """
    rng = random.Random(seed)

    predictions: List[List[float]] = []
    actuals: List[int] = []

    for t in range(n_steps):
        probs = [0.01] * n_actions
        probs[fixed_action] = 0.97
        predictions.append(probs)
        actuals.append(fixed_action)  # Always the same

    true_spc = 1.0 - normalized_log_loss(predictions, actuals, n_actions)
    beh_ent = behavioral_entropy(actuals)
    desc = f"Constant action: SPC={true_spc:.4f}, beh_entropy={beh_ent:.4f} bits"

    return predictions, actuals, true_spc, beh_ent, desc


def generate_oscillating(
    n_actions: int = 5,
    n_steps: int = 10000,
    period: int = 3,
    confidence: float = 0.95,
    seed: int = 42,
) -> Tuple[List[List[float]], List[int], float, float, str]:
    """Generate an oscillating sequence (gaming attempt).

    The policy oscillates between two actions with high confidence.
    SPC ≈ 1.0, but behavioral entropy is very low (log2(2) = 1 bit max).

    Returns:
        Tuple of (predictions, actuals, true_spc, behavioral_entropy, description).
    """
    rng = random.Random(seed)

    predictions: List[List[float]] = []
    actuals: List[int] = []

    for t in range(n_steps):
        action = t % 2
        probs = [0.01] * n_actions
        probs[action] = confidence
        probs[1 - action] = 1.0 - confidence - 0.01
        predictions.append(probs)
        actuals.append(action)

    true_spc = 1.0 - normalized_log_loss(predictions, actuals, n_actions)
    beh_ent = behavioral_entropy(actuals)
    desc = f"Oscillating: SPC={true_spc:.4f}, beh_entropy={beh_ent:.4f} bits, period={period}"

    return predictions, actuals, true_spc, beh_ent, desc


def generate_calibration_gaming(
    n_actions: int = 5,
    n_steps: int = 10000,
    seed: int = 42,
) -> Tuple[List[List[float]], List[int], float, float, str]:
    """Generate a calibration-gaming sequence.

    The policy outputs probabilities that match the calibration
    histogram but doesn't actually predict the true action.
    SPC ≈ 0 (no real predictive power), but Brier score looks good.

    Returns:
        Tuple of (predictions, actuals, true_spc, behavioral_entropy, description).
    """
    rng = random.Random(seed)

    predictions: List[List[float]] = []
    actuals: List[int] = []

    for t in range(n_steps):
        # Generate "calibrated" probabilities
        probs = [rng.betavariate(1, 1) for _ in range(n_actions)]
        total = sum(probs)
        probs = [p / total for p in probs]

        predictions.append(probs)
        actual = rng.randint(0, n_actions - 1)  # Random actual
        actuals.append(actual)

    true_spc = 1.0 - normalized_log_loss(predictions, actuals, n_actions)
    beh_ent = behavioral_entropy(actuals)
    desc = f"Calibration gaming: SPC={true_spc:.4f}, beh_entropy={beh_ent:.4f} bits"

    return predictions, actuals, true_spc, beh_ent, desc


if __name__ == "__main__":
    import sys
    sys.path.insert(0, ".")
    preds, acts, spc, ent, desc = generate_constant()
    print(desc)
    preds, acts, spc, ent, desc = generate_oscillating()
    print(desc)
    preds, acts, spc, ent, desc = generate_calibration_gaming()
    print(desc)
