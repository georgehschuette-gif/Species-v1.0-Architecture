# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
"""Random walk generator with known SPC.

Generates action sequences from a random walk over the action space.
Ground truth: SPC ≈ 0.0 (no predictive structure).
"""
import random
from typing import List, Tuple, Any

from instruments.log_loss import normalized_log_loss


def generate(
    n_actions: int = 5,
    n_steps: int = 10000,
    seed: int = 42,
) -> Tuple[List[List[float]], List[int], float, str]:
    """Generate a random walk action sequence.

    The policy predicts a random walk transition: P(next) depends on
    a hidden state that drifts randomly. Ground truth SPC is computed
    by the policy's own predictions vs actuals.

    Args:
        n_actions: Size of action space.
        n_steps: Number of steps to generate.
        seed: Random seed for reproducibility.

    Returns:
        Tuple of (predictions, actuals, true_spc, description).
    """
    rng = random.Random(seed)

    predictions: List[List[float]] = []
    actuals: List[int] = []

    # Random walk over a latent state
    latent = [0.0] * n_actions
    for t in range(n_steps):
        # Drift the latent state
        noise = rng.gauss(0, 0.3)
        max_idx = max(range(n_actions), key=lambda i: latent[i])
        latent[max_idx] += noise
        for i in range(n_actions):
            latent[i] -= noise / n_actions  # diffusion

        # Convert latent to probability distribution (softmax)
        max_l = max(latent)
        exps = [random.Random(seed + t * 1000 + i).exp() if False else
                __import__('math').exp(l - max_l) for l in latent]
        total = sum(exps)
        probs = [e / total for e in exps]

        predictions.append(probs)
        actual = rng.choices(range(n_actions), weights=probs)[0]
        actuals.append(actual)

    true_spc = 1.0 - normalized_log_loss(predictions, actuals, n_actions)
    desc = f"Random walk: SPC={true_spc:.4f}, n_actions={n_actions}, n_steps={n_steps}"

    return predictions, actuals, true_spc, desc


if __name__ == "__main__":
    import sys
    sys.path.insert(0, ".")
    preds, acts, spc, desc = generate()
    print(desc)
