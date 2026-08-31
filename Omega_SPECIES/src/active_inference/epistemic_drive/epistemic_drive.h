#pragma once
#include "../free_energy/free_energy.h"

namespace omega {

// Epistemic drive: the expected information gain (in nats) of a hypothetical
// observation — i.e. how much it would reduce uncertainty about the belief.
// For a linear model the precision gain is constant, so we scale it by the
// novelty of the predicted state (explore what is currently uncertain).
float information_gain(const AIModel& m, const float* predicted_state);

}  // namespace omega
