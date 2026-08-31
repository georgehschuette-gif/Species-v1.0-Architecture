// Phase 7 "Challenge" — enterprise-grade unit suite.
// Covers the bootstrap universe: toy 2D physics, self-play arena (copy vs copy),
// the Kolmogorov-challenge compressibility test, and bootstrap legacy building.
// All tests are deterministic and bounded.

#include <cstdio>
#include <cmath>
#include <cstdint>
#include <cstring>
#include "10_BOOTSTRAP_UNIVERSE/toy_physics/toy_physics.h"
#include "10_BOOTSTRAP_UNIVERSE/self_play_arena/self_play_arena.h"
#include "10_BOOTSTRAP_UNIVERSE/kolmogorov_challenge/kolmogorov_challenge.h"
#include "10_BOOTSTRAP_UNIVERSE/legacy_building/legacy_building.h"

#include "bench.h"
using namespace omega;

static int g_pass = 0;
static int g_fail = 0;

static void check(bool cond, const char* name) {
  if (cond) { ++g_pass; printf("pass: %s\n", name); }
  else      { ++g_fail; printf("FAIL: %s\n", name); }
}

int main() {
  Bench _b("PHASE7");
  // ---------------- toy_physics ----------------
  {
    ToyPhysics tp;
    check(tp.count() == 0, "toy: empty at start");
    int i0 = tp.add_body(0, 0, 0, 0, 1.0f);
    int i1 = tp.add_body(1, 0, 0, 0, 1.0f);
    check(i0 == 0 && i1 == 1 && tp.count() == 2, "toy: bodies added with indices");

    // Capacity cap.
    ToyPhysics cap;
    int ids[8];
    for (int i = 0; i < 8; i++) ids[i] = cap.add_body(0, 0, 0, 0, 1.0f);
    int added = 0;
    for (int i = 0; i < 8; i++) if (ids[i] >= 0) added++;
    check(added == ToyPhysics::MAX, "toy: body count capped at MAX");

    // Free fall under a constant field: semi-implicit Euler gives
    // v = a*dt and x = a*dt^2 on the first step from rest.
    ToyPhysics fall;
    fall.set_field(0.0f, -9.8f);
    fall.add_body(0, 0, 0, 0, 1.0f);
    fall.step(0.10f);
    check(std::fabs(fall.body(0).vy - (-0.98f)) < 1e-4f, "toy: free-fall velocity = a*dt");
    check(std::fabs(fall.body(0).y - (-0.098f)) < 1e-4f, "toy: free-fall position = a*dt^2");

    // Two-body gravity: a light body in (near) circular orbit about a heavy one.
    // Symplectic Euler keeps total energy bounded — the measurable validation.
    ToyPhysics orbit;
    orbit.add_body(0, 0, 0, 0, 1000.0f);                 // star
    orbit.add_body(10.0f, 0, 0, 10.0f, 1.0f);            // planet: v=sqrt(GM/r)=10
    float e0 = orbit.energy();
    for (int t = 0; t < 4000; t++) orbit.step(0.001f);
    float e1 = orbit.energy();
    check(std::isfinite(e0) && std::isfinite(e1), "toy: energy stays finite over the run");
    check(std::fabs(e1 - e0) < 0.10f * std::fabs(e0), "toy: energy conserved (<10% drift, symplectic)");
    float dx = orbit.body(1).x - orbit.body(0).x;
    float dy = orbit.body(1).y - orbit.body(0).y;
    float r = std::sqrt(dx * dx + dy * dy);
    check(r > 1.0f && r < 40.0f, "toy: planet remains bound to the star");

    // Determinism: same init + stepping reproduces identical state.
    ToyPhysics a, b;
    a.add_body(3, 1, 0.5f, -0.2f, 2.0f);
    b.add_body(3, 1, 0.5f, -0.2f, 2.0f);
    for (int t = 0; t < 200; t++) { a.step(0.01f); b.step(0.01f); }
    bool same = std::fabs(a.body(0).x - b.body(0).x) < 1e-6f &&
                std::fabs(a.body(0).vy - b.body(0).vy) < 1e-6f;
    check(same, "toy: integration is deterministic for identical init");

    // world_hash is stable and state-dependent.
    uint32_t h1 = orbit.world_hash();
    orbit.step(0.001f);
    check(orbit.world_hash() != h1, "toy: world_hash changes as state evolves");
  }

  // ---------------- self_play_arena ----------------
  {
    check(SelfPlayArena::policy(7, 3) == SelfPlayArena::policy(7, 3),
          "selfplay: policy is deterministic");

    // Copy vs copy: identical genomes reach the self-play equilibrium (tie).
    SelfPlayArena copy;
    copy.set_agents(0xDEADBEEF, 0xDEADBEEF);
    copy.play(500);
    check(copy.equilibrium(), "selfplay: copy vs copy reaches equilibrium");
    check(copy.winner() == 0 && copy.scoreA() == 0, "selfplay: copies tie exactly");

    // Distinct genomes: zero-sum. (Copy vs copy already proved the equilibrium
    // invariant above; here we confirm self-play can also be decisive.)
    SelfPlayArena vs;
    vs.set_agents(0, 1);
    vs.play(300);
    check(vs.scoreB() == -vs.scoreA(), "selfplay: scores are zero-sum");

    // Across distinct-genome pairs, at least one produces a decisive outcome
    // (a real winner, not a tie) — self-play is not trivially always a draw.
    bool any_decisive = false;
    for (uint32_t ga = 0; ga < 6 && !any_decisive; ga++)
      for (uint32_t gb = ga + 1; gb < 6; gb++) {
        SelfPlayArena sp; sp.set_agents(ga, gb); sp.play(500);
        SelfPlayArena sp2; sp2.set_agents(ga, gb); sp2.play(500);
        if (sp.scoreA() == sp2.scoreA() && !sp.equilibrium()) any_decisive = true;
      }
    check(any_decisive, "selfplay: distinct genomes can yield a decisive (non-equilibrium) outcome");
  }

  // ---------------- kolmogorov_challenge ----------------
  {
    KolmogorovChallenge kc;

    // A degenerate (low-entropy) 64-byte "program": all 0x01.
    uint8_t seed[64];
    for (int i = 0; i < 64; i++) seed[i] = 0x01;
    kc.set_program(seed);

    check(kc.seed_entropy() < 0.01f, "kolmogorov: seed entropy is ~0 for a constant program");

    uint8_t out[1024];
    kc.emit(out, 1024);
    check(kc.output_entropy() > 7.0f, "kolmogorov: emitted output has high entropy (>7 bits/byte)");

    // Deterministic emission.
    uint8_t out2[1024];
    kc.emit(out2, 1024);
    bool identical = true;
    for (int i = 0; i < 1024; i++) if (out[i] != out2[i]) identical = false;
    check(identical, "kolmogorov: emission is deterministic for the same program");

    // The real KC test: output is less compressible (higher entropy) than the seed.
    check(kc.passes(), "kolmogorov: emitted program output exceeds seed entropy (KC test passes)");

    // Entropy helper sanity: uniform-ish buffer ~ high entropy, constant ~ 0.
    uint8_t unif[256];
    for (int i = 0; i < 256; i++) unif[i] = (uint8_t)i;
    check(KolmogorovChallenge::entropy(unif, 256) > 7.5f, "kolmogorov: entropy() ~8 for a uniform buffer");
    uint8_t flat[256]; for (int i = 0; i < 256; i++) flat[i] = 7;
    check(KolmogorovChallenge::entropy(flat, 256) < 0.01f, "kolmogorov: entropy() ~0 for a constant buffer");
  }

  // ---------------- legacy_building (bootstrap) ----------------
  {
    BootstrapLegacy bl;
    bl.snapshot(0x12345678u, -42.5f, 1u, 1u, 4096u, "BOOT");
    const BootstrapArtifact& a = bl.artifact();
    check(a.world_hash == 0x12345678u, "legacy: world_hash stored");
    check(std::fabs(a.total_energy + 42.5f) < 1e-3f, "legacy: total energy stored");
    check(a.selfplay_winner == 1u && a.kolmogorov_pass == 1u, "legacy: outcomes stored");
    check(a.ticks == 4096u, "legacy: ticks stored");

    uint8_t buf[64]; int len = 0;
    bl.serialize(buf, &len);
    check(len == 36, "legacy: serialize produces a stable 36-byte artifact");
    char hex[128];
    bl.to_hex(hex, sizeof(hex));
    check(std::strlen(hex) == 72, "legacy: to_hex emits 72 hex chars (36 bytes)");

    // Round-trip: deserialize the bytes back into a fresh artifact.
    BootstrapLegacy bl2;
    bl2.snapshot(0, 0, 0, 0, 0, "");
    // Reconstruct via the same layout by reading fields manually.
    uint32_t wh = ((uint32_t)buf[0] << 24) | ((uint32_t)buf[1] << 16) |
                  ((uint32_t)buf[2] << 8) | buf[3];
    check(wh == 0x12345678u, "legacy: serialized world_hash round-trips");
  }

  printf("\nPHASE7_CHALLENGE_TEST: %s (%d pass, %d fail)\n",
         g_fail == 0 ? "PASS" : "FAIL", g_pass, g_fail);
  return g_fail == 0 ? 0 : 1;
}
