#pragma once
#include <cstdint>

namespace omega {

// Graceful failure: when confidence (resonance / grounding match) is too low,
// respond honestly with "I don't know" plus a constructive suggestion.
class GracefulFailure {
 public:
  bool should_defer(float confidence, float threshold) const {
    return confidence < threshold;
  }
  void respond(char* out, int n, const char* suggestion) const;
};

}  // namespace omega
