#include "toy_physics.h"
#include <cmath>
#include <cstring>
#include <cfloat>

namespace omega {

void ToyPhysics::reset() {
  n_ = 0;
  fx_ = 0; fy_ = 0;
  std::memset(b_, 0, sizeof(b_));
}

int ToyPhysics::add_body(float x, float y, float vx, float vy, float mass) {
  if (n_ >= MAX) return -1;
  if (!std::isfinite(mass) || mass <= 0.0f) mass = 1.0f;
  const float MAX_VAL = 1e6f;
  if (!std::isfinite(x) || std::abs(x) > MAX_VAL) x = 0.0f;
  if (!std::isfinite(y) || std::abs(y) > MAX_VAL) y = 0.0f;
  if (!std::isfinite(vx) || std::abs(vx) > MAX_VAL) vx = 0.0f;
  if (!std::isfinite(vy) || std::abs(vy) > MAX_VAL) vy = 0.0f;
  Body& b = b_[n_];
  b.x = x; b.y = y; b.vx = vx; b.vy = vy; b.mass = mass;
  return n_++;
}

void ToyPhysics::step(float dt) {
  // Accumulate accelerations from current positions (mutual gravity + field).
  float ax[MAX], ay[MAX];
  for (int i = 0; i < n_; i++) {
    float axi = fx_, ayi = fy_;
    for (int j = 0; j < n_; j++) {
       if (i == j) continue;
       float dx = b_[j].x - b_[i].x;
       float dy = b_[j].y - b_[i].y;
       float r2 = dx * dx + dy * dy;
       if (!std::isfinite(r2) || r2 < 1e-6f) r2 = 1e-6f;
      float r = std::sqrt(r2);
      float f = G * b_[j].mass / (r2 * r);  // |a| = G*m_j / r^2, direction (dx,dy)/r
      axi += f * dx;
      ayi += f * dy;
    }
    ax[i] = axi;
    ay[i] = ayi;
  }
  // Symplectic Euler: update velocity, then position.
  for (int i = 0; i < n_; i++) {
    b_[i].vx += ax[i] * dt;
    b_[i].vy += ay[i] * dt;
    b_[i].x += b_[i].vx * dt;
    b_[i].y += b_[i].vy * dt;
  }
}

float ToyPhysics::energy() const {
  float ke = 0.0f;
  for (int i = 0; i < n_; i++) ke += 0.5f * b_[i].mass * (b_[i].vx * b_[i].vx + b_[i].vy * b_[i].vy);
  float pe = 0.0f;
  for (int i = 0; i < n_; i++)
    for (int j = i + 1; j < n_; j++) {
      float dx = b_[j].x - b_[i].x;
      float dy = b_[j].y - b_[i].y;
      float r = std::sqrt(dx * dx + dy * dy);
      if (r < 1e-6f) r = 1e-6f;
      pe += -G * b_[i].mass * b_[j].mass / r;
    }
  return ke + pe;
}

uint32_t ToyPhysics::world_hash() const {
  uint32_t h = 0x811C9DC5u;
  for (int i = 0; i < n_; i++) {
    auto mix = [&](float v) {
      int iv = (int)(v * 1000.0f);
      uint8_t b[4];
      b[0] = (uint8_t)(iv & 0xFF);
      b[1] = (uint8_t)((iv >> 8) & 0xFF);
      b[2] = (uint8_t)((iv >> 16) & 0xFF);
      b[3] = (uint8_t)((iv >> 24) & 0xFF);
      for (int k = 0; k < 4; k++) { h ^= b[k]; h *= 0x01000193u; }
    };
    mix(b_[i].x); mix(b_[i].y); mix(b_[i].vx); mix(b_[i].vy);
  }
  return h;
}

}  // namespace omega
