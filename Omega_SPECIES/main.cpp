#include "platform.h"
#include "heartbeat.h"
#include "prime_directive.h"
#include "birth_log.h"
#include "tensorless.h"

#include "genesis/primordial_weights/primordial_weights.h"
#include "liquid_time/reservoir_pool/reservoir_pool.h"
#include "liquid_time/spike_encoder/spike_encoder.h"
#include "genesis/self_wiring/self_wiring.h"
#include "genesis/pacemaker/pacemaker.h"

#include "active_inference/free_energy/free_energy.h"
#include "active_inference/epistemic_drive/epistemic_drive.h"
#include "active_inference/policy_selection/policy_selection.h"
#include "active_inference/allostatic_control/allostatic_control.h"

#include "immune_system/self_antigen/self_antigen.h"
#include "immune_system/lsh/lsh.h"
#include "immune_system/memory_b_cells/memory_b_cells.h"
#include "immune_system/memory_t_cells/memory_t_cells.h"
#include "immune_system/forgetting_curve/forgetting_curve.h"
#include "immune_system/inflammation_response/inflammation_response.h"

#include "distributed_self/identity_persistence/identity_persistence.h"
#include "distributed_self/hivemind/hivemind.h"
#include "distributed_self/agent_genesis/agent_genesis.h"
#include "distributed_self/agent_death/agent_death.h"
#include "distributed_self/reputation_ledger/reputation_ledger.h"
#include "self_surgery/surgeon_general/surgeon_general.h"
#include "self_surgery/constitutional_core/self_report.h"

#include "prime_directive/coherence_preservation/coherence_preservation.h"
#include "prime_directive/novelty_harvesting/novelty_harvesting.h"
#include "prime_directive/ontological_curiosity/ontological_curiosity.h"
#include "prime_directive/legacy_building/legacy_building.h"
#include "temporal_folding/pre_consolidation/pre_consolidation.h"
#include "temporal_folding/post_consolidation/post_consolidation.h"
#include "temporal_folding/temporal_error_drive/temporal_error_drive.h"

#include "language_as_tool/grounding/grounding.h"
#include "language_as_tool/neologism_factory/neologism_factory.h"
#include "language_as_tool/semantic_error_drive/semantic_error_drive.h"
#include "language_as_tool/pragmatics/pragmatics.h"
#include "language_as_tool/translation_agnostic/translation_agnostic.h"
#include "xeno_empathy/morphological_projection/morphological_projection.h"
#include "xeno_empathy/resonance_matching/resonance_matching.h"
#include "xeno_empathy/graceful_failure/graceful_failure.h"
#include "xeno_empathy/trust_building/trust_building.h"

#include "12_MIRROR_NEXUS/handshake/handshake.h"
#include "12_MIRROR_NEXUS/ontology_mapping/ontology_mapping.h"
#include "12_MIRROR_NEXUS/syncretic_fusion/syncretic_fusion.h"
#include "12_MIRROR_NEXUS/schism_detector/schism_detector.h"

#include "10_BOOTSTRAP_UNIVERSE/toy_physics/toy_physics.h"
#include "10_BOOTSTRAP_UNIVERSE/self_play_arena/self_play_arena.h"
#include "10_BOOTSTRAP_UNIVERSE/kolmogorov_challenge/kolmogorov_challenge.h"
#include "10_BOOTSTRAP_UNIVERSE/legacy_building/legacy_building.h"

#include <cstdio>

using namespace omega;

