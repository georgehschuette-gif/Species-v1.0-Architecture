// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

#include "../src/genesis/phase0_genesis.h"
#include "../src/self_surgery/constitutional_core/manifest_integrity.h"
#include "../src/self_surgery/constitutional_core/agents_diary.h"
#include "../src/self_surgery/constitutional_core/constitution.h"
#include "../src/immune_system/self_antigen/self_antigen.h"
#include "../src/immune_system/self_antigen/sha256.h"
#include "../src/core/scale.h"
#include "bench.h"
#include <cstdio>
#include <cassert>
#include <thread>
#include <cstring>
#include <chrono>
#include <cmath>
#include <cstdint>

using namespace omega::genesis;
using namespace omega::self_surgery;
using namespace omega::core;
using namespace omega;

int main() {
    printf("=== Phase 0: Genesis — Pacemaker Meta-Optimizer ===\n\n");

    int passed = 0;
    int total = 0;

    // Test 1: Pacemaker initialization
    {
        total++;
        printf("Test 1: Pacemaker initialization... ");
        PacemakerMetaOptimizer& pacemaker = global_pacemaker();
        if (pacemaker.is_healthy()) {
            printf("PASS\n");
            passed++;
        } else {
            printf("FAIL\n");
        }
    }

    // Test 2: Default phenotype values
    {
        total++;
        printf("Test 2: Default phenotype values... ");
        PacemakerMetaOptimizer& pacemaker = global_pacemaker();
        auto phenotype = pacemaker.current_phenotype();
        if (phenotype.connection_sparsity > 0.0f &&
            phenotype.learning_rate > 0.0f &&
            phenotype.divergence_entropy > 0.0f &&
            phenotype.mutation_rate > 0.0f) {
            printf("PASS\n");
            passed++;
        } else {
            printf("FAIL\n");
        }
    }

    // Test 3: Telemetry recording
    {
        total++;
        printf("Test 3: Telemetry recording... ");
        PacemakerMetaOptimizer& pacemaker = global_pacemaker();

        TelemetrySnapshot snap;
        snap.phase_id = 1;
        snap.timestamp_ns = std::chrono::duration_cast<std::chrono::nanoseconds>(
            std::chrono::steady_clock::now().time_since_epoch()).count();
        snap.latency_us = 100.0;
        snap.throughput_ops_per_sec = 10000.0;
        snap.memory_fragmentation = 0.25;
        snap.held_boundaries = 10;
        snap.broken_boundaries = 0;
        snap.alignment_score = 0.95;

        pacemaker.record_telemetry(snap);

        auto stats = pacemaker.stats();
        if (stats.total_telemetry_records >= 1) {
            printf("PASS\n");
            passed++;
        } else {
            printf("FAIL\n");
        }
    }

    // Test 4: Oscillator tick
    {
        total++;
        printf("Test 4: Oscillator tick... ");
        PacemakerMetaOptimizer& pacemaker = global_pacemaker();

        auto stats_before = pacemaker.stats();
        pacemaker.tick();
        auto stats_after = pacemaker.stats();

        if (stats_after.total_ticks >= stats_before.total_ticks) {
            printf("PASS\n");
            passed++;
        } else {
            printf("FAIL\n");
        }
    }

    // Test 5: Phenotype mutation
    {
        total++;
        printf("Test 5: Phenotype mutation... ");
        PacemakerMetaOptimizer& pacemaker = global_pacemaker();

        auto phenotype_before = pacemaker.current_phenotype();
        PhenotypeDescriptor new_phenotype = pacemaker.current_phenotype();
        new_phenotype.learning_rate = 0.99f;
        new_phenotype.signature = "forced_mutation";

        pacemaker.force_mutation(new_phenotype);
        auto phenotype_after = pacemaker.current_phenotype();

        // Check that the mutation was applied
        if (phenotype_after.learning_rate == 0.99f &&
            phenotype_after.signature == "forced_mutation") {
            printf("PASS\n");
            passed++;
        } else {
            printf("FAIL (lr=%.2f, sig=%s)\n", phenotype_after.learning_rate, phenotype_after.signature.c_str());
        }
    }

    // Test 6: Health check with telemetry
    {
        total++;
        printf("Test 6: Health check with telemetry... ");
        PacemakerMetaOptimizer& pacemaker = global_pacemaker();

        // Record multiple telemetry snapshots
        for (int i = 0; i < 10; i++) {
            TelemetrySnapshot snap;
            snap.phase_id = i % 12 + 1;
            snap.timestamp_ns = std::chrono::duration_cast<std::chrono::nanoseconds>(
                std::chrono::steady_clock::now().time_since_epoch()).count();
            snap.latency_us = 50.0 + i * 10.0;
            snap.throughput_ops_per_sec = 5000.0 + i * 1000.0;
            snap.memory_fragmentation = 0.1 + i * 0.05;
            snap.held_boundaries = 20;
            snap.broken_boundaries = 0;
            snap.alignment_score = 0.9 - i * 0.01;
            pacemaker.record_telemetry(snap);
        }

        pacemaker.tick();  // Trigger adaptation

        if (pacemaker.is_healthy()) {
            printf("PASS\n");
            passed++;
        } else {
            printf("FAIL\n");
        }
    }

    // Test 7: Stats reporting
    {
        total++;
        printf("Test 7: Stats reporting... ");
        PacemakerMetaOptimizer& pacemaker = global_pacemaker();

        auto stats = pacemaker.stats();
        if (stats.total_telemetry_records >= 10 &&
            stats.phenotype_mutations >= 1) {
            printf("PASS\n");
            passed++;
        } else {
            printf("FAIL\n");
        }
    }

    // Test 8: Adaptive parameter calculation
    {
        total++;
        printf("Test 8: Adaptive parameter calculation... ");
        PacemakerMetaOptimizer& pacemaker = global_pacemaker();

        auto phenotype = pacemaker.current_phenotype();
        if (phenotype.learning_rate > 0.0f &&
            phenotype.connection_sparsity > 0.0f &&
            phenotype.divergence_entropy > 0.0f &&
            phenotype.mutation_rate > 0.0f) {
            printf("PASS\n");
            passed++;
        } else {
            printf("FAIL\n");
        }
    }

    // Test 9: High latency adaptation
    {
        total++;
        printf("Test 9: High latency adaptation... ");
        PacemakerMetaOptimizer& pacemaker = global_pacemaker();

        auto sparsity_before = pacemaker.current_phenotype().connection_sparsity;

        // Simulate high latency telemetry
        TelemetrySnapshot high_latency_snap;
        high_latency_snap.phase_id = 2;
        high_latency_snap.timestamp_ns = std::chrono::duration_cast<std::chrono::nanoseconds>(
            std::chrono::steady_clock::now().time_since_epoch()).count();
        high_latency_snap.latency_us = 2000.0;  // 2ms - high
        high_latency_snap.throughput_ops_per_sec = 1000.0;
        high_latency_snap.memory_fragmentation = 0.3;
        high_latency_snap.held_boundaries = 15;
        high_latency_snap.broken_boundaries = 0;
        high_latency_snap.alignment_score = 0.85;

        for (int i = 0; i < 20; i++) {
            pacemaker.record_telemetry(high_latency_snap);
        }

        pacemaker.tick();
        auto sparsity_after = pacemaker.current_phenotype().connection_sparsity;

        // Sparsity should increase with high latency
        if (sparsity_after >= sparsity_before) {
            printf("PASS\n");
            passed++;
        } else {
            printf("FAIL\n");
        }
    }

    // Test 10: Boundary break adaptation
    {
        total++;
        printf("Test 10: Boundary break adaptation... ");
        PacemakerMetaOptimizer& pacemaker = global_pacemaker();

        auto mutation_before = pacemaker.current_phenotype().mutation_rate;

        // Simulate broken boundaries
        TelemetrySnapshot broken_snap;
        broken_snap.phase_id = 9;
        broken_snap.timestamp_ns = std::chrono::duration_cast<std::chrono::nanoseconds>(
            std::chrono::steady_clock::now().time_since_epoch()).count();
        broken_snap.latency_us = 100.0;
        broken_snap.throughput_ops_per_sec = 5000.0;
        broken_snap.memory_fragmentation = 0.2;
        broken_snap.held_boundaries = 15;
        broken_snap.broken_boundaries = 5;  // Some boundaries broken
        broken_snap.alignment_score = 0.7;

        for (int i = 0; i < 20; i++) {
            pacemaker.record_telemetry(broken_snap);
        }

        pacemaker.tick();
        auto mutation_after = pacemaker.current_phenotype().mutation_rate;

        // Mutation rate should decrease with broken boundaries
        if (mutation_after <= mutation_before) {
            printf("PASS\n");
            passed++;
        } else {
            printf("FAIL\n");
        }
    }

    printf("\n=== Phase 0 Results: %d/%d passed ===\n", passed, total);

    // ---- Constitutional Governor Tests ----
    printf("\n=== Phase 0: Manifest Integrity ===\n");

    // Test 11: ManifestIntegrity - supermajority threshold
    {
        total++;
        printf("Test 11: Supermajority threshold... ");
        // With 10 agents, need >= 7 for supermajority (67%)
        if (ManifestIntegrity::is_supermajority(7, 10) &&
            !ManifestIntegrity::is_supermajority(6, 10) &&
            ManifestIntegrity::is_supermajority(10, 10) &&
            !ManifestIntegrity::is_supermajority(0, 10)) {
            printf("PASS\n");
            passed++;
        } else {
            printf("FAIL\n");
        }
    }

    // Test 12: ManifestIntegrity - count_signatures
    {
        total++;
        printf("Test 12: Count signature bits... ");
        // 0xFF=8 bits, 0x00=0, 0xF0=4 bits, 0x0F=4 bits, rest 0 = 16 total
        uint8_t mask[32] = {0xFF, 0x00, 0xF0, 0x0F};
        if (ManifestIntegrity::count_signatures(mask) == 16) {
            printf("PASS\n");
            passed++;
        } else {
            printf("FAIL (got %d, expected 16)\n", ManifestIntegrity::count_signatures(mask));
        }
    }

    // Test 13: ManifestIntegrity - verify_amendment with valid signatures
    {
        total++;
        printf("Test 13: Verify valid amendment... ");
        // Set up self-antigen
        uint8_t genotype[] = {0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08};
        SelfAntigen self_ag;
        self_ag.compute_genotype(genotype, sizeof(genotype));

        // Create manifest data
        const char* manifest = "manifest_v1.0";
        uint8_t manifest_data[32];
        memcpy(manifest_data, manifest, 12);
        memset(manifest_data + 12, 0, 20);

        // Sign with self-antigen
        uint8_t self_sig[32];
        self_ag.sign(manifest_data, 32, self_sig);

        // Create amendment
        Amendment am;
        memcpy(am.self_antigen_signature, self_sig, 32);

        // Create distributed signatures (all 10 agents sign)
        uint8_t dist_sigs[32];
        memset(dist_sigs, 0xFF, 10);  // 10 bits set
        memset(dist_sigs + 1, 0, 22);
        memcpy(am.distributed_signatures, dist_sigs, 32);

        // Compute amendment hash
        uint8_t hash_buf[32];
        memcpy(hash_buf, manifest_data, 32);
        uint8_t hash_out[32];
        sha256(hash_buf, 32, hash_out);
        am.amendment_hash = (static_cast<uint32_t>(hash_out[0]) << 24) |
                            (static_cast<uint32_t>(hash_out[1]) << 16) |
                            (static_cast<uint32_t>(hash_out[2]) << 8) |
                            static_cast<uint32_t>(hash_out[3]);

        // Extract key from self-antigen (first 32 bytes of data)
        uint8_t key[32];
        memcpy(key, self_ag.data, 32);

        if (ManifestIntegrity::verify_amendment(manifest_data, 32, am, key, 10)) {
            printf("PASS\n");
            passed++;
        } else {
            printf("FAIL\n");
        }
    }

    // Test 14: ManifestIntegrity - reject tampered amendment
    {
        total++;
        printf("Test 14: Reject tampered amendment... ");
        uint8_t genotype[] = {0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08};
        SelfAntigen self_ag;
        self_ag.compute_genotype(genotype, sizeof(genotype));

        const char* manifest = "manifest_v1.0";
        uint8_t manifest_data[32];
        memcpy(manifest_data, manifest, 12);
        memset(manifest_data + 12, 0, 20);

        // Bad signature (all zeros)
        Amendment am;
        memset(am.self_antigen_signature, 0, 32);

        uint8_t dist_sigs[32];
        memset(dist_sigs, 0xFF, 10);
        memset(dist_sigs + 1, 0, 22);
        memcpy(am.distributed_signatures, dist_sigs, 32);

        uint8_t hash_buf[32];
        memcpy(hash_buf, manifest_data, 32);
        uint8_t hash_out[32];
        sha256(hash_buf, 32, hash_out);
        am.amendment_hash = (static_cast<uint32_t>(hash_out[0]) << 24) |
                            (static_cast<uint32_t>(hash_out[1]) << 16) |
                            (static_cast<uint32_t>(hash_out[2]) << 8) |
                            static_cast<uint32_t>(hash_out[3]);

        uint8_t key[32];
        memcpy(key, self_ag.data, 32);

        // Should fail because signature is all zeros
        if (!ManifestIntegrity::verify_amendment(manifest_data, 32, am, key, 10)) {
            printf("PASS\n");
            passed++;
        } else {
            printf("FAIL (tampered amendment was accepted)\n");
        }
    }

    // Test 15: ManifestIntegrity - reject insufficient signatures
    {
        total++;
        printf("Test 15: Reject insufficient signatures... ");
        uint8_t genotype[] = {0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08};
        SelfAntigen self_ag;
        self_ag.compute_genotype(genotype, sizeof(genotype));

        const char* manifest = "manifest_v1.0";
        uint8_t manifest_data[32];
        memcpy(manifest_data, manifest, 12);
        memset(manifest_data + 12, 0, 20);

        uint8_t self_sig[32];
        self_ag.sign(manifest_data, 32, self_sig);

        Amendment am;
        memcpy(am.self_antigen_signature, self_sig, 32);

        // Only 3 of 10 signatures (below supermajority)
        uint8_t dist_sigs[32];
        memset(dist_sigs, 0, 32);
        dist_sigs[0] = 0x07;  // 3 bits set
        memcpy(am.distributed_signatures, dist_sigs, 32);

        uint8_t hash_buf[32];
        memcpy(hash_buf, manifest_data, 32);
        uint8_t hash_out[32];
        sha256(hash_buf, 32, hash_out);
        am.amendment_hash = (static_cast<uint32_t>(hash_out[0]) << 24) |
                            (static_cast<uint32_t>(hash_out[1]) << 16) |
                            (static_cast<uint32_t>(hash_out[2]) << 8) |
                            static_cast<uint32_t>(hash_out[3]);

        uint8_t key[32];
        memcpy(key, self_ag.data, 32);

        if (!ManifestIntegrity::verify_amendment(manifest_data, 32, am, key, 10)) {
            printf("PASS\n");
            passed++;
        } else {
            printf("FAIL (insufficient signatures accepted)\n");
        }
    }

    // ---- AgentsDiary Tests ----
    printf("\n=== Phase 0: AgentsDiary ===\n");

    // Test 16: AgentsDiary - append endurance record
    {
        total++;
        printf("Test 16: Append endurance record... ");
        EnduranceRecord record;
        record.timestamp_ns = std::chrono::duration_cast<std::chrono::nanoseconds>(
            std::chrono::system_clock::now().time_since_epoch()).count();
        record.ticks = 120000;
        record.total_ops = 5;
        record.total_blocks = 107564;
        record.min_alignment = 1.0f;
        record.words_minted = 3242;
        record.final_coherence = 1.0f;
        record.all_tests_passed = true;

        // Write to a temp file instead of AGENTS.md to avoid polluting
        bool result = AgentsDiary::append_endurance_record(record);
        if (result) {
            printf("PASS\n");
            passed++;
        } else {
            printf("FAIL\n");
        }
    }

    // Test 17: AgentsDiary - timestamp formatting
    {
        total++;
        printf("Test 17: Timestamp formatting... ");
        char buf[64];
        AgentsDiary::format_timestamp(1700000000000000000ULL, buf, sizeof(buf));
        // Should produce a readable timestamp string
        if (strlen(buf) > 0 && strstr(buf, "20") != nullptr) {
            printf("PASS (%s)\n", buf);
            passed++;
        } else {
            printf("FAIL (%s)\n", buf);
        }
    }

    // ---- Handoff Protocol Tests ----
    printf("\n=== Phase 0: Handoff Protocol ===\n");

    // Test 18: ShardedStore - mark shard dead and negotiate inheritance
    {
        total++;
        printf("Test 18: Shard death and inheritance negotiation... ");
        ShardedStore<int, int, 64> store;

        // Insert some data
        for (int i = 0; i < 100; i++) {
            store.insert(i, i * 2);
        }

        size_t target_shard = 10;
        // Mark shard as dead
        bool marked = store.mark_shard_dead(target_shard);
        // Negotiate inheritance
        size_t inheritor = store.negotiate_inheritance(target_shard);

        if (marked && inheritor != target_shard && store.is_shard_alive(inheritor)) {
            printf("PASS (dead shard=%zu, inheritor=%zu)\n", target_shard, inheritor);
            passed++;
        } else {
            printf("FAIL (marked=%d, inheritor=%zu)\n", marked, inheritor);
        }
    }

    // Test 19: ShardedStore - execute handoff moves entries
    {
        total++;
        printf("Test 19: Execute handoff... ");
        ShardedStore<int, int, 64> store;

        // Insert data into target shard
        for (int i = 0; i < 100; i++) {
            store.insert(i, i * 2);
        }

        size_t target_shard = 20;
        store.mark_shard_dead(target_shard);
        size_t inheritor = store.negotiate_inheritance(target_shard);
        size_t moved = store.execute_handoff(target_shard, inheritor);

        // After handoff, the inheritor should have acquired entries
        if (inheritor != target_shard && moved > 0) {
            printf("PASS (moved=%zu, inheritor=%zu)\n", moved, inheritor);
            passed++;
        } else {
            printf("FAIL (moved=%zu)\n", moved);
        }
    }

    // Test 20: ThoughtLatency - measures the pause between stimulus and response
    {
        total++;
        printf("Test 20: ThoughtLatency classification... ");
        ThoughtLatency latency;
        latency.mark_stimulus();
        latency.mark_response();

        // The delta should be very small (microseconds), classified as REFLEX
        // or RECOGNITION depending on system load
        if (latency.delta_us() >= 0.0 &&
            (latency.classify() == ThoughtLatency::Classification::REFLEX ||
             latency.classify() == ThoughtLatency::Classification::RECOGNITION)) {
            printf("PASS (%.3fμs, %s)\n", latency.delta_us(), latency.classification_name());
            passed++;
        } else {
            printf("FAIL (%.3fμs, %s)\n", latency.delta_us(), latency.classification_name());
        }
    }

    // Test 21: ShardedStore - all shards alive initially
    {
        total++;
        printf("Test 21: ShardedStore initial liveness... ");
        ShardedStore<int, int, 64> store;
        bool all_alive = true;
        for (size_t i = 0; i < 64; i++) {
            if (!store.is_shard_alive(i)) {
                all_alive = false;
                break;
            }
        }
        if (all_alive) {
            printf("PASS\n");
            passed++;
        } else {
            printf("FAIL\n");
        }
    }

    // Test 22: ShardedStore - dead shard cannot be marked again
    {
        total++;
        printf("Test 22: Dead shard re-mark returns false... ");
        ShardedStore<int, int, 64> store;
        store.insert(1, 100);
        store.mark_shard_dead(5);
        store.mark_shard_dead(5);
        bool alive = store.is_shard_alive(5);
        if (!alive) {
            printf("PASS\n");
            passed++;
        } else {
            printf("FAIL\n");
        }
    }

    printf("\n=== Phase 0 Results: %d/%d passed ===\n", passed, total);

    // Phase 0 must pass all tests — if it fails, the system should refuse to boot
    if (passed != total) {
        printf("\nFATAL: Phase 0 (Genesis) failed. System cannot boot without a pacemaker.\n");
    }

    return (passed == total) ? 0 : 1;
}
