// Phase 4 "Soul -> proxies" — enterprise-grade unit suite.
// Coherence preservation (identity integrity), novelty harvesting (1 pattern /
// N ticks, caloric-gated), temporal error drive (prediction mismatch), and
// pre/post-consolidation (planned vs actual outcome). Deterministic, bounded.

#include <cstdio>
#include <cmath>
#include "prime_directive/coherence_preservation/coherence_preservation.h"
#include "prime_directive/novelty_harvesting/novelty_harvesting.h"
#include "temporal_folding/temporal_error_drive/temporal_error_drive.h"
#include "temporal_folding/pre_consolidation/pre_consolidation.h"
#include "temporal_folding/post_consolidation/post_consolidation.h"
#include "immune_system/self_antigen/self_antigen.h"

#include "bench.h"
using namespace omega;

static int g_pass = 0;
static int g_fail = 0;

static void check(bool cond, const char* name) {
  if (cond) { ++g_pass; printf("pass: %s\n", name); }
  else      { ++g_fail; printf("FAIL: %s\n", name); }
}

int main() {
  Bench _b("PHASE4");
  // ---------------- coherence preservation (identity integrity) ----------------
  uint8_t g[4] = {1, 2, 3, 4};
  uint8_t g2[4] = {9, 9, 9, 9};
  float sA[8] = {0.10f, 0.20f, 0.30f, 0.40f, 0.50f, 0.60f, 0.70f, 0.80f};
  float sB[8] = {0.91f, 0.82f, 0.73f, 0.64f, 0.55f, 0.46f, 0.37f, 0.28f};

  SelfAntigen ref; ref.compute_genotype(g, 4); ref.project_phenotype(sA, 8);
  CoherencePreservation coh;
  check(!coh.verify(ref), "coherence: unverifiable before reference set");

  coh.set_reference(ref);
  SelfAntigen cur; cur.compute_genotype(g, 4); cur.project_phenotype(sB, 8);
  check(coh.verify(cur), "coherence: verifies same-genotype antigen");
  check(std::fabs(coh.coherence_score(cur) - 1.0f) < 1e-3f, "coherence_score 1.0 for self");
  check(coh.phenotype_drift(cur) > 0, "phenotype_drift > 0 for divergent state");

  SelfAntigen other; other.compute_genotype(g2, 4);
  check(!coh.verify(other), "coherence: rejects non-self genotype");
  check(coh.coherence_score(other) == 0.0f, "coherence_score 0.0 on genotype breach");

  // ---------------- novelty harvesting (caloric-gated) ----------------
  NoveltyHarvesting nov; nov.init(10, 100, 0.05f);
  for (uint32_t t = 0; t < 50; t++) {
    float novelty = 0.01f + ((t % 5 == 0) ? 0.5f : 0.0f);
    nov.tick(t, novelty, 200);  // ample calories
  }
  check(nov.harvested() > 0, "novelty harvesting emits >=1 pattern when funded");

  NoveltyHarvesting nov2; nov2.init(5, 100, 0.01f);
  for (uint32_t t = 0; t < 20; t++) nov2.tick(t, 0.5f, 10);  // under budget
  check(nov2.harvested() == 0, "novelty harvesting gated by caloric budget");

  // ---------------- temporal error drive ----------------
  TemporalErrorDrive ted;
  check(std::fabs(ted.error(1.0f, 0.6f) - 0.4f) < 1e-3f, "ted.error is absolute delta");
  check(std::fabs(ted.severity(1.0f, 0.6f, 0.15f) - 0.25f) < 1e-3f, "ted.severity above tolerance");
  check(ted.severity(1.0f, 0.95f, 0.15f) == 0.0f, "ted.severity zero within tolerance");
  check(ted.triggers_immune(1.0f, 0.6f, 0.15f), "ted triggers immune when severe");
  check(!ted.triggers_immune(1.0f, 0.95f, 0.15f), "ted no immune trigger within tolerance");

  // ---------------- pre/post-consolidation (planned vs actual) ----------------
  PreConsolidation pre;
  pre.plan(1000, 2, 0.7f);
  pre.plan(1010, 1, 0.4f);
  check(pre.count() == 2, "pre-consolidation stores future memories");
  const FutureMemory* m = pre.at(0);
  check(m && m->tick == 1000 && m->action == 2, "future memory fields stored correctly");

  PostConsolidation post;
  FutureMemory plan; plan.tick = 1000; plan.action = 2; plan.predicted = 0.7f;
  Outcome out; out.tick = 1000; out.action = 2; out.actual = 0.7f;
  check(std::fabs(post.delta(plan, out)) < 1e-3f, "post delta zero on exact match");
  check(std::fabs(post.compare(plan, out) - 1.0f) < 1e-3f, "post compare 1.0 on exact match");
  out.actual = 0.2f;
  const float c = post.compare(plan, out);
  check(c >= 0.0f && c <= 1.0f, "post compare bounded to [0,1]");
  check(c < 1.0f, "post compare < 1.0 on mismatch");

  printf("\nPHASE4_SOUL_TEST: %s (%d pass, %d fail)\n",
         g_fail == 0 ? "PASS" : "FAIL", g_pass, g_fail);
  return g_fail == 0 ? 0 : 1;
}
