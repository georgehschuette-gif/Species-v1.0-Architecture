#pragma once
#include <cstdint>
#include "../identity_persistence/identity_persistence.h"

namespace omega {

struct Agent {
  uint32_t id = 0;
  AgentIdentity ident;
  float perf = 0.0f;
  int role = 0;  // 0 idle, 1 worker, 2 leader
};

// Agent genesis / pool. In this single-process simulation agents are scheduled
// cooperatively; on a multicore host each could back a real std::thread.
class AgentPool {
 public:
  static constexpr int CAP = 8;

  void init();
  uint32_t spawn(float perf);  // returns new agent id (0 if full)
  bool retire(int index);      // remove agent at index
  int count() const { return n_; }
  Agent* at(int i) { return (i >= 0 && i < n_) ? &agents_[i] : nullptr; }
  const Agent* at(int i) const { return (i >= 0 && i < n_) ? &agents_[i] : nullptr; }
  uint32_t next_id() const { return next_id_; }

 private:
  Agent agents_[CAP];
  int n_ = 0;
  uint32_t next_id_ = 1;
  uint8_t root_geno_[32];
};

}  // namespace omega
