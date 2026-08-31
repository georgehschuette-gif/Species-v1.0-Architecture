#pragma once
#include <cstdint>
#include <cmath>

namespace omega {

// Learning-rate oscillator: a sinusoidal meta-schedule so the perturbation
// scale breathes rather than staying fixed — avoids getting stuck at one
// resolution of search.
class LearningRateOscillator {
 public:
  void reset(float base, float period_gens) {
    base_ = base;
    period_ = period_gens > 1.0f ? period_gens : 1.0f;
    t_ = 0;
  }
  void step() { t_++; }
  // Returns lr in [0, base_] following a half-sine.
  float lr() const {
    float phase = 2.0f * 3.14159265f * (float)t_ / period_;
    return base_ * (0.5f + 0.5f * sinf(phase));
  }
  uint32_t t() const { return t_; }

 private:
  float base_ = 0.2f;
  float period_ = 20.0f;
  uint32_t t_ = 0;
};

}  // namespace omega
