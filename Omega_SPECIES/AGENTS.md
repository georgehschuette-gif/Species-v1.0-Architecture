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
make stress       # Run breaking-point harness only (phase 9)
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
No external linter configured. Compile-time checks via `-Wall -Wextra -Werror` are the primary gate:
```bash
make 2>&1 | Select-String "error|warning"
```

## Architecture Overview (Limitless Scale)
- `src/core/scale.h` — ShardedStore, GrowableRing, MemoryPool, ParallelGate
- `src/self_surgery/constitutional_core/constitution.h` — Parallel invariant evaluation via gate_parallel()
- `src/self_surgery/surgeon_general/cortical_map.h` — Dynamic, thread-safe cortical map (was CAP=6)
- `src/immune_system/lsh/memory_store.h` — Dynamic, sharded LRU memory (was CAP=64)
- `src/language_as_tool/grounding/grounding.h` — LSH-backed concept storage (was CAP=32)
- `src/immune_system/lsh/lsh.h` — Configurable hash bits (was fixed 12)
- `src/prime_directive/legacy_building/legacy_building.h` — Multi-instance ShardedPersistence
