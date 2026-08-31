#include "kolmogorov_challenge.h"
#include <cmath>
#include <cstring>

namespace omega {

void KolmogorovChallenge::set_program(const uint8_t prog[PROGRAM_BYTES]) {
  std::memcpy(prog_, prog, PROGRAM_BYTES);
}

void KolmogorovChallenge::emit(uint8_t* out, int n) const {
  // The 64-byte program seeds a linear-congruential generator; each output byte
  // is a mixed extract of the evolving state. This is the "emitted universe".
  uint64_t s = 0, inc = 0;
  for (int i = 0; i < 8; i++) s = (s << 8) | prog_[i];
  for (int i = 0; i < 8; i++) inc = (inc << 8) | prog_[8 + i];
  if (s == 0) s = 0x9E3779B97F4A7C15ULL;

  int cache = n < MAX_EMIT ? n : MAX_EMIT;
  for (int i = 0; i < n; i++) {
    s = s * 6364136223846793005ULL + (inc | 1ULL);
    uint8_t b = (uint8_t)((s >> 33) ^ (s >> 21) ^ s);
    if (out) out[i] = b;
    if (i < cache) emit_[i] = b;
  }
  emit_len_ = cache;
}

float KolmogorovChallenge::entropy(const uint8_t* data, int n) {
  if (n <= 0) return 0.0f;
  int hist[256];
  for (int i = 0; i < 256; i++) hist[i] = 0;
  for (int i = 0; i < n; i++) hist[data[i]]++;
  float e = 0.0f;
  for (int i = 0; i < 256; i++) {
    if (hist[i] == 0) continue;
    float p = (float)hist[i] / (float)n;
    e -= p * std::log2f(p);
  }
  return e;  // bits per byte, in [0, 8]
}

float KolmogorovChallenge::compression_ratio(const uint8_t* data, int n) {
  if (n <= 0) return 0.0f;
  int runs = 0;
  int i = 0;
  while (i < n) {
    runs++;
    uint8_t v = data[i];
    int len = 1;
    while (i + len < n && data[i + len] == v && len < 255) len++;
    i += len;
  }
  // Each run -> (value, length) = 2 bytes.
  int compressed = runs * 2;
  return (float)compressed / (float)n;
}

float KolmogorovChallenge::output_entropy() const { return entropy(emit_, emit_len_); }
float KolmogorovChallenge::seed_entropy() const { return entropy(prog_, PROGRAM_BYTES); }
float KolmogorovChallenge::output_compression_ratio() const { return compression_ratio(emit_, emit_len_); }
float KolmogorovChallenge::seed_compression_ratio() const { return compression_ratio(prog_, PROGRAM_BYTES); }

bool KolmogorovChallenge::passes() const {
  return output_entropy() >= seed_entropy() &&
         output_compression_ratio() >= seed_compression_ratio();
}

}  // namespace omega
