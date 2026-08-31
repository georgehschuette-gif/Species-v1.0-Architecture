#include "ontology_mapping.h"
#include <cmath>
#include <cfloat>

namespace omega {

float OntologyMapping::dist(const float* a, const float* b) {
  float s = 0.0f;
  for (int i = 0; i < D; i++) {
    float d = a[i] - b[i];
    s += d * d;
  }
  return std::sqrt(s);
}

bool OntologyMapping::map_nearest(const float* state, const Concept* remote,
                                  int n_remote, uint32_t* out_id,
                                  float* out_dist) const {
  if (n_remote <= 0) return false;
  float best = FLT_MAX;
  uint32_t best_id = 0;
  for (int i = 0; i < n_remote; i++) {
    float d = dist(state, remote[i].state);
    if (d < best) { best = d; best_id = remote[i].id; }
  }
  if (out_id) *out_id = best_id;
  if (out_dist) *out_dist = best;
  return true;
}

int OntologyMapping::build_map(const Concept* local, int n_local,
                               const Concept* remote, int n_remote,
                               uint32_t* out_local_id, uint32_t* out_remote_id,
                               float* out_dist) const {
  int matched = 0;
  for (int i = 0; i < n_local; i++) {
    uint32_t rid = 0;
    float d = 0.0f;
    if (map_nearest(local[i].state, remote, n_remote, &rid, &d)) {
      if (out_local_id) out_local_id[matched] = local[i].id;
      if (out_remote_id) out_remote_id[matched] = rid;
      if (out_dist) out_dist[matched] = d;
      matched++;
    }
  }
  return matched;
}

}  // namespace omega
