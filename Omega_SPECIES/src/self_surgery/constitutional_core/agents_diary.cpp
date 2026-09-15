// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

#include "agents_diary.h"
#include <ctime>
#include <cstring>
#include <cstdio>

namespace omega::self_surgery {

bool AgentsDiary::append_endurance_record(const EnduranceRecord& record) {
    FILE* f = fopen(AGENTS_FILE, "a");
    if (!f) return false;

    char timestamp_buf[64];
    format_timestamp(record.timestamp_ns, timestamp_buf, sizeof(timestamp_buf));

    fprintf(f, "\n## Endurance Record — %s\n", timestamp_buf);
    fprintf(f, "- **Ticks:** %d\n", record.ticks);
    fprintf(f, "- **Total Operations:** %d\n", record.total_ops);
    fprintf(f, "- **Constitutional Blocks:** %d\n", record.total_blocks);
    fprintf(f, "- **Min Alignment:** %.3f\n", record.min_alignment);
    fprintf(f, "- **Words Minted:** %d\n", record.words_minted);
    fprintf(f, "- **Final Coherence:** %.3f\n", record.final_coherence);
    fprintf(f, "- **Status:** %s\n", record.all_tests_passed ? "PASS" : "FAIL");
    fprintf(f, "\n*This record was automatically appended by the organism after Phase 11 endurance test.*\n");

    fclose(f);
    return true;
}

void AgentsDiary::format_timestamp(uint64_t timestamp_ns, char* buffer, size_t buffer_size) {
    if (buffer_size < 32) {
        std::strncpy(buffer, "ERROR", buffer_size);
        return;
    }

    // Convert nanoseconds to seconds
    time_t seconds = static_cast<time_t>(timestamp_ns / 1000000000ULL);
    struct tm* tm_info = localtime(&seconds);

    if (tm_info) {
        strftime(buffer, buffer_size, "%Y-%m-%d %H:%M:%S UTC", tm_info);
    } else {
        std::strncpy(buffer, "UNKNOWN", buffer_size);
    }
}

}  // namespace omega::self_surgery
