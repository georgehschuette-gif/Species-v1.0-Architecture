// Phase 2 "The Body" — enterprise-grade unit suite.
// Active inference (free-energy / perception) + immune system
// (64-byte self-antigen identity + inflammation priority queue).
// Deterministic, bounded assertions — gold-standard gate.

#include <cstdio>
#include <cmath>
#include <cstring>
#include "active_inference/free_energy/free_energy.h"
#include "immune_system/self_antigen/self_antigen.h"
#include "immune_system/inflammation_response/inflammation_response.h"

#include "bench.h"
using namespace omega;

static int g_pass = 0;
static int g_fail = 0;

static void check(bool cond, const char* name) {
  if (cond) { ++g_pass; printf("pass: %s\n", name); }
  else      { ++g_fail; printf("FAIL: %s\n", name); }
}

int main() {
  Bench _b("PHASE2");
  // ---------------- active inference: free energy ----------------
  AIModel m;
  for (int j = 0; j < AIModel::M; j++)
    for (int i = 0; i < AIModel::N; i++)
      m.A[j][i] = 0.10f * (j + 1) + 0.05f * i;
  m.sigma2 = 0.5f;
  for (int i = 0; i < AIModel::N; i++) { m.mu0[i] = 0.0f; m.pi0[i] = 1.0f; }

  float x[4] = {0.5f, -0.3f, 0.2f, 0.1f};
  float y[3];
  for (int j = 0; j < AIModel::M; j++) {
    y[j] = 0.0f;
    for (int i = 0; i < AIModel::N; i++) y[j] += m.A[j][i] * x[i];
  }

  const float F0 = free_energy(m, x, y);
  check(F0 >= 0.0f, "free_energy is non-negative");

  float x2[4];
  for (int i = 0; i < AIModel::N; i++) x2[i] = x[i];
  perceive(m, x2, y, 40, 0.05f);
  const float F1 = free_energy(m, x2, y);
  check(F1 <= F0 + 1e-2f, "perceive reduces or holds free energy (coordinate descent)");

  // ---------------- immune system: self-antigen (64-byte identity) ----------------
  uint8_t g1[4] = {1, 2, 3, 4};
  uint8_t g2[4] = {9, 9, 9, 9};

  SelfAntigen a; a.compute_genotype(g1, 4);
  SelfAntigen b; b.compute_genotype(g1, 4);
  check(a.is_self(b), "identical genotype recognized as self");

  SelfAntigen c; c.compute_genotype(g2, 4);
  check(!a.is_self(c), "different genotype recognized as non-self");

  float s1[8] = {0.10f, 0.20f, 0.30f, 0.40f, 0.50f, 0.60f, 0.70f, 0.80f};
  float s2[8] = {0.91f, 0.82f, 0.73f, 0.64f, 0.55f, 0.46f, 0.37f, 0.28f};
  a.project_phenotype(s1, 8);

  SelfAntigen a2; a2.compute_genotype(g1, 4); a2.project_phenotype(s1, 8);
  check(a2.phenotype_drift(a) == 0, "identical phenotype projection -> zero drift");

  a2.project_phenotype(s2, 8);
  const int drift = a2.phenotype_drift(a);
  check(drift > 0, "divergent phenotype projection -> measurable drift (>0 bytes)");

  char h1[130], h2[130];
  a.to_hex(h1, 130);
  a2.to_hex(h2, 130);
  check(std::strlen(h1) == 128 && std::strcmp(h1, h2) != 0,
        "to_hex emits stable 128-char identity; distinct states differ");

  // ---------------- immune system: inflammation (priority queue) ----------------
  Inflammation inf;
  check(inf.pending() == 0, "inflammation queue starts empty");
  check(inf.trigger(0.3f, 1, 10), "trigger accepts task");
  check(inf.trigger(0.9f, 2, 30), "trigger accepts higher-severity task");
  check(inf.trigger(0.6f, 3, 20), "trigger accepts third task");
  check(inf.pending() == 3, "pending count is accurate");
  check(inf.total_allocated() == 60, "total_allocated sums compute budgets");
  check(std::fabs(inf.total_severity() - 1.8f) < 1e-3f, "total_severity sums queued severities");

  InflammationTask t;
  check(inf.pop(t) && t.severity == 0.9f, "pop returns highest-severity task first (max-heap)");
  check(inf.pop(t) && t.severity == 0.6f, "pop returns next-highest severity");

  Inflammation inf2;
  for (int i = 0; i < 20; i++) inf2.trigger(0.1f, (uint32_t)i, 1);
  check(inf2.pending() == Inflammation::CAP, "trigger rejects beyond CAP (16)");

  printf("\nPHASE2_BODY_TEST: %s (%d pass, %d fail)\n",
         g_fail == 0 ? "PASS" : "FAIL", g_pass, g_fail);
  return g_fail == 0 ? 0 : 1;
}
