#pragma once
#include <cstdint>
#include "12_MIRROR_NEXUS/ontology_mapping/ontology_mapping.h"

namespace omega {

// Syncretic fusion: merges two agents' ontologies into one shared lexicon.
// Concepts from A and B that are close in state space (within `thr`) are fused
// into a single shared symbol (source = both); the rest are carried over.
struct FusedConcept {
  uint32_t fused_id = 0;
  float state[8];
  uint8_t source = 0;  // 1 = only A, 2 = only B, 3 = both
};

class SyncreticFusion {
 public:
  // Fuse concept lists A and B. Writes up to max_out fused concepts.
  // Returns the number of fused concepts produced.
  int fuse(const Concept* A, int nA, const Concept* B, int nB, float thr,
           FusedConcept* out, int max_out) const;

  // How many pairs were merged (source == both).
  int merged_pairs(const FusedConcept* fused, int n) const;
};

}  // namespace omega
