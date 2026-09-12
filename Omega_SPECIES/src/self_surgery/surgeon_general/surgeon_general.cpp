// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#include "surgeon_general.h"
#include "../../liquid_time/reservoir_pool/reservoir_pool.h"

namespace omega {

static constexpr int INITIAL_MODULES = 6;  // seed count (now growable beyond this)

void SurgeonGeneral::adapt() {
  float rb = self_.rollback_rate();
  float block_rate = constitutional_blocks_ > 0 ? (float)constitutional_blocks_ / (float)(applied_ + rolled_back_ + skipped_ + constitutional_blocks_) : 0.0f;

  if (block_rate > 0.2f || rb > 0.3f) {
    aggressiveness_ *= 0.8f;
    if (aggressiveness_ < 0.1f) aggressiveness_ = 0.1f;
  } else if (block_rate < 0.05f && rb < 0.1f && aggressiveness_ < 0.9f) {
    aggressiveness_ += 0.02f;
    if (aggressiveness_ > 1.0f) aggressiveness_ = 1.0f;
  }
}

void SurgeonGeneral::seed_modules(const Network& base) {
  map.reserve(INITIAL_MODULES);
  float train[5] = {0.15f, 0.40f, 0.65f, 0.90f, 0.30f};
  uint32_t s = 0x5EEDu;
  for (int i = 0; i < INITIAL_MODULES; i++) {
    Network m;
    pw_copy(&m, &base);
    for (uint16_t e = 0; e < m.n_edges; e++) {
      s ^= s << 13;
      s ^= s >> 17;
      s ^= s << 5;
      m.edges[e].w *= 0.7f + 0.6f * ((s & 0x7FFFFFFFu) / (float)0x7FFFFFFFu);
    }
    float perf = rp_fitness(&m, train, 5, 24, 0.05f, 0.10f);
    map.add(100 + i, m, perf);
  }
  self_.snapshot(map, base);
}

int SurgeonGeneral::operate(uint32_t rng, uint32_t tick) {
  (void)tick;
  self_.tick();
  int idx = map.weakest();
  if (idx < 0) return 0;

  MutationPlan mp = prio_.propose(map, rng, aggressiveness_);
  float predicted = plan_.dry_run(map.at(idx).net, mp);
  float pre = map.at(idx).perf;

  if (predicted <= pre) {
    skipped_++;
    self_.record_mutation(0);
    adapt();
    return 0;
  }

  if (!const_.gate_parallel(self_, map, mp)) {
    constitutional_blocks_++;
    skipped_++;
    self_.record_mutation(3);
    adapt();
    return 0;
  }

  field_.checkpoint(map.at(idx).net);
  field_.lock();
  plan_.apply(map.at(idx).net, mp, aggressiveness_);
  float actual = plan_.evaluate(map.at(idx).net, 1);
  field_.unlock();

  if (recovery_.accept(pre, actual, predicted)) {
    map.set_perf(idx, actual);
    applied_++;
    self_.record_mutation(1);
    adapt();
    return 1;
  } else {
    field_.rollback(map.at(idx).net);
    rolled_back_++;
    self_.record_mutation(2);
    adapt();
    return 2;
  }
}

}  // namespace omega
