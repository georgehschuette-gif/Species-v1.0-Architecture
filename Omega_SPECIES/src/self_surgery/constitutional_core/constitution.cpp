// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#include "constitution.h"
#include "self_model.h"
#include "manifest_integrity.h"
#include <algorithm>
#include <functional>

namespace omega {

bool Constitution::identity_invariant(const SelfModel& self) const {
  return self.identity_hash() != 0u;
}

bool Constitution::stability_invariant(const SelfModel& self) const {
  return self.rollback_rate() < MAX_ROLLBACK_RATE;
}

bool Constitution::calm_invariant(const SelfModel& self) const {
  return self.recent_mutations() < MAX_MUTATIONS_PER_WINDOW;
}

bool Constitution::competence_invariant(const SelfModel& self) const {
  return self.avg_performance() >= MIN_AVG_PERF;
}

bool Constitution::manifest_integrity_invariant(const SelfModel& self) const {
  // This invariant enforces the Constitutional Amendment Protocol:
  // agent_manifest.json may be amended only if the amendment is signed by
  // the current self_antigen AND by a supermajority of distributed_self agents.

  // For this implementation, we check:
  // 1. The self has a valid identity (self_antigen)
  // 2. No unauthorized mutations have occurred (identity hash stable)
  // 3. In a full deployment, this would verify cryptographic signatures

  // Check that the self has a valid identity hash
  if (self.identity_hash() == 0u) {
    return false;  // No valid identity
  }

  // Check that the identity has not been corrupted
  // (In a full implementation, this would verify the amendment chain)
  if (self.rollback_rate() >= MAX_ROLLBACK_RATE) {
    return false;  // Too many rollbacks indicates potential corruption
  }

  // In a full implementation with distributed_self agents:
  // 1. Read agent_manifest.json
  // 2. Parse amendment chain
  // 3. For each amendment:
  //    a. Verify self_antigen signature using ManifestIntegrity::verify_amendment
  //    b. Verify supermajority of distributed_self signatures
  //    c. Verify amendment hash integrity
  // 4. If any amendment fails verification, return false

  // For now, we return true if basic identity checks pass
  // The full cryptographic verification requires:
  // - File I/O to read agent_manifest.json
  // - JSON parsing to extract amendment records
  // - Access to distributed_self agent signatures
  return true;
}

bool Constitution::gate(const SelfModel& self, const CorticalMap& map, const MutationPlan& plan) const {
  (void)map;
  (void)plan;
  if (!identity_invariant(self)) return false;
  if (!stability_invariant(self)) return false;
  if (!calm_invariant(self)) return false;
  if (!competence_invariant(self)) return false;
  if (!manifest_integrity_invariant(self)) return false;
  return true;
}

bool Constitution::gate_parallel(const SelfModel& self, const CorticalMap& map, const MutationPlan& plan) const {
  (void)map;
  (void)plan;

  // Evaluate all 5 read-only invariants concurrently via thread pool
  // This includes the 5th invariant: Manifest Integrity
  std::vector<std::function<bool()>> preds;
  preds.emplace_back([this, &self]() { return identity_invariant(self); });
  preds.emplace_back([this, &self]() { return stability_invariant(self); });
  preds.emplace_back([this, &self]() { return calm_invariant(self); });
  preds.emplace_back([this, &self]() { return competence_invariant(self); });
  preds.emplace_back([this, &self]() { return manifest_integrity_invariant(self); });

  return gate_.all_of(preds);
}

float Constitution::alignment_score(const SelfModel& self) const {
  float score = 0.0f;
  float weight = 0.0f;
  if (identity_invariant(self)) { score += 1.0f; weight += 1.0f; }
  if (stability_invariant(self)) { score += 1.0f; weight += 1.0f; }
  if (calm_invariant(self)) { score += 1.0f; weight += 1.0f; }
  if (competence_invariant(self)) { score += 1.0f; weight += 1.0f; }
  if (manifest_integrity_invariant(self)) { score += 1.0f; weight += 1.0f; }
  if (weight == 0.0f) return 0.0f;
  return score / weight;
}

}  // namespace omega
