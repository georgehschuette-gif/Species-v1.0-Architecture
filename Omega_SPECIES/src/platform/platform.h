#pragma once
#include <cstdint>

// Host abstraction layer.
// PC target (default): stdio + std::chrono.
// Teensy target (TARGET_TEENSY): UART + hardware timer (see platform_teensy.cpp).
// The rest of the codebase talks only to this surface, so the same sources
// compile for both targets.

namespace omega {

void host_init();
void host_log(const char* msg);
uint32_t host_millis();
void host_sleep_ms(uint32_t ms);

}  // namespace omega
