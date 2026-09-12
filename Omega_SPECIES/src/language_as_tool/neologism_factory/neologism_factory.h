// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#pragma once
#include <cstdint>
#include <vector>

namespace omega {

class NeologismFactory {
 public:
  void mint(uint32_t concept_id, const float* state, char* out, int n) const;

 private:
  static uint32_t hash_state(const float* s, int d);
  static void to_base36(uint32_t v, char* out, int n);
};

class ShardedNeologismFactory {
 public:
  explicit ShardedNeologismFactory(size_t num_shards = 4)
      : num_shards_(num_shards > 0 ? num_shards : 1) {
    factories_.resize(num_shards_);
  }

  void mint(uint32_t concept_id, const float* state, char* out, int n) const {
    factories_[shard_index(concept_id)].mint(concept_id, state, out, n);
  }

  size_t shard_index(uint32_t concept_id) const {
    return concept_id % num_shards_;
  }

  size_t num_shards() const { return num_shards_; }

 private:
  size_t num_shards_;
  std::vector<NeologismFactory> factories_;
};

}  // namespace omega
