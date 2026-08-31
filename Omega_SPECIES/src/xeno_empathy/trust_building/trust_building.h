#pragma once
#include <cstdint>

namespace omega {

// Trust building: tracks the consistency of an external agent's projections
// over time. Stable resonance raises trust; volatile resonance lowers it.
class TrustBuilding {
 public:
  void observe(float resonance);  // resonance in [-1,1]
  float trust() const { return trust_; }

  private:
   float trust_ = 0.5f;
   float prev_ = 0.0f;
   int n_ = 0;
};

}  // namespace omega
