#include "tensorless.h"

namespace omega {

void TensorlessEngine::euler_step(uint32_t key, float dt, float drive) {
  float v = get(key);
  float deriv = drive - 0.1f * v;  // continuous-time dynamics, no gradient
  v += dt * deriv;
  state_[key] = v;
}

}  // namespace omega
