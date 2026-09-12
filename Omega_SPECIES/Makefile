# Ω_SPECIES — unified build (PC default + Teensy cross-compile).
#
#   make            -> build pc/omega.exe        (links libomega.a)
#   make run        -> build + execute
#   make test       -> build libomega.a, run all phase suites + stress harness
#   make test5      -> build + run phase 5 only
#   make test6      -> build + run phase 6 only
#   make test-parallel -> compile all + run in parallel
#   make stress     -> run the breaking-point harness only (phase 9)
#   make fuzz       -> run random-input fuzz harness
#   make endurance-1m -> run 1M-tick endurance test with memory metrics
#   make test-parallel-posix -> POSIX-native parallel test execution (Linux/macOS)
#   make TARGET=teensy -> cross-compile for Teensy (needs arm-none-eabi-g++)
#   make clean
#
# Modules are compiled once into a static archive (libomega.a); each test binary
# and the main binary link against that single archive — no duplicated object
# lists, no redundant relinking of module code.

CXX      ?= g++
CXXFLAGS ?= -std=c++17 -O2 -Wall -Wextra -Isrc \
              -Isrc/platform -Isrc/heartbeat -Isrc/prime_directive \
              -Isrc/genesis -Isrc/liquid_time -Isrc/core \
              -pthread -MMD -MP
TARGET   ?= pc
# Windows executable extension
ifeq ($(OS),Windows_NT)
  EXE_EXT := .exe
else
  EXE_EXT :=
endif
OUT      ?= omega$(EXE_EXT)

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
  EXE_EXT := .elf
endif

BUILD := build/$(TARGET)
# All module objects (main.cpp excluded from the library — tests/main supply their own main).
OBJS := $(addprefix $(BUILD)/,$(SRC:.cpp=.o))
OBJS := $(OBJS:.c=.o)
MOD_OBJS := $(filter-out $(BUILD)/main.o,$(OBJS))
LIB := $(BUILD)/libomega.a

$(OUT): $(BUILD)/main.o $(LIB)
	$(CXX) $(CXXFLAGS) -o $@ $^ -pthread

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

# ---- Test targets ----------------------------------------------------------
# Compile all test binaries first, then run with parallelism.
# test: sequential (backward compatible)
# test-parallel: compiles all + runs in parallel via xargs/make -j
# testN: build + run individual phase N

TEST_SRCS := tests/phase1_seed_test.cpp tests/phase2_body_test.cpp \
             tests/phase3_mind_test.cpp tests/phase4_soul_test.cpp \
             tests/phase5_language_test.cpp tests/phase6_mirror_test.cpp \
             tests/phase7_challenge_test.cpp tests/phase8_integration_test.cpp \
             tests/phase9_stress_test.cpp tests/phase10_constitution_test.cpp \
             tests/phase11_endurance_test.cpp tests/phase12_dream_test.cpp

TEST_BINS := $(addsuffix $(EXE_EXT),$(TEST_SRCS:tests/%.cpp=build/$(TARGET)/%_test))

$(BUILD)/%_test$(EXE_EXT): tests/%.cpp $(LIB)
	@$(MKDIR)
	$(CXX) $(CXXFLAGS) -Itests $< $(LIB) -o $@ -pthread

# Sequential test execution (cross-platform)
test: $(TEST_BINS)
ifeq ($(OS),Windows_NT)
	@for %%f in ($(TEST_BINS)) do @(echo --- %%f --- & "%%f" || exit /b 1)
else
	@for bin in $(TEST_BINS); do echo "--- $$bin ---"; ./$$bin || exit 1; done
endif

# Per-phase targets for incremental builds
test1: $(BUILD)/phase1_seed_test_test$(EXE_EXT)
	@echo "=== Phase 1: Seed ===" ; ./$<
test2: $(BUILD)/phase2_body_test_test$(EXE_EXT)
	@echo "=== Phase 2: Body ===" ; ./$<
test3: $(BUILD)/phase3_mind_test_test$(EXE_EXT)
	@echo "=== Phase 3: Mind ===" ; ./$<
test4: $(BUILD)/phase4_soul_test_test$(EXE_EXT)
	@echo "=== Phase 4: Soul ===" ; ./$<
