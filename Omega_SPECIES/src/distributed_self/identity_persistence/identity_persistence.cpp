#include "identity_persistence.h"
#include "../../immune_system/self_antigen/sha256.h"
#include <cstring>

namespace omega {

void AgentIdentity::derive(uint32_t id, const uint8_t* parent, size_t plen) {
  this->id = id;
  uint8_t buf[64];
  buf[0] = (uint8_t)(id & 0xFF);
  buf[1] = (uint8_t)((id >> 8) & 0xFF);
  buf[2] = (uint8_t)((id >> 16) & 0xFF);
  buf[3] = (uint8_t)((id >> 24) & 0xFF);
  size_t n = 4;
  if (parent && plen > 0) {
    size_t k = plen < 28 ? plen : 28;
    memcpy(buf + n, parent, k);
    n += k;
  }
  sha256(buf, n, genotype);
}

void AgentIdentity::to_hex(char* out, int outlen) const {
  static const char* hex = "0123456789abcdef";
  int k = 0;
  for (int i = 0; i < 32 && k + 2 < outlen; i++) {
    out[k++] = hex[(genotype[i] >> 4) & 0xF];
    out[k++] = hex[genotype[i] & 0xF];
  }
  out[k] = '\0';
}

}  // namespace omega
