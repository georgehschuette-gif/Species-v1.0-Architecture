// Phase 6 "Mirror" — enterprise-grade unit suite.
// Covers the Mirror Nexus: ZKP handshake (Schnorr over a prime field),
// ontology mapping, syncretic fusion, and schism detection. All tests are
// deterministic and bounded.

#include <cstdio>
#include <cmath>
#include <cstdint>
#include <cstring>
#include "12_MIRROR_NEXUS/handshake/handshake.h"
#include "12_MIRROR_NEXUS/ontology_mapping/ontology_mapping.h"
#include "12_MIRROR_NEXUS/syncretic_fusion/syncretic_fusion.h"
#include "12_MIRROR_NEXUS/schism_detector/schism_detector.h"

#include "bench.h"
using namespace omega;

static int g_pass = 0;
static int g_fail = 0;

static void check(bool cond, const char* name) {
  if (cond) { ++g_pass; printf("pass: %s\n", name); }
  else      { ++g_fail; printf("FAIL: %s\n", name); }
}

// Deterministic 64-byte identity from a seed (no CSPRNG in tests).
static void make_identity(uint32_t seed, uint8_t id[Handshake::IDENTITY_BYTES]) {
  uint32_t s = seed;
  for (int i = 0; i < Handshake::IDENTITY_BYTES; i += 4) {
    s = s * 1664525u + 1013904223u;
    id[i] = (uint8_t)(s & 0xFF);
    id[i + 1] = (uint8_t)((s >> 8) & 0xFF);
    id[i + 2] = (uint8_t)((s >> 16) & 0xFF);
    id[i + 3] = (uint8_t)((s >> 24) & 0xFF);
  }
}

