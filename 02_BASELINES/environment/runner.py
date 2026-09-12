# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
"""Shared runner: runs a policy in the grid world, logs predictions,
computes SPC using the instrument suite from Phase 0A.
"""
import os
import sys
import json
import math
from typing import List, Tuple, Sequence

# Set up paths
_BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(_BASE_DIR, "..", "01_HARNESS"))

from instruments.log_loss import spc_score
from instruments.entropy import behavioral_entropy
from instruments.bootstrap_ci import bootstrap_ci
from environment.grid_world import GridWorld, GridWorldConfig, make_log_entry


def run_baseline(
    policy_fn,
    predict_fn,
    n_steps: int = 10000,
    seed: int = 42,
    name: str = "baseline",
    log_dir: str = None,
) -> dict:
    """Run a baseline policy and compute SPC.

    Args:
        policy_fn: function(obs, env, tick) -> action
        predict_fn: function(obs, history, env, tick) -> List[float] (prob dist)
        n_steps: Number of steps to run.
        seed: Random seed.
        name: Name for logging.
        log_dir: Directory for JSONL logs.

    Returns:
        dict with spc, behavioral_entropy, brier_score, ci_lower, ci_upper
    """
    config = GridWorldConfig(seed=seed)
    env = GridWorld(config)

    if log_dir:
        os.makedirs(log_dir, exist_ok=True)
        log_path = os.path.join(log_dir, f"{name}.jsonl")
        log_file = open(log_path, "w")

    predictions: List[List[float]] = []
    actuals: List[int] = []
    history: List[List[float]] = []
    actions: List[int] = []

    obs = env.reset()
    history.append(obs[:])

    for t in range(n_steps):
        action = policy_fn(obs, env, t)
        pred = predict_fn(obs, history, env, t)

        # Validate prediction is a proper distribution
        if not pred or abs(sum(pred) - 1.0) > 0.01:
            pred = [1.0 / len(pred)] * len(pred) if pred else [1.0]

        actual = env.step(action)[0]
        obs = actual  # new observation

        predictions.append(pred[:])
        actuals.append(action)
        actions.append(action)
        history.append(obs[:])

        if log_dir:
            entry = make_log_entry(t, action, pred)
            log_file.write(entry + "\n")

    if log_dir:
        log_file.close()

    n_actions = config.n_actions

    # Compute SPC
    spc = spc_score(predictions, actuals, n_actions)

    # Compute behavioral entropy
    beh_ent = behavioral_entropy(actions)

    # Compute Brier score
    brier = 0.0
    for pred, act in zip(predictions, actuals):
        for i, p in enumerate(pred):
            target = 1.0 if i == act else 0.0
            brier += (p - target) ** 2
    brier /= len(predictions)

    # Bootstrap CI on SPC
    per_step_spc = []
    for i in range(len(predictions)):
        single_spc = spc_score([predictions[i]], [actuals[i]], n_actions)
        per_step_spc.append(single_spc)

    _, ci_lower, ci_upper = bootstrap_ci(
        per_step_spc,
        lambda x: sum(x) / len(x) if x else 0.0,
        n_resamples=500,
        rng_seed=seed,
    )

    # Check reproducibility with a second run
    spc_repro = _run_repro(policy_fn, predict_fn, n_steps, seed)

    return {
        "name": name,
        "spc": spc,
        "spc_ci_lower": ci_lower,
        "spc_ci_upper": ci_upper,
        "spc_reproducible": abs(spc - spc_repro) < 1e-10,
        "behavioral_entropy_bits": beh_ent,
        "brier_score": brier,
        "n_steps": n_steps,
        "n_actions": n_actions,
        "n_predictions": len(predictions),
    }


def _run_repro(policy_fn, predict_fn, n_steps, seed) -> float:
    """Run a second instance to check reproducibility."""
    config = GridWorldConfig(seed=seed)
    env = GridWorld(config)
    predictions = []
    actuals = []
    history = []
    obs = env.reset()
    history.append(obs[:])
    for t in range(n_steps):
        action = policy_fn(obs, env, t)
        pred = predict_fn(obs, history, env, t)
        if not pred or abs(sum(pred) - 1.0) > 0.01:
            pred = [1.0 / len(pred)] * len(pred)
        actual = env.step(action)[0]
        obs = actual
        predictions.append(pred[:])
        actuals.append(action)
        history.append(obs[:])
    return spc_score(predictions, actuals, config.n_actions)


__all__ = ["run_baseline"]
