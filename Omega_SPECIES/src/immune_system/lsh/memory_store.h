#pragma once
#include "lsh.h"
#include <cstdint>

namespace omega {

// Fixed-capacity associative memory keyed by LSH bucket. Strengths decay over
// time (forgetting) and strong memories can be consolidated into long-term
// storage. Shared by B-cells (persistent) and T-cells (active).
class MemoryStore {
 public:
  static constexpr int V = 8;    // stored vector dims
  static constexpr int CAP = 64;

  struct Mem {
    uint32_t bucket;
    uint32_t tag;
    float vec[V];
    float strength;
    bool used;
  };

  void init(const LSH* lsh, float decay_per_step, float consolidate_thr);
  void store(uint32_t tag, const float* vec, float init_strength);
  float recall(uint32_t tag, float* out) const;  // returns strength (0 if absent)
  void decay_all(float dt);                       // forgetting curve step
  int prune(float min_strength);                  // drop weak memories
  void consolidate_to(MemoryStore& dst) const;    // move strong -> dst
  int count() const;
  float avg_strength() const;

 private:
  const LSH* lsh_ = nullptr;
  Mem mem_[CAP];
  float decay_ = 0.0f;
  float consol_thr_ = 0.0f;
};

}  // namespace omega
