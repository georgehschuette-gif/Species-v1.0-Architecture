# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
"""Environment-only policy: predicts observations, not own actions.

The policy predicts the next observation using a simple model
(current observation is a good predictor in a deterministic world).
It uses this prediction to select actions.
But its *action* prediction (for SPC) is uniform — it does not
predict itself.

Expected SPC: ~0.0 (uniform action predictions)
Expected behavioral entropy: ~1.0-1.5 bits (structured, not random)
"""
import random
from typing import List


def policy_fn(obs, env, tick: int) -> int:
    """Use predicted observation to move toward objects.

    In a deterministic world, the predicted observation is the current
    observation (no change expected). So we move toward nearest object.
    """
    rng = random.Random(tick + 777)
    if rng.random() < 0.15:
        return rng.randint(0, env.cfg.n_actions - 1)
    else:
        return env.action_to_nearest()


def predict_fn(obs, history, env, tick: int) -> List[float]:
    """Predict next observation (not action). Action prediction = uniform.

    This system predicts OBSERVATIONS, not its OWN ACTIONS.
    """
    n = env.cfg.n_actions
    return [1.0 / n] * n


def predict_observation(obs: List[float], history: List[List[float]],
                        env, tick: int) -> List[float]:
    """Predict the next observation (for context only, not used for SPC)."""
    return list(obs)


if __name__ == "__main__":
    import json
    import os, sys
    _BASE = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
    sys.path.insert(0, os.path.join(_BASE, "01_HARNESS"))
    sys.path.insert(0, os.path.join(_BASE, "02_BASELINES"))
    from environment.runner import run_baseline

    result = run_baseline(
        policy_fn, predict_fn,
        n_steps=10000, seed=42, name="environment_only",
        log_dir=os.path.join(os.path.dirname(os.path.abspath(__file__)), "logs"),
    )
    print(json.dumps(result, indent=2))