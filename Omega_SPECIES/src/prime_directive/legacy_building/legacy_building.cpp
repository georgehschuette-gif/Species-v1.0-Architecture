// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#include "legacy_building.h"
#include <cstring>
#include <algorithm>

namespace omega {

void LegacyBuilding::snapshot(uint32_t topo, const uint8_t* geno, uint32_t patterns,
                              uint32_t surgeries, uint32_t anomalies, uint32_t ticks,
                              const char* tag, uint32_t shard_id, uint64_t ts,
                              uint32_t parent, const char* inst) {
  art_.topo_hash = topo;
  if (geno) memcpy(art_.antigen, geno, 32);
  art_.patterns_harvested = patterns;
  art_.surgeries = surgeries;
  art_.anomalies = anomalies;
  art_.ticks = ticks;
  memset(art_.tag, 0, sizeof(art_.tag));
  if (tag) {
    int n = 0;
    while (tag[n] && n < 15) { art_.tag[n] = tag[n]; n++; }
  }
  // Extended fields
  art_.shard_id = shard_id;
  art_.timestamp = ts;
  art_.parent_hash = parent;
  memset(art_.instance_name, 0, sizeof(art_.instance_name));
  if (inst) {
    int n = 0;
    while (inst[n] && n < 15) { art_.instance_name[n] = inst[n]; n++; }
  }
}

void LegacyBuilding::serialize(uint8_t* out, int* outlen) const {
  if (!out) return;
  memcpy(out, &art_, sizeof(art_));
  if (outlen) *outlen = (int)sizeof(art_);
}

void LegacyBuilding::to_hex(char* out, int outlen) const {
  static const char* hex = "0123456789abcdef";
  const uint8_t* p = reinterpret_cast<const uint8_t*>(&art_);
  int n = (int)sizeof(art_);
  int k = 0;
  for (int i = 0; i < n && k + 2 < outlen; i++) {
    out[k++] = hex[(p[i] >> 4) & 0xF];
    out[k++] = hex[p[i] & 0xF];
  }
  out[k] = '\0';
}

// ---- ShardedPersistence ----

void ShardedPersistence::add(const LegacyArtifact& art) {
  std::lock_guard<std::mutex> lock(mutex_);
  shards_.push_back(art);
}

LegacyArtifact ShardedPersistence::merge_shard(uint32_t shard_id) const {
  std::lock_guard<std::mutex> lock(mutex_);
  LegacyArtifact merged{};
  int count = 0;
  for (const auto& a : shards_) {
    if (a.shard_id != shard_id) continue;
    if (count == 0) {
      merged = a;
    } else {
      merged.patterns_harvested += a.patterns_harvested;
      merged.surgeries += a.surgeries;
      merged.anomalies += a.anomalies;
      merged.ticks += a.ticks;
    }
    count++;
  }
  return merged;
}

void ShardedPersistence::serialize_all(uint8_t* out, size_t* outlen) const {
  std::lock_guard<std::mutex> lock(mutex_);
  size_t offset = 0;
  for (const auto& a : shards_) {
    if (out && offset + sizeof(a) <= *outlen) {
      memcpy(out + offset, &a, sizeof(a));
      offset += sizeof(a);
    }
  }
  if (outlen) *outlen = offset;
}

size_t ShardedPersistence::count() const {
  std::lock_guard<std::mutex> lock(mutex_);
  return shards_.size();
}

size_t ShardedPersistence::shard_count() const {
  std::lock_guard<std::mutex> lock(mutex_);
  std::vector<uint32_t> ids;
  for (const auto& a : shards_) {
    if (std::find(ids.begin(), ids.end(), a.shard_id) == ids.end())
      ids.push_back(a.shard_id);
  }
  return ids.size();
}

}  // namespace omega
