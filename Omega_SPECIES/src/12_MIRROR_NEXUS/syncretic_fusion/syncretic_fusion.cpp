#include "syncretic_fusion.h"
#include <cstring>

namespace omega {

int SyncreticFusion::fuse(const Concept* A, int nA, const Concept* B, int nB,
                          float thr, FusedConcept* out, int max_out) const {
  int n = 0;
  // Seed the fused lexicon with A's concepts.
  for (int i = 0; i < nA && n < max_out; i++) {
    FusedConcept& f = out[n++];
    f.fused_id = A[i].id;
    std::memcpy(f.state, A[i].state, sizeof(float) * 8);
    f.source = 1;
  }
  // Merge B: match each B concept against the current fused lexicon.
  for (int i = 0; i < nB; i++) {
    float best = thr;
    int best_j = -1;
    for (int j = 0; j < n; j++) {
      float d = OntologyMapping::dist(B[i].state, out[j].state);
      if (d <= best) { best = d; best_j = j; }
    }
    if (best_j >= 0) {
      // Fuse: mark shared and average the two states.
      FusedConcept& f = out[best_j];
      for (int k = 0; k < 8; k++) f.state[k] = 0.5f * (f.state[k] + B[i].state[k]);
      f.source = 3;
    } else if (n < max_out) {
      FusedConcept& f = out[n++];
      f.fused_id = B[i].id;
      std::memcpy(f.state, B[i].state, sizeof(float) * 8);
      f.source = 2;
    }
  }
  return n;
}

int SyncreticFusion::merged_pairs(const FusedConcept* fused, int n) const {
  int c = 0;
  for (int i = 0; i < n; i++) if (fused[i].source == 3) c++;
  return c;
}

}  // namespace omega
