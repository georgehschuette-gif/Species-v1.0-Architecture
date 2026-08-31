#pragma once
#include <cstdint>

namespace omega {

// Novelty harvesting: accumulate a novelty signal and, subject to the caloric
// budget, emit one new pattern per NOVELTY_EVERY_TICKS. This is the
// "1 new pattern / 1000 ticks" directive, gated by available compute.
class NoveltyHarvesting {
 public:
  void init(uint32_t every_ticks, uint32_t cal_budget, float novelty_threshold) {
    every_ = every_ticks;
    budget_ = cal_budget;
    thr_ = novelty_threshold;
    count_ = 0;
    since_ = 0;
    acc_ = 0.0f;
  }

  // Call each tick. Returns true on the tick a pattern is harvested.
  bool tick(uint32_t now_tick, float novelty, uint32_t calories_available) {
    (void)now_tick;
    since_++;
    acc_ += novelty;
    bool due = since_ >= every_;
    bool rich = acc_ >= thr_;
    bool funded = calories_available >= budget_;
    if (due && rich && funded) {
      count_++;
      since_ = 0;
      acc_ = 0.0f;
      return true;
    }
    return false;
  }

  uint32_t harvested() const { return count_; }

 private:
  uint32_t every_ = 1000;
  uint32_t budget_ = 100;
  float thr_ = 8.0f;
  uint32_t count_ = 0;
  uint32_t since_ = 0;
  float acc_ = 0.0f;
};

}  // namespace omega
