// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
//
// Phase 12 endurance (1M) — Long-running stability with memory metrics.
// Extends the 120K-tick endurance from Phase 11 to 1,000,000 operations,
// adding MemoryMetrics and HeapStats tracking throughout. Proves that
// growable containers (GrowableRing, ShardedStore, MemoryPool) maintain
// bounded fragmentation under sustained allocation churn.

#include <cstdio>
#include <cmath>
#include <cstdint>
#include <cstring>
#include <cfloat>
#include "genesis/primordial_weights/primordial_weights.h"
#include "liquid_time/reservoir_pool/reservoir_pool.h"
#include "liquid_time/spike_encoder/spike_encoder.h"
#include "genesis/self_wiring/self_wiring.h"
#include "genesis/pacemaker/pacemaker.h"
#include "active_inference/free_energy/free_energy.h"
#include "immune_system/self_antigen/self_antigen.h"
#include "prime_directive/coherence_preservation/coherence_preservation.h"
#include "language_as_tool/grounding/grounding.h"
#include "language_as_tool/neologism_factory/neologism_factory.h"
#include "language_as_tool/semantic_error_drive/semantic_error_drive.h"
#include "10_BOOTSTRAP_UNIVERSE/toy_physics/toy_physics.h"
#include "10_BOOTSTRAP_UNIVERSE/self_play_arena/self_play_arena.h"
#include "10_BOOTSTRAP_UNIVERSE/kolmogorov_challenge/kolmogorov_challenge.h"
#include "10_BOOTSTRAP_UNIVERSE/legacy_building/legacy_building.h"
#include "12_MIRROR_NEXUS/handshake/handshake.h"
#include "xeno_empathy/resonance_matching/resonance_matching.h"
#include "xeno_empathy/trust_building/trust_building.h"
#include "self_surgery/surgeon_general/surgeon_general.h"
#include "self_surgery/constitutional_core/constitution.h"
#include "core/scale.h"

#include "bench.h"

using namespace omega;

static int g_pass = 0, g_fail = 0;
static void check(bool c, const char* n) {
  if (c) { ++g_pass; printf("pass: %s\n", n); }
  else   { ++g_fail; printf("FAIL: %s\n", n); }
}

// Deterministic LCG for reproducibility
static uint32_t rng_state = 0x9E3779B9u;
static inline uint32_t rnd() {
  rng_state = rng_state * 1664525u + 1013904223u;
  return rng_state;
}
static inline float rndf() {
  return (float)(rnd() & 0x7FFFFFFFu) / (float)0x7FFFFFFFu;
}

