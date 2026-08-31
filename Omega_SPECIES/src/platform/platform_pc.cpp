#include "platform.h"
#include <cstdio>
#include <chrono>
#include <thread>

namespace omega {

void host_init() {}

void host_log(const char* msg) {
  std::printf("%s\n", msg);
  std::fflush(stdout);
}

uint32_t host_millis() {
  using namespace std::chrono;
  return static_cast<uint32_t>(
      duration_cast<milliseconds>(steady_clock::now().time_since_epoch()).count());
}

void host_sleep_ms(uint32_t ms) {
  std::this_thread::sleep_for(std::chrono::milliseconds(ms));
}

}  // namespace omega
