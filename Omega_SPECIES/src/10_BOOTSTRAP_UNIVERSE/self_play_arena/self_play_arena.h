#pragma once
#include <cstdint>

namespace omega {

// Self-play arena: two agents (each defined by a genome) compete in a symmetric
// zero-sum game (rock-paper-scissors style) over many rounds. Because a copy is
// a deterministic function of its genome, two identical copies necessarily reach
// an equilibrium (equal scores). This is the "copy vs copy" self-play milestone.
class SelfPlayArena {
 public:
  // Deterministic policy: move in {0,1,2} from genome + round index.
  static int policy(uint32_t genome, int round);

  void set_agents(uint32_t genomeA, uint32_t genomeB);
  void play(int rounds);

  int scoreA() const { return scoreA_; }
  int scoreB() const { return scoreB_; }

  // True when neither copy beats the other (scores within 1 point) — the
  // self-play equilibrium. Two identical genomes always satisfy this.
  bool equilibrium() const;

  // 0 = tie, 1 = A won, 2 = B won.
  int winner() const;

 private:
  uint32_t gA_ = 0, gB_ = 0;
  int scoreA_ = 0, scoreB_ = 0;
  int rounds_ = 0;
};

}  // namespace omega
