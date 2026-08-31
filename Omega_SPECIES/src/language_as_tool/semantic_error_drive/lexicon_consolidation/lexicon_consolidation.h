#pragma once
#include <cstdint>

namespace omega {

// Lexicon consolidation: resolves a detected semantic divergence. Small drift
// is tolerated (KEEP); large divergence means the word has split and deserves
// its own symbol (SPLIT).
class LexiconConsolidation {
 public:
  enum Resolution { KEEP = 0, SPLIT = 1 };

  Resolution resolve(float divergence, float split_threshold) const {
    return (divergence > split_threshold) ? SPLIT : KEEP;
  }
};

}  // namespace omega
