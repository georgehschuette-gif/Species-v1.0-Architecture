#include "grounding.h"
#include <cmath>
#include <cstring>

namespace omega {

float Grounding::dist(const float* a, const float* b) {
  float s = 0.0f;
  for (int i = 0; i < D; i++) {
    float d = a[i] - b[i];
    s += d * d;
  }
  return std::sqrt(s);
}

uint32_t Grounding::find(const float* state, float thr) const {
  if (state == nullptr) return 0;
  float best = thr;
  uint32_t id = 0;
  for (int i = 0; i < n_; i++) {
    float d = dist(e_[i].s, state);
    if (d <= best) {
      best = d;
      id = e_[i].id;
    }
  }
  return id;
}

uint32_t Grounding::bind(const float* state, float thr) {
  if (state == nullptr) return 0;
  for (int i = 0; i < D; i++)
    if (!std::isfinite(state[i])) return 0;  // reject non-finite states
  uint32_t id = find(state, thr);
  if (id != 0) {
    for (int i = 0; i < n_; i++)
      if (e_[i].id == id) {
        e_[i].uses++;
        break;
      }
    return id;
  }
  if (n_ >= CAP) return 0;
  Entry& e = e_[n_++];
  e.id = next_id_++;
  memcpy(e.s, state, sizeof(float) * D);
  e.uses = 1;
  return e.id;
}

bool Grounding::lookup(uint32_t id, float* out) const {
  for (int i = 0; i < n_; i++)
    if (e_[i].id == id) {
      memcpy(out, e_[i].s, sizeof(float) * D);
      return true;
    }
  return false;
}

}  // namespace omega
