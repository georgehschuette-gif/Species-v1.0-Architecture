#include "platform.h"

// Compiled only when cross-compiling for the Teensy 4.1 (TARGET_TEENSY).
// Kept dependency-free: raw UART + hardware millis().
#ifdef TARGET_TEENSY
#include <Arduino.h>

namespace omega {

void host_init() { Serial.begin(115200); }

void host_log(const char* msg) { Serial.println(msg); }

uint32_t host_millis() { return static_cast<uint32_t>(millis()); }

void host_sleep_ms(uint32_t ms) { delay(ms); }

}  // namespace omega
#endif  // TARGET_TEENSY
