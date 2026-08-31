#pragma once
#include "../agent_genesis/agent_genesis.h"
#include "../reputation_ledger/reputation_ledger.h"

namespace omega {

// Agent death: retires obsolete agents (lowest trust) from the pool.
class AgentDeath {
 public:
  // Retire the pool agent with the lowest mean trust. Returns its id, or 0.
  uint32_t retire_lowest(AgentPool& pool, const ReputationLedger& rep);
};

}  // namespace omega
