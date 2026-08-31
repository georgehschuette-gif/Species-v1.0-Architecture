#include "reservoir_pool.h"
#include <math.h>

void rp_init(Reservoir* r, uint16_t n, float tau) {
  r->n = n > RP_MAX_NODES ? RP_MAX_NODES : n;
  r->tau = tau;
  rp_reset(r);
}

void rp_reset(Reservoir* r) {
  for (uint16_t i = 0; i < r->n; i++) r->state[i] = 0.0f;
}

void rp_step(Reservoir* r, const Network* net, const float* input, float dt) {
  float next[RP_MAX_NODES];
  for (uint16_t j = 0; j < r->n; j++) {
    float acc = 0.0f;
    // Recurrent contribution: sum over edges ending at j.
    for (uint16_t e = 0; e < net->n_edges; e++) {
      if (net->edges[e].dst == j) acc += net->edges[e].w * r->state[net->edges[e].src];
    }
    float leak = (1.0f - dt / r->tau);
    if (leak < 0.0f) leak = 0.0f;
    float drive = input ? input[j] : 0.0f;
    next[j] = leak * r->state[j] + dt * acc + drive;
  }
  for (uint16_t j = 0; j < r->n; j++) r->state[j] = next[j];
}

float rp_fitness(const Network* net, const float* stimuli, int n_stim,
                 int steps, float dt, float tau) {
  Reservoir r;
  rp_init(&r, net->n_nodes, tau);

  float sum = 0.0f;
  float sum2 = 0.0f;
  long count = 0;
  float penalty = 0.0f;

  for (int s = 0; s < n_stim; s++) {
    rp_reset(&r);
    float input[RP_MAX_NODES];
    for (uint16_t i = 0; i < r.n; i++) input[i] = stimuli[s];  // broadcast stimulus
    for (int t = 0; t < steps; t++) {
      rp_step(&r, net, input, dt);
      for (uint16_t i = 0; i < r.n; i++) {
        float v = r.state[i];
        sum += v;
        sum2 += v * v;
        count++;
        if (v > 10.0f || v < -10.0f) penalty += 1.0f;  // instability
      }
    }
  }

  if (count == 0) return 0.0f;
  float mean = sum / (float)count;
  float var = sum2 / (float)count - mean * mean;
  if (var < 0.0f) var = 0.0f;
  return var - 0.05f * penalty;
}
