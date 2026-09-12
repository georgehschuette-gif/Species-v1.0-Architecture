# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
"""Reproducibility check: run each baseline 3 times with different seeds,
verify SPC is stable within tolerance.
"""
import os
import sys
import json

_BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(_BASE, "..", "01_HARNESS"))
sys.path.insert(0, _BASE)

from environment.runner import run_baseline


def check_reproducibility(policy_fn, predict_fn, name: str, seed: int,
                          n_steps: int = 10000, n_runs: int = 3,
                          spc_tolerance: float = 0.001) -> dict:
    """Run n_runs with different seeds, check SPC stability."""
    results = []
    for run_idx in range(n_runs):
        run_seed = seed + run_idx * 1000
        r = run_baseline(policy_fn, predict_fn, n_steps=n_steps, seed=run_seed,
                         name=f"{name}_run{run_idx}", log_dir=None)
        results.append(r)

    spcs = [r["spc"] for r in results]
    spc_mean = sum(spcs) / len(spcs)
    spc_min = min(spcs)
    spc_max = max(spcs)
    spc_std = (sum((s - spc_mean) ** 2 for s in spcs) / len(spcs)) ** 0.5

    reproducible = spc_std < spc_tolerance or (spc_max - spc_min) < spc_tolerance

    return {
        "name": name,
        "spcs": spcs,
        "spc_mean": spc_mean,
        "spc_min": spc_min,
        "spc_max": spc_max,
        "spc_std": spc_std,
        "reproducible": reproducible,
        "n_runs": n_runs,
    }


def main():
    from random_policy.policy import policy_fn as random_policy, predict_fn as random_predict
    from reactive_policy.policy import policy_fn as reactive_policy, predict_fn as reactive_predict
    from environment_only.predictor import policy_fn as env_policy, predict_fn as env_predict

    print("=== Baseline Reproducibility Check (3 runs each, 10K steps) ===\n")

    random_result = check_reproducibility(random_policy, random_predict, "random", seed=42)
    print(f"Random:     SPC values={[f'{v:.6f}' for v in random_result['spcs']]}")
    print(f"           reproducible={random_result['reproducible']}\n")

    reactive_result = check_reproducibility(reactive_policy, reactive_predict, "reactive", seed=42)
    print(f"Reactive:   SPC values={[f'{v:.6f}' for v in reactive_result['spcs']]}")
    print(f"           reproducible={reactive_result['reproducible']}\n")

    env_result = check_reproducibility(env_policy, env_predict, "environment_only", seed=42)
    print(f"Env-only:   SPC values={[f'{v:.6f}' for v in env_result['spcs']]}")
    print(f"           reproducible={env_result['reproducible']}\n")

    all_reproducible = (random_result['reproducible'] and
                        reactive_result['reproducible'] and
                        env_result['reproducible'])
    print(f"All baselines reproducible: {all_reproducible}")
    print(f"Gate: {'GREEN' if all_reproducible else 'RED'}")

    return {
        "random": random_result,
        "reactive": reactive_result,
        "environment_only": env_result,
        "all_reproducible": all_reproducible,
    }


if __name__ == "__main__":
    result = main()
    sys.path.insert(0, os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), ".."))
    with open(os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "baseline_report.md"), "w") as f:
        pass  # will write report separately
