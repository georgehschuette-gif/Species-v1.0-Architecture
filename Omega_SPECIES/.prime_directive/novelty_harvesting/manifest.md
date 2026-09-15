# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

# Novelty Harvesting — Physiological Function

**Purpose:** Explores and integrates novel patterns without losing coherence

## Test Manifest

### Phase 7: Challenge
- **File:** `tests/phase7_challenge_test.cpp`
- **Tests:** 28
- **Duration:** ~0.3ms
- **Coverage:** Toy physics, self-play, Kolmogorov challenge
- **Status:** PASS

### Phase 12: Dream
- **File:** `tests/phase12_dream_test.cpp`
- **Tests:** 16
- **Duration:** ~0.07ms
- **Coverage:** Temporal folding recombination, memory replay
- **Status:** PASS

## Constitutional Invariants

1. **Kolmogorov Complexity:** The system must recognize patterns that compress information (Kolmogorov challenge)
2. **Temporal Folding:** The system must recombine memories across time scales (dreaming)
3. **Self-Play Exploration:** The system must explore its own capabilities through adversarial self-play
4. **Novelty Without Corruption:** The system must harvest novelty without breaking core invariants

## Failure Mode

If novelty harvesting tests fail:
- The system is either **over-conservative** (no exploration) or **over-exploratory** (losing coherence)
- The Governor should adjust divergence_entropy parameter
- The Pacemaker should modulate mutation_rate based on novelty success rate
- The system may need to balance exploration vs exploitation

## Recovery Protocol

1. Measure novelty success rate (new patterns integrated / total attempts)
2. If success rate < 20%, increase divergence_entropy to allow more drift
3. If success rate > 80% but coherence fails, decrease divergence_entropy
4. Monitor Kolmogorov complexity of generated patterns
5. Use dreaming to consolidate successful novel patterns
