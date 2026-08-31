#pragma once
#include <cstdint>

namespace omega {

// Hivemind: a minimal but functional RAFT consensus over a single proposed
// log entry, simulated in one process (no network transport). Elects a leader
// by majority vote, replicates the entry, and commits on majority replication.
class Hivemind {
 public:
  static constexpr int N = 5;        // cluster size
  static constexpr int MAJORITY = 3;

  enum Role { FOLLOWER = 0, CANDIDATE = 1, LEADER = 2 };

  struct Node {
    int role = FOLLOWER;
    uint32_t term = 0;
    int voted_for = -1;     // node index or -1
    int log_len = 0;        // replicated entries
    uint32_t commit_index = 0;
    int election_timer = 0;
    int votes = 0;
  };

  void init();
  void tick();                 // advance one simulation step
  void propose_entry();        // leader will replicate a single entry
  bool committed() const { return entry_committed_; }
  int leader() const { return leader_; }
  uint32_t term() const { return term_; }

 private:
  Node nodes_[N];
  uint32_t term_ = 0;
  int leader_ = -1;
  bool pending_entry_ = false;
  bool entry_committed_ = false;
  uint32_t rng_ = 0x5A17u;
};

}  // namespace omega
