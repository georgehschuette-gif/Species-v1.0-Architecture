// Phase 9 — Breaking-point / stress harness (INFORMATIONAL, not part of `make test`).
// Pushes every module to its limits with degenerate inputs (tiny buffers, NaN/Inf,
// coincident bodies, capacity caps, huge loops) and reports what HOLDS vs BREAKS.
// A probe that behaves outside its documented contract is flagged BROKE.

#include <cstdio>
#include <cmath>
#include <cstdint>
#include <cstring>
#include <cfloat>
#include <string>
#include "genesis/primordial_weights/primordial_weights.h"
#include "liquid_time/reservoir_pool/reservoir_pool.h"
#include "language_as_tool/grounding/grounding.h"
#include "language_as_tool/neologism_factory/neologism_factory.h"
#include "language_as_tool/semantic_error_drive/semantic_error_drive.h"
#include "10_BOOTSTRAP_UNIVERSE/toy_physics/toy_physics.h"
#include "10_BOOTSTRAP_UNIVERSE/self_play_arena/self_play_arena.h"
#include "10_BOOTSTRAP_UNIVERSE/kolmogorov_challenge/kolmogorov_challenge.h"
#include "12_MIRROR_NEXUS/handshake/handshake.h"
#include "xeno_empathy/resonance_matching/resonance_matching.h"
#include "xeno_empathy/trust_building/trust_building.h"
#include "12_MIRROR_NEXUS/schism_detector/schism_detector.h"
#include "active_inference/free_energy/free_energy.h"
#include "immune_system/self_antigen/self_antigen.h"

#include "bench.h"
using namespace omega;

static int held = 0, broke = 0;
static void probe(const char* name, bool ok, const char* detail) {
  if (ok) { held++; printf("  [HELD]  %s%s%s\n", name, detail ? " — " : "", detail ? detail : ""); }
  else    { broke++; printf("  [BROKE] %s%s%s\n", name, detail ? " — " : "", detail ? detail : ""); }
}

