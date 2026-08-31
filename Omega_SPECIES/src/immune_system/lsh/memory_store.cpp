#include "memory_store.h"
#include <cstring>

namespace omega {

void MemoryStore::init(const LSH* lsh, float decay_per_step, float consolidate_thr) {
  lsh_ = lsh;
  decay_ = decay_per_step;
  consol_thr_ = consolidate_thr;
  for (int i = 0; i < CAP; i++) {
    mem_[i].used = false;
    mem_[i].strength = 0.0f;
    mem_[i].bucket = 0;
    mem_[i].tag = 0;
  }
}

int MemoryStore::count() const {
  int c = 0;
  for (int i = 0; i < CAP; i++)
    if (mem_[i].used) c++;
  return c;
}

void MemoryStore::store(uint32_t tag, const float* vec, float init_strength) {
  if (!lsh_) return;
  uint32_t b = lsh_->bucket(vec);

  // Strengthen an existing memory with the same tag.
  for (int i = 0; i < CAP; i++) {
    if (mem_[i].used && mem_[i].tag == tag) {
      mem_[i].strength += init_strength;
      if (mem_[i].strength > 10.0f) mem_[i].strength = 10.0f;
      memcpy(mem_[i].vec, vec, sizeof(float) * V);
      return;
    }
  }

  // Find a free slot near the bucket; else evict the weakest.
  int idx = -1;
  for (int i = 0; i < CAP; i++) {
    int probe = (int)((b + i) % CAP);
    if (!mem_[probe].used) {
      idx = probe;
      break;
    }
  }
  if (idx < 0) {
    idx = 0;
    for (int i = 1; i < CAP; i++)
      if (mem_[i].strength < mem_[idx].strength) idx = i;
  }
  mem_[idx].used = true;
  mem_[idx].bucket = b;
  mem_[idx].tag = tag;
  memcpy(mem_[idx].vec, vec, sizeof(float) * V);
  mem_[idx].strength = init_strength;
}

float MemoryStore::recall(uint32_t tag, float* out) const {
  for (int i = 0; i < CAP; i++) {
    if (mem_[i].used && mem_[i].tag == tag) {
      if (out) memcpy(out, mem_[i].vec, sizeof(float) * V);
      return mem_[i].strength;
    }
  }
  return 0.0f;
}

void MemoryStore::decay_all(float dt) {
  for (int i = 0; i < CAP; i++) {
    if (!mem_[i].used) continue;
    mem_[i].strength *= (1.0f - decay_ * dt);
    if (mem_[i].strength < 0.001f) mem_[i].used = false;
  }
}

int MemoryStore::prune(float min_strength) {
  int removed = 0;
  for (int i = 0; i < CAP; i++) {
    if (mem_[i].used && mem_[i].strength < min_strength) {
      mem_[i].used = false;
      removed++;
    }
  }
  return removed;
}

void MemoryStore::consolidate_to(MemoryStore& dst) const {
  for (int i = 0; i < CAP; i++) {
    if (mem_[i].used && mem_[i].strength >= consol_thr_) {
      dst.store(mem_[i].tag, mem_[i].vec, mem_[i].strength * 0.5f);
    }
  }
}

float MemoryStore::avg_strength() const {
  float s = 0.0f;
  int c = 0;
  for (int i = 0; i < CAP; i++)
    if (mem_[i].used) {
      s += mem_[i].strength;
      c++;
    }
  return c ? s / (float)c : 0.0f;
}

}  // namespace omega
