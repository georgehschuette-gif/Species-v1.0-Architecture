// Phase 10 — Constitutional Core (SelfModel + Constitution + SelfReport).
// Validates the agent's introspective self-model and its non-negotiable invariants.

#include <cstdio>
#include <cstring>
#include <cmath>
#include "self_surgery/surgeon_general/surgeon_general.h"
#include "self_surgery/constitutional_core/self_report.h"
#include "genesis/primordial_weights/primordial_weights.h"

#include "bench.h"
using namespace omega;

static int g_pass = 0;
static int g_fail = 0;

static void check(bool cond, const char* name) {
  if (cond) { ++g_pass; printf("pass: %s\n", name); }
  else      { ++g_fail; printf("FAIL: %s\n", name); }
}

int main() {
  Bench _b("PHASE10");

  Network base; pw_init(&base, 16, 0xBEEFu);
  SurgeonGeneral sg;
  sg.seed_modules(base);

  check(sg.self_model().identity_hash() != 0u, "self_model has non-zero identity hash after seeding");
  check(sg.self_model().tick_count() == 0, "self_model tick starts at zero");
  check(sg.self_model().recent_mutations() == 0, "self_model recent mutations starts at zero");
  check(sg.self_model().avg_performance() > 0.0f, "self_model avg_performance positive after seeding");
  check(sg.self_model().rollback_rate() == 0.0f, "self_model rollback_rate starts at zero");

  check(sg.constitution().alignment_score(sg.self_model()) == 1.0f,
        "constitution alignment is 1.0 for a fresh, healthy self-model");

  check(sg.constitutional_blocks() == 0, "constitutional_blocks starts at zero");

  uint32_t rng = 0x12345u;
  uint32_t tick = 0;
  for (int k = 0; k < 20; k++) {
    sg.operate(rng, tick++);
    rng = rng * 1664525u + 1013904223u;
  }
  check(sg.self_model().tick_count() == 20, "self_model tick advances with each operate call");
  check(sg.self_model().recent_mutations() > 0, "self_model recent mutations positive after 20 ops");
  int total = sg.applied() + sg.rolled_back() + sg.skipped();
  check(total == 20, "all operate calls are recorded");
  check(sg.constitutional_blocks() >= 0, "constitutional_blocks is non-negative");

  {
    char buf[128];
    SelfReport::introspect(sg, buf, sizeof(buf));
    check(strlen(buf) > 0, "introspect produces non-empty output");
    check(strstr(buf, "SELF:") != nullptr, "introspect contains SELF prefix");
    check(strstr(buf, "identity=0x") != nullptr, "introspect contains identity hash");
  }

  {
    char buf[128];
    SelfReport::constitutional_state(sg.constitution(), sg.self_model(), buf, sizeof(buf));
    check(strlen(buf) > 0, "constitutional_state produces non-empty output");
    check(strstr(buf, "CONST:") != nullptr, "constitutional_state contains CONST prefix");
    check(strstr(buf, "alignment=") != nullptr, "constitutional_state contains alignment score");
    check(strstr(buf, "[") != nullptr && strstr(buf, "]") != nullptr,
          "constitutional_state contains invariant bracket notation");
  }

  printf("\nPHASE10_CONSTITUTION_TEST: %s (%d pass, %d fail)\n",
         g_fail == 0 ? "PASS" : "FAIL", g_pass, g_fail);
  return g_fail == 0 ? 0 : 1;
}
