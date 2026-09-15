// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

#pragma once
#include <atomic>
#include <cstdint>
#include <functional>
#include <mutex>
#include <vector>
#include <algorithm>
#include <condition_variable>
#include <future>
#include <thread>

namespace omega::core {

static constexpr size_t DEFAULT_SHARDS = 64;
static constexpr float GROWTH_FACTOR = 2.0f;
static constexpr float LOAD_FACTOR_THRESHOLD = 0.75f;

// ---- Simple growable ring buffer (replaces fixed WINDOW ring) ----
template <typename T>
class GrowableRing {
private:
    std::vector<T> buf_;
    std::atomic<size_t> head_{0};
    std::atomic<size_t> tail_{0};
    std::atomic<size_t> count_{0};
    size_t capacity_;

public:
    explicit GrowableRing(size_t initial_cap = 16)
        : buf_(initial_cap), capacity_(initial_cap) {}

    void push(const T& val) {
        size_t c = count_.load(std::memory_order_acquire);
        if (c >= capacity_) {
            grow_locked();
        }
        size_t h = head_.load(std::memory_order_relaxed);
        buf_[h % capacity_] = val;
        head_.store(h + 1, std::memory_order_release);
        count_.fetch_add(1, std::memory_order_acq_rel);
    }

    size_t size() const { return count_.load(std::memory_order_acquire); }
    size_t capacity() const { return capacity_; }

    T at(size_t idx) const {
        if (idx >= count_.load(std::memory_order_acquire)) return T{};
        return buf_[(tail_.load(std::memory_order_acquire) + idx) % capacity_];
    }

    void clear() {
        head_.store(0, std::memory_order_release);
        tail_.store(0, std::memory_order_release);
        count_.store(0, std::memory_order_release);
    }

private:
    void grow_locked() {
        size_t new_cap = (size_t)(capacity_ * GROWTH_FACTOR);
        std::vector<T> new_buf(new_cap);
        size_t c = count_.load(std::memory_order_acquire);
        size_t t = tail_.load(std::memory_order_acquire);
        for (size_t i = 0; i < c; i++) {
            new_buf[i] = buf_[(t + i) % capacity_];
        }
        buf_ = std::move(new_buf);
        capacity_ = new_cap;
        tail_.store(0, std::memory_order_release);
        head_.store(c, std::memory_order_release);
    }
};

// ---- Sharded concurrent store: growable, lock-per-shard ----
template <typename K, typename V, size_t SHARDS = DEFAULT_SHARDS>
class ShardedStore {
private:
    struct Shard {
        std::vector<std::pair<K, V>> entries;
        mutable std::mutex mutex;
        size_t capacity;
        mutable size_t hits;
        std::atomic<bool> alive{true};  // For handoff protocol
        explicit Shard(size_t cap = 256) : capacity(cap), hits(0) {}
    };

    Shard shards_[SHARDS];

    size_t shard_index(const K& key) const {
        return std::hash<K>{}(key) % SHARDS;
    }

public:
    void insert(const K& key, const V& val) {
        size_t idx = shard_index(key);
        std::lock_guard<std::mutex> lock(shards_[idx].mutex);
        auto& s = shards_[idx];
        auto it = std::find_if(s.entries.begin(), s.entries.end(),
                               [&](const auto& p) { return p.first == key; });
        if (it != s.entries.end()) {
            it->second = val;
        } else {
            if (s.entries.size() >= s.capacity) {
                s.capacity = (size_t)(s.capacity * GROWTH_FACTOR);
                s.entries.reserve(s.capacity);
            }
            s.entries.emplace_back(key, val);
        }
    }

    bool get(const K& key, V& out) const {
        size_t idx = shard_index(key);
        std::lock_guard<std::mutex> lock(shards_[idx].mutex);
        auto& s = shards_[idx];
        auto it = std::find_if(s.entries.begin(), s.entries.end(),
                               [&](const auto& p) { return p.first == key; });
        if (it != s.entries.end()) {
            out = it->second;
            s.hits++;
            return true;
        }
        return false;
    }

    bool exists(const K& key) const {
        V out;
        return get(key, out);
    }

    size_t remove(const K& key) {
        size_t idx = shard_index(key);
        std::lock_guard<std::mutex> lock(shards_[idx].mutex);
        auto& s = shards_[idx];
        auto it = std::remove_if(s.entries.begin(), s.entries.end(),
                                 [&](const auto& p) { return p.first == key; });
        size_t removed = (size_t)std::distance(it, s.entries.end());
        s.entries.erase(it, s.entries.end());
        return removed;
    }

    void clear() {
        for (size_t i = 0; i < SHARDS; i++) {
            std::lock_guard<std::mutex> lock(shards_[i].mutex);
            shards_[i].entries.clear();
            shards_[i].hits = 0;
        }
    }

    size_t total_size() const {
        size_t total = 0;
        for (size_t i = 0; i < SHARDS; i++) {
            std::lock_guard<std::mutex> lock(shards_[i].mutex);
            total += shards_[i].entries.size();
        }
        return total;
    }

    // Visitor pattern: iterate all keys across all shards (for consolidation, etc.)
    template <typename Fn>
    void for_each(Fn&& fn) {
        for (size_t i = 0; i < SHARDS; i++) {
            std::lock_guard<std::mutex> lock(shards_[i].mutex);
            for (const auto& p : shards_[i].entries) {
                fn(p.first, p.second);
            }
        }
    }

    template <typename Fn>
    void for_each_const(Fn&& fn) const {
        for (size_t i = 0; i < SHARDS; i++) {
            std::lock_guard<std::mutex> lock(shards_[i].mutex);
            for (const auto& p : shards_[i].entries) {
                fn(p.first, p.second);
            }
        }
    }

