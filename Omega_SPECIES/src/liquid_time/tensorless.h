#pragma once
#include <cstdint>
#include <unordered_map>

namespace omega {

// Tensorless state engine: nodes addressed by hashed keys, no matrices,
// no gradients. Phase 1 wires these keys into an evolving topology.
class TensorlessEngine {
 public:
  void set(uint32_t key, float v) { state_[key] = v; }
  float get(uint32_t key) const {
    auto it = state_.find(key);
    return it == state_.end() ? 0.0f : it->second;
  }

  // Euler integration of a leaky-integrator node: dv = (drive - 0.1*v) * dt.
  void euler_step(uint32_t key, float dt, float drive);

  uint32_t size() const { return static_cast<uint32_t>(state_.size()); }
  void clear() { state_.clear(); }

 private:
  std::unordered_map<uint32_t, float> state_;
};

// Deterministic integer hash (Wang mix) to derive node keys from (a,b).
inline uint32_t hash_key(uint32_t a, uint32_t b) {
  uint32_t x = a * 374761393u + b * 668265263u;
  x = (x ^ (x >> 13)) * 1274126177u;
  return x ^ (x >> 16);
}

}  // namespace omega
