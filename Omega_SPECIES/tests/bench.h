#pragma once
#include <cstdio>
#include <chrono>

// Lightweight per-phase benchmark: prints elapsed wall-clock time on scope exit.
// Usage: place `Bench _b("PHASE1");` at the top of main().
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
