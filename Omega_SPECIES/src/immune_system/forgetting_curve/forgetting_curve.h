#pragma once
#include "../lsh/memory_store.h"

namespace omega {

// Surprise-based pruning. Each step decays all memories and drops the weakest,
// so unrefreshed (non-surprising) content is forgotten.
class ForgettingCurve {
 public:
  int apply(MemoryStore& s, float dt, float min_strength) {
    s.decay_all(dt);
    return s.prune(min_strength);
  }
};

}  // namespace omega