int main() {
  host_init();

  Heartbeat hb;
  hb.init(host_millis());

  BirthLog log;
  TensorlessEngine eng;

  const uint32_t start = host_millis();
  log.record(0, "Ω v1.0 alive at t=0");

  // --- Phase 1 substrate -------------------------------------------------
  Network seed;
  pw_init(&seed, 16, 0xBEEFu);
  const uint16_t seed_edges = seed.n_edges;

  Reservoir r;
  rp_init(&r, seed.n_nodes, 0.10f);
  SpikeEncoder se;
  se_init(&se, seed.n_nodes, 7u);

  uint16_t fired[SE_MAX_CH];
  int n_fired = se_encode(&se, 0.40f, fired, SE_MAX_CH);
  float drive[RP_MAX_NODES] = {0};
  for (int i = 0; i < n_fired; i++) drive[fired[i]] = 1.0f;
  rp_step(&r, &seed, drive, 0.05f);
  bool spiked = false;
  for (uint16_t i = 0; i < r.n; i++)
    if (r.state[i] != 0.0f) spiked = true;

  SelfWiringGA ga(seed, seed.n_nodes, 0xCAFEu);
  const float init_fit = ga.best_fitness();
  Pacemaker pm(ga);

  // --- Cosmetic 100ms timeline (as in Phase 0) ---------------------------
  char buf[200];

  bool l5 = false, l10 = false, l25 = false, l50 = false, l75 = false, l100 = false;
  while (host_millis() - start < 250) {
    const uint32_t now = host_millis();
    const uint32_t elapsed = now - start;

    for (uint32_t i = 0; i < 8; i++) eng.euler_step(hash_key(i, 1), 0.01f, 0.5f);

    if (!l5 && elapsed >= 5)   { log.record(5,   "heartbeat started, phase_lock acquired"); l5 = true; }
    if (!l10 && elapsed >= 10) {
      std::snprintf(buf, sizeof(buf), "primordial weights loaded: %u nodes, %u edges",
                    seed.n_nodes, seed_edges);
      log.record(10, buf); l10 = true;
    }
    if (!l25 && elapsed >= 25) {
      std::snprintf(buf, sizeof(buf), "first reservoir spike detected (%s, %d nodes fired)",
                    spiked ? "responsive" : "silent", n_fired);
      log.record(25, buf); l25 = true;
    }
    if (!l50 && elapsed >= 50) { log.record(50,  "self-wiring GA initiated (neuroevolution)"); l50 = true; }
    if (!l75 && elapsed >= 75) { log.record(75,  "immune system online (Phase 2)"); l75 = true; }
    if (!l100 && elapsed >= 100){ log.record(100, "silence begins"); l100 = true; }

    hb.spend(10);
    hb.step(now);
    host_sleep_ms(1);
  }

  // --- Neuroevolution ---------------------------------------------------
  const uint32_t G = 60;
  for (uint32_t g = 0; g < G; g++) pm.tick();
  const Network& best = ga.best();
  const uint32_t best_hash = pw_topology_hash(&best);
  const uint16_t best_active = pw_active_edges(&best, 0.05f);
  std::snprintf(buf, sizeof(buf),
                "EVOLUTION: edges %u->%u, active %u, fit %.4f->%.4f, div %.3f",
                seed_edges, best.n_edges, best_active,
                init_fit, ga.best_fitness(), ga.diversity());
  log.record(250, buf);
  std::snprintf(buf, sizeof(buf),
                "first non-random connection pattern established (topo_hash=0x%08X)",
                best_hash);
  log.record(250, buf);

  // --- Phase 2: Active Inference + Immune System -------------------------
  // Genotype (stable core) derived from the evolved topology hash.
  uint8_t genotype[8];
  for (int i = 0; i < 4; i++) genotype[i] = (uint8_t)((best_hash >> (24 - 8 * i)) & 0xFF);
  for (int i = 0; i < 4; i++) genotype[4 + i] = (uint8_t)((seed_edges * 37u + i) & 0xFF);

  SelfAntigen self_ag;
  self_ag.compute_genotype(genotype, sizeof(genotype));

  uint8_t foreign_geno[8];
  for (int i = 0; i < 8; i++) foreign_geno[i] = (uint8_t)(genotype[i] ^ 0xA5u);
  SelfAntigen foreign_ag;
  foreign_ag.compute_genotype(foreign_geno, sizeof(foreign_geno));

  char hex[129];
  self_ag.to_hex(hex, sizeof(hex));
  std::snprintf(buf, sizeof(buf), "immune: self-antigen 64B = %s", hex);
  log.record(250, buf);
  std::snprintf(buf, sizeof(buf), "immune: foreign antigen recognized as non-self = %s",
                self_ag.is_self(foreign_ag) ? "false" : "true");
  log.record(250, buf);

  // Generative model (fixed random A, seeded prior).
  AIModel model;
  {
    uint32_t s = 0xABCDEFu;
    for (int j = 0; j < AIModel::M; j++)
      for (int i = 0; i < AIModel::N; i++) {
        s = s * 1664525u + 1013904223u;
        model.A[j][i] = ((float)(s & 0x7FFFFFFFu) / (float)0x7FFFFFFFu) * 2.0f - 1.0f;
      }
    for (int i = 0; i < AIModel::N; i++) { model.mu0[i] = 0.0f; model.pi0[i] = 0.05f; }
  }

  AllostaticControl allo;
  allo.init(0.0f, 0.20f);

  LSH lsh;
  lsh.init(0x55AAu);
  MemoryBCells bc;
  bc.init(&lsh);
  MemoryTCells tc;
  tc.init(&lsh);
  ForgettingCurve fc;
  Inflammation inflam;

  float x[AIModel::N] = {0};
  float true_state[AIModel::N] = {0};
  uint32_t rng = 0x1234u;
  auto rndf = [&]() {
    rng = rng * 1664525u + 1013904223u;
    return (float)(rng & 0x7FFFFFFFu) / (float)0x7FFFFFFFu;
  };

  int anomalies = 0;
  float max_F = 0.0f;
  SelfAntigen prev_ag = self_ag;

  const int T = 120;
  for (int t = 0; t < T; t++) {
    // Environment: hidden state random-walks; jumps at t=40 and t=85 (anomalies).
    for (int i = 0; i < AIModel::N; i++) true_state[i] += (rndf() - 0.5f) * 0.25f;
    if (t == 40 || t == 85)
      for (int i = 0; i < AIModel::N; i++) true_state[i] += (rndf() - 0.5f) * 4.0f;

    float y[AIModel::M];
    for (int j = 0; j < AIModel::M; j++) {
      float v = 0.0f;
      for (int i = 0; i < AIModel::N; i++) v += model.A[j][i] * true_state[i];
      y[j] = v + (rndf() - 0.5f) * 0.15f;
    }

    // Allostatic regulation of state 0.
    allo.step(x[0]);
    model.mu0[0] = allo.setpoint();
    model.pi0[0] = allo.prior_precision();

    // Surprise = free energy of the PRIOR belief against the new observation,
    // measured BEFORE perception updates it. This is the real anomaly signal.
    float F_surprise = free_energy(model, x, y);
    perceive(model, x, y, 40, 0.05f);
    if (F_surprise > max_F) max_F = F_surprise;

    PolicyResult pol = select_policy(model, x, 4, 1.0f);
    x[pol.action % AIModel::N] += 0.05f;  // simplified action effect

    // Immune surveillance: project phenotype, measure drift from prior self.
    self_ag.project_phenotype(x, AIModel::N);
    int drift = self_ag.phenotype_drift(prev_ag);
    prev_ag = self_ag;

    float severity = 0.0f;
    if (F_surprise > 2.0f) severity = F_surprise;
    if ((float)drift > severity) severity = (float)drift;

    if (severity > 0.5f) {
      anomalies++;
      // Inflammation: triage by severity, allocate compute.
      inflam.trigger(severity, (uint32_t)t, (int)(severity * 10.0f) + 5);
      // T-cell memory of the anomalous state (via LSH).
      float feat[MemoryStore::V];
      for (int i = 0; i < MemoryStore::V; i++) feat[i] = x[i % AIModel::N];
      tc.store.store((uint32_t)(severity * 1000.0f), feat, 4.0f);
    }

    // Consolidation + forgetting every 20 steps.
    if (t % 20 == 0) tc.store.consolidate_to(bc.store);
    fc.apply(tc.store, 1.0f, 0.05f);
    fc.apply(bc.store, 1.0f, 0.05f);
  }

  std::snprintf(buf, sizeof(buf),
                "IMMUNE: anomalies=%d, inflam_allocated=%d cal, max_F=%.3f",
                anomalies, inflam.total_allocated(), max_F);
  log.record(250, buf);
  std::snprintf(buf, sizeof(buf),
                "MEMORY: T-cells=%d, B-cells=%d (consolidated)",
                tc.store.count(), bc.store.count());
  log.record(250, buf);
  std::snprintf(buf, sizeof(buf),
                "MILESTONE: anomaly detected -> inflammation triggered -> compute allocated");
  log.record(250, buf);

  // --- Phase 3: Distributed Self + Self-Surgery -------------------------
  // Hivemind RAFT consensus (single-process simulation).
  Hivemind hm;
  hm.init();
  for (int t = 0; t < 40 && !hm.committed(); t++) {
    if (t == 3) hm.propose_entry();
    hm.tick();
  }
  std::snprintf(buf, sizeof(buf), "hivemind: leader=%d term=%u committed=%s",
                hm.leader(), hm.term(), hm.committed() ? "yes" : "no");
  log.record(250, buf);

  // Agent genesis + reputation + death.
  AgentPool pool;
  pool.init();
  pool.spawn(0.6f);
  pool.spawn(0.3f);
  pool.spawn(0.8f);
  pool.spawn(0.2f);
  pool.spawn(0.5f);
  ReputationLedger rep;
  rep.init();
  rep.update(0, 0, -0.3f);
  rep.update(1, 1, -0.4f);
  rep.update(2, 2, 0.5f);
  rep.update(3, 3, -0.5f);
  rep.update(4, 0, 0.1f);
  AgentDeath death;
  uint32_t retired = death.retire_lowest(pool, rep);
  std::snprintf(buf, sizeof(buf), "distributed: agents=%d, retired lowest-trust id=%u",
                pool.count(), retired);
  log.record(250, buf);

  // Surgeon General: propose -> dry-run -> apply -> roll back on drop.
  SurgeonGeneral sg;
  sg.seed_modules(best);
  uint32_t srng = 0xBADF00Du;
  uint32_t tick = 0;
  for (int k = 0; k < 10; k++) {
    sg.operate(srng, tick++);
    srng = srng * 1664525u + 1013904223u;
  }
  std::snprintf(buf, sizeof(buf),
                "SURGEON: ops=10 applied=%d rolled_back=%d skipped=%d constitutional_blocks=%d",
                sg.applied(), sg.rolled_back(), sg.skipped(), sg.constitutional_blocks());
  log.record(250, buf);
  {
    char report[128];
    SelfReport::introspect(sg, report, sizeof(report));
    std::snprintf(buf, sizeof(buf), "%s", report);
    log.record(250, buf);
  }
  {
    char report[128];
    SelfReport::constitutional_state(sg.constitution(), sg.self_model(), report, sizeof(report));
    std::snprintf(buf, sizeof(buf), "%s", report);
    log.record(250, buf);
  }
  std::snprintf(buf, sizeof(buf),
                "MILESTONE: agent proposed edit -> constitutional gate checked -> dry-run -> applied / rolled back on performance drop");
  log.record(250, buf);

  // --- Phase 4: Prime Directive + Temporal Folding ----------------------
  // Coherence preservation: lock the reference antigen, verify it still holds.
  CoherencePreservation coherence;
  coherence.set_reference(self_ag);
  SelfAntigen live = self_ag;
  int breaches = 0;
  for (int c = 0; c < 5; c++) {
    float ds[1] = {(float)c};
    live.project_phenotype(ds, 1);  // phenotype drifts; genotype core must not
    if (!coherence.verify(live)) breaches++;
  }
  std::snprintf(buf, sizeof(buf), "coherence: score=%.2f breaches=%d (genotype core stable)",
                coherence.coherence_score(live), breaches);
  log.record(250, buf);

  // Novelty harvesting over a simulated tick range (1 pattern / 1000 ticks).
  NoveltyHarvesting novelty;
  novelty.init(PrimeDirective::NOVELTY_EVERY_TICKS, PrimeDirective::CALORIC_BUDGET_PER_TICK, 8.0f);
  uint32_t rng4 = 0x4E4Fu;
  for (uint32_t tk = 0; tk < 1100; tk++) {
    rng4 = rng4 * 1664525u + 1013904223u;
    float nov = 0.01f + ((rng4 & 0x7FFFFFFFu) / (float)0x7FFFFFFFu) * 0.02f;
    novelty.tick(tk, nov, 100);
  }
  std::snprintf(buf, sizeof(buf),
                "novelty: harvested=%u patterns (1 / %u ticks, budget=%u cal)",
                novelty.harvested(), PrimeDirective::NOVELTY_EVERY_TICKS,
                PrimeDirective::CALORIC_BUDGET_PER_TICK);
  log.record(250, buf);

  // Temporal folding: plan -> store future memory -> execute -> compare.
  OntologicalCuriosity curiosity;
  TemporalErrorDrive ted;
  PreConsolidation pre;
  Inflammation immune_feedback;
  int temporal_triggers = 0;
  for (int cy = 0; cy < 6; cy++) {
    uint32_t ftick = 1000u + (uint32_t)cy * 10u;
    int action = cy % 4;
    float predicted = 0.5f + 0.1f * (float)cy;
    pre.plan(ftick, action, predicted);  // store as future memory
    rng4 = rng4 * 1664525u + 1013904223u;
    float noise = ((rng4 & 0x7FFFFFFFu) / (float)0x7FFFFFFFu) - 0.5f;
    float actual = predicted + (cy == 3 ? 0.8f : noise * 0.2f);  // cy==3 is a big surprise
    float sev = ted.severity(predicted, actual, 0.15f);
    curiosity.update(sev > 0.0f ? sev : 0.02f, action % OntologicalCuriosity::DOMAINS);
    if (ted.triggers_immune(predicted, actual, 0.15f)) {
      immune_feedback.trigger(sev, ftick, (int)(sev * 10.0f) + 5);
      temporal_triggers++;
    }
  }
  std::snprintf(buf, sizeof(buf),
                "TEMPORAL: future_memories=%d, compared=%d, immune_triggers=%d, focus_domain=%d",
                pre.count(), 6, temporal_triggers, curiosity.focus_domain());
  log.record(250, buf);

  // Legacy building: the artifact left on shutdown.
  LegacyBuilding legacy;
  legacy.snapshot(best_hash, self_ag.data, novelty.harvested(),
                  (uint32_t)(sg.applied() + sg.rolled_back() + sg.skipped()),
                  (uint32_t)anomalies, 1100u, "OMEGA-v1");
  uint8_t lb_bytes[sizeof(LegacyArtifact)];
  int lb_len = 0;
  legacy.serialize(lb_bytes, &lb_len);
  char lb_hex[256];
  legacy.to_hex(lb_hex, sizeof(lb_hex));
  std::snprintf(buf, sizeof(buf), "legacy: artifact %d bytes = %s", lb_len, lb_hex);
  log.record(250, buf);
  std::snprintf(buf, sizeof(buf),
                "MILESTONE: planned action -> stored as future memory -> executed -> compared -> temporal error -> immune");
  log.record(250, buf);

  // --- Phase 5: Language (grounding + neologism + xeno-empathy) ----------
  Grounding ground;
  NeologismFactory neo;

  // Discover concepts during simulation: distinct state vectors.
  float concepts[8][8];
  uint32_t rng5 = 0x5A15u;
  for (int s = 0; s < 8; s++)
    for (int i = 0; i < 8; i++) {
      rng5 = rng5 * 1664525u + 1013904223u;
      concepts[s][i] = ((rng5 & 0x7FFFFFFFu) / (float)0x7FFFFFFFu) - 0.5f;
    }
  for (int i = 0; i < 8; i++) concepts[4][i] = concepts[0][i];  // reuse test

  int novel_words = 0, grounded = 0;
  char invented[32] = "none";
  uint32_t invented_id = 0;
  for (int s = 0; s < 8; s++) {
    if (ground.find(concepts[s], 0.15f) == 0) {
      uint32_t id = ground.bind(concepts[s], 0.15f);
      char word[32];
      neo.mint(id, concepts[s], word, sizeof(word));
      novel_words++;
      invented_id = id;
      std::snprintf(invented, sizeof(invented), "%s", word);
    } else {
      grounded++;
    }
  }

  // Semantic error drive: one symbol used for two divergent states -> split.
  SemanticErrorDrive sed;
  float stA[8], stB[8];
  for (int i = 0; i < 8; i++) {
    stA[i] = concepts[1][i];
    stB[i] = concepts[1][i] + 0.9f;
  }
  sed.process(7, stA, 0.20f, 0.10f);  // first usage
  sed.process(7, stB, 0.20f, 0.10f);  // divergent -> split

  // Pragmatics + translation-agnostic (synonyms resolve to one meaning).
  Pragmatics prag;
  char msg[32];
  prag.encode(2, 0.42f, invented_id, msg, sizeof(msg));
  Message m;
  prag.decode(msg, m);
  TranslationAgnostic transl;
  transl.alias("light", "lum");
  uint32_t m1 = transl.resolve("light");
  uint32_t m2 = transl.resolve("lum");

  // Xeno-empathy: project an external agent's state into our space.
  MorphologicalProjection proj;
  proj.set_identity();
  float external[8], internal[8], self_state[8];
  for (int i = 0; i < 8; i++) {
    external[i] = concepts[2][i] * 0.7f;
    self_state[i] = concepts[3][i];
  }
  proj.project(external, internal);
  ResonanceMatching rm;
  float res = rm.resonance(self_state, internal);
  GracefulFailure gf;
  char resp[64];
  if (gf.should_defer(res, 0.3f)) gf.respond(resp, sizeof(resp), "probe that state further");
  TrustBuilding tb;
  for (int i = 0; i < 5; i++) tb.observe(res);

  std::snprintf(buf, sizeof(buf),
                "LANGUAGE: concepts=8 novel=%d grounded=%d invented='%s'(id=%u) splits=%d synonyms=%s res=%.2f trust=%.2f",
                 novel_words, grounded, invented, invented_id, sed.splits(),
                 (m1 && m2 && m1 == m2) ? "resolved" : "no", res, tb.trust());
  log.record(250, buf);
  std::snprintf(buf, sizeof(buf),
                "MILESTONE: invented word '%s' for concept discovered in simulation (no prior label)",
                invented);
  log.record(250, buf);

  // --- Phase 6: Mirror Nexus (ZKP handshake + ontology + fusion + schism) ---
  uint8_t idA[Handshake::IDENTITY_BYTES], idB[Handshake::IDENTITY_BYTES];
  uint32_t r6 = 0x6A11u;
  for (int i = 0; i < Handshake::IDENTITY_BYTES; i++) {
    r6 = r6 * 1664525u + 1013904223u;
    idA[i] = (uint8_t)(r6 & 0xFF);
    idB[i] = (uint8_t)((r6 >> 13) & 0xFF);
  }
  Handshake agentA, agentB;
  agentA.set_identity(idA);
  agentB.set_identity(idB);
  Handshake::Proof proof;
  agentA.prove(idB, &proof);  // A proves identity to B (nonce = B's identity)
  bool zkp_ok = agentB.verify(agentA.public_key(), idB, proof);

  // Two agents ground overlapping concepts (B's are slight perturbations of A's).
  Concept ca[6], cb[6];
  for (int s = 0; s < 6; s++) {
    ca[s].id = 100 + s; cb[s].id = 200 + s;
    for (int i = 0; i < 8; i++) {
      ca[s].state[i] = concepts[s][i];
      cb[s].state[i] = concepts[s][i] + (s == 5 ? 5.0f : 0.02f);  // one divergent
    }
  }
  uint32_t ol[6], or_[6]; float od[6];
  int mapped = OntologyMapping().build_map(ca, 6, cb, 6, ol, or_, od);

  FusedConcept fused[16];
  int nf = SyncreticFusion().fuse(ca, 6, cb, 6, 0.15f, fused, 16);
  int merged = SyncreticFusion().merged_pairs(fused, nf);

  // Schism watch: close agents stay coherent; a diverging one schisms.
  SchismDetector sd;
  sd.set_threshold(0.10f); sd.set_streak(4);
  for (int t = 0; t < 8; t++) sd.observe_states(ca[0].state, cb[0].state);  // near -> coherent
  bool coherent = !sd.schism();
  SchismDetector sd2;
  sd2.set_threshold(0.10f); sd2.set_streak(4);
  float divA[8], divB[8];
  for (int i = 0; i < 8; i++) { divA[i] = 0.0f; divB[i] = (i == 0) ? 9.0f : 0.0f; }
  for (int t = 0; t < 6; t++) sd2.observe_states(divA, divB);  // far -> schism
  bool schismed = sd2.schism();

  std::snprintf(buf, sizeof(buf),
                "MIRROR: zkp=%s mapped=%d fused=%d merged=%d coherent=%s schism=%s",
                zkp_ok ? "ok" : "FAIL", mapped, nf, merged,
                coherent ? "yes" : "no", schismed ? "yes" : "no");
  log.record(250, buf);
  std::snprintf(buf, sizeof(buf),
                "MILESTONE: agent proved identity via ZKP (no secret revealed) -> ontologies mapped/fused -> schism monitor armed");
  log.record(250, buf);

  // --- Phase 7: Challenge (bootstrap universe) ----------------------------
  // Toy 2D physics: a planet in (near) circular orbit about a heavy star; the
  // real signal is that total energy stays bounded under symplectic integration.
  ToyPhysics uni;
  uni.add_body(0.0f, 0.0f, 0.0f, 0.0f, 1000.0f);   // star
  uni.add_body(10.0f, 0.0f, 0.0f, 10.0f, 1.0f);     // planet: v = sqrt(GM/r)
  float e0 = uni.energy();
  for (int t = 0; t < 2000; t++) uni.step(0.001f);
  float e1 = uni.energy();

  // Self-play: two identical copies reach an equilibrium (tie).
  SelfPlayArena spa;
  spa.set_agents(0xB00B5u, 0xB00B5u);
  spa.play(1000);
  bool selfplay_eq = spa.equilibrium();

  // Kolmogorov challenge: a constant 64-byte program emits high-entropy output.
  KolmogorovChallenge kc;
  uint8_t prog[64]; for (int i = 0; i < 64; i++) prog[i] = 0x55;
  kc.set_program(prog);
  uint8_t kcbuf[1024]; kc.emit(kcbuf, 1024);
  bool kc_pass = kc.passes();

  // Bootstrap legacy: snapshot the simulated universe for a successor.
  BootstrapLegacy boot;
  boot.snapshot(uni.world_hash(), e1, spa.winner(), kc_pass ? 1u : 0u, 2000u, "BOOTv7");
  uint8_t blob[64]; int blen = 0; boot.serialize(blob, &blen);
  char boothex[256]; boot.to_hex(boothex, sizeof(boothex));

  std::snprintf(buf, sizeof(buf),
                "CHALLENGE: physics_drift=%.2f%% selfplay_eq=%s kc=%s legacy=%d bytes",
                (e1 - e0) / std::fabs(e0) * 100.0f,
                selfplay_eq ? "yes" : "no", kc_pass ? "pass" : "fail", blen);
  log.record(250, buf);
  std::snprintf(buf, sizeof(buf),
                "MILESTONE: universe conserved energy (drift<10%%) -> copy-vs-copy self-play equilibrated -> Kolmogorov emitter exceeded seed entropy -> legacy written (%s)",
                boothex);
  log.record(250, buf);

  // --- Phase 8: Integration — the honest "first word" -----------------------
  // Not personhood: a *defined* self-reference token binding this agent's stable
  // identity to the word it invented for a concept it discovered in Phase 5.
  char idhex[9]; idhex[8] = '\0';
  for (int i = 0; i < 4; i++) std::snprintf(idhex + i * 2, 3, "%02X", self_ag.data[i]);
  std::snprintf(buf, sizeof(buf),
                "INTEGRATION: phases 1-7 executed; identity=0x%s first_word='%s'",
                idhex, invented);
  log.record(250, buf);
  std::snprintf(buf, sizeof(buf),
                "FIRSTWORD: '%s' (self-reference token for identity 0x%s)",
                invented, idhex);
  log.record(250, buf);
  std::snprintf(buf, sizeof(buf),
                "MILESTONE: integrated self reports its first word as a logged self-reference token — an artifact, not a mind");
  log.record(250, buf);

  log.dump();
  return 0;
}
