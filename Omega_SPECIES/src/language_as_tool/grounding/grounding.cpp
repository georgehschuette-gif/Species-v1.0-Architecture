// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#include "grounding.h"
#include <cmath>
#include <cstring>
#include <mutex>
#include <future>
#include <unordered_map>

namespace omega {

float Grounding::dist(const float* a, const float* b) {
  float s = 0.0f;
  for (int i = 0; i < D; i++) {
    float d = a[i] - b[i];
    s += d * d;
  }
  return std::sqrt(s);
}

uint32_t Grounding::find_unlocked(const float* state, float thr) const {
  if (state == nullptr) return 0;
  uint32_t target_bucket = lsh_.bucket(state);
  float best = thr;
  uint32_t id = 0;
  for (const auto& e : entries_) {
    if (e.bucket == target_bucket || (e.bucket ^ target_bucket) <= 3u) {
      float d = dist(e.s, state);
      if (d <= best) {
        best = d;
        id = e.id;
      }
    }
  }
  return id;
}

uint32_t Grounding::find(const float* state, float thr) const {
  if (state == nullptr) return 0;
  std::shared_lock<std::shared_mutex> lk(mutex_);
  return find_unlocked(state, thr);
}

uint32_t Grounding::bind(const float* state, float thr) {
  if (state == nullptr) return 0;
  for (int i = 0; i < D; i++)
    if (!std::isfinite(state[i])) return 0;

  uint32_t bid = lsh_.bucket(state);

  {
    std::shared_lock<std::shared_mutex> lk(mutex_);
    uint32_t existing = find_unlocked(state, thr);
    if (existing != 0) {
      for (auto& e : entries_)
        if (e.id == existing) { e.uses++; break; }
      return existing;
    }
  }

  std::unique_lock<std::shared_mutex> lk(mutex_);
  uint32_t existing = find_unlocked(state, thr);
  if (existing != 0) {
    for (auto& e : entries_)
      if (e.id == existing) { e.uses++; break; }
    return existing;
  }

  entries_.emplace_back();
  Entry& e = entries_.back();
  e.id = next_id_++;
  memcpy(e.s, state, sizeof(float) * D);
  e.uses = 1;
  e.bucket = bid;
  return e.id;
}

bool Grounding::lookup(uint32_t id, float* out) const {
  std::shared_lock<std::shared_mutex> lk(mutex_);
  for (const auto& e : entries_)
    if (e.id == id) {
      memcpy(out, e.s, sizeof(float) * D);
      return true;
    }
  return false;
}

std::vector<uint32_t> Grounding::bind_batch(const std::vector<const float*>& states, float thr) {
  size_t n = states.size();
  std::vector<uint32_t> results(n);

  std::unordered_map<uint32_t, std::vector<size_t>> bucket_groups;
  for (size_t i = 0; i < n; i++) {
    uint32_t b = lsh_.bucket(states[i]);
    bucket_groups[b].push_back(i);
  }

  std::vector<std::future<void>> futures;
  futures.reserve(bucket_groups.size());

  for (auto& [bucket, indices] : bucket_groups) {
    futures.push_back(std::async(std::launch::async,
      [this, &states, &results, indices, thr]() {
        for (size_t idx : indices) {
          results[idx] = this->bind(states[idx], thr);
        }
      }));
  }

  for (auto& f : futures) f.get();
  return results;
}

}  // namespace omega