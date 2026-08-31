#include "self_play_arena.h"

namespace omega {

int SelfPlayArena::policy(uint32_t genome, int round) {
  // Mix the genome with the round so the sequence is non-trivial but fully
  // deterministic. mod 3 -> {0,1,2}.
  uint32_t h = genome * 2654435761u + (uint32_t)round * 40503u;
  h ^= h >> 15;
  return (int)(h % 3u);
}

void SelfPlayArena::set_agents(uint32_t genomeA, uint32_t genomeB) {
  gA_ = genomeA; gB_ = genomeB;
  scoreA_ = 0; scoreB_ = 0; rounds_ = 0;
}

void SelfPlayArena::play(int rounds) {
  rounds_ = rounds;
  scoreA_ = 0; scoreB_ = 0;
  for (int r = 0; r < rounds; r++) {
    int mA = policy(gA_, r);
    int mB = policy(gB_, r);
    // RPS payoff for A: +1 win, -1 loss, 0 tie.
    if (mA == mB) continue;
    if ((mA + 1) % 3 == mB) scoreA_ -= 1;  // B beats A
    else scoreA_ += 1;                       // A beats B
  }
  scoreB_ = -scoreA_;  // zero-sum
}

bool SelfPlayArena::equilibrium() const {
  return (scoreA_ >= -1 && scoreA_ <= 1) || (rounds_ > 0 && scoreA_ == 0);
}

int SelfPlayArena::winner() const {
  if (scoreA_ > 0) return 1;
  if (scoreA_ < 0) return 2;
  return 0;
}

}  // namespace omega
