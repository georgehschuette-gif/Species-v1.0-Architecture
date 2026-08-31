#pragma once
#include <cstdint>

namespace omega {

// Trust per (agent, domain). Lower trust -> candidate for retirement.
class ReputationLedger {
 public:
  static constexpr int MAX_AGENTS = 8;
  static constexpr int DOMAINS = 4;

  void init() {
    for (int i = 0; i < MAX_AGENTS; i++)
      for (int d = 0; d < DOMAINS; d++) trust_[i][d] = 0.5f;
  }

  float get(int agent, int domain) const {
    if (agent < 0 || agent >= MAX_AGENTS || domain < 0 || domain >= DOMAINS) return 0.0f;
    return trust_[agent][domain];
  }

  void update(int agent, int domain, float delta) {
    if (agent < 0 || agent >= MAX_AGENTS || domain < 0 || domain >= DOMAINS) return;
    float v = trust_[agent][domain] + delta;
    if (v < 0.0f) v = 0.0f;
    if (v > 1.0f) v = 1.0f;
    trust_[agent][domain] = v;
  }

  // Mean trust across domains for an agent.
  float agent_trust(int agent) const {
    if (agent < 0 || agent >= MAX_AGENTS) return 0.0f;
    float s = 0.0f;
    for (int d = 0; d < DOMAINS; d++) s += trust_[agent][d];
    return s / DOMAINS;
  }

 private:
  float trust_[MAX_AGENTS][DOMAINS];
};

}  // namespace omega
