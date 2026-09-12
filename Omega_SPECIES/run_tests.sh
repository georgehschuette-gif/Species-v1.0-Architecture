# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
#!/bin/sh
# run_tests.sh — POSIX-native test runner for Ω_SPECIES (Linux/macOS)
# Builds all test suites and runs them in parallel, reporting pass/fail per suite.
# Usage: ./run_tests.sh [--sequential] [--stress] [--fuzz] [--endurance-1m]

set -e

PHASE_COUNT=12
PARALLEL=1
RUN_STRESS=1
RUN_FUZZ=0
RUN_ENDURANCE=0

for arg in "$@"; do
  case "$arg" in
    --sequential) PARALLEL=0 ;;
    --stress)     RUN_STRESS=1 ;;
    --fuzz)       RUN_FUZZ=1 ;;
    --endurance-1m) RUN_ENDURANCE=1 ;;
    *) echo "Unknown option: $arg"; exit 1 ;;
  esac
done

echo "=== Ω_SPECIES Test Runner ==="
echo "OS: $(uname -s)  ARCH: $(uname -m)"
echo ""

# Build all test binaries first
echo ">>> Building test suite (parallel jobs: $(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4))..."
make -j"$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)" $(TEST_PARALLEL_EXTRAS) 2>&1 || {
  echo "ERROR: Build failed"
  exit 1
}
echo "Build complete."
echo ""

PASS=0
FAIL=0
FAILED_SUITES=""

if [ "$PARALLEL" = "1" ]; then
  echo ">>> Running $PHASE_COUNT phase suites in parallel..."
  TMPDIR=$(mktemp -d)
  trap 'rm -rf "$TMPDIR"' EXIT

  pid_list=""
  for i in $(seq 1 "$PHASE_COUNT"); do
    make "test$i" >"$TMPDIR/phase${i}.log" 2>&1 &
    pid_list="$pid_list $!"
  done

  for i in $(seq 1 "$PHASE_COUNT"); do
    bin="phase${i}"
    wait $(echo "$pid_list" | cut -d' ' -f"$i")
    rc=$?
    if [ "$rc" = "0" ]; then
      PASS=$((PASS + 1))
      tail -1 "$TMPDIR/phase${i}.log" | sed "s/^/  [PASS] /"
    else
      FAIL=$((FAIL + 1))
      FAILED_SUITES="$FAILED_SUITES phase${i}"
      echo "  [FAIL] Phase $i — see $TMPDIR/phase${i}.log"
    fi
  done
else
  echo ">>> Running $PHASE_COUNT phase suites sequentially..."
  for i in $(seq 1 "$PHASE_COUNT"); do
    if make "test$i" >/dev/null 2>&1; then
      PASS=$((PASS + 1))
      echo "  [PASS] Phase $i"
    else
      FAIL=$((FAIL + 1))
      FAILED_SUITES="$FAILED_SUITES phase${i}"
      echo "  [FAIL] Phase $i"
    fi
  done
fi

# Optional stress harness
if [ "$RUN_STRESS" = "1" ]; then
  echo ""
  echo ">>> Running breaking-point sweep (Phase 9 stress)..."
  if make stress >/dev/null 2>&1; then
    PASS=$((PASS + 1))
    echo "  [PASS] Stress harness"
  else
    FAIL=$((FAIL + 1))
    FAILED_SUITES="$FAILED_SUITES stress"
    echo "  [FAIL] Stress harness"
  fi
fi

# Optional fuzz test
if [ "$RUN_FUZZ" = "1" ]; then
  echo ""
  echo ">>> Running fuzz test..."
  if make fuzz >/dev/null 2>&1; then
    PASS=$((PASS + 1))
    echo "  [PASS] Fuzz test"
  else
    FAIL=$((FAIL + 1))
    FAILED_SUITES="$FAILED_SUITES fuzz"
    echo "  [FAIL] Fuzz test"
  fi
fi

# Optional endurance test
if [ "$RUN_ENDURANCE" = "1" ]; then
  echo ""
  echo ">>> Running 1M-tick endurance test..."
  if make endurance-1m >/dev/null 2>&1; then
    PASS=$((PASS + 1))
    echo "  [PASS] Endurance 1M"
  else
    FAIL=$((FAIL + 1))
    FAILED_SUITES="$FAILED_SUITES endurance-1m"
    echo "  [FAIL] Endurance 1M"
  fi
fi

echo ""
echo "=== Results: $PASS passed, $FAIL failed ==="
if [ -n "$FAILED_SUITES" ]; then
  echo "Failed suites:$FAILED_SUITES"
  exit 1
fi
echo "All suites passed."
exit 0
