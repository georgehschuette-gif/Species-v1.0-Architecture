#pragma once
#include <stdint.h>

// Reservoir pool: untrained recurrent substrate (C, event-driven).
// Recurrent connectivity is supplied by a Network's edge list; the reservoir
// itself never learns — it only integrates. State is a fixed-size buffer.

#include "../../genesis/primordial_weights/primordial_weights.h"

#ifdef __cplusplus
extern "C" {
#endif

#define RP_MAX_NODES 32

typedef struct {
  uint16_t n;
  float state[RP_MAX_NODES];
  float tau;  // time constant (leak)
} Reservoir;

void rp_init(Reservoir* r, uint16_t n, float tau);
void rp_reset(Reservoir* r);

// One integration step. `input` is per-node external drive (e.g. spikes).
// Recurrent term uses edges src->j from the supplied Network.
void rp_step(Reservoir* r, const Network* net, const float* input, float dt);

// Fitness proxy: drive the reservoir over a stimulus set and return
// activation diversity minus an instability penalty for blow-ups.
// Higher = richer, stable dynamics (a reservoir-quality metric).
float rp_fitness(const Network* net, const float* stimuli, int n_stim,
                 int steps, float dt, float tau);

#ifdef __cplusplus
}
#endif
