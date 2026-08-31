#include "hivemind.h"

namespace omega {

static uint32_t xrng(uint32_t& s) {
  s ^= s << 13;
  s ^= s >> 17;
  s ^= s << 5;
  return s;
}

void Hivemind::init() {
  rng_ = 0x5A17u;
  term_ = 0;
  leader_ = -1;
  entry_committed_ = false;
  pending_entry_ = false;
  for (int i = 0; i < N; i++) {
    nodes_[i].role = FOLLOWER;
    nodes_[i].term = 0;
    nodes_[i].voted_for = -1;
    nodes_[i].log_len = 0;
    nodes_[i].commit_index = 0;
    nodes_[i].votes = 0;
    nodes_[i].election_timer = 5 + i * 3;  // staggered so node 0 wakes first
  }
}

void Hivemind::propose_entry() { pending_entry_ = true; }

void Hivemind::tick() {
  // --- Leader: replicate the proposed entry, then heartbeat followers -------
  for (int i = 0; i < N; i++) {
    if (nodes_[i].role != LEADER) continue;
    if (pending_entry_) {
      if (nodes_[i].log_len == 0) {
        nodes_[i].log_len = 1;
      }
      for (int j = 0; j < N; j++) {
        if (j != i && nodes_[j].term == nodes_[i].term && nodes_[j].log_len == 0)
          nodes_[j].log_len = 1;
      }
      int repl = 1;  // leader counts itself
      for (int j = 0; j < N; j++)
        if (j != i && nodes_[j].term == nodes_[i].term && nodes_[j].log_len >= 1) repl++;
      if (repl >= MAJORITY) {
        nodes_[i].commit_index = 1;
        entry_committed_ = true;
        pending_entry_ = false;
      }
    }
    // Heartbeat: refresh followers' election timers so they don't contest.
    for (int j = 0; j < N; j++)
      if (j != i) {
        nodes_[j].election_timer = 6;
        if (nodes_[j].term < nodes_[i].term) nodes_[j].term = nodes_[i].term;
      }
  }

  // --- Followers / candidates: election timers ----------------------------
  for (int i = 0; i < N; i++) {
    if (nodes_[i].role == LEADER) continue;
    nodes_[i].election_timer--;
    if (nodes_[i].election_timer > 0) continue;

    // Timeout -> start an election.
    nodes_[i].role = CANDIDATE;
    nodes_[i].term++;
    nodes_[i].voted_for = i;
    nodes_[i].votes = 1;
    nodes_[i].election_timer = 5 + (int)(xrng(rng_) % 5);
    for (int j = 0; j < N; j++) {
      if (j == i) continue;
      if (nodes_[j].term <= nodes_[i].term && nodes_[j].voted_for == -1) {
        nodes_[j].voted_for = i;
        if (nodes_[j].term < nodes_[i].term) nodes_[j].term = nodes_[i].term;
        nodes_[i].votes++;
      }
    }
    if (nodes_[i].votes >= MAJORITY) {
      nodes_[i].role = LEADER;
      leader_ = i;
      if (nodes_[i].log_len == 0) nodes_[i].log_len = 1;
    }
  }

  for (int i = 0; i < N; i++)
    if (nodes_[i].term > term_) term_ = nodes_[i].term;
}

}  // namespace omega
