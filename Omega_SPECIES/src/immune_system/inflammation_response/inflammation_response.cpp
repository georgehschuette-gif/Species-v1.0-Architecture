#include "inflammation_response.h"

namespace omega {

bool Inflammation::trigger(float severity, uint32_t tag, int budget) {
  if (n_ >= CAP) return false;
  q_[n_].severity = severity;
  q_[n_].tag = tag;
  q_[n_].compute_budget = budget;
  allocated_total_ += budget;
  bubble_up(n_);
  n_++;
  return true;
}

bool Inflammation::pop(InflammationTask& out) {
  if (n_ == 0) return false;
  out = q_[0];
  q_[0] = q_[n_ - 1];
  n_--;
  if (n_ > 0) bubble_down(0);
  return true;
}

float Inflammation::total_severity() const {
  float s = 0.0f;
  for (int i = 0; i < n_; i++) s += q_[i].severity;
  return s;
}

int Inflammation::total_allocated() const { return allocated_total_; }

void Inflammation::bubble_up(int i) {
  while (i > 0) {
    int p = (i - 1) / 2;
    if (q_[p].severity >= q_[i].severity) break;
    InflammationTask t = q_[p];
    q_[p] = q_[i];
    q_[i] = t;
    i = p;
  }
}

void Inflammation::bubble_down(int i) {
  while (true) {
    int l = 2 * i + 1, r = 2 * i + 2, m = i;
    if (l < n_ && q_[l].severity > q_[m].severity) m = l;
    if (r < n_ && q_[r].severity > q_[m].severity) m = r;
    if (m == i) break;
    InflammationTask t = q_[m];
    q_[m] = q_[i];
    q_[i] = t;
    i = m;
  }
}

}  // namespace omega
