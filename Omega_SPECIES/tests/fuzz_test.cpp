// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
//
// Fuzz Test 0 — Random-Input Harness
// Feeds degenerate, extreme, and randomized inputs across all core modules
// to surface unknown failures: NaN/Inf propagation, zero-size buffers,
// memory boundary overflow, extreme float magnitudes, and hash edge cases.
// Uses a deterministic LCG so failures are reproducible.

#include <cstdio>
#include <cstdint>
#include <cmath>
#include <cstring>
#include <climits>
#include <cfloat>
#include <limits>
#include "genesis/primordial_weights/primordial_weights.h"
#include "liquid_time/reservoir_pool/reservoir_pool.h"
#include "immune_system/lsh/lsh.h"
#include "immune_system/lsh/memory_store.h"
#include "immune_system/self_antigen/self_antigen.h"
#include "language_as_tool/grounding/grounding.h"
#include "language_as_tool/neologism_factory/neologism_factory.h"
#include "language_as_tool/semantic_error_drive/semantic_error_drive.h"
#include "10_BOOTSTRAP_UNIVERSE/toy_physics/toy_physics.h"
#include "10_BOOTSTRAP_UNIVERSE/kolmogorov_challenge/kolmogorov_challenge.h"
#include "core/scale.h"
#include "bench.h"

using namespace omega;
using namespace omega::core;

// Deterministic LCG for reproducible fuzzing
static uint32_t rng_state = 0xDEADBEEFu;
static inline uint32_t rng() {
  rng_state = rng_state * 1664525u + 1013904223u;
  return rng_state;
}
static inline float rng_float() {  uint32_t raw = rng();
  // Occasional extreme values for deeper coverage
  switch (raw % 12) {
    case 0:  return NAN;
    case 1:  return INFINITY;
    case 2:  return -INFINITY;
    case 3:  return 0.0f;
    case 4:  return -0.0f;
    case 5:  return std::numeric_limits<float>::max();
    case 6:  return -std::numeric_limits<float>::max();
    case 7:  return std::numeric_limits<float>::denorm_min();
    case 8:  return 1e30f;
    case 9:  return -1e30f;
    case 10: return 1e-30f;
    default: return (float)(raw & 0x7FFFFFFFu) / (float)0x7FFFFFFFu * 2.0f - 1.0f;
  }
}

static int g_pass = 0, g_fail = 0;
static void check(bool c, const char* n) {
  if (c) { ++g_pass; printf("  pass: %s\n", n); }
  else   { ++g_fail; printf("  FAIL: %s\n", n); }
}

