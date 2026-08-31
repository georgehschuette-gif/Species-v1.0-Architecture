#include "birth_log.h"
#include "platform.h"
#include <cstdio>

namespace omega {

void BirthLog::record(uint32_t t_ms, const char* msg) {
  if (n_ < CAP) {
    events_[n_].t_ms = t_ms;
    std::snprintf(events_[n_].msg, sizeof(events_[n_].msg), "%s", msg);
    n_++;
  }
}

void BirthLog::dump() {
  for (uint32_t i = 0; i < n_; i++) {
    char buf[128];
    std::snprintf(buf, sizeof(buf), "[t=%u ms] %s", events_[i].t_ms, events_[i].msg);
    host_log(buf);
  }
}

}  // namespace omega
