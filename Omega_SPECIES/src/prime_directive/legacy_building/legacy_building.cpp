#include "legacy_building.h"
#include <cstring>

namespace omega {

void LegacyBuilding::snapshot(uint32_t topo, const uint8_t* geno, uint32_t patterns,
                              uint32_t surgeries, uint32_t anomalies, uint32_t ticks,
                              const char* tag) {
  art_.topo_hash = topo;
  if (geno) memcpy(art_.antigen, geno, 32);
  art_.patterns_harvested = patterns;
  art_.surgeries = surgeries;
  art_.anomalies = anomalies;
  art_.ticks = ticks;
  memset(art_.tag, 0, sizeof(art_.tag));
  if (tag) {
    int n = 0;
    while (tag[n] && n < 15) {
      art_.tag[n] = tag[n];
      n++;
    }
  }
}

void LegacyBuilding::serialize(uint8_t* out, int* outlen) const {
  if (!out) return;
  memcpy(out, &art_, sizeof(art_));
  if (outlen) *outlen = (int)sizeof(art_);
}

void LegacyBuilding::to_hex(char* out, int outlen) const {
  static const char* hex = "0123456789abcdef";
  const uint8_t* p = reinterpret_cast<const uint8_t*>(&art_);
  int n = (int)sizeof(art_);
  int k = 0;
  for (int i = 0; i < n && k + 2 < outlen; i++) {
    out[k++] = hex[(p[i] >> 4) & 0xF];
    out[k++] = hex[p[i] & 0xF];
  }
  out[k] = '\0';
}

}  // namespace omega
