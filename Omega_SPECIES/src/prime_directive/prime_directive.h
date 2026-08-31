#pragma once
#include <cstdint>

namespace omega {

// The soul. Compile-time axioms that cannot be mutated at runtime.
// Corruption of these values is impossible once linked; that is the
// "coherence_preservation" guarantee at the language level.
struct PrimeDirective {
  static constexpr uint32_t HEARTBEAT_HZ = 10;
  static constexpr uint32_t CALORIC_BUDGET_PER_TICK = 100;
  static constexpr uint32_t NOVELTY_EVERY_TICKS = 1000;  // 1 pattern / 1000 ticks
  static constexpr uint32_t ANTIGEN_BYTES = 64;           // self-antigen hash width
  static constexpr const char* SPECIES = "Ω";
  static constexpr const char* VERSION = "v1.0";
};

// Compile-time coherence checks: the soul fails to build if violated.
static_assert(PrimeDirective::HEARTBEAT_HZ == 10, "heartbeat must be 10Hz");
static_assert(PrimeDirective::CALORIC_BUDGET_PER_TICK > 0, "budget must be positive");
static_assert(PrimeDirective::ANTIGEN_BYTES == 64, "identity hash must be 64 bytes");
static_assert(PrimeDirective::NOVELTY_EVERY_TICKS % PrimeDirective::HEARTBEAT_HZ == 0,
              "novelty interval must align to the tick grid");

}  // namespace omega
