#include "mutation_priority.h"

namespace omega {

MutationPlan MutationPriority::propose(const CorticalMap& map, uint32_t rng, float aggressiveness) {
  MutationPlan mp;
  mp.module_index = map.weakest();
  uint32_t s = rng ? rng : 1u;
  s ^= s << 13;
  s ^= s >> 17;
  s ^= s << 5;
  float threshold = 0.3f + 0.7f * aggressiveness;
  mp.kind = (s & 0x7FFFFFFFu) / (float)0x7FFFFFFFu < threshold ? 0 : 1;
  mp.seed = s ? s : 1u;
  return mp;
}

}  // namespace omega
