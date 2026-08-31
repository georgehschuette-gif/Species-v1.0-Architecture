#pragma once
#include <cstdint>

namespace omega {

// Locality-Sensitive Hashing: maps a D-dim vector into a bucket id via random
// hyperplane signs. Used to index immune memories by similarity.
struct LSH {
  static constexpr int D = 8;     // feature dimensions
  static constexpr int BITS = 12;  // hash bits -> 4096 buckets
  float w[BITS][D];

  void init(uint32_t rng);
  uint32_t bucket(const float* v) const;
};

}  // namespace omega
