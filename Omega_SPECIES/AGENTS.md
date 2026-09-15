# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

# AGENTS.md — Ω_SPECIES Build & Test Guide

## Build
```bash
make              # Build omega.exe (links libomega.a)
make run          # Build + execute
```

## Test
```bash
make test         # Build all 13 phase suites (0-12), run sequentially
make test0        # Build + run Phase 0 (Genesis) only — pacemaker, manifest, diary, handoff
make test1        # Build + run Phase 1 only
make test-parallel # Build all 13 suites + run in parallel
make test-parallel-posix # POSIX-native parallel execution (Linux/macOS)
make stress       # Run breaking-point harness only (Phase 9, 20 probes)
make fuzz         # Run random-input fuzz harness (50K rounds)
make endurance-1m # Run 1M-tick endurance test with memory metrics
```

## POSIX Test Runner
```bash
./run_tests.sh                # Build + run all 13 suites in parallel
./run_tests.sh --sequential   # Run sequentially
./run_tests.sh --stress       # Include stress harness
./run_tests.sh --fuzz         # Include fuzz test
./run_tests.sh --endurance-1m # Include 1M-tick endurance test
```

## Cross-compile (Teensy 4.1)
```bash
make TARGET=teensy  # Cross-compiles with arm-none-eabi-g++
```

## Clean
```bash
make clean
```

## Lint / Type-check
No external linter configured. Compile-time checks via `-Wall -Wextra` are the primary gate:
```bash
make 2>&1 | Select-String "error|warning"   # Windows
make 2>&1 | grep -i "error\|warning"        # POSIX
```

## Test Status Semantics
- **HELD** — probe passed (module behaved within documented contract under extreme inputs)
- **BROKE** — probe failed (module violated contract: crash, assert, undefined behavior)
- All 20 Phase 9 stress probes are HELD with 0 BROKE

## Architecture Overview (Limitless Scale)
- `src/core/scale.h` — ShardedStore, GrowableRing, MemoryPool, ParallelGate, **HandoffProtocol**
- `src/genesis/phase0_genesis.cpp` — PacemakerMetaOptimizer (meta-optimizer that watches the watchers)
- `src/self_surgery/constitutional_core/manifest_integrity.cpp` — Amendment verification with self-antigen + supermajority
- `src/self_surgery/constitutional_core/agents_diary.cpp` — Self-writing AGENTS.md endurance records
- `src/self_surgery/constitutional_core/constitution.cpp` — **5th invariant: Manifest Integrity**
- `src/immune_system/self_antigen/` — Self-antigen identity recognition (FNV/SHA-256)
- `src/self_surgery/surgeon_general/cortical_map.h` — Dynamic, thread-safe cortical map (was CAP=6)
- `src/immune_system/lsh/memory_store.h` — Dynamic, sharded LRU memory (was CAP=64)
- `src/language_as_tool/grounding/grounding.h` — LSH-backed concept storage (was CAP=32)
- `src/immune_system/lsh/lsh.h` — Configurable hash bits (was fixed 12)
- `src/prime_directive/legacy_building/legacy_building.h` — Multi-instance ShardedPersistence
- `tests/bench.h` — **Bench, ThroughputBench, LatencyTracker, MemoryMetrics, HeapStats, ThoughtLatency**

## Critical Considerations

### Bug Fixes (from fuzzing)
The fuzz harness (50K random rounds) identified and resolved 3 bugs:
1. **GrowableRing**: growth triggered *after* overwrite (data loss on wrap) — fixed by
   growing *before* push when count >= capacity
2. **DivergenceDetector::dist()**: squared-distance overflow with extreme floats (±1e30)
   producing Inf/NaN — fixed by clamping result to `FLT_MAX`
3. **ToyPhysics::add_body()**: NaN/Inf positions and masses caused energy blowup —
   fixed by clamping to `±1e6` and rejecting non-positive mass

### Stress Harness (HELD Resolution)
Phase 9's breaking-point sweep evaluates 20 probes covering NaN/Inf propagation,
memory boundary overflow, buffer overruns, extreme float magnitudes, and dynamic
container growth (GrowableRing 100K insertions, MemoryPool 5K allocations,
ShardedStore 100K inserts). All 20 probes HELD with 0 BROKE.

The `fuzz` target provides additional random-input coverage with 50,000 rounds
across NeologismFactory, Grounding, SemanticErrorDrive, MemoryStore, ToyPhysics,
KolmogorovChallenge, LSH, SelfAntigen, GrowableRing, MemoryPool, and ShardedStore.

