#pragma once
#include "../self_wiring/self_wiring.h"
#include "mutation_scheduler.h"
#include "learning_rate_oscillator.h"
#include "divergence_entropy.h"

namespace omega {

// The Pacemaker: meta-optimizer that wraps the self-wiring GA. Each tick it
// reads population state, adapts hyperparameters via the scheduler / oscillator
// / entropy modules, then advances the inner GA one generation.
class Pacemaker {
 public:
  Pacemaker(SelfWiringGA& ga, float base_mr = 0.20f, float base_lr = 0.20f)
      : ga_(ga) {
    sched_.reset(base_mr);
    lro_.reset(base_lr, /*period_gens=*/20.0f);
  }

  void tick() {
    float div = ga_.diversity();
    float mod = div_.adjust(div);  // entropy-driven modulation
    sched_.step();                  // anneal base mutation rate

    GAParams p = ga_.params();
    p.mutation_rate = sched_.rate() * mod;
    p.lr = lro_.lr() * mod;         // oscillating perturbation scale
    ga_.set_params(p);

    lro_.step();
    ga_.step();
  }

  const MutationScheduler& scheduler() const { return sched_; }
  const LearningRateOscillator& oscillator() const { return lro_; }

 private:
  SelfWiringGA& ga_;
  MutationScheduler sched_;
  LearningRateOscillator lro_;
  DivergenceEntropy div_;
};

}  // namespace omega