test5: $(BUILD)/phase5_language_test_test$(EXE_EXT)
	@echo "=== Phase 5: Language ===" ; ./$<
test6: $(BUILD)/phase6_mirror_test_test$(EXE_EXT)
	@echo "=== Phase 6: Mirror ===" ; ./$<
test7: $(BUILD)/phase7_challenge_test_test$(EXE_EXT)
	@echo "=== Phase 7: Challenge ===" ; ./$<
test8: $(BUILD)/phase8_integration_test_test$(EXE_EXT)
	@echo "=== Phase 8: Integration ===" ; ./$<
test9: $(BUILD)/phase9_stress_test_test$(EXE_EXT)
	@echo "=== Phase 9: Stress ===" ; ./$<
test10: $(BUILD)/phase10_constitution_test_test$(EXE_EXT)
	@echo "=== Phase 10: Constitution ===" ; ./$<
test11: $(BUILD)/phase11_endurance_test_test$(EXE_EXT)
	@echo "=== Phase 11: Endurance ===" ; ./$<
test12: $(BUILD)/phase12_dream_test_test$(EXE_EXT)
	@echo "=== Phase 12: Dream ===" ; ./$<

# Parallel test execution
test-parallel: $(TEST_BINS)
	@echo "=== Running $(words $(TEST_BINS)) test suites in parallel ==="
ifeq ($(OS),Windows_NT)
	@powershell -NoProfile -Command " $$jobs = @(); $(foreach bin,$(TEST_BINS), $$jobs += Start-Job -ScriptBlock { param($$p) & $$p 2>&1 } -ArgumentList '$(CURDIR)/$(bin)';) $$jobs | Wait-Job; $$jobs | Receive-Job; $$jobs | Remove-Job"
else
	@for bin in $(TEST_BINS); do (./$$bin > /tmp/$$bin.out 2>&1) & done; wait; for bin in $(TEST_BINS); do echo "--- $$bin ---"; cat /tmp/$$bin.out; done
endif

# POSIX-native parallel execution (Linux/macOS) using make -j jobserver
STRESS_BIN := $(BUILD)/phase9_stress_test_test$(EXE_EXT)
FUZZ_BIN   := $(BUILD)/fuzz_test_test$(EXE_EXT)
ENDURANCE_BIN := $(BUILD)/phase12_endurance_1m_test_test$(EXE_EXT)

test-parallel-posix: $(TEST_BINS) $(STRESS_BIN) $(FUZZ_BIN)
	@echo "=== Running $(words $(TEST_BINS)) test suites in parallel (POSIX jobserver) ==="
	@failed=0; \
	for bin in $(TEST_BINS); do \
	  ./$$bin & \
	done; \
	for bin in $(TEST_BINS); do \
	  wait $${!; echo "--- $$bin ---"; if [ $${PIPESTATUS[0]} -ne 0 ]; then failed=1; fi; \
	done; \
	echo "Parallel test run complete ($$failed failures)"; exit $$failed

# Breaking-point harness (Phase 9)
stress: $(STRESS_BIN)
	@echo "=== Phase 9: Breaking-point sweep ===" ; ./$(STRESS_BIN)

# Random-input fuzz harness
fuzz: $(FUZZ_BIN)
	@echo "=== Fuzz Test: random inputs ===" ; ./$(FUZZ_BIN)

# 1M-tick endurance test with memory metrics
endurance-1m: $(ENDURANCE_BIN)
	@echo "=== Endurance 1M: long-running stability ===" ; ./$(ENDURANCE_BIN)

# ---- Dependency tracking (auto-generated) ----------------------------------
-include $(OBJS:.o=.d) $(MOD_OBJS:.o=.d)

# ---- Clean (portable) ---------------------------------------------------
ifeq ($(OS),Windows_NT)
clean:
	-@rmdir /s /q build 2>nul
	-@del /q $(OUT) omega.elf $(TEST_BINS) libomega.a 2>nul
else
clean:
	-rm -rf build
	-rm -f $(OUT) omega.elf $(TEST_BINS) libomega.a
endif

.PHONY: run test test-parallel test-parallel-posix test1 test2 test3 test4 test5 test6 test7 test8 test9 test10 test11 test12 stress fuzz endurance-1m clean
