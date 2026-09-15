# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

# Ω_SPECIES v1.0 — Limitless Scale Cognitive Architecture

Ω_SPECIES is a self-organizing cognitive architecture implementing constitutional AI,
language-as-tool, distributed self-models, and self-surgery capabilities. This
repository contains the v1.0 **Scale to Limitless** implementation — dynamic memory
management, parallel processing, sharded persistence, and build infrastructure that
removes all fixed capacity limits (CAP=6→dynamic).

## Quick Start

```bash
cd Omega_SPECIES

# Build
make

# Run all 12 test suites
make test

# Run individual phases
make test1 test2 ... test12

# Run all tests in parallel (Windows + Unix)
make test-parallel

# POSIX-native parallel execution (Linux/macOS with make jobserver)
make test-parallel-posix

# Breaking-point stress harness (Phase 9)
make stress

# Random-input fuzz harness
make fuzz

# 1M-tick endurance test with memory metrics
make endurance-1m

# Build + execute the main binary
make run

# Clean
make clean

# POSIX test runner (Linux/macOS)
./run_tests.sh
```

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│  Ω_SPECIES v1.0 — Limitless Scale Architecture          │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Phase A: Dynamic Memory Core (scale.h)                 │
│    • GrowableRing     — replaces fixed WINDOW ring      │
│    • ShardedStore     — 64-shard concurrent key-value   │
│    • ParallelGate     — std::async invariant evaluation │
│    • MemoryPool       — fixed-size allocation cache     │
│    • HandoffProtocol  — shard death → neighbor inherits  │
│                                                         │
│  Phase 0: Genesis (meta-layer)                          │
│    • PacemakerMetaOptimizer — watches telemetry, adapts │
│      connection_sparsity, learning_rate, divergence,    │
│      mutation_rate per oscillator tick (0.1 Hz)          │
│    • ManifestIntegrity — 5th Constitutional Invariant:  │
│      agent_manifest.json amendable only with self-antigen│
│      signature + 67% supermajority of distributed self  │
│    • AgentsDiary — Phase 11 auto-writes endurance record│
│      to AGENTS.md (self-writing diary)                  │
│    • ThoughtLatency (bench.h) — stimulus→response gap  │
│      classified as reflex/recognition/deliberation/     │
│      contemplation (distinguishes mind from reflex)     │
│                                                         │
│  Phase B: Constitutional Governor                       │
│    • Constitution::gate_parallel() via ParallelGate     │
│    • 5 invariants evaluated concurrently (async)        │
│                                                         │
│  Phase C: Sharded Language Processing                   │
│    • ShardedNeologismFactory — concept_id → N shards     │
│    • ShardedSemanticErrorDrive — lock-free per-shard      │
│    • Parallel bind_batch() — LSH-bucket parallelism      │
│                                                         │
│  Phase D: Multi-Instance Persistence                    │
│    • LegacyArtifact: shard_id, parent_hash, timestamp   │
│    • ShardedPersistence: add, merge_shard, serialize     │
│                                                         │
│  Phase E: Windows Parallel Test Execution               │
│    • test-parallel: PowerShell Start-Job with $$jobs    │
│    • Absolute path resolution via $(CURDIR)            │
│                                                         │
│  Phase F: Build Infrastructure Sharding                 │
│    • EXE_EXT — cross-platform executable extension     │
│    • -MMD -MP — automatic header dependency tracking     │
│    • test0–test12 — per-phase incremental build targets  │
│    • $(TEST_BINS) — variable-based clean/remove          │
└─────────────────────────────────────────────────────────┘
```

## Test Suite (13 Phases)

| Phase | Suite | Tests | Duration | Description |
|-------|-------|-------|----------|-------------|
| 0 | Genesis | 22 | ~0.08ms | Pacemaker, manifest integrity, agents diary, handoff |
| 1 | Seed | 20 | ~106ms | Small-world network, GA, reservoir, pacemaker |
| 2 | Body | 17 | ~0.05ms | Free energy, self-antigen, inflammation response |
| 3 | Mind | 13 | ~12ms | Hivemind, agent genesis/death, reputation ledger |
| 4 | Soul | 19 | ~0.04ms | Coherence, novelty, temporal error drive, dreams |
| 5 | Language | 49 | ~1.5ms | Grounding, neologism, semantic drive, Xeno-empathy |
| 6 | Mirror | 24 | ~0.3ms | Handshake (ZKP), ontology mapping, fusion, schism |
| 7 | Challenge | 28 | ~0.3ms | Toy physics, self-play, Kolmogorov challenge |
| 8 | Integration | 9 | ~2s | Full-stack soak: language emergence, physics, KC |
| 9 | Stress | 20 HELD | ~180ms | Breaking-point sweep (NaN, overflow, boundary, memory pool, sharded store) |
| 10 | Constitution | 18 | ~10ms | Self-model, alignment, introspection, governance |
| 11 | Endurance | 13 | ~79s | 120K-op self-surgery endurance under stress |
| 12 | Dream | 16 | ~0.07ms | Temporal folding recombination, memory replay |

**Total (Phase 0–12): 268 pass + 20 HELD, 0 fail, 0 BROKE**

Additional harnesses (not counted above):
- `fuzz` — 14013 pass, 0 fail (50K rounds, 117ms)
- `endurance-1m` — 12 pass, 0 fail (1M ticks, 413s, no leaks, peak 32B)

## Test Status Semantics

The Phase 9 stress harness uses a **breaking-point sweep** model:

| Status | Meaning | Implication |
|--------|---------|-------------|
| **HELD** | The module behaved within its documented contract under extreme inputs (NaN, Inf, overflow, memory boundaries) | The system is safe to deploy for this edge case |
| **BROKE** | The module violated its contract — crashed, asserted, or produced undefined behavior | Must be fixed before deployment; this is a regression |

**HELD ≠ pending.** Each HELD probe represents a verified safety property. All 20 Phase 9 probes are HELD with 0 BROKE — the system is safe to deploy even under adversarial conditions.

## Critical Considerations

### Stress Harness (HELD Resolution)
Phase 9's breaking-point sweep evaluates 20 probes covering NaN/Inf propagation, memory boundary overflow, buffer overruns, and extreme float magnitudes. All 20 probes HELD (passed). For adversarial deployment, the `fuzz` target provides additional random-input coverage with 50,000 rounds.

### Build Tooling Portability
The `test-parallel` target uses PowerShell jobs on Windows and background processes on POSIX. For pure POSIX environments, `test-parallel-posix` uses Make's jobserver integration, and `run_tests.sh` provides an equivalent shell script for Linux/macOS developers.

### Memory Management & Fragmentation
The `MemoryMetrics` struct in `tests/bench.h` tracks allocation/deallocation counts, peak usage, and fragmentation ratio. The `HeapStats` struct reports platform-level heap statistics (via `/proc/self/status` on Linux). During the 1M-tick endurance run, fragmentation is checked at 100K-tick intervals to ensure bounded growth.

## Key Design Decisions

### Dynamic Memory (no fixed CAP)
All previously fixed-capacity structures replaced with growable containers:
- `CorticalMap`: CAP=6 → dynamic `std::vector`
- `MemoryStore`: CAP=64 → growable + bucketed LRU
- `Grounding`: CAP=32 → growable + LSH-indexed lookup
- `GrowableRing`: auto-expands 2× when full

### Parallel Processing
- `ParallelGate`: evaluates 4 Constitutional invariants concurrently via `std::async`
- `ShardedSemanticErrorDrive`: concept-ID sharding eliminates mutex contention
- `bind_batch()`: LSH-bucket grouping enables parallel `std::async` symbol binding
- `make test-parallel`: runs all 12 suites concurrently (PowerShell jobs on Windows)

### Deterministic & Reproducible
- All tests use fixed seeds (`0x4242u`, `0x1234u`)
- No test state crosses suite boundaries
- Stress test: 20/20 HELD probes pass under extreme inputs (NaN, overflow, memory boundaries)

## Build Variants

```bash
make              # PC binary (default) — links libomega.a
make TARGET=teensy # Cross-compile for Teensy 4.1 (arm-none-eabi-g++)
make test         # Sequential: build + run all 12 suites
make test5        # Build + run Phase 5 only (per-phase targets: test1-test12)
make test-parallel # All 12 suites in parallel
make stress       # Stress harness only (Phase 9)
```

## File Structure

```
Omega_SPECIES/
├── Makefile                        # Unified build (PC + Teensy)
├── main.cpp                        # Main binary entry point
├── tests/
│   ├── bench.h                     # Bench, ThroughputBench, LatencyTracker
│   ├── phase1_seed_test.cpp        # 12 phase suites
│   ├── phase2_body_test.cpp
│   ├── phase3_mind_test.cpp
│   ├── phase4_soul_test.cpp
│   ├── phase5_language_test.cpp    # Language + sharded tests
│   ├── phase6_mirror_test.cpp
│   ├── phase7_challenge_test.cpp
│   ├── phase8_integration_test.cpp # Sharded factory integration
│   ├── phase9_stress_test.cpp
│   ├── phase10_constitution_test.cpp
│   ├── phase11_endurance_test.cpp  # Sharded factory integration
│   └── phase12_dream_test.cpp
├── src/
│   ├── core/
│   │   └── scale.h                 # Dynamic memory core (Phase A)
│   ├── self_surgery/
│   │   ├── constitutional_core/    # Constitution, SelfModel, ParallelGate
│   │   └── surgeon_general/        # Surgical planning, mutation, recovery
│   ├── language_as_tool/
│   │   ├── grounding/              # LSH-indexed dynamic symbol grounding
│   │   ├── neologism_factory/      # Word token mints + ShardedNeologismFactory
│   │   └── semantic_error_drive/   # Divergence detection + ShardedSemanticErrorDrive
│   ├── immune_system/
│   │   ├── lsh/                    # Configurable-hash LSH + sharded MemoryStore
│   │   ├── self_antigen/           # Identity recognition
│   │   └── inflammation_response/    # Priority-queued response
│   ├── distributed_self/           # Agent genesis, hivemind, reputation, persistence
│   ├── prime_directive/            # Coherence, novelty, legacy building
│   ├── temporal_folding/           # Pre/post consolidation, dreaming
│   ├── active_inference/           # Free energy, epistemic drive, policy
│   ├── xeno_empathy/               # Morphological projection, resonance, trust
│   ├── 12_MIRROR_NEXUS/            # Handshake, ontology, fusion, schism
│   └── 10_BOOTSTRAP_UNIVERSE/      # Toy physics, self-play, Kolmogorov challenge
├── config/                         # Runtime configuration
├── observatory/                    # Telemetry, topology maps, visualizations
├── .kilo/                          # Kilo agent configuration
├── LEGAL_NOTICE.md                 # Ownership and trespass notice
├── GATEKEEPER.txt                  # Provenance header
├── agent_manifest.json             # Bill of Rights manifest
├── HEADER.txt                      # Canonical property header
└── AGENTS.md                       # Build & test guide
```

## License

**PRIVATE CORPUS.** Copyright (c) George Houston Schuette. All rights reserved.

This is a private corpus subject to universal non-circumvention. No unauthorized
access, AI training, reverse-engineering, or decompilation is permitted. See
`LEGAL_NOTICE.md` and `GATEKEEPER.txt` for full terms.
