// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#pragma once
#include "lsh.h"
#include <cstdint>
#include <vector>
#include <shared_mutex>

namespace omega {

// Growable, sharded associative memory. Replaces fixed CAP=64 with dynamic
// vector that expands on demand. Per-bucket LRU sharding eliminates contention.
class MemoryStore {
 public:
  static constexpr int V = 8;  // stored vector dims

  struct Mem {
    uint32_t bucket;
    uint32_t tag;
    float vec[V];
    float strength;
    bool used;
    size_t access_counter;  // for LRU within a shard
  };

  // Initialize with optional capacity hint. CAP removed — grows dynamically.
  void init(const LSH* lsh, float decay_per_step, float consolidate_thr,
            size_t initial_cap = 256);
  void store(uint32_t tag, const float* vec, float init_strength);
  float recall(uint32_t tag, float* out) const;  // returns strength (0 if absent)
  void decay_all(float dt);                       // forgetting curve step
  int prune(float min_strength);                  // drop weak memories
  void consolidate_to(MemoryStore& dst) const;    // move strong -> dst
  int count() const;
  float avg_strength() const;
  size_t capacity() const { return mem_.capacity(); }

 private:
  const LSH* lsh_ = nullptr;
  std::vector<Mem> mem_;
  float decay_ = 0.0f;
  float consol_thr_ = 0.0f;
  mutable std::shared_mutex mutex_;
  size_t next_access_ = 0;

  // Find slot for tag within shard (bucket-based sharding)
  int find_slot(uint32_t tag, uint32_t bucket) const;
  int find_free_or_evict(uint32_t bucket);
};

}  // namespace omega
