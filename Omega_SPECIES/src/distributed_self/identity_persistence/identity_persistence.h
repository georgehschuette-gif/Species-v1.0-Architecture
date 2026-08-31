#pragma once
#include <cstdint>
#include <cstddef>

namespace omega {

// Crypto-genotype: a stable 32-byte identity hash per agent, derived from its
// id and its parent's genotype (so lineage is traceable). Uses SHA-256.
struct AgentIdentity {
  uint32_t id = 0;
  uint8_t genotype[32];

  // Derive genotype = SHA-256( id || parent_genotype ). For a genesis agent
  // pass parent=nullptr, plen=0.
  void derive(uint32_t id, const uint8_t* parent, size_t plen);
  void to_hex(char* out, int outlen) const;
};

}  // namespace omega
