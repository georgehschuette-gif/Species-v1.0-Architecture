#pragma once
#include <cstdint>
#include <cstddef>

namespace omega {

// Kolmogorov challenge: a 64-byte "program" that emits a longer byte stream.
// The real KC test is a *compressibility* check — the emitted program output
// must be less compressible (higher entropy) than the program (seed) itself.
// We measure both Shannon entropy and a run-length compression ratio.
class KolmogorovChallenge {
 public:
  static constexpr int PROGRAM_BYTES = 64;
  static constexpr int MAX_EMIT = 4096;

  void set_program(const uint8_t prog[PROGRAM_BYTES]);

  // Run the 64-byte program, producing n bytes into out. Also caches the first
  // MAX_EMIT bytes internally for entropy/compression measurement.
  void emit(uint8_t* out, int n) const;

  // Normalized Shannon entropy in bits/byte (0..8) of a buffer.
  static float entropy(const uint8_t* data, int n);

  // RLE compression ratio = compressed_size / raw_size (lower = more compressible).
  // A run is encoded as (value, length) pairs, 2 bytes each.
  static float compression_ratio(const uint8_t* data, int n);

  float output_entropy() const;
  float seed_entropy() const;
  float output_compression_ratio() const;
  float seed_compression_ratio() const;

  // Passes the KC test: emitted output is at least as incompressible as the
  // seed (higher entropy AND not more compressible).
  bool passes() const;

 private:
  uint8_t prog_[PROGRAM_BYTES];
  mutable uint8_t emit_[MAX_EMIT];
  mutable int emit_len_ = 0;
};

}  // namespace omega
