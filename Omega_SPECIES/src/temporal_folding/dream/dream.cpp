#include "dream.h"

namespace omega {

void Dream::ingest(const PreConsolidation& pre, const PostConsolidation& post,
                   const Outcome* outcomes, int n_outcomes) {
  for (int i = 0; i < pre.count() && n_ < CAP; i++) {
    const FutureMemory* fm = pre.at(i);
    mem_[n_].tick = fm->tick;
    mem_[n_].action = fm->action;
    mem_[n_].predicted = fm->predicted;
    n_++;
  }
  (void)post;
  (void)outcomes;
  (void)n_outcomes;
}

uint32_t Dream::replay(uint32_t rng) {
  dream_count_++;
  if (n_ == 0) return rng;

  // Shuffle memories using a deterministic Fisher-Yates with provided RNG.
  for (int i = n_ - 1; i > 0; i--) {
    uint32_t j = rng % (uint32_t)(i + 1);
    if (j != (uint32_t)i) {
      FutureMemory tmp = mem_[i];
      mem_[i] = mem_[j];
      mem_[j] = tmp;
    }
  }

  // Recombine pairs into a novel sequence.
  int seq_len = n_ < RECOMBINE_K ? n_ : RECOMBINE_K;
  if (seq_len > CAP) seq_len = CAP;

  last_.len = seq_len;
  for (int i = 0; i < seq_len; i++) {
    last_.ticks[i] = mem_[i].tick;
    last_.actions[i] = mem_[i].action;
    last_.predicted[i] = mem_[i].predicted;
  }
  recombinations_++;

  // Emit a mutation seed from the recombined sequence.
  uint32_t seed = 0;
  for (int i = 0; i < seq_len; i++) {
    seed ^= (last_.actions[i] + 0x9E3779B9u) ^ (last_.ticks[i] * 0x85EBCA6Bu);
    seed ^= ((uint32_t)(last_.predicted[i] * 1000.0f) << (i % 16));
  }
  return seed;
}

}  // namespace omega
