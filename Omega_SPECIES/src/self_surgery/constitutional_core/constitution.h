// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#pragma once
#include <cstdint>
#include <vector>
#include <future>
#include "core/scale.h"
#include "../surgeon_general/cortical_map.h"
#include "../surgeon_general/mutation_priority.h"
#include "self_model.h"

namespace omega {

class Constitution {
 public:
  static constexpr float MAX_ROLLBACK_RATE = 0.5f;
  static constexpr float MIN_AVG_PERF = 0.05f;
  static constexpr int MAX_MUTATIONS_PER_WINDOW = 8;

  bool gate(const SelfModel& self, const CorticalMap& map, const MutationPlan& plan) const;
  float alignment_score(const SelfModel& self) const;

  // Parallel gate: evaluates all invariants concurrently via thread pool.
  // All invariants are read-only on SelfModel, making them trivially parallelizable.
  bool gate_parallel(const SelfModel& self, const CorticalMap& map, const MutationPlan& plan) const;

  size_t worker_count() const { return gate_.workers(); }

 private:
  bool identity_invariant(const SelfModel& self) const;
  bool stability_invariant(const SelfModel& self) const;
  bool calm_invariant(const SelfModel& self) const;
  bool competence_invariant(const SelfModel& self) const;

  core::ParallelGate gate_;
};

}  // namespace omega
