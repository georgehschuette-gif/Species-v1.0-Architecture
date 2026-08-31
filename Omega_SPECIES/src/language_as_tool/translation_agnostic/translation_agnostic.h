#pragma once
#include <cstdint>

namespace omega {

// Translation-agnostic: meaning across syntax. Different surface tokens that
// refer to the same grounded state are synonyms — they resolve to one concept.
class TranslationAgnostic {
 public:
  static constexpr int N = 16;

  // Declare that tokens a and b are synonyms (same meaning / concept id).
  void alias(const char* a, const char* b);
  // Resolve any known token to its concept id (0 if unknown).
  uint32_t resolve(const char* token) const;

 private:
  struct Alias {
    char a[16];
    char b[16];
    uint32_t id;
  };
  Alias al_[N];
  int n_ = 0;
  uint32_t next_ = 1;
};

}  // namespace omega