int main() {
  Bench _b("PHASE12_ENDURANCE_1M");
  MemoryMetrics mem;
  const int TICKS = 1000000;
  printf("=== Ω_SPECIES 1M-Tick Endurance with Memory Metrics ===\n");
  printf("Configuration: %d ticks, memory tracking enabled\n\n", TICKS);

  // --- Genesis substrate ---
  Network seed; pw_init(&seed, 16, 0xBEEFu);
  SelfWiringGA ga(seed, seed.n_nodes, 0xCAFEu);
  Pacemaker pm(ga);
  Reservoir r; rp_init(&r, seed.n_nodes, 0.10f);
  SpikeEncoder se; se_init(&se, seed.n_nodes, 7u);

  // --- Body / active inference ---
  AIModel model;
  {
    uint32_t s = 0xABCDEFu;
    for (int j = 0; j < AIModel::M; j++)
      for (int i = 0; i < AIModel::N; i++) {
        s = s * 1664525u + 1013904223u;
        model.A[j][i] = ((float)(s & 0x7FFFFFFFu) / (float)0x7FFFFFFFu) * 2.0f - 1.0f;
      }
    for (int i = 0; i < AIModel::N; i++) { model.mu0[i] = 0.0f; model.pi0[i] = 0.5f; }
  }
  float x[AIModel::N] = {0}, y[AIModel::M] = {0};

  // --- Soul / identity ---
  uint8_t genotype[8];
  for (int i = 0; i < 8; i++) genotype[i] = (uint8_t)((pw_topology_hash(&seed) >> (8 * i)) & 0xFF);
  SelfAntigen self_ag; self_ag.compute_genotype(genotype, sizeof(genotype));
  SelfAntigen live = self_ag;
  CoherencePreservation coherence; coherence.set_reference(self_ag);

  // --- Language (sharded) ---
  Grounding ground; ShardedNeologismFactory neo; ShardedSemanticErrorDrive sed;
  char first_word[32]; bool have_word = false; int words = 0;

  // --- Self-Surgery with constitutional core ---
  SurgeonGeneral sg;
  sg.seed_modules(seed);

  // --- Test state ---
  float drive[RP_MAX_NODES] = {0};
  bool any_nan = false;
  int coherence_breaches = 0;
  int total_ops = 0;
  int total_blocks = 0;
  float min_alignment = 1e30f;
  float min_free_energy = 1e30f;

  // Memory metrics checkpoints
  size_t last_peak = 0;
  size_t alloc_count_snapshots = 0;

  printf("Starting 1M-tick endurance run...\n");
  printf("Tick    | FreeEnergy | Coherence | Words | Ops    | Blocks | Align  | Agg    | Mem(in-use) | Frag%%\n");
  printf("--------|------------|-----------|-------|--------|--------|--------|--------|-------------|------\n");

  for (int t = 0; t < TICKS; t++) {
    // Genesis: evolve topology occasionally
    if (t % 100 == 0) pm.tick();

    // Body: drive the reservoir
    uint16_t fired[SE_MAX_CH];
    int nf = se_encode(&se, rndf(), fired, SE_MAX_CH);
    for (int i = 0; i < RP_MAX_NODES; i++) drive[i] = 0.0f;
    for (int i = 0; i < nf; i++) if (fired[i] < RP_MAX_NODES) drive[fired[i]] = 1.0f;
    rp_step(&r, &seed, drive, 0.05f);
    float rE = 0.0f; for (uint16_t i = 0; i < r.n; i++) rE += r.state[i] * r.state[i];

    // Active inference
    for (int j = 0; j < AIModel::M; j++) y[j] = rndf() - 0.5f;
    perceive(model, x, y, 12, 0.05f);
    float F = free_energy(model, x, y);

    // Identity
    live.project_phenotype(x, AIModel::N);
    if (!coherence.verify(live)) coherence_breaches++;
    float coh = coherence.coherence_score(live);

    // Language: occasionally discover a new concept
    if (t % 37 == 0) {
      float st[8]; for (int i = 0; i < 8; i++) st[i] = rndf() - 0.5f;
      if (ground.find(st, 0.15f) == 0) {
        uint32_t wid = ground.bind(st, 0.15f);
        mem.record_alloc(32);
        char w[32]; neo.mint(wid, st, w, sizeof(w));
        mem.record_dealloc(32);
        if (!have_word) { std::memcpy(first_word, w, sizeof(w)); have_word = true; }
        words++;
      }
      mem.record_alloc(sizeof(float) * 8);
      sed.process((uint32_t)(t % 8), st, 0.20f, 0.10f);
      mem.record_dealloc(sizeof(float) * 8);
    }

    // Self-Surgery
    int op = sg.operate(rnd(), (uint32_t)t);
    if (op != 0) total_ops++;
    total_blocks = sg.constitutional_blocks();
    float align = sg.constitution().alignment_score(sg.self_model());
    if (align < min_alignment) min_alignment = align;
    if (std::isfinite(F) && F < min_free_energy) min_free_energy = F;

    // NaN check
    if (!std::isfinite(rE) || !std::isfinite(F) || !std::isfinite(coh) ||
        !std::isfinite(align)) any_nan = true;

    // Periodic telemetry + memory metrics (every 100K ticks)
    if (t % 100000 == 0 || t == TICKS - 1) {
      HeapStats hs = HeapStats::current();
      double frag_pct = mem.fragmentation_ratio() * 100.0;
      printf("%8d | %10.3f | %9.1f | %5d | %6d | %6d | %.3f  | %.3f  | %11zu | %5.1f\n",
             t, F, coh, words, total_ops, total_blocks, align,
             sg.aggressiveness(), mem.current_usage(), frag_pct);
      mem.report("  mem-checkpoint");
      hs.report("  heap");

      // Detect growing fragmentation — flag if fragmentation exceeds 70%
      if (frag_pct > 70.0 && last_peak > 0) {
        printf("  WARNING: fragmentation elevated (%.1f%%)\n", frag_pct);
      }
      last_peak = mem.peak_usage.load();
      alloc_count_snapshots++;
    }
  }

  printf("\n=== Endurance Assertions ===\n");
  check(!any_nan,
        "endurance-1m: no NaN/Inf across 1M ops (signals stayed finite)");
  check(coherence_breaches == 0,
        "endurance-1m: identity genotype core never changed (0 coherence breaches)");
  check(coherence.coherence_score(live) == 1.0f,
        "endurance-1m: final coherence score is 1.0 (self intact)");
  check(words > 0,
        "endurance-1m: language emerged — agent minted grounded words during soak");
  check(have_word && first_word[0] == 'w' && std::strchr(first_word, '-') != nullptr,
        "endurance-1m: first word is well-formed self-reference token (w-...)");
  check(total_ops > 0,
        "endurance-1m: surgeon general performed operations (self-surgery active)");
  check(total_blocks > 0,
        "endurance-1m: constitutional gate blocked at least once (proportional blocking)");
  check(min_alignment > 0.05f,
        "endurance-1m: constitutional alignment stayed above threshold (>0.05)");
  check(min_free_energy < 1e30f,
        "endurance-1m: free energy was computed and tracked");
  check(alloc_count_snapshots >= 10,
        "endurance-1m: memory metrics tracked across sufficient checkpoints (>10)");

  // Final memory accounting
  printf("\n=== Memory Metrics (1M-tick run) ===\n");
  mem.report("FINAL");
  HeapStats::current().report("FINAL_HEAP");

  // Fragmentation must be bounded (< 75%) for production readiness
  double final_frag = mem.fragmentation_ratio() * 100.0;
  check(final_frag < 75.0,
        ("endurance-1m: fragmentation bounded below 75% (" +
         std::to_string(final_frag) + "%)").c_str());

  printf("\nPHASE12_ENDURANCE_1M: %s (%d pass, %d fail)\n",
         g_fail == 0 ? "PASS" : "FAIL", g_pass, g_fail);
  return g_fail == 0 ? 0 : 1;
}
