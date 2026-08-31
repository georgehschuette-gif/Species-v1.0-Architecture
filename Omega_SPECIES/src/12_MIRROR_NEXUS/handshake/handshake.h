#pragma once
#include <cstdint>
#include <cstddef>

namespace omega {

// Mirror Nexus — handshake (zero-knowledge proof of identity).
//
// A Fiat-Shamir Schnorr identification over a prime field (p, q, g). The agent
// proves knowledge of its 64-byte identity secret without revealing it: the
// verifier only ever sees the public key y = g^s mod p and a proof that the
// prover knows s. Field size is simulation-grade (documented in the .cpp); the
// protocol math is a faithful Schnorr ZKP.
//
// On Teensy this degrades to a raw 64-byte identity exchange over UART.
class Handshake {
 public:
  static constexpr int IDENTITY_BYTES = 64;

  // Prime field: p is a safe prime (p = 2q + 1), g=2 has order q.
  static constexpr uint64_t P = 1414819537958024999ULL;  // 0x13a272dfad453727
  static constexpr uint64_t Q = 707409768979012499ULL;   // (P-1)/2, prime
  static constexpr uint64_t G = 2ULL;

  struct Proof {
    uint64_t R = 0;  // commitment  g^k mod p
    uint64_t z = 0;  // response    (k + e*s) mod q
    uint64_t e = 0;  // challenge   H(R||y||nonce) mod q
  };

  // Load the 64-byte identity; derive the public key y = g^s mod p.
  void set_identity(const uint8_t identity[IDENTITY_BYTES]);

  // Produce a proof of knowledge of the identity, bound to a verifier nonce.
  void prove(const uint8_t nonce[IDENTITY_BYTES], Proof* out) const;

  // Verify a proof against the public key y and the same verifier nonce.
  // Returns true iff the prover knows the secret behind y.
  bool verify(const uint64_t y, const uint8_t nonce[IDENTITY_BYTES],
              const Proof& p) const;

  // Raw 64-byte identity exchange (Teensy degraded mode).
  void degraded_exchange(const uint8_t peer[IDENTITY_BYTES],
                         uint8_t out[IDENTITY_BYTES]) const;

  uint64_t public_key() const { return y_; }

 private:
  uint8_t identity_[IDENTITY_BYTES];
  uint64_t y_ = 0;
};

}  // namespace omega
