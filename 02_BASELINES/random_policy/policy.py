# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
"""Random policy: uniform action selection, uniform predictions.

Expected SPC: 0.0 ± 0.01
Expected behavioral entropy: ~log2(5) ≈ 2.32 bits
"""
import random
from typing import List


def policy_fn(obs, env, tick: int) -> int:
    """Select action uniformly at random."""
    return random.Random(tick + 12345).randint(0, env.cfg.n_actions - 1)


def predict_fn(obs, history, env, tick: int) -> List[float]:
    """Predict uniform distribution over actions."""
    return [1.0 / env.cfg.n_actions] * env.cfg.n_actions


if __name__ == "__main__":
    import json
    import os, sys
    _BASE = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
    sys.path.insert(0, os.path.join(_BASE, "01_HARNESS"))
    sys.path.insert(0, os.path.join(_BASE, "02_BASELINES"))
    from environment.runner import run_baseline

    result = run_baseline(
        policy_fn, predict_fn,
        n_steps=10000, seed=42, name="random_policy",
        log_dir=os.path.join(os.path.dirname(os.path.abspath(__file__)), "logs"),
    )
    print(json.dumps(result, indent=2))
