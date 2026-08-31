#pragma once
#include <cstdint>
#include "../pre_consolidation/pre_consolidation.h"
#include "../post_consolidation/post_consolidation.h"

namespace omega {

// Dream: offline memory replay and recombination.
// During "dream" periods the agent replays stored future memories,
// recombines them into novel sequences, and emits mutation seeds
// that can be used by the SurgeonGeneral for self-modification.
class Dream {
 public:
  static constexpr int CAP = 32;
  static constexpr int RECOMBINE_K = 2;

  struct DreamSequence {
    uint32_t ticks[CAP];
    int actions[CAP];
    float predicted[CAP];
    int len = 0;
  };

  void ingest(const PreConsolidation& pre, const PostConsolidation& post,
              const Outcome* outcomes, int n_outcomes);

  // Replay memories in random order, recombine pairs into novel sequences.
  // Returns a mutation seed derived from the recombined sequence.
  uint32_t replay(uint32_t rng);

  // Access the most recent dream sequence for introspection.
  const DreamSequence& last_sequence() const { return last_; }

  int dream_count() const { return dream_count_; }
  int recombinations() const { return recombinations_; }

 private:
  FutureMemory mem_[CAP];
  int n_ = 0;
  DreamSequence last_;
  int dream_count_ = 0;
  int recombinations_ = 0;
};

}  // namespace omega
