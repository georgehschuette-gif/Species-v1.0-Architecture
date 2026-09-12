# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
"""SPC (Self-Predictive Coherence) — primary metric.

SPC = 1.0 - NormalizedLogLoss

Computes self-predictive coherence from logged prediction/actual pairs.
Supports sliding-window computation for time-series analysis.
"""
from typing import Sequence, List, Tuple, Dict, Any
from .log_loss import normalized_log_loss, spc_score


class LogEntry:
    """A single logged prediction-actual pair."""

    def __init__(self, timestamp: int, tick: int, action: int,
                 prediction: Sequence[float], context: Dict[str, Any] | None = None):
        self.timestamp = timestamp
        self.tick = tick
        self.action = action
        self.prediction = list(prediction)
        self.context = context or {}

    @property
    def action_space_size(self) -> int:
        return len(self.prediction)

    def __repr__(self) -> str:
        return (
            f"LogEntry(tick={self.tick}, action={self.action}, "
            f"pred_sum={sum(self.prediction):.4f})"
        )


class SPCResult:
    """Result of SPC computation over a window."""

    def __init__(self, spc: float, nll: float, n_predictions: int,
                 window_start: int, window_end: int):
        self.spc = spc
        self.normalized_log_loss = nll
        self.n_predictions = n_predictions
        self.window_start = window_start
        self.window_end = window_end

    def __repr__(self) -> str:
        return (
            f"SPCResult(spc={self.spc:.4f}, nll={self.normalized_log_loss:.4f}, "
            f"n={self.n_predictions}, ticks=[{self.window_start},{self.window_end}])"
        )


def compute_spc(entries: Sequence[LogEntry]) -> SPCResult:
    """Compute SPC over a sequence of log entries.

    Args:
        entries: Sequence of LogEntry objects.

    Returns:
        SPCResult with spc, normalized_log_loss, and window bounds.
    """
    if not entries:
        return SPCResult(spc=0.0, nll=0.0, n_predictions=0,
                        window_start=0, window_end=0)

    predictions = [e.prediction for e in entries]
    actuals = [e.action for e in entries]
    action_space_size = entries[0].action_space_size

    nll = normalized_log_loss(predictions, actuals, action_space_size)
    spc = 1.0 - nll

    return SPCResult(
        spc=spc,
        nll=nll,
        n_predictions=len(entries),
        window_start=min(e.tick for e in entries),
        window_end=max(e.tick for e in entries),
    )


def compute_spc_sliding(
    entries: Sequence[LogEntry],
    window_size: int = 1000,
    stride: int = 100,
) -> List[SPCResult]:
    """Compute SPC over sliding windows.

    Args:
        entries: Sequence of LogEntry objects (must be sorted by tick).
        window_size: Number of predictions per window.
        stride: Step between windows.

    Returns:
        List of SPCResult for each window.
    """
    if len(entries) < window_size:
        return [compute_spc(entries)] if entries else []

    results = []
    for start in range(0, len(entries) - window_size + 1, stride):
        window = entries[start:start + window_size]
        results.append(compute_spc(window))

    if len(entries) >= window_size:
        remainder_start = len(entries) - window_size
        if (len(entries) - remainder_start) < window_size:
            window = entries[remainder_start:]
            if len(window) >= 100:
                results.append(compute_spc(window))

    return results


def spc_from_lists(
    predictions: Sequence[Sequence[float]],
    actuals: Sequence[int],
    action_space_size: int,
) -> float:
    """Compute SPC directly from prediction/actual lists.

    Convenience wrapper for simple cases.
    """
    return spc_score(predictions, actuals, action_space_size)


__all__ = [
    "LogEntry",
    "SPCResult",
    "compute_spc",
    "compute_spc_sliding",
    "spc_from_lists",
]
