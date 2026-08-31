#pragma once
#include "../../genesis/primordial_weights/primordial_weights.h"
#include <cstdint>

namespace omega {

// Neuroevolution of topology: a genetic algorithm over Network edge sets.
// Fitness is delegated to the reservoir substrate (rp_fitness). The GA itself
// does not understand dynamics — it only searchs topology/weight space.
struct GAParams {
  float mutation_rate = 0.15f;  // probability an individual is mutated
  float struct_prob = 0.30f;    // fraction of mutations that alter topology
  float lr = 0.20f;             // weight-perturbation scale
  uint32_t seed = 0x1234u;
};

class SelfWiringGA {
 public:
  static constexpr uint16_t POP = 10;
  static constexpr uint16_t GENS_CAP = 400;

  SelfWiringGA(const Network& seed, uint16_t n_nodes, uint32_t rng);

  void set_params(const GAParams& p) { params_ = p; }
  const GAParams& params() const { return params_; }

  void step();  // advance one generation

  const Network& best() const { return best_; }
  float best_fitness() const { return best_fit_; }
  float mean_fitness() const { return mean_fit_; }
  uint32_t generation() const { return gen_; }

  // Population diversity: mean pairwise edge-set difference (Jaccard-style).
  float diversity() const;

 private:
  Network pop_[POP];
  float fit_[POP];
  Network best_;
  float best_fit_ = -1e30f;
  float mean_fit_ = 0.0f;
  GAParams params_;
  uint16_t n_nodes_;
  uint32_t rng_;
  uint32_t gen_ = 0;

  uint32_t rnd();
  float rndf();

  void mutate(Network& net);
  void crossover(const Network& a, const Network& b, Network& out);
  float evaluate(const Network& net);
};

}  // namespace omega
