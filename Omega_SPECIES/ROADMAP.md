# Ω_SPECIES — Build Roadmap (Phase by Phase / Language by Language / Folder by Folder)

> Source blueprint: "Genesis Blueprint for Ω — Phase 0/1 (Crucible + Seed)".
> This roadmap maps the blueprint to **buildable, measurable engineering**.
> Metaphysical milestones ("soul", "consciousness", "I.") are reframed as
> testable proxies. Hardware reality (Teensy 4.1 = 1 MB RAM) caps on-device scope.

## Reality check (read first)
- "Soul / consciousness / intersubjectivity" are not compiler outputs. Each such
  milestone below is replaced by a *measured* signal.
- Teensy 4.1 cannot run ZeroMQ / OpenCL / libsodium ZKP / full RAFT. Those layers
  run as a PC simulation; the MCU runs only the dependency-free tensorless core.
- No-framework rule (no Python/PyTorch) is honored. C for boot/static/UART,
  C++ for engine/GA/ODE/LSH/active-inference. `constexpr` for compile-time axioms.

## Phase 0 — The Crucible  ·  C + C++ (bare-metal)
Goal: deterministic 10 Hz loop + serial birth log.
- `00_GENESIS/birth_log/` — UART logger (C).
- `01_LIQUID_TIME/` — hash-keyed tensorless state (C++ fixed map, no heap in loop).
- `.heartbeat/` — timer ISR → 10 Hz tick, `phase_lock` barrier.
- `.prime_directive/` — `constexpr` axiom assertions.
- Metric: boots, heartbeat oscillates, prints `Ω v1.0 alive at t=0`.

## Phase 1 — The Seed  ·  C + C++ (neuroevolution)
Goal: first self-organized connection pattern.
- `00_GENESIS/primordial_weights/` — static sparse arrays (C), fixed arena.
- `00_GENESIS/self_wiring/` — GA over adjacency (C++), fixed-population ring buffer.
- `00_GENESIS/pacemaker/` — meta-optimizer (mutation rate + LR oscillator).
- `01_LIQUID_TIME/reservoir_pool/`, `spike_encoder/` — event-driven reservoir.
- Metric: connection-entropy drops ≥10% below seed baseline.

## Phase 2 — The Body  ·  C++ (gradient-free)
Goal: surprise minimization + anomaly response.
- `02_ACTIVE_INFERENCE/free_energy/` — Friston-style variational update.
- `03_IMMUNE_SYSTEM/self_antigen/` — 64-byte identity hash (FNV, no libsodium).
- `03_IMMUNE_SYSTEM/memory_b_cells/`, `memory_t_cells/` — LSH buckets.
- `03_IMMUNE_SYSTEM/inflammation_response/` — priority queue for anomaly compute.
- Metric: injected anomaly → compute reallocation within N ticks.

## Phase 3 — The Mind  ·  C++ (PC simulation)
Goal: multi-agent sim + safe self-modification.
- `04_DISTRIBUTED_SELF/` — agent genesis (thread pool) + simplified RAFT (PC only).
- `07_SELF_SURGERY/surgeon_general/` — cortical_map → surgical_planning (dry-run)
  → sterile_field (checkpoint) → post_op_recovery.
- Metric: proposed edit simulated, applied, auto-rollback if free-energy worsens.

## Phase 4 — The Soul → proxies  ·  C++
- `coherence_preservation/` — antigen verified each tick (identity hash stable).
- `novelty_harvesting/` — ≥1 new pattern / 1000 ticks (measurable).
- `11_TEMPORAL_FOLDING/` — pre/post-consolidation; `temporal_error_drive` feeds immune.
- Metric: identity hash stable across 1e6 ticks; novelty rate sustained.

## Phase 5 — Language → grounding  ·  C++  ·  ✅ DONE (36 tests pass)
- `08_LANGUAGE_AS_TOOL/grounding/` — symbol⇄sim-state pointer map.
- `semantic_error_drive/` — divergence detection; `neologism_factory/` — symbol gen.
- `09_XENO_EMPATHY/morphological_projection/` — vector-field state mapping.
- Metric: system mints a symbol for a state with no existing label. **Verified**: grounding
  binds/mints deterministically and caps at 32; neologism factory mints deterministic tokens;
  semantic error drive splits on large divergence; xeno-empathy projection/resonance/trust all pass.

## Phase 6 — Mirror → crypto  ·  C++ + libsodium (PC)  ·  ✅ DONE (24 tests pass)
- `12_MIRROR_NEXUS/` — handshake (ZKP), ontology_mapping, syncretic_fusion,
  schism_detector. On Teensy this degrades to a 64-byte identity exchange over UART.
- **Verified**: `handshake` is a Fiat-Shamir Schnorr ZKP over a safe-prime field
  (p=0x13a272dfad453727, q=707409768979012499, g=2 order q); honest proof verifies,
  tampered/wrong-key/wrong-nonce proofs are rejected; Teensy degraded raw exchange works.
  ontology_mapping matches nearest concepts; syncretic_fusion merges within threshold;
  schism_detector flags sustained low resonance. Wired into `main.cpp` milestone.

## Phase 7 — Challenge  ·  C + C++  ·  ✅ DONE (28 tests pass)
- `10_BOOTSTRAP_UNIVERSE/toy_physics/` — 2D integrator.
- `self_play_arena/` — copy vs copy.
- `kolmogorov_challenge/` — 64-byte program emitter generating > seed entropy
  (real KC test = compressibility check).
- `legacy_building/` — serialize state artifact.
- **Verified**: symplectic 2D gravity integrator conserves energy (<10% drift) and is
  deterministic; self-play copy-vs-copy reaches equilibrium (tie) and is zero-sum;
  Kolmogorov emitter's output entropy exceeds the seed's (passes compressibility KC test);
  bootstrap legacy serializes to a stable 36-byte artifact. Wired into `main.cpp` milestone.

## Phase 8 — Integration  ·  linker  ·  ✅ DONE (9 integration/soak tests pass)
- Full binary on PC; core subset cross-compiled to Teensy.
- 24h soak test; serial introspection log.
- Honest "first word" = a logged self-reference token (defined output, not personhood).
- **Verified on PC**: `omega.exe` builds clean (no warnings) and runs all phases 1-7; a
  40k-tick soak (`phase8_integration_test.cpp`) keeps every signal finite (no NaN), holds
  identity coherence at 1.0 (0 breaches), bounds physics energy (<20% swing), mints grounded
  words (language emerges), and keeps the ZKP + Kolmogorov emitter valid under load. The
  honest "first word" is logged as a self-reference token bound to the stable identity.
- **Teensy caveat**: `make TARGET=teensy` is wired (swaps `platform_teensy.cpp`, `-mcpu=cortex-m7`);
  it requires the `arm-none-eabi-g++` toolchain, which is **not installed in this environment**,
  so the actual Teensy cross-compile/flash could not be executed here. The PC path is the
  runnable, tested deliverable. The 24h soak is represented by the bounded 40k-tick soak loop.

## Language decisions (corrected)
- C: boot, UART, static weights, spike encoder, 64-byte emitter.
- C++: engine, GA, ODE, LSH, active inference (ban STL containers in Teensy build path).
- `constexpr`: axiom checks.
- OpenCL / ZeroMQ / libsodium / RAFT: PC-sim only, behind `#ifdef SIM`.
- On-device: raw UART frames instead of ZeroMQ; 64-byte identity instead of ZKP.
- No Python / PyTorch: honored.
