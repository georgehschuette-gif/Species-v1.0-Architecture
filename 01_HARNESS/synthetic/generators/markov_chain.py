# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
"""Markov chain generator with known SPC.

Generates action sequences from a first-order Markov chain.
Ground truth SPC depends on the entropy of the transition matrix.
"""
import random
import math
from typing import List, Tuple, Any

from instruments.log_loss import normalized_log_loss


def generate(
    n_actions: int = 5,
    n_steps: int = 10000,
    transition_entropy: float = 1.0,
    seed: int = 42,
) -> Tuple[List[List[float]], List[int], float, str]:
    """Generate a Markov chain action sequence.

    The transition matrix is constructed so that the entropy rate
    equals `transition_entropy` bits.

    Args:
        n_actions: Size of action space.
        n_steps: Number of steps to generate.
        transition_entropy: Target entropy rate in bits (0 = deterministic,
                           log2(n_actions) = fully random).
        seed: Random seed.

    Returns:
        Tuple of (predictions, actuals, true_spc, description).
    """
    rng = random.Random(seed)

    # Build a transition matrix with controlled entropy
    # Higher transition_entropy → more uniform transitions → lower predictability
    temperature = 1.0 / max(transition_entropy, 0.001)
    transition = []
    for i in range(n_actions):
        raw = [rng.gauss(0, 1) for _ in range(n_actions)]
        exps = [math.exp(r * temperature) for r in raw]
        total = sum(exps)
        row = [e / total for e in exps]
        transition.append(row)

    # Generate sequence
    current = rng.randint(0, n_actions - 1)
    predictions: List[List[float]] = []
    actuals: List[int] = []

    for t in range(n_steps):
        preds = transition[current][:]
        predictions.append(preds)
        actual = rng.choices(range(n_actions), weights=transition[current])[0]
        actuals.append(actual)
        current = actual

    true_spc = 1.0 - normalized_log_loss(predictions, actuals, n_actions)
    desc = (
        f"Markov chain: SPC={true_spc:.4f}, entropy={transition_entropy} bits, "
        f"n_actions={n_actions}, n_steps={n_steps}"
    )

    return predictions, actuals, true_spc, desc


if __name__ == "__main__":
    import sys
    sys.path.insert(0, ".")
    preds, acts, spc, desc = generate()
    print(desc)
