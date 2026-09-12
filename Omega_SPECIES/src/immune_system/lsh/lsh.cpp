// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#include "lsh.h"

namespace omega {

static uint32_t xrng(uint32_t& s) {
  s ^= s << 13;
  s ^= s >> 17;
  s ^= s << 5;
  return s;
}
static float frng(uint32_t& s) {
  return (float)(xrng(s) & 0x7FFFFFFFu) / (float)0x7FFFFFFFu * 2.0f - 1.0f;
}

void LSH::init(uint32_t rng, int hash_bits) {
  if (hash_bits < 1) hash_bits = 1;
  if (hash_bits > MAX_BITS) hash_bits = MAX_BITS;
  bits = hash_bits;
  w.resize((size_t)bits * D);
  uint32_t s = rng ? rng : 0x1234u;
  for (int b = 0; b < bits; b++)
    for (int i = 0; i < D; i++)
      w[(size_t)b * D + i] = frng(s);
}

uint32_t LSH::bucket(const float* v) const {
  uint32_t h = 0;
  for (int b = 0; b < bits; b++) {
    float dot = 0.0f;
    for (int i = 0; i < D; i++)
      dot += w[(size_t)b * D + i] * v[i];
    if (dot >= 0.0f) h |= (1u << b);
  }
  return h;
}

uint32_t LSH::shard(const float* v, int num_shards) const {
  uint32_t b = bucket(v);
  return b % (uint32_t)num_shards;
}

}  // namespace omega
