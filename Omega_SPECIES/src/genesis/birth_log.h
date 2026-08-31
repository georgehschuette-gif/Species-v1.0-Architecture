#pragma once
#include <cstdint>

namespace omega {

struct BirthEvent {
  uint32_t t_ms;
  char msg[96];  // copied on record() — caller may reuse its buffer
};

// Ring-backed log of the first moments. Fixed capacity, zero heap use.
class BirthLog {
 public:
  static constexpr uint32_t CAP = 64;

  void record(uint32_t t_ms, const char* msg);
  void dump();                       // flush every event via host_log
  uint32_t count() const { return n_; }

 private:
  BirthEvent events_[CAP];
  uint32_t n_ = 0;
};

}  // namespace omega
