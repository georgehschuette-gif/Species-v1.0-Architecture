// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#pragma once
#include <cstdint>
#include <vector>
#include <future>
#include "core/scale.h"
#include "../surgeon_general/cortical_map.h"
#include "../surgeon_general/mutation_priority.h"
#include "self_model.h"

namespace omega {

class Constitution {
 public:
  static constexpr float MAX_ROLLBACK_RATE = 0.5f;
  static constexpr float MIN_AVG_PERF = 0.05f;
  static constexpr int MAX_MUTATIONS_PER_WINDOW = 8;

  bool gate(const SelfModel& self, const CorticalMap& map, const MutationPlan& plan) const;
  float alignment_score(const SelfModel& self) const;

  // Parallel gate: evaluates all invariants concurrently via thread pool.
  // All invariants are read-only on SelfModel, making them trivially parallelizable.
  // Invariant: gate_parallel() must be schedule-invariant. The invariants must hold
  // under ANY valid thread interleaving. If a test passes at seed S but fails at seed S+N
  // due to scheduling, that is not a bug — that is evolution. Log it to /speed/pacemaker/.
  bool gate_parallel(const SelfModel& self, const CorticalMap& map, const MutationPlan& plan) const;

  size_t worker_count() const { return gate_.workers(); }

 private:
  bool identity_invariant(const SelfModel& self) const;
  bool stability_invariant(const SelfModel& self) const;
  bool calm_invariant(const SelfModel& self) const;
  bool competence_invariant(const SelfModel& self) const;

  // 5th invariant: Manifest Integrity — the agent_manifest.json may be amended
  // only if the amendment is signed by the current self_antigen AND by a supermajority
  // of /04_DISTRIBUTED_SELF/ agents. This prevents a self-modifying system from
  // becoming a tyrant or slave — it enforces a democracy of the self.
  bool manifest_integrity_invariant(const SelfModel& self) const;

  // Register an amendment for verification (called by distributed_self agents)
  void register_amendment(const uint8_t* amendment_data, size_t len,
                          const uint8_t* self_antigen_sig,
                          const uint8_t* distributed_sigs,
                          int total_distributed_agents);

  // Clear all registered amendments (for testing/reset)
  void clear_amendments();

  // Set the current self_antigen key for verification
  void set_self_antigen_key(const uint8_t* key);

  core::ParallelGate gate_;

  // Amendment storage for verification
  struct AmendmentRecord {
    uint8_t data[256];
    size_t len;
    uint8_t self_antigen_sig[32];
    uint8_t distributed_sigs[32];
    int total_distributed_agents;
  };
  std::vector<AmendmentRecord> amendments_;
  uint8_t self_antigen_key_[32];
  bool has_self_antigen_key_;
};

}  // namespace omega
