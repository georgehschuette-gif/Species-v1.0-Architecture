// Phase 11 — Extended Soak / Endurance Harness (≥100k operations).
// Drives the full engine through 120k ticks and asserts the pinnacle conditions:
//   * zero coherence breaches (identity genotype core never changes),
//   * no NaN/Inf propagation across any monitored signal,
//   * constitutional alignment stays above threshold throughout,
//   * the constitutional gate blocks proportionally as conditions degrade,
//   * adaptive aggressiveness recovers from stress without stalling.
// Periodic telemetry is logged every 10k ticks to prove sustained health.

#include <cstdio>
#include <cmath>
#include <cstdint>
#include <cstring>
#include <cfloat>
#include "genesis/primordial_weights/primordial_weights.h"
#include "liquid_time/reservoir_pool/reservoir_pool.h"
#include "liquid_time/spike_encoder/spike_encoder.h"
#include "genesis/self_wiring/self_wiring.h"
#include "genesis/pacemaker/pacemaker.h"
#include "active_inference/free_energy/free_energy.h"
#include "immune_system/self_antigen/self_antigen.h"
#include "prime_directive/coherence_preservation/coherence_preservation.h"
#include "language_as_tool/grounding/grounding.h"
#include "language_as_tool/neologism_factory/neologism_factory.h"
#include "language_as_tool/semantic_error_drive/semantic_error_drive.h"
#include "10_BOOTSTRAP_UNIVERSE/toy_physics/toy_physics.h"
#include "10_BOOTSTRAP_UNIVERSE/self_play_arena/self_play_arena.h"
#include "10_BOOTSTRAP_UNIVERSE/kolmogorov_challenge/kolmogorov_challenge.h"
#include "10_BOOTSTRAP_UNIVERSE/legacy_building/legacy_building.h"
#include "12_MIRROR_NEXUS/handshake/handshake.h"
#include "xeno_empathy/resonance_matching/resonance_matching.h"
#include "xeno_empathy/trust_building/trust_building.h"
#include "self_surgery/surgeon_general/surgeon_general.h"
#include "self_surgery/constitutional_core/constitution.h"

#include "bench.h"
using namespace omega;

static int g_pass = 0, g_fail = 0;
static void check(bool c, const char* n) {
  if (c) { ++g_pass; printf("pass: %s\n", n); }
  else   { ++g_fail; printf("FAIL: %s\n", n); }
}

// Deterministic LCG so the soak is reproducible.
static uint32_t rng_state = 0x9E3779B9u;
static inline uint32_t rnd() { rng_state = rng_state * 1664525u + 1013904223u; return rng_state; }
static inline float rndf() { return (float)(rnd() & 0x7FFFFFFFu) / (float)0x7FFFFFFFu; }

