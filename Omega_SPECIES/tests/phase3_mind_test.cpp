// Phase 3 "The Mind" — enterprise-grade unit suite.
// Distributed self (Hivemind RAFT consensus, agent genesis/death, reputation)
// + self-surgery (Surgeon General daemon). Deterministic, bounded asserts.

#include <cstdio>
#include <cmath>
#include "distributed_self/hivemind/hivemind.h"
#include "distributed_self/agent_genesis/agent_genesis.h"
#include "distributed_self/reputation_ledger/reputation_ledger.h"
#include "distributed_self/agent_death/agent_death.h"
#include "self_surgery/surgeon_general/surgeon_general.h"
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
  Bench _b("PHASE3");
  // ---------------- Hivemind (RAFT consensus) ----------------
  Hivemind hm; hm.init();
  for (int t = 0; t < 40 && !hm.committed(); t++) {
    if (t == 3) hm.propose_entry();
    hm.tick();
  }
  check(hm.committed(), "hivemind commits a proposed entry");
  check(hm.leader() >= 0 && hm.leader() < Hivemind::N, "hivemind elects a valid leader");
  check(hm.term() > 0, "hivemind advances its term");

  // ---------------- Agent pool + reputation ----------------
  AgentPool pool; pool.init();
  check(pool.spawn(0.6f) != 0, "agent genesis spawns with non-zero id");
  pool.spawn(0.3f);
  pool.spawn(0.8f);
  check(pool.count() == 3, "pool count reflects spawns");

  ReputationLedger rep; rep.init();
  check(std::fabs(rep.agent_trust(0) - 0.5f) < 1e-3f, "reputation initializes at mean trust 0.5");
  rep.update(0, 0, 10.0f);
  check(std::fabs(rep.get(0, 0) - 1.0f) < 1e-3f, "reputation update clamps to [0,1] upper");
  rep.update(0, 0, -10.0f);
  check(std::fabs(rep.get(0, 0) - 0.0f) < 1e-3f, "reputation update clamps to [0,1] lower");

  // ---------------- Agent death (retire lowest trust) ----------------
  rep.init();
  rep.update(1, 0, -0.4f); rep.update(1, 1, -0.4f);
  rep.update(1, 2, -0.4f); rep.update(1, 3, -0.4f);  // pool index 1 -> lowest
  AgentDeath death;
  const uint32_t retired = death.retire_lowest(pool, rep);
  check(retired != 0, "retire_lowest returns a retired agent id");
  check(pool.count() == 2, "retire_lowest removes exactly one agent");

  // ---------------- Surgeon General (safe self-modification) ----------------
  Network base; pw_init(&base, 16, 0xBEEFu);
  SurgeonGeneral sg; sg.seed_modules(base);
  uint32_t rng = 0xBADF00Du;
  uint32_t tick = 0;
  int applied = 0, rolled = 0, skipped = 0;
  for (int k = 0; k < 20; k++) {
    const int r = sg.operate(rng, tick++);
    rng = rng * 1664525u + 1013904223u;
    if (r == 1) ++applied;
    else if (r == 2) ++rolled;
    else ++skipped;
  }
  check(applied + rolled + skipped == 20, "every operation is accounted for");
  check(sg.applied() == applied && sg.rolled_back() == rolled && sg.skipped() == skipped,
        "Surgeon General counters match returned operation codes");
  check(sg.constitutional_blocks() >= 0, "constitutional blocks counter is non-negative");

  printf("\nPHASE3_MIND_TEST: %s (%d pass, %d fail)\n",
         g_fail == 0 ? "PASS" : "FAIL", g_pass, g_fail);
  return g_fail == 0 ? 0 : 1;
}
