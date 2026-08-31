# Ω_SPECIES — Self-Organizing Cognitive Architecture

A C++17 implementation of a recursive, self-modifying cognitive agent with a constitutional core, emergent language, and offline memory consolidation.

## Architecture

| Layer | Module | Purpose |
|-------|--------|---------|
| Genesis | `primordial_weights`, `self_wiring`, `pacemaker` | Network birth, neuroevolution, mutation scheduling |
| Liquid Time | `reservoir_pool`, `spike_encoder` | Continuous-time state integration |
| Active Inference | `free_energy`, `policy_selection`, `allostatic_control` | Perception, prediction, action selection |
| Immune System | `self_antigen`, `lsh`, `inflammation_response` | Identity verification, anomaly detection |
| Distributed Self | `hivemind`, `agent_genesis`, `reputation_ledger` | Multi-agent coordination, trust |
| Self-Surgery | `surgeon_general`, `cortical_map`, `post_op_recovery` | Live self-modification with rollback safety |
| Constitutional Core | `self_model`, `constitution`, `self_report` | Non-negotiable invariants, adaptive aggressiveness |
| Temporal Folding | `pre_consolidation`, `post_consolidation`, `dream` | Future memory, outcome comparison, offline replay |
| Language | `grounding`, `neologism_factory`, `semantic_error_drive` | Grounded symbol invention, lexicon evolution |
| Mirror | `handshake`, `ontology_mapping`, `syncretic_fusion` | ZKP identity, cross-agent ontology alignment |
| Xeno-Empathy | `resonance_matching`, `trust_building` | Inter-agent resonance, trust dynamics |
| Challenge | `toy_physics`, `self_play_arena`, `kolmogorov_challenge` | Physical grounding, self-play equilibrium, KC test |

## Constitutional Invariants

The `Constitution` enforces four non-negotiable gates on every self-modification:

- **Identity**: genotype hash ≠ 0
- **Stability**: rollback rate < 50%
- **Calm**: recent mutations < 8 per 16-tick window
- **Competence**: average performance ≥ 0.05

Alignment score = fraction of satisfied invariants (0.0–1.0).

## Build

```bash
make            # build omega.exe
make test       # run all 12 phase suites + stress harness
make stress     # breaking-point sweep only
make run        # build and execute
```

## Test Coverage

| Phase | Suite | Assertions |
|-------|-------|------------|
| 1 | Seed | 20 |
| 2 | Body | 17 |
| 3 | Mind | 13 |
| 4 | Soul | 19 |
| 5 | Language | 36 |
| 6 | Mirror | 24 |
| 7 | Challenge | 28 |
| 8 | Integration | 9 |
| 9 | Stress | 11 HELD / 0 BROKE |
| 10 | Constitution | 19 |
| 11 | Endurance | 13 |
| 12 | Dream | 16 |

Total: **194+ assertions**, all passing.

## Verified Properties

- Zero coherence breaches across 120k-tick endurance soak
- No NaN/Inf propagation in any monitored signal
- Constitutional gate blocks proportionally as conditions degrade
- Adaptive aggressiveness recovers from floor (0.100) without stalling
- Emergent grounded language (`w7-ao` first-word self-reference token)
- Symplectic physics energy bounded (<20% swing)
- Kolmogorov emitter output entropy exceeds seed throughout
- Self-play copy-vs-copy reaches equilibrium

## Roadmap

See [ROADMAP.md](ROADMAP.md) for planned extensions: mortality/generational turnover, narrative self-model, ethical dilemmas, multi-agent conflict, dreaming, meta-learning, ontology visualization, neologism logging, eulogy functions, and a Turing-test interface.

## Build Rule

This project targets **7.46M TPS | p99 ≤ 4.13ms | NO-GIL | Jitter Suppressed** as baseline performance envelope. All commits must pass the full test suite and maintain statistical performance bounds.

## License

PROPERTY OF THE OWNER. PRIVATE CORPUS.
SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