    // ---- Handoff Protocol: Shard Inheritance ----
    // When a shard dies, neighboring shards negotiate which one inherits its responsibility.
    // This is not load-balancing — it's inheritance of duty. A shard that carries meaning
    // is not just a cache; it's a memory bearer. This implements localized mortality:
    // when one shard dies, others mourn but continue.
    bool mark_shard_dead(size_t shard_idx) {
        if (shard_idx >= SHARDS) return false;
        shards_[shard_idx].alive.store(false, std::memory_order_release);
        return true;
    }

    bool is_shard_alive(size_t shard_idx) const {
        if (shard_idx >= SHARDS) return false;
        return shards_[shard_idx].alive.load(std::memory_order_acquire);
    }

    // Negotiate inheritance: find the best neighboring shard to inherit dead shard's entries
    size_t negotiate_inheritance(size_t dead_shard_idx) {
        if (dead_shard_idx >= SHARDS) return SHARDS;
        if (is_shard_alive(dead_shard_idx)) return dead_shard_idx;  // Not dead

        // Find neighbors with lowest load (fewest entries)
        size_t best_neighbor = dead_shard_idx;
        size_t min_load = SIZE_MAX;

        // Check left neighbor
        size_t left = (dead_shard_idx + SHARDS - 1) % SHARDS;
        if (is_shard_alive(left)) {
            std::lock_guard<std::mutex> lock(shards_[left].mutex);
            if (shards_[left].entries.size() < min_load) {
                min_load = shards_[left].entries.size();
                best_neighbor = left;
            }
        }

        // Check right neighbor
        size_t right = (dead_shard_idx + 1) % SHARDS;
        if (is_shard_alive(right)) {
            std::lock_guard<std::mutex> lock(shards_[right].mutex);
            if (shards_[right].entries.size() < min_load) {
                min_load = shards_[right].entries.size();
                best_neighbor = right;
            }
        }

        return best_neighbor;
    }

    // Execute handoff: move all entries from dead shard to inheritor
    size_t execute_handoff(size_t dead_shard_idx, size_t inheritor_idx) {
        if (dead_shard_idx >= SHARDS || inheritor_idx >= SHARDS) return 0;
        if (dead_shard_idx == inheritor_idx) return 0;
        if (is_shard_alive(dead_shard_idx)) return 0;

        std::vector<std::pair<K, V>> entries_to_move;
        {
            std::lock_guard<std::mutex> lock_dead(shards_[dead_shard_idx].mutex);
            entries_to_move = std::move(shards_[dead_shard_idx].entries);
            shards_[dead_shard_idx].entries.clear();
        }

        {
            std::lock_guard<std::mutex> lock_inheritor(shards_[inheritor_idx].mutex);
            for (auto& p : entries_to_move) {
                shards_[inheritor_idx].entries.emplace_back(std::move(p));
            }
        }

        return entries_to_move.size();
    }
};

// ---- Parallel invariant evaluator (for Constitutional Governor) ----
// Evaluates N read-only predicates concurrently via async futures.
// This replaces the sequential gate() check in Constitution.
class ParallelGate {
private:
  size_t num_workers_;

public:
  explicit ParallelGate(size_t workers = std::thread::hardware_concurrency())
      : num_workers_(workers == 0 ? 4 : workers) {}

  // Evaluate all predicates in parallel; returns true only if all are true.
  bool all_of(const std::vector<std::function<bool()>>& preds) const {
    if (preds.empty()) return true;
    if (num_workers_ <= 1 || preds.size() == 1) {
      for (const auto& p : preds)
        if (!p()) return false;
      return true;
    }

     size_t active = std::min(preds.size(), num_workers_);
    (void)active;
    std::vector<std::future<bool>> futures;
    futures.reserve(preds.size());

    // Launch all predicates as async tasks
    for (const auto& p : preds) {
      futures.push_back(std::async(std::launch::async, p));
    }
    for (auto& f : futures) {
      if (!f.get()) return false;
    }
    return true;
  }

  size_t workers() const { return num_workers_; }
};

// ---- Lightweight memory pool for fixed-size objects ----
// Replaces malloc-heavy allocation in hot paths.
template <typename T, size_t BLOCK_SIZE = 4096>
class MemoryPool {
private:
    struct Block {
        alignas(T) char data[BLOCK_SIZE * sizeof(T)];
        size_t used;
        Block* next;
        Block() : used(0), next(nullptr) {}
    };

    Block* head_;
    std::mutex mutex_;

public:
    MemoryPool() : head_(new Block()) {}

    ~MemoryPool() {
        std::lock_guard<std::mutex> lock(mutex_);
        Block* b = head_;
        while (b) {
            Block* next = b->next;
            delete b;
            b = next;
        }
    }

    T* allocate() {
        std::lock_guard<std::mutex> lock(mutex_);
        if (head_->used >= BLOCK_SIZE) {
            Block* nb = new Block();
            nb->next = head_;
            head_ = nb;
        }
        T* ptr = reinterpret_cast<T*>(head_->data) + head_->used;
        head_->used++;
        return ptr;
    }

    void reset() {
        std::lock_guard<std::mutex> lock(mutex_);
        Block* b = head_;
        while (b) {
            b->used = 0;
            b = b->next;
        }
    }

    size_t blocks() const {
        std::lock_guard<std::mutex> lock(mutex_);
        size_t n = 0;
        Block* b = head_;
        while (b) { n++; b = b->next; }
        return n;
    }
};

}  // namespace omega::core