int main() {
  Bench _b("STRESS");
  printf("=== Ω_SPECIES breaking-point sweep ===\n");

  // 1) Neologism buffer contract: caller promises n bytes; writing past index n-1
  //    is a contract violation (and a real out-of-bounds write for n<=3).
  {
    NeologismFactory neo;
    float st[8] = {0}; st[0] = 0.5f;
    bool all_ok = true;
    char buf[32];
    for (int n = 1; n <= 8; n++) {
      std::memset(buf, 0xAA, sizeof(buf));
      char tag[4]; std::snprintf(tag, sizeof(tag), "%d", n);
      neo.mint(7, st, buf, n);
      for (int i = n; i < (int)sizeof(buf); i++) if (buf[i] != (char)0xAA) all_ok = false;
    }
    probe("neologism respects caller buffer size (n=1..8)", !all_ok ? false : true,
          all_ok ? "no overrun" : "writes past n (buffer overflow for small n)");
  }

  // 2) Grounding dynamic growth — no CAP limit.
  {
    Grounding g; float s[8]; int added = 0;
    for (int k = 0; k < 128; k++) {  // well beyond old CAP=32 — grows dynamically
      for (int i = 0; i < 8; i++) s[i] = (float)(k + 1) * 10.0f;
      if (g.bind(s)) added++;
    }
    probe("grounding grows dynamically (no CAP)", added == 128,
          ("added " + std::to_string(added)).c_str());
  }

  // 3) Grounding with NaN state: must be handled consistently (rejected safely,
  //    so bind and find agree) rather than silently binding then losing it.
  {
    Grounding g; float nanv[8]; for (int i = 0; i < 8; i++) nanv[i] = NAN;
    uint32_t id = g.bind(nanv);
    uint32_t found = g.find(nanv);
    probe("grounding NaN state handled consistently", id == 0 && found == 0,
          (std::string("bind=") + std::to_string(id) + " find=" + std::to_string(found) + " (rejected safely)").c_str());
  }

  // 4) Toy physics: coincident bodies (r->0). Clamped to 1e-6 so finite, but the
  //    resulting velocity/energy blows up to huge magnitudes.
  {
    ToyPhysics tp;
    tp.add_body(0, 0, 0, 0, 1.0f);
    tp.add_body(0, 0, 0, 0, 1.0f);
    for (int t = 0; t < 10; t++) tp.step(0.01f);
    float e = tp.energy();
    bool finite = std::isfinite(e);
    probe("toy physics stays finite under coincident bodies", finite,
          finite ? ("energy=" + std::to_string(e)).c_str() : "energy=NaN/Inf");
  }

  // 5) Resonance matching with NaN inputs -> NaN (no sanitization).
  {
    ResonanceMatching rm; float a[8], b[8];
    for (int i = 0; i < 8; i++) { a[i] = NAN; b[i] = NAN; }
    float r = rm.resonance(a, b);
    probe("resonance handles NaN inputs", std::isfinite(r),
          std::isfinite(r) ? "finite" : "returns NaN");
  }

  // 6) Schism detector fed NaN resonance -> coherence becomes NaN.
  {
    SchismDetector sd; sd.observe(NAN);
    bool ok = std::isfinite(sd.coherence());
    probe("schism coherence stays finite under NaN", ok,
          ok ? "finite" : "coherence=NaN");
  }

  // 7) Kolmogorov emit with n=0 must not divide by zero / crash.
  {
    KolmogorovChallenge kc; uint8_t prog[64]; for (int i = 0; i < 64; i++) prog[i] = (uint8_t)i;
    kc.set_program(prog); uint8_t o[4]; kc.emit(o, 0);
    bool ok = std::isfinite(kc.output_entropy()) && kc.output_entropy() >= 0.0f;
    probe("kolmogorov emit(n=0) is safe", ok, ok ? "entropy ok" : "bad");
  }

  // 8) Handshake: 2000 proofs across distinct nonces all verify; public key stable.
  {
    Handshake h; uint8_t id[64]; for (int i = 0; i < 64; i++) id[i] = (uint8_t)(i * 7 + 3);
    h.set_identity(id); uint64_t pk = h.public_key();
    uint8_t nonce[64]; int bad = 0;
    for (int i = 0; i < 2000; i++) {
      for (int j = 0; j < 64; j++) nonce[j] = (uint8_t)(i * 13 + j);
      Handshake::Proof p; h.prove(nonce, &p);
      if (!h.verify(pk, nonce, p)) bad++;
    }
    probe("handshake verifies 2000 distinct nonces", bad == 0,
          ("public_key stable=" + std::to_string(h.public_key() == pk) + " failed=" + std::to_string(bad)).c_str());
  }

  // 9) Self-play: 2,000,000 rounds, no crash, deterministic result.
  {
    SelfPlayArena a; a.set_agents(0xBEEF, 0xBEEF); a.play(2000000);
    SelfPlayArena b; b.set_agents(0xBEEF, 0xBEEF); b.play(2000000);
    probe("self-play 2M rounds (copy vs copy)", a.scoreA() == b.scoreA() && a.equilibrium(),
          ("scoreA=" + std::to_string(a.scoreA())).c_str());
  }

  // 10) Free energy with NaN belief -> NaN surprise (no sanitization).
  {
    AIModel m; float x[4], y[3]; for (int i = 0; i < 4; i++) x[i] = NAN; for (int i = 0; i < 3; i++) y[i] = 0;
    perceive(m, x, y, 10, 0.05f);
    float F = free_energy(m, x, y);
    probe("free_energy handles NaN belief", std::isfinite(F), std::isfinite(F) ? "finite" : "returns NaN");
  }

  // 11) Reservoir driven by absurdly large input stays finite (leaky integrator).
  {
    Network seed; pw_init(&seed, 16, 0xBEEFu);
    Reservoir r; rp_init(&r, seed.n_nodes, 0.10f);
    float drive[RP_MAX_NODES]; for (int i = 0; i < RP_MAX_NODES; i++) drive[i] = 1e9f;
    rp_step(&r, &seed, drive, 0.05f);
    float e = 0; for (uint16_t i = 0; i < r.n; i++) e += r.state[i] * r.state[i];
    probe("reservoir bounded under huge drive", std::isfinite(e),
          std::isfinite(e) ? ("energy=" + std::to_string(e)).c_str() : "NaN/Inf");
  }

  printf("=== summary: %d HELD, %d BROKE ===\n", held, broke);
  return 0;  // informational only
}
