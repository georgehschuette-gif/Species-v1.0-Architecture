// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

#include "phase0_genesis.h"
#include <algorithm>
#include <cmath>
#include <cstring>

namespace omega::genesis {

PacemakerMetaOptimizer::PacemakerMetaOptimizer()
    : last_tick_time_(std::chrono::steady_clock::now()),
      telemetry_buffer_(TELEMETRY_BUFFER_SIZE) {

    // Initialize default phenotype
    current_phenotype_.connection_sparsity = DEFAULT_CONNECTION_SPARSITY;
    current_phenotype_.learning_rate = DEFAULT_LEARNING_RATE;
    current_phenotype_.divergence_entropy = DEFAULT_DIVERGENCE_ENTROPY;
    current_phenotype_.mutation_rate = DEFAULT_MUTATION_RATE;
    current_phenotype_.generation = 0;
    current_phenotype_.signature = "genesis_v1.0";
}

void PacemakerMetaOptimizer::tick() {
    auto now = std::chrono::steady_clock::now();
    auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(
        now - last_tick_time_).count();

    // Only adapt on oscillator period
    if (static_cast<uint64_t>(elapsed) >= OSCILLATOR_PERIOD_MS) {
        tick_count_.fetch_add(1, std::memory_order_release);
        last_tick_time_ = now;

        adapt_parameters();
        check_health();
    }
}

void PacemakerMetaOptimizer::record_telemetry(const TelemetrySnapshot& snapshot) {
    auto now = std::chrono::steady_clock::now();
    auto now_ns = std::chrono::duration_cast<std::chrono::nanoseconds>(
        now.time_since_epoch()).count();

    // Calculate telemetry latency in microseconds
    uint64_t latency_ns = now_ns - snapshot.timestamp_ns;
    uint64_t latency_us = latency_ns / 1000;
    telemetry_latency_sum_us_.fetch_add(latency_us, std::memory_order_relaxed);
    total_telemetry_records_.fetch_add(1, std::memory_order_relaxed);

    // Store in ring buffer
    size_t idx = telemetry_index_.fetch_add(1, std::memory_order_acq_rel) % TELEMETRY_BUFFER_SIZE;
    telemetry_buffer_[idx] = snapshot;
    telemetry_count_.fetch_add(1, std::memory_order_relaxed);

    last_telemetry_time_.store(now_ns, std::memory_order_release);
}

PhenotypeDescriptor PacemakerMetaOptimizer::current_phenotype() const {
    return current_phenotype_;
}

bool PacemakerMetaOptimizer::is_healthy() const {
    return healthy_.load(std::memory_order_acquire);
}

void PacemakerMetaOptimizer::force_mutation(const PhenotypeDescriptor& new_phenotype) {
    current_phenotype_ = new_phenotype;
    phenotype_generation_.fetch_add(1, std::memory_order_release);
}

PacemakerMetaOptimizer::Stats PacemakerMetaOptimizer::stats() const {
    Stats s;
    s.total_ticks = tick_count_.load(std::memory_order_acquire);
    s.total_telemetry_records = total_telemetry_records_.load(std::memory_order_acquire);
    s.phenotype_mutations = phenotype_generation_.load(std::memory_order_acquire);

    uint64_t records = total_telemetry_records_.load(std::memory_order_acquire);
    if (records > 0) {
        s.avg_telemetry_latency_us = static_cast<double>(telemetry_latency_sum_us_.load(std::memory_order_relaxed)) / records;
    } else {
        s.avg_telemetry_latency_us = 0.0;
    }

    return s;
}

void PacemakerMetaOptimizer::adapt_parameters() {
    // Analyze recent telemetry to drive adaptation
    size_t recent_count = std::min((size_t)64, telemetry_count_.load(std::memory_order_acquire));
    if (recent_count < 4) return;  // Need minimum data

    // Calculate averages from recent telemetry
    double avg_latency = 0.0;
    double avg_throughput = 0.0;
    double avg_fragmentation = 0.0;
    double avg_alignment = 0.0;
    uint32_t total_held = 0;
    uint32_t total_broken = 0;

    size_t start_idx = (telemetry_index_.load(std::memory_order_acquire) - recent_count + TELEMETRY_BUFFER_SIZE) % TELEMETRY_BUFFER_SIZE;

    for (size_t i = 0; i < recent_count; i++) {
        size_t idx = (start_idx + i) % TELEMETRY_BUFFER_SIZE;
        const auto& snap = telemetry_buffer_[idx];
        avg_latency += snap.latency_us;
        avg_throughput += snap.throughput_ops_per_sec;
        avg_fragmentation += snap.memory_fragmentation;
        avg_alignment += snap.alignment_score;
        total_held += snap.held_boundaries;
        total_broken += snap.broken_boundaries;
    }

    avg_latency /= recent_count;
    avg_throughput /= recent_count;
    avg_fragmentation /= recent_count;
    avg_alignment /= recent_count;

    // Adapt based on observed metrics
    bool mutated = false;

    // If latency is high, increase sparsity (reduce connections)
    if (avg_latency > 1000.0) {  // > 1ms
        current_phenotype_.connection_sparsity = std::min(0.95f, current_phenotype_.connection_sparsity + 0.01f);
        mutated = true;
    } else if (avg_latency < 100.0) {  // < 100μs
        current_phenotype_.connection_sparsity = std::max(0.50f, current_phenotype_.connection_sparsity - 0.01f);
        mutated = true;
    }

    // If fragmentation is high, reduce learning rate (slower adaptation = less churn)
    if (avg_fragmentation > 0.50) {
        current_phenotype_.learning_rate = std::max(0.05f, current_phenotype_.learning_rate * 0.95f);
        mutated = true;
    } else if (avg_fragmentation < 0.20) {
        current_phenotype_.learning_rate = std::min(0.50f, current_phenotype_.learning_rate * 1.05f);
        mutated = true;
    }

    // If boundaries are breaking, reduce mutation rate (more conservative)
    if (total_broken > 0) {
        current_phenotype_.mutation_rate = std::max(0.05f, current_phenotype_.mutation_rate * 0.9f);
        mutated = true;
    } else if (total_held > recent_count * 0.8 && avg_alignment > 0.9) {
        // System is stable, can be more exploratory
        current_phenotype_.mutation_rate = std::min(0.40f, current_phenotype_.mutation_rate * 1.1f);
        mutated = true;
    }

    // If alignment is low, increase divergence entropy (allow more drift)
    if (avg_alignment < 0.5) {
        current_phenotype_.divergence_entropy = std::min(0.90f, current_phenotype_.divergence_entropy + 0.05f);
        mutated = true;
    } else if (avg_alignment > 0.95) {
        current_phenotype_.divergence_entropy = std::max(0.30f, current_phenotype_.divergence_entropy - 0.05f);
        mutated = true;
    }

    if (mutated) {
        phenotype_generation_.fetch_add(1, std::memory_order_release);
    }
}

float PacemakerMetaOptimizer::calculate_adaptive_learning_rate() const {
    return current_phenotype_.learning_rate;
}

float PacemakerMetaOptimizer::calculate_adaptive_sparsity() const {
    return current_phenotype_.connection_sparsity;
}

float PacemakerMetaOptimizer::calculate_adaptive_divergence() const {
    return current_phenotype_.divergence_entropy;
}

float PacemakerMetaOptimizer::calculate_adaptive_mutation_rate() const {
    return current_phenotype_.mutation_rate;
}

void PacemakerMetaOptimizer::check_health() {
    auto now = std::chrono::steady_clock::now();
    auto now_ns = std::chrono::duration_cast<std::chrono::nanoseconds>(
        now.time_since_epoch()).count();

    uint64_t last_telemetry = last_telemetry_time_.load(std::memory_order_acquire);

    // If no telemetry for > 30 seconds, mark unhealthy
    if (now_ns - last_telemetry > 30000000000ULL) {
        healthy_.store(false, std::memory_order_release);
    } else {
        healthy_.store(true, std::memory_order_release);
    }
}

// Global singleton
static PacemakerMetaOptimizer g_pacemaker;

PacemakerMetaOptimizer& global_pacemaker() {
    return g_pacemaker;
}

}  // namespace omega::genesis
