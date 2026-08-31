#include "epistemic_drive.h"
#include <cmath>

namespace omega {

float information_gain(const AIModel& m, const float* predicted_state) {
  // Precision gain from one observation: sum_j pi_like * A[j][i]^2.
  float plike = 1.0f / m.sigma2;
  float ig = 0.0f;
  for (int i = 0; i < AIModel::N; i++) {
    float dpi = 0.0f;
    for (int j = 0; j < AIModel::M; j++) dpi += plike * m.A[j][i] * m.A[j][i];
    float pi_post = m.pi0[i] + dpi;
    if (pi_post > 1e-6f && m.pi0[i] > 1e-6f)
      ig += 0.5f * std::log(pi_post / m.pi0[i]);
  }

  // Novelty of the predicted state: complexity term vs prior.
  float nov = 0.0f;
  for (int i = 0; i < AIModel::N; i++) {
    float d = predicted_state[i] - m.mu0[i];
    nov += 0.5f * m.pi0[i] * d * d;
  }
  return ig * nov;  // action-dependent epistemic value
}

}  // namespace omega
