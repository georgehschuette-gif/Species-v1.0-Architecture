#include "self_wiring.h"
#include "../../liquid_time/reservoir_pool/reservoir_pool.h"
#include <cfloat>
#include <cmath>

namespace omega {

// Fixed stimulus set used to score every candidate network.
static const float kStimuli[] = {0.15f, 0.40f, 0.65f, 0.90f, 0.30f};
static const int kStimN = 5;
static const int kSteps = 24;
static const float kDt = 0.05f;
static const float kTau = 0.10f;

SelfWiringGA::SelfWiringGA(const Network& seed, uint16_t n_nodes, uint32_t rng)
    : n_nodes_(n_nodes), rng_(rng ? rng : 0x9E3779B9u) {
  for (uint16_t i = 0; i < POP; i++) {
    pw_copy(&pop_[i], &seed);
    fit_[i] = -FLT_MAX;
  }
  pw_copy(&best_, &seed);
  // Score the seed so best_ starts meaningful.
  best_fit_ = evaluate(seed);
}

uint32_t SelfWiringGA::rnd() {
  uint32_t x = rng_;
  x ^= x << 13;
  x ^= x >> 17;
  x ^= x << 5;
  rng_ = x;
  return x;
}

float SelfWiringGA::rndf() {
  return (float)(rnd() & 0x7FFFFFFFu) / (float)0x7FFFFFFFu;
}

float SelfWiringGA::evaluate(const Network& net) {
  return rp_fitness(&net, kStimuli, kStimN, kSteps, kDt, kTau);
}

void SelfWiringGA::mutate(Network& net) {
  if (rndf() >= params_.mutation_rate) return;

  if (rndf() < params_.struct_prob) {
    // Structural mutation: add a new random edge, or drop an existing one.
    if (rndf() < 0.5f && net.n_edges < PW_MAX_EDGES) {
      uint16_t s = (uint16_t)(rndf() * net.n_nodes);
      uint16_t d = (uint16_t)(rndf() * net.n_nodes);
      if (s != d && pw_weight(&net, s, d) == 0.0f) {
        PWEdge e;
        e.src = s;
        e.dst = d;
        e.w = (rndf() - 0.5f) * 1.2f;
        net.edges[net.n_edges++] = e;
      }
    } else if (net.n_edges > net.n_nodes) {
      uint16_t k = (uint16_t)(rndf() * net.n_edges);
      net.edges[k] = net.edges[--net.n_edges];
    }
  } else {
    // Weight perturbation on a random existing edge.
    if (net.n_edges > 0) {
      uint16_t k = (uint16_t)(rndf() * net.n_edges);
      net.edges[k].w += (rndf() - 0.5f) * 2.0f * params_.lr;
      if (net.edges[k].w > 3.0f) net.edges[k].w = 3.0f;
      if (net.edges[k].w < -3.0f) net.edges[k].w = -3.0f;
    }
  }
}

void SelfWiringGA::crossover(const Network& a, const Network& b, Network& out) {
  out.n_nodes = a.n_nodes;
  out.n_edges = 0;
  out.seed = a.seed;
  // Union of edges, resolving duplicates by averaging weights.
  for (uint16_t i = 0; i < a.n_edges && out.n_edges < PW_MAX_EDGES; i++) {
    out.edges[out.n_edges++] = a.edges[i];
  }
  for (uint16_t i = 0; i < b.n_edges && out.n_edges < PW_MAX_EDGES; i++) {
    const PWEdge& e = b.edges[i];
    int found = 0;
    for (uint16_t j = 0; j < out.n_edges; j++) {
      if (out.edges[j].src == e.src && out.edges[j].dst == e.dst) {
        out.edges[j].w = 0.5f * (out.edges[j].w + e.w);
        found = 1;
        break;
      }
    }
    if (!found) out.edges[out.n_edges++] = e;
  }
}

void SelfWiringGA::step() {
  // Evaluate current population.
  float sum = 0.0f;
  for (uint16_t i = 0; i < POP; i++) {
    fit_[i] = evaluate(pop_[i]);
    sum += fit_[i];
    if (fit_[i] > best_fit_) {
      best_fit_ = fit_[i];
      pw_copy(&best_, &pop_[i]);
    }
  }
  mean_fit_ = sum / POP;

  // Tournament selection + elitism into a next generation.
  Network next[POP];
  // Elitism: carry the single best unchanged.
  pw_copy(&next[0], &best_);

  for (uint16_t i = 1; i < POP; i++) {
    // Binary tournament.
    uint16_t a = (uint16_t)(rndf() * POP);
    uint16_t b = (uint16_t)(rndf() * POP);
    const Network& winner = (fit_[a] >= fit_[b]) ? pop_[a] : pop_[b];
    const Network& loser = (fit_[a] < fit_[b]) ? pop_[a] : pop_[b];
    crossover(winner, loser, next[i]);
    mutate(next[i]);
  }

  for (uint16_t i = 0; i < POP; i++) pw_copy(&pop_[i], &next[i]);
  gen_++;
}

float SelfWiringGA::diversity() const {
  // Mean normalized symmetric difference across all pairs.
  if (POP < 2) return 0.0f;
  float total = 0.0f;
  int pairs = 0;
  for (uint16_t i = 0; i < POP; i++) {
    for (uint16_t j = i + 1; j < POP; j++) {
      int inter = 0, uni = 0;
      for (uint16_t a = 0; a < pop_[i].n_edges; a++) {
        int in_j = 0;
        for (uint16_t b = 0; b < pop_[j].n_edges; b++) {
          if (pop_[i].edges[a].src == pop_[j].edges[b].src &&
              pop_[i].edges[a].dst == pop_[j].edges[b].dst) {
            in_j = 1;
            break;
          }
        }
        if (in_j) inter++;
        uni++;
      }
      uni += (int)pop_[j].n_edges - inter;
      float jac = uni > 0 ? 1.0f - (float)inter / (float)uni : 0.0f;
      total += jac;
      pairs++;
    }
  }
  return pairs > 0 ? total / (float)pairs : 0.0f;
}

}  // namespace omega