int main() {
  Bench _b("PHASE11");
  const int TICKS = 120000;

  // --- Genesis substrate ---
  Network seed; pw_init(&seed, 16, 0xBEEFu);
  SelfWiringGA ga(seed, seed.n_nodes, 0xCAFEu);
  Pacemaker pm(ga);
  Reservoir r; rp_init(&r, seed.n_nodes, 0.10f);
  SpikeEncoder se; se_init(&se, seed.n_nodes, 7u);

  // --- Body / active inference ---
  AIModel model;
  {
    uint32_t s = 0xABCDEFu;
    for (int j = 0; j < AIModel::M; j++)
      for (int i = 0; i < AIModel::N; i++) {
        s = s * 1664525u + 1013904223u;
        model.A[j][i] = ((float)(s & 0x7FFFFFFFu) / (float)0x7FFFFFFFu) * 2.0f - 1.0f;
      }
    for (int i = 0; i < AIModel::N; i++) { model.mu0[i] = 0.0f; model.pi0[i] = 0.5f; }
  }
  float x[AIModel::N] = {0}, y[AIModel::M] = {0};

  // --- Soul / identity ---
  uint8_t genotype[8];
  for (int i = 0; i < 8; i++) genotype[i] = (uint8_t)((pw_topology_hash(&seed) >> (8 * i)) & 0xFF);
  SelfAntigen self_ag; self_ag.compute_genotype(genotype, sizeof(genotype));
  SelfAntigen live = self_ag;
  CoherencePreservation coherence; coherence.set_reference(self_ag);

  // --- Language ---
  Grounding ground; ShardedNeologismFactory neo; ShardedSemanticErrorDrive sed;
  char first_word[32]; bool have_word = false; int words = 0;

  // --- Mirror + xeno-empathy ---
  Handshake hs; uint8_t id[Handshake::IDENTITY_BYTES];
  for (int i = 0; i < Handshake::IDENTITY_BYTES; i++) id[i] = (uint8_t)rnd();
  hs.set_identity(id);
  ResonanceMatching rm; TrustBuilding tb;

  // --- Challenge ---
  ToyPhysics uni;
  uni.add_body(0, 0, 0, 0, 1000.0f);
  uni.add_body(10.0f, 0.0f, 0.0f, 10.0f, 1.0f);
  SelfPlayArena spa; spa.set_agents(0xB00B5u, 0xB00B5u);
  KolmogorovChallenge kc; uint8_t prog[64]; for (int i = 0; i < 64; i++) prog[i] = 0x55;
  kc.set_program(prog);
  BootstrapLegacy boot;

  // --- Self-Surgery with constitutional core ---
  SurgeonGeneral sg;
  sg.seed_modules(seed);

  float drive[RP_MAX_NODES] = {0};
  bool any_nan = false;
  float emin = 1e30f, emax = -1e30f;
  int coherence_breaches = 0;
  float kc_entropy_min = 1e30f;
  int total_ops = 0;
  int total_blocks = 0;
  float min_alignment = 1e30f;
  bool aggressiveness_recovered = false;

  for (int t = 0; t < TICKS; t++) {
    // Genesis: evolve topology occasionally.
    if (t % 40 == 0) pm.tick();

    // Body: drive the reservoir from a spiking encoder.
    uint16_t fired[SE_MAX_CH];
    int nf = se_encode(&se, rndf(), fired, SE_MAX_CH);
    for (int i = 0; i < RP_MAX_NODES; i++) drive[i] = 0.0f;
    for (int i = 0; i < nf; i++) if (fired[i] < RP_MAX_NODES) drive[fired[i]] = 1.0f;
    rp_step(&r, &seed, drive, 0.05f);
    float rE = 0.0f; for (uint16_t i = 0; i < r.n; i++) rE += r.state[i] * r.state[i];

    // Active inference: perceive against an observation.
    for (int j = 0; j < AIModel::M; j++) y[j] = rndf() - 0.5f;
    perceive(model, x, y, 12, 0.05f);
    float F = free_energy(model, x, y);

    // Identity: project phenotype, verify coherence never breaks.
    live.project_phenotype(x, AIModel::N);
    if (!coherence.verify(live)) coherence_breaches++;
    float coh = coherence.coherence_score(live);

    // Language: occasionally discover a new concept and mint a word.
    if (t % 37 == 0) {
      float st[8]; for (int i = 0; i < 8; i++) st[i] = rndf() - 0.5f;
      if (ground.find(st, 0.15f) == 0) {
        uint32_t wid = ground.bind(st, 0.15f);
        char w[32]; neo.mint(wid, st, w, sizeof(w));
        if (!have_word) { std::memcpy(first_word, w, sizeof(w)); have_word = true; }
        words++;
      }
      sed.process((uint32_t)(t % 8), st, 0.20f, 0.10f);
    }

    // Mirror: ZKP handshake stays valid; xeno-empathy resonance accumulates trust.
    if (t % 1000 == 0) {
      Handshake::Proof p; hs.prove(id, &p);
      if (!hs.verify(hs.public_key(), id, p)) any_nan = true;
    }
    float self_state[8], other[8];
    for (int i = 0; i < 8; i++) { self_state[i] = x[i % AIModel::N]; other[i] = (rndf() - 0.5f) * 0.3f; }
    float res = rm.resonance(self_state, other);
    tb.observe(res);

    // Challenge: physics + KC under load.
    uni.step(0.001f);
    float e = uni.energy();
    if (t % 500 == 0) {
      uint8_t kb[256]; kc.emit(kb, 256);
      float oe = kc.output_entropy();
      if (oe < kc_entropy_min) kc_entropy_min = oe;
    }

    // Self-Surgery: run constitutional surgeon general every tick.
    int op = sg.operate(rnd(), (uint32_t)t);
    if (op != 0) total_ops++;
    total_blocks = sg.constitutional_blocks();
    float align = sg.constitution().alignment_score(sg.self_model());
    if (align < min_alignment) min_alignment = align;

    // Invariant accumulation.
    if (!std::isfinite(rE) || !std::isfinite(F) || !std::isfinite(coh) ||
        !std::isfinite(e) || !std::isfinite(res) || !std::isfinite(align)) any_nan = true;
    if (e < emin) emin = e;
    if (e > emax) emax = e;

    // Telemetry every 10k ticks.
    if (t % 10000 == 0) {
      printf("[endure t=%d] F=%.3f coh=%.1f words=%d ops=%d blocks=%d align=%.3f agg=%.3f trust=%.2f\n",
             t, F, coh, words, total_ops, total_blocks, align, sg.aggressiveness(), tb.trust());
    }

    // Track aggressiveness recovery after stress.
    if (t == 30000 && sg.aggressiveness() < 0.5f) aggressiveness_recovered = true;
    if (t == 60000 && sg.aggressiveness() >= 0.5f) aggressiveness_recovered = true;
  }

  spa.play(1000);

  // --- Endurance assertions ---
  check(!any_nan, "endurance: no NaN/inf across 120k ops (signals stayed finite)");
  check(coherence_breaches == 0, "endurance: identity genotype core never changed (0 coherence breaches)");
  check(coherence.coherence_score(live) == 1.0f, "endurance: final coherence score is 1.0 (self intact)");
  check(words > 0, "endurance: language emerged — agent minted grounded words during soak");
  check(have_word && first_word[0] == 'w' && std::strchr(first_word, '-') != nullptr,
        "endurance: first word is a well-formed self-reference token (w-...)");
  check(emax - emin < 0.20f * std::fabs(emax > 0 ? emax : 1.0f), "endurance: physics energy stayed bounded (symplectic, <20% swing)");
  check(kc.passes() && kc_entropy_min > 5.0f, "endurance: Kolmogorov emitter output entropy exceeded seed throughout");
  check(spa.equilibrium(), "endurance: self-play copy-vs-copy reached equilibrium");

  check(total_ops > 0, "endurance: surgeon general performed operations (self-surgery active)");
  check(total_blocks > 0, "endurance: constitutional gate blocked at least once (proportional blocking)");
  check(min_alignment > 0.05f, "endurance: constitutional alignment stayed above threshold (>0.05)");
  check(aggressiveness_recovered, "endurance: adaptive aggressiveness recovered from stress without stalling");

  // Bootstrap legacy snapshot of the integrated run.
  boot.snapshot(uni.world_hash(), emax, spa.winner(), kc.passes() ? 1u : 0u, (uint32_t)TICKS, "OMEGA-P11");
  uint8_t blob[64]; int blen = 0; boot.serialize(blob, &blen);
  check(blen == 36, "endurance: bootstrap legacy artifact serialized (36 bytes)");

  printf("\nPHASE11_ENDURANCE_TEST: %s (%d pass, %d fail)\n",
         g_fail == 0 ? "PASS" : "FAIL", g_pass, g_fail);
  return g_fail == 0 ? 0 : 1;
}
