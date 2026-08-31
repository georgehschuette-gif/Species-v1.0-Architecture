#pragma once
#include "../../immune_system/self_antigen/self_antigen.h"

namespace omega {

// Coherence preservation: the soul's integrity check. The stable genotype core
// of the self-antigen must never change — if it does, identity is compromised.
class CoherencePreservation {
 public:
  void set_reference(const SelfAntigen& ref) {
    ref_ = ref;
    has_ref_ = true;
  }

  // True iff the current antigen's genotype core still matches the reference.
  bool verify(const SelfAntigen& current) const {
    return has_ref_ && ref_.is_self(current);
  }

  // 1.0 when coherent, 0.0 on a genotype breach. Phenotype drift is reported
  // separately so slow self-evolution is distinguishable from a break.
  float coherence_score(const SelfAntigen& current) const {
    if (!has_ref_) return 0.0f;
    return ref_.is_self(current) ? 1.0f : 0.0f;
  }

  int phenotype_drift(const SelfAntigen& current) const {
    return has_ref_ ? ref_.phenotype_drift(current) : 0;
  }

 private:
  SelfAntigen ref_;
  bool has_ref_ = false;
};

}  // namespace omega
