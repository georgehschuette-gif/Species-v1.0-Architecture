#include "legacy_building.h"
#include <cstring>
#include <cstdio>

namespace omega {

void BootstrapLegacy::snapshot(uint32_t world_hash, float energy, uint32_t winner,
                               uint32_t kc_pass, uint32_t ticks, const char* tag) {
  art_.world_hash = world_hash;
  art_.total_energy = energy;
  art_.selfplay_winner = winner;
  art_.kolmogorov_pass = kc_pass ? 1u : 0u;
  art_.ticks = ticks;
  std::memset(art_.tag, 0, sizeof(art_.tag));
  int n = 0;
  if (tag) while (tag[n] && n < (int)sizeof(art_.tag) - 1) { art_.tag[n] = tag[n]; n++; }
}

void BootstrapLegacy::serialize(uint8_t* out, int* outlen) const {
  int p = 0;
  auto put32 = [&](uint32_t v) { out[p++] = (uint8_t)(v >> 24); out[p++] = (uint8_t)(v >> 16); out[p++] = (uint8_t)(v >> 8); out[p++] = (uint8_t)v; };
  put32(art_.world_hash);
  // float -> raw bits
  uint32_t fe; std::memcpy(&fe, &art_.total_energy, 4); put32(fe);
  put32(art_.selfplay_winner);
  put32(art_.kolmogorov_pass);
  put32(art_.ticks);
  for (int i = 0; i < (int)sizeof(art_.tag); i++) out[p++] = (uint8_t)art_.tag[i];
  if (outlen) *outlen = p;
}

void BootstrapLegacy::to_hex(char* out, int outlen) const {
  uint8_t buf[64]; int len = 0;
  serialize(buf, &len);
  int i = 0;
  for (int k = 0; k < len && i + 2 < outlen; k++) {
    std::snprintf(out + i, outlen - i, "%02X", buf[k]);
    i += 2;
  }
  out[i] = '\0';
}

}  // namespace omega
