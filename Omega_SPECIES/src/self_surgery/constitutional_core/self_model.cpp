// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#include "self_model.h"

namespace omega {

void SelfModel::snapshot(const CorticalMap& map, const Network& identity_net) {
  perf_sum_ = 0.0f;
  perf_count_ = 0;
  for (int i = 0; i < map.count(); i++) {
    perf_sum_ += map.at(i).perf;
    perf_count_++;
  }
  identity_hash_ = pw_topology_hash(&identity_net);
}

void SelfModel::record_mutation(int kind) {
  if (kind == 1) applied_++;
  else if (kind == 2) rolled_back_++;
  else if (kind == 3) skipped_++;
  recent_.push((uint8_t)kind);
}

float SelfModel::avg_performance() const {
  if (perf_count_ == 0) return 0.0f;
  return perf_sum_ / (float)perf_count_;
}

float SelfModel::rollback_rate() const {
  int total = applied_ + rolled_back_;
  if (total == 0) return 0.0f;
  return (float)rolled_back_ / (float)total;
}

int SelfModel::recent_mutations() const {
  return (int)recent_.size();
}

float SelfModel::predict_risk(const MutationPlan& plan) const {
  (void)plan;
  float risk = 0.0f;
  if (rollback_rate() > 0.3f) risk += 0.4f;
  if (recent_.size() >= WINDOW) {
    int recent_rollbacks = 0;
    for (size_t i = 0; i < recent_.size(); i++)
      if (recent_.at(i) == 2) recent_rollbacks++;
    if (recent_rollbacks > (int)(recent_.size() / 2)) risk += 0.3f;
  }
  return risk;
}

}  // namespace omega
