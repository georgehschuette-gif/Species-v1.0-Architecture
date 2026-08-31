#include "pragmatics.h"
#include <cstdio>
#include <cstring>

namespace omega {

void Pragmatics::encode(int intent, float value, uint32_t symbol, char* out, int n) const {
  std::snprintf(out, n, "I%d:%.3f#%u", intent, value, symbol);
}

bool Pragmatics::decode(const char* msg, Message& out) const {
  int intent = 0;
  float value = 0.0f;
  unsigned int symbol = 0;
  if (std::sscanf(msg, "I%d:%f#%u", &intent, &value, &symbol) == 3) {
    out.intent = intent;
    out.value = value;
    out.symbol = symbol;
    return true;
  }
  return false;
}

}  // namespace omega
