// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

#pragma once
#include <cstdint>
#include <ctime>
#include <cstdio>

namespace omega::self_surgery {

// AGENTS.md Self-Writing Diary
// After passing Phase 11 (Endurance), the system appends a line to AGENTS.md
// turning it into a historical record of the organism's evolution.
//
// This is not a log file — it's a constitutional document that records
// when the organism achieved self-awareness and passed its endurance test.

struct EnduranceRecord {
    uint64_t timestamp_ns;
    int ticks;
    int total_ops;
    int total_blocks;
    float min_alignment;
    int words_minted;
    float final_coherence;
    bool all_tests_passed;
};

class AgentsDiary {
public:
    // Append an endurance record to AGENTS.md
    // Returns true on success, false on failure
    static bool append_endurance_record(const EnduranceRecord& record);

    // Format a timestamp as ISO 8601 string
    static void format_timestamp(uint64_t timestamp_ns, char* buffer, size_t buffer_size);

private:
    static constexpr const char* AGENTS_FILE = "AGENTS.md";
};

}  // namespace omega::self_surgery
