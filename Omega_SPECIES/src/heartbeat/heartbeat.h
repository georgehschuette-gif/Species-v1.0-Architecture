#pragma once
#include <cstdint>

namespace omega {

// 10 Hz global clock. Single-threaded on the MCU, so phase_lock is held
// trivially; the field exists so multi-agent (Phase 4) can widen it later.
struct Heartbeat {
  static constexpr uint32_t HZ = 10;
  static constexpr uint32_t PERIOD_MS = 1000 / HZ;  // 100 ms

  uint32_t tick = 0;            // global time reference (tick_counter)
  bool phase_locked = false;    // phase_lock barrier
  uint32_t calories_used = 0;   // metabolic_monitor accumulator
  uint32_t next_ms = 0;         // internal scheduler cursor

  static constexpr uint32_t CALORIC_BUDGET = 100;  // info-calories / tick

  void init(uint32_t now_ms);
  bool step(uint32_t now_ms);   // true when a new tick elapsed
  void spend(uint32_t calories);  // metabolic_monitor: log consumption
  bool over_budget() const { return calories_used > CALORIC_BUDGET; }
};

}  // namespace omega
