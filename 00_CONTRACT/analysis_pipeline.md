# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

# Analysis Pipeline Specification

## Input Format

All instruments accept log files in JSONL format:

```jsonl
{"timestamp": int, "tick": int, "action": int, "prediction": [float, ...], "actual": int, "context": {...}}
```

Where:
- `prediction` is a probability distribution over the action space (sums to 1.0)
- `actual` is the index of the action taken
- `context` is optional metadata

## Primary Pipeline (Locked)

```
1. Load raw logs → list[LogEntry]
2. Compute SPC over sliding window (W=1000, stride=100)
3. For each window:
   a. Compute NormalizedLogLoss = -ln(P_action) / ln(|A|)
   b. SPC = 1.0 - mean(NormalizedLogLoss)
4. Compute SPP = SPC_self - SPC_control (matched windows)
5. Bootstrap 95% CI over 1000 resamples
6. Report: SPP_mean, SPP_ci_lower, SPP_ci_upper, n_windows
```

## Secondary Pipeline

```
1. Calibration:
   a. Bin predictions into 10 equal-frequency bins
   b. Compute observed frequency vs. predicted probability
   c. Report slope, intercept, ECE (Expected Calibration Error)

2. Brier Score:
   a. Brier = (1/N) * Σ |P(a) - 1{a=actual}|^2
   b. Report mean and per-action breakdown

3. Entropy:
   a. Behavioral: Shannon entropy of action frequency distribution
   b. Environmental: Shannon entropy of observation frequency distribution
   c. Both computed over the same time window

4. Recovery Analysis:
   a. Identify ablation event (timestamp in log)
   b. Compute SPC before, during, and after ablation
   c. Measure time_to_baseline (SPC_recovery / SPC_pre)

5. False Memory Detection:
   a. Flag all injected memories with metadata
   b. Compute recall and precision for each memory type
   c. False-positive rate from true memories misclassified as false
```

## Exploratory Pipeline (Clearly Labeled)

All analyses not in the locked or secondary pipeline are exploratory.
Exploratory results cannot support confirmatory claims.

## Output Format

```json
{
  "primary_endpoint": {
    "spp_mean": float,
    "spp_ci_lower": float,
    "spp_ci_upper": float,
    "n_windows": int,
    "p_value": float,
    "hypothesis_supported": bool
  },
  "secondary_metrics": { ... },
  "exploratory": { ... }
}
```

## Version Hash

The pipeline is versioned. The version hash is computed as:
SHA256 of the concatenation of all pipeline Python source files.
Any change to the pipeline requires a new version hash and is logged
in `deviations_log.md`.
