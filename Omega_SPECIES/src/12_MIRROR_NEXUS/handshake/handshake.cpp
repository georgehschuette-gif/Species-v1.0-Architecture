#include "handshake.h"
#include "immune_system/self_antigen/sha256.h"
#include <cstring>

namespace omega {

namespace {
uint64_t mulmod(uint64_t a, uint64_t b, uint64_t m) {
  uint64_t r = 0;
  a %= m;
  while (b) {
    if (b & 1) { r += a; if (r >= m) r -= m; }
    a <<= 1;    if (a >= m) a -= m;
    b >>= 1;
  }
  return r;
}

uint64_t powmod(uint64_t a, uint64_t e, uint64_t m) {
  uint64_t r = 1;
  a %= m;
  while (e) {
    if (e & 1) r = mulmod(r, a, m);
    a = mulmod(a, a, m);
    e >>= 1;
  }
  return r;
}

// Reduce the first 8 bytes of a SHA-256 digest into [1, Q-1].
uint64_t digest_to_scalar(const uint8_t d[32]) {
  uint64_t lo = 0;
  for (int i = 0; i < 8; i++) lo |= (uint64_t)d[i] << (8 * i);
  uint64_t v = (lo % (Handshake::Q - 1));
  return v + 1;  // in [1, Q-1]
}

uint64_t hash_mod_q(const uint8_t* data, size_t len) {
  uint8_t dig[32];
  sha256(data, len, dig);
  return digest_to_scalar(dig);
}
}  // namespace

void Handshake::set_identity(const uint8_t identity[IDENTITY_BYTES]) {
  std::memcpy(identity_, identity, IDENTITY_BYTES);
  uint64_t s = hash_mod_q(identity, IDENTITY_BYTES);
  y_ = powmod(G, s, P);
}

void Handshake::prove(const uint8_t nonce[IDENTITY_BYTES], Proof* out) const {
  uint64_t s = hash_mod_q(identity_, IDENTITY_BYTES);

  // Deterministic-per-input nonce k in [1, Q-1] (a CSPRNG would be used in prod).
  uint8_t kb[IDENTITY_BYTES + IDENTITY_BYTES + 1];
  std::memcpy(kb, identity_, IDENTITY_BYTES);
  std::memcpy(kb + IDENTITY_BYTES, nonce, IDENTITY_BYTES);
  kb[IDENTITY_BYTES + IDENTITY_BYTES] = 'k';
  uint64_t k = hash_mod_q(kb, sizeof(kb));

  out->R = powmod(G, k, P);

  // Challenge e = H(R || y || nonce) mod Q.
  uint8_t cb[8 + 8 + IDENTITY_BYTES];
  for (int i = 0; i < 8; i++) cb[i] = (uint8_t)(out->R >> (8 * i));
  for (int i = 0; i < 8; i++) cb[8 + i] = (uint8_t)(y_ >> (8 * i));
  std::memcpy(cb + 16, nonce, IDENTITY_BYTES);
  out->e = hash_mod_q(cb, sizeof(cb));

  // Response z = (k + e*s) mod Q.
  uint64_t es = mulmod(out->e, s, Q);
  out->z = (k + es) % Q;
}

bool Handshake::verify(const uint64_t y, const uint8_t nonce[IDENTITY_BYTES],
                       const Proof& p) const {
  // Recompute the challenge from the public transcript.
  uint8_t cb[8 + 8 + IDENTITY_BYTES];
  for (int i = 0; i < 8; i++) cb[i] = (uint8_t)(p.R >> (8 * i));
  for (int i = 0; i < 8; i++) cb[8 + i] = (uint8_t)(y >> (8 * i));
  std::memcpy(cb + 16, nonce, IDENTITY_BYTES);
  uint64_t e2 = hash_mod_q(cb, sizeof(cb));
  if (e2 != p.e) return false;

  // g^z == R * y^e  (mod p)
  uint64_t lhs = powmod(G, p.z, P);
  uint64_t rhs = mulmod(p.R, powmod(y, p.e, P), P);
  return lhs == rhs;
}

void Handshake::degraded_exchange(const uint8_t peer[IDENTITY_BYTES],
                                  uint8_t out[IDENTITY_BYTES]) const {
  // Teensy path: no ZKP, just relay the 64-byte identity (UART frame).
  std::memcpy(out, peer, IDENTITY_BYTES);
}

}  // namespace omega
