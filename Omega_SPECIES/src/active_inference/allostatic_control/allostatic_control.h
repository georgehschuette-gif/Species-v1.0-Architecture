#pragma once
#include <cstdint>

namespace omega {

// Allostatic control: maintains a homeostatic setpoint for a regulated
// variable. On chronic deviation it shifts the setpoint (allostasis) and
// raises the prior precision pulling the belief back toward equilibrium.
class AllostaticControl {
 public:
  void init(float setpoint, float tolerance) {
    setpoint_ = setpoint;
    tolerance_ = tolerance;
    regulated_ = setpoint;
  }

  void step(float measured) {
    regulated_ = measured;
    float e = regulated_ - setpoint_;
    if (e > tolerance_) setpoint_ += 0.05f * e;       // drift up
    else if (e < -tolerance_) setpoint_ += 0.05f * e;  // drift down
  }

  float error() const { return regulated_ - setpoint_; }
  float setpoint() const { return setpoint_; }
  float regulated() const { return regulated_; }

  // Pull applied to the prior mean of the regulated state (state 0).
  float prior_pull() const { return error(); }

  // Prior precision for the regulated state: elevated while out of band.
  float prior_precision() const {
    float e = (regulated_ - setpoint_ > 0) ? (regulated_ - setpoint_)
                                           : (setpoint_ - regulated_);
    return 0.05f + (e > tolerance_ ? 4.0f : 0.0f);
  }

 private:
  float setpoint_ = 0.0f;
  float tolerance_ = 0.20f;
  float regulated_ = 0.0f;
};

}  // namespace omega
