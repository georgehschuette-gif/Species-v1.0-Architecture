#pragma once
#include "../lsh/memory_store.h"

namespace omega {

// Memory T-cells: active hypotheses. Faster decay, lower consolidation threshold
// — short-lived until promoted to B-cells.
class MemoryTCells {
 public:
  MemoryStore store;
  void init(const LSH* l) { store.init(l, /*decay/step=*/0.02f, /*consolidate=*/3.0f); }
};

}  // namespace omega
