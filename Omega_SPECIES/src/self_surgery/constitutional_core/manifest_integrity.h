// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

#pragma once
#include <cstdint>
#include <cstddef>

namespace omega::self_surgery {

// Amendment record for agent_manifest.json
struct Amendment {
    uint64_t timestamp_ns;
    uint8_t self_antigen_signature[32];  // Signature from current self_antigen
    uint8_t distributed_signatures[32];  // Bitmask of distributed_self agent signatures
    uint32_t amendment_hash;              // Hash of the amendment content
};

// Manifest integrity checker
// Verifies that agent_manifest.json amendments are properly signed
class ManifestIntegrity {
public:
    static constexpr int SUPERMAJORITY_THRESHOLD = 67;  // 67% required for supermajority

    // Check if an amendment is valid
    static bool verify_amendment(
        const uint8_t* manifest_data,
        size_t manifest_len,
        const Amendment& amendment,
        const uint8_t* self_antigen_key,
        int total_distributed_agents
    );

    // Count set bits in distributed signature bitmask
    static int count_signatures(const uint8_t* signature_mask);

    // Check if supermajority threshold is met
    static bool is_supermajority(int signatures, int total_agents);
};

}  // namespace omega::self_surgery
