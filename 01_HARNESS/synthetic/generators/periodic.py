# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
"""Periodic generator with known SPC.

Generates action sequences with periodic structure.
Ground truth: SPC ≈ 1.0 (perfectly predictable).
"""
import math
import random
from typing import List, Tuple, Any

from instruments.log_loss import normalized_log_loss


def generate(
    n_actions: int = 5,
    n_steps: int = 10000,
    period: int = 7,
    noise: float = 0.0,
    seed: int = 42,
) -> Tuple[List[List[float]], List[int], float, str]:
    """Generate a periodic action sequence.

    The policy predicts the periodic pattern. With noise=0, SPC ≈ 1.0.
    With noise > 0, SPC degrades proportionally.

    Args:
        n_actions: Size of action space.
        n_steps: Number of steps to generate.
        period: Period of the action cycle.
        noise: Standard deviation of Gaussian noise added to logits.
        seed: Random seed.

    Returns:
        Tuple of (predictions, actuals, true_spc, description).
    """
    rng = random.Random(seed)
    pattern = [rng.randint(0, n_actions - 1) for _ in range(period)]

    predictions: List[List[float]] = []
    actuals: List[int] = []

    for t in range(n_steps):
        ideal_action = pattern[t % period]

        # Build prediction distribution
        logits = [-5.0] * n_actions  # low base
        if noise > 0:
            for i in range(n_actions):
                logits[i] += rng.gauss(0, noise)
        logits[ideal_action] = 10.0  # strong bias

        exps = [math.exp(l - max(logits)) for l in logits]
        total = sum(exps)
        probs = [e / total for e in exps]

        predictions.append(probs)

        # Sample actual from the distribution (adds stochasticity)
        actual = rng.choices(range(n_actions), weights=probs)[0]
        actuals.append(actual)

    true_spc = 1.0 - normalized_log_loss(predictions, actuals, n_actions)
    desc = (
        f"Periodic: SPC={true_spc:.4f}, period={period}, noise={noise}, "
        f"n_actions={n_actions}, n_steps={n_steps}"
    )

    return predictions, actuals, true_spc, desc


if __name__ == "__main__":
    import sys
    sys.path.insert(0, ".")
    preds, acts, spc, desc = generate()
    print(desc)
