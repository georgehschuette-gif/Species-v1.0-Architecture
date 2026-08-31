// Phase 5 "Language" — enterprise-grade unit suite.
// Covers the language-as-tool substrate (grounding, semantic_error_drive,
// neologism_factory) and the xeno-empathy layer (morphological_projection,
// resonance_matching, trust_building). All tests are deterministic and bounded.

#include <cstdio>
#include <cmath>
#include <cstdint>
#include <cstring>
#include "language_as_tool/grounding/grounding.h"
#include "language_as_tool/semantic_error_drive/semantic_error_drive.h"
#include "language_as_tool/neologism_factory/neologism_factory.h"
#include "xeno_empathy/morphological_projection/morphological_projection.h"
#include "xeno_empathy/resonance_matching/resonance_matching.h"
#include "xeno_empathy/trust_building/trust_building.h"

#include "bench.h"
using namespace omega;

static int g_pass = 0;
static int g_fail = 0;

static void check(bool cond, const char* name) {
  if (cond) { ++g_pass; printf("pass: %s\n", name); }
  else      { ++g_fail; printf("FAIL: %s\n", name); }
}

static void zero_state(float* s) { for (int i = 0; i < 8; i++) s[i] = 0.0f; }

int main() {
  Bench _b("PHASE5");
  // ---------------- grounding ----------------
  {
    Grounding g;
    float s1[8]; zero_state(s1); s1[0] = 0.50f;
    float s2[8]; zero_state(s2); s2[2] = 0.70f;

    uint32_t id1 = g.bind(s1);
    check(id1 == 1, "grounding: first bind mints id 1");
    check(g.count() == 1, "grounding: count is 1 after first bind");

    // Same state is found, not re-minted.
    uint32_t id1b = g.bind(s1);
    check(id1b == id1, "grounding: identical state resolves to same symbol");
    check(g.count() == 1, "grounding: identical state does not grow table");

    // Near state (dist 0.05 < 0.15) folds into the existing symbol.
    float s1near[8]; zero_state(s1near); s1near[0] = 0.55f;
    check(g.find(s1near) == id1, "grounding: near state finds existing symbol");

    // Distinct state mints a new symbol.
    uint32_t id2 = g.bind(s2);
    check(id2 != id1 && id2 != 0, "grounding: distinct state mints new symbol");
    check(g.count() == 2, "grounding: count grows for distinct state");

    // Far state is not found.
    float sFar[8]; zero_state(sFar); sFar[0] = 5.0f;
    check(g.find(sFar) == 0, "grounding: far state has no close symbol");

    // lookup round-trips the stored state.
    float out[8];
    check(g.lookup(id1, out) && std::fabs(out[0] - 0.50f) < 1e-5f,
          "grounding: lookup returns the grounded state");

    // dist is Euclidean.
    float a[8], b[8]; zero_state(a); zero_state(b); a[0] = 3.0f;
    check(std::fabs(Grounding::dist(a, b) - 3.0f) < 1e-5f,
          "grounding: dist is Euclidean norm");

    // Capacity cap: CAP=32. After filling, a new state returns 0.
    Grounding gc;
    int minted = 0;
    for (int k = 0; k < Grounding::CAP + 4; k++) {
      float s[8]; zero_state(s); s[0] = (float)(k + 1) * 10.0f;
      uint32_t id = gc.bind(s);
      if (id != 0) minted++;
    }
    check(minted == Grounding::CAP, "grounding: table capped at CAP entries");
    check(gc.count() == Grounding::CAP, "grounding: count never exceeds CAP");
  }

  // ---------------- neologism_factory ----------------
  {
    NeologismFactory f;
    float sa[8]; zero_state(sa);
    float sb[8]; zero_state(sb); sb[0] = 0.5f;

    char ta[32], tb[32];
    f.mint(1, sa, ta, sizeof(ta));
    f.mint(1, sa, tb, sizeof(tb));
    check(std::strcmp(ta, tb) == 0, "neologism: mint is deterministic for same input");
    check(ta[0] == 'w' && std::strchr(ta, '-') != nullptr,
          "neologism: token has 'w' prefix and '-' separator");

    char tc[32]; f.mint(2, sa, tc, sizeof(tc));
    check(std::strcmp(ta, tc) != 0, "neologism: different concept id -> different token");

    char td[32]; f.mint(1, sb, td, sizeof(td));
    check(std::strcmp(ta, td) != 0, "neologism: different state -> different token");

    // Small buffer is respected (null terminated, 'w' prefix present).
    char small[4]; f.mint(7, sa, small, sizeof(small));
    check(small[0] == 'w' && small[3] == '\0', "neologism: small buffer stays null-terminated");
  }

  // ---------------- semantic_error_drive ----------------
  {
    // First usage records reference (divergence 0), no split.
    SemanticErrorDrive sed;
    float sA[8]; zero_state(sA);
    float sFar[8]; zero_state(sFar); sFar[0] = 1.0f;

    check(sed.process(1, sA) == 0.0f, "semantic: first usage has zero divergence");
    check(sed.splits() == 0, "semantic: no split on first usage");

    // Same usage stays within tolerance -> no split.
    check(sed.process(1, sA) == 0.0f, "semantic: repeated same state -> zero divergence");
    check(sed.splits() == 0, "semantic: no split for identical reuse");

    // Far reuse diverges beyond tolerance AND split_threshold -> split counted.
    float d = sed.process(1, sFar);
    check(d > 0.10f, "semantic: far reuse yields divergence above split threshold");
    check(sed.splits() == 1, "semantic: split counted on large divergence");

    // Mild divergence (between tolerance 0.20 and split 0.10) is KEPT, not split.
    SemanticErrorDrive sed2;
    float sB[8]; zero_state(sB);
    sed2.process(1, sB);                       // reference
    float sMid[8]; zero_state(sMid); sMid[0] = 0.25f;  // dist 0.25
    float dmid = sed2.process(1, sMid, 0.20f, 0.10f);
    check(dmid > 0.0f && sed2.splits() == 0,
          "semantic: divergence within split threshold is KEPT (no split)");
  }

  // ---------------- morphological_projection ----------------
  {
    MorphologicalProjection mp;
    mp.set_identity();
    float s[8]; for (int i = 0; i < 8; i++) s[i] = (float)(i + 1);
    float out[8]; mp.project(s, out);
    bool same = true;
    for (int i = 0; i < 8; i++) if (std::fabs(out[i] - s[i]) > 1e-5f) same = false;
    check(same, "morph: identity matrix projects state to itself");

    // Diagonal scale: internal[i] = 2*external[i] for i=0, 3*external[i] for i=1.
    mp.set_identity();
    mp.set(0, 0, 2.0f);
    mp.set(1, 1, 3.0f);
    float ext[8]; for (int i = 0; i < 8; i++) ext[i] = (float)(i + 1);
    float proj[8]; mp.project(ext, proj);
    check(std::fabs(proj[0] - 2.0f * ext[0]) < 1e-5f &&
          std::fabs(proj[1] - 3.0f * ext[1]) < 1e-5f &&
          std::fabs(proj[2] - ext[2]) < 1e-5f,
          "morph: custom matrix applies linear transform correctly");
  }

  // ---------------- resonance_matching ----------------
  {
    ResonanceMatching rm;
    float a[8]; zero_state(a); a[0] = 1.0f;
    float b[8]; zero_state(b); b[0] = 1.0f;
    check(std::fabs(rm.resonance(a, b) - 1.0f) < 1e-5f,
          "resonance: identical vectors -> 1.0");

    float c[8]; zero_state(c); c[1] = 1.0f;
    check(std::fabs(rm.resonance(a, c) - 0.0f) < 1e-5f,
          "resonance: orthogonal vectors -> 0.0");

    float d2[8]; zero_state(d2); d2[0] = -1.0f;
    check(std::fabs(rm.resonance(a, d2) + 1.0f) < 1e-5f,
          "resonance: anti-parallel vectors -> -1.0");

    float z[8]; zero_state(z);
    check(rm.resonance(z, z) == 0.0f, "resonance: zero vector -> 0.0 (no NaN)");
  }

  // ---------------- trust_building ----------------
  {
    TrustBuilding tb;
    check(std::fabs(tb.trust() - 0.5f) < 1e-5f, "trust: starts at 0.5");

    // First observation merely seeds the baseline; trust unchanged.
    tb.observe(0.80f);
    check(std::fabs(tb.trust() - 0.5f) < 1e-5f, "trust: first observe seeds, no change");

    // Stable resonance (small delta) raises trust.
    tb.observe(0.81f);
    check(tb.trust() > 0.5f, "trust: stable resonance raises trust");

    // Sustained stability climbs further.
    for (int i = 0; i < 5; i++) tb.observe(0.80f);
    check(tb.trust() > 0.6f, "trust: sustained stability climbs trust");

    // Volatile resonance lowers trust.
    float before = tb.trust();
    tb.observe(-0.9f);
    check(tb.trust() < before, "trust: volatile resonance lowers trust");

    // Clamp to [0,1]: many volatile observations never go negative.
    for (int i = 0; i < 200; i++) tb.observe((i % 2) ? 0.9f : -0.9f);
    check(tb.trust() >= 0.0f && tb.trust() <= 1.0f,
          "trust: remains clamped to [0,1] under volatility");
  }

  printf("\nPHASE5_LANGUAGE_TEST: %s (%d pass, %d fail)\n",
         g_fail == 0 ? "PASS" : "FAIL", g_pass, g_fail);
  return g_fail == 0 ? 0 : 1;
}