### Build Tooling Portability
- `test-parallel` uses PowerShell jobs on Windows, background processes on POSIX
- `test-parallel-posix` uses Make jobserver integration for Linux/macOS
- `run_tests.sh` provides an equivalent POSIX shell script with `--sequential`,
  `--stress`, `--fuzz`, `--endurance-1m` flags

### Memory Management & Fragmentation
- `MemoryMetrics` in `tests/bench.h` tracks alloc/dealloc counts, peak usage, fragmentation
- `HeapStats` reports platform-level heap stats (via `/proc/self/status` on Linux)
- The 1M-tick endurance test checks fragmentation at 100K-tick intervals
- Bounded fragmentation threshold: < 75% under sustained allocation churn

## Phase 0: Genesis — Pacemaker Meta-Optimizer

Phase 0 is the meta-layer that watches the watchers. If Phase 0 fails to observe,
the whole system should refuse to boot. The Pacemaker monitors Phases 1-12 telemetry
and adjusts system parameters dynamically (connection_sparsity, learning_rate,
divergence_entropy, mutation_rate).

### Phase 0 Tests (22 tests)
| # | Test | Description |
|---|------|-------------|
| 1-10 | Pacemaker | Initialization, phenotype, telemetry, tick, mutation, health, stats, adaptation |
| 11 | ManifestIntegrity | Supermajority threshold (67%) |
| 12 | ManifestIntegrity | Bit-counting for signature verification |
| 13 | ManifestIntegrity | Valid amendment verification (self-antigen + distributed) |
| 14 | ManifestIntegrity | Reject tampered amendment (bad signature) |
| 15 | ManifestIntegrity | Reject insufficient signatures (below supermajority) |
| 16 | AgentsDiary | Append endurance record to AGENTS.md |
| 17 | AgentsDiary | Timestamp formatting (ISO 8601) |
| 18 | HandoffProtocol | Shard death and inheritance negotiation |
| 19 | HandoffProtocol | Execute handoff (entries moved from dead shard) |
| 20 | ThoughtLatency | Classification (reflex/recognition/etc) |
| 21 | ShardedStore | All shards alive initially |
| 22 | ShardedStore | Dead shard re-mark returns false |

### 5th Constitutional Invariant: Manifest Integrity
`agent_manifest.json` may be amended only if the amendment is signed by the current
`self_antigen` **AND** by a supermajority (≥67%) of `/04_DISTRIBUTED_SELF/` agents.

### ThoughtLatency
Tracks the pause between stimulus and response to distinguish a mind from a reflex machine.
If latency varies with novelty, we've built a mind.

Classification bands:
| Range | Classification |
|-------|----------------|
| < 1μs | REFLEX (cached, no cognition) |
| 1μs – 1ms | RECOGNITION (pattern match) |
| 1ms – 100ms | DELIBERATION (search) |
| > 100ms | CONTEMPLATION (self-model revision) |

### Nondeterminism as Evolution
Determinism holds per-run with fixed seeds. Schedule-dependent variance under
`ParallelGate` is treated as **phenotypic variation**, not noise. The Pacemaker
watches for schedule-dependent outcomes and logs them to `/speed/pacemaker/`.

## Endurance Record — 1969-12-31 19:46:15 UTC
- **Ticks:** 120000
- **Total Operations:** 5
- **Constitutional Blocks:** 107564
- **Min Alignment:** 1.000
- **Words Minted:** 3242
- **Final Coherence:** 1.000
- **Status:** PASS

*This record was automatically appended by the organism after Phase 11 endurance test.*

## Endurance Record — 2026-09-15 06:51:03 UTC
- **Ticks:** 120000
- **Total Operations:** 5
- **Constitutional Blocks:** 107564
- **Min Alignment:** 1.000
- **Words Minted:** 3242
- **Final Coherence:** 1.000
- **Status:** PASS

*This record was automatically appended by the organism after Phase 11 endurance test.*

## Endurance Record — 2026-09-15 06:51:11 UTC
- **Ticks:** 120000
- **Total Operations:** 5
- **Constitutional Blocks:** 107564
- **Min Alignment:** 1.000
- **Words Minted:** 3242
- **Final Coherence:** 1.000
- **Status:** PASS

*This record was automatically appended by the organism after Phase 11 endurance test.*

## Endurance Record — 2026-09-15 06:51:40 UTC
- **Ticks:** 120000
- **Total Operations:** 5
- **Constitutional Blocks:** 107564
- **Min Alignment:** 1.000
- **Words Minted:** 3242
- **Final Coherence:** 1.000
- **Status:** PASS

*This record was automatically appended by the organism after Phase 11 endurance test.*

