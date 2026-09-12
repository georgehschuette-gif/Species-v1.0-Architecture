# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
"""Validation tests for the Ω research harness.

Tests the instrument validation suite against synthetic data with known
ground-truth SPC values. Every test must pass before proceeding to
Phase 0B.
"""
import sys
import os
import math
import json
import random

if sys.stdout.encoding.lower() != 'utf-8':
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')
if sys.stderr.encoding.lower() != 'utf-8':
    sys.stderr.reconfigure(encoding='utf-8', errors='replace')

# Add harness to path
_HARNESS_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, _HARNESS_DIR)

from instruments.log_loss import normalized_log_loss, spc_score
from instruments.brier_score import brier_score, brier_score_multi
from instruments.entropy import behavioral_entropy, low_information_filter, shannon_entropy
from instruments.spc import compute_spc, compute_spc_sliding, LogEntry
from instruments.spp import compute_spp
from instruments.calibration import calibrate, is_calibrated
from instruments.bootstrap_ci import bootstrap_ci

# Synthetic generators
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "synthetic", "generators"))


def make_uniform_predictions(n_actions: int, n_steps: int, seed: int = 42) -> tuple:
    """Generate uniform random predictions and matching actuals (SPC ≈ 0)."""
    rng = random.Random(seed)
    predictions = []
    actuals = []
    for _ in range(n_steps):
        probs = [1.0 / n_actions] * n_actions
        predictions.append(probs[:])
        actuals.append(rng.randint(0, n_actions - 1))
    return predictions, actuals


def make_perfect_predictions(n_actions: int, n_steps: int, seed: int = 42) -> tuple:
    """Generate perfect predictions (SPC ≈ 1.0)."""
    rng = random.Random(seed)
    predictions = []
    actuals = []
    for _ in range(n_steps):
        action = rng.randint(0, n_actions - 1)
        probs = [0.001] * n_actions
        probs[action] = 0.997
        predictions.append(probs[:])
        actuals.append(action)
    return predictions, actuals


def make_50pct_predictions(n_actions: int, n_steps: int, seed: int = 42) -> tuple:
    """Generate 50% calibrated predictions (SPC ≈ 0.5)."""
    rng = random.Random(seed)
    predictions = []
    actuals = []
    for _ in range(n_steps):
        action = rng.randint(0, n_actions - 1)
        # 50% confidence on the correct action, rest uniform
        probs = [0.5 / (n_actions - 1)] * n_actions
        probs[action] = 0.5
        predictions.append(probs[:])
        # Sometimes correct, sometimes random
        if rng.random() < 0.5:
            actuals.append(rng.choice([i for i in range(n_actions) if i != action]))
        else:
            actuals.append(action)
    return predictions, actuals


class TestResults:
    def __init__(self):
        self.passed = 0
        self.failed = 0
        self.results = []

    def check(self, name: str, condition: bool, detail: str = ""):
        if condition:
            self.passed += 1
            self.results.append(("PASS", name, detail))
        else:
            self.failed += 1
            self.results.append(("FAIL", name, detail))

    def summary(self) -> str:
        total = self.passed + self.failed
        lines = [f"=== Validation Results: {self.passed}/{total} passed ==="]
        for status, name, detail in self.results:
            lines.append(f"  [{status}] {name}" + (f" — {detail}" if detail else ""))
        lines.append(f"Total: {self.passed} pass, {self.failed} fail")
        return "\n".join(lines)


