#include "agent_genesis.h"
#include "../../immune_system/self_antigen/sha256.h"
#include <cstring>

namespace omega {

void AgentPool::init() {
  const char* seed = "Ω-genesis";
  sha256(reinterpret_cast<const uint8_t*>(seed), 9, root_geno_);
  n_ = 0;
  next_id_ = 1;
}

uint32_t AgentPool::spawn(float perf) {
  if (n_ >= CAP) return 0;
  Agent& a = agents_[n_++];
  a.id = next_id_++;
  a.ident.derive(a.id, root_geno_, 32);
  a.perf = perf;
  a.role = 1;
  return a.id;
}

bool AgentPool::retire(int index) {
  if (index < 0 || index >= n_) return false;
  for (int i = index; i < n_ - 1; i++) agents_[i] = agents_[i + 1];
  n_--;
  return true;
}

}  // namespace omega
