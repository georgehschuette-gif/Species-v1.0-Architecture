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
  recent_[recent_pos_] = (uint8_t)kind;
  recent_pos_ = (recent_pos_ + 1) % WINDOW;
  if (recent_count_ < WINDOW) recent_count_++;
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
  int count = 0;
  for (int i = 0; i < WINDOW; i++) if (recent_[i] != 0) count++;
  return count;
}

float SelfModel::predict_risk(const MutationPlan& plan) const {
  (void)plan;
  float risk = 0.0f;
  if (rollback_rate() > 0.3f) risk += 0.4f;
  if (recent_count_ >= WINDOW) {
    int recent_rollbacks = 0;
    for (int i = 0; i < WINDOW; i++)
      if (recent_[i] == 2) recent_rollbacks++;
    if (recent_rollbacks > WINDOW / 2) risk += 0.3f;
  }
  return risk;
}

}  // namespace omega
