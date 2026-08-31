#pragma once
#include "../../genesis/primordial_weights/primordial_weights.h"
#include "mutation_priority.h"

namespace omega {

// Surgical planning: evaluates candidate mutations in a sandbox (the reality
// engine) BEFORE they touch the live network. Two stimulus sets model
// train/validation, so over-fitting is detectable.
class SurgicalPlanning {
 public:
  // Performance of a network on a stimulus set (0 = training, 1 = validation).
  float evaluate(const Network& net, int set) const;

  // Apply the plan to a COPY and return predicted (training) performance.
  float dry_run(const Network& net, const MutationPlan& plan);

  // Apply the plan to a live network (used by the Surgeon General).
  void apply(Network& net, const MutationPlan& plan, float aggressiveness = 0.5f);

 private:
  static uint32_t rnd(uint32_t& s) {
    s ^= s << 13;
    s ^= s >> 17;
    s ^= s << 5;
    return s;
  }
};

}  // namespace omega
