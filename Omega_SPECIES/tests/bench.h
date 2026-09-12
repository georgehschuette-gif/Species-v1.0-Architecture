#pragma once
#include <cstdio>
#include <chrono>
#include <cstdint>
#include <vector>
#include <algorithm>
#include <cmath>

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
  double p99() const {
    if (samples_.empty()) return 0.0;
    auto s = samples_; std::sort(s.begin(), s.end());
    return s[(size_t)(s.size() * 0.99)];
  }
  double avg() const {
    if (samples_.empty()) return 0.0;
    double sum = 0; for (double v : samples_) sum += v;
    return sum / samples_.size();
  }
};
