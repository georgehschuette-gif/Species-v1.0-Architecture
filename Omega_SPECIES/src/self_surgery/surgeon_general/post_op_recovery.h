#pragma once

namespace omega {

// Post-op recovery: decides whether to keep a mutation or roll it back.
// We keep it only if actual performance holds up against the pre-op baseline
// (reinforces on validation, not just the training-set prediction).
class PostOpRecovery {
 public:
  bool accept(float pre_op, float actual, float predicted) const {
    (void)predicted;
    // Keep if actual performance did not degrade below the pre-op baseline.
    return actual >= pre_op - 1e-3f;
  }
};

}  // namespace omega
