# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

# Pre-registered Predictions

## Primary Prediction

| Metric | Expected Value | Rationale |
|--------|---------------|-----------|
| SPP (final 7-day mean) | > 0.05 | Self-model provides incremental predictive advantage |
| SPP 95% CI lower bound | > 0.0 | CI excludes zero |
| SPC_self | > 0.6 | Self-model drives above random |
| SPC_control | ~0.3-0.5 | No self-model, limited predictability |
| SPC_random | ~0.0 | No predictability |

## Secondary Predictions

| Metric | Expected | Rationale |
|--------|----------|-----------|
| Calibration slope | 0.9-1.0 | Self-model produces calibrated confidence |
| Brier score | < 0.15 | Well-calibrated probabilistic predictions |
| Behavioral entropy | ≥ 1.0 bit | Action space not collapsed |
| Recovery time (ablation) | 200-800 steps | Self-model accelerates recovery |
| Contradictory memory detection | ≥ 80% | Inconsistency detector works |
| Consistent-false memory acceptance | > 90% | Not a truth detector |

## Worst Case (Null Result)

If SPP ≤ 0 with 95% CI including 0:
- The self-model provides no measurable predictive advantage.
- This is a valid scientific outcome.
- Publication will state: "Null result in self-predictive coherence."
- All data, code, and logs will be released regardless.

## Boundary Conditions

| Condition | Predicted Outcome |
|-----------|------------------|
| NaN state input | SPC instrument returns 0.0 (rejects invalid) |
| Empty action history | SPC instrument returns 0.0 (no predictions) |
| Deterministic policy | SPC ≈ 1.0 (perfect self-prediction) |
| Uniform random policy | SPC = 0.0 (no predictability) |
| Single action in space | SPC = 1.0 trivially (degenerate, entropy-filtered) |
