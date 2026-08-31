#include "policy_selection.h"
#include "../epistemic_drive/epistemic_drive.h"

namespace omega {

static void policy_dynamics(const AIModel&, const float* x, int a, float* xp) {
  for (int i = 0; i < AIModel::N; i++) {
    float delta = (a % AIModel::N == i) ? 0.30f : -0.08f;
    xp[i] = x[i] + delta;
  }
}

PolicyResult select_policy(const AIModel& m, const float* x, int n_actions,
                           float ew) {
  PolicyResult best;
  float xp[AIModel::N];
  float target[AIModel::M];

  for (int j = 0; j < AIModel::M; j++) {
    float t = 0.0f;
    for (int i = 0; i < AIModel::N; i++) t += m.A[j][i] * m.mu0[i];
    target[j] = t;  // preferred observation = prior prediction
  }

  for (int a = 0; a < n_actions; a++) {
    policy_dynamics(m, x, a, xp);
    float risk = 0.0f;
    for (int j = 0; j < AIModel::M; j++) {
      float yp = 0.0f;
      for (int i = 0; i < AIModel::N; i++) yp += m.A[j][i] * xp[i];
      float e = yp - target[j];
      risk += e * e;
    }
    risk *= 0.5f / m.sigma2;

    float epistemic = information_gain(m, xp);
    float efe = risk - ew * epistemic;

    if (efe < best.efe) {
      best.action = a;
      best.efe = efe;
      best.risk = risk;
      best.epistemic = epistemic;
    }
  }
  return best;
}

}  // namespace omega
