#include "morphological_projection.h"

namespace omega {

void MorphologicalProjection::set_identity() {
  for (int i = 0; i < D; i++)
    for (int j = 0; j < D; j++) M_[i][j] = (i == j) ? 1.0f : 0.0f;
}

void MorphologicalProjection::project(const float* external, float* internal) const {
  for (int i = 0; i < D; i++) {
    float s = 0.0f;
    for (int j = 0; j < D; j++) s += M_[i][j] * external[j];
    internal[i] = s;
  }
}

}  // namespace omega
