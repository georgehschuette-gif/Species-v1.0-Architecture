// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#pragma once
#include <cstdint>
#include <vector>
#include <shared_mutex>
#include <mutex>
#include <unordered_map>
#include "../../immune_system/lsh/lsh.h"

namespace omega {

// Grounding: ties symbols to simulation states using LSH-indexed dynamic
// storage. Replaces fixed CAP=32 with growable vector + bucketed lookup.
// Find is O(λ) where λ is avg bucket size, not O(n).
class Grounding {
 public:
  static constexpr int D = 8;
  static constexpr float DEFAULT_THRESH = 0.15f;

  Grounding() { lsh_.init(0x4242u, 16); }  // 16-bit LSH for ~65K buckets

  uint32_t find(const float* state, float thr = DEFAULT_THRESH) const;
  uint32_t find_unlocked(const float* state, float thr) const;  // no-lock variant for internal use
  uint32_t bind(const float* state, float thr = DEFAULT_THRESH);
  bool lookup(uint32_t id, float* out) const;
  int count() const {
    std::shared_lock<std::shared_mutex> lk(mutex_);
    return (int)entries_.size();
  }
  void reserve(size_t cap) {
    std::unique_lock<std::shared_mutex> lk(mutex_);
    entries_.reserve(cap);
  }

  // Batch bind: resolve multiple states concurrently (for distributed language processing)
  std::vector<uint32_t> bind_batch(const std::vector<const float*>& states, float thr = DEFAULT_THRESH);

  static float dist(const float* a, const float* b);

private:
  struct Entry {
    uint32_t id = 0;
    float s[D];
    int uses = 0;
    uint32_t bucket = 0;
  };
  std::vector<Entry> entries_;        // dynamically growing
  mutable std::shared_mutex mutex_;
  LSH lsh_;                           // bucket index for O(1) lookup
  uint32_t next_id_ = 1;
};

}  // namespace omega
