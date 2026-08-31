// Phase 12 — Dream: Offline Memory Replay and Recombination.
// Validates that the agent can ingest future memories, replay them in
// deterministic shuffled order, recombine pairs into novel sequences,
// and emit mutation seeds for self-modification.

#include <cstdio>
#include <cmath>
#include <cstring>
#include "temporal_folding/pre_consolidation/pre_consolidation.h"
#include "temporal_folding/post_consolidation/post_consolidation.h"
#include "temporal_folding/dream/dream.h"

#include "bench.h"
using namespace omega;

static int g_pass = 0, g_fail = 0;
static void check(bool c, const char* n) {
  if (c) { ++g_pass; printf("pass: %s\n", n); }
  else   { ++g_fail; printf("FAIL: %s\n", n); }
}

int main() {
  Bench _b("PHASE12");

  // 1) Empty dream returns RNG unchanged.
  {
    Dream d;
    uint32_t rng = 0xABCDu;
    uint32_t out = d.replay(rng);
    check(out == rng, "dream: replay on empty memory returns unchanged RNG");
    check(d.dream_count() == 1, "dream: dream_count increments even on empty replay");
  }

  // 2) Ingest stores memories.
  {
    Dream d;
    PreConsolidation pre;
    pre.plan(100, 1, 0.5f);
    pre.plan(200, 2, 0.7f);
    pre.plan(300, 3, 0.9f);
    PostConsolidation post;
    Outcome outcomes[3];
    outcomes[0] = {100, 1, 0.6f};
    outcomes[1] = {200, 2, 0.8f};
    outcomes[2] = {300, 3, 1.0f};
    d.ingest(pre, post, outcomes, 3);
    check(d.last_sequence().len == 0, "dream: last_sequence empty before replay");
  }

  // 3) Replay produces valid sequence of bounded length.
  {
    Dream d;
    PreConsolidation pre;
    pre.plan(100, 1, 0.5f);
    pre.plan(200, 2, 0.7f);
    pre.plan(300, 3, 0.9f);
    PostConsolidation post;
    Outcome outcomes[3];
    outcomes[0] = {100, 1, 0.6f};
    outcomes[1] = {200, 2, 0.8f};
    outcomes[2] = {300, 3, 1.0f};
    d.ingest(pre, post, outcomes, 3);
    uint32_t seed = d.replay(0x1234u);
    const Dream::DreamSequence& seq = d.last_sequence();
    check(seq.len == 2, "dream: recombined sequence length is RECOMBINE_K=2");
    check(seq.len <= Dream::CAP, "dream: sequence length bounded by CAP");
    check(seed != 0, "dream: mutation seed is non-zero");
  }

  // 4) Deterministic replay for identical RNG and memories.
  {
    Dream d1, d2;
    PreConsolidation pre1, pre2;
    pre1.plan(100, 1, 0.5f); pre1.plan(200, 2, 0.7f);
    pre2.plan(100, 1, 0.5f); pre2.plan(200, 2, 0.7f);
    PostConsolidation post1, post2;
    Outcome o1[2] = {{100,1,0.6f},{200,2,0.8f}};
    Outcome o2[2] = {{100,1,0.6f},{200,2,0.8f}};
    d1.ingest(pre1, post1, o1, 2);
    d2.ingest(pre2, post2, o2, 2);
    uint32_t s1 = d1.replay(0xABCDu);
    uint32_t s2 = d2.replay(0xABCDu);
    check(s1 == s2, "dream: identical inputs yield identical mutation seeds");
  }

  // 5) Multiple dreams increment counter and recombinations.
  {
    Dream d;
    PreConsolidation pre;
    pre.plan(100, 1, 0.5f);
    PostConsolidation post;
    Outcome o[1] = {{100,1,0.6f}};
    d.ingest(pre, post, o, 1);
    d.replay(0x1u);
    d.replay(0x2u);
    d.replay(0x3u);
    check(d.dream_count() == 3, "dream: dream_count tracks multiple replays");
    check(d.recombinations() == 3, "dream: recombinations tracks multiple replays");
  }

  // 6) Recombination with single memory falls back to length 1.
  {
    Dream d;
    PreConsolidation pre;
    pre.plan(500, 7, 0.3f);
    PostConsolidation post;
    Outcome o[1] = {{500,7,0.4f}};
    d.ingest(pre, post, o, 1);
    d.replay(0x55u);
    const Dream::DreamSequence& seq = d.last_sequence();
    check(seq.len == 1, "dream: single memory yields sequence length 1");
    check(seq.actions[0] == 7, "dream: action preserved in sequence");
    check(seq.ticks[0] == 500, "dream: tick preserved in sequence");
  }

  // 7) Dream sequence values are within expected ranges.
  {
    Dream d;
    PreConsolidation pre;
    for (int i = 0; i < 5; i++) pre.plan(1000 + i * 100, i + 10, 0.1f * (i + 1));
    PostConsolidation post;
    Outcome o[5];
    for (int i = 0; i < 5; i++) o[i] = {1000 + i * 100, i + 10, 0.1f * (i + 1) + 0.05f};
    d.ingest(pre, post, o, 5);
    d.replay(0xDEADu);
    const Dream::DreamSequence& seq = d.last_sequence();
    for (int i = 0; i < seq.len; i++) {
      check(seq.actions[i] >= 10 && seq.actions[i] <= 14,
            "dream: recombined action within original range");
      check(seq.ticks[i] >= 1000 && seq.ticks[i] <= 1400,
            "dream: recombined tick within original range");
    }
  }

  printf("\nPHASE12_DREAM_TEST: %s (%d pass, %d fail)\n",
         g_fail == 0 ? "PASS" : "FAIL", g_pass, g_fail);
  return g_fail == 0 ? 0 : 1;
}
