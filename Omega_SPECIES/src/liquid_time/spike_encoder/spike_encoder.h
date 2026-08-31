#pragma once
#include <stdint.h>

// Spike encoder: converts a continuous stimulus value into discrete spike
// events on reservoir input channels (C, event-driven / temporal coding).
// Each channel owns a threshold band; a value inside the band fires that node.

#ifdef __cplusplus
extern "C" {
#endif

#define SE_MAX_CH 32

typedef struct {
  uint16_t n;
  float bands[SE_MAX_CH];  // center of each channel's firing band
} SpikeEncoder;

void se_init(SpikeEncoder* se, uint16_t n, uint32_t seed);

// Encode `value` in [0,1). Fills `spike_nodes` (capacity `max`) with the
// indices of channels that fired. Returns the number of spikes emitted.
int se_encode(SpikeEncoder* se, float value, uint16_t* spike_nodes, int max);

#ifdef __cplusplus
}
#endif
