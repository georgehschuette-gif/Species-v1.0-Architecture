// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#pragma once
#include "../../genesis/primordial_weights/primordial_weights.h"
#include <vector>
#include <shared_mutex>
#include <mutex>

namespace omega {

struct Module {
  int id = 0;
  Network net;
  float perf = 0.0f;
};

// Growable, thread-safe cortical map. Replaces fixed CAP=6 with dynamic
// vector that expands on demand. Read-write locks allow concurrent reads
// while the surgeon operates.
class CorticalMap {
 public:
  void reserve(size_t cap) {
    std::unique_lock<std::shared_mutex> lk(mutex_);
    mods_.reserve(cap);
  }

  void add(int id, const Network& net, float perf) {
    std::unique_lock<std::shared_mutex> lk(mutex_);
    mods_.emplace_back();
    mods_.back().id = id;
    pw_copy(&mods_.back().net, &net);
    mods_.back().perf = perf;
  }

  int weakest() const {
    std::shared_lock<std::shared_mutex> lk(mutex_);
    if (mods_.empty()) return -1;
    int w = 0;
    for (size_t i = 1; i < mods_.size(); i++)
      if (mods_[i].perf < mods_[w].perf) w = (int)i;
    return w;
  }

  int count() const {
    std::shared_lock<std::shared_mutex> lk(mutex_);
    return (int)mods_.size();
  }

  Module& at(int i) {
    std::unique_lock<std::shared_mutex> lk(mutex_);
    return mods_[i];
  }

  const Module& at(int i) const {
    std::shared_lock<std::shared_mutex> lk(mutex_);
    return mods_[i];
  }

  void set_perf(int i, float p) {
    std::unique_lock<std::shared_mutex> lk(mutex_);
    if ((size_t)i < mods_.size()) mods_[i].perf = p;
  }

  size_t capacity() const {
    std::shared_lock<std::shared_mutex> lk(mutex_);
    return mods_.capacity();
  }

 private:
  std::vector<Module> mods_;
  mutable std::shared_mutex mutex_;
};

}  // namespace omega
