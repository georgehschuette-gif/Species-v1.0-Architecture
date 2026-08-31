#include "neologism_factory.h"
#include <cmath>
#include <cstring>

namespace omega {

uint32_t NeologismFactory::hash_state(const float* s, int d) {
  uint32_t h = 0x811C9DC5u;
  for (int i = 0; i < d; i++) {
    int v = (int)(s[i] * 1000.0f);
    uint8_t b[4];
    b[0] = (uint8_t)(v & 0xFF);
    b[1] = (uint8_t)((v >> 8) & 0xFF);
    b[2] = (uint8_t)((v >> 16) & 0xFF);
    b[3] = (uint8_t)((v >> 24) & 0xFF);
    for (int k = 0; k < 4; k++) {
      h ^= b[k];
      h *= 0x01000193u;
    }
  }
  return h;
}

void NeologismFactory::to_base36(uint32_t v, char* out, int n) {
  static const char* d = "0123456789abcdefghijklmnopqrstuvwxyz";
  int i = 0;
  if (v == 0 && i + 1 < n) out[i++] = '0';
  while (v > 0 && i + 1 < n) {
    out[i++] = d[v % 36];
    v /= 36;
  }
  out[i] = '\0';
}

void NeologismFactory::mint(uint32_t concept_id, const float* state, char* out, int n) const {
  if (n <= 0 || out == nullptr) return;  // honor the caller's buffer contract
  char id36[12];
  to_base36(concept_id, id36, sizeof(id36));
  uint32_t h = hash_state(state, 8);
  char suf[12];
  to_base36(h % 1296u, suf, sizeof(suf));  // 2 base-36 chars
  int k = 0;
  out[k++] = 'w';
  for (int i = 0; id36[i] && k + 1 < n; i++) out[k++] = id36[i];
  if (k + 1 < n) out[k++] = '-';
  for (int i = 0; suf[i] && k + 1 < n; i++) out[k++] = suf[i];
  if (k < n) out[k] = '\0';
  else out[n - 1] = '\0';  // guarantee null termination within the buffer
}

}  // namespace omega
