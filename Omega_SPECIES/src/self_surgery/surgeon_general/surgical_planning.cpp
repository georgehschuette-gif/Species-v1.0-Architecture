#include "surgical_planning.h"
#include "../../liquid_time/reservoir_pool/reservoir_pool.h"

namespace omega {

static const float kTrain[5] = {0.15f, 0.40f, 0.65f, 0.90f, 0.30f};
static const float kVal[5]   = {0.55f, 0.20f, 0.80f, 0.05f, 0.70f};

float SurgicalPlanning::evaluate(const Network& net, int set) const {
  const float* stim = (set == 0) ? kTrain : kVal;
  return rp_fitness(&net, stim, 5, 24, 0.05f, 0.10f);
}

void SurgicalPlanning::apply(Network& net, const MutationPlan& plan, float aggressiveness) {
  uint32_t s = plan.seed ? plan.seed : 1u;
  float scale = 0.2f + 1.0f * aggressiveness;
  if (plan.kind == 0 && net.n_edges < PW_MAX_EDGES) {
    uint16_t a = (uint16_t)(rnd(s) % net.n_nodes);
    uint16_t b = (uint16_t)(rnd(s) % net.n_nodes);
    if (a != b && pw_weight(&net, a, b) == 0.0f) {
      net.edges[net.n_edges].src = a;
      net.edges[net.n_edges].dst = b;
      net.edges[net.n_edges].w = (rnd(s) - 0.5f) * scale;
      net.n_edges++;
    }
  } else if (net.n_edges > 0) {
    uint16_t k = (uint16_t)(rnd(s) % net.n_edges);
    net.edges[k].w += (rnd(s) - 0.5f) * scale;
    if (net.edges[k].w > 3.0f) net.edges[k].w = 3.0f;
    if (net.edges[k].w < -3.0f) net.edges[k].w = -3.0f;
  }
}

float SurgicalPlanning::dry_run(const Network& net, const MutationPlan& plan) {
  Network copy;
  pw_copy(&copy, &net);
  apply(copy, plan);
  return evaluate(copy, 0);  // predicted on training set
}

}  // namespace omega
