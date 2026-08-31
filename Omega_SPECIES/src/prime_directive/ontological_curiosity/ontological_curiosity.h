#pragma once
#include <cstdint>

namespace omega {

// Ontological curiosity: "What am I missing?" — tracks per-domain uncertainty
// and surfaces the domain most in need of investigation.
class OntologicalCuriosity {
 public:
  static constexpr int DOMAINS = 4;

  // Feed an uncertainty reading for a domain; returns the current curiosity level.
  float update(float uncertainty, int domain) {
    for (int d = 0; d < DOMAINS; d++) unc_[d] *= 0.99f;  // slow decay
    if (domain >= 0 && domain < DOMAINS)
      unc_[domain] = (unc_[domain] > uncertainty) ? unc_[domain] : uncertainty;
    float best = 0.0f;
    focus_ = 0;
    for (int d = 0; d < DOMAINS; d++) {
      if (unc_[d] > best) {
        best = unc_[d];
        focus_ = d;
      }
    }
    level_ = best;
    return level_;
  }

  int focus_domain() const { return focus_; }
  float level() const { return level_; }

 private:
  float unc_[DOMAINS] = {0.0f, 0.0f, 0.0f, 0.0f};
  float level_ = 0.0f;
  int focus_ = 0;
};

}  // namespace omega
