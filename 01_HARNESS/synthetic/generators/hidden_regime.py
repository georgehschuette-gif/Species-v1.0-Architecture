# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
"""Hidden regime generator with known SPC.

Generates action sequences from a Hidden Markov Model with regime switches.
Ground truth SPC depends on the switching frequency and transition entropy.
"""
import random
import math
from typing import List, Tuple, Any

from instruments.log_loss import normalized_log_loss


def generate(
    n_actions: int = 5,
    n_steps: int = 10000,
    regime_length: int = 2000,
    regime_noise: float = 0.0,
    seed: int = 42,
) -> Tuple[List[List[float]], List[int], float, str]:
    """Generate a hidden regime-switching sequence.

    Each regime has a preferred action. Regimes switch every `regime_length`
    steps with some uncertainty (regime_noise). The true SPC is computed
    from the policy's predictions vs actuals.

    Args:
        n_actions: Size of action space.
        n_steps: Number of steps to generate.
        regime_length: Steps before a regime switch.
        regime_noise: Noise in regime preference (0 = deterministic regimes).
        seed: Random seed.

    Returns:
        Tuple of (predictions, actuals, true_spc, description).
    """
    rng = random.Random(seed)

    predictions: List[List[float]] = []
    actuals: List[int] = []

    current_regime = rng.randint(0, n_actions - 1)

    for t in range(n_steps):
        # Switch regime occasionally
        if t > 0 and t % regime_length == 0:
            current_regime = rng.randint(0, n_actions - 1)

        # Build prediction: strong preference for regime action
        logits = [-3.0] * n_actions
        if regime_noise > 0:
            for i in range(n_actions):
                logits[i] += rng.gauss(0, regime_noise)
        logits[current_regime] = 5.0

        exps = [math.exp(l - max(logits)) for l in logits]
        total = sum(exps)
        probs = [e / total for e in exps]

        predictions.append(probs)
        actual = rng.choices(range(n_actions), weights=probs)[0]
        actuals.append(actual)

    true_spc = 1.0 - normalized_log_loss(predictions, actuals, n_actions)
    desc = (
        f"Hidden regime: SPC={true_spc:.4f}, regime_len={regime_length}, "
        f"noise={regime_noise}, n_actions={n_actions}, n_steps={n_steps}"
    )

    return predictions, actuals, true_spc, desc


if __name__ == "__main__":
    import sys
    sys.path.insert(0, ".")
    preds, acts, spc, desc = generate()
    print(desc)
