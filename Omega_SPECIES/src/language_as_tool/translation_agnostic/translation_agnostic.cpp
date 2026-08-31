#include "translation_agnostic.h"
#include <cstring>

namespace omega {

void TranslationAgnostic::alias(const char* a, const char* b) {
  if (n_ >= N) return;
  uint32_t id = next_++;
  std::strncpy(al_[n_].a, a, 15);
  al_[n_].a[15] = '\0';
  std::strncpy(al_[n_].b, b, 15);
  al_[n_].b[15] = '\0';
  al_[n_].id = id;
  n_++;
}

uint32_t TranslationAgnostic::resolve(const char* token) const {
  for (int i = 0; i < n_; i++) {
    if (std::strcmp(al_[i].a, token) == 0 || std::strcmp(al_[i].b, token) == 0)
      return al_[i].id;
  }
  return 0;
}

}  // namespace omega
