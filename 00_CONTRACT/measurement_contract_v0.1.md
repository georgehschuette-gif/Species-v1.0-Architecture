# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

# Measurement Contract v0.1
# Immutable constitution for the Ω Research Program

## 1. Primary Research Question

Does a developmental artificial system with a developing self-model achieve
higher Self-Predictive Coherence (SPC) — the ability to predict its own actions
— compared to an identical system without a self-model, when both operate in
the same environment?

## 2. Primary Metric

**SPC (Self-Predictive Coherence)**

Definition: SPC = 1.0 − NormalizedLogLoss

Where:
- NormalizedLogLoss = −(1/N) * Σ ln(P(a_t | h_{<t})) / ln(|A|)
- P(a_t | h_{<t}) is the predicted probability of the action taken, given history
- |A| is the action space size
- N is the number of predictions

SPC is bounded in [0, 1]:
- 0.0 = predicts no better than uniform random (no self-predictive coherence)
- 1.0 = perfectly predicts every action (maximum self-predictive coherence)

## 3. Primary Endpoint (SPP)

**SPP (Self-Predictive Power)**

Definition: SPP(t) = SPC_self(t) − SPC_control(t)

Where:
- SPC_self(t) is the running SPC of the system with a self-model
- SPC_control(t) is the running SPC of the identical system without a self-model

SPP is computed in sliding windows (default: 1000 actions).
CI: 95% bootstrap confidence interval over 1000 resamples.

## 4. Hypothesis (Pre-registered)

**H₀**: SPP = 0 (the self-model provides no predictive advantage)

**H₁**: SPP > 0 (the self-model provides a measurable predictive advantage)

Direction: SPP is expected to be positive and CI should exclude 0.
If SPP ≤ 0 with p > 0.01, we accept the null result and publish.

## 5. Secondary Metrics

| Metric | Definition | Threshold |
|--------|-----------|-----------|
| Calibration | Reliability diagram slope vs. identity line | Slope ≥ 0.9, intercept ≤ 0.1 |
| Brier Score | Mean squared error of probabilistic predictions | ≤ 0.15 for well-calibrated |
| Behavioral Entropy | Shannon entropy of action distribution | Must be ≥ 1.0 bit (non-degenerate) |
| Environmental Entropy | Shannon entropy of observation distribution | Measured, reported (not gated) |
| Recovery Time | Steps to re-attain baseline SPC after ablation | ΔSPC_recovery < 500 steps (expected) |
| Detection Rate | True positive rate for contradictory false memories | ≥ 80% |

## 6. Controls

| Control | Description |
|---------|-------------|
| Random Policy | Uniform action selection (SPP lower bound) |
| Reactive Policy | Predict = last action (baseline predictability) |
| Environment-Only | Predicts observations, not self-actions |
| Ω-Control | Full system minus self-model |
| Ω-Self | Full system plus developing self-model |

## 7. Data Policy

1. Every log file is append-only and hash-chained.
2. Every prediction is logged with timestamp, predicted distribution, and actual outcome.
3. Every metric computation is versioned and hashable.
4. Deterministic replay must reproduce all metrics exactly.
5. Data is released publicly with the preprint.

## 8. Deviation Policy

1. All deviations from this contract are recorded in `deviations_log.md`.
2. Each deviation is timestamped and given a severity rating (minor/major/critical).
3. Major or critical deviations trigger a protocol review.
4. No deviation may change the primary metric definition.
5. No deviation may alter the hypothesis test (H₀ vs H₁).

## 9. Analysis Protocol

1. The analysis pipeline is fully specified in `analysis_pipeline.md`.
2. The pipeline runs on raw data only — no manual intervention.
3. Exploratory analyses are labeled as such and cannot support confirmatory claims.
4. The primary endpoint (SPP with 95% CI) is computed by the locked pipeline.
5. If the CI excludes 0 in the positive direction, H₁ is supported.
6. If the CI includes 0, the null result is accepted and published.

## 10. Standing Rules (Immutable)

1. Interpretation may never modify mechanism.
2. Every metric has a version hash.
3. Every log is append-only and hash-chained.
4. Every analysis is preregistered before data collection.
5. Every deviation is timestamped in `deviations_log.md`.
6. Null results are published.
7. The manifesto may say "it becomes." The notebook must be allowed to say "it didn't."
8. Phase N+1 does not begin until Phase N gate is green.
9. No phase is skipped, no gate is waived.
10. When in doubt, measure more, claim less.
