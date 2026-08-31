#include "trust_building.h"
#include <cmath>

namespace omega {

void TrustBuilding::observe(float resonance) {
  // Stable resonance (close to previous reading) raises trust; volatile
  // resonance lowers it. Trust is clamped to [0,1].
  static constexpr float kStep = 0.02f;
  if (n_ > 0) {
    float delta = std::fabs(resonance - prev_);
    if (delta < 0.15f) {
      trust_ += kStep;
    } else {
      trust_ -= kStep;
    }
    if (trust_ < 0.0f) trust_ = 0.0f;
    if (trust_ > 1.0f) trust_ = 1.0f;
  }
  prev_ = resonance;
  ++n_;
}

}  // namespace omega