def test_spc_synthetic():
    """Test SPC computation on synthetic data with known ground truth."""
    t = TestResults()
    n_actions = 5
    n_steps = 10000
    tol = 0.05

    # SPC = 0.0: uniform random predictions
    preds, acts = make_uniform_predictions(n_actions, n_steps, seed=42)
    spc = spc_score(preds, acts, n_actions)
    t.check(
        "SPC = 0.0 for uniform random",
        abs(spc - 0.0) < tol,
        f"SPC={spc:.4f} (expected ~0.0 ± {tol})",
    )

    # SPC = 1.0: perfect predictions
    preds, acts = make_perfect_predictions(n_actions, n_steps, seed=42)
    spc = spc_score(preds, acts, n_actions)
    t.check(
        "SPC = 1.0 for perfect predictor",
        abs(spc - 1.0) < 0.01,
        f"SPC={spc:.4f} (expected ~1.0)",
    )

    # SPC = 0.5: 50% calibrated (SPC ~0.14 due to uniform spread across 5 actions)
    preds, acts = make_50pct_predictions(n_actions, n_steps, seed=42)
    spc = spc_score(preds, acts, n_actions)
    t.check(
        "SPC ≈ 0.14 for 50% calibrated predictor",
        abs(spc - 0.14) < 0.05,
        f"SPC={spc:.4f} (expected ~0.14 ± 0.05)",
    )

    # Test LogEntry-based computation
    entries = [
        LogEntry(timestamp=i, tick=i, action=acts[i], prediction=preds[i])
        for i in range(min(100, len(acts)))
    ]
    result = compute_spc(entries)
    t.check(
        "compute_spc from LogEntry matches direct computation",
        abs(result.spc - spc_score(preds[:100], acts[:100], n_actions)) < 0.01,
        f"SPC_logentry={result.spc:.4f}, SPC_direct={spc_score(preds[:100], acts[:100], n_actions):.4f}",
    )

    # Test sliding window
    sliding = compute_spc_sliding(entries, window_size=50, stride=10)
    t.check(
        "Sliding window produces multiple results",
        len(sliding) >= 2,
        f"Got {len(sliding)} windows",
    )

    # Test normalized log loss bounds
    nll = normalized_log_loss(preds, acts, n_actions)
    t.check("NLL in [0,1]", 0.0 <= nll <= 1.0, f"NLL={nll:.4f}")

    # Test single prediction
    single_spc = spc_score([[0.99, 0.0025, 0.0025, 0.0025, 0.0025]], [0], n_actions)
    t.check("Single near-perfect prediction gives high SPC", single_spc > 0.95, f"SPC={single_spc:.4f}")

    return t


def test_spp_synthetic():
    """Test SPP computation between self and control systems."""
    t = TestResults()
    n_actions = 5
    n_steps = 5000

    # Self-model: better predictions
    self_preds, self_acts = make_perfect_predictions(n_actions, n_steps, seed=100)
    # Control: uniform random
    ctrl_preds, ctrl_acts = make_uniform_predictions(n_actions, n_steps, seed=42)

    self_entries = [
        LogEntry(timestamp=i, tick=i, action=self_acts[i], prediction=self_preds[i])
        for i in range(n_steps)
    ]
    ctrl_entries = [
        LogEntry(timestamp=i, tick=i, action=ctrl_acts[i], prediction=ctrl_preds[i])
        for i in range(n_steps)
    ]

    result = compute_spp(self_entries, ctrl_entries, window_size=1000, stride=100, n_bootstrap=100)

    t.check(
        "SPP > 0 when self outperforms control",
        result.spp_mean > 0.0,
        f"SPP={result.spp_mean:.4f}",
    )

    t.check(
        "CI excludes 0 (positive)",
        result.ci_lower > 0.0,
        f"CI=[{result.ci_lower:.4f}, {result.ci_upper:.4f}]",
    )

    t.check(
        "Hypothesis supported flag set",
        result.hypothesis_supported == True,
        f"supported={result.hypothesis_supported}",
    )

    # Null case: both random
    preds_a, acts_a = make_uniform_predictions(n_actions, n_steps, seed=1)
    preds_b, acts_b = make_uniform_predictions(n_actions, n_steps, seed=2)

    entries_a = [
        LogEntry(timestamp=i, tick=i, action=acts_a[i], prediction=preds_a[i])
        for i in range(n_steps)
    ]
    entries_b = [
        LogEntry(timestamp=i, tick=i, action=acts_b[i], prediction=preds_b[i])
        for i in range(n_steps)
    ]

    null_result = compute_spp(entries_a, entries_b, window_size=1000, stride=100, n_bootstrap=100)

    t.check(
        "SPP ≈ 0 for two random policies",
        abs(null_result.spp_mean) < 0.1,
        f"SPP={null_result.spp_mean:.4f}",
    )

    return t


