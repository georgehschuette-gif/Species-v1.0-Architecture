#pragma once
#include <cstdint>
#include <cstddef>

namespace omega {

// 64-byte self-antigen: the identity hash. First 32 bytes = stable genotype
// core (SHA-256 of the genotype). Last 32 bytes = dynamic phenotype
// projection (SHA-256 of the current internal state). "Self" is recognized by
// matching the genotype core; the phenotype tracks drift.
struct SelfAntigen {
  static constexpr int BYTES = 64;
  uint8_t data[BYTES];

  // Derive the stable core from a genotype byte buffer (e.g. topology hash).
  void compute_genotype(const uint8_t* genotype, size_t len);

  // Re-project the dynamic phenotype half from a state vector.
  void project_phenotype(const float* state, int n);

  // Genotype core must match exactly for "self".
  bool is_self(const SelfAntigen& other) const;

  // Number of differing bytes in the phenotype half (0..32): drift measure.
  int phenotype_drift(const SelfAntigen& other) const;

  void to_hex(char* out, int outlen) const;
};

}  // namespace omega
