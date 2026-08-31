#pragma once
#include "../../genesis/primordial_weights/primordial_weights.h"

namespace omega {

// Live topology graph of the agent's mind: each module is a sub-network with a
// measured performance. The surgeon operates on these.
struct Module {
  int id = 0;
  Network net;
  float perf = 0.0f;
};

class CorticalMap {
 public:
  static constexpr int CAP = 6;

  void add(int id, const Network& net, float perf) {
    if (n_ >= CAP) return;
    mods_[n_].id = id;
    pw_copy(&mods_[n_].net, &net);
    mods_[n_].perf = perf;
    n_++;
  }

  int weakest() const {  // index of the lowest-performing module
    if (n_ == 0) return -1;
    int w = 0;
    for (int i = 1; i < n_; i++)
      if (mods_[i].perf < mods_[w].perf) w = i;
    return w;
  }

  int count() const { return n_; }
  Module& at(int i) { return mods_[i]; }
  const Module& at(int i) const { return mods_[i]; }
  void set_perf(int i, float p) { mods_[i].perf = p; }

 private:
  Module mods_[CAP];
  int n_ = 0;
};

}  // namespace omega
