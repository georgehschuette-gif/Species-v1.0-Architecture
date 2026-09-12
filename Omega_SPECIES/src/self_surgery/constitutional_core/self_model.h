// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#pragma once
#include <cstdint>
#include "core/scale.h"
#include "../surgeon_general/cortical_map.h"
#include "../surgeon_general/mutation_priority.h"

namespace omega {

// Growable self-model with dynamic ring buffer for mutation tracking.
// WINDOW replaced with GrowableRing<uint8_t> — no hard ceiling.
class SelfModel {
 public:
  static constexpr int WINDOW = 16;  // initial capacity (now growable)

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
  core::GrowableRing<uint8_t> recent_{WINDOW};
};

}  // namespace omega
