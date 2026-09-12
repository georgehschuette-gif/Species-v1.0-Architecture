// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#include "memory_store.h"
#include <cstring>
#include <mutex>

namespace omega {

void MemoryStore::init(const LSH* lsh, float decay_per_step, float consolidate_thr,
                       size_t initial_cap) {
  lsh_ = lsh;
  decay_ = decay_per_step;
  consol_thr_ = consolidate_thr;
  mem_.clear();
  mem_.reserve(initial_cap);
}

int MemoryStore::find_slot(uint32_t tag, uint32_t /*bucket*/) const {
  for (size_t i = 0; i < mem_.size(); i++)
    if (mem_[i].used && mem_[i].tag == tag) return (int)i;
  return -1;
}

void MemoryStore::store(uint32_t tag, const float* vec, float init_strength) {
  if (!lsh_) return;
  uint32_t b = lsh_->bucket(vec);
  std::unique_lock<std::shared_mutex> lk(mutex_);

  int idx = find_slot(tag, b);
  if (idx >= 0) {
    mem_[idx].strength += init_strength;
    if (mem_[idx].strength > 10.0f) mem_[idx].strength = 10.0f;
    memcpy(mem_[idx].vec, vec, sizeof(float) * V);
    mem_[idx].access_counter = next_access_++;
    return;
  }

  if (mem_.empty() || mem_.size() < mem_.capacity()) {
    mem_.emplace_back();
    idx = (int)(mem_.size() - 1);
  } else {
    // Grow the vector (replaces fixed CAP eviction)
    mem_.emplace_back();
    idx = (int)(mem_.size() - 1);
  }
  mem_[idx].used = true;
  mem_[idx].bucket = b;
  mem_[idx].tag = tag;
  memcpy(mem_[idx].vec, vec, sizeof(float) * V);
  mem_[idx].strength = init_strength;
  mem_[idx].access_counter = next_access_++;
}

float MemoryStore::recall(uint32_t tag, float* out) const {
  std::shared_lock<std::shared_mutex> lk(mutex_);
  for (size_t i = 0; i < mem_.size(); i++) {
    if (mem_[i].used && mem_[i].tag == tag) {
      if (out) memcpy(out, mem_[i].vec, sizeof(float) * V);
      return mem_[i].strength;
    }
  }
  return 0.0f;
}

void MemoryStore::decay_all(float dt) {
  std::unique_lock<std::shared_mutex> lk(mutex_);
  for (size_t i = 0; i < mem_.size(); i++) {
    if (!mem_[i].used) continue;
    mem_[i].strength *= (1.0f - decay_ * dt);
    if (mem_[i].strength < 0.001f) mem_[i].used = false;
  }
}

int MemoryStore::prune(float min_strength) {
  std::unique_lock<std::shared_mutex> lk(mutex_);
  int removed = 0;
  for (size_t i = 0; i < mem_.size(); i++) {
    if (mem_[i].used && mem_[i].strength < min_strength) {
      mem_[i].used = false;
      removed++;
    }
  }
  return removed;
}

void MemoryStore::consolidate_to(MemoryStore& dst) const {
  std::shared_lock<std::shared_mutex> lk(mutex_);
  for (size_t i = 0; i < mem_.size(); i++) {
    if (mem_[i].used && mem_[i].strength >= consol_thr_) {
      dst.store(mem_[i].tag, mem_[i].vec, mem_[i].strength * 0.5f);
    }
  }
}

int MemoryStore::count() const {
  std::shared_lock<std::shared_mutex> lk(mutex_);
  int c = 0;
  for (size_t i = 0; i < mem_.size(); i++)
    if (mem_[i].used) c++;
  return c;
}

float MemoryStore::avg_strength() const {
  std::shared_lock<std::shared_mutex> lk(mutex_);
  float s = 0.0f;
  int c = 0;
  for (size_t i = 0; i < mem_.size(); i++)
    if (mem_[i].used) {
      s += mem_[i].strength;
      c++;
    }
  return c ? s / (float)c : 0.0f;
}

}  // namespace omega
