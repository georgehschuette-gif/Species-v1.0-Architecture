#pragma once
#include <cstdio>
#include <chrono>
#include <cstdint>
#include <vector>
#include <algorithm>
#include <cmath>
#include <atomic>
#include <cstring>

struct Bench {
  const char* name_;
  std::chrono::high_resolution_clock::time_point t0_;
  explicit Bench(const char* name) : name_(name), t0_(std::chrono::high_resolution_clock::now()) {}
  ~Bench() {
    double ms = std::chrono::duration<double, std::milli>(
                   std::chrono::high_resolution_clock::now() - t0_).count();
    printf("[timing] %s: %.2f ms\n", name_, ms);
  }
};

struct ThroughputBench {
  const char* name_;
  int64_t ops_;
  std::chrono::high_resolution_clock::time_point t0_;
  explicit ThroughputBench(const char* name, int64_t ops)
      : name_(name), ops_(ops), t0_(std::chrono::high_resolution_clock::now()) {}
  ~ThroughputBench() {
    double ms = std::chrono::duration<double, std::milli>(
                   std::chrono::high_resolution_clock::now() - t0_).count();
    double tps = (ops_ / ms) * 1000.0;
    printf("[bench] %s: %lld ops in %.2f ms (%.0f ops/s)\n", name_, (long long)ops_, ms, tps);
  }
};

struct LatencyTracker {
  std::vector<double> samples_;
  std::chrono::high_resolution_clock::time_point start_;

  void start() { start_ = std::chrono::high_resolution_clock::now(); }
  void stop() {
    double ms = std::chrono::duration<double, std::milli>(
                   std::chrono::high_resolution_clock::now() - start_).count();
    samples_.push_back(ms);
  }
  double p50() const {
    if (samples_.empty()) return 0.0;
    auto s = samples_; std::sort(s.begin(), s.end());
    return s[s.size() / 2];
  }
  double p95() const {
    if (samples_.empty()) return 0.0;
    auto s = samples_; std::sort(s.begin(), s.end());
    return s[(size_t)(s.size() * 0.95)];
  }
  double p99() const {
    if (samples_.empty()) return 0.0;
    auto s = samples_; std::sort(s.begin(), s.end());
    return s[(size_t)(s.size() * 0.99)];
  }
  double p999() const {
    if (samples_.empty()) return 0.0;
    auto s = samples_; std::sort(s.begin(), s.end());
    return s[(size_t)(s.size() * 0.999)];
  }
  double avg() const {
    if (samples_.empty()) return 0.0;
    double sum = 0; for (double v : samples_) sum += v;
    return sum / samples_.size();
  }
};

/// Tracks memory allocation/deallocation patterns and fragmentation.
/// Use record_alloc(n) / record_dealloc(n) to feed observed allocations.
/// fragmentation_ratio() returns (wasted / total) where wasted = allocated - in-use.
struct MemoryMetrics {
  std::atomic<size_t> total_allocated{0};
  std::atomic<size_t> total_freed{0};
  std::atomic<size_t> alloc_count{0};
  std::atomic<size_t> dealloc_count{0};
  std::atomic<size_t> peak_usage{0};

  void record_alloc(size_t bytes) {
    size_t new_total = total_allocated.fetch_add(bytes) + bytes;
    alloc_count.fetch_add(1);
    size_t usage = new_total - total_freed.load();
    size_t prev_peak = peak_usage.load();
    while (usage > prev_peak && !peak_usage.compare_exchange_weak(prev_peak, usage)) {}
  }

  void record_dealloc(size_t bytes) {
    total_freed.fetch_add(bytes);
    dealloc_count.fetch_add(1);
  }

  size_t current_usage() const {
    return total_allocated.load() - total_freed.load();
  }

  double fragmentation_ratio() const {
    size_t alloc = total_allocated.load();
    if (alloc == 0) return 0.0;
    return (double)(alloc - current_usage()) / (double)alloc;
  }

  void reset() {
    total_allocated.store(0);
    total_freed.store(0);
    alloc_count.store(0);
    dealloc_count.store(0);
    peak_usage.store(0);
  }

  void report(const char* label) const {
    printf("[mem] %s: alloc=%zuB (%zu ops) freed=%zuB (%zu ops) "
           "in-use=%zuB peak=%zuB fragmentation=%.1f%%\n",
           label,
           total_allocated.load(), alloc_count.load(),
           total_freed.load(), dealloc_count.load(),
           current_usage(), peak_usage.load(),
           fragmentation_ratio() * 100.0);
  }
};

/// RAII guard that snapshots MemoryMetrics at construction and reports on destruction.
struct MemoryMetricsGuard {
  MemoryMetrics& metrics;
  const char* label;
  std::chrono::high_resolution_clock::time_point t0;
  size_t start_allocated;
  size_t start_freed;

  explicit MemoryMetricsGuard(MemoryMetrics& m, const char* lbl)
      : metrics(m), label(lbl), t0(std::chrono::high_resolution_clock::now()),
        start_allocated(m.total_allocated.load()), start_freed(m.total_freed.load()) {}
  ~MemoryMetricsGuard() {
    double ms = std::chrono::duration<double, std::milli>(
                   std::chrono::high_resolution_clock::now() - t0).count();
    size_t delta_alloc = metrics.total_allocated.load() - start_allocated;
    size_t delta_freed = metrics.total_freed.load() - start_freed;
    printf("[mem] %s: %.1fms alloc_delta=%zuB freed_delta=%zuB fragmentation=%.1f%%\n",
           label, ms, delta_alloc, delta_freed,
           (delta_alloc > 0 ? (double)(delta_alloc - metrics.current_usage()) / (double)delta_alloc * 100.0 : 0.0));
  }
};

/// Platform-level heap statistics. On POSIX uses malloc_trim/mallinfo if available.
struct HeapStats {
  size_t arena_size;     // total heap space obtained from OS
  size_t in_use;         // total bytes in use by application
  size_t available;      // total bytes available in free blocks (fragmentation)
  double fragmentation_ratio;

  static HeapStats current() {
    HeapStats stats;
    stats.arena_size = 0;
    stats.in_use = 0;
    stats.available = 0;
    stats.fragmentation_ratio = 0.0;
#if defined(__linux__)
    // Use /proc/self/status for VmRSS (resident set size)
    FILE* f = fopen("/proc/self/status", "r");
    if (f) {
      char line[256];
      while (fgets(line, sizeof(line), f)) {
        if (strncmp(line, "VmRSS:", 6) == 0) {
          sscanf(line + 6, "%zu", &stats.in_use);
          stats.in_use *= 1024;  // kB to bytes
        }
      }
      fclose(f);
    }
#elif defined(__APPLE__)
    // On macOS, malloc_size can give us per-allocation usable size;
    // full heap stats require malloc_zone_statistics (complex), so we approximate.
    stats.in_use = 0;
    stats.available = 0;
    stats.arena_size = 0;
    stats.fragmentation_ratio = 0.0;
#else
    stats.in_use = 0;
    stats.available = 0;
    stats.arena_size = 0;
    stats.fragmentation_ratio = 0.0;
#endif
    if (stats.arena_size > 0) {
      stats.available = stats.arena_size - stats.in_use;
      stats.fragmentation_ratio = (double)stats.available / (double)stats.arena_size;
    }
    return stats;
  }

  void report(const char* label) const {
    printf("[heap] %s: arena=%zuB in-use=%zuB available=%zuB fragmentation=%.1f%%\n",
           label, arena_size, in_use, available, fragmentation_ratio * 100.0);
  }
};
