#pragma once
#include <cstdint>
#include <cstddef>

namespace omega {

// Legacy building: the artifact left behind on shutdown — a self-contained
// snapshot of identity and lifetime statistics, serializable to bytes
// (e.g. a flash page on the Teensy, or a file on PC).
struct LegacyArtifact {
  uint32_t topo_hash;
  uint8_t antigen[32];      // genotype core
  uint32_t patterns_harvested;
  uint32_t surgeries;
  uint32_t anomalies;
  uint32_t ticks;
  char tag[16];
};

class LegacyBuilding {
 public:
  void snapshot(uint32_t topo, const uint8_t* geno, uint32_t patterns,
                uint32_t surgeries, uint32_t anomalies, uint32_t ticks,
                const char* tag);

  const LegacyArtifact& artifact() const { return art_; }

  // Pack the artifact into a byte buffer (stable layout for persistence).
  void serialize(uint8_t* out, int* outlen) const;

  void to_hex(char* out, int outlen) const;

 private:
  LegacyArtifact art_;
};

}  // namespace omega
