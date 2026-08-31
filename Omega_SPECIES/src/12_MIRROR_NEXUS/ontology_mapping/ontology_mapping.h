#pragma once
#include <cstdint>

namespace omega {

// Ontology mapping: relates one agent's grounded concepts to another's by
// matching 8-dim state vectors. A "concept" is a symbol id plus its state.
struct Concept {
  uint32_t id = 0;
  float state[8];
};

class OntologyMapping {
 public:
  static constexpr int D = 8;

  // Find the remote concept nearest `state`. Returns true on success; writes
  // the matched remote id and Euclidean distance. Returns false if remote empty.
  bool map_nearest(const float* state, const Concept* remote, int n_remote,
                   uint32_t* out_id, float* out_dist) const;

  // Build a full mapping: for each local concept, record the nearest remote id
  // and distance. Returns the number of local concepts that found a match.
  int build_map(const Concept* local, int n_local, const Concept* remote,
                int n_remote, uint32_t* out_local_id, uint32_t* out_remote_id,
                float* out_dist) const;

  static float dist(const float* a, const float* b);
};

}  // namespace omega
