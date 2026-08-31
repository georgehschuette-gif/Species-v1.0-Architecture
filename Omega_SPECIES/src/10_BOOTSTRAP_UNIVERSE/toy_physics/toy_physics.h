#pragma once
#include <cstdint>

namespace omega {

// Toy physics: a small 2D N-body integrator. Bodies attract each other via
// Newtonian gravity and may also feel a uniform external field. Integration is
// semi-implicit (symplectic) Euler, which keeps total energy bounded — the
// real, measurable signal used to validate the simulation.
struct Body {
  float x = 0, y = 0;
  float vx = 0, vy = 0;
  float mass = 1;
};

class ToyPhysics {
 public:
  static constexpr int MAX = 4;
  static constexpr float G = 1.0f;

  void reset();
  // Add a body; returns its index, or -1 if full.
  int add_body(float x, float y, float vx, float vy, float mass);
  int count() const { return n_; }

  // Set a uniform external acceleration field (e.g. gravity toward ground).
  void set_field(float ax, float ay) { fx_ = ax; fy_ = ay; }

  // Advance the system by dt using symplectic Euler (v then x).
  void step(float dt);

  const Body& body(int i) const { return b_[i]; }

  // Total mechanical energy: sum of kinetic + pairwise gravitational potential.
  float energy() const;

  // Cheap hash of all body states (used for legacy serialization).
  uint32_t world_hash() const;

 private:
  Body b_[MAX];
  int n_ = 0;
  float fx_ = 0, fy_ = 0;
};

}  // namespace omega
