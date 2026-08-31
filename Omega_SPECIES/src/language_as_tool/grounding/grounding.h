#pragma once
#include <cstdint>

namespace omega {

// Grounding: ties symbols to simulation states. A "concept" is an 8-dim state
// vector; binding returns an existing symbol when the state is close enough to
// a previously grounded one, otherwise mints a new symbol id.
class Grounding {
 public:
  static constexpr int D = 8;
  static constexpr int CAP = 32;
  static constexpr float DEFAULT_THRESH = 0.15f;

  // Return existing symbol id near `state` (within thr), or 0 if none.
  uint32_t find(const float* state, float thr = DEFAULT_THRESH) const;

  // Bind `state` to a symbol: existing if near, else a new id.
  uint32_t bind(const float* state, float thr = DEFAULT_THRESH);

  bool lookup(uint32_t id, float* out) const;
  int count() const { return n_; }

  static float dist(const float* a, const float* b);

 private:
  struct Entry {
    uint32_t id = 0;
    float s[D];
    int uses = 0;
  };
  Entry e_[CAP];
  int n_ = 0;
  uint32_t next_id_ = 1;
};

}  // namespace omega
