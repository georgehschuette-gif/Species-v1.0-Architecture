#include "free_energy.h"
#include <cmath>

namespace omega {

float free_energy(const AIModel& m, const float* x, const float* y) {
  for (int i = 0; i < AIModel::N; i++)
    if (!std::isfinite(x[i])) return 1e18f;
  for (int j = 0; j < AIModel::M; j++)
    if (!std::isfinite(y[j])) return 1e18f;

  float acc = 0.0f;
  for (int j = 0; j < AIModel::M; j++) {
    float yh = 0.0f;
    for (int i = 0; i < AIModel::N; i++) yh += m.A[j][i] * x[i];
    float e = y[j] - yh;
    if (!std::isfinite(e)) return 1e18f;
    acc += e * e;
  }
  acc /= (2.0f * m.sigma2);

  float cx = 0.0f;
  for (int i = 0; i < AIModel::N; i++) {
    float d = x[i] - m.mu0[i];
    if (!std::isfinite(d)) return 1e18f;
    cx += 0.5f * m.pi0[i] * d * d;
  }
  return acc + cx;
}

void perceive(AIModel& m, float* x, const float* y, int iters, float step) {
  for (int it = 0; it < iters; it++) {
    for (int i = 0; i < AIModel::N; i++) {
      float base = free_energy(m, x, y);
      float old = x[i];
      x[i] = old + step;
      float up = free_energy(m, x, y);
      x[i] = old - step;
      float dn = free_energy(m, x, y);
      if (up < base && up <= dn)
        x[i] = old + step;
      else if (dn < base)
        x[i] = old - step;
      else
        x[i] = old;
    }
  }
}

}  // namespace omega
