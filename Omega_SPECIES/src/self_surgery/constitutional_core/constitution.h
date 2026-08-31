// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#pragma once
#include <cstdint>
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

 private:
  bool identity_invariant(const SelfModel& self) const;
  bool stability_invariant(const SelfModel& self) const;
  bool calm_invariant(const SelfModel& self) const;
  bool competence_invariant(const SelfModel& self) const;
};

}  // namespace omega