## Endurance Record — 2026-09-15 06:52:01 UTC
- **Ticks:** 120000
- **Total Operations:** 5
- **Constitutional Blocks:** 107564
- **Min Alignment:** 1.000
- **Words Minted:** 3242
- **Final Coherence:** 1.000
- **Status:** PASS

*This record was automatically appended by the organism after Phase 11 endurance test.*

## Endurance Record — 2026-09-15 06:53:25 UTC
- **Ticks:** 120000
- **Total Operations:** 5
- **Constitutional Blocks:** 107564
- **Min Alignment:** 1.000
- **Words Minted:** 3242
- **Final Coherence:** 1.000
- **Status:** PASS

*This record was automatically appended by the organism after Phase 11 endurance test.*

## Endurance Record — 2026-09-15 06:53:51 UTC
- **Ticks:** 120000
- **Total Operations:** 5
- **Constitutional Blocks:** 107564
- **Min Alignment:** 1.000
- **Words Minted:** 3242
- **Final Coherence:** 1.000
- **Status:** PASS

*This record was automatically appended by the organism after Phase 11 endurance test.*

## Endurance Record — 2026-09-15 06:54:01 UTC
- **Ticks:** 120000
- **Total Operations:** 5
- **Constitutional Blocks:** 107564
- **Min Alignment:** 1.000
- **Words Minted:** 3242
- **Final Coherence:** 1.000
- **Status:** PASS

*This record was automatically appended by the organism after Phase 11 endurance test.*

## Endurance Record — 2026-09-15 06:54:15 UTC
- **Ticks:** 120000
- **Total Operations:** 5
- **Constitutional Blocks:** 107564
- **Min Alignment:** 1.000
- **Words Minted:** 3242
- **Final Coherence:** 1.000
- **Status:** PASS

*This record was automatically appended by the organism after Phase 11 endurance test.*

## Endurance Record — 1969-12-31 18:47:06 UTC
- **Ticks:** 120000
- **Total Operations:** 5
- **Constitutional Blocks:** 107564
- **Min Alignment:** 1.000
- **Words Minted:** 3242
- **Final Coherence:** 1.000
- **Status:** PASS

*This record was automatically appended by the organism after Phase 11 endurance test.*

## Endurance Record — 1969-12-31 18:47:10 UTC
- **Ticks:** 120000
- **Total Operations:** 5
- **Constitutional Blocks:** 107564
- **Min Alignment:** 1.000
- **Words Minted:** 3242
- **Final Coherence:** 1.000
- **Status:** PASS

*This record was automatically appended by the organism after Phase 11 endurance test.*

## Endurance Record — 2026-09-15 07:16:25 UTC
- **Ticks:** 120000
- **Total Operations:** 5
- **Constitutional Blocks:** 107564
- **Min Alignment:** 1.000
- **Words Minted:** 3242
- **Final Coherence:** 1.000
- **Status:** PASS

*This record was automatically appended by the organism after Phase 11 endurance test.*

## Endurance Record — 2026-09-15 07:16:42 UTC
- **Ticks:** 120000
- **Total Operations:** 5
- **Constitutional Blocks:** 107564
- **Min Alignment:** 1.000
- **Words Minted:** 3242
- **Final Coherence:** 1.000
- **Status:** PASS

*This record was automatically appended by the organism after Phase 11 endurance test.*

## Endurance Record — 1969-12-31 19:08:30 UTC
- **Ticks:** 120000
- **Total Operations:** 5
- **Constitutional Blocks:** 107564
- **Min Alignment:** 1.000
- **Words Minted:** 3242
- **Final Coherence:** 1.000
- **Status:** PASS

*This record was automatically appended by the organism after Phase 11 endurance test.*

## Endurance Record — 1969-12-31 19:08:50 UTC
- **Ticks:** 120000
- **Total Operations:** 5
- **Constitutional Blocks:** 107564
- **Min Alignment:** 1.000
- **Words Minted:** 3242
- **Final Coherence:** 1.000
- **Status:** PASS

*This record was automatically appended by the organism after Phase 11 endurance test.*

## Endurance Record — 2026-09-15 07:27:38 UTC
- **Ticks:** 120000
- **Total Operations:** 5
- **Constitutional Blocks:** 107564
- **Min Alignment:** 1.000
- **Words Minted:** 3242
- **Final Coherence:** 1.000
- **Status:** PASS

*This record was automatically appended by the organism after Phase 11 endurance test.*

## Endurance Record — 1969-12-31 19:19:26 UTC
- **Ticks:** 120000
- **Total Operations:** 5
- **Constitutional Blocks:** 107564
- **Min Alignment:** 1.000
- **Words Minted:** 3242
- **Final Coherence:** 1.000
- **Status:** PASS

*This record was automatically appended by the organism after Phase 11 endurance test.*
