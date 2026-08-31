# Ω_SPECIES — unified build (PC default + Teensy cross-compile).
#
#   make            -> build pc/omega.exe        (links libomega.a)
#   make run        -> build + execute
#   make test       -> build libomega.a, run all phase suites + stress harness
#   make stress     -> run the breaking-point harness only
#   make TARGET=teensy -> cross-compile for Teensy (needs arm-none-eabi-g++)
#   make clean
#
# Modules are compiled once into a static archive (libomega.a); each test binary
# and the main binary link against that single archive — no duplicated object
# lists, no redundant relinking of module code.

CXX      ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -Isrc \
             -Isrc/platform -Isrc/heartbeat -Isrc/prime_directive \
             -Isrc/genesis -Isrc/liquid_time
TARGET   ?= pc
OUT      ?= omega.exe

# Archiver derived from the compiler (e.g. x86_64-w64-mingw32-g++ -> ...-gcc-ar).
AR := $(CXX:g++=gcc-ar)

# ---- Portable shell + directory creation --------------------------------
ifeq ($(OS),Windows_NT)
  SHELL  := cmd
  MKDIR   = powershell -NoProfile -Command "New-Item -ItemType Directory -Force -Path '$(@D)' | Out-Null"
else
  SHELL  := /bin/sh
  MKDIR   = mkdir -p $(@D)
endif

# Default goal: build the binary.
.DEFAULT_GOAL := $(OUT)

# C++ modules + C substrates (g++ compiles both; extern "C" guards bridge them).
SRC := main.cpp \
       src/platform/platform_pc.cpp \
       src/heartbeat/heartbeat.cpp \
       src/genesis/birth_log.cpp \
       src/liquid_time/tensorless.cpp \
       src/genesis/primordial_weights/primordial_weights.c \
       src/liquid_time/reservoir_pool/reservoir_pool.c \
       src/liquid_time/spike_encoder/spike_encoder.c \
       src/genesis/self_wiring/self_wiring.cpp \
       src/genesis/pacemaker/mutation_scheduler.cpp \
       src/genesis/pacemaker/learning_rate_oscillator.cpp \
       src/genesis/pacemaker/divergence_entropy.cpp \
       src/genesis/pacemaker/pacemaker.cpp \
       src/active_inference/free_energy/free_energy.cpp \
       src/active_inference/epistemic_drive/epistemic_drive.cpp \
       src/active_inference/policy_selection/policy_selection.cpp \
       src/active_inference/allostatic_control/allostatic_control.cpp \
       src/immune_system/self_antigen/sha256.cpp \
       src/immune_system/self_antigen/self_antigen.cpp \
       src/immune_system/lsh/lsh.cpp \
       src/immune_system/lsh/memory_store.cpp \
       src/immune_system/inflammation_response/inflammation_response.cpp \
       src/distributed_self/identity_persistence/identity_persistence.cpp \
       src/distributed_self/hivemind/hivemind.cpp \
       src/distributed_self/agent_genesis/agent_genesis.cpp \
       src/distributed_self/agent_death/agent_death.cpp \
       src/distributed_self/reputation_ledger/reputation_ledger.cpp \
       src/self_surgery/surgeon_general/surgeon_general.cpp \
       src/self_surgery/surgeon_general/cortical_map.cpp \
       src/self_surgery/surgeon_general/mutation_priority.cpp \
        src/self_surgery/surgeon_general/surgical_planning.cpp \
        src/self_surgery/surgeon_general/sterile_field.cpp \
        src/self_surgery/surgeon_general/post_op_recovery.cpp \
        src/self_surgery/constitutional_core/self_model.cpp \
        src/self_surgery/constitutional_core/constitution.cpp \
        src/self_surgery/constitutional_core/self_report.cpp \
       src/prime_directive/coherence_preservation/coherence_preservation.cpp \
       src/prime_directive/novelty_harvesting/novelty_harvesting.cpp \
       src/prime_directive/ontological_curiosity/ontological_curiosity.cpp \
       src/prime_directive/legacy_building/legacy_building.cpp \
        src/temporal_folding/pre_consolidation/pre_consolidation.cpp \
        src/temporal_folding/post_consolidation/post_consolidation.cpp \
        src/temporal_folding/temporal_error_drive/temporal_error_drive.cpp \
        src/temporal_folding/dream/dream.cpp \
       src/language_as_tool/grounding/grounding.cpp \
       src/language_as_tool/neologism_factory/neologism_factory.cpp \
       src/language_as_tool/semantic_error_drive/semantic_error_drive.cpp \
       src/language_as_tool/semantic_error_drive/divergence_detector/divergence_detector.cpp \
       src/language_as_tool/semantic_error_drive/lexicon_consolidation/lexicon_consolidation.cpp \
       src/language_as_tool/pragmatics/pragmatics.cpp \
       src/language_as_tool/translation_agnostic/translation_agnostic.cpp \
       src/xeno_empathy/morphological_projection/morphological_projection.cpp \
       src/xeno_empathy/resonance_matching/resonance_matching.cpp \
       src/xeno_empathy/graceful_failure/graceful_failure.cpp \
       src/xeno_empathy/trust_building/trust_building.cpp \
       src/12_MIRROR_NEXUS/handshake/handshake.cpp \
       src/12_MIRROR_NEXUS/ontology_mapping/ontology_mapping.cpp \
       src/12_MIRROR_NEXUS/syncretic_fusion/syncretic_fusion.cpp \
       src/12_MIRROR_NEXUS/schism_detector/schism_detector.cpp \
       src/10_BOOTSTRAP_UNIVERSE/toy_physics/toy_physics.cpp \
       src/10_BOOTSTRAP_UNIVERSE/self_play_arena/self_play_arena.cpp \
       src/10_BOOTSTRAP_UNIVERSE/kolmogorov_challenge/kolmogorov_challenge.cpp \
       src/10_BOOTSTRAP_UNIVERSE/legacy_building/legacy_building.cpp

