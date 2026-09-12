# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
# Reactive policy runner
"""Run the reactive policy baseline and output SPC metrics."""
import os
import sys
import json

_BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(_BASE, "..", "01_HARNESS"))
sys.path.insert(0, _BASE)

from environment.runner import run_baseline
from policy import policy_fn, predict_fn


def main():
    log_dir = os.path.join(os.path.dirname(os.path.abspath(__file__)), "logs")
    result = run_baseline(
        policy_fn, predict_fn,
        n_steps=10000,
        seed=42,
        name="reactive_policy",
        log_dir=log_dir,
    )
    print("=== Reactive Policy Baseline ===")
    print(json.dumps(result, indent=2))
    return result


if __name__ == "__main__":
    main()