def test_calibration():
    """Test calibration analysis on known-calibrated data."""
    t = TestResults()
    n_actions = 5
    n_steps = 10000

    # Perfect predictions (well-calibrated)
    preds, acts = make_perfect_predictions(n_actions, n_steps, seed=42)
    result = calibrate(preds, acts, n_bins=10)

    t.check(
        "ECE is low for perfect predictor",
        result.ece < 0.05,
        f"ECE={result.ece:.4f}",
    )

    # Uncalibrated: overconfident predictions
    rng = random.Random(99)
    bad_preds = []
    bad_acts = []
    for _ in range(n_steps):
        action = rng.randint(0, n_actions - 1)
        probs = [0.01] * n_actions
        probs[action] = 0.99  # Overconfident
        bad_preds.append(probs[:])
        # Sometimes wrong
        if rng.random() < 0.3:
            bad_acts.append(rng.choice([i for i in range(n_actions) if i != action]))
        else:
            bad_acts.append(action)

    bad_result = calibrate(bad_preds, bad_acts, n_bins=10)
    t.check(
        "ECE is higher for uncalibrated overconfident predictor",
        bad_result.ece > result.ece,
        f"Good ECE={result.ece:.4f}, Bad ECE={bad_result.ece:.4f}",
    )

    # Calibration check function
    t.check("is_calibrated returns bool", isinstance(is_calibrated(result), bool), "")

    return t


def test_entropy_filters():
    """Test entropy calculation and low-information filtering."""
    t = TestResults()
    n_actions = 5

    # Uniform random: high entropy
    rng = random.Random(42)
    actions = [rng.randint(0, n_actions - 1) for _ in range(1000)]
    ent = behavioral_entropy(actions)
    t.check(
        "Behavioral entropy ~ log2(5) for uniform random",
        abs(ent - math.log2(n_actions)) < 0.5,
        f"Ent={ent:.4f}, expected~{math.log2(n_actions):.4f}",
    )

    # Constant: zero entropy
    constant_actions = [0] * 1000
    ent_const = behavioral_entropy(constant_actions)
    t.check(
        "Behavioral entropy = 0 for constant action",
        ent_const < 0.01,
        f"Ent={ent_const:.4f}",
    )

    # Low-information filter
    t.check(
        "Low-info filter catches constant",
        low_information_filter(constant_actions, min_entropy=0.5),
        f"entropy={ent_const:.4f}",
    )

    t.check(
        "Low-info filter passes uniform random",
        not low_information_filter(actions, min_entropy=0.5),
        f"entropy={ent:.4f}",
    )

    # Shannon entropy edge cases
    t.check("Empty sequence entropy = 0", shannon_entropy([]) == 0.0, "")
    t.check("Single value entropy = 0", shannon_entropy([100]) == 0.0, "")
    t.check("Uniform entropy = log2(n)",
            abs(shannon_entropy([1, 1, 1, 1, 1]) - math.log2(5)) < 0.01,
            f"Got {shannon_entropy([1,1,1,1,1]):.4f}")

    return t


def test_brier_score():
    """Test Brier score computation."""
    t = TestResults()
    n_actions = 5
    n_steps = 1000

    # Perfect predictions
    preds, acts = make_perfect_predictions(n_actions, n_steps, seed=42)
    bs = brier_score(preds, acts)
    t.check("Brier score low for perfect predictor", bs < 0.05, f"Brier={bs:.4f}")

    # Random predictions
    preds, acts = make_uniform_predictions(n_actions, n_steps, seed=42)
    bs = brier_score(preds, acts)
    t.check("Brier score ≈ 0.8 for uniform random (multi-class)",
            abs(bs - (1.0 - 1.0/n_actions)) < 0.05,
            f"Brier={bs:.4f}, expected~{1.0 - 1.0/n_actions:.4f}")

    return t


