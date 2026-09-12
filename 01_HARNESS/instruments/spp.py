# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
"""SPP (Self-Predictive Power) — primary endpoint.

SPP(t) = SPC_self(t) - SPC_control(t)

Measures the predictive advantage of a system with a self-model
over an identical system without one.
"""
from typing import Sequence, Tuple, List, Optional
from .spc import SPCResult, compute_spc, compute_spc_sliding, LogEntry
from .bootstrap_ci import bootstrap_ci


class SPPResult:
    """Result of SPP computation."""

    def __init__(self, spp_mean: float, ci_lower: float, ci_upper: float,
                 n_windows: int, p_value: float, self_spc: float,
                 control_spc: float):
        self.spp_mean = spp_mean
        self.ci_lower = ci_lower
        self.ci_upper = ci_upper
        self.n_windows = n_windows
        self.p_value = p_value
        self.self_spc = self_spc
        self.control_spc = control_spc
        self.hypothesis_supported = (
            spp_mean > 0.0 and ci_lower > 0.0
        )

    def __repr__(self) -> str:
        return (
            f"SPPResult(spp={self.spp_mean:.4f}, ci=[{self.ci_lower:.4f}, "
            f"{self.ci_upper:.4f}], n={self.n_windows}, p={self.p_value:.4f}, "
            f"supported={self.hypothesis_supported})"
        )


def _spc_from_window_result(result: SPCResult) -> float:
    """Extract the SPC value from an SPCResult."""
    return result.spc


def compute_spp(
    self_entries: Sequence[LogEntry],
    control_entries: Sequence[LogEntry],
    window_size: int = 1000,
    stride: int = 100,
    n_bootstrap: int = 1000,
    confidence: float = 0.95,
    rng_seed: Optional[int] = None,
) -> SPPResult:
    """Compute SPP between a self-model system and a control system.

    Both entry sequences must be aligned by tick for meaningful comparison.
    SPP is computed over matched sliding windows.

    Args:
        self_entries: Log entries from the system with self-model.
        control_entries: Log entries from the control system (no self-model).
        window_size: Predictions per window.
        stride: Step between windows.
        n_bootstrap: Bootstrap resamples for CI.
        confidence: CI confidence level.
        rng_seed: Optional seed for reproducibility.

    Returns:
        SPPResult with mean, CI, p-value, and hypothesis decision.
    """
    self_windows = compute_spc_sliding(self_entries, window_size, stride)
    control_windows = compute_spc_sliding(control_entries, window_size, stride)

    min_windows = min(len(self_windows), len(control_windows))
    if min_windows == 0:
        return SPPResult(
            spp_mean=0.0, ci_lower=0.0, ci_upper=0.0,
            n_windows=0, p_value=1.0,
            self_spc=0.0, control_spc=0.0,
        )

    spp_values = [
        s.spc - c.spc
        for s, c in zip(self_windows[:min_windows], control_windows[:min_windows])
    ]

    mean_spp = sum(spp_values) / len(spp_values)
    mean_self = sum(s.spc for s in self_windows[:min_windows]) / min_windows
    mean_control = sum(c.spc for c in control_windows[:min_windows]) / min_windows

    if len(spp_values) >= 2:
        stat_val, ci_lower, ci_upper = bootstrap_ci(
            spp_values,
            lambda x: sum(x) / len(x),
            n_resamples=n_bootstrap,
            confidence=confidence,
            rng_seed=rng_seed,
        )
    else:
        ci_lower = mean_spp
        ci_upper = mean_spp

    def _pvalue(boot_vals: List[float]) -> float:
        observed = mean_spp
        count = sum(1 for v in boot_vals if v >= observed)
        return count / len(boot_vals) if boot_vals else 1.0

    if len(spp_values) >= 2:
        _, boot_samples, _ = _bootstrap_samples(spp_values, n_bootstrap, rng_seed)
        p_val = _pvalue(boot_samples)
    else:
        p_val = 1.0 if mean_spp <= 0 else 0.5

    return SPPResult(
        spp_mean=mean_spp,
        ci_lower=ci_lower,
        ci_upper=ci_upper,
        n_windows=min_windows,
        p_value=p_val,
        self_spc=mean_self,
        control_spc=mean_control,
    )


def _bootstrap_samples(
    data: List[float],
    n_resamples: int,
    rng_seed: Optional[int] = None,
) -> Tuple[float, List[float], float]:
    """Generate bootstrap samples and return (original_stat, samples, mean)."""
    import random
    if rng_seed is not None:
        rng = random.Random(rng_seed)
    else:
        rng = random.Random()

    n = len(data)
    observed = sum(data) / n if n > 0 else 0.0
    samples = []
    for _ in range(n_resamples):
        sample = [rng.choice(data) for _ in range(n)]
        samples.append(sum(sample) / len(sample) if sample else 0.0)

    return (observed, samples, sum(samples) / len(samples))


__all__ = ["SPPResult", "compute_spp"]
