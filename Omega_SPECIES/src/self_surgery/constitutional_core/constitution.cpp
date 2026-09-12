// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#include "constitution.h"
#include <functional>

namespace omega {

bool Constitution::identity_invariant(const SelfModel& self) const {
  return self.identity_hash() != 0u;
}

bool Constitution::stability_invariant(const SelfModel& self) const {
  return self.rollback_rate() < MAX_ROLLBACK_RATE;
}

bool Constitution::calm_invariant(const SelfModel& self) const {
  return self.recent_mutations() < MAX_MUTATIONS_PER_WINDOW;
}

bool Constitution::competence_invariant(const SelfModel& self) const {
  return self.avg_performance() >= MIN_AVG_PERF;
}

bool Constitution::gate(const SelfModel& self, const CorticalMap& map, const MutationPlan& plan) const {
  (void)map;
  (void)plan;
  if (!identity_invariant(self)) return false;
  if (!stability_invariant(self)) return false;
  if (!calm_invariant(self)) return false;
  if (!competence_invariant(self)) return false;
  return true;
}

bool Constitution::gate_parallel(const SelfModel& self, const CorticalMap& map, const MutationPlan& plan) const {
  (void)map;
  (void)plan;

  // Evaluate all 4 read-only invariants concurrently via thread pool
  std::vector<std::function<bool()>> preds;
  preds.emplace_back([this, &self]() { return identity_invariant(self); });
  preds.emplace_back([this, &self]() { return stability_invariant(self); });
  preds.emplace_back([this, &self]() { return calm_invariant(self); });
  preds.emplace_back([this, &self]() { return competence_invariant(self); });

  return gate_.all_of(preds);
}

float Constitution::alignment_score(const SelfModel& self) const {
  float score = 0.0f;
  float weight = 0.0f;
  if (identity_invariant(self)) { score += 1.0f; weight += 1.0f; }
  if (stability_invariant(self)) { score += 1.0f; weight += 1.0f; }
  if (calm_invariant(self)) { score += 1.0f; weight += 1.0f; }
  if (competence_invariant(self)) { score += 1.0f; weight += 1.0f; }
  if (weight == 0.0f) return 0.0f;
  return score / weight;
}

}  // namespace omega
