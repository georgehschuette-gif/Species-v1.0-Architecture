#pragma once
#include <cstdint>

namespace omega {

// Resonance matching: how well this agent can model the other. Resonance is
// the cosine similarity between the projected-other state and the internal
// self state — high resonance means "I can be you."
class ResonanceMatching {
 public:
  static constexpr int D = 8;

  // Cosine similarity in [-1,1]; 1 = perfect resonance.
  float resonance(const float* self_state, const float* other_projected) const;
};

}  // namespace omega