int main() {
  Bench _b("FUZZ");
  MemoryMetrics mem;
  const int ROUNDS = 50000;
  printf("=== Ω_SPECIES Fuzz Test (seed=0x%X, %d rounds) ===\n",
         rng_state, ROUNDS);

  int crashes = 0;

  // ---- NeologismFactory fuzzing ----
  {
    NeologismFactory neo;
    for (int i = 0; i < 5000; i++) {
      uint32_t cid = rng() ? rng() : 1u;
      float st[8];
      for (int j = 0; j < 8; j++) st[j] = rng_float();
      int buf_sz = (rng() % 16) + 1;  // 1..16
      char* buf = (char*)malloc(buf_sz);
      mem.record_alloc(buf_sz);
      memset(buf, 0xCD, buf_sz);
      neo.mint(cid, st, buf, buf_sz);
      // Verify null-termination within buffer
      bool null_terminated = (buf[buf_sz - 1] == '\0') ||
                             (memchr(buf, '\0', buf_sz) != nullptr);
      if (!null_terminated) crashes++;
      mem.record_dealloc(buf_sz);
      free(buf);
    }
    check(crashes == 0, "neologism mint: no buffer overruns across random sizes/states");
  }

  // ---- ShardedNeologismFactory sharding consistency ----
  {
    crashes = 0;
    for (int shards = 1; shards <= 64; shards *= 2) {
      ShardedNeologismFactory factory((size_t)shards);
      for (int i = 0; i < 2000; i++) {
        uint32_t cid = rng();
        float st[8];
        for (int j = 0; j < 8; j++) st[j] = rng_float();
        char buf[32];
        memset(buf, 0, sizeof(buf));
        factory.mint(cid, st, buf, sizeof(buf));
        if (buf[sizeof(buf)-1] != '\0' && memchr(buf, '\0', sizeof(buf)) == nullptr)
          crashes++;
        // Verify sharding routes correctly
        check(factory.shard_index(cid) == cid % (size_t)shards,
              ("sharded factory shard routing (shards=" + std::to_string(shards) + ")").c_str());
      }
    }
    check(crashes == 0, "sharded neologism factory: no crashes across shard counts 1..64");
  }

  // ---- Grounding fuzzing (NaN, extreme states) ----
  {
    crashes = 0;
    for (int i = 0; i < 5000; i++) {
      Grounding g;
      float st[8];
      for (int j = 0; j < 8; j++) st[j] = rng_float();
      uint32_t id = g.bind(st, 0.15f);
      uint32_t found = g.find(st, 0.15f);
      // If state has NaN, bind should reject (return 0)
      bool has_nan = false;
      for (int j = 0; j < 8; j++)
        if (!std::isfinite(st[j])) has_nan = true;
      if (has_nan) {
        if (id != 0) crashes++;
        if (found != 0) crashes++;
      } else {
        // For finite states, bind and find should agree
        if (id != 0 && found == 0) crashes++;
      }
    }
    check(crashes == 0, "grounding: NaN rejected consistently, finite states match");
  }

  // ---- MemoryStore fuzzing ----
  {
    crashes = 0;
    MemoryStore ms;
    LSH lsh; lsh.init(0xF00Du, 16);
    ms.init(&lsh, 0.01f, 0.5f, 256);
    for (int i = 0; i < 3000; i++) {
      uint32_t tag = rng();
      float vec[MemoryStore::V];
      for (int j = 0; j < MemoryStore::V; j++) vec[j] = rng_float();
      float strength = (float)(rng() & 0xFF) / 255.0f;
      ms.store(tag, vec, strength);
      float out[MemoryStore::V];
      float ret = ms.recall(tag, out);
      (void)ret;
    }
    check(crashes == 0, "memory store: no crashes with random tags/vectors/strengths");
  }

  // ---- SemanticErrorDrive fuzzing ----
  {
    crashes = 0;
    SemanticErrorDrive sed;
    for (int i = 0; i < 10000; i++) {
      uint32_t id = (uint32_t)i % 16;
      float st[8];
      for (int j = 0; j < 8; j++) st[j] = rng_float();
      float d = sed.process(id, st, 0.20f, 0.10f);
      if (!std::isfinite(d) && d != 0.0f) crashes++;
    }
    check(crashes == 0, "semantic error drive: process returns finite/zero for all inputs");
  }

  // ---- ShardedSemanticErrorDrive fuzzing ----
  {
    crashes = 0;
    ShardedSemanticErrorDrive ssed(8);
    std::vector<std::pair<uint32_t, const float*>> batch;
    for (int i = 0; i < 2000; i++) {
      uint32_t id = rng();
      float* st = new float[8];
      for (int j = 0; j < 8; j++) st[j] = rng_float();
      mem.record_alloc(32);
      batch.emplace_back(id, st);
    }
    auto results = ssed.process_batch(batch, 0.20f, 0.10f);
    for (size_t i = 0; i < batch.size(); i++) {
      if (!std::isfinite(results[i]) && results[i] != 0.0f) crashes++;
      mem.record_dealloc(32);
      delete[] batch[i].second;
    }
    check(crashes == 0, "sharded semantic error drive: process_batch returns finite values");
  }

  // ---- ToyPhysics fuzzing ----
  {
    crashes = 0;
    for (int i = 0; i < 500; i++) {
      ToyPhysics tp;
      for (int b = 0; b < 4; b++) {
        float x = rng_float(), y = rng_float();
        float vx = rng_float(), vy = rng_float();
        float mass = (float)(rng() % 1000) + 0.001f;
        tp.add_body(x, y, vx, vy, mass);
      }
      for (int t = 0; t < 50; t++) tp.step(0.001f);
      float e = tp.energy();
      if (!std::isfinite(e)) crashes++;
    }
    check(crashes == 0, "toy physics: energy stays finite under random initial conditions");
  }

  // ---- KolmogorovChallenge fuzzing ----
  {
    crashes = 0;
    for (int i = 0; i < 200; i++) {
      KolmogorovChallenge kc;
      uint8_t prog[64];
      for (int j = 0; j < 64; j++) prog[j] = (uint8_t)rng();
      kc.set_program(prog);
      uint8_t out[256];
      int emit_n = (rng() % 5) * 64;  // 0, 64, 128, 192, or 256
      if (emit_n > 0) {
        kc.emit(out, emit_n);
        float ent = kc.output_entropy();
        if (!std::isfinite(ent) || ent < 0.0f) crashes++;
      }
      // n=0 must be safe
      kc.emit(out, 0);
      if (!std::isfinite(kc.output_entropy()) || kc.output_entropy() < 0.0f) crashes++;
    }
    check(crashes == 0, "kolmogorov challenge: emit(n=0..256) safe, entropy valid");
  }

  // ---- LSH fuzzing with NaN/Inf ----
  {
    crashes = 0;
    LSH lsh; lsh.init(0xBEEFu, 20);
    for (int i = 0; i < 10000; i++) {
      float v[8];
      for (int j = 0; j < 8; j++) v[j] = rng_float();
      uint32_t b = lsh.bucket(v);
      (void)b;
      // Should not crash even with NaN/Inf
    }
    check(crashes == 0, "LSH: bucket computation handles NaN/Inf without crash");
  }

  // ---- SelfAntigen fuzzing ----
  {
    crashes = 0;
    SelfAntigen sa;
    uint8_t gen[32];
    for (int i = 0; i < 32; i++) gen[i] = (uint8_t)rng();
    sa.compute_genotype(gen, 32);
    SelfAntigen clone = sa;
    check(clone.is_self(sa), "self antigen: clone recognized as self");
    uint8_t gen2[32];
    for (int i = 0; i < 32; i++) gen2[i] = (uint8_t)(rng() ^ 0xFF);
    SelfAntigen sa2; sa2.compute_genotype(gen2, 32);
    check(!sa2.is_self(sa), "self antigen: different genotype not recognized as self");
  }

// GrowableRing stress
  {
    GrowableRing<uint32_t> ring(4);
    crashes = 0;
    for (uint32_t i = 0; i < 10000; i++) {
      ring.push(i);
    }
    for (uint32_t i = 0; i < 5000; i++) {
      uint32_t val = ring.at(i);
      if (val != i) crashes++;
    }
    check(ring.size() == 10000, "growable ring: size matches after 10K pushes");
    check(crashes == 0, "growable ring: all values retained after growth");
  }

  // ---- MemoryMetrics integration ----
  mem.report("FUZZ");

  printf("\nFUZZ_TEST: %s (%d pass, %d fail, %d anomalies)\n",
         g_fail == 0 ? "PASS" : "FAIL", g_pass, g_fail, crashes);
  return g_fail == 0 ? 0 : 1;
}
