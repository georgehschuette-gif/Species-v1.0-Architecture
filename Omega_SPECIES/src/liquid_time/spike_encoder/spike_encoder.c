#include "spike_encoder.h"

void se_init(SpikeEncoder* se, uint16_t n, uint32_t seed) {
  se->n = n > SE_MAX_CH ? SE_MAX_CH : n;
  uint32_t s = seed ? seed : 0x2545F491u;
  for (uint16_t i = 0; i < se->n; i++) {
    // Evenly spaced, jittered threshold bands across [0,1).
    float base = (float)(i + 1) / (float)(se->n + 1);
    s ^= s << 13; s ^= s >> 17; s ^= s << 5;
    float jitter = ((float)(s & 0x7FFFFFFFu) / (float)0x7FFFFFFFu - 0.5f) * (1.0f / (se->n + 1));
    float b = base + jitter;
    if (b < 0.0f) b = 0.0f;
    if (b > 1.0f) b = 1.0f;
    se->bands[i] = b;
  }
}

int se_encode(SpikeEncoder* se, float value, uint16_t* spike_nodes, int max) {
  int n = 0;
  for (uint16_t i = 0; i < se->n; i++) {
    // Fire when the value is within +/- half-band of this channel's center.
    float half = 0.5f / (float)(se->n + 1);
    if (value >= se->bands[i] - half && value <= se->bands[i] + half) {
      if (n < max) spike_nodes[n] = i;
      n++;
    }
  }
  return n;
}
