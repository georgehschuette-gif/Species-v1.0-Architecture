#pragma once
#include <cstdint>
#include <cstddef>

namespace omega {

// Bootstrap legacy: the artifact the simulated universe leaves behind so a
// successor can be bootstrapped — a snapshot of the toy-physics world, the
// self-play outcome, and the Kolmogorov-challenge result. Serializes to a stable
// byte layout (a flash page on Teensy, or a file on PC). Distinct from the
// prime_directive lifetime artifact.
struct BootstrapArtifact {
  uint32_t world_hash;       // hash of toy_physics state
  float    total_energy;     // conserved energy of the world
  uint32_t selfplay_winner;  // 0 = tie, 1 = A, 2 = B
  uint32_t kolmogorov_pass;  // 1 if the KC test passed, else 0
  uint32_t ticks;            // simulation ticks elapsed
  char     tag[16];
};

class BootstrapLegacy {
 public:
  void snapshot(uint32_t world_hash, float energy, uint32_t winner,
                uint32_t kc_pass, uint32_t ticks, const char* tag);

  const BootstrapArtifact& artifact() const { return art_; }

  // Pack into a stable byte buffer for persistence.
  void serialize(uint8_t* out, int* outlen) const;

  void to_hex(char* out, int outlen) const;

 private:
  BootstrapArtifact art_;
};

}  // namespace omega
