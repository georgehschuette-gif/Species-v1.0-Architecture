#pragma once
#include <cstdint>
#include <cstddef>

namespace omega {

// Self-contained SHA-256 (FIPS 180-4). No external dependencies.
// Produces a 32-byte digest.
void sha256(const uint8_t* data, size_t len, uint8_t digest[32]);

}  // namespace omega
