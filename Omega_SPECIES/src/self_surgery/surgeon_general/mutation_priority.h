#pragma once
#include "cortical_map.h"
#include <cstdint>

namespace omega {

struct MutationPlan {
  int module_index = -1;
  int kind = 0;        // 0 = add edge, 1 = perturb weight
  uint32_t seed = 1;
};

// Decides which module to edit (the weakest) and what mutation to attempt.
class MutationPriority {
 public:
  MutationPlan propose(const CorticalMap& map, uint32_t rng, float aggressiveness = 0.5f);
};

}  // namespace omega
