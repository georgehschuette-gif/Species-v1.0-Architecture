// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#pragma once
#include "divergence_detector/divergence_detector.h"
#include "lexicon_consolidation/lexicon_consolidation.h"
#include <atomic>
#include <cstdint>
#include <future>
#include <mutex>
#include <utility>
#include <vector>

namespace omega {

class SemanticErrorDrive {
 public:
  float process(uint32_t id, const float* state, float tolerance = 0.20f,
                float split_threshold = 0.10f) {
    float d = det_.record(id, state, tolerance);
    if (consol_.resolve(d, split_threshold) == LexiconConsolidation::SPLIT)
      splits_++;
    return d;
  }

  int splits() const { return splits_; }

  std::vector<float> process_batch(
      const std::vector<std::pair<uint32_t, const float*>>& symbols,
      float tolerance = 0.20f, float split_threshold = 0.10f) {
    std::vector<std::future<float>> futures;
    futures.reserve(symbols.size());

    for (const auto& sym : symbols) {
      futures.push_back(std::async(std::launch::async,
        [this, sym, tolerance, split_threshold]() -> float {
          std::lock_guard<std::mutex> lock(mtx_);
          float d = det_.record(sym.first, sym.second, tolerance);
          if (consol_.resolve(d, split_threshold) ==
              LexiconConsolidation::SPLIT)
            splits_++;
          return d;
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

}  // namespace omega