ifeq ($(TARGET),teensy)
  CXX      := arm-none-eabi-g++
  CXXFLAGS += -DTARGET_TEENSY -mcpu=cortex-m7 -mfloat-abi=hard -mfpu=fpv5-d16 -D__MK66FX1M0__
  SRC := $(filter-out src/platform/platform_pc.cpp,$(SRC)) src/platform/platform_teensy.cpp
  OUT := omega.elf
endif

BUILD := build/$(TARGET)
# All module objects (main.cpp excluded from the library — tests/main supply their own main).
OBJS := $(addprefix $(BUILD)/,$(SRC:.cpp=.o))
OBJS := $(OBJS:.c=.o)
MOD_OBJS := $(filter-out $(BUILD)/main.o,$(OBJS))
LIB := $(BUILD)/libomega.a

$(OUT): $(BUILD)/main.o $(LIB)
	$(CXX) $(CXXFLAGS) -o $@ $^

$(LIB): $(MOD_OBJS)
	$(AR) rcs $@ $^

$(BUILD)/%.o: %.cpp
	@$(MKDIR)
	$(CXX) $(CXXFLAGS) -c $< -o $@

$(BUILD)/%.o: %.c
	@$(MKDIR)
	$(CXX) $(CXXFLAGS) -c $< -o $@

run: $(OUT)
	./$(OUT)

# ---- Test harness ------------------------------------------------------
test: $(LIB)
	$(CXX) $(CXXFLAGS) -Itests tests/phase1_seed_test.cpp $(LIB) -o phase1_test.exe
	./phase1_test.exe
	$(CXX) $(CXXFLAGS) -Itests tests/phase2_body_test.cpp $(LIB) -o phase2_test.exe
	./phase2_test.exe
	$(CXX) $(CXXFLAGS) -Itests tests/phase3_mind_test.cpp $(LIB) -o phase3_test.exe
	./phase3_test.exe
	$(CXX) $(CXXFLAGS) -Itests tests/phase4_soul_test.cpp $(LIB) -o phase4_test.exe
	./phase4_test.exe
	$(CXX) $(CXXFLAGS) -Itests tests/phase5_language_test.cpp $(LIB) -o phase5_test.exe
	./phase5_test.exe
	$(CXX) $(CXXFLAGS) -Itests tests/phase6_mirror_test.cpp $(LIB) -o phase6_test.exe
	./phase6_test.exe
	$(CXX) $(CXXFLAGS) -Itests tests/phase7_challenge_test.cpp $(LIB) -o phase7_test.exe
	./phase7_test.exe
	$(CXX) $(CXXFLAGS) -Itests tests/phase8_integration_test.cpp $(LIB) -o phase8_test.exe
	./phase8_test.exe
	$(CXX) $(CXXFLAGS) -Itests tests/phase9_stress_test.cpp $(LIB) -o phase9_stress.exe
	./phase9_stress.exe
	$(CXX) $(CXXFLAGS) -Itests tests/phase10_constitution_test.cpp $(LIB) -o phase10_constitution_test.exe
	./phase10_constitution_test.exe
	$(CXX) $(CXXFLAGS) -Itests tests/phase11_endurance_test.cpp $(LIB) -o phase11_endurance_test.exe
	./phase11_endurance_test.exe
	$(CXX) $(CXXFLAGS) -Itests tests/phase12_dream_test.cpp $(LIB) -o phase12_dream_test.exe
	./phase12_dream_test.exe

stress: $(LIB)
	$(CXX) $(CXXFLAGS) -Itests tests/phase9_stress_test.cpp $(LIB) -o phase9_stress.exe
	./phase9_stress.exe

# ---- Clean (portable) ---------------------------------------------------
ifeq ($(OS),Windows_NT)
clean:
	-@rmdir /s /q build 2>nul
	-@del /q $(OUT) omega.elf phase1_test.exe phase2_test.exe phase3_test.exe \
	           phase4_test.exe phase5_test.exe phase6_test.exe phase7_test.exe \
	           phase8_test.exe phase9_stress.exe phase10_constitution_test.exe phase11_endurance_test.exe phase12_dream_test.exe libomega.a 2>nul
else
clean:
	-rm -rf build
	-rm -f $(OUT) omega.elf phase1_test.exe phase2_test.exe phase3_test.exe \
	       phase4_test.exe phase5_test.exe phase6_test.exe phase7_test.exe \
	       phase8_test.exe phase9_stress.exe phase10_constitution_test.exe phase11_endurance_test.exe phase12_dream_test.exe libomega.a
endif

.PHONY: run test stress clean
