#pragma once
#include <cstdint>

namespace omega {

// Inflammation response: an anomaly is triaged into a priority queue. Severity
// sets the priority; the scheduler pops the most urgent and allocates compute
// (budget) to it.
struct InflammationTask {
  float severity;        // priority (higher = more urgent)
  uint32_t tag;          // what triggered it (e.g. antigen/phenotype tag)
  int compute_budget;    // info-calories allocated to resolve it
};

class Inflammation {
 public:
  static constexpr int CAP = 16;

  // Queue an anomaly. Returns true if accepted.
  bool trigger(float severity, uint32_t tag, int budget);

  // Pop the highest-severity task (max-heap). Returns false if empty.
  bool pop(InflammationTask& out);

  int pending() const { return n_; }
  float total_severity() const;
  int total_allocated() const;  // cumulative compute budget queued

 private:
  InflammationTask q_[CAP];
  int n_ = 0;
  int allocated_total_ = 0;
  void bubble_up(int i);
  void bubble_down(int i);
};

}  // namespace omega
