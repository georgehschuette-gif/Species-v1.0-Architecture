#pragma once
#include <cstdint>
#include <cmath>

namespace omega {

// Mutation scheduler: anneals the base mutation rate across generations so
// the search explores broadly early and refines later.
class MutationScheduler {
 public:
  void reset(float base) {
    base_ = base;
    cur_ = base;
    gen_ = 0;
  }
  void step() {
    gen_++;
    cur_ = base_ * powf(0.985f, (float)gen_);  // exponential anneal
    if (cur_ < 0.02f) cur_ = 0.02f;
  }
  float rate() const { return cur_; }
  uint32_t generation() const { return gen_; }

 private:
  float base_ = 0.2f;
  float cur_ = 0.2f;
  uint32_t gen_ = 0;
};

}  // namespace omega
