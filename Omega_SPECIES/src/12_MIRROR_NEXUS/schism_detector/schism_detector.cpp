#include "schism_detector.h"
#include <cmath>

namespace omega {

bool SchismDetector::observe(float resonance) {
  if (!std::isfinite(resonance)) resonance = 0.0f;  // guard against NaN/Inf input
  buf_[head_] = resonance;
  head_ = (head_ + 1) % WINDOW;
  if (n_ < WINDOW) n_++;
  if (resonance < thr_) low_streak_++;
  else low_streak_ = 0;
  return schism();
}

bool SchismDetector::observe_states(const float* self, const float* other) {
  float r = rm_.resonance(self, other);
  return observe(r);
}

float SchismDetector::coherence() const {
  if (n_ == 0) return 0.0f;
  float s = 0.0f;
  for (int i = 0; i < n_; i++) s += buf_[i];
  return s / (float)n_;
}

}  // namespace omega
