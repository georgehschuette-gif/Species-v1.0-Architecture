#pragma once
#include <cstdint>

namespace omega {

// Pragmatics: intent over grammar. A message carries an intent code plus a
// payload; we encode/decode the (intent, value) regardless of surface form.
struct Message {
  int intent = 0;   // 0 unknown, 1 query, 2 assert, 3 defer
  float value = 0.0f;
  uint32_t symbol = 0;
};

class Pragmatics {
 public:
  void encode(int intent, float value, uint32_t symbol, char* out, int n) const;
  bool decode(const char* msg, Message& out) const;
};

}  // namespace omega
