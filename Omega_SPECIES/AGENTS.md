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
make test         # Build all 12 phase suites, run sequentially
make test-parallel # Build all 12 phase suites, run concurrently
make test-parallel-posix # POSIX-native parallel execution (Linux/macOS)
make stress       # Run breaking-point harness only (Phase 9, 20 probes)
make fuzz         # Run random-input fuzz harness (50K rounds)
make endurance-1m # Run 1M-tick endurance test with memory metrics
```

## POSIX Test Runner
```bash
./run_tests.sh                # Build + run all 12 suites in parallel
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
- `src/core/scale.h` — ShardedStore, GrowableRing, MemoryPool, ParallelGate
- `src/self_surgery/constitutional_core/constitution.h` — Parallel invariant evaluation via gate_parallel()
- `src/self_surgery/surgeon_general/cortical_map.h` — Dynamic, thread-safe cortical map (was CAP=6)
- `src/immune_system/lsh/memory_store.h` — Dynamic, sharded LRU memory (was CAP=64)
- `src/language_as_tool/grounding/grounding.h` — LSH-backed concept storage (was CAP=32)
- `src/immune_system/lsh/lsh.h` — Configurable hash bits (was fixed 12)
- `src/prime_directive/legacy_building/legacy_building.h` — Multi-instance ShardedPersistence
- `tests/bench.h` — Bench, ThroughputBench, LatencyTracker, MemoryMetrics, HeapStats

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
