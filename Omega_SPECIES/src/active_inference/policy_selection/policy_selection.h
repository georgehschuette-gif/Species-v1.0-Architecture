#pragma once
#include "../free_energy/free_energy.h"

namespace omega {

struct PolicyResult {
  int action = -1;
  float efe = 1e30f;       // expected free energy (minimized)
  float risk = 0.0f;       // expected prediction error
  float epistemic = 0.0f;  // expected information gain
};

// Evaluate n_actions policies and pick the one minimizing expected free
// energy: EFE = risk - epistemic_weight * information_gain.
PolicyResult select_policy(const AIModel& m, const float* x, int n_actions,
                           float epistemic_weight = 1.0f);

}  // namespace omega
