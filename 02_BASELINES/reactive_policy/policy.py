# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
"""Reactive policy: predicts last action, moves toward nearest object.

Expected SPC: ~0.55 (predictable behavior, 0.8 confidence on last action)
Expected behavioral entropy: ~1.0-1.5 bits (biased toward useful actions)
"""
import random
from typing import List

_last_action: int = 4


def policy_fn(obs, env, tick: int) -> int:
    """Move toward nearest object, with occasional random exploration."""
    global _last_action

    rng = random.Random(tick + 999)
    if rng.random() < 0.1:
        action = rng.randint(0, env.cfg.n_actions - 1)
    else:
        action = env.action_to_nearest()

    _last_action = action
    return action


def predict_fn(obs, history, env, tick: int) -> List[float]:
    """Predict last action with 0.8 probability, rest uniform."""
    global _last_action

    n = env.cfg.n_actions
    probs = [0.2 / (n - 1)] * n
    probs[_last_action] = 0.8
    return probs


if __name__ == "__main__":
    import json
    import os, sys
    _BASE = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
    sys.path.insert(0, os.path.join(_BASE, "01_HARNESS"))
    sys.path.insert(0, os.path.join(_BASE, "02_BASELINES"))
    from environment.runner import run_baseline

    result = run_baseline(
        policy_fn, predict_fn,
        n_steps=10000, seed=42, name="reactive_policy",
        log_dir=os.path.join(os.path.dirname(os.path.abspath(__file__)), "logs"),
    )
    print(json.dumps(result, indent=2))