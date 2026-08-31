#include "resonance_matching.h"
#include <cmath>

namespace omega {

float ResonanceMatching::resonance(const float* self_state, const float* other_projected) const {
  float dot = 0.0f, na = 0.0f, nb = 0.0f;
  for (int i = 0; i < D; i++) {
    if (!std::isfinite(self_state[i]) || !std::isfinite(other_projected[i])) return 0.0f;
    dot += self_state[i] * other_projected[i];
    na += self_state[i] * self_state[i];
    nb += other_projected[i] * other_projected[i];
  }
  if (na < 1e-9f || nb < 1e-9f) return 0.0f;
  float c = dot / (std::sqrt(na) * std::sqrt(nb));
  return c < -1.0f ? -1.0f : (c > 1.0f ? 1.0f : c);
}

}  // namespace omega
