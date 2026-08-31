#pragma once
#include <cstdint>

namespace omega {

// Morphological projection: maps an external agent's state vector into this
// agent's internal state space via a linear transform M. Simulates "being the
// other" by expressing their state in our coordinates.
class MorphologicalProjection {
 public:
  static constexpr int D = 8;

  void set_identity();
  void set(int i, int j, float v) { M_[i][j] = v; }
  float get(int i, int j) const { return M_[i][j]; }

  // internal = M * external
  void project(const float* external, float* internal) const;

 private:
  float M_[D][D];
};

}  // namespace omega
