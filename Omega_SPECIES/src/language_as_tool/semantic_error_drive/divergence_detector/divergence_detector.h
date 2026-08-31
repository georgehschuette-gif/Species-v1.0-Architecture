#pragma once
#include <cstdint>

namespace omega {

// Divergence detector: the same symbol used for two very different states is a
// semantic split. Records a reference state on first usage, then reports the
// divergence of later usages.
class DivergenceDetector {
 public:
  static constexpr int D = 8;
  static constexpr int N = 16;

  // Record a usage of `id` with `state`. Returns divergence magnitude (0 if
  // within tolerance or first usage).
  float record(uint32_t id, const float* state, float tolerance);

 private:
  struct Usage {
    uint32_t id = 0;
    float s[D];
    bool has = false;
  };
  Usage u_[N];
  int find(uint32_t id) const;
  static float dist(const float* a, const float* b);
};

}  // namespace omega
