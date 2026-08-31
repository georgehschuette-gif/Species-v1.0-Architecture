#include "heartbeat.h"

namespace omega {

void Heartbeat::init(uint32_t now_ms) {
  tick = 0;
  phase_locked = true;  // single-threaded: barrier trivially satisfied
  calories_used = 0;
  next_ms = now_ms + PERIOD_MS;
}

bool Heartbeat::step(uint32_t now_ms) {
  if (now_ms >= next_ms) {
    tick++;
    next_ms += PERIOD_MS;
    calories_used = 0;  // reset metabolic budget per tick
    return true;
  }
  return false;
}

void Heartbeat::spend(uint32_t calories) {
  calories_used += calories;
  // Phase 6 (ANOREXIA_NERVOSA) will throttle work when over_budget() is true.
}

}  // namespace omega
