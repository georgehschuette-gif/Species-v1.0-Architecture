#pragma once
#include "divergence_detector/divergence_detector.h"
#include "lexicon_consolidation/lexicon_consolidation.h"

namespace omega {

// Semantic error drive: orchestrates divergence detection + lexicon
// consolidation, producing a semantic-error signal and counting splits.
class SemanticErrorDrive {
 public:
  // Process a symbol usage. Returns divergence magnitude; if consolidation
  // decides SPLIT, the split counter increments.
  float process(uint32_t id, const float* state, float tolerance = 0.20f,
                float split_threshold = 0.10f) {
    float d = det_.record(id, state, tolerance);
    if (consol_.resolve(d, split_threshold) == LexiconConsolidation::SPLIT)
      splits_++;
    return d;
  }

  int splits() const { return splits_; }

 private:
  DivergenceDetector det_;
  LexiconConsolidation consol_;
  int splits_ = 0;
};

}  // namespace omega
