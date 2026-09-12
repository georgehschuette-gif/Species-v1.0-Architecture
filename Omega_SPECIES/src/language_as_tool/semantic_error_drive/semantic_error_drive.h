// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#pragma once
#include "divergence_detector/divergence_detector.h"
#include "lexicon_consolidation/lexicon_consolidation.h"
#include <atomic>
#include <cstdint>
#include <future>
#include <memory>
#include <mutex>
#include <utility>
#include <vector>

namespace omega {

class SemanticErrorDrive {
 public:
  float process(uint32_t id, const float* state, float tolerance = 0.20f,
                float split_threshold = 0.10f) {
    std::lock_guard<std::mutex> lock(mtx_);
    float d = det_.record(id, state, tolerance);
    if (consol_.resolve(d, split_threshold) == LexiconConsolidation::SPLIT)
      splits_++;
    return d;
  }

  int splits() const { return splits_.load(std::memory_order_acquire); }

  std::vector<float> process_batch(
      const std::vector<std::pair<uint32_t, const float*>>& symbols,
      float tolerance = 0.20f, float split_threshold = 0.10f) {
    std::vector<std::future<float>> futures;
    futures.reserve(symbols.size());

    for (const auto& sym : symbols) {
      futures.push_back(std::async(std::launch::async,
        [this, sym, tolerance, split_threshold]() -> float {
          return this->process(sym.first, sym.second, tolerance, split_threshold);
        }));
    }

    std::vector<float> results;
    results.reserve(futures.size());
    for (auto& f : futures)
      results.push_back(f.get());
    return results;
  }

 private:
  DivergenceDetector det_;
  LexiconConsolidation consol_;
  std::atomic<int> splits_{0};
  mutable std::mutex mtx_;
};

class ShardedSemanticErrorDrive {
 public:
  explicit ShardedSemanticErrorDrive(size_t num_shards = 8)
      : num_shards_(num_shards > 0 ? num_shards : 1) {
    drives_.reserve(num_shards_);
    for (size_t i = 0; i < num_shards_; i++)
      drives_.emplace_back(std::make_unique<SemanticErrorDrive>());
  }

  float process(uint32_t id, const float* state, float tolerance = 0.20f,
                float split_threshold = 0.10f) {
    return drives_[shard_index(id)]->process(id, state, tolerance, split_threshold);
  }

  int total_splits() const {
    int total = 0;
    for (const auto& d : drives_)
      total += d->splits();
    return total;
  }

  std::vector<float> process_batch(
      const std::vector<std::pair<uint32_t, const float*>>& symbols,
      float tolerance = 0.20f, float split_threshold = 0.10f) {
    size_t n = symbols.size();
    std::vector<float> results(n);

    std::vector<std::vector<std::pair<size_t, std::pair<uint32_t, const float*>>>> groups(num_shards_);
    for (size_t i = 0; i < n; i++) {
      size_t s = shard_index(symbols[i].first);
      groups[s].push_back({i, symbols[i]});
    }

    std::vector<std::future<void>> futures;
    for (size_t s = 0; s < num_shards_; s++) {
      if (groups[s].empty()) continue;
      futures.push_back(std::async(std::launch::async,
        [&results, &group = groups[s], &drive = *drives_[s], tolerance, split_threshold]() {
          for (const auto& [idx, sym] : group) {
            results[idx] = drive.process(sym.first, sym.second, tolerance, split_threshold);
          }
        }));
    }

    for (auto& f : futures) f.get();
    return results;
  }

  size_t num_shards() const { return num_shards_; }

 private:
  size_t shard_index(uint32_t id) const {
    return static_cast<size_t>(id % num_shards_);
  }
  size_t num_shards_;
  std::vector<std::unique_ptr<SemanticErrorDrive>> drives_;
};

}  // namespace omega
