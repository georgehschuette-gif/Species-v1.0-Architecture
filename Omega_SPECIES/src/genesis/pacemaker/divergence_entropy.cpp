#include "divergence_entropy.h"
#include <cmath>

namespace omega {

float DivergenceEntropy::entropy_of(const Network& net) const {
  if (net.n_edges == 0) return 0.0f;
  int hist[BINS];
  for (int i = 0; i < BINS; i++) hist[i] = 0;
  const float lo = -3.0f, hi = 3.0f;
  const float span = hi - lo;
  for (uint16_t i = 0; i < net.n_edges; i++) {
    float v = net.edges[i].w;
    if (v < lo) v = lo;
    if (v > hi) v = hi;
    int b = (int)((v - lo) / span * BINS);
    if (b >= BINS) b = BINS - 1;
    hist[b]++;
  }
  float h = 0.0f;
  for (int i = 0; i < BINS; i++) {
    if (hist[i] == 0) continue;
    float p = (float)hist[i] / (float)net.n_edges;
    h -= p * logf(p);
  }
  float maxh = logf((float)BINS);
  return maxh > 0.0f ? h / maxh : 0.0f;
}

float DivergenceEntropy::adjust(float diversity) const {
  if (diversity < 0.12f) return 2.0f;  // collapsing -> boost exploration
  if (diversity > 0.55f) return 0.5f;  // scattering -> tighten
  return 1.0f;
}

}  // namespace omega
