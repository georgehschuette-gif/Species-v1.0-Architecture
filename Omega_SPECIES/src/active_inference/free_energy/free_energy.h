#pragma once
#include <cstdint>

namespace omega {

// Small fixed-dimensional active-inference generative model (gradient-free).
// Belief (hidden states) x: N-dim. Observations y: M-dim. Predicted obs ŷ = A x.
struct AIModel {
  static constexpr int N = 4;  // hidden states
  static constexpr int M = 3;  // observations
  float A[M][N];                // observation mapping (fixed)
  float sigma2 = 0.5f;          // observation noise variance
  float mu0[N];                 // prior mean (shaped by allostatic control)
  float pi0[N];                 // prior precision (inverse variance)
};

// Variational free energy ~ accuracy (prediction error) + complexity
// (deviation from prior). Lower is better; this is the "surprise" signal.
float free_energy(const AIModel& m, const float* x, const float* y);

// Gradient-free perception: hill-climb each belief dimension by +/- step to
// reduce F (coordinate descent). Updates x in place.
void perceive(AIModel& m, float* x, const float* y, int iters, float step);

}  // namespace omega
