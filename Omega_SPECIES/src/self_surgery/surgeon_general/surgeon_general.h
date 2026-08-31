// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#pragma once
#include "cortical_map.h"
#include "mutation_priority.h"
#include "surgical_planning.h"
#include "sterile_field.h"
#include "post_op_recovery.h"
#include "../../genesis/primordial_weights/primordial_weights.h"
#include "../constitutional_core/self_model.h"
#include "../constitutional_core/constitution.h"

namespace omega {

class SurgeonGeneral {
 public:
  CorticalMap map;

  void seed_modules(const Network& base);

  int operate(uint32_t rng, uint32_t tick);

  int applied() const { return applied_; }
  int rolled_back() const { return rolled_back_; }
  int skipped() const { return skipped_; }
  int constitutional_blocks() const { return constitutional_blocks_; }
  float aggressiveness() const { return aggressiveness_; }
  const SelfModel& self_model() const { return self_; }
  const Constitution& constitution() const { return const_; }

  void set_aggressiveness(float a) { aggressiveness_ = a; }
  void adapt();

 private:
  MutationPriority prio_;
  SurgicalPlanning plan_;
  SterileField field_;
  PostOpRecovery recovery_;
  SelfModel self_;
  Constitution const_;
  float aggressiveness_ = 0.5f;
  int applied_ = 0;
  int rolled_back_ = 0;
  int skipped_ = 0;
  int constitutional_blocks_ = 0;
};

}  // namespace omega
