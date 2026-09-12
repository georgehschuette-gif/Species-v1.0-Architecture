// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#pragma once
#include <cstdint>
#include <vector>
#include <shared_mutex>

namespace omega {

// Locality-Sensitive Hashing with configurable bit width. Replaces fixed
// BITS=12 (4K buckets) with dynamic, sharded hash space. Supports up to
// 32-bit hashes (4 billion buckets) and sharded bucketing for concurrent
// lookup across multiple LSH instances.
struct LSH {
  static constexpr int D = 8;     // feature dimensions
  static constexpr int MAX_BITS = 32;

  int bits = 12;                  // configurable: 12 default, up to 32
  std::vector<float> w;           // [bits * D] — dynamically sized
  mutable std::shared_mutex mutex_;

  void init(uint32_t rng, int hash_bits = 12);
  uint32_t bucket(const float* v) const;

  // Sharded bucketing: route to one of N shards
  uint32_t shard(const float* v, int num_shards) const;
};

}  // namespace omega
