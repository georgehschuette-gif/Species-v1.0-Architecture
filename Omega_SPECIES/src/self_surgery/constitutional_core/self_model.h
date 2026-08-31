// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#pragma once
#include <cstdint>
#include "../surgeon_general/cortical_map.h"
#include "../surgeon_general/mutation_priority.h"

namespace omega {

class SelfModel {
 public:
  static constexpr int WINDOW = 16;

  void snapshot(const CorticalMap& map, const Network& identity_net);
  void record_mutation(int kind);
  void tick() { tick_++; }

  float avg_performance() const;
  float rollback_rate() const;
  uint32_t identity_hash() const { return identity_hash_; }
  int recent_mutations() const;
  uint32_t tick_count() const { return tick_; }
  float predict_risk(const MutationPlan& plan) const;

 private:
  float perf_sum_ = 0.0f;
  int perf_count_ = 0;
  int applied_ = 0;
  int rolled_back_ = 0;
  int skipped_ = 0;
  uint32_t identity_hash_ = 0;
  uint32_t tick_ = 0;
  uint8_t recent_[WINDOW] = {0};
  int recent_pos_ = 0;
  int recent_count_ = 0;
};

}  // namespace omega