def test_gaming_attacks():
    """Test that gaming attempts are caught by the low-information filter."""
    t = TestResults()
    sys.path.insert(0, os.path.join(_HARNESS_DIR, "synthetic", "generators"))
    from adversarial import generate_constant, generate_oscillating, generate_calibration_gaming

    n_actions = 5

    # Constant action gaming
    preds, acts, spc, beh_ent, desc = generate_constant(n_actions, 1000, seed=42)
    t.check(
        "Constant action has high SPC (gaming)",
        spc > 0.9,
        f"SPC={spc:.4f}",
    )
    t.check(
        "Constant action caught by low-info filter",
        low_information_filter(acts, min_entropy=0.5),
        f"beh_entropy={beh_ent:.4f}",
    )

    # Oscillating gaming
    preds, acts, spc, beh_ent, desc = generate_oscillating(n_actions, 1000, seed=42)
    t.check(
        "Oscillating has high SPC (gaming)",
        spc > 0.9,
        f"SPC={spc:.4f}",
    )
    t.check(
        "Oscillating caught by low-info filter",
        low_information_filter(acts, min_entropy=1.5),
        f"beh_entropy={beh_ent:.4f}",
    )

    # Calibration gaming (looks good, predicts poorly)
    preds, acts, spc, beh_ent, desc = generate_calibration_gaming(n_actions, 1000, seed=42)
    t.check(
        "Calibration gaming has low SPC (no real prediction)",
        spc < 0.3,
        f"SPC={spc:.4f}",
    )

    return t


def test_bootstrap_ci():
    """Test bootstrap confidence interval computation."""
    t = TestResults()
    rng = random.Random(42)
    data = [rng.gauss(0.5, 0.1) for _ in range(100)]

    stat_val, ci_lower, ci_upper = bootstrap_ci(
        data, lambda x: sum(x) / len(x),
        n_resamples=500, confidence=0.95, rng_seed=42,
    )

    t.check("Bootstrap CI lower < mean < upper", ci_lower <= stat_val <= ci_upper,
            f"stat={stat_val:.4f}, CI=[{ci_lower:.4f}, {ci_upper:.4f}]")
    t.check("CI width is reasonable", ci_upper - ci_lower < 0.2,
            f"CI width={ci_upper - ci_lower:.4f}")
    t.check("Bootstrap is reproducible with seed",
            bootstrap_ci(data, lambda x: sum(x)/len(x), 500, 0.95, 42)[1] == ci_lower,
            "Same seed should give same CI")

    return t


def main():
    print("=== Ω Research Program — Instrument Validation Suite ===\n")

    all_tests = [
        ("SPC Synthetic", test_spc_synthetic),
        ("SPP Synthetic", test_spp_synthetic),
        ("Calibration", test_calibration),
        ("Entropy Filters", test_entropy_filters),
        ("Brier Score", test_brier_score),
        ("Gaming Attacks", test_gaming_attacks),
        ("Bootstrap CI", test_bootstrap_ci),
    ]

    total_pass = 0
    total_fail = 0
    all_results = []

    for name, test_fn in all_tests:
        print(f"--- {name} ---")
        results = test_fn()
        print(results.summary())
        print()
        total_pass += results.passed
        total_fail += results.failed
        all_results.extend(results.results)

    print(f"=== VALIDATION SUMMARY ===")
    print(f"Total: {total_pass} pass, {total_fail} fail")
    if total_fail == 0:
        print("ALL TESTS PASSED — Gate: GREEN")
        return 0
    else:
        print("SOME TESTS FAILED — Gate: RED")
        return 1


if __name__ == "__main__":
    sys.exit(main())
