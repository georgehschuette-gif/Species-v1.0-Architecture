#include "divergence_detector.h"
#include <cmath>
#include <cstring>

namespace omega {

float DivergenceDetector::dist(const float* a, const float* b) {
  float s = 0.0f;
  for (int i = 0; i < D; i++) {
    float d = a[i] - b[i];
    s += d * d;
  }
  return std::sqrt(s);
}

int DivergenceDetector::find(uint32_t id) const {
  for (int i = 0; i < N; i++)
    if (u_[i].has && u_[i].id == id) return i;
  return -1;
}

float DivergenceDetector::record(uint32_t id, const float* state, float tolerance) {
  int i = find(id);
  if (i < 0) {
    int k = 0;
    while (k < N && u_[k].has) k++;
    if (k >= N) k = 0;
    u_[k].id = id;
    memcpy(u_[k].s, state, sizeof(float) * D);
    u_[k].has = true;
    return 0.0f;
  }
  float d = dist(u_[i].s, state);
  if (d > tolerance) {
    memcpy(u_[i].s, state, sizeof(float) * D);  // track drift
    return d - tolerance;
  }
  return 0.0f;
}

}  // namespace omega
