#pragma once
#include "../../genesis/primordial_weights/primordial_weights.h"
#include <cstdint>

namespace omega {

// Divergence entropy: monitors semantic drift of the population. Low entropy
// (collapse / premature convergence) calls for more mutation; high entropy
// (chaotic scatter) calls for less. Exposes a modulation factor in [0.5, 2].
class DivergenceEntropy {
 public:
  static constexpr int BINS = 8;

  // Shannon entropy of the weight histogram of a single network, normalized
  // to [0,1] (log2(BINS) is the maximum).
  float entropy_of(const Network& net) const;

  // Map population diversity -> mutation modulation. `diversity` is the
  // Jaccard-style value from SelfWiringGA (0 = identical, 1 = disjoint).
  float adjust(float diversity) const;
};

}  // namespace omega
