#pragma once
#include "../../genesis/primordial_weights/primordial_weights.h"

namespace omega {

// Sterile field: locks a region during surgery and holds a checkpoint so the
// operation can be rolled back atomically if post-op results degrade.
class SterileField {
 public:
  void checkpoint(const Network& net) {
    pw_copy(&cp_, &net);
    has_ = true;
  }
  void lock() { locked_ = true; }
  void unlock() { locked_ = false; }
  bool locked() const { return locked_; }
  bool has_checkpoint() const { return has_; }

  // Restore the live network to the checkpoint.
  void rollback(Network& net) const { pw_copy(&net, &cp_); }

 private:
  Network cp_;
  bool has_ = false;
  bool locked_ = false;
};

}  // namespace omega
