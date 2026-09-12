// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#pragma once
#include <cstdint>
#include <cstddef>
#include <vector>
#include <mutex>

namespace omega {

// Legacy artifact: self-contained snapshot left on shutdown.
// Extended for multi-instance persistence with shard identity, timestamp,
// and lineage tracking. Still backward-compatible with single-instance use.
struct LegacyArtifact {
  uint32_t topo_hash;
  uint8_t antigen[32];          // genotype core
  uint32_t patterns_harvested;
  uint32_t surgeries;
  uint32_t anomalies;
  uint32_t ticks;
  char tag[16];

  // Extended fields for sharded/multi-instance persistence
  uint32_t shard_id = 0;         // which instance produced this
  uint64_t timestamp = 0;        // high-resolution birth timestamp
  uint32_t parent_hash = 0;      // lineage: predecessor's topo_hash
  char instance_name[16] = {};   // human-readable instance identifier
};

// Sharded persistence manager: collects, merges, and exports artifacts
// from multiple Ω instances. Thread-safe for concurrent append.
class ShardedPersistence {
 public:
  void add(const LegacyArtifact& art);

  // Merge compatible artifacts (same shard_id lineage) into a consolidated view
  LegacyArtifact merge_shard(uint32_t shard_id) const;

  // Export all artifacts as a binary blob (for file/network persistence)
  void serialize_all(uint8_t* out, size_t* outlen) const;

  size_t count() const;
  size_t shard_count() const;

 private:
  std::vector<LegacyArtifact> shards_;
  mutable std::mutex mutex_;
};

class LegacyBuilding {
 public:
  void snapshot(uint32_t topo, const uint8_t* geno, uint32_t patterns,
                uint32_t surgeries, uint32_t anomalies, uint32_t ticks,
                const char* tag, uint32_t shard_id = 0, uint64_t ts = 0,
                uint32_t parent = 0, const char* inst = "OMEGA-v1");

  const LegacyArtifact& artifact() const { return art_; }

  void serialize(uint8_t* out, int* outlen) const;
  void to_hex(char* out, int outlen) const;

 private:
  LegacyArtifact art_;
};

}  // namespace omega