int main() {
  Bench _b("PHASE6");
  // ---------------- handshake (ZKP) ----------------
  {
    uint8_t idA[64], idB[64], nonce[64];
    make_identity(0xAAAAu, idA);
    make_identity(0xBBBBu, idB);
    make_identity(0xCCCCu, nonce);

    Handshake a, b;
    a.set_identity(idA);
    b.set_identity(idB);

    // Each agent has a distinct public key derived from its identity.
    check(a.public_key() != b.public_key(), "handshake: distinct identities -> distinct public keys");

    // Honest prover A convinces verifier using A's public key.
    Handshake::Proof p;
    a.prove(nonce, &p);
    check(a.verify(a.public_key(), nonce, p), "handshake: honest proof verifies against own public key");
    check(!a.verify(b.public_key(), nonce, p), "handshake: proof does NOT verify against a different public key");

    // Same inputs are deterministic (replayable in tests).
    Handshake::Proof p2;
    a.prove(nonce, &p2);
    check(p.R == p2.R && p.z == p2.z && p.e == p2.e, "handshake: proof is deterministic for same inputs");

    // Tampered response fails verification.
    Handshake::Proof pbad = p;
    pbad.z ^= 1;  // flip a bit
    check(!a.verify(a.public_key(), nonce, pbad), "handshake: tampered proof is rejected");

    // Wrong nonce fails (challenge mismatch).
    uint8_t nonce2[64]; make_identity(0xDDDDu, nonce2);
    Handshake::Proof p3; a.prove(nonce2, &p3);
    check(!a.verify(a.public_key(), nonce, p3), "handshake: proof bound to a different nonce is rejected");

    // Degraded Teensy mode: raw identity exchange.
    uint8_t out[64];
    b.degraded_exchange(idA, out);
    check(std::memcmp(out, idA, 64) == 0, "handshake: degraded mode relays the raw 64-byte identity");
  }

  // ---------------- ontology_mapping ----------------
  {
    OntologyMapping om;
    // Remote ontology: id 10 at origin, id 11 at (1,0,...).
    Concept remote[2];
    remote[0].id = 10; for (int i = 0; i < 8; i++) remote[0].state[i] = 0.0f;
    remote[1].id = 11; for (int i = 0; i < 8; i++) remote[1].state[i] = 0.0f;
    remote[1].state[0] = 1.0f;

    float q1[8]; for (int i = 0; i < 8; i++) q1[i] = 0.0f;  // matches remote[0]
    uint32_t rid; float rd;
    check(om.map_nearest(q1, remote, 2, &rid, &rd) && rid == 10 && rd < 1e-5f,
          "ontology: nearest maps to the exact match (id 10)");

    float q2[8]; for (int i = 0; i < 8; i++) q2[i] = 0.0f; q2[0] = 1.0f;  // matches remote[1]
    check(om.map_nearest(q2, remote, 2, &rid, &rd) && rid == 11,
          "ontology: nearest maps to the close match (id 11)");

    // Empty remote -> no mapping.
    check(!om.map_nearest(q1, nullptr, 0, &rid, &rd), "ontology: empty remote yields no mapping");

    // Build a full map for several local concepts.
    Concept local[3];
    for (int i = 0; i < 3; i++) { local[i].id = 100 + i; for (int k = 0; k < 8; k++) local[i].state[k] = 0.0f; }
    local[1].state[0] = 1.0f;
    local[2].state[0] = 5.0f;  // far, but still nearest is remote[1]
    uint32_t olid[3], orid[3]; float od[3];
    int m = om.build_map(local, 3, remote, 2, olid, orid, od);
    check(m == 3, "ontology: build_map returns one entry per local concept");
    check(orid[0] == 10 && orid[1] == 11 && orid[2] == 11,
          "ontology: each local concept resolves to its nearest remote symbol");

    check(std::fabs(om.dist(q1, remote[0].state)) < 1e-5f, "ontology: dist computes Euclidean norm");
  }

  // ---------------- syncretic_fusion ----------------
  {
    SyncreticFusion sf;
    // A: two concepts; B: one overlapping A[0], one disjoint.
    Concept A[2], B[2];
    for (int i = 0; i < 8; i++) { A[0].state[i] = 0.0f; A[1].state[i] = 0.0f; B[0].state[i] = 0.0f; B[1].state[i] = 0.0f; }
    A[0].id = 1; A[1].id = 2;
    B[0].id = 20; B[0].state[0] = 0.05f;  // close to A[0] -> fuse
    B[1].id = 21; B[1].state[0] = 9.0f;    // far -> separate

    FusedConcept fused[8];
    int n = sf.fuse(A, 2, B, 2, 0.15f, fused, 8);
    check(n == 3, "fusion: 2 from A + 1 disjoint from B = 3 fused concepts");
    check(sf.merged_pairs(fused, n) == 1, "fusion: exactly one pair was merged (source == both)");
    bool hasBoth = false, hasOnlyB = false;
    for (int i = 0; i < n; i++) {
      if (fused[i].source == 3) hasBoth = true;
      if (fused[i].source == 2) hasOnlyB = true;
    }
    check(hasBoth && hasOnlyB, "fusion: fused lexicon records both merged and disjoint entries");

    // Respects the threshold: with a tiny threshold nothing merges.
    FusedConcept fused2[8];
    int n2 = sf.fuse(A, 2, B, 2, 0.01f, fused2, 8);
    check(n2 == 4 && sf.merged_pairs(fused2, n2) == 0, "fusion: tight threshold prevents merging");
  }

  // ---------------- schism_detector ----------------
  {
    SchismDetector sd;
    sd.set_threshold(0.10f);
    sd.set_streak(4);

    // Sustained high resonance -> no schism, coherence high.
    for (int i = 0; i < 10; i++) sd.observe(0.9f);
    check(!sd.schism(), "schism: sustained high resonance -> no schism");
    check(sd.coherence() > 0.8f, "schism: coherence reflects high resonance");

    // Sustained low resonance beyond the streak -> schism flagged.
    SchismDetector sd2;
    sd2.set_threshold(0.10f);
    sd2.set_streak(4);
    for (int i = 0; i < 6; i++) sd2.observe(-0.5f);
    check(sd2.schism(), "schism: sustained low resonance -> schism flagged");
    check(sd2.low_streak() >= 4, "schism: low streak accumulates over sustained divergence");

    // Brief dip below threshold does not trip the streak.
    SchismDetector sd3;
    sd3.set_threshold(0.10f);
    sd3.set_streak(4);
    sd3.observe(0.9f); sd3.observe(-0.5f); sd3.observe(0.9f); sd3.observe(-0.5f);
    check(!sd3.schism(), "schism: intermittent dips do not trip the streak");

    // Direct state observation: identical states resonate -> no schism.
    SchismDetector sd4;
    float s1[8], s2[8];
    for (int i = 0; i < 8; i++) { s1[i] = (float)i; s2[i] = (float)i; }
    for (int i = 0; i < 5; i++) sd4.observe_states(s1, s2);
    check(!sd4.schism() && sd4.coherence() > 0.99f, "schism: identical states -> resonance ~1, no schism");

    check(sd3.samples() == 4, "schism: samples counts observations");
  }

  printf("\nPHASE6_MIRROR_TEST: %s (%d pass, %d fail)\n",
         g_fail == 0 ? "PASS" : "FAIL", g_pass, g_fail);
  return g_fail == 0 ? 0 : 1;
}
