#pragma once
#include <cstdint>

namespace omega {

// Pre-consolidation: stores an intended action as a "future memory" before it
// is executed, time-stamped so it can be matched against the outcome later.
struct FutureMemory {
  uint32_t tick;
  int action;
  float predicted;
};

class PreConsolidation {
 public:
  static constexpr int CAP = 16;

  void plan(uint32_t tick, int action, float predicted) {
    if (n_ >= CAP) return;
    mem_[n_].tick = tick;
    mem_[n_].action = action;
    mem_[n_].predicted = predicted;
    n_++;
  }

  const FutureMemory* at(int i) const { return (i >= 0 && i < n_) ? &mem_[i] : nullptr; }
  int count() const { return n_; }

 private:
  FutureMemory mem_[CAP];
  int n_ = 0;
};

}  // namespace omega
