#include "self_antigen.h"
#include "sha256.h"
#include <cstring>

namespace omega {

void SelfAntigen::compute_genotype(const uint8_t* genotype, size_t len) {
  uint8_t h1[32];
  sha256(genotype, len, h1);
  memcpy(data, h1, 32);

  // Second pass over genotype || 0x01 -> distinct 32 bytes for phenotype seed.
  uint8_t buf[64];
  size_t n = len < 63 ? len : 63;
  if (genotype && len > 0) memcpy(buf, genotype, n);
  buf[n] = 0x01;
  uint8_t h2[32];
  sha256(buf, n + 1, h2);
  memcpy(data + 32, h2, 32);
}

void SelfAntigen::project_phenotype(const float* state, int n) {
  uint8_t buf[64];
  int m = n < 16 ? n : 16;
  for (int i = 0; i < m; i++) {
    int v = (int)(state[i] * 1000.0f);
    buf[i * 4] = (uint8_t)(v & 0xFF);
    buf[i * 4 + 1] = (uint8_t)((v >> 8) & 0xFF);
    buf[i * 4 + 2] = (uint8_t)((v >> 16) & 0xFF);
    buf[i * 4 + 3] = (uint8_t)((v >> 24) & 0xFF);
  }
  uint8_t h[32];
  sha256(buf, (size_t)(m * 4), h);
  memcpy(data + 32, h, 32);  // phenotype half tracks current state
}

bool SelfAntigen::is_self(const SelfAntigen& other) const {
  return memcmp(data, other.data, 32) == 0;  // genotype core must match
}

int SelfAntigen::phenotype_drift(const SelfAntigen& other) const {
  int d = 0;
  for (int i = 32; i < BYTES; i++)
    if (data[i] != other.data[i]) d++;
  return d;
}

void SelfAntigen::to_hex(char* out, int outlen) const {
  static const char* hex = "0123456789abcdef";
  int k = 0;
  for (int i = 0; i < BYTES && k + 2 < outlen; i++) {
    out[k++] = hex[(data[i] >> 4) & 0xF];
    out[k++] = hex[data[i] & 0xF];
  }
  out[k] = '\0';
}

}  // namespace omega
