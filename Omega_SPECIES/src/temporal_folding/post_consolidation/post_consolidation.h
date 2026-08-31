#pragma once
#include "../pre_consolidation/pre_consolidation.h"

namespace omega {

// Post-consolidation: compares a planned future memory against the actual
// outcome and scores how well the prediction matched reality.
struct Outcome {
  uint32_t tick;
  int action;
  float actual;
};

class PostConsolidation {
 public:
  // Absolute prediction error (predicted - actual).
  float delta(const FutureMemory& plan, const Outcome& outcome) const {
    return plan.predicted - outcome.actual;
  }

  // Match score in [0,1]: 1.0 when actual == predicted, falling off with error.
  float compare(const FutureMemory& plan, const Outcome& outcome) const {
    float d = delta(plan, outcome);
    float score = 1.0f - (d > 0 ? d : -d) / (1.0f + (d > 0 ? d : -d));
    return score < 0.0f ? 0.0f : score;
  }
};

}  // namespace omega
