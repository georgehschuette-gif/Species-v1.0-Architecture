#pragma once
#include <stdint.h>

// Primordial weights: the structured-random seed substrate (C, static arrays).
// This is the only place the network topology lives; the GA (self_wiring)
// evolves copies of this struct. No heap, fixed capacity, deterministic PRNG.

#ifdef __cplusplus
extern "C" {
#endif

#define PW_MAX_NODES 32
#define PW_MAX_EDGES 512

typedef struct {
  uint16_t src;
  uint16_t dst;
  float w;
} PWEdge;

typedef struct {
  uint16_t n_nodes;
  uint16_t n_edges;
  PWEdge edges[PW_MAX_EDGES];
  uint32_t seed;  // mutable PRNG state
} Network;

// xorshift32 (deterministic). Advances net->seed each call.
uint32_t pw_rand(Network* net);
float pw_randf(Network* net);  // uniform in [0,1)

// Structured-random seed: ring lattice (local wiring) + sparse long-range
// links. Produces a small-world topology rather than pure noise.
void pw_init(Network* net, uint16_t n_nodes, uint32_t seed);

// Topology copy (edges + weights + node count).
void pw_copy(Network* dst, const Network* src);

// Look up weight of edge src->dst, or 0.0 if absent.
float pw_weight(const Network* net, uint16_t src, uint16_t dst);

// Stable hash of the topology (edge endpoints + rounded weights).
uint32_t pw_topology_hash(const Network* net);

// Number of edges whose |w| exceeds a threshold (sparsity metric).
uint16_t pw_active_edges(const Network* net, float threshold);

#ifdef __cplusplus
}
#endif
