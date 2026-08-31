#include "agent_death.h"

namespace omega {

uint32_t AgentDeath::retire_lowest(AgentPool& pool, const ReputationLedger& rep) {
  if (pool.count() == 0) return 0;
  int worst = 0;
  float worst_t = rep.agent_trust(0);
  for (int i = 1; i < pool.count(); i++) {
    float t = rep.agent_trust(i);
    if (t < worst_t) {
      worst_t = t;
      worst = i;
    }
  }
  uint32_t id = pool.at(worst)->id;
  pool.retire(worst);
  return id;
}

}  // namespace omega
