#pragma once
#include "../lsh/memory_store.h"

namespace omega {

// Memory B-cells: long-term consolidated memory. Slow decay, high consolidation
// threshold — once written, it persists.
class MemoryBCells {
 public:
  MemoryStore store;
  void init(const LSH* l) { store.init(l, /*decay/step=*/0.002f, /*consolidate=*/5.0f); }
};

}  // namespace omega
