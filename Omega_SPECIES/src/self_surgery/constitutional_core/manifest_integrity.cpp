// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

#include "manifest_integrity.h"
#include "immune_system/self_antigen/sha256.h"
#include <cstring>

namespace omega::self_surgery {

bool ManifestIntegrity::verify_amendment(
    const uint8_t* manifest_data,
    size_t manifest_len,
    const Amendment& amendment,
    const uint8_t* self_antigen_key,
    int total_distributed_agents
) {
    // Step 1: Verify self_antigen signature
    // The amendment must be signed by the current self_antigen
    uint8_t computed_self_sig[32];
    // Simple HMAC: SHA256(key || manifest || key)
    uint8_t buf[512];
    size_t key_len = 32;
    size_t n = manifest_len < 448 ? manifest_len : 448;

    memcpy(buf, self_antigen_key, key_len);
    if (manifest_data && manifest_len > 0) memcpy(buf + key_len, manifest_data, n);
    memcpy(buf + key_len + n, self_antigen_key, key_len);

    sha256(buf, key_len + n + key_len, computed_self_sig);

    if (memcmp(computed_self_sig, amendment.self_antigen_signature, 32) != 0) {
        return false;  // Self-antigen signature invalid
    }

    // Step 2: Verify supermajority of distributed_self signatures
    int signature_count = count_signatures(amendment.distributed_signatures);

    if (!is_supermajority(signature_count, total_distributed_agents)) {
        return false;  // Supermajority not met
    }

    // Step 3: Verify amendment hash integrity
    uint8_t hash_buf[512];
    size_t hash_len = manifest_len < 480 ? manifest_len : 480;
    if (manifest_data && manifest_len > 0) memcpy(hash_buf, manifest_data, hash_len);

    uint8_t computed_hash[32];
    sha256(hash_buf, hash_len, computed_hash);

    // Convert first 4 bytes of hash to uint32_t for comparison
    uint32_t computed_amendment_hash = 
        (static_cast<uint32_t>(computed_hash[0]) << 24) |
        (static_cast<uint32_t>(computed_hash[1]) << 16) |
        (static_cast<uint32_t>(computed_hash[2]) << 8) |
        static_cast<uint32_t>(computed_hash[3]);

    if (computed_amendment_hash != amendment.amendment_hash) {
        return false;  // Amendment hash mismatch
    }

    return true;
}

int ManifestIntegrity::count_signatures(const uint8_t* signature_mask) {
    int count = 0;
    for (int i = 0; i < 32; i++) {
        uint8_t byte = signature_mask[i];
        while (byte) {
            count += byte & 1;
            byte >>= 1;
        }
    }
    return count;
}

bool ManifestIntegrity::is_supermajority(int signatures, int total_agents) {
    if (total_agents <= 0) return false;
    int required = (total_agents * SUPERMAJORITY_THRESHOLD) / 100;
    if ((total_agents * SUPERMAJORITY_THRESHOLD) % 100 != 0) {
        required++;  // Round up
    }
    return signatures >= required;
}

}  // namespace omega::self_surgery
