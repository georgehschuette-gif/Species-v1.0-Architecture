#pragma once
#include <cstdint>

namespace omega {

// Neologism factory: mints a new word token for a concept that has no existing
// label. The token is deterministic in the concept id + state, so the same
// concept always yields the same word.
class NeologismFactory {
 public:
  // Produce a token like "w<id36><suffix>" into out (n bytes).
  void mint(uint32_t concept_id, const float* state, char* out, int n) const;

 private:
  static uint32_t hash_state(const float* s, int d);
  static void to_base36(uint32_t v, char* out, int n);
};

}  // namespace omega
