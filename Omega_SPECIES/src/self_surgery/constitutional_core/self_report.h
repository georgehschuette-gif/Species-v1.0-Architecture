// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#pragma once
#include <cstdint>
#include "../surgeon_general/surgeon_general.h"

namespace omega {

class SelfReport {
 public:
  static void introspect(const SurgeonGeneral& sg, char* buf, int n);
  static void constitutional_state(const Constitution& const_, const SelfModel& self, char* buf, int n);
};

}  // namespace omega
