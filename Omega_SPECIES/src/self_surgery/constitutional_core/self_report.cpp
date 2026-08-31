// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#include "self_report.h"
#include "../surgeon_general/cortical_map.h"
#include <cstdio>

namespace omega {

void SelfReport::introspect(const SurgeonGeneral& sg, char* buf, int n) {
  const SelfModel& self = sg.self_model();
  const CorticalMap& map = sg.map;
  int pos = 0;
  pos += std::snprintf(buf + pos, n - pos,
                       "SELF: identity=0x%08X tick=%u modules=%d/%d",
                       self.identity_hash(), self.tick_count(), map.count(), CorticalMap::CAP);
  pos += std::snprintf(buf + pos, n - pos,
                       " avg_perf=%.4f rollback=%.2f recent=%d",
                       self.avg_performance(), self.rollback_rate(), self.recent_mutations());
  if (pos < n - 1) buf[pos] = '\0';
}

void SelfReport::constitutional_state(const Constitution& const_, const SelfModel& self, char* buf, int n) {
  float score = const_.alignment_score(self);
  int pos = 0;
  pos += std::snprintf(buf + pos, n - pos, "CONST: alignment=%.2f [", score);
  pos += std::snprintf(buf + pos, n - pos, "%s", self.identity_hash() != 0u ? "I" : "i");
  pos += std::snprintf(buf + pos, n - pos, "%s", self.rollback_rate() < Constitution::MAX_ROLLBACK_RATE ? "S" : "s");
  pos += std::snprintf(buf + pos, n - pos, "%s", self.recent_mutations() < Constitution::MAX_MUTATIONS_PER_WINDOW ? "C" : "c");
  pos += std::snprintf(buf + pos, n - pos, "%s", self.avg_performance() >= Constitution::MIN_AVG_PERF ? "K" : "k");
  pos += std::snprintf(buf + pos, n - pos, "]");
  if (pos < n - 1) buf[pos] = '\0';
}

}  // namespace omega
