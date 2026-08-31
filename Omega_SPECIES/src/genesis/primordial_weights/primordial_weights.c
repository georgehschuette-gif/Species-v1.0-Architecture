#include "primordial_weights.h"

uint32_t pw_rand(Network* net) {
  uint32_t x = net->seed;
  x ^= x << 13;
  x ^= x >> 17;
  x ^= x << 5;
  net->seed = x;
  return x;
}

float pw_randf(Network* net) {
  return (float)(pw_rand(net) & 0x7FFFFFFFu) / (float)0x7FFFFFFFu;
}

static int pw_has_edge(const Network* net, uint16_t src, uint16_t dst) {
  for (uint16_t i = 0; i < net->n_edges; i++) {
    if (net->edges[i].src == src && net->edges[i].dst == dst) return 1;
  }
  return 0;
}

static void pw_add_edge(Network* net, uint16_t src, uint16_t dst, float w) {
  if (net->n_edges >= PW_MAX_EDGES) return;
  if (src == dst) return;
  if (pw_has_edge(net, src, dst)) return;
  PWEdge* e = &net->edges[net->n_edges++];
  e->src = src;
  e->dst = dst;
  e->w = w;
}

void pw_init(Network* net, uint16_t n_nodes, uint32_t seed) {
  net->n_nodes = n_nodes > PW_MAX_NODES ? PW_MAX_NODES : n_nodes;
  net->n_edges = 0;
  net->seed = seed ? seed : 0x9E3779B9u;

  // Ring lattice: each node feeds its two nearest neighbours (local wiring).
  for (uint16_t i = 0; i < net->n_nodes; i++) {
    uint16_t a = (uint16_t)((i + 1) % net->n_nodes);
    uint16_t b = (uint16_t)((i + 2) % net->n_nodes);
    pw_add_edge(net, i, a, (pw_randf(net) - 0.5f) * 0.6f);
    pw_add_edge(net, i, b, (pw_randf(net) - 0.5f) * 0.6f);
  }

  // Sparse long-range links: a few random shortcuts (small-world).
  uint16_t long_range = (uint16_t)(net->n_nodes * 2);
  for (uint16_t k = 0; k < long_range; k++) {
    uint16_t s = (uint16_t)(pw_randf(net) * net->n_nodes);
    uint16_t d = (uint16_t)(pw_randf(net) * net->n_nodes);
    pw_add_edge(net, s, d, (pw_randf(net) - 0.5f) * 1.2f);
  }
}

void pw_copy(Network* dst, const Network* src) {
  dst->n_nodes = src->n_nodes;
  dst->n_edges = src->n_edges;
  dst->seed = src->seed;
  for (uint16_t i = 0; i < src->n_edges; i++) dst->edges[i] = src->edges[i];
}

float pw_weight(const Network* net, uint16_t src, uint16_t dst) {
  for (uint16_t i = 0; i < net->n_edges; i++) {
    if (net->edges[i].src == src && net->edges[i].dst == dst) return net->edges[i].w;
  }
  return 0.0f;
}

uint32_t pw_topology_hash(const Network* net) {
  uint32_t h = 0x811C9DC5u;
  for (uint16_t i = 0; i < net->n_edges; i++) {
    const PWEdge* e = &net->edges[i];
    uint32_t v = ((uint32_t)e->src * 31u + e->dst) ^ ((uint32_t)(e->w * 1000.0f) & 0xFFFFu);
    h ^= v;
    h *= 0x01000193u;
  }
  return h;
}

uint16_t pw_active_edges(const Network* net, float threshold) {
  uint16_t c = 0;
  for (uint16_t i = 0; i < net->n_edges; i++) {
    if (net->edges[i].w > threshold || net->edges[i].w < -threshold) c++;
  }
  return c;
}
