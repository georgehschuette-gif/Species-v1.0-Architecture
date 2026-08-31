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

void LSH::init(uint32_t rng) {
  uint32_t s = rng ? rng : 0x1234u;
  for (int b = 0; b < BITS; b++)
    for (int i = 0; i < D; i++) w[b][i] = frng(s);
}

uint32_t LSH::bucket(const float* v) const {
  uint32_t h = 0;
  for (int b = 0; b < BITS; b++) {
    float dot = 0.0f;
    for (int i = 0; i < D; i++) dot += w[b][i] * v[i];
    if (dot >= 0.0f) h |= (1u << b);
  }
  return h;
}

}  // namespace omega
