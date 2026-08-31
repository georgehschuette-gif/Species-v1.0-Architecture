#pragma once
#include <cstdint>
#include "xeno_empathy/resonance_matching/resonance_matching.h"

namespace omega {

// Schism detector: watches the resonance between two agents over time. When
// resonance stays below a threshold for a sustained streak, the two agents have
// "schismed" (diverged irreconcilably). Uses a rolling window of recent samples.
class SchismDetector {
 public:
  static constexpr int D = 8;
  static constexpr int WINDOW = 32;

  SchismDetector() : thr_(0.10f), streak_(8) {}

  // Feed a resonance reading in [-1,1]. Returns true if a schism is now active.
  bool observe(float resonance);

  // Convenience: observe two state vectors directly (computes resonance first).
  bool observe_states(const float* self, const float* other);

  bool schism() const { return low_streak_ >= streak_; }
  float coherence() const;          // mean resonance over the window
  int samples() const { return n_; }
  int low_streak() const { return low_streak_; }

  void set_threshold(float t) { thr_ = t; }
  void set_streak(int s) { streak_ = s; }

 private:
  float buf_[WINDOW];
  int n_ = 0;
  int head_ = 0;
  int low_streak_ = 0;
  float thr_;
  int streak_;
  ResonanceMatching rm_;
};

}  // namespace omega
