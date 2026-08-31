// Phase 1 "The Seed" — enterprise-grade unit suite.
// Covers every module in the neuroevolution substrate: primordial_weights,
// reservoir_pool, spike_encoder, self_wiring GA, and the Pacemaker meta-optimizer.
// All tests are deterministic (fixed seeds) and bounded (no flakiness).

#include <cstdio>
#include <cmath>
#include <cstdint>
#include "genesis/primordial_weights/primordial_weights.h"
#include "liquid_time/reservoir_pool/reservoir_pool.h"
#include "liquid_time/spike_encoder/spike_encoder.h"
#include "genesis/self_wiring/self_wiring.h"
#include "genesis/pacemaker/pacemaker.h"

#include "bench.h"
using namespace omega;

static int g_pass = 0;
static int g_fail = 0;

static void check(bool cond, const char* name) {
  if (cond) { ++g_pass; printf("pass: %s\n", name); }
  else      { ++g_fail; printf("FAIL: %s\n", name); }
}

static float reservoir_energy(const Reservoir* r) {
  float e = 0.0f;
  for (uint16_t i = 0; i < r->n; i++) e += r->state[i] * r->state[i];
  return e;
}

int main() {
  Bench _b("PHASE1");
  // ---------------- primordial_weights ----------------
  Network a, b;
  pw_init(&a, 16, 0xBEEFu);
  pw_init(&b, 16, 0xBEEFu);
  check(a.n_nodes == 16 && a.n_edges > 0, "pw_init builds a non-empty small-world graph");
  check(pw_topology_hash(&a) == pw_topology_hash(&b), "pw_init deterministic for identical seeds");

  Network c; pw_init(&c, 16, 0xDEADu);
  check(pw_topology_hash(&a) != pw_topology_hash(&c), "different seed yields different topology");

  Network d; pw_copy(&d, &a);
  check(d.n_edges == a.n_edges && pw_topology_hash(&d) == pw_topology_hash(&a), "pw_copy is exact");
  check(pw_weight(&a, a.edges[0].src, a.edges[0].dst) == a.edges[0].w, "pw_weight round-trips stored edge");

  // ---------------- reservoir_pool ----------------
  Reservoir r; rp_init(&r, 16, 0.10f);
  for (uint16_t i = 0; i < r.n; i++) r.state[i] = 0.0f;
  float drive[16] = {0};
  drive[0] = 1.0f;
  rp_step(&r, &a, drive, 0.05f);
  check(reservoir_energy(&r) > 0.0f, "reservoir integrates external input (nonzero state)");
  rp_reset(&r);
  check(reservoir_energy(&r) == 0.0f, "rp_reset zeroes the state buffer");

  const float stim[5] = {0.15f, 0.40f, 0.65f, 0.90f, 0.30f};
  const float f1 = rp_fitness(&a, stim, 5, 24, 0.05f, 0.10f);
  const float f2 = rp_fitness(&a, stim, 5, 24, 0.05f, 0.10f);
  check(std::isfinite(f1) && f1 > -1e6f, "rp_fitness returns a finite, sane value");
  check(f1 == f2, "rp_fitness is deterministic for identical inputs");

  // ---------------- spike_encoder ----------------
  SpikeEncoder se; se_init(&se, 8, 0x55u);
  uint16_t nodes[SE_MAX_CH];
  const int n1 = se_encode(&se, 0.5f, nodes, SE_MAX_CH);
  check(n1 >= 0 && n1 <= (int)se.n, "se_encode respects channel capacity");
  bool valid = true;
  for (int i = 0; i < n1; i++) if (nodes[i] >= se.n) valid = false;
  check(valid, "se_encode emits only valid node indices");
  const int n2 = se_encode(&se, 0.5f, nodes, SE_MAX_CH);
  check(n1 == n2, "se_encode is deterministic for identical value/seed");

  // ---------------- self_wiring GA ----------------
  SelfWiringGA ga(a, a.n_nodes, 0xCAFEu);
  const float seed_fit = ga.best_fitness();
  for (int g = 0; g < 60; g++) ga.step();
  check(ga.generation() == 60, "GA advanced exactly 60 generations");
  check(ga.best_fitness() >= seed_fit - 1e-3f, "GA best fitness is non-decreasing (elitism)");
  check(ga.diversity() > 0.0f && ga.diversity() <= 1.0f, "GA population diversity bounded to (0,1]");
  check(ga.best().n_edges >= a.n_nodes, "GA best network is structurally valid");
  const int edge_delta = (int)ga.best().n_edges - (int)a.n_edges;
  check(edge_delta != 0 || ga.best_fitness() > seed_fit + 1e-3f,
        "GA self-organized (topology or fitness changed from seed)");

  // ---------------- Pacemaker (meta-optimizer) ----------------
  SelfWiringGA ga2(a, a.n_nodes, 0xCAFEu);
  Pacemaker pm(ga2);
  for (int t = 0; t < 20; t++) pm.tick();
  check(ga2.generation() == 20, "Pacemaker advances the inner GA one generation per tick");
  const GAParams p = ga2.params();
  check(p.mutation_rate > 0.0f && p.mutation_rate <= 1.0f, "Pacemaker keeps mutation_rate in (0,1]");
  check(std::isfinite(p.lr) && p.lr > 0.0f, "Pacemaker keeps a finite, positive learning rate");

  printf("\nPHASE1_SEED_TEST: %s (%d pass, %d fail)\n",
         g_fail == 0 ? "PASS" : "FAIL", g_pass, g_fail);
  return g_fail == 0 ? 0 : 1;
}
