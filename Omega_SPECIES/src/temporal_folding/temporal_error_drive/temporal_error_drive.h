#pragma once
#include <cstdint>

namespace omega {

// Temporal error drive: the divergence between a planned (predicted) and actual
// outcome. A large delta signals a model mismatch and is fed to the immune
// system as an inflammation trigger.
class TemporalErrorDrive {
 public:
  // Prediction error magnitude (absolute).
  float error(float predicted, float actual) const {
    float d = predicted - actual;
    return d > 0.0f ? d : -d;
  }

  // Severity to route to the immune system (0 if within tolerance).
  float severity(float predicted, float actual, float tolerance) const {
    float e = error(predicted, actual);
    return e > tolerance ? e - tolerance : 0.0f;
  }

  bool triggers_immune(float predicted, float actual, float tolerance) const {
    return severity(predicted, actual, tolerance) > 0.0f;
  }
};

}  // namespace omega
